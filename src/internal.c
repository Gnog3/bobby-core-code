void* memcpy (void *dest, const void *src, int len)
{
  char *d = dest;
  const char *s = src;
  while (len--)
    *d++ = *s++;
  return dest;
}

void* memset (void *dest, int val, unsigned int len)
{
  volatile unsigned char *ptr = dest;
  while (len-- > 0)
    *ptr++ = val;
  return dest;
}

unsigned int
__mulsi3 (unsigned int a, unsigned int b)
{
    unsigned int r = 0;

    while (a)
    {
        if (a & 1)
            r += b;
        a >>= 1;
        b <<= 1;
    }
    return r;
}
