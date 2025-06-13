use core::mem::MaybeUninit;

use alloc::string::String;
use bobby_fs_common::MAX_LEN_NAME;

use crate::{
    internal_fs::{InternalFilesystem, PageKind},
    Error, NameArray, Offset, Result, Storage, FILE_PAGES, PAGE_LIST_PAGES, PAGE_SIZE,
};

#[derive(Debug, Clone, Copy)]
enum PageCursor {
    FileInline {
        index: u32, // [0, 96)
    },
    PageList {
        index: u32, // [0, 126)
        page_list_index: u32,
        page_list: Offset,
    },
}

pub struct File<S: Storage> {
    fs: InternalFilesystem<S>,
    file: Offset,
    cursor: u32,
    cursor_in_page: u32, // [1, PAGE_SIZE]
    cursor_page_index: u32,
    size_in_pages: u32,
    page_cursor: Option<PageCursor>,
    file_data_page: Offset,
}

impl<S: Storage> File<S> {
    pub(crate) fn new(fs: InternalFilesystem<S>, page: Offset) -> Self {
        let size = fs.get_file_size(page);
        Self {
            fs,
            file: page,
            cursor: 0,
            cursor_in_page: PAGE_SIZE,
            cursor_page_index: u32::MAX,
            size_in_pages: size_to_pages(size),
            page_cursor: None,
            file_data_page: 0,
        }
    }

    pub(crate) fn try_new(fs: InternalFilesystem<S>, page: Offset) -> Result<Self> {
        let kind = fs.get_page_kind(page);
        if kind != PageKind::File {
            return Err(Error::IsADirectory);
        }
        fs.try_increment_in_use(page)?;
        Ok(Self::new(fs, page))
    }

    pub(crate) fn create(fs: InternalFilesystem<S>, dir: Offset, name: &str) -> Result<Self> {
        if name.is_empty() || name.len() as u32 > MAX_LEN_NAME {
            return Err(Error::InvalidName);
        }
        fs.get_iter_in_use(dir)?;
        let page = fs.add_entry(dir, name, PageKind::File)?;
        fs.increment_in_use(page);
        Ok(Self::new(fs, page))
    }

    pub(crate) fn open(fs: InternalFilesystem<S>, dir: Offset, name: &str) -> Result<Self> {
        let page = fs.find_entry(dir, name)?;
        Self::try_new(fs, page)
    }

    pub fn size(&self) -> u32 {
        self.fs.get_file_size(self.file)
    }

    pub fn position(&self) -> u32 {
        self.cursor
    }

    // moves to the next page, assuming that it exists
    fn next_page(&mut self) -> Option<Offset> {
        let mut ret = None;
        match self.page_cursor {
            Some(ref mut cursor) => match cursor {
                PageCursor::FileInline { index } => {
                    *index += 1;
                    if *index >= FILE_PAGES {
                        let page_list = self.fs.get_next_pages(self.file);
                        self.file_data_page = self.fs.get_page_list_file_data_page(page_list, 0);
                        *cursor = PageCursor::PageList {
                            index: 0,
                            page_list_index: 0,
                            page_list,
                        };
                    } else {
                        self.file_data_page = self.fs.get_inline_file_data_page(self.file, *index);
                    }
                }
                PageCursor::PageList {
                    index,
                    page_list_index,
                    page_list,
                } => {
                    *index += 1;
                    if *index >= PAGE_LIST_PAGES {
                        ret = Some(*page_list);
                        *index = 0;
                        *page_list_index += 1;
                        *page_list = self.fs.get_next_pages(*page_list);
                        self.file_data_page = self.fs.get_page_list_file_data_page(*page_list, 0);
                    } else {
                        self.file_data_page =
                            self.fs.get_page_list_file_data_page(*page_list, *index);
                    }
                }
            },
            None => {
                self.page_cursor = Some(PageCursor::FileInline { index: 0 });
                self.file_data_page = self.fs.get_inline_file_data_page(self.file, 0);
            }
        }
        self.cursor_in_page = 0;
        self.cursor_page_index = self.cursor_page_index.wrapping_add(1);
        ret
    }

    // moves to the next page, possibly allocating it
    fn next_page_alloc(&mut self) -> Result<()> {
        match &mut self.page_cursor {
            Some(cursor) => match cursor {
                PageCursor::FileInline { index } => {
                    let new_cursor_page_index = self.cursor_page_index + 1;
                    let new_index = *index + 1;
                    if new_index >= FILE_PAGES {
                        let page_list;
                        let new_file_data_page;
                        if self.size_in_pages == new_cursor_page_index {
                            page_list = self.fs.alloc_next_pages(self.file)?;
                            new_file_data_page =
                                match self.fs.alloc_page_list_file_data_page(page_list, 0) {
                                    Ok(p) => p,
                                    Err(err) => {
                                        self.fs.free_next_pages(page_list);
                                        return Err(err);
                                    }
                                };
                            self.size_in_pages += 1;
                        } else {
                            page_list = self.fs.get_next_pages(self.file);
                            new_file_data_page = self.fs.get_page_list_file_data_page(page_list, 0);
                        };

                        self.file_data_page = new_file_data_page;
                        self.cursor_page_index = new_cursor_page_index;
                        *cursor = PageCursor::PageList {
                            index: 0,
                            page_list_index: 0,
                            page_list,
                        };
                    } else {
                        let new_file_data_page = if self.size_in_pages == new_cursor_page_index {
                            let alloc =
                                self.fs.alloc_inline_file_data_page(self.file, new_index)?;
                            self.size_in_pages += 1;
                            alloc
                        } else {
                            self.fs.get_inline_file_data_page(self.file, new_index)
                        };
                        *index = new_index;
                        self.file_data_page = new_file_data_page;
                        self.cursor_page_index = new_cursor_page_index;
                    }
                }
                PageCursor::PageList {
                    index,
                    page_list_index,
                    page_list,
                } => {
                    let new_cursor_page_index = self.cursor_page_index + 1;
                    let new_index = *index + 1;
                    if new_index >= PAGE_LIST_PAGES {
                        let new_page_list;
                        let new_file_data_page;
                        if self.size_in_pages == new_cursor_page_index {
                            new_page_list = self.fs.alloc_next_pages(*page_list)?;
                            new_file_data_page =
                                match self.fs.alloc_page_list_file_data_page(new_page_list, 0) {
                                    Ok(p) => p,
                                    Err(err) => {
                                        self.fs.free_next_pages(*page_list);
                                        return Err(err);
                                    }
                                };
                            self.size_in_pages += 1;
                        } else {
                            new_page_list = self.fs.get_next_pages(*page_list);
                            new_file_data_page =
                                self.fs.get_page_list_file_data_page(new_page_list, 0);
                        };
                        *index = 0;
                        *page_list_index += 1;
                        *page_list = new_page_list;
                        self.file_data_page = new_file_data_page;
                        self.cursor_page_index = new_cursor_page_index;
                    } else {
                        let new_file_data_page = if self.size_in_pages == new_cursor_page_index {
                            let alloc = self
                                .fs
                                .alloc_page_list_file_data_page(*page_list, new_index)?;
                            self.size_in_pages += 1;
                            alloc
                        } else {
                            self.fs.get_page_list_file_data_page(*page_list, new_index)
                        };
                        *index = new_index;
                        self.file_data_page = new_file_data_page;
                        self.cursor_page_index = new_cursor_page_index;
                    }
                }
            },
            None => {
                let new_cursor_page_index = 0;

                let new_file_data_page = if self.size_in_pages == new_cursor_page_index {
                    // file is empty
                    let alloc = self.fs.alloc_inline_file_data_page(self.file, 0)?;
                    self.size_in_pages += 1;
                    alloc
                } else {
                    self.fs.get_inline_file_data_page(self.file, 0)
                };
                self.page_cursor = Some(PageCursor::FileInline { index: 0 });
                self.file_data_page = new_file_data_page;
                self.cursor_page_index = new_cursor_page_index;
            }
        }
        self.cursor_in_page = 0;
        Ok(())
    }

    pub fn read(&mut self, mut buf: &mut [MaybeUninit<u8>]) -> Result<()> {
        if buf.len() as u32 + self.cursor > self.size() {
            return Err(Error::Eof);
        }

        let to_read = buf.len() as u32;
        let mut read = 0u32;

        while read < to_read {
            if self.cursor_in_page == PAGE_SIZE {
                self.next_page();
            }
            let to_read = (buf.len() as u32).min(PAGE_SIZE - self.cursor_in_page);
            // TODO: check if boundary check is optimized out
            self.fs.read_file_data(
                self.file_data_page + self.cursor_in_page,
                &mut buf[0..to_read as usize],
            );
            self.cursor_in_page += to_read;
            read += to_read;
            buf = &mut buf[to_read as usize..];
        }
        self.cursor += to_read;

        Ok(())
    }

    pub fn write(&mut self, mut buf: &[u8]) -> Result<()> {
        let to_write = buf.len() as u32;
        let mut written = 0u32;
        let mut result = Ok(());

        while written < to_write {
            if self.cursor_in_page == PAGE_SIZE {
                if let Err(err) = self.next_page_alloc() {
                    result = Err(err);
                    break;
                }
            }
            let to_write = (buf.len() as u32).min(PAGE_SIZE - self.cursor_in_page);
            self.fs.write_file_data(
                self.file_data_page + self.cursor_in_page,
                &buf[0..to_write as usize],
            );
            self.cursor_in_page += to_write;
            written += to_write;
            buf = &buf[to_write as usize..];
        }

        self.cursor += written;

        let size = self.fs.get_file_size(self.file);
        if self.cursor > size {
            self.fs.set_file_size(self.file, self.cursor);
        }

        result
    }

    pub fn truncate(&mut self) {
        let cursor_page_index = self.cursor_page_index;
        let size_in_pages = self.size_in_pages;
        let page_cursor = self.page_cursor;
        let file_data_page = self.file_data_page;
        let cursor_in_page = self.cursor_in_page;

        let mut last_removed = None;
        for _ in cursor_page_index.wrapping_add(1)..size_in_pages {
            let page_list = self.next_page();
            if let Some(page) = page_list {
                self.fs.free_page_list(page);
                last_removed = Some(page);
            }
            self.fs.free_file_data(self.file_data_page);
        }
        if let Some(PageCursor::PageList { page_list, .. }) = self.page_cursor {
            if Some(page_list) != last_removed {
                self.fs.free_page_list(page_list);
            }
        }

        self.cursor_page_index = cursor_page_index;
        self.size_in_pages = cursor_page_index.wrapping_add(1);
        self.page_cursor = page_cursor;
        self.file_data_page = file_data_page;
        self.cursor_in_page = cursor_in_page;

        self.fs.set_file_size(self.file, self.cursor);
    }

    pub fn seek_from_current(&mut self, pos: i32) -> Result<()> {
        let size = self.size() as i32;
        let final_cursor = self.cursor as i32 + pos;
        if final_cursor < 0 || final_cursor > size {
            return Err(Error::InvalidInput);
        }

        let final_page =
            final_cursor / PAGE_SIZE as i32 - (final_cursor % PAGE_SIZE as i32 == 0) as i32;
        let page_diff = final_page - self.cursor_page_index as i32;

        if page_diff > 0 {
            // forward
            let mut page_diff = page_diff as u32;
            'outer: loop {
                match &mut self.page_cursor {
                    Some(cursor) => match cursor {
                        PageCursor::FileInline { index } => {
                            let max_forward = FILE_PAGES - *index - 1;
                            if max_forward >= page_diff {
                                *index += page_diff;
                                self.file_data_page =
                                    self.fs.get_inline_file_data_page(self.file, *index);
                                break;
                            } else {
                                *cursor = PageCursor::PageList {
                                    index: 0,
                                    page_list_index: 0,
                                    page_list: self.fs.get_next_pages(self.file),
                                };
                                page_diff -= max_forward + 1;
                            }
                        }
                        PageCursor::PageList {
                            index,
                            page_list_index,
                            page_list,
                        } => loop {
                            let max_forward = PAGE_LIST_PAGES - *index - 1;
                            if max_forward >= page_diff {
                                *index += page_diff;
                                self.file_data_page =
                                    self.fs.get_page_list_file_data_page(*page_list, *index);
                                break 'outer;
                            } else {
                                *index = 0;
                                *page_list_index += 1;
                                *page_list = self.fs.get_next_pages(*page_list);
                                page_diff -= max_forward + 1;
                            }
                        },
                    },
                    None => {
                        self.page_cursor = Some(PageCursor::FileInline { index: 0 });
                        page_diff -= 1;
                    }
                }
            }
        } else {
            // page_diff <= 0
            // backward
            let mut page_diff = (-page_diff) as u32;
            'outer: loop {
                match &mut self.page_cursor {
                    Some(cursor) => match cursor {
                        PageCursor::FileInline { index } => {
                            if page_diff > *index {
                                debug_assert_eq!(page_diff - 1, *index);
                                self.page_cursor = None;
                            } else {
                                *index -= page_diff;
                                self.file_data_page =
                                    self.fs.get_inline_file_data_page(self.file, *index);
                            }
                            break;
                        }
                        PageCursor::PageList {
                            index,
                            page_list_index,
                            page_list,
                        } => loop {
                            if page_diff > *index {
                                if *page_list_index == 0 {
                                    page_diff -= *index + 1;
                                    *cursor = PageCursor::FileInline {
                                        index: FILE_PAGES - 1,
                                    };
                                    break;
                                } else {
                                    page_diff -= *index + 1;
                                    *index = PAGE_LIST_PAGES - 1;
                                    *page_list_index -= 1;
                                    *page_list = self.fs.get_prev_pages(*page_list);
                                }
                            } else {
                                *index -= page_diff;
                                self.file_data_page =
                                    self.fs.get_page_list_file_data_page(*page_list, *index);
                                break 'outer;
                            }
                        },
                    },
                    None => {
                        // this branch is when page_diff <= 0
                        // if page_cursor is none, then page_diff must be 0
                        debug_assert_eq!(page_diff, 0);
                        break;
                    }
                }
            }
        }
        let final_cursor_in_page = final_cursor as u32 % PAGE_SIZE;
        self.cursor_in_page = if final_cursor_in_page == 0 {
            PAGE_SIZE
        } else {
            final_cursor_in_page
        };
        self.cursor_page_index = final_page as u32;
        self.cursor = final_cursor as u32;
        Ok(())
    }

    pub fn seek_from_start(&mut self, pos: u32) -> Result<()> {
        let rel_pos = pos as i32 - self.cursor as i32;
        self.seek_from_current(rel_pos)
    }

    pub fn seek_from_end(&mut self, pos: u32) -> Result<()> {
        let pos = self.size() - pos;
        self.seek_from_start(pos)
    }

    pub fn name(&self, name_array: &mut NameArray) {
        let array = unsafe { name_array.array_mut() };
        let len = self.fs.get_name(self.file, array);
        unsafe {
            name_array.set_len(len as _);
        }
    }

    pub fn path(&self) -> String {
        let mut s = String::new();
        self.fs.get_path(self.file, &mut s);
        s
    }
}

impl<S: Storage> Drop for File<S> {
    fn drop(&mut self) {
        self.fs.decrement_in_use(self.file);
    }
}

fn size_to_pages(size: u32) -> u32 {
    (size + PAGE_SIZE - 1) / PAGE_SIZE
}

#[cfg(test)]
pub mod tests {
    extern crate std;

    use super::*;
    use crate::fs::tests::fs;
    use crate::{Dir, Storage, FILE_PAGES, PAGE_SIZE};
    use bobby_fs_common::Error;
    use core::mem::MaybeUninit;
    use rand::RngCore;
    use rand::{rngs::SmallRng, SeedableRng};
    use std::io::{Cursor, Read, Seek, SeekFrom, Write};
    use std::prelude::v1::*;
    use std::{format, vec};
    struct TestCursor<'a, S: Storage> {
        file: File<S>,
        cursor: Cursor<&'a mut Vec<u8>>,
    }

    impl<'a, S: Storage> TestCursor<'a, S> {
        pub fn create(dir: &Dir<S>, name: &str, vec: &'a mut Vec<u8>) -> Self {
            let file = dir.create_file(name).unwrap();
            let cursor = Cursor::new(vec);
            Self { file, cursor }
        }

        pub fn open(dir: &Dir<S>, name: &str, vec: &'a mut Vec<u8>) -> Self {
            let file = dir.open_file(name).unwrap();
            let cursor = Cursor::new(vec);
            Self { file, cursor }
        }

        pub fn read(&mut self, len: usize) {
            let mut buf_from_file = vec![MaybeUninit::zeroed(); len];
            self.file.read(&mut buf_from_file).unwrap();
            let buf_from_file = unsafe { MaybeUninit::slice_assume_init_ref(&buf_from_file) };
            let mut buf_from_cursor = vec![0; len];
            self.cursor.read_exact(&mut buf_from_cursor).unwrap();
            assert_eq!(buf_from_file, &*buf_from_cursor);
        }

        pub fn write(&mut self, buf: &[u8]) {
            self.file.write(buf).unwrap();
            self.cursor.write_all(buf).unwrap();
        }

        pub fn truncate(&mut self) {
            let pos = self.cursor.position();
            assert_eq!(self.file.position() as u64, pos);
            self.cursor.get_mut().resize(pos as usize, 0);
            self.file.truncate();
        }

        pub fn seek_from_start(&mut self, pos: u32) {
            self.file.seek_from_start(pos).unwrap();
            self.cursor.seek(SeekFrom::Start(pos as _)).unwrap();
        }

        pub fn seek_from_end(&mut self, pos: u32) {
            self.file.seek_from_end(pos).unwrap();
            self.cursor.seek(SeekFrom::End(pos as _)).unwrap();
        }

        pub fn size(&self) -> usize {
            let file_size = self.file.size();
            let cursor_size = self.cursor.get_ref().len();
            assert_eq!(file_size as usize, cursor_size);
            cursor_size
        }
    }

    #[test]
    fn double_open() {
        let fs = fs();
        let root = fs.root();
        let _file = root.create_file("a file".try_into().unwrap()).unwrap();
        let Err(err) = root.open_file("a file".try_into().unwrap()) else {
            panic!("Should be error");
        };
        assert_eq!(err, Error::InUse);
    }

    #[test]
    fn write_a_byte() {
        let fs = fs();
        let root = fs.root();

        fs.root().create_file("a byte".try_into().unwrap()).unwrap();
        let mut vec = vec![];
        for i in 0..=255u8 {
            {
                let mut file = TestCursor::open(&root, "a byte", &mut vec);
                file.write(&[i]);
            }
            {
                let mut file = TestCursor::open(&root, "a byte", &mut vec);
                file.read(1);
            }
        }
    }

    #[test]
    fn write_a_byte_files() {
        let fs = fs();
        let root = fs.root();

        let mut vecs = vec![vec![]; 256];
        for i in 0..=255u8 {
            let mut file = TestCursor::create(&root, &format!("file {}", i), &mut vecs[i as usize]);
            file.write(&[i]);
        }

        for i in 0..=255u8 {
            let mut file = TestCursor::open(&root, &format!("file {}", i), &mut vecs[i as usize]);
            file.read(1);
        }
    }

    #[test]
    fn write_a_random_page() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(354876234);
        let mut bytes = vec![0u8; PAGE_SIZE as usize];
        rng.fill_bytes(&mut bytes);

        let mut vec = vec![];
        {
            let mut file = TestCursor::create(&root, "random_page", &mut vec);
            file.write(&bytes);
        }
        {
            let mut file = TestCursor::open(&root, "random_page", &mut vec);
            file.read(bytes.len());
        }
    }

    #[test]
    fn write_a_random_inline_file() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(354876234);
        let mut bytes = vec![0u8; (PAGE_SIZE * FILE_PAGES) as usize];
        rng.fill_bytes(&mut bytes);

        let mut vec = vec![];
        {
            let mut file = TestCursor::create(&root, "inline_file", &mut vec);
            file.write(&bytes);
        }
        {
            let mut file = TestCursor::open(&root, "inline_file", &mut vec);
            file.read(bytes.len());
        }
    }

    #[test]
    fn write_random_inline_files() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(6734534564);
        let mut vecs = vec![vec![]; (FILE_PAGES + 1) as usize];
        for i in 0..=FILE_PAGES {
            let mut bytes = vec![0u8; (PAGE_SIZE * i) as usize];
            rng.fill_bytes(&mut bytes);
            let mut file = TestCursor::create(
                &root,
                &format!("random_inline_file {}", i),
                &mut vecs[i as usize],
            );
            file.write(&bytes);
        }
        for i in 0..=FILE_PAGES {
            let mut file = TestCursor::open(
                &root,
                &format!("random_inline_file {}", i),
                &mut vecs[i as usize],
            );
            file.read((PAGE_SIZE * i) as usize);
        }
    }

    #[test]
    fn write_huge_file() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(745623456);
        let mut bytes = vec![0u8; (PAGE_SIZE * 10000) as usize];
        rng.fill_bytes(&mut bytes);
        let mut vec = vec![];
        {
            let mut file = TestCursor::create(&root, "huge_file", &mut vec);
            file.write(&bytes);
        }
        {
            let mut file = TestCursor::open(&root, "huge_file", &mut vec);
            file.read(bytes.len());
        }
    }

    #[test]
    fn rewrite_huge_file_parts() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(45678567);
        let mut bytes = vec![0u8; (PAGE_SIZE * 10000) as usize];
        rng.fill_bytes(&mut bytes);
        let mut vec = vec![];
        {
            let mut file = TestCursor::create(&root, "huge_file", &mut vec);
            let mut remaining = bytes.len();
            let mut written = 0;
            while remaining > 0 {
                let size = ((rng.next_u32() % 16) as usize).min(remaining);

                file.write(&mut bytes[written..written + size]);

                remaining -= size;
                written += size;
            }
        }
        {
            let mut file = TestCursor::open(&root, "huge_file", &mut vec);
            let mut remaining = bytes.len();
            while remaining > 0 {
                let size = ((rng.next_u32() % 16) as usize).min(remaining);

                file.read(size);

                remaining -= size;
            }
        }
        rng.fill_bytes(&mut bytes);
        {
            let mut file = TestCursor::open(&root, "huge_file", &mut vec);
            let mut remaining = bytes.len();
            let mut written = 0;
            while remaining > 0 {
                let size = ((rng.next_u32() % 16) as usize).min(remaining);

                file.write(&mut bytes[written..written + size]);

                remaining -= size;
                written += size;
            }
        }
        {
            let mut file = TestCursor::open(&root, "huge_file", &mut vec);
            let mut remaining = bytes.len();
            while remaining > 0 {
                let size = ((rng.next_u32() % 16) as usize).min(remaining);

                file.read(size);

                remaining -= size;
            }
        }
    }

    #[test]
    fn truncate_empty() {
        let fs = fs();
        let root = fs.root();
        let mut vec = vec![];
        let mut file = TestCursor::create(&root, "empty", &mut vec);
        file.truncate();
    }

    #[test]
    fn truncate_page() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(7356734564);
        let mut bytes = vec![0u8; PAGE_SIZE as usize];
        rng.fill_bytes(&mut bytes);
        let mut vec = vec![];
        {
            let mut file = TestCursor::create(&root, "page", &mut vec);
            file.write(&bytes);
            assert_eq!(file.file.size(), PAGE_SIZE);
        }
        {
            let mut file = TestCursor::open(&root, "page", &mut vec);
            file.read(200);
            file.truncate();
            assert_eq!(file.file.size(), 200);
        }
        {
            let mut file = TestCursor::open(&root, "page", &mut vec);
            file.read(200);
            let mut buf = [MaybeUninit::zeroed(); 1];
            let result = file.file.read(&mut buf);
            assert_eq!(result, Err(Error::Eof));
        }
    }

    #[test]
    fn truncate_huge_file_full() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(734564573456);
        let mut bytes = vec![0u8; (PAGE_SIZE * 10000) as usize];
        rng.fill_bytes(&mut bytes);
        let free;
        let mut vec = vec![];
        {
            let mut file = TestCursor::create(&root, "huge_file", &mut vec);
            free = fs.get_free_page_count();
            file.write(&bytes);
        }
        {
            let mut file = TestCursor::open(&root, "huge_file", &mut vec);
            file.read(bytes.len());
        }
        {
            let mut file = TestCursor::open(&root, "huge_file", &mut vec);
            file.truncate();
        }
        {
            let file = TestCursor::open(&root, "huge_file", &mut vec);
            assert_eq!(file.file.size(), 0);
            assert_eq!(fs.get_free_page_count(), free);
        }
    }

    #[test]
    fn double_read_page() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(85677357345);
        let mut bytes = vec![0u8; PAGE_SIZE as usize];
        rng.fill_bytes(&mut bytes);
        let mut vec = vec![];
        {
            let mut file = TestCursor::create(&root, "page", &mut vec);
            file.write(&bytes);
        }
        {
            let mut file = TestCursor::open(&root, "page", &mut vec);
            file.read(bytes.len());
            file.seek_from_start(0);
            file.read(bytes.len());
        }
    }

    #[test]
    fn double_read_huge() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(74562345763);
        let mut bytes = vec![0u8; (PAGE_SIZE * 10000) as usize];
        rng.fill_bytes(&mut bytes);
        let mut vec = vec![];
        {
            let mut file = TestCursor::create(&root, "huge", &mut vec);
            file.write(&bytes);
        }
        {
            let mut file = TestCursor::open(&root, "huge", &mut vec);
            file.read(bytes.len());
            file.seek_from_start(0);
            file.read(bytes.len());
        }
    }

    #[test]
    fn append_huge() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(45672456345);
        let mut vec = vec![];
        TestCursor::create(&root, "huge_append", &mut vec);
        for _ in 0..250 {
            let n_bytes = rng.next_u32() % 1000;
            let mut bytes = vec![0u8; n_bytes as usize];
            rng.fill_bytes(&mut bytes);
            let mut file = TestCursor::open(&root, "huge_append", &mut vec);
            file.seek_from_end(0);
            file.write(&bytes);
            file.seek_from_start(0);
            file.read(file.size());
        }
    }

    #[test]
    fn random_read() {
        let fs = fs();
        let root = fs.root();
        let mut rng = SmallRng::seed_from_u64(364534563456);
        let mut bytes = vec![0u8; (PAGE_SIZE * 250) as usize];
        rng.fill_bytes(&mut bytes);
        let mut vec = vec![];
        {
            let mut file = TestCursor::create(&root, "random", &mut vec);
            file.write(&bytes);
        }
        let mut file = TestCursor::open(&root, "random", &mut vec);
        for _ in 0..1000 {
            let pos = rng.next_u32() % bytes.len() as u32;
            let n_bytes = (rng.next_u32() % 1000).min(bytes.len() as u32 - pos);
            file.seek_from_start(pos);
            file.read(n_bytes as usize);
        }
    }
}
