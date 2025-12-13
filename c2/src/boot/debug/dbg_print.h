#pragma once

#include <csr.h>
#include <csr_defs.h>
#include <stddef.h>

inline void dbg_clear()
{
    csr_write(CSR_DEBUG_CLEAR, 0);
}

inline void dbg_print(const char* str, size_t len)
{
    csr_write(CSR_DEBUG_LENGTH, len);
    csr_write(CSR_DEBUG_PRINT, str);
    nop();
}

inline void dbg_newline()
{
    csr_write(CSR_DEBUG_NEWLINE, 0);
}
