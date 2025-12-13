#ifndef TEST_CURSOR_H
#define TEST_CURSOR_H

#include "bobbyfs.h"
#include "vector.h"

typedef struct {
    vector* v;
    size_t pos;
} cursor;

typedef struct {
    bobbyfs_file file;
    cursor cursor;
} test_cursor;

test_cursor
test_cursor_create(bobbyfs_ctx ctx, bobbyfs_dir dir, vector* v, const char* name);
test_cursor
test_cursor_open(bobbyfs_ctx ctx, bobbyfs_dir dir, vector* v, const char* name);
void test_cursor_read(bobbyfs_ctx ctx, test_cursor* c, size_t size);
void test_cursor_write(bobbyfs_ctx ctx, test_cursor* c, const void* data, size_t size);
void test_cursor_truncate(bobbyfs_ctx ctx, test_cursor* c);
void test_cursor_seek_from_start(bobbyfs_ctx ctx, test_cursor* c, uint32_t pos);
void test_cursor_seek_from_end(bobbyfs_ctx ctx, test_cursor* c, uint32_t pos);
uint32_t test_cursor_size(bobbyfs_ctx ctx, test_cursor* c);

#endif
