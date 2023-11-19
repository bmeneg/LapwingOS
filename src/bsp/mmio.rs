#![allow(dead_code)]

use core::{marker::PhantomData, ops::Deref};

// BCM2711 has three memory mapped access mode
#[allow(dead_code)]
pub enum PeripheralMode {
    Legacy, // 32-bits seen by the peripherics
    Low,    // VPU "low peripheral" mode enabled
    Full,   // 35-bit addresses, seen by "large addresses peripherics" (eg. DMA4)
}

pub struct MemMap<T> {
    base_addr: usize,
    phantom: PhantomData<T>,
}

impl<T> Deref for MemMap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.base_addr as *const T) }
    }
}

impl<T> MemMap<T> {
    pub const fn new(base_addr: usize) -> Self {
        Self {
            base_addr: Self::addr_translate(PeripheralMode::Low, base_addr),
            phantom: PhantomData,
        }
    }

    // When acessing the peripherics it's always important to make sure the
    // correct memory is being accessed.
    const fn addr_translate(mode: PeripheralMode, addr: usize) -> usize {
        // TODO: Get in what state we working on (VPU low peripheral mode?)
        let curr_mode = PeripheralMode::Legacy;

        match mode {
            PeripheralMode::Legacy => match curr_mode {
                PeripheralMode::Low => addr - 0x8000_0000,
                PeripheralMode::Legacy => addr,
                PeripheralMode::Full => addr + 0x4_0000_0000,
            },
            PeripheralMode::Low => match curr_mode {
                PeripheralMode::Low => addr,
                PeripheralMode::Legacy => addr + 0x8000_0000,
                PeripheralMode::Full => addr + 0x3_8000_0000,
            },
            PeripheralMode::Full => match curr_mode {
                PeripheralMode::Low => addr - 0x3_8000_0000,
                PeripheralMode::Legacy => addr - 0x4_0000_0000,
                PeripheralMode::Full => addr,
            },
        }
    }
}
