extern crate alloc;

use alloc::boxed::Box;
use blib::std_types::{RSlice, RSliceMut, RStr, RString};
use bobby_fs::NameArray;
use core::{mem::MaybeUninit, ptr::NonNull};
use fs::{Dir, DirIter, Entry, Error, File};

use crate::{
    fs, result_into_bool, result_into_option, BobbyDir, BobbyDirIter, BobbyEntry, BobbyFile, ERROR,
};

fn alloc_dir(dir: BobbyDir) -> Dir {
    let ptr = Box::leak(Box::new(dir)) as *mut _ as *mut ();
    unsafe { Dir::from_ptr(NonNull::new_unchecked(ptr)) }
}

fn alloc_file(file: BobbyFile) -> File {
    let ptr = Box::leak(Box::new(file)) as *mut _ as *mut ();
    unsafe { File::from_ptr(NonNull::new_unchecked(ptr)) }
}

fn alloc_dir_iter(dir_iter: BobbyDirIter) -> DirIter {
    let ptr = Box::leak(Box::new(dir_iter)) as *mut _ as *mut ();
    unsafe { DirIter::from_ptr(NonNull::new_unchecked(ptr)) }
}

fn alloc_entry(entry: BobbyEntry) -> Entry {
    let ptr = Box::leak(Box::new(entry)) as *mut _ as *mut ();
    unsafe { Entry::from_ptr(NonNull::new_unchecked(ptr)) }
}

// Dir

pub extern "C" fn fs_root() -> Dir {
    let dir = fs().root();
    alloc_dir(dir)
}

pub extern "C" fn fs_dir_open_dir(dir: &BobbyDir, path: RStr) -> Option<Dir> {
    result_into_option(dir.open_dir(path.as_str())).map(alloc_dir)
}

pub extern "C" fn fs_dir_open_file(dir: &BobbyDir, path: RStr) -> Option<File> {
    result_into_option(dir.open_file(path.as_str())).map(alloc_file)
}

pub extern "C" fn fs_dir_create_file(dir: &BobbyDir, path: RStr) -> Option<File> {
    result_into_option(dir.create_file(path.as_str())).map(alloc_file)
}
pub extern "C" fn fs_dir_create_dir(dir: &BobbyDir, path: RStr) -> Option<Dir> {
    result_into_option(dir.create_dir(path.as_str())).map(alloc_dir)
}

pub extern "C" fn fs_dir_remove(dir: &BobbyDir, path: RStr) -> bool {
    result_into_bool(dir.remove(path.as_str()))
}

pub extern "C" fn fs_dir_iter_contents(dir: &BobbyDir) -> Option<DirIter> {
    result_into_option(dir.iter_contents()).map(alloc_dir_iter)
}

pub extern "C" fn fs_dir_name(dir: &BobbyDir, name_array: &mut NameArray) {
    dir.name(name_array);
}

pub extern "C" fn fs_dir_path(dir: &BobbyDir) -> RString {
    RString::from_string(dir.path())
}

pub extern "C" fn fs_dir_drop(_dir: Box<BobbyDir>) {}

// File

pub extern "C" fn fs_file_truncate(file: &mut BobbyFile) {
    file.truncate()
}

pub extern "C" fn fs_file_read(file: &mut BobbyFile, mut buf: RSliceMut<MaybeUninit<u8>>) -> bool {
    result_into_bool(file.read(buf.as_slice_mut()))
}

pub extern "C" fn fs_file_write(file: &mut BobbyFile, buf: RSlice<u8>) -> bool {
    result_into_bool(file.write(buf.as_slice()))
}

pub extern "C" fn fs_file_size(file: &BobbyFile) -> u32 {
    file.size()
}

pub extern "C" fn fs_file_name(file: &BobbyFile, name_array: &mut NameArray) {
    file.name(name_array);
}

pub extern "C" fn fs_file_path(file: &BobbyFile) -> RString {
    RString::from_string(file.path())
}

pub extern "C" fn fs_file_drop(_file: Box<BobbyFile>) {}

// DirIter

pub extern "C" fn fs_dir_iter_next(dir_iter: &mut BobbyDirIter) -> Option<Entry> {
    dir_iter.next().map(alloc_entry)
}

pub extern "C" fn fs_dir_iter_drop(_dir_iter: Box<BobbyDirIter>) {}

// Entry

pub extern "C" fn fs_entry_is_file(entry: &BobbyEntry) -> bool {
    entry.is_file()
}

pub extern "C" fn fs_entry_is_dir(entry: &BobbyEntry) -> bool {
    entry.is_dir()
}

pub extern "C" fn fs_entry_open_as_file(entry: &BobbyEntry) -> Option<File> {
    result_into_option(entry.open_as_file()).map(alloc_file)
}

pub extern "C" fn fs_entry_open_as_dir(entry: &BobbyEntry) -> Option<Dir> {
    result_into_option(entry.open_as_dir()).map(alloc_dir)
}

pub extern "C" fn fs_entry_name(entry: &BobbyEntry, name_array: &mut NameArray) {
    entry.name(name_array);
}

pub extern "C" fn fs_entry_path(entry: &BobbyEntry) -> RString {
    RString::from_string(entry.path())
}

pub extern "C" fn fs_entry_drop(_entry: Box<BobbyEntry>) {}

pub extern "C" fn fs_get_error() -> Error {
    unsafe { ERROR }
}
