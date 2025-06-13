#include "impl.h"

#include "lfs.h"

#include <lib.io/io.h>

#define RAM_SIZE (16 * 1024 * 1024)
#define FILESYSTEM_ADDR (RAM_SIZE - LFS_CONFIG_FILESYSTEM_SIZE)

int block_device_read(const struct lfs_config *c, lfs_block_t block, lfs_off_t off, void *buffer, lfs_size_t size) {
    char* fs = (char*) FILESYSTEM_ADDR;
    char* buffer_c = (char*) buffer;
    for (lfs_size_t i = 0; i < size; i++) {
        buffer_c[i] = fs[block * LFS_CONFIG_BLOCK_SIZE + off + i];
    }
    return 0;
}

int block_device_prog(const struct lfs_config *c, lfs_block_t block, lfs_off_t off, const void *buffer, lfs_size_t size) {
    char* fs = (char*) FILESYSTEM_ADDR;
    char* buffer_c = (char*) buffer;
    for (lfs_size_t i = 0; i < size; i++) {
        fs[block * LFS_CONFIG_BLOCK_SIZE + off + i] = buffer_c[i];
    }
    return 0;
}

int block_device_erase(const struct lfs_config *c, lfs_block_t block) {
    return 0;
}

int block_device_sync(const struct lfs_config *c) {
    return 0;
}

const struct lfs_config cfg = {
    // block device operations
    .read  = block_device_read,
    .prog  = block_device_prog,
    .erase = block_device_erase,
    .sync  = block_device_sync,

    // block device configuration
    .read_size = 1,
    .prog_size = 1,
    .block_size = LFS_CONFIG_BLOCK_SIZE,
    .block_count = LFS_CONFIG_BLOCK_COUNT,
    .block_cycles = -1,
    .cache_size = 64,
    .lookahead_size = 16,
};

lfs_t lfs;

int mount_fs() {
    io::cerr << "Mounting fs..." << io::endl;
    int err = lfs_mount(&lfs, &cfg);
    if (err) {
        io::cerr << "Couldn't mount fs, formatting..." << io::endl;
        err = lfs_format(&lfs, &cfg);
        if (err) {
            io::cerr << "Couldn't format fs" << io::endl;
            return err;
        } else {
            io::cerr << "Formatted fs successfully" << io::endl;
        }
        err = lfs_mount(&lfs, &cfg);
        if (err) {
            io::cerr << "Couldn't mount fs" << io::endl;
            return err;
        } else {
            io::cerr << "Fs mounted successfully" << io::endl;
        }
    } else {
        io::cerr << "Mounted existing fs" << io::endl;
    }
    return 0;
}
