/* ROM at 0x02000000, RAM at 0x01000000 */
MEMORY {
    ROM (rx) : ORIGIN = 0x02000000, LENGTH = 16M
    RAM (rw) : ORIGIN = 0x01000000, LENGTH = 16M
}

ENTRY(_start);

SECTIONS
{
    /* --- ROM --- */
    .text : {
        KEEP(*(.text._start))
        *(.text .text.*)
    } > ROM

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    } > ROM

    /* --- FORBIDDEN SECTION --- */
    .data : {
        KEEP(*(.data*))
        ASSERT(SIZEOF(.data) == 0, "ERROR: .data section is forbidden. Use uninitialized globals instead.");
    } > ROM

    /* --- RAM --- */
    .bss (NOLOAD) : ALIGN(4)
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        __bss_end = .;
    } > RAM
}
