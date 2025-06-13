#ifndef LIB_KEYS_KEYS_H
#define LIB_KEYS_KEYS_H

#include <cstdint>

// handles CAPSLOCK and shift
bool handle_control(uint8_t key);
char get_char(uint8_t key);

#endif
