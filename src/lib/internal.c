#include "arith.h"

unsigned int
__mulsi3(unsigned int a, unsigned int b) {
    send_mul(a, b);
    return mul_lo();
}
