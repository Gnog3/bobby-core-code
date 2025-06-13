#![no_std]
extern crate alloc;

mod dir;
mod dir_iter;
mod entry;
mod file;

pub use bobby_fs_common::{Error, NameArray, Result};
pub use dir::Dir;
pub use dir_iter::DirIter;
pub use entry::Entry;
pub use file::File;

unsafe extern "C" {
    safe fn fs_get_error() -> Error;
}

fn error_bool(error: bool) -> Result<()> {
    if error {
        Err(fs_get_error())
    } else {
        Ok(())
    }
}
