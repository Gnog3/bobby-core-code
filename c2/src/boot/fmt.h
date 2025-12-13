#pragma once
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

size_t u32_to_s(uint32_t value, char s[11]);
void u32_to_s_hex(uint32_t value, char s[8]);

int vfmt(int (*write)(const char* s, size_t len), const char* format, va_list ap);
int fmt(int (*write)(const char* s, size_t len), const char* format, ...)
    __attribute__((format(printf, 2, 3)));
