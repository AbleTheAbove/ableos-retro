use alloc::{string::String, vec::Vec};

pub struct UserPermissions {
	admin: bool,
	own_devices: bool,
}

pub struct User {
	id: u8,
	name: String,
	permissions: UserPermissions,
	// Kept as references to owned devices
	owned_devices: Vec<u8>,
}
impl User {}
