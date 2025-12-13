#include "bobbyfs.h"
#include <stddef.h>
#include <string.h>

#if TESTING
#include <check.h>
#endif

#define PAGEKIND_MAIN 1
#define PAGEKIND_FILE 2
#define PAGEKIND_DIR 3

#define MAX_INLINE_FILE_DATA_PAGES 96
#define MAX_PAGE_LIST_FILE_DATE_PAGES 126
#define INLINE_PAGES_OFFSET (sizeof(struct header) + BOBBYFS_MAX_NAME_LEN + 1)
#define NEXT_PAGES_OFFSET (BOBBYFS_PAGE_SIZE - sizeof(uint32_t))
#define PREV_PAGES_OFFSET (BOBBYFS_PAGE_SIZE - 2 * sizeof(uint32_t))

#ifdef BOBBYFS_DYN_CTX
#define CTX bobbyfs_ctx ctx
#define CTX_AND CTX,
#else
#define CTX
#define CTX_AND
#endif

struct header {
    uint32_t kind;
    uint32_t parent;
    uint32_t in_use;
    uint32_t left_child;
    uint32_t right_child;
    uint32_t height;
    uint32_t attr;
    uint32_t name_len;
};

struct main {
    uint32_t kind;
    uint32_t total_page_count;
    uint32_t free_page_count;
    uint32_t next_free_page;
};

struct alloc {
    uint32_t free_page_count;
    uint32_t next_free_page;
};

#ifdef BOBBYFS_DYN_CTX
#define READ(offset, buf, size) ctx.vtable->read(ctx.ctx, offset, buf, size)
#define WRITE(offset, buf, size) ctx.vtable->write(ctx.ctx, offset, buf, size)
#define CAPACITY() ctx.vtable->capacity(ctx.ctx)
#define CTX_ARG ctx
#define CTX_ARG_AND CTX_ARG,
#else
#define CTX_ARG
#define CTX_ARG_AND
#ifndef READ
#define READ(offset, buf, size)
#endif
#ifndef WRITE
#define WRITE(offset, buf, size)
#endif
#ifndef CAPACITY
#define CAPACITY() 0
#endif
#endif

#ifndef READ_VAL
#define READ_VAL(offset, ptr) READ(offset, ptr, sizeof(*ptr))
#endif

#ifndef WRITE_VAL
#define WRITE_VAL(offset, val) WRITE(offset, &val, sizeof(val))
#endif

#define HEADER_OFFSET(field) offsetof(struct header, field)

#define READ_VAL_HO(offset, field, ptr) READ_VAL(offset + HEADER_OFFSET(field), ptr)
#define WRITE_VAL_HO(offset, field, val) WRITE_VAL(offset + HEADER_OFFSET(field), val)

// --- FS STATS PUBLIC ---

uint32_t bobbyfs_get_free_page_count(CTX)
{
    uint32_t free_page_count;
    READ_VAL(offsetof(struct main, free_page_count), &free_page_count);
    return free_page_count;
}

// --- FORMATTING ---

static void make_free_list_chain(CTX_AND uint32_t page_from, uint32_t page_to)
{
    for (uint32_t i = page_from; i < page_to - 1; i++) {
        uint32_t offset = i * BOBBYFS_PAGE_SIZE;
        uint32_t next = (i + 1) * BOBBYFS_PAGE_SIZE;
        WRITE_VAL(offset, next);
    }

    // Next page for last page is 0
    uint32_t offset = (page_to - 1) * BOBBYFS_PAGE_SIZE;
    uint32_t next = 0;
    WRITE_VAL(offset, next);
}

static void make_root_directory(CTX_AND uint32_t page)
{
    struct header h = {
        .kind = PAGEKIND_DIR,
        .parent = 0,
        .in_use = 0,
        .left_child = 0,
        .right_child = 0,
        .height = 1,
        .attr = 0,
        .name_len = 0,
    };
    WRITE_VAL(page, h);
}

void bobbyfs_format(CTX)
{
    uint32_t total_page_count = CAPACITY() / BOBBYFS_PAGE_SIZE;
    make_free_list_chain(CTX_ARG_AND 2, total_page_count);
    make_root_directory(CTX_ARG_AND BOBBYFS_PAGE_SIZE);
    struct main m = {
        .kind = PAGEKIND_MAIN,
        .total_page_count = total_page_count,
        .free_page_count = total_page_count - 2,
        .next_free_page = 2 * BOBBYFS_PAGE_SIZE,
    };
    WRITE_VAL(0, m);
}

// --- PAGE ALLOCATION ---

// returns 0 if storage full
static uint32_t alloc_page(CTX)
{
    struct alloc a;
    READ_VAL(offsetof(struct main, free_page_count), &a);
    if (a.free_page_count == 0) {
        return 0;
    }

    uint32_t page = a.next_free_page;
    READ_VAL(page, &a.next_free_page);
    a.free_page_count--;
    WRITE_VAL(offsetof(struct main, free_page_count), a);
    return page;
}

static void free_page(CTX_AND uint32_t page)
{
    struct alloc a;
    READ_VAL(offsetof(struct main, free_page_count), &a);
    WRITE_VAL(page, a.next_free_page);
    a.next_free_page = page;
    a.free_page_count++;
    WRITE_VAL(offsetof(struct main, free_page_count), a);
}

// --- INTERNAL AVL ---

static uint32_t get_height(CTX_AND uint32_t page)
{
    if (page == 0) {
        return 0;
    }

    uint32_t h;
    READ_VAL_HO(page, height, &h);
    return h;
}

static int32_t get_b_factor(CTX_AND uint32_t page)
{
    struct {
        uint32_t left;
        uint32_t right;
    } c;
    READ_VAL_HO(page, left_child, &c);
    int32_t l = get_height(CTX_ARG_AND c.left);
    int32_t r = get_height(CTX_ARG_AND c.right);
    return r - l;
}

static void fix_height(CTX_AND uint32_t page)
{
    struct {
        uint32_t left;
        uint32_t right;
    } c;
    READ_VAL_HO(page, left_child, &c);
    uint32_t l = get_height(CTX_ARG_AND c.left);
    uint32_t r = get_height(CTX_ARG_AND c.right);
    uint32_t h = (l > r ? l : r) + 1;
    WRITE_VAL_HO(page, height, h);
}

static uint32_t rotate_right(CTX_AND uint32_t p)
{
    uint32_t q;
    READ_VAL_HO(p, left_child, &q);
    uint32_t q_right;
    READ_VAL_HO(q, right_child, &q_right);

    WRITE_VAL_HO(p, left_child, q_right);
    WRITE_VAL_HO(q, right_child, p);
    fix_height(CTX_ARG_AND p);
    fix_height(CTX_ARG_AND q);
    return q;
}

static uint32_t rotate_left(CTX_AND uint32_t q)
{
    uint32_t p;
    READ_VAL_HO(q, right_child, &p);
    uint32_t p_left;
    READ_VAL_HO(p, left_child, &p_left);

    WRITE_VAL_HO(q, right_child, p_left);
    WRITE_VAL_HO(p, left_child, q);
    fix_height(CTX_ARG_AND q);
    fix_height(CTX_ARG_AND p);
    return p;
}

static uint32_t balance(CTX_AND uint32_t page)
{
    fix_height(CTX_ARG_AND page);
    int32_t b_factor = get_b_factor(CTX_ARG_AND page);

    if (b_factor == 2) {
        uint32_t right;
        READ_VAL_HO(page, right_child, &right);
        if (get_b_factor(CTX_ARG_AND right) < 0) {
            uint32_t new_right = rotate_right(CTX_ARG_AND right);
            WRITE_VAL_HO(page, right_child, new_right);
        }
        return rotate_left(CTX_ARG_AND page);
    }

    if (b_factor == -2) {
        uint32_t left;
        READ_VAL_HO(page, left_child, &left);
        if (get_b_factor(CTX_ARG_AND left) > 0) {
            uint32_t new_left = rotate_left(CTX_ARG_AND left);
            WRITE_VAL_HO(page, left_child, new_left);
        }
        return rotate_right(CTX_ARG_AND page);
    }

    return page; // no balancing needed
}

static int cmp_names(CTX_AND uint32_t page, const char* name, uint32_t name_len)
{
    uint32_t entry_name_len;
    READ_VAL_HO(page, name_len, &entry_name_len);
#if TESTING
    ck_assert_int_le(name_len, BOBBYFS_MAX_NAME_LEN);
    ck_assert_int_le(entry_name_len, BOBBYFS_MAX_NAME_LEN);
#endif
    if (entry_name_len != name_len) {
        return (int)name_len - (int)entry_name_len;
    }

    char entry_name[BOBBYFS_MAX_NAME_LEN];
    READ(page + sizeof(struct header), entry_name, entry_name_len);
    return memcmp(name, entry_name, name_len);
}

// returns 0 if already exists
static uint32_t
insert_entry(CTX_AND uint32_t p, uint32_t page, const char* entry_name, uint32_t name_len)
{
    if (p == 0) {
        return page;
    }

    int cmp = cmp_names(CTX_ARG_AND p, entry_name, name_len);

    if (cmp < 0) {
        uint32_t left;
        READ_VAL_HO(p, left_child, &left);
        uint32_t new_left = insert_entry(CTX_ARG_AND left, page, entry_name, name_len);
        if (new_left == 0) {
            return 0; // already exists
        }
        WRITE_VAL_HO(p, left_child, new_left);
    } else if (cmp > 0) {
        uint32_t right;
        READ_VAL_HO(p, right_child, &right);
        uint32_t new_right = insert_entry(CTX_ARG_AND right, page, entry_name, name_len);
        if (new_right == 0) {
            return 0; // already exists
        }
        WRITE_VAL_HO(p, right_child, new_right);
    } else {
        return 0; // already exists
    }

    return balance(CTX_ARG_AND p);
}

static uint32_t find_min(CTX_AND uint32_t p)
{
    while (1) {
        uint32_t left;
        READ_VAL_HO(p, left_child, &left);
        if (left == 0) {
            return p;
        }
        p = left;
    }
}

static uint32_t remove_min(CTX_AND uint32_t p)
{
    uint32_t left;
    READ_VAL_HO(p, left_child, &left);
    if (left == 0) {
        uint32_t right;
        READ_VAL_HO(p, right_child, &right);
        return right;
    }

    uint32_t new_left = remove_min(CTX_ARG_AND left);
    WRITE_VAL_HO(p, left_child, new_left);
    return balance(CTX_ARG_AND p);
}

// required for remove_entry
static bobbyfs_file file_new(CTX_AND uint32_t page);

// returns bobbyfs_error
static int remove_entry(CTX_AND uint32_t* p, const char* entry_name, uint32_t name_len)
{
    if (*p == 0) {
        return BOBBYFS_NOT_FOUND;
    }

    int cmp = cmp_names(CTX_ARG_AND * p, entry_name, name_len);

    if (cmp < 0) {
        uint32_t left;
        READ_VAL_HO(*p, left_child, &left);
        int error = remove_entry(CTX_ARG_AND & left, entry_name, name_len);
        if (error != BOBBYFS_OK) {
            return error;
        }

        WRITE_VAL_HO(*p, left_child, left);
    } else if (cmp > 0) {
        uint32_t right;
        READ_VAL_HO(*p, right_child, &right);
        int error = remove_entry(CTX_ARG_AND & right, entry_name, name_len);
        if (error != BOBBYFS_OK) {
            return error;
        }

        WRITE_VAL_HO(*p, right_child, right);
    } else {
        struct {
            uint32_t left;
            uint32_t right;
        } c;
        READ_VAL_HO(*p, left_child, &c);

        // Before delete logic
        uint32_t in_use;
        READ_VAL_HO(*p, in_use, &in_use);
        if (in_use != 0) {
            return BOBBYFS_IN_USE;
        }

        uint32_t kind;
        READ_VAL_HO(*p, kind, &kind);
        if (kind == PAGEKIND_FILE) {
            bobbyfs_file file = file_new(CTX_ARG_AND * p);
            bobbyfs_truncate(CTX_ARG_AND & file);
        } else { // PAGEKIND_DIR
#if TESTING
            ck_assert_int_eq(kind, PAGEKIND_DIR);
#endif
            uint32_t contents;
            READ_VAL_HO(*p, attr, &contents);
            if (contents != 0) {
                return BOBBYFS_DIRECTORY_NOT_EMPTY;
            }
        }

        free_page(CTX_ARG_AND * p);

        if (c.right == 0) {
            *p = c.left;
            return BOBBYFS_OK;
        }

        uint32_t min = find_min(CTX_ARG_AND c.right);
        uint32_t new_right = remove_min(CTX_ARG_AND c.right);
        WRITE_VAL_HO(min, right_child, new_right);
        WRITE_VAL_HO(min, left_child, c.left);
        *p = balance(CTX_ARG_AND min);
        return BOBBYFS_OK;
    }

    *p = balance(CTX_ARG_AND * p);
    return BOBBYFS_OK;
}

// returns 0 if not found
static uint32_t find_entry(CTX_AND uint32_t p, const char* entry_name, uint32_t name_len)
{
    if (p == 0) {
        return 0;
    }

    int cmp = cmp_names(CTX_ARG_AND p, entry_name, name_len);
    if (cmp < 0) {
        uint32_t left;
        READ_VAL_HO(p, left_child, &left);
        return find_entry(CTX_ARG_AND left, entry_name, name_len);
    } else if (cmp > 0) {
        uint32_t right;
        READ_VAL_HO(p, right_child, &right);
        return find_entry(CTX_ARG_AND right, entry_name, name_len);
    } else {
        return p;
    }
}

// returns 0 if storage full
static uint32_t
make_entry(CTX_AND uint32_t parent, uint32_t kind, const char* name, uint32_t name_len)
{
    uint32_t entry = alloc_page(CTX_ARG);
    if (entry == 0) {
        return 0;
    }

    struct header h = {
        .kind = kind,
        .parent = parent,
        .in_use = 0,
        .left_child = 0,
        .right_child = 0,
        .height = 1,
        .attr = 0,
        .name_len = name_len,
    };
    WRITE_VAL(entry, h);
    WRITE(entry + sizeof(struct header), name, name_len);

    return entry;
}

// returns bobbyfs_error
static int add_entry(
    CTX_AND uint32_t dir,
    uint32_t kind,
    const char* name,
    uint32_t name_len,
    uint32_t* result)
{
    uint32_t entry = make_entry(CTX_ARG_AND dir, kind, name, name_len);
    if (entry == 0) {
        return BOBBYFS_STORAGE_FULL;
    }

    uint32_t contents;
    READ_VAL_HO(dir, attr, &contents);

    uint32_t new_root = insert_entry(CTX_ARG_AND contents, entry, name, name_len);
    if (new_root == 0) {
        free_page(CTX_ARG_AND entry);
        return BOBBYFS_ALREADY_EXISTS;
    }

    WRITE_VAL_HO(dir, attr, new_root);
    *result = entry;
    return BOBBYFS_OK;
}

// --- IN USE FUNCTIONS ---

// returns bobbyfs_error
static int try_set_iter_in_use(CTX_AND uint32_t page)
{
    uint32_t count;
    READ_VAL_HO(page, in_use, &count);
    if (count >> 31 != 0) {
        return BOBBYFS_IN_USE;
    }

    count |= 1u << 31;
    WRITE_VAL_HO(page, in_use, count);

    return BOBBYFS_OK;
}

static void unset_iter_in_use(CTX_AND uint32_t page)
{
    uint32_t count;
    READ_VAL_HO(page, in_use, &count);
    count &= ~(1u << 31);
    WRITE_VAL_HO(page, in_use, count);
}

// returns bobbyfs_error
static int get_iter_in_use(CTX_AND uint32_t page)
{
    uint32_t count;
    READ_VAL_HO(page, in_use, &count);
    if (count >> 31 != 0) {
        return BOBBYFS_IN_USE;
    }

    return BOBBYFS_OK;
}

// returns bobbyfs_error
static int try_increment_in_use(CTX_AND uint32_t page)
{
    uint32_t count;
    READ_VAL_HO(page, in_use, &count);
    if (count > 0) {
        return BOBBYFS_IN_USE;
    }

    count++;
    WRITE_VAL_HO(page, in_use, count);
    return BOBBYFS_OK;
}

static void increment_in_use(CTX_AND uint32_t page)
{
    uint32_t count;
    READ_VAL_HO(page, in_use, &count);
    count++;
    WRITE_VAL_HO(page, in_use, count);
}

static void decrement_in_use(CTX_AND uint32_t page)
{
    uint32_t count;
    READ_VAL_HO(page, in_use, &count);
    count--;
    WRITE_VAL_HO(page, in_use, count);
}

// --- DATA PAGE FUNCTIONS ---

static uint32_t get_inline_file_data_page(CTX_AND uint32_t file, uint32_t index)
{
#if TESTING
    ck_assert_int_lt(index, MAX_INLINE_FILE_DATA_PAGES);
#endif
    uint32_t page;
    READ_VAL(file + INLINE_PAGES_OFFSET + index * sizeof(uint32_t), &page);
    return page;
}

// returns 0 if storage full
static uint32_t alloc_inline_file_data_page(CTX_AND uint32_t file, uint32_t index)
{
#if TESTING
    ck_assert_int_lt(index, MAX_INLINE_FILE_DATA_PAGES);
#endif
    uint32_t data_page = alloc_page(CTX_ARG);
    if (data_page == 0) {
        return 0;
    }

    WRITE_VAL(file + INLINE_PAGES_OFFSET + index * sizeof(uint32_t), data_page);
    return data_page;
}

static uint32_t get_page_list_file_data_page(CTX_AND uint32_t page_list, uint32_t index)
{
#if TESTING
    ck_assert_int_lt(index, MAX_PAGE_LIST_FILE_DATE_PAGES);
#endif
    uint32_t page;
    READ_VAL(page_list + index * sizeof(uint32_t), &page);
    return page;
}

// returns 0 if storage full
static uint32_t alloc_page_list_file_data_page(CTX_AND uint32_t page_list, uint32_t index)
{
#if TESTING
    ck_assert_int_lt(index, MAX_PAGE_LIST_FILE_DATE_PAGES);
#endif
    uint32_t data_page = alloc_page(CTX_ARG);
    if (data_page == 0) {
        return 0;
    }

    WRITE_VAL(page_list + index * sizeof(uint32_t), data_page);
    return data_page;
}

// returns 0 if storage full
static uint32_t alloc_next_pages(CTX_AND uint32_t page)
{
    uint32_t next_pages = alloc_page(CTX_ARG);
    if (next_pages == 0) {
        return 0;
    }

    WRITE_VAL(page + NEXT_PAGES_OFFSET, next_pages);
    WRITE_VAL(next_pages + PREV_PAGES_OFFSET, page);
    return next_pages;
}
// --- FILE ---

static uint32_t size_to_pages(uint32_t size)
{
    return (size + BOBBYFS_PAGE_SIZE - 1) / BOBBYFS_PAGE_SIZE;
}

static bobbyfs_file file_new(CTX_AND uint32_t page)
{
    uint32_t size;
    READ_VAL_HO(page, attr, &size);
    bobbyfs_file file = {
        .file = page,
        .cursor = 0,
        .cursor_in_page = BOBBYFS_PAGE_SIZE,
        .cursor_page_index = UINT32_MAX,
        .size_in_pages = size_to_pages(size),
        .file_data_page = 0,
    };
    file.page_cursor.type = BOBBYFS_NONE;
    return file;
}

// returns bobbyfs_error
static int file_try_new(CTX_AND uint32_t page, bobbyfs_file* result)
{
    uint32_t kind;
    READ_VAL_HO(page, kind, &kind);
    if (kind != PAGEKIND_FILE) {
        return BOBBYFS_IS_A_DIRECTORY;
    }

    int error = try_increment_in_use(CTX_ARG_AND page);
    if (error != BOBBYFS_OK) {
        return error;
    }

    *result = file_new(CTX_ARG_AND page);
    return BOBBYFS_OK;
}

// returns bobbyfs_error
static int
file_create(CTX_AND uint32_t dir, const char* name, uint32_t name_len, bobbyfs_file* result)
{
    if (*name == 0) {
        return BOBBYFS_EMPTY_NAME;
    }

    int error = get_iter_in_use(CTX_ARG_AND dir);
    if (error != BOBBYFS_OK) {
        return error;
    }

    uint32_t page;
    error = add_entry(CTX_ARG_AND dir, PAGEKIND_FILE, name, name_len, &page);
    if (error != BOBBYFS_OK) {
        return error;
    }

    increment_in_use(CTX_ARG_AND page);
    *result = file_new(CTX_ARG_AND page);
    return BOBBYFS_OK;
}

// returns bobbyfs_error
static int file_open(CTX_AND uint32_t dir, const char* name, size_t name_len, bobbyfs_file* result)
{
    uint32_t contents;
    READ_VAL_HO(dir, attr, &contents);
    int page = find_entry(CTX_ARG_AND contents, name, name_len);
    if (page == 0) {
        return BOBBYFS_NOT_FOUND;
    }

    return file_try_new(CTX_ARG_AND page, result);
}

// moves to the next page, assuming that it exists
// if it moves from page list to the next page list,
// returns the page list it moved from
static uint32_t next_page(CTX_AND bobbyfs_file* file)
{
    uint32_t ret = 0;
    if (file->page_cursor.type == BOBBYFS_NONE) {
        file->page_cursor.type = BOBBYFS_FILE_INLINE;
        file->page_cursor.file_inline.index = 0;
        file->file_data_page = get_inline_file_data_page(CTX_ARG_AND file->file, 0);
    } else if (file->page_cursor.type == BOBBYFS_FILE_INLINE) {
        uint32_t index = ++file->page_cursor.file_inline.index;
        if (index >= MAX_INLINE_FILE_DATA_PAGES) {
            uint32_t page_list;
            READ_VAL(file->file + NEXT_PAGES_OFFSET, &page_list);
            file->file_data_page = get_page_list_file_data_page(CTX_ARG_AND page_list, 0);
            file->page_cursor.type = BOBBYFS_PAGE_LIST;
            file->page_cursor.page_list.index = 0;
            file->page_cursor.page_list.page_list_index = 0;
            file->page_cursor.page_list.page_list = page_list;
        } else {
            file->file_data_page = get_inline_file_data_page(CTX_ARG_AND file->file, index);
        }
    } else { // BOBBYFS_PAGE_LIST
#if TESTING
        ck_assert_int_eq(file->page_cursor.type, BOBBYFS_PAGE_LIST);
#endif
        uint32_t index = ++file->page_cursor.page_list.index;
        if (index >= MAX_PAGE_LIST_FILE_DATE_PAGES) {
            ret = file->page_cursor.page_list.page_list;
            file->page_cursor.page_list.index = 0;
            file->page_cursor.page_list.page_list_index += 1;
            READ_VAL(
                file->page_cursor.page_list.page_list + NEXT_PAGES_OFFSET,
                &file->page_cursor.page_list.page_list);
            file->file_data_page =
                get_page_list_file_data_page(CTX_ARG_AND file->page_cursor.page_list.page_list, 0);
        } else {
            file->file_data_page = get_page_list_file_data_page(
                CTX_ARG_AND file->page_cursor.page_list.page_list,
                index);
        }
    }

    file->cursor_in_page = 0;
    file->cursor_page_index++;
    return ret;
}

// moves to the next page, possibly allocating it
// returns bobbyfs_error
static int next_page_alloc(CTX_AND bobbyfs_file* file)
{
    if (file->page_cursor.type == BOBBYFS_NONE) {
        // file is empty, allocate first page
        uint32_t new_file_data_page;
        if (file->size_in_pages == 0) {
            new_file_data_page = alloc_inline_file_data_page(CTX_ARG_AND file->file, 0);
            if (new_file_data_page == 0) {
                return BOBBYFS_STORAGE_FULL;
            }
            file->size_in_pages = 1;
        } else {
            new_file_data_page = get_inline_file_data_page(CTX_ARG_AND file->file, 0);
        }

        file->page_cursor.type = BOBBYFS_FILE_INLINE;
        file->page_cursor.file_inline.index = 0;
        file->file_data_page = new_file_data_page;
        file->cursor_page_index = 0;
    } else if (file->page_cursor.type == BOBBYFS_FILE_INLINE) {
        uint32_t new_cursor_page_index = file->cursor_page_index + 1;
        uint32_t new_index = file->page_cursor.file_inline.index + 1;

        if (new_index >= MAX_INLINE_FILE_DATA_PAGES) {
            // transition to page list
            uint32_t page_list;
            uint32_t new_file_data_page;

            if (file->size_in_pages == new_cursor_page_index) {
                page_list = alloc_next_pages(CTX_ARG_AND file->file);
                if (page_list == 0) {
                    return BOBBYFS_STORAGE_FULL;
                }

                new_file_data_page = alloc_page_list_file_data_page(CTX_ARG_AND page_list, 0);
                if (new_file_data_page == 0) {
                    free_page(CTX_ARG_AND page_list);
                    return BOBBYFS_STORAGE_FULL;
                }
                file->size_in_pages++;
            } else {
                READ_VAL(file->file + NEXT_PAGES_OFFSET, &page_list);
                new_file_data_page = get_page_list_file_data_page(CTX_ARG_AND page_list, 0);
            }

            file->file_data_page = new_file_data_page;
            file->cursor_page_index = new_cursor_page_index;
            file->page_cursor.type = BOBBYFS_PAGE_LIST;
            file->page_cursor.page_list.index = 0;
            file->page_cursor.page_list.page_list_index = 0;
            file->page_cursor.page_list.page_list = page_list;
        } else {
            // stay in inline mode
            uint32_t new_file_data_page;
            if (file->size_in_pages == new_cursor_page_index) {
                new_file_data_page = alloc_inline_file_data_page(CTX_ARG_AND file->file, new_index);
                if (new_file_data_page == 0) {
                    return BOBBYFS_STORAGE_FULL;
                }
                file->size_in_pages++;
            } else {
                new_file_data_page = get_inline_file_data_page(CTX_ARG_AND file->file, new_index);
            }

            file->page_cursor.file_inline.index = new_index;
            file->file_data_page = new_file_data_page;
            file->cursor_page_index = new_cursor_page_index;
        }
    } else { // BOBBYFS_PAGE_LIST
#if TESTING
        ck_assert_int_eq(file->page_cursor.type, BOBBYFS_PAGE_LIST);
#endif
        uint32_t new_cursor_page_index = file->cursor_page_index + 1;
        uint32_t new_index = file->page_cursor.page_list.index + 1;

        if (new_index >= MAX_PAGE_LIST_FILE_DATE_PAGES) {
            // transition to next page list
            uint32_t new_page_list;
            uint32_t new_file_data_page;

            if (file->size_in_pages == new_cursor_page_index) {
                new_page_list = alloc_next_pages(CTX_ARG_AND file->page_cursor.page_list.page_list);
                if (new_page_list == 0) {
                    return BOBBYFS_STORAGE_FULL;
                }

                new_file_data_page = alloc_page_list_file_data_page(CTX_ARG_AND new_page_list, 0);
                if (new_file_data_page == 0) {
                    free_page(CTX_ARG_AND new_page_list);
                    return BOBBYFS_STORAGE_FULL;
                }
                file->size_in_pages++;
            } else {
                READ_VAL(file->page_cursor.page_list.page_list + NEXT_PAGES_OFFSET, &new_page_list);
                new_file_data_page = get_page_list_file_data_page(CTX_ARG_AND new_page_list, 0);
            }

            file->page_cursor.page_list.index = 0;
            file->page_cursor.page_list.page_list_index++;
            file->page_cursor.page_list.page_list = new_page_list;
            file->file_data_page = new_file_data_page;
            file->cursor_page_index = new_cursor_page_index;
        } else {
            // stay in current page list
            uint32_t new_file_data_page;
            if (file->size_in_pages == new_cursor_page_index) {
                new_file_data_page = alloc_page_list_file_data_page(
                    CTX_ARG_AND file->page_cursor.page_list.page_list,
                    new_index);
                if (new_file_data_page == 0) {
                    return BOBBYFS_STORAGE_FULL;
                }
                file->size_in_pages++;
            } else {
                new_file_data_page = get_page_list_file_data_page(
                    CTX_ARG_AND file->page_cursor.page_list.page_list,
                    new_index);
            }

            file->page_cursor.page_list.index = new_index;
            file->file_data_page = new_file_data_page;
            file->cursor_page_index = new_cursor_page_index;
        }
    }

    file->cursor_in_page = 0;
    return BOBBYFS_OK;
}
// --- FILE PUBLIC ---

// returns bobbyfs_error
int bobbyfs_read(CTX_AND bobbyfs_file* file, void* buf, uint32_t size)
{
    uint32_t file_size;
    READ_VAL_HO(file->file, attr, &file_size);
    if (size + file->cursor > file_size) {
        return BOBBYFS_EOF;
    }

    uint32_t to_read = size;
    uint32_t read = 0;

    while (read < to_read) {
        if (file->cursor_in_page == BOBBYFS_PAGE_SIZE) {
            next_page(CTX_ARG_AND file);
        }

        uint32_t to_read = BOBBYFS_PAGE_SIZE - file->cursor_in_page;
        if (size < to_read) {
            to_read = size;
        }

        READ(file->file_data_page + file->cursor_in_page, buf, to_read);
        file->cursor_in_page += to_read;
        read += to_read;
        buf += to_read;
        size -= to_read;
    }

    file->cursor += to_read;
    return BOBBYFS_OK;
}

// returns bobbyfs_error
int bobbyfs_write(CTX_AND bobbyfs_file* file, const void* buf, uint32_t size)
{
    uint32_t to_write = size;
    uint32_t written = 0;
    int error = BOBBYFS_OK;

    while (written < to_write) {
        if (file->cursor_in_page == BOBBYFS_PAGE_SIZE) {
            error = next_page_alloc(CTX_ARG_AND file);
            if (error != BOBBYFS_OK) {
                break;
            }
        }

        uint32_t to_write = BOBBYFS_PAGE_SIZE - file->cursor_in_page;
        if (size < to_write) {
            to_write = size;
        }

        WRITE(file->file_data_page + file->cursor_in_page, buf, to_write);

        file->cursor_in_page += to_write;
        written += to_write;
        buf += to_write;
        size -= to_write;
    }

    file->cursor += written;

    uint32_t file_size;
    READ_VAL_HO(file->file, attr, &file_size);
    if (file->cursor > file_size) {
        WRITE_VAL_HO(file->file, attr, file->cursor);
    }

    return error;
}

void bobbyfs_truncate(CTX_AND bobbyfs_file* file)
{
    uint32_t cursor_page_index = file->cursor_page_index;
    uint32_t size_in_pages = file->size_in_pages;
    struct bobbyfs_page_cursor page_cursor = file->page_cursor;
    uint32_t file_data_page = file->file_data_page;
    uint32_t cursor_in_page = file->cursor_in_page;

    uint32_t last_removed = 0;
    for (uint32_t i = cursor_page_index + 1; i < size_in_pages; i++) {
        uint32_t page_list = next_page(CTX_ARG_AND file);
        if (page_list != 0) {
            free_page(CTX_ARG_AND page_list);
            last_removed = page_list;
        }

        free_page(CTX_ARG_AND file->file_data_page);
    }

    if (file->page_cursor.type == BOBBYFS_PAGE_LIST &&
        file->page_cursor.page_list.page_list != last_removed) {
        free_page(CTX_ARG_AND file->page_cursor.page_list.page_list);
    }

    file->cursor_page_index = cursor_page_index;
    file->size_in_pages = cursor_page_index + 1;
    file->page_cursor = page_cursor;
    file->file_data_page = file_data_page;
    file->cursor_in_page = cursor_in_page;

    WRITE_VAL_HO(file->file, attr, file->cursor);
}

// returns bobbyfs_error
int bobbyfs_seek_from_current(CTX_AND bobbyfs_file* file, int32_t offset)
{
    uint32_t file_size;
    READ_VAL_HO(file->file, attr, &file_size);
    int32_t final_cursor = (int32_t)file->cursor + offset;
    if (final_cursor < 0 || (uint32_t)final_cursor > file_size) {
        return BOBBYFS_INVALID_INPUT;
    }

    int32_t final_page = final_cursor / BOBBYFS_PAGE_SIZE - (final_cursor % BOBBYFS_PAGE_SIZE == 0);
    int32_t page_diff = final_page - (int32_t)file->cursor_page_index;

    if (page_diff > 0) {
        // forward
        uint32_t page_diff2 = (uint32_t)page_diff;
        while (1) {
            if (file->page_cursor.type == BOBBYFS_NONE) {
                file->page_cursor.type = BOBBYFS_FILE_INLINE;
                file->page_cursor.file_inline.index = 0;
                page_diff2--;
            } else if (file->page_cursor.type == BOBBYFS_FILE_INLINE) {
                uint32_t max_forward =
                    MAX_INLINE_FILE_DATA_PAGES - file->page_cursor.file_inline.index - 1;
                if (max_forward >= page_diff2) {
                    file->page_cursor.file_inline.index += page_diff2;
                    file->file_data_page = get_inline_file_data_page(
                        CTX_ARG_AND file->file,
                        file->page_cursor.file_inline.index);
                    break;
                } else {
                    file->page_cursor.type = BOBBYFS_PAGE_LIST;
                    file->page_cursor.page_list.index = 0;
                    file->page_cursor.page_list.page_list_index = 0;
                    READ_VAL(
                        file->file + NEXT_PAGES_OFFSET,
                        &file->page_cursor.page_list.page_list);
                    page_diff2 -= max_forward + 1;
                }
            } else { // BOBBYFS_PAGE_LIST
#if TESTING
                ck_assert_int_eq(file->page_cursor.type, BOBBYFS_PAGE_LIST);
#endif
                while (1) {
                    uint32_t max_forward =
                        MAX_PAGE_LIST_FILE_DATE_PAGES - file->page_cursor.page_list.index - 1;
                    if (max_forward >= page_diff2) {
                        file->page_cursor.page_list.index += page_diff2;
                        file->file_data_page = get_page_list_file_data_page(
                            CTX_ARG_AND file->page_cursor.page_list.page_list,
                            file->page_cursor.page_list.index);
                        goto finish;
                    } else {
                        file->page_cursor.page_list.index = 0;
                        file->page_cursor.page_list.page_list_index++;
                        READ_VAL(
                            file->page_cursor.page_list.page_list + NEXT_PAGES_OFFSET,
                            &file->page_cursor.page_list.page_list);
                        page_diff2 -= max_forward + 1;
                    }
                }
            }
        }
    } else {
        // page_diff <= 0
        // backward
        uint32_t page_diff2 = (uint32_t)(-page_diff);
        while (1) {
            if (file->page_cursor.type == BOBBYFS_NONE) {
// this branch is when page_diff <= 0
// if page_cursor is none, then page_diff must be 0
#if TESTING
                ck_assert_int_eq(page_diff, 0);
#endif
                break;
            } else if (file->page_cursor.type == BOBBYFS_FILE_INLINE) {
                if (page_diff2 > file->page_cursor.file_inline.index) {
#if TESTING
                    ck_assert_int_eq(page_diff2 - 1, file->page_cursor.file_inline.index);
#endif
                    file->page_cursor.type = BOBBYFS_NONE;
                } else {
                    file->page_cursor.file_inline.index -= page_diff2;
                    file->file_data_page = get_inline_file_data_page(
                        CTX_ARG_AND file->file,
                        file->page_cursor.file_inline.index);
                }
                break;
            } else { // BOBBYFS_PAGE_LIST
#if TESTING
                ck_assert_int_eq(file->page_cursor.type, BOBBYFS_PAGE_LIST);
#endif
                while (1) {
                    if (page_diff2 > file->page_cursor.page_list.index) {
                        if (file->page_cursor.page_list.page_list_index == 0) {
                            page_diff2 -= file->page_cursor.page_list.index + 1;
                            file->page_cursor.type = BOBBYFS_FILE_INLINE;
                            file->page_cursor.file_inline.index = MAX_INLINE_FILE_DATA_PAGES - 1;
                            break;
                        } else {
                            page_diff2 -= file->page_cursor.page_list.index + 1;
                            file->page_cursor.page_list.index = MAX_PAGE_LIST_FILE_DATE_PAGES - 1;
                            file->page_cursor.page_list.page_list_index--;
                            READ_VAL(
                                file->page_cursor.page_list.page_list + PREV_PAGES_OFFSET,
                                &file->page_cursor.page_list.page_list);
                        }
                    } else {
                        file->page_cursor.page_list.index -= page_diff2;
                        file->file_data_page = get_page_list_file_data_page(
                            CTX_ARG_AND file->page_cursor.page_list.page_list,
                            file->page_cursor.page_list.index);
                        goto finish;
                    }
                }
            }
        }
    }

finish:;
    uint32_t final_cursor_in_page = (uint32_t)final_cursor % BOBBYFS_PAGE_SIZE;
    if (final_cursor_in_page == 0) {
        file->cursor_in_page = BOBBYFS_PAGE_SIZE;
    } else {
        file->cursor_in_page = final_cursor_in_page;
    }

    file->cursor_page_index = (uint32_t)final_page;
    file->cursor = (uint32_t)final_cursor;
    return BOBBYFS_OK;
}

int bobbyfs_seek_from_start(CTX_AND bobbyfs_file* file, uint32_t offset)
{
    int32_t rel_pos = (int32_t)offset - (int32_t)file->cursor;
    return bobbyfs_seek_from_current(CTX_ARG_AND file, rel_pos);
}

int bobbyfs_seek_from_end(CTX_AND bobbyfs_file* file, uint32_t offset)
{
    uint32_t file_size;
    READ_VAL_HO(file->file, attr, &file_size);
    uint32_t pos = file_size - offset;
    return bobbyfs_seek_from_start(CTX_ARG_AND file, pos);
}

uint32_t bobbyfs_file_size(CTX_AND bobbyfs_file* file)
{
    uint32_t file_size;
    READ_VAL_HO(file->file, attr, &file_size);
    return file_size;
}

void bobbyfs_file_close(CTX_AND bobbyfs_file* file)
{
    decrement_in_use(CTX_ARG_AND file->file);
}

// --- DIRECTORY ---

static bobbyfs_dir dir_new(CTX_AND uint32_t page)
{
    increment_in_use(CTX_ARG_AND page);
    bobbyfs_dir dir = {.page = page};
    return dir;
}

// returns bobbyfs_error
static int dir_try_new(CTX_AND uint32_t page, bobbyfs_dir* dir)
{
    uint32_t kind;
    READ_VAL_HO(page, kind, &kind);
    if (kind != PAGEKIND_DIR) {
        return BOBBYFS_NOT_A_DIRECTORY;
    }

    dir->page = page;
    return BOBBYFS_OK;
}

// returns bobbyfs_error
static int
dir_create(CTX_AND uint32_t dir, const char* name, uint32_t name_len, bobbyfs_dir* result)
{
    if (*name == 0) {
        return BOBBYFS_EMPTY_NAME;
    }

    int error = get_iter_in_use(CTX_ARG_AND dir);
    if (error != BOBBYFS_OK) {
        return error;
    }

    uint32_t page;
    error = add_entry(CTX_ARG_AND dir, PAGEKIND_DIR, name, name_len, &page);
    if (error != BOBBYFS_OK) {
        return error;
    }

    *result = dir_new(CTX_ARG_AND page);
    return BOBBYFS_OK;
}

// returns bobbyfs_error
static int dir_open(CTX_AND uint32_t dir, const char* name, size_t name_len, bobbyfs_dir* result)
{
    uint32_t contents;
    READ_VAL_HO(dir, attr, &contents);
    int page = find_entry(CTX_ARG_AND contents, name, name_len);
    if (page == 0) {
        return BOBBYFS_NOT_FOUND;
    }

    return dir_try_new(CTX_ARG_AND page, result);
}

// returns bobbyfs_error
static int
dir_get_page_and_name(CTX_AND uint32_t* dir, const char** path, uint32_t* name_len_result)
{
    while (1) {
        const char* ppath = *path;
        const char* slash_or_nul = strchrnul(ppath, '/');
        size_t name_len = slash_or_nul - ppath;

        if (*slash_or_nul == 0) {
            *name_len_result = name_len;
            return BOBBYFS_OK;
        }

        if (name_len == 2 && ppath[0] == '.' && ppath[1] == '.') {
            uint32_t parent;
            READ_VAL_HO(*dir, parent, &parent);
            if (parent != 0) {
                *dir = parent;
            }
        } else {
            uint32_t contents;
            READ_VAL_HO(*dir, attr, &contents);
            uint32_t entry = find_entry(CTX_ARG_AND contents, ppath, name_len);
            if (entry == 0) {
                return BOBBYFS_NOT_FOUND;
            }

            uint32_t kind;
            READ_VAL_HO(entry, kind, &kind);
            if (kind != PAGEKIND_DIR) {
                return BOBBYFS_NOT_A_DIRECTORY;
            }

            *dir = entry;
        }

        *path = slash_or_nul + 1;
    }
}

// --- DIRECTORY PUBLIC ---

bobbyfs_dir bobbyfs_dir_root(CTX)
{
    return dir_new(CTX_ARG_AND BOBBYFS_PAGE_SIZE);
}

// returns bobbyfs_error
int bobbyfs_file_create(CTX_AND bobbyfs_dir dir, const char* path, bobbyfs_file* result)
{
    uint32_t offset = dir.page;
    uint32_t name_len;
    int error = dir_get_page_and_name(CTX_ARG_AND & offset, &path, &name_len);
    if (error != BOBBYFS_OK) {
        return error;
    }

    return file_create(CTX_ARG_AND offset, path, name_len, result);
}

// returns bobbyfs_error
int bobbyfs_file_open(CTX_AND bobbyfs_dir dir, const char* path, bobbyfs_file* result)
{
    uint32_t offset = dir.page;
    uint32_t name_len;
    int error = dir_get_page_and_name(CTX_ARG_AND & offset, &path, &name_len);
    if (error != BOBBYFS_OK) {
        return error;
    }

    return file_open(CTX_ARG_AND offset, path, name_len, result);
}

// returns bobbyfs_error
int bobbyfs_file_create_or_open(CTX_AND bobbyfs_dir dir, const char* path, bobbyfs_file* result)
{
    uint32_t offset = dir.page;
    uint32_t name_len;
    int error = dir_get_page_and_name(CTX_ARG_AND & offset, &path, &name_len);
    if (error != BOBBYFS_OK) {
        return error;
    }

    error = file_open(CTX_ARG_AND offset, path, name_len, result);
    if (error == BOBBYFS_NOT_FOUND) {
        return file_create(CTX_ARG_AND offset, path, name_len, result);
    }

    return error;
}

// returns bobbyfs_error
int bobbyfs_dir_create(CTX_AND bobbyfs_dir dir, const char* path, bobbyfs_dir* result)
{
    uint32_t offset = dir.page;
    uint32_t name_len;
    int error = dir_get_page_and_name(CTX_ARG_AND & offset, &path, &name_len);
    if (error != BOBBYFS_OK) {
        return error;
    }

    return dir_create(CTX_ARG_AND offset, path, name_len, result);
}

// returns bobbyfs_error
int bobbyfs_dir_open(CTX_AND bobbyfs_dir dir, const char* path, bobbyfs_dir* result)
{
    uint32_t offset = dir.page;
    uint32_t name_len;
    int error = dir_get_page_and_name(CTX_ARG_AND & offset, &path, &name_len);
    if (error != BOBBYFS_OK) {
        return error;
    }

    return dir_open(CTX_ARG_AND offset, path, name_len, result);
}

void bobbyfs_dir_close(CTX_AND bobbyfs_dir dir)
{
    decrement_in_use(CTX_ARG_AND dir.page);
}

// --- DIRECTORY ITERATOR ---

static void dir_iter_go_left_till_the_end(CTX_AND bobbyfs_dir_iter* iter)
{
    while (1) {
        uint32_t next;
        READ_VAL_HO(iter->entry.page, left_child, &next);
        if (next == 0) {
            return;
        }

        iter->stack[iter->stack_size++] = iter->entry.page;
        iter->entry.page = next;
    }
}

// returns 1 if successful, 0 if no right child
static int dir_iter_try_go_right(CTX_AND bobbyfs_dir_iter* iter)
{
    uint32_t next;
    READ_VAL_HO(iter->entry.page, right_child, &next);
    if (next == 0) {
        return 0;
    }

    iter->entry.page = next;
    return 1;
}

static void dir_iter_go_up(CTX_AND bobbyfs_dir_iter* iter)
{
    if (iter->stack_size == 0) {
        iter->entry.page = 0;
        return;
    }

    iter->entry.page = iter->stack[--iter->stack_size];
}

static bobbyfs_dir_iter dir_iter_new_raw(CTX_AND uint32_t dir)
{
    uint32_t contents;
    READ_VAL_HO(dir, attr, &contents);

    if (contents != 0) {
        bobbyfs_dir_iter iter = {
            .entry = {.page = contents},
            .stack_size = 0,
            .dir = dir,
        };

        dir_iter_go_left_till_the_end(CTX_ARG_AND & iter);
        return iter;
    } else {
        bobbyfs_dir_iter iter = {
            .entry = {.page = 0},
            .stack_size = 0,
            .dir = dir,
        };
        return iter;
    }
}

// --- DIRECTORY ITERATOR PUBLIC ---

// returns bobbyfs_error
int bobbyfs_iter_contents(CTX_AND bobbyfs_dir dir, bobbyfs_dir_iter* result)
{
    int error = try_set_iter_in_use(CTX_ARG_AND dir.page);
    if (error != BOBBYFS_OK) {
        return error;
    }

    *result = dir_iter_new_raw(CTX_ARG_AND dir.page);
    return BOBBYFS_OK;
}

void bobbyfs_iter_advance(CTX_AND bobbyfs_dir_iter* iter)
{
    if (dir_iter_try_go_right(CTX_ARG_AND iter)) {
        dir_iter_go_left_till_the_end(CTX_ARG_AND iter);
    } else {
        dir_iter_go_up(CTX_ARG_AND iter);
    }
}

void bobbyfs_iter_close(CTX_AND bobbyfs_dir_iter* iter)
{
    unset_iter_in_use(CTX_ARG_AND iter->dir);
}

// --- DIRECTORY ENTRY PUBLIC ---

void bobbyfs_entry_name(CTX_AND bobbyfs_entry entry, char buf[BOBBYFS_MAX_NAME_LEN + 1])
{
    uint32_t name_len;
    READ_VAL_HO(entry.page, name_len, &name_len);

    READ(entry.page + sizeof(struct header), buf, name_len);
    buf[name_len] = 0;
}
