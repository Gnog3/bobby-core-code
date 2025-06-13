#include "io.h"

#include <string_view>
#include <cstring>
#include <lib.halt/halt.h>
#include <lib.debug/debug.h>
#include <ios>

namespace io {
    const io::err operator<<(io::err err, char c) {
        if (c == '\n') {
            debug_newline();
        } else {
            io::cerr << "Char != '\\n' not implemented";
        }
        return err;
    }

    const io::err operator<<(io::err err, std::string_view s) {
        debug_print(s);
        return err;
    }
}
