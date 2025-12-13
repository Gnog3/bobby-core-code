#include "bobbyfs.h"
#include "test_ctx.h"
#include "test_cursor.h"
#include "vector.h"
#include <check.h>
#include <stdio.h>
#include <stdlib.h>

static void fill_random(uint8_t* buf, size_t size)
{
    for (size_t i = 0; i < size; i++) {
        buf[i] = rand() % 256;
    }
}

START_TEST(test_create_single_folder)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    bobbyfs_dir new_dir;
    int error = bobbyfs_dir_create(test_ctx, root, "new_folder", &new_dir);
    ck_assert_int_eq(error, BOBBYFS_OK);
    bobbyfs_dir_close(test_ctx, new_dir);

    bobbyfs_dir_iter iter;
    error = bobbyfs_iter_contents(test_ctx, root, &iter);
    ck_assert_int_eq(error, BOBBYFS_OK);

    int number_of_entries = 0;

    while (iter.entry.page != 0) {
        number_of_entries++;
        char name[BOBBYFS_MAX_NAME_LEN + 1];
        bobbyfs_entry_name(test_ctx, iter.entry, name);
        ck_assert_str_eq(name, "new_folder");
        bobbyfs_iter_advance(test_ctx, &iter);
    }

    ck_assert_int_eq(number_of_entries, 1);
    bobbyfs_iter_close(test_ctx, &iter);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_create_multiple_folders)
{
    for (int n = 2; n < 50; n++) {
        reset_ctx();
        bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

        for (int i = 0; i < n; i++) {
            char folder_name[20];
            sprintf(folder_name, "folder_%d", i);
            bobbyfs_dir new_dir;
            int error = bobbyfs_dir_create(test_ctx, root, folder_name, &new_dir);
            ck_assert_int_eq(error, BOBBYFS_OK);
            bobbyfs_dir_close(test_ctx, new_dir);
        }

        bobbyfs_dir_iter iter;
        int error = bobbyfs_iter_contents(test_ctx, root, &iter);
        ck_assert_int_eq(error, BOBBYFS_OK);
        int number_of_entries = 0;
        while (iter.entry.page != 0) {
            number_of_entries++;
            char name[BOBBYFS_MAX_NAME_LEN + 1];
            bobbyfs_entry_name(test_ctx, iter.entry, name);
            char expected_name[20];
            sprintf(expected_name, "folder_%d", number_of_entries - 1);
            ck_assert_str_eq(name, expected_name);
            bobbyfs_iter_advance(test_ctx, &iter);
        }
        ck_assert_int_eq(number_of_entries, n);
        bobbyfs_iter_close(test_ctx, &iter);
        bobbyfs_dir_close(test_ctx, root);
    }
}

START_TEST(test_double_open)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    bobbyfs_file file;
    int error = bobbyfs_file_create(test_ctx, root, "a file", &file);
    ck_assert_int_eq(error, BOBBYFS_OK);

    bobbyfs_file file2;
    error = bobbyfs_file_open(test_ctx, root, "a file", &file2);
    ck_assert_int_eq(error, BOBBYFS_IN_USE);
    bobbyfs_file_close(test_ctx, &file);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_write_a_byte)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    bobbyfs_file file;
    int error = bobbyfs_file_create(test_ctx, root, "a file", &file);
    ck_assert_int_eq(error, BOBBYFS_OK);
    bobbyfs_file_close(test_ctx, &file);

    vector v;
    vector_init(&v);

    test_cursor c;
    for (int i = 0; i < 257; i++) {
        {
            c = test_cursor_open(test_ctx, root, &v, "a file");
            uint8_t byte = (uint8_t)i;
            test_cursor_write(test_ctx, &c, &byte, 1);
            bobbyfs_file_close(test_ctx, &c.file);
        }
        {
            c = test_cursor_open(test_ctx, root, &v, "a file");
            test_cursor_read(test_ctx, &c, 1);
            bobbyfs_file_close(test_ctx, &c.file);
        }
    }

    free(v.data);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_write_a_byte_files)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    const int number_of_files = 256;
    vector v[number_of_files];

    for (int i = 0; i < number_of_files; i++) {
        char name[20];
        sprintf(name, "file_%d", i);
        test_cursor c = test_cursor_create(test_ctx, root, &v[i], name);
        uint8_t byte = (uint8_t)i;
        test_cursor_write(test_ctx, &c, &byte, 1);
        bobbyfs_file_close(test_ctx, &c.file);
    }

    for (int i = 0; i < number_of_files; i++) {
        char name[20];
        sprintf(name, "file_%d", i);
        test_cursor c = test_cursor_open(test_ctx, root, &v[i], name);
        test_cursor_read(test_ctx, &c, 1);
        bobbyfs_file_close(test_ctx, &c.file);
        free(v[i].data);
    }

    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_write_a_random_page)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    uint8_t buf[BOBBYFS_PAGE_SIZE];
    fill_random(buf, BOBBYFS_PAGE_SIZE);

    vector v;

    {
        test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
        test_cursor_write(test_ctx, &c, buf, BOBBYFS_PAGE_SIZE);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        test_cursor_read(test_ctx, &c, BOBBYFS_PAGE_SIZE);
        bobbyfs_file_close(test_ctx, &c.file);
    }

    free(v.data);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_write_a_megabyte)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    const size_t size = 1024 * 1024;
    uint8_t* buf = malloc(size);
    fill_random(buf, size);

    vector v;

    {
        test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
        test_cursor_write(test_ctx, &c, buf, size);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        test_cursor_read(test_ctx, &c, size);
        bobbyfs_file_close(test_ctx, &c.file);
    }

    free(v.data);
    free(buf);
    bobbyfs_dir_close(test_ctx, root);
}

START_TEST(test_write_random_files)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    const int number_of_files = 100;
    const size_t size = 8 * 1024;

    uint8_t* buf = malloc(size * number_of_files);
    fill_random(buf, size * number_of_files);
    vector v[number_of_files];
    for (int i = 0; i < number_of_files; i++) {
        char name[20];
        sprintf(name, "file_%d", i);
        test_cursor c = test_cursor_create(test_ctx, root, &v[i], name);
        test_cursor_write(test_ctx, &c, buf + i * size, size);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    for (int i = 0; i < number_of_files; i++) {
        char name[20];
        sprintf(name, "file_%d", i);
        test_cursor c = test_cursor_open(test_ctx, root, &v[i], name);
        test_cursor_read(test_ctx, &c, size);
        bobbyfs_file_close(test_ctx, &c.file);
        free(v[i].data);
    }
    free(buf);
    bobbyfs_dir_close(test_ctx, root);
}

START_TEST(test_write_random_files_folders)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    const int number_of_folders = 5;
    const int number_of_files = 5;
    const size_t size = 32 * 1024;

    uint8_t* buf = malloc(size * number_of_folders * number_of_files);
    fill_random(buf, size * number_of_folders * number_of_files);
    vector v[number_of_folders][number_of_files];
    for (int i = 0; i < number_of_folders; i++) {
        char folder_name[20];
        sprintf(folder_name, "folder_%d", i);
        bobbyfs_dir folder;
        int error = bobbyfs_dir_create(test_ctx, root, folder_name, &folder);
        ck_assert_int_eq(error, BOBBYFS_OK);

        for (int j = 0; j < number_of_files; j++) {
            char file_name[20];
            sprintf(file_name, "file_%d", j);
            test_cursor c = test_cursor_create(test_ctx, folder, &v[i][j], file_name);
            test_cursor_write(test_ctx, &c, buf + (i * number_of_files + j) * size, size);
            bobbyfs_file_close(test_ctx, &c.file);
        }
        bobbyfs_dir_close(test_ctx, folder);
    }
    for (int i = 0; i < number_of_folders; i++) {
        for (int j = 0; j < number_of_files; j++) {
            char file_name[30];
            sprintf(file_name, "folder_%d/file_%d", i, j);
            test_cursor c = test_cursor_open(test_ctx, root, &v[i][j], file_name);
            test_cursor_read(test_ctx, &c, size);
            bobbyfs_file_close(test_ctx, &c.file);
            free(v[i][j].data);
        }
    }
    free(buf);
    bobbyfs_dir_close(test_ctx, root);
}

START_TEST(test_rewrite_file_parts)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    const size_t size = 128 * 1024;
    uint8_t* buf = malloc(size);
    fill_random(buf, size);

    vector v;

    {
        test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
        test_cursor_write(test_ctx, &c, buf, size);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        size_t remaining = size;
        while (remaining > 0) {
            size_t chunk_size = (rand() % 128) + 1;
            if (chunk_size > remaining) {
                chunk_size = remaining;
            }

            test_cursor_read(test_ctx, &c, chunk_size);
            remaining -= chunk_size;
        }
        bobbyfs_file_close(test_ctx, &c.file);
    }
    fill_random(buf, size);
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        size_t remaining = size;
        while (remaining > 0) {
            size_t chunk_size = (rand() % 16) + 1;
            if (chunk_size > remaining) {
                chunk_size = remaining;
            }

            test_cursor_write(test_ctx, &c, buf + (size - remaining), chunk_size);
            remaining -= chunk_size;
        }
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        size_t remaining = size;
        while (remaining > 0) {
            size_t chunk_size = (rand() % 16) + 1;
            if (chunk_size > remaining) {
                chunk_size = remaining;
            }

            test_cursor_read(test_ctx, &c, chunk_size);
            remaining -= chunk_size;
        }
        bobbyfs_file_close(test_ctx, &c.file);
    }
    free(v.data);
    free(buf);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_truncate_empty)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    vector v;
    test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
    test_cursor_truncate(test_ctx, &c);
    bobbyfs_file_close(test_ctx, &c.file);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_truncate_page)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    uint8_t buf[BOBBYFS_PAGE_SIZE];
    fill_random(buf, BOBBYFS_PAGE_SIZE);

    vector v;

    {
        test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
        test_cursor_write(test_ctx, &c, buf, BOBBYFS_PAGE_SIZE);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        test_cursor_read(test_ctx, &c, 200);
        test_cursor_truncate(test_ctx, &c);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        test_cursor_read(test_ctx, &c, 200);
        int error = bobbyfs_read(test_ctx, &c.file, buf, 1);
        ck_assert_int_eq(error, BOBBYFS_EOF);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    free(v.data);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_truncate_huge_file_full)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    const size_t size = 1024 * 1024;
    uint8_t* buf = malloc(size);
    fill_random(buf, size);

    vector v;
    uint32_t free_pages;

    {
        test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
        free_pages = bobbyfs_get_free_page_count(test_ctx);
        test_cursor_write(test_ctx, &c, buf, size);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        test_cursor_truncate(test_ctx, &c);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        int error = bobbyfs_read(test_ctx, &c.file, buf, 1);
        ck_assert_int_eq(error, BOBBYFS_EOF);
        uint32_t size = bobbyfs_file_size(test_ctx, &c.file);
        ck_assert_int_eq(size, 0);
        uint32_t free_pages2 = bobbyfs_get_free_page_count(test_ctx);
        ck_assert_int_eq(free_pages, free_pages2);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    free(v.data);
    free(buf);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_double_read_page)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    uint8_t buf[BOBBYFS_PAGE_SIZE];
    fill_random(buf, BOBBYFS_PAGE_SIZE);

    vector v;

    {
        test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
        test_cursor_write(test_ctx, &c, buf, BOBBYFS_PAGE_SIZE);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        test_cursor_read(test_ctx, &c, BOBBYFS_PAGE_SIZE);
        test_cursor_seek_from_start(test_ctx, &c, 0);
        test_cursor_read(test_ctx, &c, BOBBYFS_PAGE_SIZE);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    free(v.data);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_double_read_huge)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    const size_t size = 1024 * 1024;
    uint8_t* buf = malloc(size);
    fill_random(buf, size);

    vector v;

    {
        test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
        test_cursor_write(test_ctx, &c, buf, size);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        test_cursor_read(test_ctx, &c, size);
        test_cursor_seek_from_start(test_ctx, &c, 0);
        test_cursor_read(test_ctx, &c, size);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    free(v.data);
    free(buf);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_append_huge)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    vector v;
    {
        test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
        bobbyfs_file_close(test_ctx, &c.file);
    }

    for (int i = 0; i < 100; i++) {
        uint32_t n_bytes = (rand() % 4096) + 1;
        uint8_t* buf = malloc(n_bytes);
        fill_random(buf, n_bytes);
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        test_cursor_seek_from_end(test_ctx, &c, 0);
        test_cursor_write(test_ctx, &c, buf, n_bytes);
        test_cursor_seek_from_start(test_ctx, &c, 0);
        test_cursor_read(test_ctx, &c, bobbyfs_file_size(test_ctx, &c.file));
        bobbyfs_file_close(test_ctx, &c.file);
        free(buf);
    }
    free(v.data);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

START_TEST(test_random_read)
{
    reset_ctx();
    bobbyfs_dir root = bobbyfs_dir_root(test_ctx);

    const size_t size = 256 * 1024;
    uint8_t* buf = malloc(size);
    fill_random(buf, size);

    vector v;

    {
        test_cursor c = test_cursor_create(test_ctx, root, &v, "a file");
        test_cursor_write(test_ctx, &c, buf, size);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    for (int i = 0; i < 100; i++) {
        test_cursor c = test_cursor_open(test_ctx, root, &v, "a file");
        uint32_t pos = rand() % size;
        test_cursor_seek_from_start(test_ctx, &c, pos);
        uint32_t n_bytes = (rand() % 4096) + 1;
        if (pos + n_bytes > size) {
            n_bytes = size - pos;
        }
        test_cursor_read(test_ctx, &c, n_bytes);
        bobbyfs_file_close(test_ctx, &c.file);
    }
    free(v.data);
    free(buf);
    bobbyfs_dir_close(test_ctx, root);
}
END_TEST

Suite* bobbyfs_test_suite()
{
    Suite* s = suite_create("bobbyfs");
    TCase* tc_internal = tcase_create("Internal");

    tcase_add_test(tc_internal, test_create_single_folder);
    tcase_add_test(tc_internal, test_create_multiple_folders);
    tcase_add_test(tc_internal, test_double_open);
    tcase_add_test(tc_internal, test_write_a_byte);
    tcase_add_test(tc_internal, test_write_a_byte_files);
    tcase_add_test(tc_internal, test_write_a_random_page);
    tcase_add_test(tc_internal, test_write_a_megabyte);
    tcase_add_test(tc_internal, test_write_random_files);
    tcase_add_test(tc_internal, test_write_random_files_folders);
    tcase_add_test(tc_internal, test_rewrite_file_parts);
    tcase_add_test(tc_internal, test_truncate_empty);
    tcase_add_test(tc_internal, test_truncate_page);
    tcase_add_test(tc_internal, test_truncate_huge_file_full);
    tcase_add_test(tc_internal, test_double_read_page);
    tcase_add_test(tc_internal, test_double_read_huge);
    tcase_add_test(tc_internal, test_append_huge);

    suite_add_tcase(s, tc_internal);

    return s;
}

int main()
{
    Suite* s = bobbyfs_test_suite();
    SRunner* sr = srunner_create(s);

    srunner_run_all(sr, CK_NORMAL);
    int number_failed = srunner_ntests_failed(sr);
    srunner_free(sr);

    free_ctx();

    return (number_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
