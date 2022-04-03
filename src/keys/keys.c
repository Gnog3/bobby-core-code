#include "../lib/input.h"
#include "../lib/output.h"
#include "../lib/keydefs.h"
#include <stdbool.h>

char KEYS_DATA[256];
#define CURSOR_COLOR 0xFFFF00

u8 keys_require_key() {
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

void keys_draw_cursor(u8 x, u8 y, u32 color) {
    pixel_color(color);
    for (int i = 0; i < 5; i++) {
        pixel_plot(x + i, y);
    }
}

char* input(u8 init_x, u8 init_y) {
    u8 x = init_x;
    u8 y = init_y;
    u8 overflow_x;
    term_position_raw(0);
    term_color(0xFFFFFF);
    int length = 0;
    int cursor = 0;
    char* data_ptr = KEYS_DATA;
    keys_draw_cursor(x, y + 7, CURSOR_COLOR);
    while (1) {
        u8 key = keys_require_key();
        if (key == KEY_ENTER) {
            break;
        } else if (key >= ASCII_0 && key <= ASCII_9) {
            if (length == cursor) {
                KEYS_DATA[length] = key;
                KEYS_DATA[length + 1] = 0;
                term_print(x, y, data_ptr + length);
                keys_draw_cursor(x, y + 7, 0);
                if (x >= 245) {
                    overflow_x = x;
                    x = 0;
                    y += 8;
                } else {
                    x += 6;
                }
                keys_draw_cursor(x, y + 7, CURSOR_COLOR);
                ++cursor;
                ++length;
            } else {
                keys_draw_cursor(x, y + 7, 0);
                term_color(0);
                term_print(x, y, data_ptr + cursor);
                term_color(0xFFFFFF);
                for (int i = length - 1; i >= cursor; i--) {
                    KEYS_DATA[i + 1] = KEYS_DATA[i];
                }
                KEYS_DATA[cursor] = key;
                KEYS_DATA[++length] = 0;
                term_print(x, y, data_ptr + cursor);
                ++cursor;
                if (x >= 245) {
                    overflow_x = x;
                    x = 0;
                    y += 8;
                } else {
                    x += 6;
                }
                keys_draw_cursor(x, y + 7, CURSOR_COLOR);
            }
        } else if (key == KEY_LEFT) {
            if (cursor > 0) {
                keys_draw_cursor(x, y + 7, 0);
                if (x <= 5) {
                    x = overflow_x;
                    y -= 8;
                } else {
                    x -= 6;
                }
                keys_draw_cursor(x, y + 7, CURSOR_COLOR);
                --cursor;
            }
        } else if (key == KEY_RIGHT) {
            if (cursor < length) {
                keys_draw_cursor(x, y + 7, 0);
                if (x >= 245) {
                    overflow_x = x;
                    x = 0;
                    y += 8;
                } else {
                    x += 6;
                }
                keys_draw_cursor(x, y + 7, CURSOR_COLOR);
                ++cursor;
            }
        } else if (key == KEY_BACKSPACE) {
            if (cursor == length) {
                keys_draw_cursor(x, y + 7, 0);
                term_color(0);
                if (x <= 5) {
                    x = overflow_x;
                    y -= 8;
                } else {
                    x -= 6;
                }
                term_print(x, y, data_ptr + length - 1);
                term_color(0xFFFFFF);
                KEYS_DATA[length--] = 0;
                --cursor;
                keys_draw_cursor(x, y + 7, 0xFFFFFF);
            }
        }
    }

    return KEYS_DATA;
}

int main() {
    clear_screen();
    input(0, 0);
    return 0;
}