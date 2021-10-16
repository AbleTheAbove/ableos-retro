//! My amazing(?) kernel

#![no_std] // Makes sure the STD library is not included as we can not use it
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
//#![test_runner(ableos::test::test_runner)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)] // at the top of the file
#![feature(const_mut_refs)]
#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(repr128)]
#![allow(incomplete_features)]

use ableos::*;

/// Defines the entry point function.
///
/// The function must have the signature `fn(&'static BootInfo) -> !`.
///
/// This creates a function named `_start`, which the linker will use as the entry
/// point.

/// Start function
#[export_name = "_start"]
pub extern "C" fn __impl_start(boot_info: &'static ::bootloader::bootinfo::BootInfo) -> ! {
	init_alloc(boot_info);
	kernel_main();
}

mod window_manager;

/// The "Start" point of ableOS
fn kernel_main() -> ! {
	init();

	// info!("{:#?}", boot_info);
	// rash::shell();

	// │ ┤ ┐ └ ┴ ┬ ├ ─ ┼ ┘ ┌

	#[cfg(test)]
	test_main();

	unsafe {
		// Broken QEMU shutdown
		outw(0x604, 0x2000);
	}

	// reason for without_interrupts: mouse interrupt handler and init_mouse acquires the same mutex
	x86_64::instructions::interrupts::without_interrupts(|| {
		hardware::mouse::init_mouse();
	});

	use task::{executor::Executor, keyboard, Task};
	let mut executor = Executor::new();
	executor.spawn(Task::new(example_task()));
	executor.spawn(Task::new(keyboard::print_keypresses()));
	executor.spawn(Task::new(test_1()));

	// FIXME: Freezes the kernel
	// executor.spawn(Task::new(_test_clipboard()));
	executor.run();
}

/// Initialize
pub fn init() {
	gdt::init();
	interrupts::init_idt();
	unsafe { interrupts::pic::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
	if hardware::encrypt::aes_detect() {
		success!("Encryption driver loaded")
	}

	sri::init();
}

async fn _test_clipboard() {
	// FIXME: pauses execution of key board task
	info!("{:?}", CLIPBOARD.lock().paste());
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
	info!("performing async task: vec allocation");

	let mut vec = Vec::new();
	for i in 0..500 {
		vec.push(i);
	}
}
