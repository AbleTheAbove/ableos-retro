use crate::{String, Vec};

pub struct UserPermissions {
	_admin: bool,
	_own_devices: bool,
}

pub struct User {
	_id: u8,
	_name: String,
	_permissions: UserPermissions,
	// Kept as references to owned devices
	_owned_devices: Vec<u8>,
}
impl User {}
