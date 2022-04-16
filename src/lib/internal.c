#include "arith.h"
#include <stdint.h>

uint32_t __mulsi3(uint32_t a, uint32_t b)
{
    send_mul(a, b);
    return mul_lo();
}

uint32_t __udivsi3(uint32_t a, uint32_t b)
{
    send_div(a, b);
    return div_q();
}

uint32_t __umodsi3(uint32_t a, uint32_t b) {
    send_div(a, b);
    return div_r();
}
