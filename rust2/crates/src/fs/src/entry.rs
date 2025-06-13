use core::ptr::NonNull;

use alloc::string::String;
use blib::std_types::RString;

use crate::{fs_get_error, Dir, File, NameArray, Result};

unsafe extern "C" {
    safe fn fs_entry_is_file(entry: Entry) -> bool;
    safe fn fs_entry_is_dir(entry: Entry) -> bool;
    safe fn fs_entry_open_as_file(entry: Entry) -> Option<File>;
    safe fn fs_entry_open_as_dir(entry: Entry) -> Option<Dir>;
    safe fn fs_entry_name(entry: Entry, name_array: &mut NameArray);
    safe fn fs_entry_path(entry: Entry) -> RString;

    safe fn fs_entry_drop(entry: Entry);
}

#[repr(transparent)]
pub struct Entry(NonNull<()>);

impl Entry {
    pub unsafe fn from_ptr(ptr: NonNull<()>) -> Self {
        Entry(ptr)
    }

    pub fn is_file(&self) -> bool {
        fs_entry_is_file(Entry(self.0))
    }

    pub fn is_dir(&self) -> bool {
        fs_entry_is_dir(Entry(self.0))
    }

    pub fn open_as_file(&self) -> Result<File> {
        let file = fs_entry_open_as_file(Entry(self.0));
        file.ok_or_else(|| fs_get_error())
    }

    pub fn open_as_dir(&self) -> Result<Dir> {
        let dir = fs_entry_open_as_dir(Entry(self.0));
        dir.ok_or_else(|| fs_get_error())
    }

    pub fn name(&self, name_array: &mut NameArray) {
        fs_entry_name(Entry(self.0), name_array);
    }

    pub fn path(&self) -> String {
        RString::to_string(fs_entry_path(Entry(self.0)))
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        fs_entry_drop(Entry(self.0));
    }
}
