#include "halt.h"
#include <lib.csr/csr.h>
#include <lib.debug/debug.h>
#include <lib.io/io.h>

void halt(int value) {
    io::cerr << "Halt (" << io::hex((uint32_t)value) << ")" << io::endl;
    csr_write(CSR_HALT, 0);
}
