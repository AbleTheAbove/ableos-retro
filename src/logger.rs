use crate::{print, serial_print, vga_buffer};
use lliw::Fg;

use crate::util::{term_reset, term_set};
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
    term_set(vga_buffer::Color::Red);
    print!("Error");
    term_reset();
}
fn success_log() {
    term_set(vga_buffer::Color::Green);
    print!("Success");
    term_reset();
}
fn info_log() {
    print!("Info");
}
fn debug_log() {
    term_set(vga_buffer::Color::Yellow);
    print!("Debug");
    term_reset();
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
