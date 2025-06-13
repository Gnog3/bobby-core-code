#ifndef LIB_OUTPUT_OUTPUT_H
#define LIB_OUTPUT_OUTPUT_H

#include "structs.h"

#include <stdint.h>
#include <string_view>
#include <lib.csr/csr.h>

#define SCREEN_WIDTH 256
#define SCREEN_HEIGHT 256
#define CHAR_WIDTH 6
#define CHAR_HEIGHT 8

inline void term_color(color_t color) {
    csr_write(CSR_TERM_COLOR, color.color);
}

inline void term_position(pos_t pos) {
    csr_write(CSR_TERM_POSITION, pos.pos);
}

inline void term_print_raw(const char* str) {
    csr_write(CSR_TERM_PRINT, str);
    __asm__("nop");
}

inline void term_set_length(size_t length) {
    csr_write(CSR_TERM_LENGTH, length);
}

inline void term_print(pos_t pos, std::string_view str) {
    term_position(pos);
    term_set_length(str.length());
    term_print_raw(str.data());
}

#endif
