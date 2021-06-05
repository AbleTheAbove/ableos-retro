struct TextManipulationPermissions {
    copy: bool,
    paste: bool,
    cut: bool,
}
struct WindowMovementPermissions {
    movement: bool,
    minimize: bool,
}
struct WindowPermission {
    interact: bool,
    text_manipulation: TextManipulationPermissions,
    movement: WindowMovementPermissions,
}
