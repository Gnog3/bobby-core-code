#include "../output.h"
#include "../input.h"
#include "../typedef.h"
#include "../keydefs.h"
#include "../calcv1/calcv1.h"

_Alignas(4) u8 FIRST[40] = {0};
_Alignas(4) u8 SECOND[40] = {0};
_Alignas(4) u8 RESULT[40] = {0};

void require_number_(u8 arr[40]) {
    char str[2];
    str[1] = 0;

}

void calc_v1_1() {
#pragma clang diagnostic push
#pragma ide diagnostic ignored "EndlessLoop"
    while (1) {
        clear_screen();
        term_color(0xFFFFFF);
        term_print(0, 0, "Calculator v1.1");
        term_print(0, 8, "1. Addition");
        term_print(0, 16, "2. Subtraction");
        term_print(0, 24, "3. Multiplication");

        u8 operation;
        while (1) {
            u8 key = require_key();
            if (key >= ASCII_1 && key <= ASCII_3) {
                operation = key;
                break;
            }
        }


    }
#pragma clang diagnostic pop
}