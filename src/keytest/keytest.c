#include "keytest.h"
#include "../typedef.h"
#include "../output.h"
#include "../input.h"

void keytest() {
	char data[16] = {0}; 
	csr_write(1024, 0xFFFFFF);
	csr_write(1025, 0);
	u8 cursor = 0;
	while (1) {
		u8 key = read_key() & 0b1111111;
		if (key != 0 && cursor < 15) {
			if (key >= 0x30 && key <= 0x39) {
				data[cursor] = key;
				++cursor;
			}
		} else if (cursor > 0) {
			csr_write(1026, data);
			__asm__("nop");
			for (int i = 0; i < cursor; i++) {
				data[i] = 0;
			}
			cursor = 0;
		}
	}
}
