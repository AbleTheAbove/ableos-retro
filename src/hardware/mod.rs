pub mod cpu;
pub mod encrypt;
pub mod mouse;

#[derive(Debug)]
pub struct Hardware {
	pub cpu: cpu::Cpu,
}
