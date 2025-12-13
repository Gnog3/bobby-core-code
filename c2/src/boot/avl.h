#pragma once

#include <stdint.h>

typedef int (*avl_cmp)(char left[], char right[]);

typedef struct avl_node {
    uint32_t height;
    struct avl_node* left;
    struct avl_node* right;
    char data[]; // key and value
} avl_node;

avl_node* insert(avl_node* p, avl_node* n, avl_cmp cmp);
avl_node* remove(avl_node* p, char k[], avl_node** removed, avl_cmp cmp);
avl_node* find(avl_node* p, char k[], avl_cmp cmp);
