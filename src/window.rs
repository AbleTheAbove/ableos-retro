use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

use crate::alloc::string::ToString;
use lazy_static::lazy_static;

lazy_static! {
    static ref GRAPHICS: Graphics640x480x16 = {
        let mode = Graphics640x480x16::new();
        mode.set_mode();
        mode.clear_screen(Color16::Black);
        mode
    };
}

pub static WINDOW_BORDER_COLOR: Color16 = Color16::LightBlue;
pub static WINDOW_DECORATOR_COLOR: Color16 = Color16::LightBlue;
pub static WINDOW_DECORATOR_TEXT_COLOR: Color16 = Color16::Black;
/*
pub struct Size {
    x: usize,
    y: usize,
}

impl core::ops::Add for Size {}
*/
pub struct Window<'a> {
    pub title: &'a str,
    pub offset: (isize, isize),
    pub size: (usize, usize),
}
// BUG: drawing bigger than the screen size causes the buffer to wrap around
pub fn windows(id: u8, offset: (isize, isize)) {
    use alloc::format;
    use alloc::string::String;
    let mut win_title: String;
    match id {
        0 => {
            win_title = "AbleOS Terminal".to_string();
        }
        _ => {
            win_title = format!("AbleOS Window [ID: {}]", id);
        }
    }
    let window = Window {
        title: &win_title,
        offset: (offset.0, offset.1),
        size: (200, 100),
    };

    for y in 0..window.size.1 {
        GRAPHICS.draw_line(
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
    GRAPHICS.draw_line(
        (0 + window.offset.0, 0 + window.offset.1),
        (
            0 + window.offset.0,
            window.size.1 as isize + window.offset.1,
        ),
        WINDOW_BORDER_COLOR,
    );

    // Lowest line
    GRAPHICS.draw_line(
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
    GRAPHICS.draw_line(
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
            GRAPHICS.set_pixel(
                x + window.offset.0 as usize,
                y + window.offset.1 as usize,
                WINDOW_DECORATOR_COLOR,
            );
        }
    }

    let title_width = window.title.len() * 8;
    for (offset, character) in window.title.chars().enumerate() {
        GRAPHICS.draw_character(
            // TODO: Get length of character size and then do math
            (window.offset.0 as usize + ((window.size.0 - title_width) / 2)) as usize + offset * 8,
            (6 + window.offset.1) as usize,
            character,
            WINDOW_DECORATOR_TEXT_COLOR,
        )
    }
    logo((200, 10));
}

pub fn logo(offset: (isize, isize)) {
    // Left side of the A
    GRAPHICS.draw_line(
        (offset.0 + 20, offset.1 + 0),
        (offset.0 + 0, offset.1 + 40),
        Color16::White,
    );

    // Right side of the A
    GRAPHICS.draw_line(
        (offset.0 + 20, offset.1 + 0),
        (offset.0 + 40, offset.1 + 40),
        Color16::White,
    );

    // Center connector for the A
    GRAPHICS.draw_line(
        (offset.0 + 10, offset.1 + 20),
        (offset.0 + 30, offset.1 + 20),
        Color16::White,
    );

    //    GRAPHICS.draw_line((0, 0), (100, 100), Color16::Red);

    let mut fd = 8;
    GRAPHICS.draw_line((0, 10), (10, 10), Color16::Red);

}
/*
fn print(){

    // Turn this into a print macro
    for (offset, character) in "hello AbleOS;".chars().enumerate() {
        mode.draw_character(offset * 8, 0, character, Color16::White)
    }
}
*/
