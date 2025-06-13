use core::mem::{self, ManuallyDrop};

use alloc::string::String;

use super::RVec;

#[repr(transparent)]
pub struct RString {
    inner: ManuallyDrop<RVec<u8>>,
}

impl RString {
    pub fn from_string(s: String) -> RString {
        Self {
            inner: ManuallyDrop::new(RVec::from_vec(s.into_bytes())),
        }
    }

    pub fn to_string(mut self) -> String {
        let inner = unsafe { ManuallyDrop::take(&mut self.inner) };
        let s = unsafe { String::from_utf8_unchecked(inner.to_vec()) };
        mem::forget(self);
        s
    }
}

impl Drop for RString {
    fn drop(&mut self) {
        panic!("RString should not be dropped");
    }
}
