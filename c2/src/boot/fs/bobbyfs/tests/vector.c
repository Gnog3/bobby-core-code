#include "vector.h"
#include <stdlib.h>
#include <string.h>

void vector_init(vector* v)
{
    v->data = NULL;
    v->size = 0;
    v->capacity = 0;
}

void vector_free(vector* v)
{
    if (v->capacity) {
        free(v->data);
    }
}

static int vector_ensure_capacity(vector* v, size_t capacity)
{
    if (capacity <= v->capacity) {
        return 0;
    }

    size_t new_capacity = v->capacity == 0 ? 8 : v->capacity * 2;
    while (capacity > new_capacity) {
        new_capacity *= 2;
    }

    void* new_data = realloc(v->data, new_capacity);
    if (!new_data) {
        return 1;
    }

    v->data = new_data;
    v->capacity = new_capacity;

    return 0;
}

int vector_push_back(vector* v, const void* data, size_t size)
{
    int error = vector_ensure_capacity(v, v->size + size);
    if (error) {
        return error;
    }

    memcpy(v->data + v->size, data, size);
    v->size += size;
    return 0;
}

int vector_insert(vector* v, size_t index, const void* data, size_t size)
{
    int error = vector_ensure_capacity(v, v->size + size);
    if (error) {
        return error;
    }

    memmove(v->data + index + size, v->data + index, v->size - index);
    memcpy(v->data + index, data, size);
    v->size += size;
    return 0;
}

int vector_rewrite(vector* v, size_t index, const void* data, size_t size)
{
    int error = vector_ensure_capacity(v, index + size);
    if (error) {
        return error;
    }

    memcpy(v->data + index, data, size);
    if (v->size < index + size) {
        v->size = index + size;
    }

    return 0;
}
