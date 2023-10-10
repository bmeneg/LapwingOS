#![no_main]
#![no_std]

use core::arch::global_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Assembly file with CPU boot code.
global_asm!(include_str!("arch/boot.s"));

#[no_mangle]
pub fn _start_kernel() -> ! {
    loop {}
}
