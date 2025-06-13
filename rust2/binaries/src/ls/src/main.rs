#![no_std]
#![no_main]
#![feature(try_with_capacity)]
#![feature(vec_push_within_capacity)]
#![feature(maybe_uninit_write_slice)]
#![feature(alloc_error_handler)]

use core::mem::MaybeUninit;

use alloc::{borrow::Cow, string::String, vec::Vec};
use console::println;
use fs::{Dir, NameArray};
use init_fns as init;

extern crate alloc;
extern crate bin_panic_handler;
extern crate heap;

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    loop {}
}

unsafe fn push_unchecked<T>(vec: &mut Vec<T>, elem: T) {
    unsafe { vec.push_within_capacity(elem).unwrap_unchecked() }
}

unsafe fn extend_unchecked<T: Copy>(vec: &mut Vec<T>, slice: &[T]) {
    let old_len = vec.len();
    let uninit = unsafe { vec.spare_capacity_mut().get_unchecked_mut(0..slice.len()) };
    MaybeUninit::copy_from_slice(uninit, slice);
    unsafe {
        vec.set_len(old_len + slice.len());
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".init")]

pub extern "C" fn main() {
    let Some(current_dir) = init::current_dir().get(1..) else {
        println!("Error: empty current_dir");
        return;
    };
    let arg = init::arg();
    let (dir_result, path) = if arg.is_empty() && current_dir.is_empty() {
        (Ok(Dir::root()), Cow::Borrowed("/"))
    } else if arg.is_empty() {
        (
            Dir::root().open_dir(current_dir),
            Cow::Borrowed(current_dir),
        )
    } else if current_dir.is_empty() {
        (Dir::root().open_dir(&arg), Cow::Borrowed(arg))
    } else {
        // let Ok(mut path) = Vec::try_with_capacity(current_dir.len() + arg.len() + 1) else {
        //     println!("Error: No memory");
        //     return;
        // };

        let mut path = Vec::with_capacity(current_dir.len() + arg.len() + 1);

        unsafe {
            extend_unchecked(&mut path, current_dir.as_bytes());
            push_unchecked(&mut path, b'/');
            extend_unchecked(&mut path, arg.as_bytes());
        }

        let path = unsafe { String::from_utf8_unchecked(path) };
        let dir_result = Dir::root().open_dir(&path);
        (dir_result, Cow::Owned(path))
    };

    let dir = match dir_result {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error opening", &path, ": ", err.as_str());
            return;
        }
    };

    let iter = match dir.iter_contents() {
        Ok(iter) => iter,
        Err(err) => {
            println!("Error creating dir_iter: ", err.as_str());
            return;
        }
    };

    if arg.is_empty() {
        println!("Contents of .:");
    } else {
        println!("Contents of ", arg, ":");
    }

    for e in iter {
        let mut name = NameArray::new();
        e.name(&mut name);
        let name = unsafe { name.as_str().unwrap_unchecked() };
        if e.is_dir() {
            println!("Dir: ", name);
        } else {
            println!("File: ", name);
        }
    }

}
