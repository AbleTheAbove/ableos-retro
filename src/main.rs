//! My amazing(?) kernel

#![no_std] // Makes sure the STD library is not included as we can not use it
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)] // at the top of the file
#![feature(const_mut_refs)]
#![feature(asm)]
// #![deny(missing_docs)]

pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");

extern crate alloc;

/// The public allocator module
pub mod allocator;
mod encrypt;
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
mod vga_buffer;

pub mod task;
use cpuio::outb;
use logger::{log, LogLevel};

/// The holder of tests
#[cfg(test)]
pub mod test;

mod sri;

use bootloader::{entry_point, BootInfo};

/// Undocumnetable
entry_point!(kernel_main);

/// The "Start" point of ableOS
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    util::banner();
    init_alloc(boot_info);
    init();
    unsafe {
        outb(0x0A, 0x3D4);
        outb(0x20, 0x3D5);
    }
    //    println!("{:?}", x86_64::instructions::random::RdRand(()).get_u64());

    #[cfg(test)]
    test_main();
    use vga::colors::Color16;
    if false {
        use vga::writers::{Graphics640x480x16, GraphicsWriter};

        let mode = Graphics640x480x16::new();
        mode.set_mode();
        mode.clear_screen(Color16::Black);
        mode.draw_line((80, 60), (80, 420), Color16::White);
        mode.draw_line((80, 60), (540, 60), Color16::White);
        mode.draw_line((80, 420), (540, 420), Color16::White);
        mode.draw_line((540, 420), (540, 60), Color16::White);
        mode.draw_line((80, 90), (540, 90), Color16::White);
        for (offset, character) in "AbleOS Window Example".chars().enumerate() {
            mode.draw_character(270 + offset * 8, 72, character, Color16::White)
        }
    } else {
        use vga::colors::TextModeColor;
        use vga::writers::{ScreenCharacter, Text80x25, TextWriter};

        let text_mode = Text80x25::new();
        let color = TextModeColor::new(Color16::Yellow, Color16::Black);

        text_mode.set_mode();
        text_mode.clear_screen();

        for (offset, character) in "Hello ableOS!".chars().enumerate() {
            let screen_character = ScreenCharacter::new(character as u8, color);
            text_mode.write_character(offset, 0, screen_character);
        }
    }

    use task::{executor::Executor, keyboard, Task};

    let mut executor = Executor::new();

    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.spawn(Task::new(test_1()));
    executor.run();
}
/// Initialize
pub fn init() {
    gdt::init();
    interrupts::init_idt();

    unsafe { interrupts::pic::PICS.lock().initialize() }; // new
    x86_64::instructions::interrupts::enable(); // new
    if encrypt::aes_detect() {
        log(LogLevel::Success);
        // println!("Encryption driver loaded");
    }

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

    log(LogLevel::Success);
    // println!("Allocator loaded");
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    // println!("async number: {}", number);
}

async fn test_1() {
    use alloc::vec::Vec;

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    // println!("vec at {:p}", vec.as_slice());
}
