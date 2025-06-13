#![no_std]
#![no_main]
#![feature(try_with_capacity)]
#![feature(maybe_uninit_write_slice)]

mod export;

extern crate alloc;
extern crate bin_panic_handler;
extern crate heap;

use core::mem::MaybeUninit;

use alloc::{string::String, vec::Vec};
use binary::Binary;
use console::{print, println};
use export::{set_arg, set_current_dir};
use fs::Dir;

unsafe fn extend_unchecked<T: Copy>(vec: &mut Vec<T>, slice: &[T]) {
    let old_len = vec.len();
    let uninit = unsafe { vec.spare_capacity_mut().get_unchecked_mut(0..slice.len()) };
    MaybeUninit::copy_from_slice(uninit, slice);
    unsafe {
        vec.set_len(old_len + slice.len());
    }
}

fn run_with_arg(current_dir: &str, command: &str, arg: &str) {
    let Ok(mut s) = Vec::try_with_capacity(command.len() + 4) else {
        println!("Error: No memory");
        return;
    };

    let s = unsafe {
        extend_unchecked(&mut s, command.as_bytes());
        extend_unchecked(&mut s, b".exe");
        String::from_utf8_unchecked(s)
    };

    let bin_dir = match Dir::root().open_dir("bin") {
        Ok(bin) => bin,
        Err(err) => {
            println!("Can't open /bin: ", err.as_str());
            return;
        }
    };

    let mut file = match bin_dir.open_file(&s) {
        Ok(file) => file,
        Err(err) => {
            println!("Can't open /bin/", command, ": ", err.as_str());
            return;
        }
    };

    let binary = match Binary::from_file(&mut file) {
        Ok(binary) => binary,
        Err(err) => {
            println!("Can't open binary: ", &err);
            return;
        }
    };

    set_current_dir(current_dir);
    set_arg(arg);
    binary.run();
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".init")]
pub extern "C" fn main() {
    println!("Console");
    let mut current_dir: Dir = Dir::root();

    loop {
        let path = current_dir.path();
        print!(&path, ">");
        let input = console::input();
        if input.starts_with("cd ") {
            let path = unsafe { input.get_unchecked(3..) };
            match current_dir.open_dir(path) {
                Ok(dir) => current_dir = dir,
                Err(err) => {
                    println!("Error. ", err.as_str());
                }
            }
        } else if input == "exit" {
            break;
        } else {
            match input.split_once(' ') {
                Some((command, arg)) => {
                    run_with_arg(&path, command, arg);
                }
                None => {
                    run_with_arg(&path, &input, "");
                }
            }
        }
    }
}
