// Public trait to be implemented by those device drivers willing to be used
// as consoles for a certain BSP.

use super::sync;
use core::fmt;

// The fmt::Write trait requires write_str() function to be implemented,
// once it's done both write_char and write_fmt are provided, being the
// later used to build the formatted printing macro.
pub trait Console: fmt::Write {
    fn read_char(&self) -> u8;
}

static SYSTEM_CONSOLE: sync::SafeStaticData<&'static dyn Console> =
    sync::SafeStaticData::new(&DUMMY_CONSOLE);

pub fn system() -> &'static dyn Console {
    *SYSTEM_CONSOLE.inner()
}

pub fn register_console(console: &'static dyn Console) {
    SYSTEM_CONSOLE.set_inner(console);
}

// Print macros are built at compile time, thus we need to have a static
// instance of what gonna be the system console. However, since we need to
// first initialize the whole BSP hardware we create a dummy_console with
// void implementation of the required methods.
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

static DUMMY_CONSOLE: DummyConsole = DummyConsole {};
