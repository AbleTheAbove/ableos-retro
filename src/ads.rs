pub enum Ad {
	AbleScript,
}
impl Ad {
	fn display() {
		GRAPHICS_RAW.draw_character(
			// TODO: Get length of character size and then do math
			0,
			1,
			'b',
			Color16::LightBlue,
		)
	}
}
