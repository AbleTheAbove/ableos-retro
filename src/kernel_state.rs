use alloc::string::String;
use core::fmt;
/// todo: owo
pub struct KernelState {
    /// The first value is the release state and the second is the version string
    pub version: KernelVersion,
}

pub struct KernelVersion {
    pub version_str: String,
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
