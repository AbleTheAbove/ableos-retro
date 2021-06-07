use crate::serial;
use crate::{
	// print,
	serial_print,
	serial_println, // vga_buffer,
};
use core::fmt::Arguments;
use lliw::Fg;

/// print an error message to serial, this will append a newline to the end

// Reason; this will definitely get used in the future
#[allow(unused_macros)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => (
        $crate::logger::slog($crate::logger::LogLevel::Error, format_args!($($arg)+))
    )
}

/// print a debug message to serial, this will append a newline to the end

#[cfg(not(test))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => (
    $crate::logger::slog($crate::logger::LogLevel::Debug, format_args!($($arg)+)))
}
#[cfg(test)]
macro_rules! debug {}
/// print a success message to serial, this will append a newline to the end

#[macro_export]
macro_rules! success {
    ($($arg:tt)+) => (
        $crate::logger::slog($crate::logger::LogLevel::Success, format_args!($($arg)+))
    )
}

/// print a info message to serial, this will append a newline to the end

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => (
        $crate::logger::slog($crate::logger::LogLevel::Info, format_args!($($arg)+))
    )
}

// use crate::util::{term_reset, term_set};
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
	// print!("[");

	match level {
		LogLevel::Error => error_log(),
		LogLevel::Debug => debug_log(),
		LogLevel::Info => info_log(),
		LogLevel::Success => success_log(),
	}

	// print!("] ");
}

fn error_log() {
	// term_set(vga_buffer::Color::Red);
	// print!("Error");
	// term_reset();
}
fn success_log() {
	// term_set(vga_buffer::Color::Green);
	// print!("Success");
	// term_reset();
}
fn info_log() {
	// print!("Info");
}
fn debug_log() {
	// term_set(vga_buffer::Color::Yellow);
	// print!("Debug");
	// term_reset();
}

/// print a log message to the serial port, a newline will be appended.
/// This should not be used, use the macros named by log levels instead.
pub fn slog(level: LogLevel, message_args: Arguments) {
	if crate::kernel_state::KERNEL_STATE.lock().serial_log {
		serial_print!("[");

		match level {
			LogLevel::Error => error_slog(),
			LogLevel::Debug => debug_slog(),
			LogLevel::Info => info_slog(),
			LogLevel::Success => success_slog(),
		}
		serial_print!("] ");
		serial::_print(message_args);
		serial_println!();
	}
}
fn error_slog() {
	serial_print!("{}Error{}", Fg::Red, Fg::Reset);
}
fn success_slog() {
	serial_print!("{}Success{}", Fg::Green, Fg::Reset);
}
fn info_slog() {
	serial_print!("Info");
}
fn debug_slog() {
	serial_print!("{}Debug{}", Fg::Yellow, Fg::Reset);
}
