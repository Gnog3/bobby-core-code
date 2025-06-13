#ifndef LIB_MEMORY_MEMORY_H
#define LIB_MEMORY_MEMORY_H

#include "stddef.h"

extern "C" void* memcpy(void* dest, const void* src, size_t count);
extern "C" size_t strlen(const char* str);

#endif
