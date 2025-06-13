use core::{hint::unreachable_unchecked, mem};

use alloc::string::String;

use crate::{
    dir_iter::DirIter,
    internal_fs::{InternalFilesystem, PageKind},
    storage::Storage,
    Error, File, NameArray, Offset, Result, MAX_LEN_NAME,
};

pub struct Dir<S: Storage> {
    fs: InternalFilesystem<S>,
    pub(crate) dir: Offset,
}

impl<S: Storage> Dir<S> {
    pub(crate) fn new(fs: InternalFilesystem<S>, page: Offset) -> Self {
        fs.increment_in_use(page);
        Self { fs, dir: page }
    }

    pub(crate) fn try_new(fs: InternalFilesystem<S>, page: Offset) -> Result<Self> {
        let kind = fs.get_page_kind(page);
        if kind != PageKind::Directory {
            return Err(Error::NotADirectory);
        }
        Ok(Self::new(fs, page))
    }

    fn create(fs: InternalFilesystem<S>, dir: Offset, name: &str) -> Result<Self> {
        if name.is_empty() || name.len() as u32 > MAX_LEN_NAME {
            return Err(Error::InvalidName);
        }
        fs.get_iter_in_use(dir)?;
        let page = fs.add_entry(dir, name, PageKind::Directory)?;
        Ok(Self::new(fs, page))
    }

    // fn open(fs: InternalFilesystem<S>, dir: Offset, name: &str) -> Result<Self> {
    //     let page = fs.find_entry(dir, name)?;
    //     Self::try_new(fs, page)
    // }

    fn get_dir_page_at(&self, path: &str) -> Result<Offset> {
        let mut current_dir = self.dir;
        for dir_name in path.split('/') {
            if dir_name == ".." {
                let parent = self.fs.get_parent(current_dir);
                if parent != 0 {
                    current_dir = parent;
                }
                continue;
            }
            current_dir = self.fs.find_entry(current_dir, dir_name)?;
            let kind = self.fs.get_page_kind(current_dir);
            if kind != PageKind::Directory {
                return Err(Error::NotADirectory);
            }
        }
        Ok(current_dir)
    }

    fn get_dir_and_name<'a>(&self, path: &'a str) -> Result<(Offset, &'a str)> {
        if let Some((dir_path, name)) = path.rsplit_once('/') {
            Ok((self.get_dir_page_at(dir_path)?, name))
        } else {
            Ok((self.dir, path))
        }
    }

    pub fn create_file(&self, path: &str) -> Result<File<S>> {
        let (dir, name) = self.get_dir_and_name(path)?;
        File::create(self.fs.clone(), dir, name)
    }

    pub fn open_file(&self, path: &str) -> Result<File<S>> {
        let (dir, name) = self.get_dir_and_name(path)?;
        File::open(self.fs.clone(), dir, name)
    }

    pub fn create_dir(&self, path: &str) -> Result<Dir<S>> {
        let (dir, name) = self.get_dir_and_name(path)?;
        Self::create(self.fs.clone(), dir, name)
    }

    pub fn open_dir(&self, path: &str) -> Result<Dir<S>> {
        let page = self.get_dir_page_at(path)?;
        Self::try_new(self.fs.clone(), page)
    }

    pub fn create_or_open_file(&self, path: &str) -> Result<File<S>> {
        let (dir, name) = self.get_dir_and_name(path)?;
        match File::open(self.fs.clone(), dir, name) {
            Ok(file) => return Ok(file),
            Err(Error::NotFound) => {}
            Err(err) => return Err(err),
        }
        File::create(self.fs.clone(), dir, name)
    }

    pub fn remove(&self, path: &str) -> Result<()> {
        let (dir, name) = self.get_dir_and_name(path)?;
        self.fs.get_iter_in_use(dir)?;
        self.fs.remove_entry(dir, name, |entry| {
            if self.fs.get_in_use(entry) != 0 {
                return Err(Error::InUse);
            }
            match self.fs.get_page_kind(entry) {
                PageKind::File => {
                    let mut file = File::new(self.fs.clone(), entry);
                    file.truncate();
                    mem::forget(file);
                }
                PageKind::Directory => {
                    if self.fs.get_contents(entry) != 0 {
                        return Err(Error::DirectoryNotEmpty);
                    }
                }
                _ => unsafe { unreachable_unchecked() },
            }
            Ok(())
        })
    }

    pub fn iter_contents(&self) -> Result<DirIter<S>> {
        DirIter::new(self.fs.clone(), self.dir)
    }

    pub fn name(&self, name_array: &mut NameArray) {
        let array = unsafe { name_array.array_mut() };
        let len = self.fs.get_name(self.dir, array);
        unsafe {
            name_array.set_len(len as _);
        }
    }

    pub fn path(&self) -> String {
        let mut s = String::new();
        self.fs.get_path(self.dir, &mut s);
        s
    }
}

impl<S: Storage> Drop for Dir<S> {
    fn drop(&mut self) {
        self.fs.decrement_in_use(self.dir);
    }
}

#[cfg(test)]
pub mod tests {
    extern crate std;

    use crate::fs::tests::{fs, get_contents, TestEntry};
    use std::prelude::v1::*;
    use std::vec;

    #[test]
    fn dir_operations() {
        let fs = fs();
        let root = fs.root();
        assert_eq!(get_contents(&root), vec![]);
        root.create_dir("My dir").unwrap();
        root.create_dir("My dir/my other dir").unwrap();
        root.create_file("My dir/my file.txt").unwrap();
        root.create_file("My dir/my other dir/my other file.txt")
            .unwrap();
        root.create_file("My dir/my other dir/../my other dir/my other file 2.txt")
            .unwrap();
        assert_eq!(
            get_contents(&root),
            vec![TestEntry::Directory("My dir".into())]
        );
        assert_eq!(
            get_contents(&root.open_dir("My dir").unwrap()),
            vec![
                TestEntry::File("my file.txt".into()),
                TestEntry::Directory("my other dir".into()),
            ]
        );
        assert_eq!(
            get_contents(&root.open_dir("My dir/my other dir".into()).unwrap()),
            vec![
                TestEntry::File("my other file 2.txt".into()),
                TestEntry::File("my other file.txt".into()),
            ]
        )
    }

    #[test]
    fn remove_file() {
        let fs = fs();
        let root = fs.root();
        assert_eq!(get_contents(&root), vec![]);
        root.create_file("abc").unwrap();
        assert_eq!(get_contents(&root), vec![TestEntry::File("abc".into())]);
        root.remove("abc").unwrap();
        assert_eq!(get_contents(&root), vec![]);
        root.remove("doesn't exist").unwrap_err();
    }

    #[test]
    fn path() {
        let fs = fs();
        let root = fs.root();
        assert_eq!(root.path(), "/");
        let dir = root.create_dir("My dir").unwrap();
        assert_eq!(dir.path(), "/My dir");
        let dir = root.create_dir("My dir/my other dir").unwrap();
        assert_eq!(dir.path(), "/My dir/my other dir");
    }
}
