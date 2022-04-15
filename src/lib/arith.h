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
