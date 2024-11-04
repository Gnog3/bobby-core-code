#include "hex.h"

const char digit_hex[] = { '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F' };

void int_to_hex_str(uint32_t value, char string[8])
{
    for (int i = 7; i >= 0; i--) {
        int digit = value & 0xF;
        string[i] = digit_hex[digit];
        value >>= 4;
    }
}