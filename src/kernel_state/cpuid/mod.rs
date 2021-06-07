/// Responses identification request with eax 0
#[allow(dead_code)]
#[repr(u128)]
pub enum CpuVendorSignatures {
	/* AMD:     "AuthenticAMD" */
	AMD = 0x6874754169746e65444d4163,
	/* CENTAUR: "CentaurHauls" */
	CENTAUR = 0x746e654348727561736c7561,
	/* CYRIX:   "CyrixInstead" */
	CYRIX = 0x69727943736e497864616574,
	/* INTEL:   "GenuineIntel" */
	INTEL = 0x756e654749656e696c65746e,
	/* TM1:     "TransmetaCPU" */
	TM1 = 0x6e61725474656d7355504361,
	/* TM2:     "GenuineTMx86" */
	TM2 = 0x756e654754656e693638784d,
	/* NSC:     "Geode by NSC" */
	NSC = 0x646f654743534e2079622065,
	/* NEXGEN:  "NexGenDriven" */
	NEXGEN = 0x4778654e72446e656e657669,
	/* RISE:    "RiseRiseRise" */
	RISE = 0x657369526573695265736952,
	/* SIS:     "SiS SiS SiS " */
	SIS = 0x205369532053695320536953,
	/* UMC:     "UMC UMC UMC " */
	UMC = 0x20434d5520434d5520434d55,
	/* VIA:     "VIA VIA VIA " */
	VIA = 0x204149562041495620414956,
	/* VORTEX:  "Vortex86 SoC" */
	VORTEX = 0x74726f5636387865436f5320,
}

/// Features in ecx register for eax = 1
#[allow(dead_code)]
#[repr(u32)]
pub enum ECXFeatureMasks {
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

/// Features in edx register for eax = 1
#[allow(dead_code)]
#[repr(u32)]
pub enum EDXFeatureMasks {
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

/// Features in ebx for eax=7 ecx=0
#[allow(dead_code)]
#[repr(u32)]
pub enum EBXExtendedFeatureMasks {
	FsGsBaseAccess = 0x00000001,
	Ia32TscAdjust = 0x00000002,
	SoftwareGuardExt = 0x00000004,
	BitManipulationInstructionSet1 = 0x00000008,
	HardwareLockElision = 0x00000010,
	AdvancedVectorExtensions2 = 0x00000020,
	FdpExcptnOnly = 0x00000040,
	SupervisorModeExecPrevention = 0x00000080,
	BitManipulationInstructionSet2 = 0x00000100,
	EnhancedMovSBStoSB = 0x00000200,
	InvpcidInstruction = 0x00000400,
	RestrictedTransactionalMemory = 0x00000800,
	PlatformServiceQualityMonitoring = 0x00001000,
	FpuCSFpuDS = 0x00002000,
	MemoryProtextionExtensions = 0x00004000,
	PlatformServiceQualityEnforcement = 0x00008000,
	Avx512Foundation = 0x00010000,
	Avx512DQ = 0x00020000,
	RdseedInstruction = 0x00040000,
	AdxInstruction = 0x00080000,
	SupervisorModeAccessPrevention = 0x00100000,
	Avx512IFMA = 0x00200000,
	PcommitInstruction = 0x00400000,
	ClflushoptInstruction = 0x00800000,
	ClwbInstruction = 0x01000000,
	IntelProcessorTrace = 0x02000000,
	Avx512Prefetch = 0x04000000,
	Avx512ExponentialReciprocalInstructions = 0x08000000,
	Avx512ConflictDetection = 0x10000000,
	SHA = 0x20000000,
	Avx512BW = 0x40000000,
	Avx512VL = 0x80000000,
}

/// Features in ecx for eax=7 ecx=0
#[allow(dead_code)]
#[repr(u32)]
pub enum ECXExtendedFeatureMasks {
	PREFTCHWT1 = 0x00000001,
	AVX512VBMI = 0x00000002,
	PKU = 0x00000004,
	OSPKE = 0x00000010,
	AVX512VBMI2 = 0x00000040,
	SHSTK = 0x00000080,
	GFNI = 0x00000100,
	VAES = 0x00000200,
	VPCLMULQDQ = 0x00000400,
	AVX512VNNI = 0x00000800,
	AVX512BITALG = 0x00001000,
	AVX512VPOPCNTDQ = 0x00004000,
	RDPID = 0x00400000,
}

/// Features in edx for eax=7 ecx=0
#[allow(dead_code)]
#[repr(u32)]
pub enum EDXExtendedFeatureMasks {
	AVX5124VNNIW = 0x00000004,
	AVX5124FMAPS = 0x00000008,
	IBT = 0x00100000,
}

/// Features in %eax for eax=13 ecx=1
#[allow(dead_code)]
#[repr(u32)]
pub enum EAXExtendedFeatureMasks {
	XSAVEOPT = 0x00000001,
	XSAVEC = 0x00000002,
	XSAVES = 0x00000008,
}

/// Features in ecx for eax=0x80000001
#[allow(dead_code)]
#[repr(u32)]
pub enum ECXSuperFeautureMasks {
	LAHF_LM = 0x00000001,
	ABM = 0x00000020,
	SSE4a = 0x00000040,
	PRFCHW = 0x00000100,
	XOP = 0x00000800,
	LWP = 0x00008000,
	FMA4 = 0x00010000,
	TBM = 0x00200000,
	MWAITX = 0x20000000,
}

/// Features in edx for leaf 0x80000001
#[allow(dead_code)]
#[repr(u32)]
pub enum EDXSuperFeatureMasks {
	MMXEXT = 0x00400000,
	LM = 0x20000000,
	_3DNOWP = 0x40000000,
	_3DNOW = 0x80000000,
}

/// Features in ebx for leaf 0x80000001
#[allow(dead_code)]
#[repr(u32)]
pub enum EBXSuperFeatureMasks {
	CLZERO = 0x00000001,
}

/// Returns what eax, ebx, ecx, and edx get set to when sending
/// cpuid instruction to processor. Sets these initially to given
/// input parameters, then resets to previous values.
pub fn cpuid(mut eax_: u32, mut ebx_: u32, mut ecx_: u32, mut edx_: u32) -> [u32; 4] {
	unsafe {
		asm![
			"xchg {:e}, eax",
			"xchg {:e}, ebx",
			"xchg {:e}, ecx",
			"xchg {:e}, edx",
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
		];
	}
	[eax_, ebx_, ecx_, edx_]
}

pub fn cpu_vendor_signature() -> [u8; 12] {
	let [a, b, c, d] = cpuid(0, 1, 1, 1);
	unsafe { core::mem::transmute::<[u32; 3], [u8; 12]>([b, c, d]) }
}