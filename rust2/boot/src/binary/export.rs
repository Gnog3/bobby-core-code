use alloc::{boxed::Box, format};
use binary::Binary;
use blib::std_types::{RResult, RString};
use core::ptr::NonNull;
use fs_impl::BobbyFile;

fn alloc_binary(binary: crate::binary::Binary) -> Binary {
    let ptr = Box::leak(Box::new(binary)) as *mut _ as *mut ();
    unsafe { Binary::from_ptr(NonNull::new_unchecked(ptr)) }
}

pub extern "C" fn binary_from_file(file: &mut BobbyFile) -> RResult<Binary, RString> {
    let binary = crate::binary::Binary::from_file(file);
    match binary {
        Ok(binary) => RResult::ROk(alloc_binary(binary)),
        Err(err) => RResult::RErr(RString::from_string(format!("{:?}", err))),
    }
}

pub extern "C" fn binary_run(binary: &crate::binary::Binary) {
    binary.run_main();
}

pub extern "C" fn binary_drop(_binary: Box<crate::binary::Binary>) {}
