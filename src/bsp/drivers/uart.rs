#![allow(dead_code)]

use core::fmt;

// Read and Write operations are abstracted by tock_registers, with that we
// don't need to create additional abstraction for generic read() and
// write() into ::mmio or other crate.
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
    registers::{ReadOnly, ReadWrite},
};

use crate::{
    bsp::{drivers::gpio, mmio},
    klib::{console, device, sync},
};

register_bitfields! {
    u32,
    IRQStatus [
        UART OFFSET(0) NUMBITS(1) [],
    ],
    Enables [
        UART OFFSET(0) NUMBITS(1) [],
    ],
    IOData [
        DATA OFFSET(0) NUMBITS(8) [],
    ],
    IRQEnable [
        TX OFFSET(0) NUMBITS(1) [],
        RX OFFSET(1) NUMBITS(1) [],
    ],
    IRQIdent [
        IRQ_PENDING OFFSET(0) NUMBITS(1) [],
        // ID bits has dual behavior depending if it's being read or written
        ID_READ OFFSET(1) NUMBITS(2) [
            IRQ_free = 0b00,
            TX_empty = 0b01,
            RX_byte_available = 0b10,
        ],
        ID_WRITE OFFSET(1) NUMBITS(2) [
            RX_FIFO_clear = 0b01,
            TX_FIFO_clear = 0b10,
        ],
    ],
    LineControl [
        DATA_SIZE OFFSET(0) NUMBITS(1) [
            Seven = 0b0,
            Eight = 0b1,
        ],
        BREAK OFFSET(6) NUMBITS(1) [],
        DLAB OFFSET(7) NUMBITS(1) [],
    ],
    Control [
        RX_EN OFFSET(0) NUMBITS(1) [],
        TX_EN OFFSET(1) NUMBITS(1) [],
        // We're ignoring RTS/CTS/... modem control signals
    ],
    Status [
        RX_FIFO_AVAILABLE OFFSET(0) NUMBITS(1) [
            Empty = 0b0,
            Load = 0b1,
        ],
        TX_FIFO_AVAILABLE OFFSET(1) NUMBITS(1) [
            Full = 0b0,
            Load = 0b1,
        ],
        RX_IDLE OFFSET(2) NUMBITS(1) [
            Busy = 0b0,
            Idle = 0b1,
        ],
        TX_IDLE OFFSET(3) NUMBITS(1) [
            Busy = 0b0,
            Idle = 0b1,
        ],
        RX_OVERRUN OFFSET(4) NUMBITS(1) [],
        TX_FULL OFFSET(5) NUMBITS(1) [],
        TX_EMPTY OFFSET(8) NUMBITS(1) [],
        TX_DONE OFFSET(9) NUMBITS(1) [],
        RX_NUM_BYTES OFFSET(16) NUMBITS(4) [],
        TX_NUM_BYTES OFFSET(24) NUMBITS(4) [],
    ],
    Baudrate [
        COUNTER OFFSET(0) NUMBITS(16) [],
    ],
}

// Those registers we won't use we're making them _reserved
#[repr(C)]
struct Registers {
    irq: ReadOnly<u32, IRQStatus::Register>,
    en: ReadWrite<u32, Enables::Register>,
    _reserved1: [u32; 16],
    io: ReadWrite<u32, IOData::Register>,
    ier: ReadWrite<u32, IRQEnable::Register>,
    iir: ReadWrite<u32, IRQIdent::Register>,
    lcr: ReadWrite<u32, LineControl::Register>,
    _reserved2: [u32; 4],
    cntl: ReadWrite<u32, Control::Register>,
    stat: ReadOnly<u32, Status::Register>,
    baud: ReadWrite<u32, Baudrate::Register>,
}

// UART1 is an Auxiliary peripheric, aka MiniUART
static UART1_DRIVER: sync::SafeStaticData<UART1> = sync::SafeStaticData::new(UART1::new());

pub struct UART1 {
    registers: mmio::MemMap<Registers>,
}

impl device::DriverDescriptor for UART1 {
    // The MiniUART clock is controlled by the VPU clock which can be set in
    // the config.txt file with the option `core_freq=`. Using the default
    // 250MHz frequence we can initialize the UART with the baudrate of
    // 115200, since the error rate is below the threshold.
    //
    // For that, we need to set the baud counter to 270 (value taken from
    // the formula given by the peripheral documentation):
    //
    // baudrate = (cpu clock)/(8 * (counter + 1))
    //
    // Also, once the MiniUART is enabled, it starts recieving data at
    // maximum baudrate, filling the FIFOs completely (they're always
    // enabled), but if we don't enable it we don't enable it, we also can't
    // access any of its registers for configuration. Hence, we enable it,
    // but disable both RX and TX to prevent using CPU cycles for nothing.
    fn init(&self) -> Result<(), &'static str> {
        // First, configure the GPIO pins 14 (TX) and 15 (RX)
        let gpio = gpio::device();
        gpio.set_function(14, gpio::PinFnSpec::Alt5);
        gpio.set_pull(14, gpio::PinPullSpec::Down);
        gpio.set_function(15, gpio::PinFnSpec::Alt5);
        gpio.set_pull(15, gpio::PinPullSpec::Down);

        // Allow access to the registers by enabling the peripheric
        self.registers.en.write(Enables::UART.val(1));
        // But to configure the UART, lets turn the lines off
        self.registers
            .cntl
            .write(Control::RX_EN.val(0) + Control::TX_EN.val(0));
        // Set the baudrate counter
        self.registers.baud.set(270);
        // 8 bytes data
        self.registers.lcr.write(LineControl::DATA_SIZE::Eight);
        // Enabled TX and RX IRQs
        self.registers
            .ier
            .write(IRQEnable::RX.val(1) + IRQEnable::TX.val(1));
        // Since we don't need to worry about FIFOs,as they're always
        // enabled, we can enable RX and TX now
        self.registers
            .cntl
            .write(Control::RX_EN.val(1) + Control::TX_EN.val(1));

        Ok(())
    }
}

impl UART1 {
    const fn new() -> Self {
        // Base address taken from the reference manual
        Self {
            registers: mmio::MemMap::new(0x7e21_5000),
        }
    }

    pub fn read_byte(&self) -> u8 {
        self.registers.io.get() as u8
    }

    pub fn write_byte(&self, byte: u8) {
        self.registers.io.set(byte as u32);
    }
}

// Implementing the fmt::Write::write_str function we automatically gain
// access to both write_char() and write_fmt(). The later is required to
// implement formatted printing macro.
impl fmt::Write for UART1 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_byte(c as u8);
        }

        Ok(())
    }
}

impl console::Console for UART1 {
    fn read_char(&self) -> u8 {
        self.read_byte()
    }
}

pub fn build() -> device::DeviceDriver {
    console::system().register(&UART1_DRIVER);
    device::DeviceDriver {
        description: "BCM2711 MiniUART",
        descriptor: UART1_DRIVER.inner(),
        phase: device::InitPhase::Protocol,
    }
}
