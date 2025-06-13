use core::ptr::NonNull;

use crate::Entry;

unsafe extern "C" {
    safe fn fs_dir_iter_next(dir_iter: DirIter) -> Option<Entry>;

    safe fn fs_dir_iter_drop(dir_iter: DirIter);
}

#[repr(transparent)]
pub struct DirIter(NonNull<()>);

impl DirIter {
    pub unsafe fn from_ptr(ptr: NonNull<()>) -> Self {
        DirIter(ptr)
    }
}

impl Iterator for DirIter {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        fs_dir_iter_next(DirIter(self.0))
    }
}

impl Drop for DirIter {
    fn drop(&mut self) {
        fs_dir_iter_drop(DirIter(self.0));
    }
}
