use crate::interrupts::pic::PICS;
use crate::interrupts::InterruptIndex;
use crate::ps2_mouse::{Mouse, MouseState};
use crate::window_manager::GRAPHICS;
use crate::{info, success};
use lazy_static::lazy_static;
use spin::Mutex;
use vga::colors::Color16;
use vga::writers::GraphicsWriter;
use x86_64::instructions::port::PortReadOnly;
use x86_64::structures::idt::InterruptStackFrame;

const CURSOR_COLOR: Color16 = Color16::Cyan;

const MOUSE_MAX_X: u16 = 638;
const MOUSE_MAX_Y: u16 = 478;

lazy_static! {
	pub static ref _MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
	pub static ref MOUSE: Mutex<OnScreenMouse> = Mutex::new(OnScreenMouse::default());
}

#[derive(Default)]
pub struct OnScreenMouse {
	// absolute position on screen
	x: u16,
	// absolute position on screen
	y: u16,
}

impl OnScreenMouse {
	pub fn get_x(&self) -> u16 {
		return self.x;
	}

	pub fn get_y(&self) -> u16 {
		return self.y;
	}

	pub fn change_x(&mut self, delta_x: i16) {
		if delta_x.is_negative() {
			self.x = self.x.saturating_sub((-delta_x) as u16)
		} else {
			self.x = self.x.saturating_add(delta_x as u16)
		}
	}

	pub fn change_y(&mut self, delta_y: i16) {
		if delta_y.is_negative() {
			self.y = self.y.saturating_add((-delta_y) as u16)
		} else {
			self.y = self.y.saturating_sub(delta_y as u16)
		}
	}
	pub fn set_x(&mut self, x: u16) {
		if x >= MOUSE_MAX_X {
			self.x = MOUSE_MAX_X - 1;
		} else {
			self.x = x;
		}
	}
	pub fn set_y(&mut self, y: u16) {
		if y >= MOUSE_MAX_Y {
			self.y = MOUSE_MAX_Y - 1;
		} else {
			self.y = y;
		}
	}
}

// Initialize the mouse and set the on complete event.
pub fn init_mouse() {
	let mut mouse = _MOUSE.lock();
	info!("Trying to initialize mouse");
	mouse.init().unwrap();
	mouse.set_on_complete(on_complete);
	success!("Mouse initialized");
}

// This will be fired when a packet is finished being processed.
fn on_complete(mouse_state: MouseState) {
	// how much the cursor has moved
	let delta_x = mouse_state.get_x();
	let delta_y = mouse_state.get_y();

	let mut mouse = MOUSE.lock();
	if mouse.get_x() >= MOUSE_MAX_X {
		mouse.set_x(MOUSE_MAX_X - 1);
	} else {
		mouse.change_x(delta_x);
	}

	if mouse.get_y() >= MOUSE_MAX_Y {
		mouse.set_y(MOUSE_MAX_Y - 1);
	} else {
		mouse.change_y(delta_y);
	}
	// only move the cursor when delta_x is in some range
	// i.e. if the cursor moves too fast, ignore it.
	// if the mouse moves too fast the delta will overflow
	mouse.change_y(delta_y);

	draw_mouse((mouse.get_x() as usize, mouse.get_y() as usize));
}

// An example interrupt based on https://os.phil-opp.com/hardware-interrupts/.
// The ps2 mouse is configured to fire
// interrupts at PIC offset 12.
pub extern "x86-interrupt" fn mouse_interrupt_handler(_stack_frame: InterruptStackFrame) {
	let mut port = PortReadOnly::new(0x60);
	let packet = unsafe { port.read() };
	_MOUSE.lock().process_packet(packet);

	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(InterruptIndex::Mouse.into());
	}
}

fn draw_mouse(mouse_coord: (usize, usize)) {
	GRAPHICS.clear_screen(Color16::Black);
	//	GRAPHICS.draw_character(mouse_coord.0, mouse_coord.1, '.', CURSOR_COLOR);

	GRAPHICS.draw_line(
		(mouse_coord.0 as isize + 0, mouse_coord.1 as isize + 0),
		(mouse_coord.0 as isize + 10, mouse_coord.1 as isize + 10),
		CURSOR_COLOR,
	);
	GRAPHICS.draw_line(
		(mouse_coord.0 as isize + 0, mouse_coord.1 as isize + 0),
		(mouse_coord.0 as isize + 5, mouse_coord.1 as isize + 0),
		CURSOR_COLOR,
	);
	GRAPHICS.draw_line(
		(mouse_coord.0 as isize + 0, mouse_coord.1 as isize + 0),
		(mouse_coord.0 as isize + 0, mouse_coord.1 as isize + 5),
		CURSOR_COLOR,
	);
}
