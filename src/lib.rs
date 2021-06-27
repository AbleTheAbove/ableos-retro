// Makes sure the STD library is not included as we can not use it
#![no_std]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(test::test_runner)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)] // at the top of the file
#![feature(const_mut_refs)]
#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![warn(missing_docs)]
#![feature(repr128)]
#![allow(incomplete_features)]

//! TODO: DOCUMENT

pub extern crate alloc;

pub use acpi::AcpiTables;

pub use alloc::string::String;
pub use bootloader::BootInfo;
pub use cpuio::outw;
/// kernel state
pub use kernel_state::{KernelState, KernelVersion};
// use uefi::{self, table::{Table, boot}};
// use uefi_services;
//use vga::{colors::Color16, writers::GraphicsWriter};

//use alloc::format;
//use window_manager::GRAPHICS;

/// The AbleOS Shell
pub use rash;

pub use hardware::cpu::cpu_vendor_signature;

/// The global allocator impl
pub mod allocator;
//pub mod encrypt;
/// Global Descriptor Table
pub mod gdt;
/// Interrupt module
pub mod interrupts;

pub mod hardware;

/// A logging assistance crate
pub mod logger;
pub use logger::*;
/// Memory management
pub mod memory;
pub mod panic;
pub mod serial;
pub mod vga_buffer;

/// Asyncronous module
pub mod task;
pub mod user;

/// The holder of tests
#[cfg(test)]
pub mod test;

pub mod devices;
pub mod drivers;
pub mod kernel_state;
pub mod ps2_mouse;
pub mod sri;
/// A work in progress time module for real time clock and computer time + user time offset
pub mod time;

/// The window manager module
pub mod window_manager;
pub use vga::writers::GraphicsWriter;
pub use window_manager::*;

/// Loop forever
pub fn hlt_loop() -> ! {
	loop {
		x86_64::instructions::hlt();
	}
}
