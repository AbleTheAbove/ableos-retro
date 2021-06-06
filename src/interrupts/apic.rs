use crate::kernel_state::cpuid::{cpuid, EDXFeatureBitMasks};

/// Returns whether or not apic is available on this processor.
/// Sets register eax to 1, then queries cpuid
pub fn has_apic() -> bool {
	// Note: this is not where it sets eax to 1.
	let mut eax: u32 = 0;
	let mut edx: u32 = 0;
	// This is.
	cpuid(1, &mut eax, &mut edx);
	(edx & EDXFeatureBitMasks::HasAPIC as u32) > 0
}
