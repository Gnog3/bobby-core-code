#define RAM_SIZE (16 * 1024 * 1024) // 16 MB
#define FS_LEN (1024 * 1024)        // 1 MB
#define FS_PTR (void*)(RAM_SIZE - FS_LEN)

#define READ(offset, buf, size) memcpy(buf, FS_PTR + offset, size)
#define WRITE(offset, buf, size) memcpy(FS_PTR + offset, buf, size)
#define CAPACITY() FS_LEN

#define READ_VAL(offset, ptr)                                                                      \
    memcpy(ptr, __builtin_assume_aligned(FS_PTR + offset, 4), sizeof(*ptr))

#define WRITE_VAL(offset, val) memcpy(__builtin_assume_aligned(FS_PTR + offset, 4), &val, sizeof(val))

#include "bobbyfs/bobbyfs.c"
