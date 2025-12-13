#ifndef BOBBYFS_H
#define BOBBYFS_H

#include <stdint.h>

#ifdef BOBBYFS_DYN_CTX
#define BOBBYFS_CTX bobbyfs_ctx ctx
#define BOBBYFS_CTX_AND BOBBYFS_CTX,
#else
#define BOBBYFS_CTX
#define BOBBYFS_CTX_AND
#endif

// Maximum length excluding null terminator
#define BOBBYFS_MAX_NAME_LEN 92
#define BOBBYFS_PAGE_SIZE 512
#define BOBBYFS_MAX_TREE_DEPTH 20

enum bobbyfs_error {
    BOBBYFS_OK,
    BOBBYFS_STORAGE_FULL,
    BOBBYFS_NOT_FOUND,
    BOBBYFS_IN_USE,
    BOBBYFS_NOT_A_DIRECTORY,
    BOBBYFS_IS_A_DIRECTORY,
    BOBBYFS_EMPTY_NAME,
    BOBBYFS_ALREADY_EXISTS,
    BOBBYFS_EOF,
    BOBBYFS_INVALID_INPUT,
    BOBBYFS_DIRECTORY_NOT_EMPTY,
    BOBBYFS_ERROR_COUNT
};

#ifdef BOBBYFS_DYN_CTX
typedef struct {
    void (*read)(void* ctx, uint32_t offset, void* buf, uint32_t size);
    void (*write)(void* ctx, uint32_t offset, const void* buf, uint32_t size);
    uint32_t (*capacity)(void* ctx);
} bobbyfs_vtable;

typedef struct {
    const bobbyfs_vtable* vtable;
    void* ctx;
} bobbyfs_ctx;
#endif

typedef struct {
    uint32_t page;
} bobbyfs_dir;

enum bobbyfs_page_cursor_type {
    BOBBYFS_NONE,
    BOBBYFS_FILE_INLINE,
    BOBBYFS_PAGE_LIST,
};

struct bobbyfs_page_cursor {
    enum bobbyfs_page_cursor_type type;
    union {
        struct {
            uint32_t index;
        } file_inline;
        struct {
            uint32_t index;
            uint32_t page_list_index;
            uint32_t page_list;
        } page_list;
    };
};

typedef struct {
    uint32_t file;
    uint32_t cursor;
    uint32_t cursor_in_page; // [1, BOBBYFS_PAGE_SIZE]
    uint32_t cursor_page_index;
    uint32_t size_in_pages;
    struct bobbyfs_page_cursor page_cursor;
    uint32_t file_data_page;
} bobbyfs_file;

typedef struct {
    uint32_t page;
} bobbyfs_entry;

typedef struct {
    bobbyfs_entry entry;
    uint32_t stack[BOBBYFS_MAX_TREE_DEPTH];
    uint32_t stack_size;
    uint32_t dir;
} bobbyfs_dir_iter;

uint32_t bobbyfs_get_free_page_count(BOBBYFS_CTX);

void bobbyfs_format(BOBBYFS_CTX);

int bobbyfs_read(BOBBYFS_CTX_AND bobbyfs_file* file, void* buf, uint32_t size);
int bobbyfs_write(BOBBYFS_CTX_AND bobbyfs_file* file, const void* buf, uint32_t size);
void bobbyfs_truncate(BOBBYFS_CTX_AND bobbyfs_file* file);
int bobbyfs_seek_from_current(BOBBYFS_CTX_AND bobbyfs_file* file, int32_t offset);
int bobbyfs_seek_from_start(BOBBYFS_CTX_AND bobbyfs_file* file, uint32_t offset);
int bobbyfs_seek_from_end(BOBBYFS_CTX_AND bobbyfs_file* file, uint32_t offset);
uint32_t bobbyfs_file_size(BOBBYFS_CTX_AND bobbyfs_file* file);
void bobbyfs_file_close(BOBBYFS_CTX_AND bobbyfs_file* file);

bobbyfs_dir bobbyfs_dir_root(BOBBYFS_CTX);
int bobbyfs_file_create(BOBBYFS_CTX_AND bobbyfs_dir dir, const char* path, bobbyfs_file* result);
int bobbyfs_file_open(BOBBYFS_CTX_AND bobbyfs_dir dir, const char* path, bobbyfs_file* result);
int bobbyfs_file_create_or_open(
    BOBBYFS_CTX_AND bobbyfs_dir dir,
    const char* path,
    bobbyfs_file* result);
int bobbyfs_dir_create(BOBBYFS_CTX_AND bobbyfs_dir dir, const char* path, bobbyfs_dir* result);
int bobbyfs_dir_open(BOBBYFS_CTX_AND bobbyfs_dir dir, const char* path, bobbyfs_dir* result);
void bobbyfs_dir_close(BOBBYFS_CTX_AND bobbyfs_dir dir);

int bobbyfs_iter_contents(BOBBYFS_CTX_AND bobbyfs_dir dir, bobbyfs_dir_iter* result);
void bobbyfs_iter_advance(BOBBYFS_CTX_AND bobbyfs_dir_iter* iter);
void bobbyfs_iter_close(BOBBYFS_CTX_AND bobbyfs_dir_iter* iter);

void bobbyfs_entry_name(BOBBYFS_CTX_AND bobbyfs_entry entry, char buf[BOBBYFS_MAX_NAME_LEN + 1]);

#endif // BOBBYFS_H
