#ifndef LIB_DEBUG_DEBUG_H
#define LIB_DEBUG_DEBUG_H

#include <lib.csr/csr.h>
#include <lib.hex/hex.h>
#include <string_view>

inline void debug_set_length(size_t length) {
    csr_write(CSR_DEBUG_LENGTH, length);
}

inline void debug_print_raw(const char* s) {
    csr_write(CSR_DEBUG_PRINT, s);
    __asm__("nop");
}

inline void debug_print(std::string_view str) {
    debug_set_length(str.length());
    debug_print_raw(str.data());
}

inline void debug_newline() {
    csr_write(CSR_DEBUG_NEWLINE, 0);
}

inline void debug_hex(uint32_t number) {
    char hex[8];
    int_to_hex_str(number, hex);
    std::string_view s(hex, 8);
    debug_print(s);
}

inline void debug_clear() {
    csr_write(CSR_DEBUG_CLEAR, 0);
}

inline uint32_t debug_counter() {
    return csr_read(CSR_INSTRUCTION_COUNTER);
}

#endif
