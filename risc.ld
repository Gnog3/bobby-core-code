ENTRY(_start);

SECTIONS
{
	. = 0;

	PROVIDE( __global_pointer$ = . + 0x800 );

	.text :
	{
		*(.text._start)
		*(.text)
	}
	.data :
	{
		*(.data)
	}
	.rodata :
	{
		*(.rodata)
	}
	.bss :
	{
		*(.bss)
	}

	PROVIDE( STATIC_PROGRAM_SIZE = .);
}