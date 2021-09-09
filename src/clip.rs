use crate::alloc::vec;
use crate::String;
use crate::Vec;
use lazy_static::lazy_static;
#[derive(Debug)]
pub enum Mime {
	None,
	Text(String),
}

lazy_static! {
	pub static ref CLIPBOARD: spin::Mutex<Clipboard> = {
		let clipboard = Clipboard::new();

		spin::Mutex::new(clipboard)
	};
}

pub struct Clipboard {
	pub index: usize,
	pub pages: Vec<Mime>,
}
impl Clipboard {
	pub fn new() -> Clipboard {
		Clipboard {
			index: 0,
			pages: vec![],
		}
	}

	pub fn clear(&mut self) {
		self.pages = vec![];
	}
	pub fn set_index(&mut self, index_new: usize) {
		self.index = index_new;
	}
	pub fn clip_end(&mut self) {
		self.index = 0;
	}
	pub fn clip_home(&mut self) {
		self.index = self.pages.len();
	}

	pub fn copy(&mut self, copy_mime: Mime) {
		self.pages.push(copy_mime);
	}
	pub fn paste(&mut self) -> &Mime {
		let paste_pos = &self.pages[self.index];
		paste_pos
	}
}
