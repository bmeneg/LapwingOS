#![allow(dead_code)]
#![allow(unused_variables)]

use super::console;
use core::fmt;

fn _print(args: fmt::Arguments) {
    console::system().write_fmt(args).unwrap();
}

// Copied from the original macro definition.
// Refs:
//  https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

// Copied from the original macro definition.
// Refs:
//  https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args_nl!($($arg)*));
    })
}
