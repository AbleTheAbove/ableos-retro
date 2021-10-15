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
pub const RELEASE_TYPE: KernelReleaseType = KernelReleaseType::Debug;
#[cfg(not(debug_assertions))]
/// A constant to check if the kernel is in release mode
pub const RELEASE_TYPE: KernelReleaseType = KernelReleaseType::Release;

lazy_static! {
	pub static ref KERNEL_STATE: spin::Mutex<KernelState> = {
		let state = KernelState {
			version: KernelVersion {
				version_str: KERNEL_VERSION.to_string(),
				release_type: RELEASE_TYPE,
			},
			serial_log: true,
			hardware: Hardware {
				cpu: Cpu {
					apic: has_apic(),
					aes: aes_detect(),
					cpu_vendor_signature: crate::cpu_vendor_signature(),
				},
			},
			/// Temporary view
			task_menu: false,
		};
		spin::Mutex::new(state)
	};
}

/// TODO: owo
#[derive(Debug)]
pub struct KernelState {
	/// The first value is the release state and the second is the version string
	pub version: KernelVersion,
	/// This declares whether debug should be logged
	pub serial_log: bool,
	/// The representation of the hardware connected to the kernel
	pub hardware: Hardware,
	pub task_menu: bool,
}

impl fmt::Display for KernelState {
	// This trait requires `fmt` with this exact signature.
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Write strictly the first element into the supplied output
		// stream: `f`. Returns `fmt::Result` which indicates whether the
		// operation succeeded or failed. Note that `write!` uses syntax which
		// is very similar to `println!`.
		write!(f, "{}\n{}", self.version, self.serial_log)
	}
}

#[derive(Debug, Clone, Eq, PartialEq,)]
pub enum KernelReleaseType {
	Debug,
	Release,
}

impl fmt::Display for KernelReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KernelReleaseType::Debug => write!(f, "{}", "debug"),
            KernelReleaseType::Release => write!(f, "{}", "release"),
        }
    }
}

/// Kernel Versioning used to assist in debugging
#[derive(Debug)]
pub struct KernelVersion {
	/// A semantic versioning
	pub version_str: String,
	/// The release type of the kernel "release" or "debug"
	pub release_type: KernelReleaseType,
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
