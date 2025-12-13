#pragma once

#include "csr.h"

#define KEY_ENTER 0x10
#define KEY_UP 0x11
#define KEY_LEFT 0x12
#define KEY_DOWN 0x13
#define KEY_RIGHT 0x14
#define KEY_BACKSPACE 0x0D

#define KEY_A 0x41
#define KEY_B 0x42
#define KEY_C 0x43
#define KEY_D 0x44
#define KEY_E 0x45
#define KEY_F 0x46
#define KEY_G 0x47
#define KEY_H 0x48
#define KEY_I 0x49
#define KEY_J 0x4A
#define KEY_K 0x4B
#define KEY_L 0x4C
#define KEY_M 0x4D
#define KEY_N 0x4E
#define KEY_O 0x4F
#define KEY_P 0x50
#define KEY_Q 0x51
#define KEY_R 0x52
#define KEY_S 0x53
#define KEY_T 0x54
#define KEY_U 0x55
#define KEY_V 0x56
#define KEY_W 0x57
#define KEY_X 0x58
#define KEY_Y 0x59
#define KEY_Z 0x5A

#define KEY_MINUS 0x2D
#define KEY_EQUALS 0x3D
#define KEY_LBRACKET 0x5B
#define KEY_RBRACKET 0x5D
#define KEY_SEMICOLON 0x3B
#define KEY_APOSTROPHE 0x27
#define KEY_COMMA 0x2C
#define KEY_PERIOD 0x2E
#define KEY_SLASH 0x2F
#define KEY_TILDE 0x60
#define KEY_BACKSLASH 0x5C

inline char read_key()
{
    return csr_read(CSR_KEYBOARD);
}
