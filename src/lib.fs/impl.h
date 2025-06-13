#ifndef LIB_FS_IMPL_H
#define LIB_FS_IMPL_H

#define LFS_CONFIG_FILESYSTEM_SIZE (4 * 1024 * 1024)
#define LFS_CONFIG_BLOCK_SIZE 512
#define LFS_CONFIG_BLOCK_COUNT (LFS_CONFIG_FILESYSTEM_SIZE / LFS_CONFIG_BLOCK_SIZE)

#include "lfs.h"

extern "C" int mount_fs();

extern lfs_t lfs;

#endif
