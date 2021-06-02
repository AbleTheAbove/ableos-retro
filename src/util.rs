//! A simple utility module to reduce repeated code

use crate::{println, vga};

/// ASCII art banner
const BANNER: &str = include_str!("../root/banner.txt");
/// Prints the ascii banner
pub fn banner() {
    term_set(vga::Color::LightBlue);
    println!("{}", BANNER);
    term_reset();
    println!("ableOS version: {}", crate::KERNEL_VERSION);
    println!("================================================================================");
}

// TODO: move term_* functions to a submodule when there is enough of a "Terminal" to justify it

/// Reset the terminal styles
pub fn term_reset() {
    vga::WRITER
        .lock()
        .set_color(vga::Color::White, vga::Color::Black);
}
/// set the forground color
pub fn term_set(color: vga::Color) {
    vga::WRITER.lock().set_color(color, vga::Color::Black);
}
