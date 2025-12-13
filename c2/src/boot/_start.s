.global _start
_start:
    li sp, 0x01000000 + 4 * 1024
    call bmain
1:
    j 1b
