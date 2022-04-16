#ifndef ARITH_H
#define ARITH_H

#include <stdint.h>
#include "csr.h"
#include "typedef.h"

inline void send_mul(uint32_t valA, uint32_t valB)
{
	csr_write(1040, valA);
	csr_write(1041, valB);
	__asm__("nop");	
}

inline uint32_t mul_lo(void)
{
    return csr_read(1042);
}

inline uint32_t mul_hi(void)
{
    return csr_read(1043);
}

inline void send_div(uint32_t valA, uint32_t valB)
{
	csr_write(1044, valA);
	csr_write(1045, valB);
	__asm__("nop");
	__asm__("nop");
	__asm__("nop");
	__asm__("nop");
	__asm__("nop");
	__asm__("nop");
	__asm__("nop");
	__asm__("nop");
	__asm__("nop");
	__asm__("nop");
	__asm__("nop");
}

inline uint32_t div_q(void)
{
    return csr_read(1046);
}

inline uint32_t div_r(void)
{
    return csr_read(1047);
}

#endif
