#pragma once

#include <stdint.h>

void panic_hex(uint32_t value) __attribute__((cold));
void panic_str(const char* str) __attribute__((cold));
void panicf(const char* format, ...) __attribute__((cold, format(printf, 1, 2)));
