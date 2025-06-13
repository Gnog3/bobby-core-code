#![no_std]
#![feature(maybe_uninit_as_bytes)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_write_slice)]

extern crate alloc;

mod dir;
mod dir_iter;
mod file;
mod fs;
mod internal_fs;
mod stack_vec;
mod storage;

pub use bobby_fs_common::{Error, NameArray, Result, MAX_LEN_NAME};
pub use dir::Dir;
pub use dir_iter::{DirIter, Entry};
pub use file::File;
pub use fs::Filesystem;
pub use storage::Storage;

pub const PAGE_SIZE: u32 = 512;
pub const FILE_PAGES: u32 = 96;
pub const PAGE_LIST_PAGES: u32 = 126;

type Offset = u32;
