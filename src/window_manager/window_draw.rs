//! this module draws windows

use super::{
	// Comented out because unused but will be used in the future
	Window,
	GRAPHICS_RAW,
	WINDOW_BORDER_COLOR,
	WINDOW_DECORATOR_COLOR,
	WINDOW_DECORATOR_TEXT_COLOR,
};

pub use vga::{colors::Color16, writers::GraphicsWriter};

// BUG: drawing bigger than the screen size causes the buffer to wrap around
/// todo: pwees write docs
pub fn _windows(_id: u8, offset: (isize, isize)) {
	let win_title = "hi";
	let size: (usize, usize) = (0, 0);
	/*
	match id {
		0 => {
			win_title = "AbleOS Terminal".to_string();
			//size = (639, 479);
			size = (639, 479);
		}
		_ => {
			win_title = format!("AbleOS Window [ID: {}]", id);
			size = (200, 100);
		}
	}
	*/
	let window = Window {
		title: &win_title,
		offset,
		size,
	};

	window.draw()
}

/// todo: pwees write docs
pub fn logo(offset: (isize, isize)) {
	{
		const A_COLOR: Color16 = Color16::Pink;
		// Left side of the A
		GRAPHICS_RAW.draw_line(
			(offset.0 + 20, offset.1),
			(offset.0 + 10, offset.1 + 20),
			A_COLOR,
		);

		// Right side of the A
		GRAPHICS_RAW.draw_line(
			(offset.0 + 20, offset.1),
			(offset.0 + 30, offset.1 + 20),
			A_COLOR,
		);

		// Center connector for the A
		GRAPHICS_RAW.draw_line(
			(offset.0 + 10, offset.1 + 10),
			(offset.0 + 30, offset.1 + 10),
			A_COLOR,
		);
	}

	let offset_c = (offset.0 + 5, offset.1 - 15);
	const CROWN_COLOR: Color16 = Color16::Yellow;
	GRAPHICS_RAW.draw_line(
		(offset_c.0, offset_c.1),
		(offset_c.0 + 10 / 2, offset_c.1 + 20 / 2),
		CROWN_COLOR,
	);

	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 10 / 2, offset_c.1 + 20 / 2),
		(offset_c.0 + 50 / 2, offset_c.1 + 20 / 2),
		CROWN_COLOR,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 50 / 2, offset_c.1 + 20 / 2),
		(offset_c.0 + 60 / 2, offset_c.1),
		CROWN_COLOR,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 60 / 2, offset_c.1),
		(offset_c.0 + 40 / 2, offset_c.1 + 10 / 2),
		CROWN_COLOR,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 40 / 2, offset_c.1 + 10 / 2),
		(offset_c.0 + 30 / 2, offset_c.1),
		CROWN_COLOR,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 30 / 2, offset_c.1),
		(offset_c.0 + 20 / 2, offset_c.1 + 10 / 2),
		CROWN_COLOR,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 20 / 2, offset_c.1 + 10 / 2),
		(offset_c.0 + 0 / 2, offset_c.1),
		CROWN_COLOR,
	);
}
