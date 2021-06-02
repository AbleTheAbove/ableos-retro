//! My amazing(?) kernel

#![no_std] // Makes sure the STD library is not included as we can not use it
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)] // at the top of the file

extern crate alloc;

pub mod allocator;

/// Global Descriptor Table
pub mod gdt;
/// Interrupt module
pub mod interrupts;

/// A logging assistance crate
pub mod logger;
/// Memory management
pub mod memory;
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

use bootloader::{entry_point, BootInfo};
entry_point!(kernel_main);

/// The "Start" point of ableOS
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    init_alloc(boot_info);
    util::banner();
    init();

    use alloc::vec::Vec;

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    #[cfg(test)]
    test_main();

    hlt_loop();
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
/// Loop forever
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

fn init_alloc(boot_info: &'static BootInfo) {
    use memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}
