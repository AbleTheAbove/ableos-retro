use alloc::vec::Vec;
use array_init::array_init;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Device {
    data: Vec<u8>,
}

pub struct DeviceDescriptorTable {
    table: [Device; 0x400],
}

lazy_static! {
    pub static ref DEVICE_DESCRIPTOR_TABLE: DeviceDescriptorTable = DeviceDescriptorTable {
        table: array_init(|i| { Device { data: Vec::new() } }),
    };
}

