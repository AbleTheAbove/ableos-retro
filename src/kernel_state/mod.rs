use alloc::string::{String, ToString};

use core::fmt;
use lazy_static::lazy_static;

use crate::{
	// Hardware representation
	hardware::{cpu::Cpu, encrypt::aes_detect, Hardware},
	interrupts::has_apic,
};

/// TODO: owo what is this?
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
/// A constant to check if the kernel is in debug mode
pub const RELEASE_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
/// A constant to check if the kernel is in release mode
pub const RELEASE_TYPE: &str = "release";

lazy_static! {
	pub static ref KERNEL_STATE: spin::Mutex<KernelState> = {
		let state = KernelState {
			version: KernelVersion {
				version_str: KERNEL_VERSION.to_string(),
				release_type: RELEASE_TYPE.to_string(),
			},
			serial_log: true,
			hardware: Hardware {
				cpu: Cpu {
					apic: has_apic(),
					aes: aes_detect(),
					cpu_vendor_signature: crate::cpu_vendor_signature(),
				},
			},
		};
		spin::Mutex::new(state)
	};
}

/// TODO: owo
pub struct KernelState {
	/// The first value is the release state and the second is the version string
	pub version: KernelVersion,
	/// This declares whether debug should be logged
	pub serial_log: bool,
	/// The representation of the hardware connected to the kernel
	pub hardware: Hardware,
}
/// Kernel Versioning used to assist in debugging
pub struct KernelVersion {
	/// A semantic versioning
	pub version_str: String,
	/// The release type of the kernel "release" or "debug"
	pub release_type: String,
}
impl fmt::Display for KernelVersion {
	// This trait requires `fmt` with this exact signature.
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Write strictly the first element into the supplied output
		// stream: `f`. Returns `fmt::Result` which indicates whether the
		// operation succeeded or failed. Note that `write!` uses syntax which
		// is very similar to `println!`.
		write!(f, "{} {}", self.version_str, self.release_type)
	}
}
