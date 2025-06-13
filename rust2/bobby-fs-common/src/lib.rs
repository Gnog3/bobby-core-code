#![no_std]
#![feature(maybe_uninit_slice)]

use core::{mem::MaybeUninit, str};

pub const MAX_LEN_NAME: u32 = 92;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Error {
    AlreadyExists,
    NotADirectory,
    IsADirectory,
    StorageFull,
    NotFound,
    InUse,
    Eof,
    InvalidInput,
    InvalidName,
    DirectoryNotEmpty,
}

impl Error {
    pub fn as_str(self) -> &'static str {
        match self {
            Error::AlreadyExists => "Already exists",
            Error::NotADirectory => "Not a directory",
            Error::IsADirectory => "Is a directory",
            Error::StorageFull => "Storage full",
            Error::NotFound => "Not found",
            Error::InUse => "In use",
            Error::Eof => "Eof",
            Error::InvalidInput => "Invalid input",
            Error::InvalidName => "Invalid name",
            Error::DirectoryNotEmpty => "Directory not empty",
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

#[repr(C)]
pub struct NameArray {
    array: [MaybeUninit<u8>; MAX_LEN_NAME as _],
    len: i32,
}

impl NameArray {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            array: [MaybeUninit::uninit(); MAX_LEN_NAME as _],
            len: -1,
        }
    }

    #[inline(always)]
    pub unsafe fn array_mut(&mut self) -> &mut [MaybeUninit<u8>; MAX_LEN_NAME as _] {
        &mut self.array
    }

    #[inline(always)]
    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len as _;
    }

    #[inline(always)]
    pub fn as_str(&self) -> Option<&str> {
        if self.len < 0 {
            return None;
        }
        unsafe { Some(self.as_str_unchecked()) }
    }

    #[inline(always)]
    pub unsafe fn as_str_unchecked(&self) -> &str {
        unsafe {
            let slice = self.array.get_unchecked(0..self.len as _);
            let slice = MaybeUninit::slice_assume_init_ref(slice);
            str::from_utf8_unchecked(slice)
        }
    }
}
