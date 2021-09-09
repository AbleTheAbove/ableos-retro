use crate::{debug, kernel_state::KERNEL_STATE, CLIPBOARD, GRAPHICS_RAW};
use conquer_once::spin::OnceCell;
use core::{
	pin::Pin,
	task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{
	stream::{Stream, StreamExt},
	task::AtomicWaker,
};
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use vga::{colors::Color16, writers::GraphicsWriter};
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

/// Called by the keyboard interrupt handler
///
/// Must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
	if let Ok(queue) = SCANCODE_QUEUE.try_get() {
		if let Err(_) = queue.push(scancode) {
			// println!("WARNING: scancode queue full; dropping keyboard input");
		} else {
			WAKER.wake(); // new
		}
	} else {
		// println!("WARNING: scancode queue uninitialized");
	}
}

/// A stream of scancodes from the keyboard
pub struct ScancodeStream {
	_private: (),
}
impl ScancodeStream {
	///  create new scancode stream
	pub fn new() -> Self {
		SCANCODE_QUEUE
			.try_init_once(|| ArrayQueue::new(100))
			.expect("ScancodeStream::new should only be called once");
		ScancodeStream { _private: () }
	}
}
impl Stream for ScancodeStream {
	type Item = u8;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
		let queue = SCANCODE_QUEUE
			.try_get()
			.expect("scancode queue not initialized");

		// fast path
		if let Ok(scancode) = queue.pop() {
			return Poll::Ready(Some(scancode));
		}

		WAKER.register(&cx.waker());
		match queue.pop() {
			Ok(scancode) => {
				WAKER.take();
				Poll::Ready(Some(scancode))
			}
			Err(crossbeam_queue::PopError) => Poll::Pending,
		}
	}
}

// TODO: This should be refactored to add scancodes to the current windows key buffer
/// add scancodes to the current user window key buffer
pub async fn print_keypresses() {
	let mut scancodes = ScancodeStream::new();
	let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

	while let Some(scancode) = scancodes.next().await {
		if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
			if let Some(key) = keyboard.process_keyevent(key_event) {
				match key {
					DecodedKey::Unicode(character) => {
						match_key(character);
					}
					DecodedKey::RawKey(key) => {
						match_raw_key(key);
					}
				}
			}
		}
	}
}

fn match_raw_key(key: KeyCode) {
	match key {
		_key => {
			debug!("{:?}", key)
		}
	}
}

fn match_key(character: char) {
	match character {
		'\t' => toggle_task_menu(),
		'\n' => {
			debug!("Enter Pressed");
			CLIPBOARD.lock().clip_home();
		}
		'\u{1b}' => {
			debug!("Escape Pressed");
			// debug!("{:?}", CLIPBOARD.lock().paste());
		}
		// Pop the last element added to the text buffer and force redraw
		'\u{5B}' => debug!("Backspace Pressed"),
		_ => {
			debug!("{:?}", character);
		}
	}
}

fn _debug_kernel_state() {
	let kstate = KERNEL_STATE.lock();
	debug!("{:?}", kstate.hardware);
}

fn toggle_task_menu() {
	GRAPHICS_RAW.clear_screen(Color16::Black);

	let task_menu_visible = KERNEL_STATE.lock().task_menu;
	KERNEL_STATE.lock().task_menu ^= true;
	debug!("Task Menu Visible: {:?}", !task_menu_visible);
	if !task_menu_visible {
		GRAPHICS_RAW.draw_line((80, 60), (540, 60), Color16::White);
	}
}
