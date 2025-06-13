use core::mem::MaybeUninit;

pub struct StackVec<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    len: usize,
}

impl<T: Copy, const N: usize> StackVec<T, N> {
    pub fn new() -> Self {
        Self {
            array: [MaybeUninit::uninit(); N],
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        // really, up to a million entries is enough
        let x = unsafe { self.array.get_unchecked_mut(self.len) };
        *x = MaybeUninit::new(value);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe {
            let uninit = *self.array.get_unchecked(self.len);
            Some(uninit.assume_init())
        }
    }
}
