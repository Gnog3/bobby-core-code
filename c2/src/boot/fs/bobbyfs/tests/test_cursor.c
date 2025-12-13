#include "test_cursor.h"
#include "bobbyfs.h"
#include "vector.h"
#include <check.h>
#include <stdio.h>
#include <stdlib.h>

test_cursor test_cursor_create(bobbyfs_ctx ctx, bobbyfs_dir dir, vector* v, const char* name)
{
    test_cursor c;
    c.cursor.v = v;
    c.cursor.pos = 0;
    vector_init(c.cursor.v);
    int error = bobbyfs_file_create(ctx, dir, name, &c.file);
    ck_assert_int_eq(error, BOBBYFS_OK);
    return c;
}

test_cursor test_cursor_open(bobbyfs_ctx ctx, bobbyfs_dir dir, vector* v, const char* name)
{
    test_cursor c;
    int error = bobbyfs_file_open(ctx, dir, name, &c.file);
    ck_assert_int_eq(error, BOBBYFS_OK);
    c.cursor.v = v;
    c.cursor.pos = 0;
    return c;
}

void test_cursor_read(bobbyfs_ctx ctx, test_cursor* c, size_t size)
{
    ck_assert_int_le(c->cursor.pos + size, c->cursor.v->size);

    void* buf_from_file = malloc(size);
    int error = bobbyfs_read(ctx, &c->file, buf_from_file, size);
    ck_assert_int_eq(error, BOBBYFS_OK);

    ck_assert_mem_eq(buf_from_file, c->cursor.v->data + c->cursor.pos, size);
    c->cursor.pos += size;
    free(buf_from_file);
}

void test_cursor_write(bobbyfs_ctx ctx, test_cursor* c, const void* data, size_t size)
{
    int error = bobbyfs_write(ctx, &c->file, data, size);
    ck_assert_int_eq(error, BOBBYFS_OK);

    error = vector_rewrite(c->cursor.v, c->cursor.pos, data, size);
    ck_assert_int_eq(error, 0);
    c->cursor.pos += size;
}

void test_cursor_truncate(bobbyfs_ctx ctx, test_cursor* c)
{
    bobbyfs_truncate(ctx, &c->file);
    c->cursor.v->size = c->cursor.pos;
}

void test_cursor_seek_from_start(bobbyfs_ctx ctx, test_cursor* c, uint32_t pos)
{
    int error = bobbyfs_seek_from_start(ctx, &c->file, pos);
    ck_assert_int_eq(error, BOBBYFS_OK);

    ck_assert_int_le(pos, c->cursor.v->size);
    c->cursor.pos = pos;
}

void test_cursor_seek_from_end(bobbyfs_ctx ctx, test_cursor* c, uint32_t pos)
{
    int error = bobbyfs_seek_from_end(ctx, &c->file, pos);
    ck_assert_int_eq(error, BOBBYFS_OK);

    ck_assert_int_le(pos, c->cursor.v->size);
    c->cursor.pos = c->cursor.v->size - pos;
}

uint32_t test_cursor_size(bobbyfs_ctx ctx, test_cursor* c)
{
    uint32_t size = bobbyfs_file_size(ctx, &c->file);
    ck_assert_int_eq(size, c->cursor.v->size);
    return size;
}
