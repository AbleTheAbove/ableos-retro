use crate::hardware::cpu::EDXFeatureMasks;
use raw_cpuid::cpuid;
//use x86::msr::rdmsr;

/// Returns true if apic is available on this processor.
/// Sets register eax to 1, then queries cpuid
pub fn has_apic() -> bool {
	// Note: this is not where it sets eax to 1.
	// This is.
	let result = cpuid!(1, 0);
	(result.edx & EDXFeatureMasks::Apic as u32) > 0
}

const _IA32_APIC_BASE_MSR: u32 = 0x1B;
const _IA32_APIC_BASE_MSR_BSP: u32 = 0x100; // Processor is a BSP
const _IA32_APIC_BASE_MSR_ENABLE: u32 = 0x800;

// 'e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128'
// 'e-m:e-i64:64-f80:128-n8:16:32:64-S128'
