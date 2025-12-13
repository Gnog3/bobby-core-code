#include "test_ctx.h"
#include <check.h>
#include <stddef.h>
#include <stdlib.h>

#define TEST_BUFFER_SIZE (2 * 1024 * 1024) // 2 MB

struct test_ctx {
    void* buffer;
    size_t size;
};

static void test_read(void* ctx, uint32_t offset, void* buf, uint32_t size)
{
    struct test_ctx* tctx = ctx;

    ck_assert_int_ge(offset, 0);
    ck_assert_int_le(offset + size, tctx->size);

    memcpy(buf, tctx->buffer + offset, size);
}

static void test_write(void* ctx, uint32_t offset, const void* buf, uint32_t size)
{
    struct test_ctx* tctx = ctx;

    ck_assert_int_ge(offset, 0);
    ck_assert_int_le(offset + size, tctx->size);

    memcpy(tctx->buffer + offset, buf, size);
}

static uint32_t test_capacity(void* ctx)
{
    struct test_ctx* tctx = ctx;
    return tctx->size;
}

bobbyfs_ctx test_ctx;
static void* formatted_buffer;
static struct test_ctx ctx;
static bobbyfs_vtable test_vtable = {
    .read = test_read,
    .write = test_write,
    .capacity = test_capacity,
};

static void init()
{
    ctx.buffer = malloc(TEST_BUFFER_SIZE);
    ctx.size = TEST_BUFFER_SIZE;
    test_ctx.ctx = &ctx;
    test_ctx.vtable = &test_vtable;

    bobbyfs_format(test_ctx);
    formatted_buffer = ctx.buffer;
    ctx.buffer = malloc(TEST_BUFFER_SIZE);
}

void reset_ctx()
{
    if (!formatted_buffer) {
        init();
    }
    memcpy(ctx.buffer, formatted_buffer, ctx.size);
}

void free_ctx()
{
    free(ctx.buffer);
    free(formatted_buffer);
}
