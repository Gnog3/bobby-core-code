use crate::{csr::{nop, write, write_ptr}, csr_defs::{TERM_COLOR, TERM_LENGTH, TERM_POSITION, TERM_PRINT}, ddi::Pos};

#[inline(always)]
pub fn set_color(color: u32) {
    write::<TERM_COLOR>(color);
}

#[inline(always)]
pub fn set_position(pos: Pos) {
    write::<TERM_POSITION>(pos.into_u32());
}

#[inline(always)]
pub fn set_length(len: usize) {
    write::<TERM_LENGTH>(len as _);
}

#[inline(always)]
pub fn run_print_ptr(ptr: *const u8) {
    write_ptr::<TERM_PRINT, _>(ptr);
    nop();
}

#[inline(always)]
pub fn print_slice(pos: Pos, slice: &[u8]) {
    set_position(pos);
    set_length(slice.len());
    run_print_ptr(slice.as_ptr());
}
