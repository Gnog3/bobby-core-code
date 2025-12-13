#include "panic.h"
#include "debug/dbg_print.h"
#include "fmt.h"
#include <csr.h>
#include <debug.h>
#include <string.h>

void panic_hex(uint32_t value)
{
    dbg_newline();

    const char* msg = "Panic! Code: 0x";
    dbg_print(msg, strlen(msg));

    char hex[8];
    u32_to_s_hex(value, hex);
    dbg_print(hex, 8);

    dbg_newline();

    halt();
}

void panic_str(const char* str)
{
    dbg_newline();
    const char* msg = "Panic! ";
    dbg_print(msg, strlen(msg));
    dbg_print(str, strlen(str));
    dbg_newline();

    halt();
}

void panicf(const char* format, ...)
{
    va_list ap;
    va_start(ap, format);

    dbg_newline();
    const char* msg = "Panic! ";
    dbg_print(msg, strlen(msg));
    vprintd(format, ap);

    va_end(ap);
}
