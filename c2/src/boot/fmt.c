#include "fmt.h"
#include <stdarg.h>
#include <string.h>

size_t u32_to_s(uint32_t value, char s[11])
{
    size_t len = 0;
    char buf[10];

    while (value > 0) {
        buf[len] = '0' + value % 10;
        value /= 10;
        len++;
    }

    for (size_t i = 0, rev = len - 1; i < len; i++, rev--) {
        s[i] = buf[rev];
    }

    s[len] = 0;
    return len;
}

void u32_to_s_hex(uint32_t value, char s[8])
{
    for (int i = 7; i >= 0; i--) {
        uint32_t nibble = value & 0xF;
        char c;
        if (nibble < 10) {
            c = '0' + nibble;
        } else {
            c = 'A' + nibble - 10;
        }

        s[i] = c;
        value >>= 4;
    }
}

static int fmt_unsigned(int (*write)(const char* s, size_t len), va_list* ap)
{
    uint32_t value = va_arg(*ap, uint32_t);
    char s[11];
    size_t len = u32_to_s(value, s);
    return write(s, len);
}

static int fmt_str(int (*write)(const char* s, size_t len), va_list* ap)
{
    const char* s = va_arg(*ap, const char*);
    size_t len = strlen(s);
    return write(s, len);
}

int vfmt(int (*write)(const char* s, size_t len), const char* format, va_list ap)
{
    int error;

    while (*format) {
        const char* percent = strchrnul(format, '%');
        size_t len = percent - format;
        if (len) {
            error = write(format, len);
            if (error) {
                return error;
            }
        }

        if (!*percent) {
            return 0;
        }

        percent++;

        if (*percent == 'u') {
            error = fmt_unsigned(write, &ap);
            if (error) {
                return error;
            }
        } else if (*percent == 's') {
            error = fmt_str(write, &ap);
            if (error) {
                return error;
            }
        } else if (*percent == '%') {
            error = write(percent, 1);
            if (error) {
                return error;
            }
        }

        percent++;
        format = percent;
    }

    return 0;
}

int fmt(int (*write)(const char* s, size_t len), const char* format, ...)
{
    va_list ap;
    va_start(ap, format);

    int error = vfmt(write, format, ap);

    va_end(ap);

    return error;
}
