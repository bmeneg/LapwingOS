#![no_main]
#![no_std]
#![feature(core_intrinsics)]
#![feature(variant_count)]

mod bsp;
mod klib;

use core::{arch::global_asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Assembly file with CPU boot code.
global_asm!(include_str!("arch/boot.s"));

#[no_mangle]
pub fn _start_kernel() -> ! {
    bsp::drivers::devices_build();

    let dev_manager = klib::device::manager();
    dev_manager.init_drivers(None);

    loop {}
}
