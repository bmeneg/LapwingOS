/* The RPi4B firmware requires one of the following physical addrs for the
 * kernel image:
 *
 * 0x8000: 32 bits
 * 0x200000: 64 bits (un)compressed (gzip) image:
 *		bootloader decides based on the signature found at the beginning of
 *		the image
 *
 * Source:
 * https://www.raspberrypi.com/documentation/computers/config_txt.html#arm_64bit
 * https://www.raspberrypi.com/documentation/computers/config_txt.html#kernel_address
 */
kernel_load_addr = 0x200000;
ENTRY(kernel_load_addr)

/* Program headers (segments) defining how the executable must be loaded:
 * We want to load the "code" segment with RX permissions, meaning we need
 * to load it with PF_R and PF_X bits set. Considering:
 *
 * PF_X = 1
 * PF_W = 2
 * PF_R = 4
 *
 * PF_R + PF_X = 5 <--- what we want to set
 *
 * Source: https://refspecs.linuxfoundation.org/elf/elf.pdf
 */
PHDRS
{
    code_phdr PT_LOAD FLAGS(5);
}

SECTIONS
{
    . = kernel_load_addr;
    .text :
    {
        KEEP(*(.text._start))
    } :code_phdr
}
