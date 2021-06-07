#![no_std]
#![warn(missing_docs)]

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The internal data of the shell
struct Shell {}

/// The publically avalible shell runner
pub fn shell() {
	// Represent the shell data in an accessable way
	let mut shell = Shell {};
	// the Prompt
	// TODO: allow this to be configured
	let prompt = "~>";
}
