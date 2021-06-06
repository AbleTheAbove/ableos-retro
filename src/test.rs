/// The tests
<<<<<<< HEAD
use crate::{
	logger::{slog, LogLevel::Success},
	serial_println,
};
=======
use crate::{serial_println, success};
>>>>>>> 8a9091888dc077c9253bdeb9205bf14f41b03bb9

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
	serial_println!("Running {} tests", tests.len());
	for test in tests {
		test.run(); // new
	}
	exit_qemu(QemuExitCode::Success);
}

/// Qemu exit codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	///The Success code
	Success = 0x10,
	///The Failed code
	Failed = 0x11,
}
/// Quits qemu with the provided exit code
pub fn exit_qemu(exit_code: QemuExitCode) {
	use x86_64::instructions::port::Port;

	let mut port = Port::new(0xf4);
	unsafe {
		port.write(exit_code as u32);
	}
}

/// Trait to simplify my life
pub trait Testable {
	/// Run the test

	fn run(&self) -> ();
}
impl<T> Testable for T
where
	T: Fn(),
{
<<<<<<< HEAD
	/// Run the test
	fn run(&self) {
		slog(Success);
		serial_println!("{}\t", core::any::type_name::<T>());
		self();
	}
=======
    /// Run the test
    fn run(&self) {
        success!("{}\t", core::any::type_name::<T>());
        self();
    }
>>>>>>> 8a9091888dc077c9253bdeb9205bf14f41b03bb9
}
