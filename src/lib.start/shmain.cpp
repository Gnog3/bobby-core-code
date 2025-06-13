#include <lib.io/io.h>
#include <lib.halt/halt.h>
#include <lib.debug/debug.h>

extern int main();

extern "C" int shmain() {
    debug_clear();
    io::cerr << io::endl << "Start" << io::endl;
    int ret_code = main();
    halt(ret_code);
    return 0;
}