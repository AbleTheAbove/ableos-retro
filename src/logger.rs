use crate::{print, serial_print, vga};
use lliw::Fg;
/// Denote log levels
pub enum LogLevel {
    /// The Error log level to be used with errors
    Error,
    /// Used by developers mostly
    Debug,
    /// The Info log level should be used for all info like "Loaded this or that"
    Info,
    /// Used for successful things
    Success,
}
/// print a log prefix to the framebuffer
pub fn log(level: LogLevel) {
    print!("[");

    match level {
        LogLevel::Error => error_log(),
        LogLevel::Debug => debug_log(),
        LogLevel::Info => info_log(),
        LogLevel::Success => success_log(),
    }

    print!("] ");
}

fn error_log() {
    vga::WRITER
        .lock()
        .set_color(vga::Color::Red, vga::Color::Black);
    print!("Error");
    vga::WRITER
        .lock()
        .set_color(vga::Color::White, vga::Color::Black);
}
fn success_log() {
    vga::WRITER
        .lock()
        .set_color(vga::Color::Green, vga::Color::Black);
    print!("Success");
    vga::WRITER
        .lock()
        .set_color(vga::Color::White, vga::Color::Black);
}
fn info_log() {
    print!("Info");
}
fn debug_log() {
    vga::WRITER
        .lock()
        .set_color(vga::Color::Yellow, vga::Color::Black);
    print!("Debug");
    vga::WRITER
        .lock()
        .set_color(vga::Color::White, vga::Color::Black);
}

/// print a log prefix to the serial port
pub fn slog(level: LogLevel) {
    serial_print!("[");

    match level {
        LogLevel::Error => error_slog(),
        LogLevel::Debug => debug_slog(),
        LogLevel::Info => info_slog(),
        LogLevel::Success => success_slog(),
    }

    serial_print!("] ");
}
fn error_slog() {
    serial_print!("{}Error{}", Fg::Red, Fg::Reset);
}
fn success_slog() {
    serial_print!("{}Success{}", Fg::Green, Fg::Reset);
}
fn info_slog() {
    print!("Info");
}
fn debug_slog() {
    serial_print!("{}Debug{}", Fg::Yellow, Fg::Reset);
}
