use alloc::vec::Vec;
use array_init::array_init;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Device {
	data: Vec<u8>,
}
#[allow(dead_code)]
pub struct DeviceDescriptorTable {
	table: [Device; 0x400],
}

lazy_static! {
	pub static ref DEVICE_DESCRIPTOR_TABLE: DeviceDescriptorTable = DeviceDescriptorTable {
		table: array_init(|_i| { Device { data: Vec::new() } }),
	};
}

impl DeviceDescriptorTable {
	pub fn _scan_hw(&mut self) {
		for _i in 0..0x400usize {}
	}
}
