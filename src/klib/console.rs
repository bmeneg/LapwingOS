#![allow(dead_code)]

// Public trait to be implemented by those device drivers willing to be used
// as consoles for a certain BSP.

use super::sync::SafeStaticData;
use core::fmt;

// The fmt::Write trait requires write_str() function to be implemented,
// once it's done both write_char and write_fmt are provided, being the
// later used to build the formatted printing macro.
pub trait Console: fmt::Write {
    fn read_char(&self) -> u8;
}

pub struct SystemConsole<'a> {
    console: &'a SafeStaticData<dyn Console>,
}

impl<'a> SystemConsole<'a> {
    pub fn register(&mut self, console: &'a SafeStaticData<dyn Console>) {
        self.console = console;
    }

    pub fn print_fmt(&self, args: fmt::Arguments) {
        let console: &mut dyn Console;
        unsafe {
            console = &mut *self.console.data.get();
        }
        fmt::Write::write_fmt(console, args).unwrap();
    }
}

static SYSTEM_CONSOLE: SafeStaticData<SystemConsole> = SafeStaticData::new(SystemConsole {
    console: &DUMMY_CONSOLE,
});

pub fn system<'a>() -> &'static mut SystemConsole<'a> {
    SYSTEM_CONSOLE.inner()
}

// This dummy console is used just to satisfy an empty system console while
// the peripheric drivers aren't initialized.
struct DummyConsole;

impl fmt::Write for DummyConsole {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Ok(())
    }
}

impl Console for DummyConsole {
    fn read_char(&self) -> u8 {
        ' ' as u8
    }
}

static DUMMY_CONSOLE: SafeStaticData<DummyConsole> = SafeStaticData::new(DummyConsole {});
