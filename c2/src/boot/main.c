#include "csr_defs.h"
#include <csr.h>
#include <ddi.h>
#include <keyboard.h>
#include <stdio.h>
#include <string.h>

#define START_OF_RAM ((volatile void*)0x01000000)

#define RAM(offset) (((volatile char*)START_OF_RAM) + offset)

int putchar(int c)
{
    csr_write(CSR_TERM_PUTCHAR, c);
    return c;
}

int get_pos()
{
    return csr_read(CSR_TERM_POSITION);
}

void set_pos(int pos)
{
    csr_write(CSR_TERM_POSITION, pos);
}

void print(const char* str)
{
    while (*str) {
        putchar(*str++);
    }
}

int puts(const char* s)
{
    print(s);
    return putchar('\n');
}

void clear_screen()
{
    for (int i = 0; i < TERM_HEIGHT; i++) {
        putchar('\n');
    }
}

// Convert number to decimal string
void int_to_str(int num, char* buf)
{
    if (num == 0) {
        buf[0] = '0';
        buf[1] = '\0';
        return;
    }

    char temp[12];
    int i = 0;

    while (num > 0) {
        temp[i++] = '0' + (num % 10);
        num = num / 10;
    }

    // Reverse into buf
    int j = 0;
    while (i > 0) {
        buf[j++] = temp[--i];
    }
    buf[j] = '\0';
}

// Read a line of input from keyboard (max 16 chars)
// Returns length of string
int read_line(char* buf, int max_len)
{
    int len = 0;

    while (1) {
        uint8_t key = read_key();
        if (key == 0) {
            continue;
        }

        // Wait for key press (high bit set)
        if (!(key & 0x80)) {
            continue;
        }

        key &= 0x7F;

        if (key == KEY_ENTER) {
            buf[len] = '\0';
            putchar('\n');
            return len;
        }

        if (key == KEY_BACKSPACE) {
            if (len > 0) {
                len--;
                int pos = get_pos();
                set_pos(pos - 1);
                putchar(' ');
                set_pos(pos - 1);
            }
            continue;
        }

        // Handle letter keys (A-Z) - convert to lowercase
        if (key >= KEY_A && key <= KEY_Z) {
            if (len < max_len) {
                char c = (char)(key - KEY_A + 'a');
                buf[len++] = c;
                putchar(c);
            }
            continue;
        }
    }
}

void print_number(int num)
{
    char buf[12];
    int_to_str(num, buf);
    print(buf);
}

void bmain()
{
    clear_screen();
    puts("=== NUMBER GUESSING GAME ===");
    puts("");
    puts("Think of a number from 0 to 1000.");
    puts("I will try to guess it!");
    puts("");
    puts("Answer with:");
    puts("  greater - if your number is higher");
    puts("  less    - if your number is lower");
    puts("  equal   - if I guessed it!");
    puts("");
    puts("Press any key to start...");

    // Wait for any key press
    while (1) {
        uint8_t key = read_key();
        if (key != 0 && (key & 0x80)) {
            break;
        }
    }

    int low = 0;
    int high = 1000;
    int attempts = 0;
    char input[17];
    int valid = 0;
    while (low <= high) {
        int guess = low + (high - low) / 2;
        attempts++;

        puts("");
        print("Is your number ");
        print_number(guess);
        puts("?");
        print("> ");

        read_line(input, 16);

        if (strcmp(input, "equal") == 0) {
            puts("");
            print("I guessed it in ");
            print_number(attempts);
            puts(" attempts!");
            puts("");
            puts("Thanks for playing!");
            valid = 1;
            break;
        } else if (strcmp(input, "greater") == 0) {
            low = guess + 1;
        } else if (strcmp(input, "less") == 0) {
            high = guess - 1;
        } else {
            puts("Please enter: greater, less, or equal");
            attempts--; // Don't count invalid input
        }
    }

    if (!valid) {
        puts("");
        puts("Something went wrong...");
        puts("Are you sure you followed the rules?");
    }
    halt();
}
