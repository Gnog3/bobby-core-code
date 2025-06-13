#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_as_bytes)]
#![feature(maybe_uninit_slice)]

mod binary;
mod console;
mod image;
mod init_symbols;
mod keyboard;
mod panic_handler;
mod panic_screen;
mod symbols;

use core::arch::naked_asm;

use binary::{Binary, BinaryError};
use blib::{debug, halt, print_err, println_err};
use fs_impl::{
    self,
    bobby_fs::{Error, NameArray},
    fs,
};

extern crate alloc;

#[naked]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".init")]
unsafe extern "C" fn _start() -> ! {
    unsafe {
        naked_asm!(
            "
        li sp, 1024*1024
        j main
        "
        );
    }
}

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    debug::clear();
    println_err!("Boot.");
    unsafe {
        heap_impl::HEAP.init();
    }

    init_symbols::init_symbols();
    console::clear();
    keyboard::reset_shift();

    print_err!("Formatting fs... ");
    fs().format();
    println_err!("OK.");

    println_err!(
        "fs stat: {} free / {} total (pages)",
        fs().get_free_page_count(),
        fs().get_total_page_count()
    );

    let result = main2();
    if let Err(err) = result {
        panic!("Error in main2(): {:?}", err);
    }

    symbols::destroy_symbols();
    println_err!("Heap size on halt: {} bytes.", heap_impl::HEAP.used());
    println_err!("Halt.");
    halt();
}

static INIT: &[u8] = include_bytes!(r"../../binaries/init.exe");
static LS: &[u8] = include_bytes!(r"../../binaries/ls.exe");

fn write_binary_to_fs(bytes: &[u8], path: &str) -> Result<(), BinaryError> {
    print_err!("Creating file {path}... ");
    let mut file = fs_impl::fs().root().create_or_open_file(path)?;
    println_err!("OK.");
    println_err!("Writing bytes... ");
    file.truncate();
    file.write(bytes)?;
    println_err!("OK.");
    Ok(())
}

fn main2() -> Result<(), BinaryError> {
    {
        write_binary_to_fs(INIT, "init.exe")?;
    }
    {
        match fs().root().open_dir("bin") {
            Ok(_) => (),
            Err(Error::NotFound) => {
                println_err!("/bin not found, creating it.");
                fs().root().create_dir("bin").unwrap();
            }
            Err(err) => return Err(err.into()),
        };
        write_binary_to_fs(LS, "bin/ls.exe")?;
    }
    {
        println_err!("Contents of /");
        let root_dir = fs().root();
        for e in root_dir.iter_contents()? {
            let mut name = NameArray::new();
            e.name(&mut name);
            let name = name.as_str().unwrap();
            if e.is_dir() {
                println_err!("Dir: {:?}", name);
            } else {
                let file = e.open_as_file()?;
                let size = file.size();
                println_err!("File: {:?}, size: {} bytes", name, size);
            }
        }
    }
    {
        println_err!("Contents of /bin/");
        let root_dir = fs().root().open_dir("bin")?;
        for e in root_dir.iter_contents()? {
            let mut name = NameArray::new();
            e.name(&mut name);
            let name = name.as_str().unwrap();
            if e.is_dir() {
                println_err!("Dir: {:?}", name);
            } else {
                let file = e.open_as_file()?;
                let size = file.size();
                println_err!("File: {:?}, size: {} bytes", name, size);
            }
        }
    }

    let binary = {
        print_err!("Opening init.exe...");
        let mut file = fs().root().open_file("init.exe")?;
        println_err!("OK.");
        print_err!("Reading binary... ");
        let binary = Binary::from_file(&mut file)?;
        println_err!("OK.");
        binary
    };
    println_err!("Running main.");
    binary.run_main();

    // let root = fs().root();
    // print_err!("Creating a file... ");
    // let mut file = root.create_file("A thousand zeroes")?;
    // println_err!("OK.");
    // let zeroes = [0u8; 256];
    // for _ in 0..4 {
    //     print_err!("Writing 256 bytes... ");
    //     file.write(&zeroes)?;
    //     println_err!("OK.");
    // }
    Ok(())
}
