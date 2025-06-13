#ifndef PROGRAM_SIZE_H
#define PROGRAM_SIZE_H

#include <stdint.h>

extern int STATIC_PROGRAM_SIZE;

inline uint32_t program_size() {
    return (uint32_t) &STATIC_PROGRAM_SIZE;
}

#endif