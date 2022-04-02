#include "keytest.h"
#include "../typedef.h"
#include "../output.h"
#include "../input.h"

void keytest() {
	char data[16] = {0};
    term_color(0xFFFFFF);
    term_position_raw(0);
	u8 cursor = 0;
#pragma clang diagnostic push
#pragma ide diagnostic ignored "EndlessLoop"
	while (1) {
		u8 key = read_key();
        if ((key >> 7) == 0 && key != 0) continue;
        key &= 0x7F;
		if (key != 0 && cursor < 15) {
			if (key >= 0x30 && key <= 0x39) {
				data[cursor] = key;
				++cursor;
			}
		} else if (cursor > 0) {
            term_print_raw(data);
			for (int i = 0; i < cursor; i++) {
				data[i] = 0;
			}
			cursor = 0;
		}
	}
#pragma clang diagnostic pop
}
