#include <stdbool.h>

#include "calcv1.h"
#include "../output.h"
#include "../input.h"
#include "../keydefs.h"

#define ADDITION ASCII_1
#define SUBTRACTION ASCII_2
#define MULTIPLICATION ASCII_3
#define DIVISION ASCII_4

u8 require_key() {
    while (1) {
        u8 key = read_key();
        bool pressed = key >> 7;
        bool enter = key == 0b10010000;
        bool digit = (key >= 0b10110000) && (key <= 0b10111001);
        if (pressed && (enter || digit)) {
            return key & 0b01111111;
        }
    }
}

int require_number() {
    int number = 0;
    char str[2];
    str[1] = 0;
    while (1) {
        u8 key = require_key();
        if (key == KEY_ENTER) {
            return number;
        } else {
            str[0] = key;
            term_print_raw(str);
            number = (number << 3) + (number << 1); // multiply by 10
            number += key & 0b1111;
        }
    }
}

u32 bin_to_bcd(u32 number) {
    u32 x = number;
    u32 bcd = 0;
    for (int i = 30 - 1; i >= 0; i--) {
        bcd <<= 1;
        if (x & (1 << i)) {
            bcd |= 1;
        }
        int k = 0;
        if (i != 0) {
            for (int j = 0; j < 8; j++) {

                if ((bcd & (0xf << k)) > (0x4 << k))
                    bcd += 0x3 << k;
                k += 4;
            }
        }
    }
    return bcd;
}

void print_number(u32 number) {
    u32 bcd = bin_to_bcd(number);
    char str[11];
    for (int d = 0; d < 10; d++) {
        u8 digit = (bcd >> (d << 2)) & 0b1111;
        u8 ascii = digit | 0x30;
        str[9 - d] = ascii;
    }
    str[10] = 0;
    char* str_ptr = str;
    if (number == 0) {
        str_ptr += 9;
    } else {
        while (*str_ptr == ASCII_0) {
            ++str_ptr;
        }
    }
    term_print_raw(str_ptr);
}

void calc_v1() {
#pragma clang diagnostic push
#pragma ide diagnostic ignored "EndlessLoop"
    while (1) {
        clear_screen();
        term_color(0xFFFFFF);
        term_print(0, 0, "Calculator v1");
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
        term_print(0, 40, "First number: ");
        int number1 = require_number();
        term_print(0, 48, "Second number: ");
        int number2 = require_number();

        int result;
        switch (operation) {
            case ADDITION:
                result = number1 + number2;
                break;
            case SUBTRACTION:
                result = number1 - number2;
                break;
            case MULTIPLICATION:
                result = number1 * number2;
                break;
        }
        term_print(0, 56, "Result: ");
        print_number(result);
        while (require_key() != KEY_ENTER) {}
    }
#pragma clang diagnostic pop
}
