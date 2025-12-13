#include "../../boot/debug/dbg_print.h"
#include <string.h>

const char* s = "Hello, world!";

int main()
{
    dbg_print(s, strlen(s));
}
