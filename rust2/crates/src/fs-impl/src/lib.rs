#![no_std]
pub use bobby_fs;
use bobby_fs::Result;
use storage::FsStorage;

pub mod export;
mod storage;

pub type FileSystem = bobby_fs::Filesystem<FsStorage>;

static FS: FileSystem = bobby_fs::Filesystem::new(FsStorage);

pub type BobbyDir = bobby_fs::Dir<FsStorage>;
pub type BobbyFile = bobby_fs::File<FsStorage>;
pub type BobbyDirIter = bobby_fs::DirIter<FsStorage>;
pub type BobbyEntry = bobby_fs::Entry<FsStorage>;

static mut ERROR: fs::Error = bobby_fs::Error::InvalidInput; // just a random error to initialize

pub fn fs() -> &'static FileSystem {
    &FS
}

fn result_into_option<T>(result: Result<T>) -> Option<T> {
    match result {
        Ok(t) => Some(t),
        Err(err) => {
            unsafe {
                ERROR = err;
            }
            None
        }
    }
}

fn result_into_bool(result: Result<()>) -> bool {
    match result {
        Ok(_) => false,
        Err(err) => {
            unsafe {
                ERROR = err;
            }
            true
        }
    }
}
