#include "alloc.h"
#include "tlsf/tlsf.h"

#include <stdlib.h>

#define START_ADDRESS (2 * 1024 * 1024)
#define HEAP_SIZE (1024 * 1024)

static tlsf_t tlsf;

void alloc_init()
{
    tlsf = tlsf_create_with_pool((void*)START_ADDRESS, HEAP_SIZE);
}

void* malloc(size_t bytes)
{
    return tlsf_malloc(tlsf, bytes);
}

void free(void* ptr)
{
    tlsf_free(tlsf, ptr);
}

void* realloc(void* ptr, size_t bytes)
{
    return tlsf_realloc(tlsf, ptr, bytes);
}
