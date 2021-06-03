use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

use lazy_static::lazy_static;

lazy_static! {
    static ref GRAPHICS: Graphics640x480x16 = {
        let mode = Graphics640x480x16::new();
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
    title: &'a str,
    offset: (isize, isize),
    size: (usize, usize),
}

pub fn windows() {
    let window = Window {
        title: "AbleOS Window Example",
        offset: (0, 0),
        size: (400, 60),
    };

    GRAPHICS.set_mode();
    GRAPHICS.clear_screen(Color16::Black);
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
}

/*
fn print(){

    // Turn this into a print macro
    for (offset, character) in "hello AbleOS;".chars().enumerate() {
        mode.draw_character(offset * 8, 0, character, Color16::White)
    }
}
*/
