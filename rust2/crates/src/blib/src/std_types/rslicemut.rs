use core::{marker::PhantomData, slice};

#[repr(C)]
pub struct RSliceMut<'a, T> {
    data: *mut T,
    length: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> RSliceMut<'a, T> {
    #[inline(always)]
    pub const unsafe fn from_raw_parts_mut(ptr_: *mut T, len: usize) -> Self {
        Self {
            data: ptr_,
            length: len,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data, self.length) }
    }

    #[inline(always)]
    pub const fn from_slice_mut(slice: &'a mut [T]) -> Self {
        unsafe { Self::from_raw_parts_mut(slice.as_mut_ptr(), slice.len()) }
    }
}

impl<'a, T> From<&'a mut [T]> for RSliceMut<'a, T> {
    fn from(value: &'a mut [T]) -> Self {
        RSliceMut::from_slice_mut(value)
    }
}
