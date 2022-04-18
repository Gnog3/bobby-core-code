#include "../lib/output.h"
#include "../lib/csr.h"
#include "../lib/arith.h"
#include <array>

const u8 square1_x[] = {
        0x20,
        0x20,
        0xC4,
		0xC4

};

const u8 square1_y[] = {
        0x20,
        0xC4,
        0x20,
		0xC4

};

int main()
{
    clear_screen(0);
    term_color(0xFFFFFF);
	red = 0xFF0000;
	green = 0x00FF00;
	blue = 0x0000FF;
	white = 0xFFFFFF;
	
	drawline(red, square1_x[0], square1_y[0], square1_x[1], square1_y[1]);
	drawline(green, square1_x[0], square1_y[0], square1_x[2], square1_y[2]);
	drawline(blue, square1_x[3], square1_y[3], square1_x[2], square1_y[2]);
	drawline(white, square1_x[3], square1_y[3], square1_x[1], square1_y[1]);

    return 0;
}