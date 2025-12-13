#pragma once

#include <stdarg.h>

void vprintd(const char* format, va_list ap);
void printd(const char* format, ...) __attribute__((format(printf, 1, 2)));
