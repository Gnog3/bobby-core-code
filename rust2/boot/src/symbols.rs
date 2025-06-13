use core::mem::take;

use alloc::{collections::btree_map::BTreeMap, string::String};

static mut SYMBOLS: BTreeMap<String, *const ()> = BTreeMap::new();

pub fn init_symbols(map: BTreeMap<String, *const ()>) {
    unsafe {
        (&raw mut SYMBOLS).write(map);
    }
}

pub fn add_symbol(name: String, ptr: *const ()) {
    unsafe {
        let prev = (*&raw mut SYMBOLS).insert(name, ptr);
        if let Some(prev) = prev {
            panic!("Symbol already added: {:#p}", prev);
        }
    }
}

pub fn get_symbol(name: &str) -> Option<*const ()> {
    unsafe { (*&raw mut SYMBOLS).get(name).copied() }
}

pub fn remove_symbol(name: &str) {
    unsafe {
        (*&raw mut SYMBOLS).remove(name);
    }
}

pub fn destroy_symbols() {
    unsafe {
        take(&mut *&raw mut SYMBOLS);
    }
}
