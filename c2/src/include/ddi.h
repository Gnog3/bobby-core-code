#ifndef DDI_H
#define DDI_H

// direct display interface

#include <stdint.h>
#include <csr.h>

inline void ddi_set_matrix_1(uint32_t m) {
    csr_write(CSR_DDI_MATRIX_1, m);
}

inline void ddi_set_matrix_2(uint32_t m) {
    csr_write(CSR_DDI_MATRIX_2, m);
}

inline void ddi_set_target(uint16_t x, uint16_t y) {
    uint32_t combined = ((uint32_t)y << 16) | (uint32_t)x;
    csr_write(CSR_DDI_TARGET, combined);
}

inline void ddi_set_source(uint16_t x, uint16_t y) {
    uint32_t combined = ((uint32_t)y << 16) | (uint32_t)x;
    csr_write(CSR_DDI_SOURCE, combined);
}

inline void ddi_set_size(uint16_t w, uint16_t h) {
    uint32_t combined = ((uint32_t)h << 16) | (uint32_t)w;
    csr_write(CSR_DDI_SIZE, combined);
}

inline void ddi_set_color(uint32_t color) {
    csr_write(CSR_DDI_COLOR, color);
}

inline void ddi_run_matrix() {
    csr_write(CSR_DDI_MATRIX, 0);
}

inline void ddi_floodfill() {
    csr_write(CSR_DDI_FLOODFILL, 0);
}

inline void ddi_copy() {
    csr_write(CSR_DDI_COPY, 0);
}

inline void ddi_rectangle() {
    csr_write(CSR_DDI_RECTANGLE, 0);
}

#endif
