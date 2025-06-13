use core::arch::asm;

#[inline(always)]
pub fn read<const CSR: u32>() -> u32 {
    let value;
    unsafe {
        asm!(
            "csrrw {}, {}, zero",
            out(reg) value,
            const { CSR },
            options(nostack, nomem),
        );
    }
    value
}

#[inline(always)]
pub fn write<const CSR: u32>(value: u32) {
    unsafe {
        asm!(
            "csrw {}, {}",
            const { CSR },
            in(reg) value,
            options(nostack, nomem),
        );
    }
}

#[inline(always)]
pub fn write_ptr<const CSR: u32, T>(value: *const T) {
    unsafe {
        asm!(
            "csrw {}, {}",
            const { CSR },
            in(reg) value,
            options(nostack),
        );
    }
}

#[inline(always)]
pub fn nop() {
    unsafe {
        asm!("nop");
    }
}
