pub mod cpu;
pub mod encrypt;
pub mod mouse;

pub struct Hardware {
	pub cpu: cpu::Cpu,
}
