ENTRY(_start);

SECTIONS
{
	. = 0x0;

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
}