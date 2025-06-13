use crate::{binary, console, keyboard, symbols};
use alloc::{collections::btree_map::BTreeMap, string::String};
use blib::{print_err, println_err};

pub fn init_symbols() {
    print_err!("Building symbol map... ");
    let mut map: BTreeMap<String, *const ()> = BTreeMap::new();
    insert_fs_fns(&mut map);
    insert_heap_fns(&mut map);
    insert_compiler_builtins_fns(&mut map);
    insert_console_fns(&mut map);
    insert_keyboard_fns(&mut map);
    insert_binary_fns(&mut map);
    symbols::init_symbols(map);
    println_err!("OK.");
}



#[rustfmt::skip]
fn insert_fs_fns(map: &mut BTreeMap<String, *const ()>) {
    map.insert("fs_root".into(), fs_impl::export::fs_root as _);

    map.insert("fs_dir_open_dir".into(), fs_impl::export::fs_dir_open_dir as _);
    map.insert("fs_dir_open_file".into(), fs_impl::export::fs_dir_open_file as _);
    map.insert("fs_dir_create_file".into(), fs_impl::export::fs_dir_create_file as _);
    map.insert("fs_dir_create_dir".into(), fs_impl::export::fs_dir_create_dir as _);
    map.insert("fs_dir_remove".into(), fs_impl::export::fs_dir_remove as _);
    map.insert("fs_dir_iter_contents".into(), fs_impl::export::fs_dir_iter_contents as _);
    map.insert("fs_dir_name".into(), fs_impl::export::fs_dir_name as _);
    map.insert("fs_dir_path".into(), fs_impl::export::fs_dir_path as _);
    map.insert("fs_dir_drop".into(), fs_impl::export::fs_dir_drop as _);

    map.insert("fs_file_truncate".into(), fs_impl::export::fs_file_truncate as _);
    map.insert("fs_file_read".into(), fs_impl::export::fs_file_read as _);
    map.insert("fs_file_write".into(), fs_impl::export::fs_file_write as _);
    map.insert("fs_file_size".into(), fs_impl::export::fs_file_size as _);
    map.insert("fs_file_name".into(), fs_impl::export::fs_file_name as _);
    map.insert("fs_file_path".into(), fs_impl::export::fs_file_path as _);
    map.insert("fs_file_drop".into(), fs_impl::export::fs_file_drop as _);

    map.insert("fs_dir_iter_next".into(), fs_impl::export::fs_dir_iter_next as _);
    map.insert("fs_dir_iter_drop".into(), fs_impl::export::fs_dir_iter_drop as _);

    map.insert("fs_entry_is_file".into(), fs_impl::export::fs_entry_is_file as _);
    map.insert("fs_entry_is_dir".into(), fs_impl::export::fs_entry_is_dir as _);
    map.insert("fs_entry_open_as_file".into(), fs_impl::export::fs_entry_open_as_file as _);
    map.insert("fs_entry_open_as_dir".into(), fs_impl::export::fs_entry_open_as_dir as _);
    map.insert("fs_entry_name".into(), fs_impl::export::fs_entry_name as _);
    map.insert("fs_entry_path".into(), fs_impl::export::fs_entry_path as _);
    map.insert("fs_entry_drop".into(), fs_impl::export::fs_entry_drop as _);

    map.insert("fs_get_error".into(), fs_impl::export::fs_get_error as _);
}

#[rustfmt::skip]
fn insert_heap_fns(map: &mut BTreeMap<String, *const ()>) {
    map.insert("heap_alloc".into(), heap_impl::export::heap_alloc as _);
    map.insert("heap_dealloc".into(), heap_impl::export::heap_dealloc as _);
}

unsafe extern "C" {
    fn memcpy();
    fn memcmp();
    fn __udivsi3();
    fn __mulsi3();
}

#[rustfmt::skip]
fn insert_compiler_builtins_fns(map: &mut BTreeMap<String, *const ()>) {
    map.insert("memcpy".into(), memcpy as _);
    map.insert("memcmp".into(), memcmp as _);
    map.insert("__udivsi3".into(), __udivsi3 as _);
    map.insert("__mulsi3".into(), __mulsi3 as _);
}

#[rustfmt::skip]
fn insert_console_fns(map: &mut BTreeMap<String, *const ()>) {
    map.insert("console_clear".into(), console::export::console_clear as _);
    map.insert("console_print".into(), console::export::console_print as _);
    map.insert("console_newline".into(), console::export::console_newline as _);
    map.insert("console_input".into(), console::export::console_input as _);
}

#[rustfmt::skip]
fn insert_keyboard_fns(map: &mut BTreeMap<String, *const ()>) {
    map.insert("keyboard_handle_control".into(), keyboard::export::keyboard_handle_control as _);
    map.insert("keyboard_get_char".into(), keyboard::export::keyboard_get_char as _);
}

#[rustfmt::skip]
fn insert_binary_fns(map: &mut BTreeMap<String, *const ()>) {
    map.insert("binary_from_file".into(), binary::export::binary_from_file as _);
    map.insert("binary_run".into(), binary::export::binary_run as _);
    map.insert("binary_drop".into(), binary::export::binary_drop as _);
}