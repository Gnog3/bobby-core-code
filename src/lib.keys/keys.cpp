#include "keys.h"
#include "defs.h"

#include <lib.io/io.h>

bool handle_control(uint8_t key) {
    return false;
}

char get_char(uint8_t key) {
    if (key >= KEY_0 && key <= KEY_9) {
        return key;
    } else {
        io::cerr << "Unknown key " << io::hex(key) << io::endl;
        return 0;
    }
}