use core::ops::Add;

use crate::csr::write;
use crate::csr_defs::{
    DDI_COLOR, DDI_COPY, DDI_FLOODFILL, DDI_MATRIX, DDI_MATRIX_1, DDI_MATRIX_2, DDI_RECTANGLE,
    DDI_SIZE, DDI_SOURCE, DDI_TARGET,
};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);
    #[inline(always)]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    #[inline(always)]
    pub fn into_u32(self) -> u32 {
        ((self.y as u32) << 16) | self.x as u32
    }
}

impl Add for Pos {
    type Output = Pos;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[inline(always)]
pub fn set_matrix_1(matrix: u32) {
    write::<DDI_MATRIX_1>(matrix);
}

#[inline(always)]
pub fn set_matrix_2(matrix: u32) {
    write::<DDI_MATRIX_2>(matrix);
}

#[inline(always)]
pub fn set_target(target: Pos) {
    write::<DDI_TARGET>(target.into_u32());
}

#[inline(always)]
pub fn set_source(source: Pos) {
    write::<DDI_SOURCE>(source.into_u32());
}

#[inline(always)]
pub fn set_size(size: Pos) {
    write::<DDI_SIZE>(size.into_u32());
}

#[inline(always)]
pub fn set_color(color: u32) {
    write::<DDI_COLOR>(color);
}

#[inline(always)]
pub fn run_matrix() {
    write::<DDI_MATRIX>(0);
}

#[inline(always)]
pub fn run_floodfill() {
    write::<DDI_FLOODFILL>(0);
}

#[inline(always)]
pub fn run_copy() {
    write::<DDI_COPY>(0);
}

#[inline(always)]
pub fn run_rectangle() {
    write::<DDI_RECTANGLE>(0);
}

#[inline(always)]
pub fn floodfill(color: u32) {
    set_color(color);
    run_floodfill();
}

#[inline(always)]
pub fn copy(target: Pos, source: Pos, size: Pos) {
    set_target(target);
    set_source(source);
    set_size(size);
    run_copy();
}

#[inline(always)]
pub fn rectangle(target: Pos, size: Pos, color: u32) {
    set_target(target);
    set_size(size);
    set_color(color);
    run_rectangle();
}
