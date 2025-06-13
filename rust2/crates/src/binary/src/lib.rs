#![no_std]
extern crate alloc;

use core::ptr::NonNull;

use alloc::string::String;
use blib::std_types::{RResult, RString};
use fs::File;

unsafe extern "C" {
    safe fn binary_from_file(file: File) -> RResult<Binary, RString>;
    safe fn binary_run(binary: Binary);

    safe fn binary_drop(binary: Binary);
}

#[repr(transparent)]
pub struct Binary(NonNull<()>);

impl Binary {
    pub unsafe fn from_ptr(ptr: NonNull<()>) -> Self {
        Self(ptr)
    }

    pub fn from_file(file: &mut File) -> Result<Binary, String> {
        let result = binary_from_file(unsafe { file.copy() });
        match result {
            RResult::ROk(binary) => Ok(binary),
            RResult::RErr(err) => Err(err.to_string()),
        }
    }

    pub fn run(&self) {
        binary_run(Binary(self.0));
    }
}

impl Drop for Binary {
    fn drop(&mut self) {
        binary_drop(Binary(self.0));
    }
}
