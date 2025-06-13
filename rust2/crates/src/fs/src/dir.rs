use core::ptr::NonNull;

use crate::{error_bool, fs_get_error, DirIter, File, NameArray, Result};
use alloc::string::String;
use blib::std_types::{RStr, RString};

unsafe extern "C" {
    // root dir can't fail
    safe fn fs_root() -> Dir;
    safe fn fs_dir_open_dir(dir: Dir, path: RStr) -> Option<Dir>;
    safe fn fs_dir_open_file(dir: Dir, path: RStr) -> Option<File>;
    safe fn fs_dir_create_file(dir: Dir, path: RStr) -> Option<File>;
    safe fn fs_dir_create_dir(dir: Dir, path: RStr) -> Option<Dir>;
    safe fn fs_dir_remove(dir: Dir, path: RStr) -> bool;
    safe fn fs_dir_iter_contents(dir: Dir) -> Option<DirIter>;
    safe fn fs_dir_name(dir: Dir, name_array: &mut NameArray);
    safe fn fs_dir_path(dir: Dir) -> RString;
    
    safe fn fs_dir_drop(dir: Dir);
}

#[repr(transparent)]
pub struct Dir(NonNull<()>);

impl Dir {
    pub unsafe fn from_ptr(ptr: NonNull<()>) -> Self {
        Dir(ptr)
    }

    pub fn root() -> Self {
        fs_root()
    }

    pub fn open_dir(&self, path: &str) -> Result<Self> {
        let dir = fs_dir_open_dir(Dir(self.0), path.into());
        dir.ok_or_else(|| fs_get_error())
    }

    pub fn open_file(&self, path: &str) -> Result<File> {
        let file = fs_dir_open_file(Dir(self.0), path.into());
        file.ok_or_else(|| fs_get_error())
    }

    pub fn create_file(&self, path: &str) -> Result<File> {
        let file = fs_dir_create_file(Dir(self.0), path.into());
        file.ok_or_else(|| fs_get_error())
    }

    pub fn create_dir(&self, path: &str) -> Result<Self> {
        let dir = fs_dir_create_dir(Dir(self.0), path.into());
        dir.ok_or_else(|| fs_get_error())
    }

    pub fn remove(&self, path: &str) -> Result<()> {
        let error = fs_dir_remove(Dir(self.0), path.into());
        error_bool(error)
    }

    pub fn iter_contents(&self) -> Result<DirIter> {
        let dir_iter = fs_dir_iter_contents(Dir(self.0));
        dir_iter.ok_or_else(|| fs_get_error())
    }

    pub fn name(&self, name_array: &mut NameArray) {
        fs_dir_name(Dir(self.0), name_array);
    }

    pub fn path(&self) -> String {
        fs_dir_path(Dir(self.0)).to_string()
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        fs_dir_drop(Dir(self.0));
    }
}
