use crate::Vec;

// use alloc::vec::Vec;
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
	pub static ref GRAPHICS_RAW: Graphics640x480x16 = {
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
pub const WINDOW_BORDER_COLOR: Color16 = Color16::LightBlue;
/// The window decorator color
pub const WINDOW_DECORATOR_COLOR: Color16 = Color16::LightBlue;
/// Window decorator Text color
pub const WINDOW_DECORATOR_TEXT_COLOR: Color16 = Color16::Black;

/// A struct for holding windows
pub struct WindowHolder<'a>(pub Mutex<Vec<&'a Window<'a>>>);
/// Window struct
pub struct Window<'holder> {
	/// IDEA: Convert to String
	/// The title of a window
	pub title: &'holder str,
	/// Window offset
	pub offset: Offset,
	/// Size of a window
	pub size: Size,
}

impl<'w> Window<'w> {

	fn draw(&self) {
		let (window_width, window_height) = self.size;
		let (offset_x, offset_y) = self.offset;
		for y in 0..window_height {
			GRAPHICS_RAW.draw_line(
				(
					0 + offset_x,
					window_height as isize + offset_y - y as isize,
				),
				(
					window_width as isize + offset_x,
					window_height as isize + offset_y - y as isize,
				),
				Color16::Black,
			);
		}

		// Left line
		GRAPHICS_RAW.draw_line(
			(0 + offset_x, 0 + offset_y),
			(
				0 + offset_x,
				window_height as isize + offset_y,
			),
			WINDOW_BORDER_COLOR,
		);

		// Lowest line
		GRAPHICS_RAW.draw_line(
			(
				0 + offset_x,
				window_height as isize + offset_y,
			),
			(
				window_width as isize + offset_x,
				window_height as isize + offset_y,
			),
			WINDOW_BORDER_COLOR,
		);

		//right most line
		GRAPHICS_RAW.draw_line(
			(
				window_width as isize + offset_x,
				window_height as isize + offset_y,
			),
			(window_width as isize + offset_x, offset_y),
			WINDOW_BORDER_COLOR,
		);

		// A simple window decorator that I think should be fully implemented
		for y in 0..20 {
			for x in 1..window_width {
				GRAPHICS_RAW.set_pixel(
					x + offset_x as usize,
					y + offset_y as usize,
					WINDOW_DECORATOR_COLOR,
				);
			}
		}

		let title_width = self.title.len() * 8;
		for (offset, character) in self.title.chars().enumerate() {
			GRAPHICS_RAW.draw_character(
				// TODO: Get length of character size and then do math
				(offset_x as usize + ((window_width - title_width) / 2)) as usize + offset * 8,
				(6 + offset_y) as usize,
				character,
				WINDOW_DECORATOR_TEXT_COLOR,
			)
		}
	}

}
