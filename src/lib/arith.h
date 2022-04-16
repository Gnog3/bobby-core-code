#ifndef ARITH_H
#define ARITH_H

#include "csr.h"
#include "typedef.h"

inline void send_mul(int valA, int valB)
{
	csr_write(1040, valA);
	csr_write(1041, valB);
	__asm__("nop");	
}

inline int mul_lo(void)
{
    return csr_read(1042);
}

inline int mul_hi(void)
{
    return csr_read(1043);
}

inline void send_div(int valA, int valB)
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

inline int div_q(void)
{
    return csr_read(1046);
}

inline int div_r(void)
{
    return csr_read(1047);
}

#endif
