#pragma once
#include <fs.h>

enum binary_error { BINARY_INVALID_SIZE };

struct binary {
    void* data;
};

int binary_load(bobbyfs_file* file, struct binary* result);
