/* The RPi4B firmware requires one of the following physical addrs for the
 * kernel image:
 *
 * 0x8000: 32 bits
 * 0x80000: 64 bits w/ flat image
 * 0x20000: 64 bits w/ compressed (gzip) image
 *
 * TODO: The source of this information is questionable. I need to dive into
 * it a bit more.
 */
kernel_load_addr = 0x80000;


ENTRY(kernel_load_addr)

/* Flags: 4 (readonly), 5 (read-exec), 6 (read-write) */
PHDRS
{
    segment_code PT_LOAD FLAGS(5);
}

SECTIONS
{
    . = kernel_load_addr;
    .text :
    {
        KEEP(*(.text._start))
    } :segment_code
}
