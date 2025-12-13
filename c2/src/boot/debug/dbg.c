#include "dbg.h"
#include "../fmt.h"
#include "dbg_print.h"
#include <stdarg.h>
#include <stddef.h>
#include <string.h>

static int write_debug(const char* s, size_t len)
{
    while (len) {
        const char* part = memchr(s, '\n', len);
        if (!part) {
            dbg_print(s, len);
            return 0;
        }

        size_t len2 = part - s;

        if (len2) {
            dbg_print(s, len2);
        }

        len -= len2 + 1;
        dbg_newline();
        s = part + 1;
    }

    return 0;
}

void vprintd(const char* format, va_list ap)
{
    vfmt(write_debug, format, ap);
}

void printd(const char* format, ...)
{
    va_list ap;
    va_start(ap, format);

    vprintd(format, ap);

    va_end(ap);
}
