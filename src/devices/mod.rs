use alloc::vec::Vec;
use array_init::array_init;
use lazy_static::lazy_static;

#[derive(Clone, Default,)]
pub struct Device {
	data: Vec<u8>,
}

const DEVICE_CAP: usize = 0x400;

#[allow(dead_code)]
pub struct DeviceDescriptorTable {
	table: [Device; DEVICE_CAP],
}

lazy_static! {
	/// Descriptor table holding the devices
	pub static ref DEVICE_DESCRIPTOR_TABLE: DeviceDescriptorTable = DeviceDescriptorTable {
		table: array_init(|_i| Device::default() ),
	};
}

impl DeviceDescriptorTable {
	pub fn _scan_hw(&mut self) {
		for _i in 0..DEVICE_CAP {
			// TODO: Scan for hardware devices and register them in the DDT
		}
	}
}
