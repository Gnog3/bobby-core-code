#include "heap.h"
#include "o1heap.h"
#include <lib.io/io.h>
#include <lib.halt/halt.h>
#include <lib.memory/memory.h>

#include <new>

O1HeapInstance* heap;

void heap_init() {
    io::cerr << "Initializing heap... ";
    heap = o1heapInit((void*) 800000, 32768);
    if (heap) {
        io::cerr << "OK" << io::endl;
    } else {
        io::cerr << "Heap is NULL" << io::endl;
        halt(-1);
    }
}

extern "C" void* malloc(size_t size) {
    io::cerr << "Allocating " << size << " bytes... ";
    void* result = o1heapAllocate(heap, size);
    if (result) {
        io::cerr << "Allocated at " << io::hex((uint32_t) result) << io::endl;
        O1HeapDiagnostics diag = o1heapGetDiagnostics(heap);
    } else {
        io::cerr << "Allocate returned NULL" << io::endl;
        halt(-1);
    }
    return result;
}

extern "C" void* realloc(void* ptr, size_t newsize) {
    io::cerr << "Realloc is not implemented" << io::endl;
    halt(-1);
    __builtin_unreachable();
}

extern "C" void free(void* ptr) {
    io::cerr << "Freeing " << io::hex((uint32_t) ptr) << "... ";
    o1heapFree(heap, ptr);
    io::cerr << "Freed." << io::endl;
}

// not in header
void* operator new(size_t count) {
    return malloc(count);
}

void* operator new[](size_t count) {
    return malloc(count);
}

void* operator new(size_t count, const std::nothrow_t& tag) {
    return malloc(count);
}

void* operator new[](std::size_t count, const std::nothrow_t& tag) {
    return malloc(count);
}

void operator delete(void* ptr, size_t bytes) {
    free(ptr);
}

namespace std {
    // void __throw_length_error(char const* s) {
    //     debug_print_nl("std::__throw_length_error");
    //     halt(-1);
    // }
}