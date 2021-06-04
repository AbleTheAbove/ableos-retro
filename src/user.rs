pub struct UserPermissions {
    admin: bool,
}

pub struct User {
    id: u8,
    permissions: UserPermissions,
}
impl User {}
