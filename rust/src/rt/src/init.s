.section .init
.global _start

_start:
    /* set stack pointer */
    li sp, 1024*1024

    /* call startup */
    call startup
