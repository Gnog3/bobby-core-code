use core::str;

use super::RSlice;

#[repr(C)]
pub struct RStr<'a> {
    inner: RSlice<'a, u8>,
}

impl<'a> RStr<'a> {
    #[inline(always)]
    pub const unsafe fn from_raw_parts(ptr_: *const u8, len: usize) -> Self {
        Self {
            inner: unsafe { RSlice::from_raw_parts(ptr_, len) },
        }
    }

    #[inline(always)]
    pub const fn from_str(s: &'a str) -> Self {
        unsafe { Self::from_raw_parts(s.as_ptr(), s.len()) }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &'a str {
        unsafe { str::from_utf8_unchecked(self.inner.as_slice()) }
    }
}

impl<'a> From<&'a str> for RStr<'a> {
    fn from(value: &'a str) -> Self {
        RStr::from_str(value)
    }
}
