use crate::GRAPHICS_RAW;
use vga::colors::Color16;
use vga::writers::GraphicsWriter;

pub enum AbleAd {}

pub struct Ad {
	_ad_type: AbleAd,
}
impl Ad {
	pub fn display() {
		for (index, char) in "https://ablecorp.us".chars().enumerate() {
			GRAPHICS_RAW.draw_character(
				// TODO: Get length of character size and then do math
				index * 8,
				0,
				char,
				Color16::LightBlue,
			)
		}
	}
}
