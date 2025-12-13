#ifndef VECTOR_H
#define VECTOR_H

#include <stddef.h>

typedef struct {
    void* data;
    size_t size;
    size_t capacity;
} vector;

void vector_init(vector* v);
void vector_free(vector* v);

int vector_push_back(vector* v, const void* data, size_t size);
int vector_insert(vector* v, size_t index, const void* data, size_t size);
int vector_rewrite(vector* v, size_t index, const void* data, size_t size);

#endif
