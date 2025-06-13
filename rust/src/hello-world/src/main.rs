#![no_std]
#![no_main]

mod console;
mod gui;

use core::mem::MaybeUninit;

use alloc::{borrow::ToOwned, string::ToString};
use console::Console;
use fatfs::{FsOptions, Write};
use rt::ddi::{self, Pos};

extern crate alloc;
#[derive(Debug)]
struct Abc {
    value_1: &'static str,
    value_2: i32,
    value_3: u32,
}

extern crate rt;

// fn run_fs(fs: &fatfs::FileSystem<rt::fs::FsStorage>) -> Result<(), Error<()>> {
//     let root_dir = fs.root_dir();
//     log::info!("Creating file");
//     let mut file = root_dir.create_file("boot_count")?;
//     log::info!("Created");
//     let mut arr = [0u8];
//     let _ = file.read(&mut arr);
//     arr[0] += 1;
//     file.seek(fatfs::SeekFrom::Start(0))?;
//     file.write_all(&arr)?;
//     log::info!("Boot count: {}", arr[0]);
//     Ok(())
// }

#[no_mangle]
extern "C" fn laladu() {
    ddi::floodfill(0x3C6EA6);
}

extern "C" {
    fn daluda();
}

#[no_mangle]
extern "C" fn main2() {
    // log::info!("This program works.");
    unsafe {
        daluda();
    }
    ddi::floodfill(0x3C6EA6);
    
    // gui::button(Pos::new(50, 20), Pos::new(75, 23), false);
    // let mut console = Console::new();
    // console.print("Console");
    // console.newline();
    // loop {
    //     console.print(">");
    //     let input = console.input().to_owned();
    //     console.print("You entered: ");
    //     console.print(&input);
    //     console.newline();
    // }
    // let f1: f32 = input.parse().unwrap();
    // console.print(">");
    // let input = console.input().to_owned();
    // console.print("You entered: ");
    // console.print(&input);
    // console.newline();
    // let f2: f32 = input.parse().unwrap();
    // let f3 = f1 + f2;
    // let s = f3.to_string();
    // console.print("Result: ");
    // console.print(&s);
    // console.newline();
    // log::info!("Mounting FS");
    // let result = fatfs::FileSystem::new(rt::fs::FsStorage::new(), FsOptions::new());
    // let fs = match result {
    //     Ok(fs) => fs,
    //     Err(err) => {
    //         log::error!("Error: {:?}", err);
    //         return;
    //     }
    // };
    // log::info!("Mounted");
    // log::info!("Fat type: {:?}", fs.fat_type());
    // log::info!("Cluster size: {}", fs.cluster_size());
    // log::info!("run_fs result: {:?}", run_fs(&fs));
    // log::info!("Contents of /");

    // for entry in fs.root_dir().iter() {
    //     let entry = entry.unwrap();
    //     if entry.is_file() {
    //         log::info!("File: {:?}", entry.file_name());
    //     } else {
    //         log::info!("Dir: {:?}", entry.file_name());
    //     }
    // }
    // fs.unmount().unwrap();
    // log::info!("Unmounted");
}
