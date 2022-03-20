#define csr_write(csr, val)                                    \
({                                                             \
       unsigned long __v = (unsigned long)(val);               \
       __asm__ __volatile__ ("csrw " #csr ", %0"               \
                             : : "r" (__v));                   \
})

#define csr_read(csr)                                          \
({                                                             \
       register unsigned long __v;                             \
       __asm__ __volatile__ ("csrr %0, " #csr                  \
                             : "=r" (__v));                    \
       __v;                                                    \
})
