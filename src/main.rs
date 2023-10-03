#![no_main]
#![no_std]

use core::arch::global_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

// Assembly counterpart to this file.
global_asm!(include_str!("arch/boot.s"));
