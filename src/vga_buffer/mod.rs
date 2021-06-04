// use core::fmt;
// use lazy_static::lazy_static;
// use spin::Mutex;
// use volatile::Volatile;

// /// The kernel print macro
// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
// }

// /// The kernel println macro
// #[macro_export]
// macro_rules! println {
//     () => ($crate::print!("\n"));
//     ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
// }

// #[doc(hidden)]
// pub fn _print(args: fmt::Arguments) {
//     use core::fmt::Write;
//     use x86_64::instructions::interrupts; // new

//     interrupts::without_interrupts(|| {
//         // new
//         WRITER.lock().write_fmt(args).unwrap();
//     });
// }
