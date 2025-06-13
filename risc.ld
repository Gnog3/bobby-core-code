ENTRY(_start);

SECTIONS
{
	. = 0;

	PROVIDE( __global_pointer$ = . + 0x800 );

	.text :
	{
		*(.text._actual_start)
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
	PROVIDE( __bss_start = .);
	.bss :
	{
		*(.bss)
	}
	PROVIDE( _end = .);
	PROVIDE( STATIC_PROGRAM_SIZE = .);
}