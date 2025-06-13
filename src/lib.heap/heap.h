#ifndef LIB_HEAP_HEAP_H
#define LIB_HEAP_HEAP_H

#include <stddef.h>

void heap_init();
extern "C" void* malloc(size_t size);
extern "C" void* realloc(void* ptr, size_t newsize);
extern "C" void free(void* ptr);

#endif
