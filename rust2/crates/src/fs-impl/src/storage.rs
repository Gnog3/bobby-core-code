use bobby_fs::Storage;

const FS_LEN: usize = 4 * 1024 * 1024;
const FS_PTR: *mut u8 = (1024 * 1024 * 12) as _;

#[derive(Debug, Clone, Copy)]
pub struct FsStorage;

impl Storage for FsStorage {
    fn read(&self, offset: u32, buf: &mut [core::mem::MaybeUninit<u8>]) {
        unsafe {
            FS_PTR
                .add(offset as _)
                .copy_to_nonoverlapping(buf.as_mut_ptr() as *mut u8, buf.len());
        }
    }

    fn write(&self, offset: u32, buf: &[u8]) {
        unsafe {
            FS_PTR
                .add(offset as _)
                .copy_from_nonoverlapping(buf.as_ptr(), buf.len());
        }
    }

    fn capacity(&self) -> u32 {
        FS_LEN as _
    }
}
