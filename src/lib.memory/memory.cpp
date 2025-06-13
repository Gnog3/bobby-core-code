#include "memory.h"

#include <lib.debug/debug.h>
#include <lib.io/io.h>

extern "C" void* memcpy(void* dest, const void* src, size_t count) {
    // io::cerr << "memcpy " << count << " bytes" << io::endl;
    char* dest2 = (char*) dest;
    const char* src2 = (char*) src;
    for (size_t i = 0; i < count; i++) {
        dest2[i] = src2[i];
    }
    return dest;
}

extern "C" size_t __attribute__((optimize("O1"))) strlen(const char* str) {
    // io::cerr << "strlen at " << io::hex((uint32_t) str) << io::endl;
    size_t len = 0;
    while (*str++) {
        len++;
    }
    return len;
}