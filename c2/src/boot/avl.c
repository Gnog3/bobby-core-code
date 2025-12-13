#include "avl.h"
#include <stddef.h>

static uint32_t height(avl_node* p)
{
    return p ? p->height : 0;
}

static int bfactor(avl_node* p)
{
    return height(p->right) - height(p->left);
}

static void fix_height(avl_node* p)
{
    uint32_t hl = height(p->left);
    uint32_t hr = height(p->right);
    p->height = (hl > hr ? hl : hr) + 1;
}

static avl_node* rotate_right(avl_node* p)
{
    avl_node* q = p->left;
    p->left = q->right;
    q->right = p;
    fix_height(p);
    fix_height(q);
    return q;
}

static avl_node* rotate_left(avl_node* q)
{
    avl_node* p = q->right;
    q->right = p->left;
    p->left = q;
    fix_height(q);
    fix_height(p);
    return p;
}

static avl_node* balance(avl_node* p)
{
    fix_height(p);
    if (bfactor(p) == 2) {
        if (bfactor(p->left) > 0) {
            p->left = rotate_left(p->left);
        }
        return rotate_right(p);
    }

    if (bfactor(p) == -2) {
        if (bfactor(p->right) < 0) {
            p->right = rotate_right(p->right);
        }
        return rotate_left(p);
    }

    return p;
}

avl_node* insert(avl_node* p, avl_node* n, avl_cmp cmp)
{
    if (!p) {
        n->height = 1;
        n->left = NULL;
        n->right = NULL;
        return n;
    }

    int cmp_result = cmp(n->data, p->data);
    if (cmp_result < 0) {
        p->left = insert(p->left, n, cmp);
    } else {
        p->right = insert(p->right, n, cmp);
    }

    return balance(p);
}

static avl_node* find_min(avl_node* p)
{
    return p->left ? find_min(p->left) : p;
}

static avl_node* remove_min(avl_node* p)
{
    if (p->left == 0) {
        return p->right;
    }

    p->left = remove_min(p->left);
    return balance(p);
}

avl_node* remove(avl_node* p, char k[], avl_node** removed, avl_cmp cmp)
{
    if (!p) {
        *removed = NULL;
        return 0;
    }

    int cmp_result = cmp(k, p->data);

    if (cmp_result < 0) {
        p->left = remove(p->left, k, removed, cmp);
    } else if (cmp_result > 0) {
        p->right = remove(p->right, k, removed, cmp);
    } else {
        avl_node* q = p->left;
        avl_node* r = p->right;
        *removed = p;
        if (!r) {
            return q;
        }

        avl_node* min = find_min(r);
        min->right = remove_min(r);
        min->left = q;
        return balance(min);
    }

    return balance(p);
}

avl_node* find(avl_node* p, char k[], avl_cmp cmp)
{
    if (!p) {
        return NULL;
    }

    int cmp_result = cmp(k, p->data);
    if (cmp_result < 0) {
        return find(p->left, k, cmp);
    } else if (cmp_result > 0) {
        return find(p->right, k, cmp);
    }

    return p;
}
