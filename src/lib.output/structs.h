#ifndef LIB_OUTPUT_STRUCTS_H
#define LIB_OUTPUT_STRUCTS_H

#include <cstdint>

union pos_t {
    uint32_t pos;
    struct {
        uint16_t x;
        uint16_t y;
    };

    pos_t(uint16_t x, uint16_t y) {
        this->x = x;
        this->y = y;
    }
};

union color_t {
    uint32_t color;
    struct {
        uint8_t r;
        uint8_t g;
        uint8_t b;
    };

    color_t(uint32_t color) {
        this->color = color;
    }
};

#endif
