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
#![feature(const_fn_fn_ptr_basics)]
#![warn(missing_docs)]
// const BANNER: &str = include_str!("../root/banner.txt");
// const ROOT: &[u8] = include_bytes!("../root");

extern crate alloc;

use bootloader::BootInfo;
use vga::{colors::Color16, writers::GraphicsWriter};

pub use kernel_state::{KernelState, KernelVersion};
use window_manager::GRAPHICS;

/// The global allocator impl
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
mod vga_buffer;

/// Asyncronous module
pub mod task;

/// The holder of tests
#[cfg(test)]
pub mod test;

mod devices;
pub mod drivers;
mod kernel_state;
mod ps2_mouse;
mod sri;
mod time;
pub mod window_manager;

/// Defines the entry point function.
///
/// The function must have the signature `fn(&'static BootInfo) -> !`.
///
/// This creates a function named `_start`, which the linker will use as the entry
/// point.
#[export_name = "_start"]
pub extern "C" fn __impl_start(boot_info: &'static ::bootloader::bootinfo::BootInfo) -> ! {
	let f: fn(&'static ::bootloader::bootinfo::BootInfo) -> ! = kernel_main;
	f(boot_info)
}

/// Checks if APIC is available
fn check_apic() -> bool {
	true
}

/// The "Start" point of ableOS
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	init_alloc(boot_info);
	init();
	init_graphics();

	//    info!("{:#?}", boot_info);

	use alloc::format;
	let v_str = format!("{}", kernel_state::KERNEL_STATE.lock().version);
	println(&v_str, (0, 0));

	#[cfg(test)]
	test_main();
	use cpuio::outw;
	unsafe {
		outw(0x604, 0x2000);
	}

	// reason for without_interrupts: mouse interrupt handler and init_mouse acquires the same mutex
	x86_64::instructions::interrupts::without_interrupts(|| {
		drivers::mouse::init_mouse();
	});

	fn println(yes: &str, coordinates: (usize, usize)) {
		let mut offset = 0;
		let mut offset_y = 0;

		for x in yes.chars() {
			match x {
				'\n' => {
					offset = 0;
					offset_y += 1;
				}
				_ => {
					GRAPHICS.draw_character(
						offset * 8 + coordinates.0,
						offset_y * 8,
						x,
						Color16::White,
					);
					offset += 1;
				}
			}
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
		success!("Encryption driver loaded")
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

	success!("Allocator loaded");
}

async fn async_number() -> u32 {
	42
}

async fn example_task() {
	let number = async_number().await;
	info!("async number: {}", number);
}

async fn test_1() {
	use alloc::vec::Vec;

	info!("performing async task: vec allocation");

	let mut vec = Vec::new();
	for i in 0..500 {
		vec.push(i);
	}
}

fn init_graphics() {
	let mut seven = 0;
	let mut nine = 0;
	for x in 0..10 {
		window_manager::window_draw::windows(x, (seven, nine));
		seven += 40;
		nine += 40;
	}
	window_manager::window_draw::logo((440, 420));
}
