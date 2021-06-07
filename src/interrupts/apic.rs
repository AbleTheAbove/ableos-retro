use crate::kernel_state::cpuid::EDXFeatureMasks;
use raw_cpuid::cpuid;

/// Returns whether or not apic is available on this processor.
/// Sets register eax to 1, then queries cpuid
pub fn has_apic() -> bool {
	// Note: this is not where it sets eax to 1.
	// This is.
	let result = cpuid!(1, 0);
	(result.edx & EDXFeatureMasks::Apic as u32) > 0
}
