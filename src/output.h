#include "csr.h"
#include "typedef.h"

inline void printstr(int color, int posx, int posy, char* str)
{
    csr_write(1024, color);
	
	int pos = (posy << 8) | posx;
    csr_write(1025, pos);
	
	csr_write(1026, str);	
}

inline void printint(int color, int posx, int posy, int value) 
{
    csr_write(1024, color);
	
	int pos = (posy << 8) | posx;
    csr_write(1025, pos);
	
	csr_write(1027, value);	
}

inline void drawline(int color, int posx0, int posy0, int posx1, int posy1 ) 
{
    csr_write(1028, color);
	int verts =  ((posy1 << 24)|(posx1 << 16)|(posy0 << 8) | posx0);
    csr_write(1029, verts);	
}

inline void drawtriangle(int color, int posx0, int posy0, int posx1, int posy1, int posx2, int posy2 ) 
{
    csr_write(1030, color);
	int twopnts =  ((posy1 << 24)|(posx1 << 16)|(posy0 << 8) | posx0);
    csr_write(1031, twopnts);
	int onepnt = (posy2 << 8) | posx2;
	csr_write(1032, onepnt);	
}

inline void plotpix(int color, int x, int y)
{
	csr_write(1033, color);
	int pos = (y << 8) | x ;
	csr_write(1034, pos);
}


