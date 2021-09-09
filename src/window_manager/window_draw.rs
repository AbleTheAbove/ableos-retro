//! this module draws windows

use super::{
	// Comented out because unused but will be used in the future
	//terminal,
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
		offset: (offset.0, offset.1),
		size: size,
	};
	for y in 0..window.size.1 {
		GRAPHICS_RAW.draw_line(
			(
				0 + window.offset.0,
				window.size.1 as isize + window.offset.1 - y as isize,
			),
			(
				window.size.0 as isize + window.offset.0,
				window.size.1 as isize + window.offset.1 - y as isize,
			),
			Color16::Black,
		);
	}

	// Left line
	GRAPHICS_RAW.draw_line(
		(0 + window.offset.0, 0 + window.offset.1),
		(
			0 + window.offset.0,
			window.size.1 as isize + window.offset.1,
		),
		WINDOW_BORDER_COLOR,
	);

	// Lowest line
	GRAPHICS_RAW.draw_line(
		(
			0 + window.offset.0,
			window.size.1 as isize + window.offset.1,
		),
		(
			window.size.0 as isize + window.offset.0,
			window.size.1 as isize + window.offset.1,
		),
		WINDOW_BORDER_COLOR,
	);

	//right most line
	GRAPHICS_RAW.draw_line(
		(
			window.size.0 as isize + window.offset.0,
			window.size.1 as isize + window.offset.1,
		),
		(window.size.0 as isize + window.offset.0, window.offset.1),
		WINDOW_BORDER_COLOR,
	);

	// A simple window decorator that I think should be fully implemented
	for y in 0..20 {
		for x in 1..window.size.0 {
			GRAPHICS_RAW.set_pixel(
				x + window.offset.0 as usize,
				y + window.offset.1 as usize,
				WINDOW_DECORATOR_COLOR,
			);
		}
	}

	let title_width = window.title.len() * 8;
	for (offset, character) in window.title.chars().enumerate() {
		GRAPHICS_RAW.draw_character(
			// TODO: Get length of character size and then do math
			(window.offset.0 as usize + ((window.size.0 - title_width) / 2)) as usize + offset * 8,
			(6 + window.offset.1) as usize,
			character,
			WINDOW_DECORATOR_TEXT_COLOR,
		)
	}
}

/// todo: pwees write docs
pub fn logo(offset: (isize, isize)) {
	{
		let a_color = Color16::Pink;
		// Left side of the A
		GRAPHICS_RAW.draw_line(
			(offset.0 + 20, offset.1),
			(offset.0 + 10, offset.1 + 20),
			a_color,
		);

		// Right side of the A
		GRAPHICS_RAW.draw_line(
			(offset.0 + 20, offset.1),
			(offset.0 + 30, offset.1 + 20),
			a_color,
		);

		// Center connector for the A
		GRAPHICS_RAW.draw_line(
			(offset.0 + 10, offset.1 + 10),
			(offset.0 + 30, offset.1 + 10),
			a_color,
		);
	}

	let offset_c = (offset.0 + 5, offset.1 - 15);
	let crown_color = Color16::Yellow;
	GRAPHICS_RAW.draw_line(
		(offset_c.0, offset_c.1),
		(offset_c.0 + 10 / 2, offset_c.1 + 20 / 2),
		crown_color,
	);

	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 10 / 2, offset_c.1 + 20 / 2),
		(offset_c.0 + 50 / 2, offset_c.1 + 20 / 2),
		crown_color,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 50 / 2, offset_c.1 + 20 / 2),
		(offset_c.0 + 60 / 2, offset_c.1),
		crown_color,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 60 / 2, offset_c.1),
		(offset_c.0 + 40 / 2, offset_c.1 + 10 / 2),
		crown_color,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 40 / 2, offset_c.1 + 10 / 2),
		(offset_c.0 + 30 / 2, offset_c.1),
		crown_color,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 30 / 2, offset_c.1),
		(offset_c.0 + 20 / 2, offset_c.1 + 10 / 2),
		crown_color,
	);
	GRAPHICS_RAW.draw_line(
		(offset_c.0 + 20 / 2, offset_c.1 + 10 / 2),
		(offset_c.0 + 0 / 2, offset_c.1),
		crown_color,
	);
}
