#ifndef LIB_INPUT_INPUT_H
#define LIB_INPUT_INPUT_H

#include "csr.h"

inline unsigned char read_key() {
	return csr_read(CSR_KEYBOARD);
}

#endif
