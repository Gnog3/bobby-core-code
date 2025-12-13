#include <stddef.h>
#include <stdint.h>
#include <string.h>

void* memcpy(void* dst, const void* src, size_t len)
{
    if ((size_t)dst % sizeof(size_t) == 0 && (size_t)src % sizeof(size_t) == 0 &&
        len % sizeof(long) == 0) {

        size_t* d = dst;
        const size_t* s = src;

        for (size_t i = 0; i < len / sizeof(size_t); i++) {
            d[i] = s[i];
        }
    } else {
        char* d = dst;
        const char* s = src;

        for (size_t i = 0; i < len; i++) {
            d[i] = s[i];
        }
    }

    return dst;
}

void* memset(void* dst, int c, size_t n)
{
    if (c == 0 && ((size_t)dst % 4 == 0)) {
        while (n >= 4) {
            *(int*)dst = 0;
            dst += 4;
            n -= 4;
        }
    }

    uint8_t* s = dst;
    for (size_t i = 0; i < n; ++i)
        s[i] = (uint8_t)c;
    return dst;
}

int memcmp(const void* s1, const void* s2, size_t n)
{
    const uint8_t* ss1 = s1;
    const uint8_t* ss2 = s2;
    for (size_t i = 0; i < n; i++) {
        int8_t result = ss1[i] - ss2[i];
        if (result) {
            return result;
        }
    }

    return 0;
}

void* memchr(const void* src, int c, size_t n)
{
    char cc = c;
    const char* src2 = src;
    for (size_t i = 0; i < n; i++) {
        if (src2[i] == cc) {
            return (void*)(src + i);
        }
    }

    return NULL;
}

char* strchrnul(const char* s, int c)
{
    char b = c;
    while (*s) {
        if (*s == b) {
            return (char*)s;
        }
        s++;
    }
    return (char*)s;
}

size_t strlen(const char* str)
{
    const char* s;

    for (s = str; *s; ++s) {
    }

    return (s - str);
}

int __clzsi2(unsigned x)
{
    // This uses a binary search (counting down) algorithm from Hacker's Delight.
    unsigned y;
    int n = 32;
    y = x >> 16;
    if (y != 0) {
        n = n - 16;
        x = y;
    }

    y = x >> 8;
    if (y != 0) {
        n = n - 8;
        x = y;
    }

    y = x >> 4;
    if (y != 0) {
        n = n - 4;
        x = y;
    }

    y = x >> 2;
    if (y != 0) {
        n = n - 2;
        x = y;
    }

    y = x >> 1;
    if (y != 0) {
        return n - 2;
    }

    return n - x;
}

int __ctzsi2(unsigned x)
{
    // This uses a binary search algorithm from Hacker's Delight.
    int n = 1;
    if ((x & 0x0000FFFF) == 0) {
        n = n + 16;
        x = x >> 16;
    }

    if ((x & 0x000000FF) == 0) {
        n = n + 8;
        x = x >> 8;
    }

    if ((x & 0x0000000F) == 0) {
        n = n + 4;
        x = x >> 4;
    }

    if ((x & 0x00000003) == 0) {
        n = n + 2;
        x = x >> 2;
    }

    return n - (x & 1);
}

int __ffssi2(int x)
{
    return (x == 0) ? 0 : __builtin_ctz(x) + 1;
}
