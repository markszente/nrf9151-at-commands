MEMORY
{
  /* Trusted Firmware-M (TF-M) is flashed at the start */
  FLASH                             : ORIGIN = 0x00008000, LENGTH = 0xf8000
  MODEM                             : ORIGIN = 0x2000C568, LENGTH = 0x8000
  RAM                         (rwx) : ORIGIN = 0x20014568, LENGTH = 0x2BA98
}

