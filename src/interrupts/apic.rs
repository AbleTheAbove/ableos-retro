use crate::kernel_state::cpuid::EDXFeatureMasks;
use raw_cpuid::{cpuid, CpuIdResult};
use x86::msr::{rdmsr};

/// Returns true if apic is available on this processor.
/// Sets register eax to 1, then queries cpuid
pub fn has_apic() -> bool {
	// Note: this is not where it sets eax to 1.
	// This is.
	let result = cpuid!(1, 0);
	(result.edx & EDXFeatureMasks::APIC as u32) > 0
}

const IA32_APIC_BASE_MSR: u32 = 0x1B;
const IA32_APIC_BASE_MSR_BSP: u32 = 0x100; // Processor is a BSP
const IA32_APIC_BASE_MSR_ENABLE: u32 = 0x800;

