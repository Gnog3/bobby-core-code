#include "binary.h"
#include <debug.h>
#include <panic.h>
#include <stdlib.h>

// #ifdef DEBUG_BINARY
#define PRINTD(...) printd(__VA_ARGS__)
// #endif

int binary_load(bobbyfs_file* file, struct binary* result)
{
    PRINTD("binary_load\n");

    int ret = 0;
    int error;
    uint32_t file_size = bobbyfs_file_size(file);
    if (file_size < 8) {
        return BINARY_INVALID_SIZE;
    }

    if (file->cursor != 0) {
        bobbyfs_seek_from_start(file, 0);
    }

    struct {
        uint32_t binary;
        uint32_t metadata;
    } size;

    error = bobbyfs_read(file, &size, sizeof(size));
    if (error) {
        panic_hex(0x5F197349);
    }

    PRINTD(
        "Header: binary_size %u, metadata size %u\nReading binary... ",
        size.binary,
        size.metadata);

    void* data = malloc(size.binary);
    error = bobbyfs_read(file, data, size.binary);
    if (error) {
        ret = BINARY_INVALID_SIZE;
        goto free_data;
    }

    PRINTD("OK\n");

    // no metadata handling

    result->data = data;
    return 0;

free_data:
    free(data);
    return ret;
}
