use core::{marker::PhantomData, slice};

#[repr(C)]
pub struct RSlice<'a, T> {
    data: *const T,
    length: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RSlice<'a, T> {
    #[inline(always)]
    pub const unsafe fn from_raw_parts(ptr_: *const T, len: usize) -> Self {
        Self {
            data: ptr_,
            length: len,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_slice(&self) -> &'a [T] {
        unsafe { slice::from_raw_parts(self.data, self.length) }
    }

    #[inline(always)]
    pub const fn from_slice(slice: &'a [T]) -> Self {
        unsafe { RSlice::from_raw_parts(slice.as_ptr(), slice.len()) }
    }
}

impl<'a, T> From<&'a [T]> for RSlice<'a, T> {
    fn from(value: &'a [T]) -> Self {
        RSlice::from_slice(value)
    }
}
