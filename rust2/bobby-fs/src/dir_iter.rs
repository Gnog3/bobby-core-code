use alloc::string::String;
use bobby_fs_common::NameArray;

use crate::{
    internal_fs::{InternalFilesystem, PageKind},
    stack_vec::StackVec,
    Dir, File, Offset, Result, Storage,
};

pub struct Entry<S> {
    fs: InternalFilesystem<S>,
    pub(crate) entry: Offset,
}

impl<S: Storage> Entry<S> {
    pub fn is_file(&self) -> bool {
        self.fs.get_page_kind(self.entry) == PageKind::File
    }

    pub fn is_dir(&self) -> bool {
        self.fs.get_page_kind(self.entry) == PageKind::Directory
    }

    pub fn open_as_file(&self) -> Result<File<S>> {
        File::try_new(self.fs.clone(), self.entry)
    }

    pub fn open_as_dir(&self) -> Result<Dir<S>> {
        Dir::try_new(self.fs.clone(), self.entry)
    }

    pub fn name(&self, name_array: &mut NameArray) {
        let array = unsafe { name_array.array_mut() };
        let len = self.fs.get_name(self.entry, array);
        unsafe {
            name_array.set_len(len as _);
        }
    }

    pub fn path(&self) -> String {
        let mut s = String::new();
        self.fs.get_path(self.entry, &mut s);
        s
    }
}

pub struct DirIter<S: Storage> {
    fs: InternalFilesystem<S>,
    dir: Offset,
    stack: StackVec<Offset, 20>, // up to a million entries
    current: Offset,             // may be zero
}

impl<S: Storage> DirIter<S> {
    pub(crate) fn new(fs: InternalFilesystem<S>, dir: Offset) -> Result<Self> {
        fs.try_set_iter_in_use(dir)?;
        Ok(Self::new_raw(fs, dir))
    }

    pub(crate) fn new_raw(fs: InternalFilesystem<S>, dir: Offset) -> Self {
        let contents = fs.get_contents(dir);

        if contents != 0 {
            let mut iter = Self {
                fs,
                dir,
                stack: StackVec::new(),
                current: contents,
            };
            iter.go_left_till_the_end();
            iter
        } else {
            Self {
                fs,
                dir,
                stack: StackVec::new(),
                current: 0,
            }
        }
    }

    // assumes that current != 0
    #[inline(always)]
    fn go_left_till_the_end(&mut self) {
        loop {
            let next = self.fs.get_left_child(self.current);
            if next == 0 {
                break;
            }
            self.stack.push(self.current);
            self.current = next;
        }
    }

    // assumes that current != 0
    #[inline(always)]
    fn try_go_right(&mut self) -> bool {
        let next = self.fs.get_right_child(self.current);
        if next == 0 {
            return false;
        }
        self.current = next;
        true
    }

    #[inline(always)]
    fn go_up_till_left(&mut self) {
        self.current = self.stack.pop().unwrap_or(0);
    }
}

impl<S: Storage> Iterator for DirIter<S> {
    type Item = Entry<S>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            return None;
        }
        let ret = self.current;

        if self.try_go_right() {
            self.go_left_till_the_end();
        } else {
            self.go_up_till_left();
        }

        Some(Entry {
            fs: self.fs.clone(),
            entry: ret,
        })
    }
}

impl<S: Storage> Drop for DirIter<S> {
    fn drop(&mut self) {
        self.fs.unset_iter_in_use(self.dir);
    }
}

#[cfg(test)]
pub mod tests {
    extern crate std;
    use super::*;
    use crate::fs::tests::fs;
    use crate::Error;
    use std::{format, prelude::v1::*, vec};

    #[test]
    fn iter() {
        for n_files in 0..256 {
            let fs = fs();
            let root = fs.root();
            let mut files = vec![];
            for i in 0..n_files {
                let name = format!("File {}", i);
                root.create_file(name.as_str().try_into().unwrap()).unwrap();
                files.push(name);
            }
            files.sort();
            let mut observed_files: Vec<String> = vec![];
            for entry in root.iter_contents().unwrap() {
                let mut name = NameArray::new();
                entry.name(&mut name);
                observed_files.push(name.as_str().unwrap().into());
            }
            assert_eq!(files, observed_files);
        }
    }

    #[test]
    fn double_iter() {
        let fs = fs();
        let root = fs.root();
        let _iter = root.iter_contents().unwrap();
        if let Err(err) = root.iter_contents() {
            assert_eq!(err, Error::InUse);
        } else {
            panic!("Should be err");
        }
    }
}
