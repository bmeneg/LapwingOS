#![allow(dead_code)]

// Read and Write operations are abstracted by tock_registers, with that we
// don't need to create additional abstraction for generic read() and
// write() into ::mmio or other crate.
use tock_registers::{
    bitmask,
    fields::FieldValue,
    interfaces::ReadWriteable,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

use crate::{
    bsp::mmio,
    klib::{device, sync},
};

pub enum PinFnSpec {
    Input,
    Output,
    Alt5,
    Alt4,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
}

pub enum PinPullSpec {
    None,
    Up,
    Down,
}

#[repr(C)]
struct Registers {
    fsel0: ReadWrite<u32>,
    fsel1: ReadWrite<u32>,
    fsel2: ReadWrite<u32>,
    fsel3: ReadWrite<u32>,
    fsel4: ReadWrite<u32>,
    fsel5: ReadWrite<u32>,
    _reserved0: u32,
    set0: WriteOnly<u32>,
    set1: WriteOnly<u32>,
    _reserved1: u32,
    clr0: ReadOnly<u32>,
    clr1: ReadOnly<u32>,
    _reserved2: u32,
    lev0: ReadOnly<u32>,
    lev1: ReadOnly<u32>,
    _reserved3: u32,
    eds0: ReadWrite<u32>, // write 0b1 to a bit clears it
    eds1: ReadWrite<u32>, // write 0b1 to a bit clears it
    _reserved4: u32,
    ren0: ReadWrite<u32>,
    ren1: ReadWrite<u32>,
    _reserved5: u32,
    fen0: ReadWrite<u32>,
    fen1: ReadWrite<u32>,
    _reserved6: u32,
    hen0: ReadWrite<u32>,
    hen1: ReadWrite<u32>,
    _reserved7: u32,
    len0: ReadWrite<u32>,
    len1: ReadWrite<u32>,
    _reserved8: u32,
    aren0: ReadWrite<u32>,
    aren1: ReadWrite<u32>,
    _reserved9: u32,
    afen0: ReadWrite<u32>,
    afen1: ReadWrite<u32>,
    _reserved10: [u32; 16],
    pull_cntrl0: ReadWrite<u32>,
    pull_cntrl1: ReadWrite<u32>,
    pull_cntrl2: ReadWrite<u32>,
    pull_cntrl3: ReadWrite<u32>,
}

// In GPIO we're working only with u32 repr regs and to facilitate modifying
// reg values (that we didn't map with register_bitfields!() macro) we
// directly create the field value.
type GPIOFieldValue = FieldValue<u32, ()>;

pub struct GPIO {
    registers: mmio::MemMap<Registers>,
}

impl device::DriverDescriptor for GPIO {
    fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

// GPIO registers are known to hold values for every available pin, so it's
// important to always modify values without touch others. At the same time,
// since every bit might be mapped to a different pin we're going to set
// values using raw shift arithmetic instead of mapping every possible field
// with register_bitfields macro.
//
// With that, when we're handling regs that each bit maps to a pin the
// arithmetic is straightforward:
//
//  value << <pin number>
//
// But when a register uses multiple bits for a single pin (eg. pin function
// selector (PSEL) and pin pull up/down controller (PUD_PDN_CNTRL)) we end
// up with multiple sequencial registers for setting the values, requiring
// us to first define in which register we want modify the value and later
// the offset we must set to touch the right bits. For that, we can use the
// following arithmetic:
//
//  value << ((<pin number> % <number of pins on reg>) * <number of bits>)
//
// For easing our job we can make use of the tock_registers function
// FieldValue::new with the bitmask macro. And for easing users life we
// export two functions to work with these registers.
impl GPIO {
    pub const fn new() -> Self {
        Self {
            registers: mmio::MemMap::new(0x7e20_0000),
        }
    }

    pub fn set_function(&self, pin: usize, alt: PinFnSpec) {
        let fsel = match pin {
            0..=9 => &self.registers.fsel0,
            10..=19 => &self.registers.fsel1,
            20..=29 => &self.registers.fsel2,
            30..=39 => &self.registers.fsel3,
            40..=49 => &self.registers.fsel4,
            50..=57 => &self.registers.fsel5,
            _ => panic!("out of GPIO pin range"),
        };
        let value = GPIOFieldValue::new(bitmask!(3), (pin % 10) * 3, alt as u32);
        fsel.modify(value);
    }

    pub fn set_pull(&self, pin: usize, pull: PinPullSpec) {
        let pull_cntrl = match pin {
            0..=15 => &self.registers.pull_cntrl0,
            16..=31 => &self.registers.pull_cntrl1,
            32..=47 => &self.registers.pull_cntrl2,
            48..=57 => &self.registers.pull_cntrl3,
            _ => panic!("out of GPIO pin range"),
        };
        let value = GPIOFieldValue::new(bitmask!(2), (pin % 16) * 2, pull as u32);
        pull_cntrl.modify(value)
    }
}

static GPIO_DEVICE_DATA: sync::SafeStaticData<GPIO> = sync::SafeStaticData::new(GPIO::new());

pub fn device() -> &'static mut GPIO {
    GPIO_DEVICE_DATA.inner()
}

pub fn build() -> device::DeviceDriver {
    device::DeviceDriver {
        description: "BCM2711 GPIO",
        descriptor: device(),
        phase: device::InitPhase::Hardware,
    }
}
