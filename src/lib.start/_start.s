.global _actual_start
_actual_start:
    li sp, 400000
    li gp, 0x800
    /* Call shmain, leaving the arguments undefined. */
    call shmain

    /* Spin after shmain() returns. */
1:
    j 1b
