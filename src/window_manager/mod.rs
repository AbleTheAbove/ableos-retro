use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;
use vga::{
    colors::Color16,
    writers::{Graphics640x480x16, GraphicsWriter},
};
pub mod window_draw;
/// Holds references to all windows
// pub struct WindowManager {
//     vga_buff: Graphics640x480x16,
// }
// impl WindowManager {}

mod terminal;

lazy_static! {
    pub static ref GRAPHICS: Graphics640x480x16 = {
        let mode = Graphics640x480x16::new();
        mode.set_mode();
        mode.clear_screen(Color16::Black);
        mode
    };
}

lazy_static! {
    pub static ref WINDOWS: WindowHolder<'static> = WindowHolder(Mutex::new(Vec::new()));
}

pub static WINDOW_BORDER_COLOR: Color16 = Color16::LightBlue;
pub static WINDOW_DECORATOR_COLOR: Color16 = Color16::LightBlue;
pub static WINDOW_DECORATOR_TEXT_COLOR: Color16 = Color16::Black;

pub struct WindowHolder<'a>(pub Mutex<Vec<&'a Window<'a>>>);

pub struct Window<'a> {
    pub title: &'a str,
    // TODO: Turn this into a type
    pub offset: (isize, isize),
    // TODO: Turn this into a type
    pub size: (usize, usize),
}
