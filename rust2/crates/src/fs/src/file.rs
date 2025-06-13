use alloc::string::String;
use blib::std_types::{RSlice, RSliceMut, RString};

use core::{mem::MaybeUninit, ptr::NonNull};

use crate::{error_bool, NameArray, Result};

unsafe extern "C" {
    safe fn fs_file_truncate(file: File);
    safe fn fs_file_read(file: File, buf: RSliceMut<MaybeUninit<u8>>) -> bool;
    safe fn fs_file_write(file: File, buf: RSlice<u8>) -> bool;
    safe fn fs_file_size(file: File) -> u32;
    safe fn fs_file_name(file: File, name_array: &mut NameArray);
    safe fn fs_file_path(file: File) -> RString;

    safe fn fs_file_drop(file: File);
}

#[repr(transparent)]
pub struct File(NonNull<()>);

impl File {
    pub unsafe fn from_ptr(ptr: NonNull<()>) -> Self {
        File(ptr)
    }

    pub unsafe fn copy(&self) -> Self {
        File(self.0)
    }

    pub fn truncate(&mut self) {
        fs_file_truncate(File(self.0));
    }

    pub fn read(&mut self, buf: &mut [MaybeUninit<u8>]) -> Result<()> {
        let error = fs_file_read(File(self.0), buf.into());
        error_bool(error)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        let error = fs_file_write(File(self.0), buf.into());
        error_bool(error)
    }

    pub fn size(&self) -> u32 {
        fs_file_size(File(self.0))
    }

    pub fn name(&self, name_array: &mut NameArray) {
        fs_file_name(File(self.0), name_array);
    }

    pub fn path(&self) -> String {
        RString::to_string(fs_file_path(File(self.0)))
    }
}

impl Drop for File {
    fn drop(&mut self) {
        fs_file_drop(File(self.0));
    }
}
