#include <stdint.h>
#include <lib.csr/csr.h>

inline void send_mul(uint32_t valA, uint32_t valB) {
	csr_write(CSR_MUL_VAL_A, valA);
	csr_write(CSR_MUL_VAL_B, valB);
	__asm__("nop");	
}

inline uint32_t mul_lo(void) {
    return csr_read(CSR_MUL_LO);
}

inline uint32_t mul_hi(void) {
    return csr_read(CSR_MUL_HI);
}

uint32_t __mulsi3(uint32_t a, uint32_t b) {
    send_mul(a, b);
    return mul_lo();
}
