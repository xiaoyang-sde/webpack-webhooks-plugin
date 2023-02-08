//! The `console` module contains functions that interacts with the debug console.
//! It exports useful macros such as `print!` and `println!`.

use crate::sbi;
use core::fmt::{self, Write};

struct Console;

impl Write for Console {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for char in string.bytes() {
            sbi::console_putchar(char as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Console.write_fmt(args).unwrap();
}

/// Print to the debug console.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::console::print(format_args!($($arg)*));
    });
}

/// Print to the debug console, with a newline.
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (crate::print!(concat!($fmt, '\n')));
    ($fmt:expr, $($arg:tt)*) => (crate::print!(concat!($fmt, '\n'), $($arg)*));
}
