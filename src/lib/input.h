#ifndef INPUT_H
#define INPUT_H

#include "csr.h"

inline unsigned char read_key() {
	return csr_read(1035);
}

#endif
