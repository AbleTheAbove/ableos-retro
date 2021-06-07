pub struct Cpu {
	pub apic: bool,
	pub cpu_vendor_signature: [u8; 12],
}

pub struct Hardware {
	pub cpu: Cpu,
}
