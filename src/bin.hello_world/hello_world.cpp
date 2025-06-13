
#include <lib.io/io.h>
#include <lib.heap/heap.h>
#include <lib.debug/debug.h>
#include <lib.hex/hex.h>
#include <lib.input/input.h>
#include <lib.output/output.h>
#include <lib.console/console.h>
#include <lib.fs/fs.h>

int read_dir2(lfs_dir_t* dir) {
    io::cerr << "Contents of /" << io::endl;
    while (1) {
        struct lfs_info info;
        int err = lfs_dir_read(&lfs, dir, &info);
        if (err < 0) {
            io::cerr << "Error during lfs_dir_read" << io::endl;
            return err;
        }
        if (info.name[0] == 0) {
            io::cerr << "Entry: empty name" << io::endl;
        } else {
            io::cerr << "Entry: \"" << info.name << "\"" << io::endl;
        }
        if (err == 0) {
            return 0;
        }
    }
}

int read_dir1() {
    io::cerr << "Opening root" << io::endl;
    lfs_dir_t dir;
    int err = lfs_dir_open(&lfs, &dir, "/");
    if (!err) {
        io::cerr << "Opened root successfully" << io::endl;
        read_dir2(&dir);
        err = lfs_dir_close(&lfs, &dir);
        if (!err) {
            io::cerr << "Closed root successfully" << io::endl;
        } else {
            io::cerr << "Error while closing root dir" << io::endl;
        }
    } else {
        io::cerr << "Error while opening root dir" << io::endl;
    }
    return 0;
}

int main() {
    heap_init();
    int err = mount_fs();
    if (err) {
        io::cerr << "main(): err while mount_fs, no unmounting" << io::endl;
        return -1;
    }

    lfs_file_t file;
    uint32_t boot_count = 0;
    io::cerr << "Opening file" << io::endl;
    lfs_file_open(&lfs, &file, "boot_count", LFS_O_RDWR | LFS_O_CREAT);
    io::cerr << "Reading file" << io::endl;
    lfs_file_read(&lfs, &file, &boot_count, sizeof(boot_count));

    // update boot count
    boot_count += 1;
    io::cerr << "File rewind" << io::endl;
    lfs_file_rewind(&lfs, &file);
    io::cerr << "Writing to file" << io::endl;
    lfs_file_write(&lfs, &file, &boot_count, sizeof(boot_count));

    // remember the storage is not updated until the file is closed successfully
    io::cerr << "Closing file" << io::endl;
    lfs_file_close(&lfs, &file);
    io::cerr << "Boot count: " << boot_count << io::endl;
    read_dir1();

    io::cerr << "Unmounting fs.." << io::endl;
    lfs_unmount(&lfs);
    io::cerr << "Unmounted" << io::endl;
    // ddi_color(color_t(0));
    // ddi_floodfill();
    
    // Console console;
    // console.print("Console");
    // console.new_line();
    // while (true) {
    //     console.print(">");
    //     std::string_view s = console.input();
    //     console.print("You entered: ");
    //     console.print(s);
    //     console.new_line();
    // }
    // while (true) {
    //     unsigned char key = read_key();
    //     if (!key) continue;
    //     bool pressed = (key >> 7) != 0;
    //     if (pressed) {
    //         key &= 0x7F;
    //         io::cerr << io::hex(key) << " pressed" << io::endl;
    //     } else {
    //         io::cerr << io::hex(key) << " released" << io::endl;
    //     }
    // }
}
