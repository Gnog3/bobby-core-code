#ifndef LIB_CSR_CSR_H
#define LIB_CSR_CSR_H

#define STRINGIFY2(X) #X
#define STRINGIFY(X) STRINGIFY2(X)

#define csr_write(csr, val)                                       \
({                                                                \
       unsigned long __v = (unsigned long)(val);                  \
       __asm__ __volatile__ ("csrw " STRINGIFY(csr) ", %0"        \
                             : : "r" (__v));                      \
})

#define csr_read(csr)                                             \
({                                                                \
       register unsigned long __v;                                \
       __asm__ __volatile__ ("csrrw %0, " STRINGIFY(csr) ", zero" \
                             : "=r" (__v));                       \
       __v;                                                       \
})

#endif
