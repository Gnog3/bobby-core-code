.global _start
_start:
    li sp, 8192

    /* Call main, leaving the arguments undefined. */
    call main

    /* Spin after main() returns. */
1:
    j 1b
