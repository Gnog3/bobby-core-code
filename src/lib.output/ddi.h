#ifndef LIB_OUTPUT_DDI_H
#define LIB_OUTPUT_DDI_H

#include "structs.h"

inline void ddi_matrix_1(uint32_t matrix) {
    csr_write(CSR_DDI_MATRIX_1, matrix);
}

inline void ddi_matrix_2(uint32_t matrix) {
    csr_write(CSR_DDI_MATRIX_2, matrix);
}

inline void ddi_target(pos_t pos) {
    csr_write(CSR_DDI_TARGET, pos.pos);
}

inline void ddi_source(pos_t pos) {
    csr_write(CSR_DDI_SOURCE, pos.pos);
}

inline void ddi_size(pos_t pos) {
    csr_write(CSR_DDI_SIZE, pos.pos);
}

inline void ddi_color(color_t color) {
    csr_write(CSR_DDI_COLOR, color.color);
}

inline void ddi_matrix() {
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
