#![allow(dead_code)]
#![allow(unused_variables)]

// Copied from the original macro definition.
// Refs:
//  https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) =>
        ($crate::console::system().print_fmt(format_args!($($arg)*)));
}

// Copied from the original macro definition.
// Refs:
//  https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::console::system().print_fmt(format_args_nl!($($arg)*));
    })
}
