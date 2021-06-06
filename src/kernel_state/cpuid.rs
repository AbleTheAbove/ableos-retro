/// Feature bit masks for EDX register.
/// Call cpuid with set: 1 to
#[allow(dead_code)]
#[repr(u32)]
pub enum EDXFeatureBitMasks {
	HasFPU = 0x00000001,
	HasVirtualModeExtensions = 0x00000002,
	HasDebuggingExtensions = 0x00000004,
	HasPageSizeExtensions = 0x0000008,
	HasTimeStampCounter = 0x00000010,
	HasModelSpecificRegisters = 0x00000020,
	HasPhysicalAddressExtension = 0x00000040,
	HasMachineCheckException = 0x0000080,
	HasCMPXCHG8 = 0x00000100,
	HasAPIC = 0x00000200,
	HasReserved10 = 0x00000400,
	HasSysenterSysexit = 0x0000800,
	HasMemoryTypeRangeRegisters = 0x00001000,
	HasPageGlobalEnable = 0x00002000,
	HasMachineCheckArchitecture = 0x00004000,
	HasCmovFcmov = 0x00008000,
	HasPageAttributeTable = 0x00010000,
	HasPageSizeExtension36Bit = 0x00020000,
	HasProcessorSerialNumber = 0x00040000,
	HasCLFLUSHInstruction = 0x00080000,
	HasReserved20 = 0x00100000,
	HasDebugStore = 0x00200000,
	HasACPI = 0x00400000,
	HasMmx = 0x00800000,
	HasFxsaveFxrestore = 0x01000000,
	HasSse = 0x02000000,
	HasSse2 = 0x04000000,
	HasSelfSnoop = 0x08000000,
	HasHyperThreading = 0x10000000,
	HasAutoLimitsTemp = 0x20000000,
	HasIa64Emulator = 0x40000000,
	HasPendingBreakEnable = 0x80000000,
}

/// Takes a u32 and two references to u32s.
/// Sets eax_ and edx_ to the respective registers.
pub fn cpuid(set: u32, eax_: &mut u32, edx_: &mut u32) {
	unsafe {
		asm![
			"mov eax, {:e}",
			"cpuid",
			"mov {:e}, eax",
			"mov {:e}, edx",
			in(reg) set,
			out(reg) *eax_,
			out(reg) *edx_
		]
	}
}