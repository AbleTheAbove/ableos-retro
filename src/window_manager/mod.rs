use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;
pub use vga::{
	colors::Color16,
	writers::{Graphics640x480x16, GraphicsWriter},
};
/// Holds references to all windows
// pub struct WindowManager {
//     vga_buff: Graphics640x480x16,
// }
// impl WindowManager {}
//mod terminal;
pub mod window_draw;

type Size = (usize, usize);
type Offset = (isize, isize);

/// Initialize the window manager
pub fn init() {
	let mut seven = 0;
	let mut nine = 0;
	for x in 0..10 {
		window_draw::_windows(x, (seven, nine));
		seven += 40;
		nine += 40;
	}
	window_draw::logo((440, 420));
}

lazy_static! {
	/// Static reference to the frame buffer
	pub static ref GRAPHICS: Graphics640x480x16 = {
		let mode = Graphics640x480x16::new();
		mode.set_mode();
		mode.clear_screen(Color16::Black);
		mode
	};
}

lazy_static! {
	/// All the windows
	pub static ref WINDOWS: WindowHolder<'static> = WindowHolder(Mutex::new(Vec::new()));
}
/// The color of window border colors
pub static WINDOW_BORDER_COLOR: Color16 = Color16::LightBlue;
/// The window decorator color
pub static WINDOW_DECORATOR_COLOR: Color16 = Color16::LightBlue;
/// Window decorator Text color
pub static WINDOW_DECORATOR_TEXT_COLOR: Color16 = Color16::Black;

/// A struct for holding windows
pub struct WindowHolder<'a>(pub Mutex<Vec<&'a Window<'a>>>);
/// Window struct
pub struct Window<'a> {
	/// IDEA: Convert to String
	/// The title of a window
	pub title: &'a str,
	/// Window offset
	pub offset: Offset,
	/// Size of a window
	pub size: Size,
}
