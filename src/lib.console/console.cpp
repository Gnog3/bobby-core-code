#include "console.h"

#include <lib.output/output.h>
#include <lib.input/input.h>
#include <lib.keys/keys.h>
#include <algorithm>

void Console::draw_cursor(bool visible) {
    pos_t cursor_pos = pos_t(next_char_pos.x, next_char_pos.y + 7);
    ddi_color(visible ? CURSOR_COLOR : BACKGROUND_COLOR);
    ddi_target(cursor_pos);
    ddi_size(pos_t(5, 1));
    ddi_rectangle();
}

void Console::scroll() {
    ddi_target(pos_t(0, 0));
    ddi_source(pos_t(0, CHAR_HEIGHT));
    ddi_size(pos_t(SCREEN_WIDTH, SCREEN_HEIGHT - CHAR_HEIGHT));
    ddi_copy();
    ddi_target(pos_t(0, SCREEN_HEIGHT - CHAR_HEIGHT));
    ddi_size(pos_t(SCREEN_WIDTH, CHAR_HEIGHT));
    ddi_color(BACKGROUND_COLOR);
    ddi_rectangle();
}

void Console::handle_overflow() {
    if (next_char_pos.x >= MAX_X) {
        next_char_pos.x = 0;
        next_char_pos.y += CHAR_HEIGHT;
        chars_on_current_line = 0;
    }
    if (next_char_pos.y >= MAX_Y) {
        scroll();
        next_char_pos.y -= CHAR_HEIGHT;
    }
}

Console::Console() {}

void Console::print(std::string_view s) {
    const char* ptr = s.data();
    int remaining_chars = s.length();
    term_color(color_t(TEXT_COLOR));
    while (remaining_chars != 0) {
        int chars_to_print = std::min(remaining_chars, MAX_CHARS_ON_LINE - chars_on_current_line);
        term_print(next_char_pos, std::string_view(ptr, chars_to_print));
        ptr += chars_to_print;
        remaining_chars -= chars_to_print;
        chars_on_current_line += chars_to_print;
        next_char_pos.x += chars_to_print * CHAR_WIDTH;
        handle_overflow();
    }
}

void Console::new_line() {
    next_char_pos.x = 0;
    next_char_pos.y += CHAR_HEIGHT;
    chars_on_current_line = 0;
    handle_overflow();
}

std::string_view Console::input() {
    buffer_len = 0;
    draw_cursor(true);
    while (true) {
        uint8_t key = read_key();
        if (key == 0) {
            continue;
        }
        if ((key >> 7) == 0) {
            continue;
        }
        key &= 0x7F;
        if (key == KEY_ENTER) {
            break;
        }
        if (key == KEY_BACKSPACE && buffer_len != 0) {
            draw_cursor(false);
            if (next_char_pos.x == 0) {
                next_char_pos.x = LAST_X;
                next_char_pos.y -= CHAR_HEIGHT;
                chars_on_current_line = MAX_CHARS_ON_LINE - 1;
            } else {
                next_char_pos.x -= CHAR_WIDTH;
                chars_on_current_line -= 1;
            }
            buffer_len -= 1;
            ddi_target(next_char_pos);
            ddi_size(pos_t(CHAR_WIDTH, CHAR_HEIGHT));
            ddi_color(BACKGROUND_COLOR);
            ddi_rectangle();
            draw_cursor(true);
            continue;
        }
        char ch = get_char(key);
        if (ch == 0) {
            continue;
        }
        draw_cursor(false);
        print(std::string_view(&ch, 1));
        draw_cursor(true);
        buffer[buffer_len] = ch;
        buffer_len++;
    }
    draw_cursor(false);
    new_line();
    return std::string_view(buffer, buffer_len);
}