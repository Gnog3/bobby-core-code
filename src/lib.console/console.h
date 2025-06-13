#ifndef LIB_CONSOLE_CONSOLE_H
#define LIB_CONSOLE_CONSOLE_H

#include <lib.output/output.h>

#define MAX_CHARS_ON_LINE (SCREEN_WIDTH / CHAR_WIDTH)
#define MAX_LINES (SCREEN_HEIGHT / CHAR_HEIGHT)
#define MAX_X (MAX_CHARS_ON_LINE * CHAR_WIDTH)
#define LAST_X (MAX_X - CHAR_WIDTH)
#define MAX_Y (MAX_LINES * CHAR_HEIGHT)
#define TEXT_COLOR 0xFFFFFF
#define BACKGROUND_COLOR color_t(0)
#define CURSOR_COLOR color_t(0xFF0000)

class Console {
  public:
    pos_t next_char_pos = pos_t(0, 0);
    int chars_on_current_line = 0;

    // input
    char buffer[256];
    int first_line_cap;
    int buffer_len = 0;
    int cursor_pos = 0;

    void draw_cursor(bool visible);
    void scroll();
    void handle_overflow();
  public:
    Console();
    void print(std::string_view s);
    void new_line();
    std::string_view input();
};

#endif
