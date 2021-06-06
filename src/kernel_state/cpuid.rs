/// Features in ecx register for leaf 1
#[allow(dead_code)]
#[repr(u32)]
pub enum ECXFeatureBitMasks {
	Sse3 = 0x00000001,
	PCLMULQDQ = 0x00000002,
	Dtes64 = 0x00000004,
	Monitor = 0x00000008,
	DSCPL = 0x00000010,
	VirtualMachineExtensions = 0x00000020,
	SaferModeExtensions = 0x00000040,
	EnhancedSpeedStep = 0x00000080,
	ThermalMonitor2 = 0x00000100,
	SupplementalSse3 = 0x00000200,
	L1ContextID = 0x00000400,
	SiliconDebugInterface = 0x00000800,
	FusedMultiplyAdd = 0x00001000,
	CMPXCHG16B = 0x00002000,
	XTPR = 0x00004000,
	PerfmonAndDebugCapable = 0x00008000,
	Reserved16 = 0x00010000,
	ProcessContextIdentifiers = 0x00020000,
	DirectCacheAccess = 0x00040000,
	Sse4p1 = 0x00080000,
	Sse4p2 = 0x00100000,
	X2APIC = 0x00200000,
	MovBigEndian = 0x00400000,
	Popcnt = 0x00800000,
	TscDeadline = 0x01000000,
	AesInstructionSet = 0x02000000,
	XSaveXRestoreXSetbvXGetbv = 0x04000000,
	OSXSave = 0x08000000,
	AdvancedVectorExtensions = 0x10000000,
	F16C = 0x20000000,
	Rdrand = 0x40000000,
	Hypervisor = 0x80000000,
}

/// Features in edx register for leaf 1.
#[allow(dead_code)]
#[repr(u32)]
pub enum EDXFeatureBitMasks {
	FPU = 0x00000001,
	VirtualModeExtensions = 0x00000002,
	DebuggingExtensions = 0x00000004,
	PageSizeExtensions = 0x0000008,
	TimeStampCounter = 0x00000010,
	ModelSpecificRegisters = 0x00000020,
	PhysicalAddressExtension = 0x00000040,
	MachineCheckException = 0x0000080,
	CMPXCHG8 = 0x00000100,
	APIC = 0x00000200,
	Reserved10 = 0x00000400,
	SysenterSysexit = 0x0000800,
	MemoryTypeRangeRegisters = 0x00001000,
	PageGlobalEnable = 0x00002000,
	MachineCheckArchitecture = 0x00004000,
	CmovFcmov = 0x00008000,
	PageAttributeTable = 0x00010000,
	PageSizeExtension36Bit = 0x00020000,
	ProcessorSerialNumber = 0x00040000,
	CLFLUSHInstruction = 0x00080000,
	Reserved20 = 0x00100000,
	DebugStore = 0x00200000,
	ACPI = 0x00400000,
	Mmx = 0x00800000,
	FXSaveFXRestore = 0x01000000,
	Sse = 0x02000000,
	Sse2 = 0x04000000,
	SelfSnoop = 0x08000000,
	HyperThreading = 0x10000000,
	AutoLimitsTemp = 0x20000000,
	Ia64Emulator = 0x40000000,
	PendingBreakEnable = 0x80000000,
}

/// Sets 
pub fn cpuid(mut eax_: u32, mut ebx_: u32, mut ecx_: u32, mut edx_: u32) -> (u32, u32, u32, u32) {
	unsafe {
		asm![
			"xchg eax, {:e}",
			"xchg ebx, {:e}",
			"xchg ecx, {:e}",
			"xchg edx, {:e}",
			"cpuid",
			"xchg {:e}, eax",
			"xchg {:e}, ebx",
			"xchg {:e}, ecx",
			"xchg {:e}, edx",
			in(reg) eax_,
			in(reg) ebx_,
			in(reg) ecx_,
			in(reg) edx_,
			out(reg) eax_,
			out(reg) ebx_,
			out(reg) ecx_,
			out(reg) edx_
		]
	}
	(eax_, ebx_, ecx_, edx_)
}
