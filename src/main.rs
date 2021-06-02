//! My amazing(?) kernel

#![no_std] // Makes sure the STD library is not included as we can not use it
#![no_main] // disable all Rust-level entry points
#![deny(missing_docs)] // Stops compiling if docs aren't added
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

/// Global Descriptor Table
pub mod gdt;
/// Interrupt module
pub mod interrupts;

/// A logging assistance crate
pub mod logger;
mod panic;
mod serial;
/// A simple utility module to reduce repeated code
pub mod util;
mod vga;

use logger::{log, LogLevel};

/// The holder of tests
#[cfg(test)]
pub mod test;

mod sri;
/// The "Start" point of ableOS
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    util::banner();
    init();

    #[cfg(test)]
    test_main();
    loop {
        x86_64::instructions::hlt();
    }
}
/// Initialize
pub fn init() {
    gdt::init();
    interrupts::init_idt();

    unsafe { interrupts::pic::PICS.lock().initialize() }; // new
    x86_64::instructions::interrupts::enable(); // new

    log(LogLevel::Success);
    println!("VGA buffer loaded");
    sri::init();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
