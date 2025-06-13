use core::mem;

use crate::{dir::Dir, internal_fs::InternalFilesystem, storage::Storage, DirIter, Offset};

pub struct Filesystem<S> {
    fs: InternalFilesystem<S>,
}

impl<S: Storage> Filesystem<S> {
    pub const fn new(storage: S) -> Self {
        Self {
            fs: InternalFilesystem::new(storage),
        }
    }

    pub fn format(&self) {
        self.fs.format();
    }

    pub fn get_total_page_count(&self) -> u32 {
        self.fs.get_total_page_count()
    }

    pub fn get_free_page_count(&self) -> u32 {
        self.fs.get_free_page_count()
    }

    pub fn root(&self) -> Dir<S> {
        Dir::new(self.fs.clone(), self.fs.root())
    }

    pub fn reset_in_use(&self) {
        fn reset<S: Storage>(s: &Filesystem<S>, dir: Offset) {
            s.fs.reset_in_use(dir);
            let mut iter = DirIter::new_raw(s.fs.clone(), dir);
            for entry in &mut iter {
                if entry.is_dir() {
                    reset(s, entry.entry);
                } else {
                    s.fs.reset_in_use(entry.entry);
                }
            }
            mem::forget(iter);
        }

        reset(self, self.fs.root())
    }
}

#[cfg(test)]
pub mod tests {
    extern crate std;

    use bobby_fs_common::NameArray;

    use super::*;
    use crate::storage::tests::HeapStorage;
    use crate::Dir;
    use std::prelude::v1::*;

    #[derive(Debug, PartialEq, Eq)]
    pub enum TestEntry {
        Directory(String),
        File(String),
    }

    pub fn fs() -> Filesystem<HeapStorage> {
        let fs = Filesystem::new(HeapStorage::new());
        fs.format();
        fs
    }

    pub fn get_contents<S: Storage>(dir: &Dir<S>) -> Vec<TestEntry> {
        dir.iter_contents()
            .unwrap()
            .map(|entry| {
                let mut name_array = NameArray::new();
                entry.name(&mut name_array);
                let string = name_array.as_str().unwrap().into();
                if entry.is_dir() {
                    TestEntry::Directory(string)
                } else {
                    TestEntry::File(string)
                }
            })
            .collect()
    }
}
