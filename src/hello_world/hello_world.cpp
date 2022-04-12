#include "../lib/output.h"
#include "../lib/csr.h"
#include <array>
class Abc {
private:
    int a;
public:
    Abc(int a) : a(a) {}
    int get_a() {
        return a;
    }
};
const u32 colors[] = {
        0,
        0xFF,
        0xFF00,
        0xFFFF,
        0xFF0000,
        0xFF00FF,
        0xFFFF00,
        0xFFFFFF
};

int main() {
    std::array<int, 18> arr = {0};

    for (auto i : arr) {
        csr_write(0, i);
    }

//    clear_screen(0x0000FF);
//    term_color(0xFFFFFF);
//    char* str = "Hello, worldSomerandomstringnottoberendered";
//    int length = 12;
//    char* end = str + length;
//    csr_write(1037, end);
//    term_print(0, 0, str);
//    return 0;
}