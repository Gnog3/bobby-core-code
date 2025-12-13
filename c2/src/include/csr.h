#pragma once

#include "csr_defs.h"

#define STRINGIFY2(X) #X
#define STRINGIFY(X) STRINGIFY2(X)

#define csr_write(csr, val)                                                                        \
    ({                                                                                             \
        unsigned long __v = (unsigned long)(val);                                                  \
        __asm__ __volatile__("csrw " STRINGIFY(csr) ", %0" : : "r"(__v));                          \
    })

#define csr_read(csr)                                                                              \
    ({                                                                                             \
        register unsigned long __v;                                                                \
        __asm__ __volatile__("csrrw %0, " STRINGIFY(csr) ", zero" : "=r"(__v));                    \
        __v;                                                                                       \
    })

#define nop() __asm__ __volatile__("nop")

inline void halt()
{
    csr_write(CSR_HALT, 0);
}
