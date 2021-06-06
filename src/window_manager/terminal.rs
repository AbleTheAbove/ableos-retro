use super::GRAPHICS;
use vga::{colors::Color16, writers::GraphicsWriter};

// TODO: Rework this
#[allow(dead_code)]
pub fn draw_terminal(terminal_offset: (isize, isize)) {
	let text_line = [('b', Color16::Red); 80];
	let mut text_buffer = [text_line; 50];
	text_buffer[0][1].0 = 'a';
	text_buffer[0][1].1 = Color16::White;

	let mut line2 = 0;
	let mut offset = 0;

	for line in text_buffer.iter() {
		for character in line.iter() {
			GRAPHICS.draw_character(
				(terminal_offset.0 + (offset * 8)) as usize,
				((terminal_offset.1 + 20) + line2 * 8) as usize,
				character.0,
				character.1,
			);
			offset += 1;
		}
		line2 += 1;
	}
}
