ENTRY(main)

SECTIONS
{
	. = 0x0;

	.text :
	{
		*(.text.main)
		*(.text)
	}
	.rodata :
	{
		*(.rodata)
	}
	.data :
	{
		*(.data)
	}
	.bss :
	{
		*(.bss)
	}
}