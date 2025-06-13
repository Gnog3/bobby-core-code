use core::{
    cmp::Ordering,
    mem::{offset_of, transmute, MaybeUninit},
};

use alloc::string::String;
use bytemuck::{NoUninit, Pod, Zeroable};

use crate::{storage::StorageExt, Error, Offset, Result, Storage, MAX_LEN_NAME, PAGE_SIZE};

const ALLOC_OFFSET: u32 = offset_of!(Main, free_page_count) as u32;
const CONTENTS_OFFSET: u32 = offset_of!(Header, attr) as u32;
const FILE_SIZE_OFFSET: u32 = CONTENTS_OFFSET;
const HEIGHT_OFFSET: u32 = offset_of!(Header, height) as u32;
const LEFT_CHILD_OFFSET: u32 = offset_of!(Header, left_child) as u32;
const RIGHT_CHILD_OFFSET: u32 = offset_of!(Header, right_child) as u32;
const NAME_LEN_OFFSET: u32 = offset_of!(Header, name_len) as u32;
const NAME_OFFSET: u32 = NAME_LEN_OFFSET + size_of::<u32>() as u32;
const IN_USE_OFFSET: u32 = offset_of!(Header, in_use) as u32;
const INLINE_PAGES_OFFSET: u32 = size_of::<Header>() as u32 + MAX_LEN_NAME;
const PREV_PAGES_OFFSET: u32 = PAGE_SIZE - 8;
const NEXT_PAGES_OFFSET: u32 = PAGE_SIZE - 4;
const PARENT_OFFSET: u32 = offset_of!(Header, parent) as u32;

#[repr(u32)]
#[derive(Debug, Clone, Copy, NoUninit, PartialEq, Eq)]
pub enum PageKind {
    Main = 1,
    File,
    Directory,
}

// Common header for a file and a directory
#[repr(C)]
#[derive(Debug, Clone, Copy, NoUninit)]
pub struct Header {
    kind: PageKind, // File or Directory
    parent: Offset,
    in_use: u32,
    left_child: Offset,
    right_child: Offset,
    height: u32,
    attr: u32, // Contents if folder, size if file
    name_len: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, NoUninit)]
pub struct Main {
    kind: PageKind, // Main
    total_page_count: u32,
    free_page_count: u32,
    next_free_page: Offset,
}

#[derive(Clone)]
pub struct InternalFilesystem<S> {
    s: S,
}

impl<S: Storage> InternalFilesystem<S> {
    pub const fn new(storage: S) -> Self {
        Self { s: storage }
    }

    fn make_free_list_chain(&self, page_from: u32, page_to: u32) {
        for i in page_from..page_to - 1 {
            let offset = i * PAGE_SIZE;
            let next = (i + 1) * PAGE_SIZE;
            self.s.write_t(offset, &next);
        }
        // Next page for last page is 0
        let offset = (page_to - 1) * PAGE_SIZE;
        let next = 0u32;
        self.s.write_t(offset, &next);
    }

    fn make_root_directory(&self, page: u32) {
        let header = Header {
            kind: PageKind::Directory,
            parent: 0,
            in_use: 0,
            left_child: 0,
            right_child: 0,
            height: 1,
            attr: 0,
            name_len: 0,
        };
        let offset = page * PAGE_SIZE;
        self.s.write_t(offset, &header);
        // let name = b"/";
        // self.s.write(offset + NAME_OFFSET, name);
    }

    pub fn format(&self) {
        let total_page_count = self.s.capacity() / PAGE_SIZE;
        self.make_free_list_chain(2, total_page_count);
        self.make_root_directory(1);
        let main = Main {
            kind: PageKind::Main,
            total_page_count,
            free_page_count: total_page_count - 2,
            next_free_page: 2 * PAGE_SIZE,
        };
        self.s.write_t(0, &main);
    }

    fn alloc_page(&self) -> Result<Offset> {
        #[repr(C)]
        #[derive(Pod, Clone, Copy, Zeroable)]
        struct Alloc {
            free_page_count: u32,
            next_free_page: Offset,
        }

        let mut alloc: Alloc = self.s.read_t(ALLOC_OFFSET);

        if alloc.free_page_count == 0 {
            return Err(Error::StorageFull);
        }

        let page = alloc.next_free_page;
        alloc.next_free_page = self.s.read_t(page);
        alloc.free_page_count -= 1;
        self.s.write_t(ALLOC_OFFSET, &alloc);

        Ok(page)
    }

    fn free_page(&self, page: Offset) {
        #[repr(C)]
        #[derive(Pod, Clone, Copy, Zeroable)]
        struct Alloc {
            free_page_count: u32,
            next_free_page: Offset,
        }

        let mut alloc: Alloc = self.s.read_t(ALLOC_OFFSET);
        self.s.write_t(page, &alloc.next_free_page);
        alloc.free_page_count += 1;
        alloc.next_free_page = page;
        self.s.write_t(ALLOC_OFFSET, &alloc);
    }

    pub fn get_free_page_count(&self) -> u32 {
        self.s.read_t(ALLOC_OFFSET)
    }

    pub fn get_total_page_count(&self) -> u32 {
        self.s.capacity() / PAGE_SIZE
    }

    pub fn root(&self) -> Offset {
        PAGE_SIZE
    }

    pub fn try_set_iter_in_use(&self, entry: Offset) -> Result<()> {
        let count: u32 = self.s.read_t(entry + IN_USE_OFFSET);
        if count >> 31 != 0 {
            return Err(Error::InUse);
        }
        let count = count | (1 << 31);
        self.s.write_t(entry + IN_USE_OFFSET, &count);
        Ok(())
    }

    pub fn unset_iter_in_use(&self, entry: Offset) {
        let count: u32 = self.s.read_t(entry + IN_USE_OFFSET);
        let count = count & !(1 << 31);
        self.s.write_t(entry + IN_USE_OFFSET, &count);
    }

    pub fn get_iter_in_use(&self, entry: Offset) -> Result<()> {
        let count: u32 = self.s.read_t(entry + IN_USE_OFFSET);
        if count >> 31 != 0 {
            return Err(Error::InUse);
        }
        Ok(())
    }

    pub fn try_increment_in_use(&self, entry: Offset) -> Result<()> {
        let count: u32 = self.s.read_t(entry + IN_USE_OFFSET);
        if count > 0 {
            return Err(Error::InUse);
        }
        self.s.write_t(entry + IN_USE_OFFSET, &1);
        Ok(())
    }

    pub fn increment_in_use(&self, entry: Offset) {
        let count: u32 = self.s.read_t(entry + IN_USE_OFFSET);
        self.s.write_t(entry + IN_USE_OFFSET, &(count + 1));
    }

    pub fn decrement_in_use(&self, entry: Offset) {
        let count: u32 = self.s.read_t(entry + IN_USE_OFFSET);
        self.s.write_t(entry + IN_USE_OFFSET, &(count - 1));
    }

    pub fn get_in_use(&self, entry: Offset) -> u32 {
        self.s.read_t(entry + IN_USE_OFFSET)
    }

    pub fn reset_in_use(&self, entry: Offset) {
        self.s.write_t(entry + IN_USE_OFFSET, &0u32);
    }

    pub fn add_entry(&self, dir: Offset, name: &str, kind: PageKind) -> Result<Offset> {
        let entry = self.make_entry(dir, name, kind)?;
        let contents: Offset = self.s.read_t(dir + CONTENTS_OFFSET);

        match self.insert_entry(contents, entry, name) {
            Ok(new_root) => {
                self.s.write_t(dir + CONTENTS_OFFSET, &new_root);
                Ok(entry)
            }
            Err(err) => {
                self.free_page(entry);
                Err(err)
            }
        }
    }

    pub fn remove_entry(
        &self,
        dir: Offset,
        name: &str,
        before_delete: impl FnOnce(Offset) -> Result<()>,
    ) -> Result<()> {
        let contents: Offset = self.s.read_t(dir + CONTENTS_OFFSET);

        let new_root = self.remove_entry2(contents, name, before_delete)?;
        self.s.write_t(dir + CONTENTS_OFFSET, &new_root);
        Ok(())
    }

    // index must be < 96
    pub fn get_inline_file_data_page(&self, file: Offset, index: u32) -> Offset {
        debug_assert!(index < 96);
        self.s
            .read_t(file + INLINE_PAGES_OFFSET + index * size_of::<u32>() as u32)
    }

    // index must be < 96
    pub fn alloc_inline_file_data_page(&self, file: Offset, index: u32) -> Result<Offset> {
        debug_assert!(index < 96);
        let file_data = self.alloc_page()?;
        self.s.write_t(
            file + INLINE_PAGES_OFFSET + index * size_of::<u32>() as u32,
            &file_data,
        );
        Ok(file_data)
    }

    // index must be < 126
    pub fn get_page_list_file_data_page(&self, page_list: Offset, index: u32) -> Offset {
        debug_assert!(index < 126);
        self.s.read_t(page_list + index * size_of::<u32>() as u32)
    }

    // index must be < 126
    pub fn alloc_page_list_file_data_page(&self, page_list: Offset, index: u32) -> Result<Offset> {
        debug_assert!(index < 126);
        let file_data = self.alloc_page()?;
        self.s
            .write_t(page_list + index * size_of::<u32>() as u32, &file_data);
        Ok(file_data)
    }

    pub fn get_next_pages(&self, page: Offset) -> Offset {
        self.s.read_t(page + NEXT_PAGES_OFFSET)
    }

    pub fn get_prev_pages(&self, page: Offset) -> Offset {
        self.s.read_t(page + PREV_PAGES_OFFSET)
    }

    pub fn alloc_next_pages(&self, page: Offset) -> Result<Offset> {
        let next_pages = self.alloc_page()?;
        self.s.write_t(page + NEXT_PAGES_OFFSET, &next_pages);
        self.s.write_t(next_pages + PREV_PAGES_OFFSET, &page);
        Ok(next_pages)
    }

    pub fn free_next_pages(&self, page: Offset) {
        let next_pages = self.get_next_pages(page);
        self.free_page(next_pages);
    }

    pub fn free_page_list(&self, page: Offset) {
        self.free_page(page);
    }

    pub fn free_file_data(&self, page: Offset) {
        self.free_page(page);
    }

    pub fn find_entry(&self, dir: Offset, name: &str) -> Result<Offset> {
        let contents: Offset = self.s.read_t(dir + CONTENTS_OFFSET);
        self.find_entry2(contents, name)
    }

    pub fn get_page_kind(&self, page: Offset) -> PageKind {
        let kind: u32 = self.s.read_t(page);
        unsafe { transmute(kind) }
    }

    pub fn get_file_size(&self, file: Offset) -> u32 {
        self.s.read_t(file + FILE_SIZE_OFFSET)
    }

    pub fn set_file_size(&self, file: Offset, size: u32) {
        self.s.write_t(file + FILE_SIZE_OFFSET, &size);
    }

    // offset can be unaligned
    pub fn read_file_data(&self, offset: Offset, buf: &mut [MaybeUninit<u8>]) {
        self.s.read(offset, buf);
    }

    // offset can be unaligned
    pub fn write_file_data(&self, offset: Offset, buf: &[u8]) {
        self.s.write(offset, buf);
    }

    pub fn get_left_child(&self, page: Offset) -> Offset {
        self.s.read_t(page + LEFT_CHILD_OFFSET)
    }

    pub fn get_right_child(&self, page: Offset) -> Offset {
        self.s.read_t(page + RIGHT_CHILD_OFFSET)
    }

    pub fn get_contents(&self, page: Offset) -> Offset {
        self.s.read_t(page + CONTENTS_OFFSET)
    }

    pub fn get_parent(&self, page: Offset) -> Offset {
        self.s.read_t(page + PARENT_OFFSET)
    }

    pub fn get_name(&self, entry: Offset, name: &mut [MaybeUninit<u8>; MAX_LEN_NAME as _]) -> u32 {
        let name_len: u32 = self.s.read_t(entry + NAME_LEN_OFFSET);

        let name = unsafe { name.get_unchecked_mut(0..name_len as usize) }; // no safety
        self.s.read(entry + NAME_OFFSET, name);
        name_len
    }

    fn get_name_to_string(&self, entry: Offset, name: &mut String) {
        let name_len: u32 = self.s.read_t(entry + NAME_LEN_OFFSET);
        let vec = unsafe { name.as_mut_vec() };
        let old_len = vec.len();
        vec.reserve(name_len as usize);
        let buf = unsafe { vec.spare_capacity_mut().get_unchecked_mut(0..name_len as _) };
        self.s.read(entry + NAME_OFFSET, buf);
        unsafe {
            vec.set_len(old_len + name_len as usize);
        }
    }

    fn get_path_internal(&self, entry: Offset, path: &mut String) {
        let parent = self.get_parent(entry);
        if parent != 0 {
            self.get_path_internal(parent, path);
            path.push('/');
            self.get_name_to_string(entry, path);
        }
    }

    pub fn get_path(&self, entry: Offset, path: &mut String) {
        self.get_path_internal(entry, path);
        if path.is_empty() {
            path.push('/');
        }
    }

    fn make_entry(&self, parent: Offset, name: &str, kind: PageKind) -> Result<Offset> {
        let new_page = self.alloc_page()?;

        let header = Header {
            kind,
            parent,
            in_use: 0,
            left_child: 0,
            right_child: 0,
            height: 1,
            attr: 0,
            name_len: name.len() as u32,
        };

        let offset = new_page;
        self.s.write_t(offset, &header);
        let offset = offset + size_of::<Header>() as u32;
        self.s.write(offset, name.as_bytes());

        Ok(new_page)
    }

    // AVL functions:

    fn height(&self, entry: Offset) -> u32 {
        if entry == 0 {
            0
        } else {
            self.s.read_t(entry + HEIGHT_OFFSET)
        }
    }

    fn b_factor(&self, entry: Offset) -> i32 {
        #[repr(C)]
        #[derive(Pod, Clone, Copy, Zeroable)]
        struct Children {
            left_child: Offset,
            right_child: Offset,
        }

        let children: Children = self.s.read_t(entry + LEFT_CHILD_OFFSET);
        let left_height = self.height(children.left_child);
        let right_height = self.height(children.right_child);
        right_height as i32 - left_height as i32
    }

    fn fix_height(&self, entry: Offset) {
        #[repr(C)]
        #[derive(Pod, Clone, Copy, Zeroable)]
        struct Children {
            left_child: Offset,
            right_child: Offset,
        }

        let children: Children = self.s.read_t(entry + LEFT_CHILD_OFFSET);
        let left_height = self.height(children.left_child);
        let right_height = self.height(children.right_child);
        let height = left_height.max(right_height) + 1;
        self.s.write_t(entry + HEIGHT_OFFSET, &height);
    }

    fn rotate_right(&self, p: Offset) -> Offset {
        let q: Offset = self.s.read_t(p + LEFT_CHILD_OFFSET);
        let q_right: Offset = self.s.read_t(q + RIGHT_CHILD_OFFSET);
        self.s.write_t(p + LEFT_CHILD_OFFSET, &q_right);
        self.s.write_t(q + RIGHT_CHILD_OFFSET, &p);
        self.fix_height(p);
        self.fix_height(q);
        q
    }

    fn rotate_left(&self, q: Offset) -> Offset {
        let p: Offset = self.s.read_t(q + RIGHT_CHILD_OFFSET);
        let p_left: Offset = self.s.read_t(p + LEFT_CHILD_OFFSET);
        self.s.write_t(q + RIGHT_CHILD_OFFSET, &p_left);
        self.s.write_t(p + LEFT_CHILD_OFFSET, &q);
        self.fix_height(q);
        self.fix_height(p);
        p
    }

    fn balance(&self, p: Offset) -> Offset {
        self.fix_height(p);
        let b_factor = self.b_factor(p);
        if b_factor == 2 {
            let p_right: Offset = self.s.read_t(p + RIGHT_CHILD_OFFSET);
            if self.b_factor(p_right) < 0 {
                let p_right = self.rotate_right(p_right);
                self.s.write_t(p + RIGHT_CHILD_OFFSET, &p_right);
            }
            self.rotate_left(p)
        } else if b_factor == -2 {
            let p_left: Offset = self.s.read_t(p + LEFT_CHILD_OFFSET);
            if self.b_factor(p_left) > 0 {
                let p_left = self.rotate_left(p_left);
                self.s.write_t(p + LEFT_CHILD_OFFSET, &p_left);
            }
            self.rotate_right(p)
        } else {
            p
        }
    }

    fn cmp_names(&self, p: Offset, entry_name: &str) -> Ordering {
        let name_len: u32 = self.s.read_t(p + NAME_LEN_OFFSET);
        let mut name = [MaybeUninit::<u8>::uninit(); MAX_LEN_NAME as usize];
        let name = unsafe { name.get_unchecked_mut(0..name_len as usize) }; // no safety
        self.s.read(p + NAME_OFFSET, name);
        let name = unsafe { MaybeUninit::slice_assume_init_ref(name) };
        entry_name.as_bytes().cmp(name)
    }

    fn insert_entry(&self, p: Offset, entry: Offset, entry_name: &str) -> Result<Offset> {
        if p == 0 {
            return Ok(entry);
        }
        let cmp = self.cmp_names(p, entry_name);
        if cmp == Ordering::Less {
            let p_left: Offset = self.s.read_t(p + LEFT_CHILD_OFFSET);
            let p_left = self.insert_entry(p_left, entry, entry_name)?;
            self.s.write_t(p + LEFT_CHILD_OFFSET, &p_left);
        } else if cmp == Ordering::Greater {
            let p_right: Offset = self.s.read_t(p + RIGHT_CHILD_OFFSET);
            let p_right = self.insert_entry(p_right, entry, entry_name)?;
            self.s.write_t(p + RIGHT_CHILD_OFFSET, &p_right);
        } else {
            return Err(Error::AlreadyExists);
        }
        Ok(self.balance(p))
    }

    fn find_min(&self, mut p: Offset) -> Offset {
        loop {
            let p_left: Offset = self.s.read_t(p + LEFT_CHILD_OFFSET);
            if p_left == 0 {
                return p;
            } else {
                p = p_left;
            }
        }
    }

    fn remove_min(&self, p: Offset) -> Offset {
        let p_left: Offset = self.s.read_t(p + LEFT_CHILD_OFFSET);
        if p_left == 0 {
            let p_right: Offset = self.s.read_t(p + RIGHT_CHILD_OFFSET);
            return p_right;
        }
        let p_left = self.remove_min(p_left);
        self.s.write_t(p + LEFT_CHILD_OFFSET, &p_left);
        self.balance(p)
    }

    fn remove_entry2(
        &self,
        p: Offset,
        entry_name: &str,
        before_delete: impl FnOnce(Offset) -> Result<()>,
    ) -> Result<Offset> {
        if p == 0 {
            return Err(Error::NotFound);
        }
        let cmp = self.cmp_names(p, entry_name);
        if cmp == Ordering::Less {
            let p_left: Offset = self.s.read_t(p + LEFT_CHILD_OFFSET);
            let p_left = self.remove_entry2(p_left, entry_name, before_delete)?;
            self.s.write_t(p + LEFT_CHILD_OFFSET, &p_left);
        } else if cmp == Ordering::Greater {
            let p_right: Offset = self.s.read_t(p + RIGHT_CHILD_OFFSET);
            let p_right = self.remove_entry2(p_right, entry_name, before_delete)?;
            self.s.write_t(p + RIGHT_CHILD_OFFSET, &p_right);
        } else {
            let q: Offset = self.s.read_t(p + LEFT_CHILD_OFFSET);
            let r: Offset = self.s.read_t(p + RIGHT_CHILD_OFFSET);
            before_delete(p)?;
            self.free_page(p);
            if r == 0 {
                return Ok(q);
            }
            let min = self.find_min(r);
            self.s
                .write_t(min + RIGHT_CHILD_OFFSET, &self.remove_min(r));
            self.s.write_t(min + LEFT_CHILD_OFFSET, &q);
            return Ok(self.balance(min));
        }
        Ok(self.balance(p))
    }

    fn find_entry2(&self, p: Offset, entry_name: &str) -> Result<Offset> {
        if p == 0 {
            return Err(Error::NotFound);
        }
        let cmp = self.cmp_names(p, entry_name);
        if cmp == Ordering::Less {
            let p_left: Offset = self.s.read_t(p + LEFT_CHILD_OFFSET);
            self.find_entry2(p_left, entry_name)
        } else if cmp == Ordering::Greater {
            let p_right: Offset = self.s.read_t(p + RIGHT_CHILD_OFFSET);
            self.find_entry2(p_right, entry_name)
        } else {
            Ok(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::mem::MaybeUninit;

    use bytemuck::{Pod, Zeroable};

    use crate::{
        internal_fs::{Error, CONTENTS_OFFSET, LEFT_CHILD_OFFSET},
        storage::{tests::HeapStorage, Storage, StorageExt},
        Offset, MAX_LEN_NAME, PAGE_SIZE,
    };

    use super::{InternalFilesystem, PageKind, NAME_LEN_OFFSET, NAME_OFFSET};
    extern crate std;
    use std::prelude::v1::*;
    // use std::println;
    #[derive(Debug, PartialEq, Eq)]
    struct Tree {
        name: String,
        left: Option<Box<Tree>>,
        right: Option<Box<Tree>>,
    }

    fn fs() -> InternalFilesystem<HeapStorage> {
        let fs = InternalFilesystem {
            s: HeapStorage::new(),
        };
        fs.format();
        fs
    }

    #[test]
    fn alloc_page() {
        let fs = fs();
        let total_pages = fs.s.capacity() / PAGE_SIZE;
        for i in 2..total_pages {
            assert_eq!(fs.alloc_page().unwrap(), i * PAGE_SIZE);
        }
        assert_eq!(fs.alloc_page(), Err(Error::StorageFull));
        assert_eq!(fs.alloc_page(), Err(Error::StorageFull));
        assert_eq!(fs.alloc_page(), Err(Error::StorageFull));
    }

    #[test]
    fn free_page() {
        let fs = fs();
        let total_pages = fs.s.capacity() / PAGE_SIZE;
        for i in 2..total_pages {
            assert_eq!(fs.alloc_page().unwrap(), i * PAGE_SIZE);
        }
        assert_eq!(fs.alloc_page(), Err(Error::StorageFull));
        fs.free_page(2 * PAGE_SIZE);
        assert_eq!(fs.alloc_page().unwrap(), 2 * PAGE_SIZE);
        assert_eq!(fs.alloc_page(), Err(Error::StorageFull));
    }

    #[test]
    fn find_entry() {
        let fs = fs();
        let root = fs.root();
        fs.add_entry(root, "lalala1".try_into().unwrap(), PageKind::File)
            .unwrap();
        fs.add_entry(root, "lalala2".try_into().unwrap(), PageKind::File)
            .unwrap();
        fs.add_entry(root, "lalala3".try_into().unwrap(), PageKind::File)
            .unwrap();

        let contents: Offset = fs.s.read_t(root + CONTENTS_OFFSET);
        let tree = traverse(&fs, contents);

        assert_eq!(
            tree,
            Some(Box::new(Tree {
                name: "lalala2".into(),
                left: Some(Box::new(Tree {
                    name: "lalala1".into(),
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Tree {
                    name: "lalala3".into(),
                    left: None,
                    right: None,
                }))
            }))
        );

        fs.find_entry(root, "lalala-1".try_into().unwrap())
            .unwrap_err();
        fs.find_entry(root, "lalala0".try_into().unwrap())
            .unwrap_err();

        let f = fs.find_entry(root, "lalala1".try_into().unwrap()).unwrap();
        assert_eq!(fs.get_page_kind(f), PageKind::File);
        let f = fs.find_entry(root, "lalala2".try_into().unwrap()).unwrap();
        assert_eq!(fs.get_page_kind(f), PageKind::File);
        let f = fs.find_entry(root, "lalala3".try_into().unwrap()).unwrap();
        assert_eq!(fs.get_page_kind(f), PageKind::File);
        fs.find_entry(root, "lalala4".try_into().unwrap())
            .unwrap_err();
        fs.find_entry(root, "lalala5".try_into().unwrap())
            .unwrap_err();
    }

    fn _a() {
        let fs = fs();
        let root = fs.root();

        for i in 0..=255 {
            fs.add_entry(
                root,
                std::format!("file {}", i).as_str().try_into().unwrap(),
                PageKind::File,
            )
            .unwrap();
        }
        let contents: Offset = fs.s.read_t(root + CONTENTS_OFFSET);
        let tree = traverse(&fs, contents);
        std::println!("{:#?}", tree);
        panic!()
    }

    fn traverse<S: Storage>(fs: &InternalFilesystem<S>, entry: Offset) -> Option<Box<Tree>> {
        #[repr(C)]
        #[derive(Pod, Clone, Copy, Zeroable)]
        struct Children {
            left_child: Offset,
            right_child: Offset,
        }

        if entry == 0 {
            return None;
        }

        let children: Children = fs.s.read_t(entry + LEFT_CHILD_OFFSET);

        let name_len: u32 = fs.s.read_t(entry + NAME_LEN_OFFSET);
        let mut name = [MaybeUninit::<u8>::uninit(); MAX_LEN_NAME as usize];
        let name = &mut name[0..name_len as usize];

        fs.s.read(entry + NAME_OFFSET, name);
        let name = unsafe { MaybeUninit::slice_assume_init_ref(name) };
        let name = String::from_utf8(name.to_vec()).unwrap();

        let left = traverse(fs, children.left_child);

        let right = traverse(fs, children.right_child);

        Some(Box::new(Tree { name, left, right }))
    }
}
