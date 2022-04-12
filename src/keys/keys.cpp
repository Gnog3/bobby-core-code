#include <stdbool.h>

#include "../lib/input.h"
#include "../lib/keydefs.h"
#include "../lib/output.h"

#define CURSOR_COLOR 0xFFFF00
#define OVERFLOW_X_DEFAULT 252

u8 keys_require_key()
{
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

void keys_draw_cursor(u8 x, u8 y, u32 color)
{
    pixel_color(color);
    for (int i = 0; i < 5; i++) {
        pixel_plot(x + i, y);
    }
}

class Input {
public:
    u8 y;
    u8 x;
    u8 overflow_x;
    u8 length = 0;
    u8 cursor = 0;
    u8 vertical_length;
    char* arr /*should be 256*/;
    Input(u8 x, u8 y, char* arr /*should be 256*/)
        : x(x)
        , y(y)
        , arr(arr)
    {
    }

    void cursor_right_unchecked()
    {
        keys_draw_cursor(x, y + 7, 0);
        if (x >= 245) {
            overflow_x = x;
            x = 0;
            y += 8;
        } else {
            x += 6;
        }
        keys_draw_cursor(x, y + 7, CURSOR_COLOR);
        cursor++;
    }

    void cursor_right()
    {
        if (length != cursor)
            cursor_right_unchecked();
    }

    void cursor_left_unchecked()
    {
        keys_draw_cursor(x, y + 7, 0);
        if (x <= 5) {
            x = overflow_x;
            y -= 8;
        } else {
            x -= 6;
        }
        keys_draw_cursor(x, y + 7, CURSOR_COLOR);
        cursor--;
    }

    void cursor_left()
    {
        if (cursor != 0)
            cursor_left_unchecked();
    }

    void input(char c)
    {
        if (length == 255)
            return;
        if (length == cursor) {
            arr[length] = c;
            arr[length + 1] = 0;
            term_print(x, y, arr + length);
            ++length;
        } else {
            term_color(0);
            term_print(x, y, arr + cursor);
            for (int i = length - 1; i >= cursor; i--) {
                arr[i + 1] = arr[i];
            }
            arr[cursor] = c;
            arr[++length] = 0;

            term_color(0xFFFFFF);
            term_print(x, y, arr + cursor);
        }
        cursor_right_unchecked();
    }

    void del()
    {
        if (cursor == 0)
            return;
        if (cursor == length) {
            cursor_left_unchecked();
            length--;
            term_color(0);
            term_print(x, y, arr + length);
            term_color(0xFFFFFF);
            arr[length] = 0;
        } else {
            cursor_left_unchecked();
            term_color(0);
            term_print(x, y, arr + cursor);
            for (int i = cursor; i < length; i++) {
                arr[i] = arr[i + 1];
            }
            length--;
            arr[length] = 0;
            term_color(0xFFFFFF);
            term_print(x, y, arr + cursor);
        }
    }
};

void input(u8 init_x, u8 init_y, char* data)
{
    Input input(init_x, init_y, data);
    keys_draw_cursor(init_x, init_y + 7, CURSOR_COLOR);
    term_color(0xFFFFFF);
    while (1) {
        u8 key = keys_require_key();
        if (key == KEY_ENTER) {
            break;
        } else if (key >= ASCII_0 && key <= ASCII_9) {
            input.input(key);
        } else if (key == KEY_LEFT) {
            input.cursor_left();
        } else if (key == KEY_RIGHT) {
            input.cursor_right();
        } else if (key == KEY_BACKSPACE) {
            input.del();
        }
    }
}

int main()
{
    clear_screen(0);
    term_color(0xFFFFFF);
    term_print(0, 245, "Hello, world!");
    char data[256];
    input(0, 0, data);
    return 0;
}