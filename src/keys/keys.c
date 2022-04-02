#include "../input.h"
#include "../output.h"
#include "../keydefs.h"
#include <stdbool.h>
char DATAF[256];
#define CURSOR_COLOR 0xFFFF00

u8 require_key_() {
    while (1) {
        u8 key = read_key();
        bool pressed = key >> 7;
        key &= 0x7F;
        bool enter = key == KEY_ENTER;
        bool backspace = key == KEY_BACKSPACE;
        bool arrows = key >= KEY_UP && key <= KEY_RIGHT;
        bool digit = key >= ASCII_0 && key <= ASCII_9;
        if (pressed && (enter || digit || backspace || arrows)) {
            return key;
        }
    }
}

void draw_cursor(u8 x, u8 y, u32 color) {
    pixel_color(color);
    for (int i = 0; i < 5; i++) {
        pixel_plot(x + i, y);
    }
}

char* input(u8 x, u8 y) {
    term_position_raw(0);
    term_color(0xFFFFFF);
    int length = 0;
    int cursor = 0;
    char* data_ptr = DATAF;
    draw_cursor(x, y + 7, CURSOR_COLOR);
    while(1) {
        u8 key = require_key_();
        if (key == KEY_ENTER) {
            break;
        } else if (key >= ASCII_0 && key <= ASCII_9) {
            if (length == cursor) {
                DATAF[length] = key;
                DATAF[length + 1] = 0;
                term_print_raw(data_ptr + length);
                draw_cursor(x, y + 7, 0);
                x += 6;
                draw_cursor(x, y + 7, CURSOR_COLOR);
            }
        }
    }

    return DATAF;
}