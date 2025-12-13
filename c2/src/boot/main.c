#include "alloc/alloc.h"
#include "debug/dbg.h"
#include "debug/dbg_print.h"
#include "fs/fs.h"
#include "install.h"
#include <csr.h>
#include <string.h>
#include <ddi.h>

#define START_OF_RAM ((volatile void*)0x01000000)

#define RAM(offset) (((volatile char*)START_OF_RAM)+offset)

void bmain()
{
    *RAM(0) = 5;
    *(volatile short*)RAM(2) = 10;

    char a = *RAM(0);
    short b = *(volatile short*)RAM(2);
    int result = ((int) a) * ((int) b);
    csr_write(CSR_HEAP, result);
    
    halt();
}
