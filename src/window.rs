use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

use crate::alloc::string::ToString;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    static ref GRAPHICS: Graphics640x480x16 = {
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

// BUG: drawing bigger than the screen size causes the buffer to wrap around
pub fn windows(id: u8, offset: (isize, isize)) {
    use alloc::format;
    use alloc::string::String;
    let mut win_title: String = "Fallback Window Name".to_string();
    let mut size = (0, 0);
    match id {
        0 => {
            win_title = "AbleOS Terminal".to_string();
            size = (639, 479);
        }
        _ => {
            win_title = format!("AbleOS Window [ID: {}]", id);
            size = (200, 100);
        }
    }
    let window = Window {
        title: &win_title,
        offset: (offset.0, offset.1),
        size: size,
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
}

pub fn logo(offset: (isize, isize)) {
    {
        let a_color = Color16::Pink;
        // Left side of the A
        GRAPHICS.draw_line(
            (offset.0 + 20, offset.1 + 0),
            (offset.0 + 10, offset.1 + 20),
            a_color,
        );

        // Right side of the A
        GRAPHICS.draw_line(
            (offset.0 + 20, offset.1 + 0),
            (offset.0 + 30, offset.1 + 20),
            a_color,
        );

        // Center connector for the A
        GRAPHICS.draw_line(
            (offset.0 + 10, offset.1 + 10),
            (offset.0 + 30, offset.1 + 10),
            a_color,
        );
    }

    let offset_c = (offset.0 + 5, offset.1 - 15);
    let crown_color = Color16::Yellow;
    GRAPHICS.draw_line(
        (offset_c.0 + 0 / 2, offset_c.1 + 0 / 2),
        (offset_c.0 + 10 / 2, offset_c.1 + 20 / 2),
        crown_color,
    );

    GRAPHICS.draw_line(
        (offset_c.0 + 10 / 2, offset_c.1 + 20 / 2),
        (offset_c.0 + 50 / 2, offset_c.1 + 20 / 2),
        crown_color,
    );
    GRAPHICS.draw_line(
        (offset_c.0 + 50 / 2, offset_c.1 + 20 / 2),
        (offset_c.0 + 60 / 2, offset_c.1 + 0 / 2),
        crown_color,
    );
    GRAPHICS.draw_line(
        (offset_c.0 + 60 / 2, offset_c.1 + 0 / 2),
        (offset_c.0 + 40 / 2, offset_c.1 + 10 / 2),
        crown_color,
    );
    GRAPHICS.draw_line(
        (offset_c.0 + 40 / 2, offset_c.1 + 10 / 2),
        (offset_c.0 + 30 / 2, offset_c.1 + 0 / 2),
        crown_color,
    );
    GRAPHICS.draw_line(
        (offset_c.0 + 30 / 2, offset_c.1 + 0 / 2),
        (offset_c.0 + 20 / 2, offset_c.1 + 10 / 2),
        crown_color,
    );
    GRAPHICS.draw_line(
        (offset_c.0 + 20 / 2, offset_c.1 + 10 / 2),
        (offset_c.0 + 0 / 2, offset_c.1 + 0 / 2),
        crown_color,
    );
}
/*
fn print(){

    // Turn this into a print macro
    for (offset, character) in "hello AbleOS;".chars().enumerate() {
        mode.draw_character(offset * 8, 0, character, crown_color)
    }
}
*/

pub fn draw_terminal() {
    //X

    type character = (char, Color16);
    type line = [char; 80];
    type buff = [line; 60];

    let mut text_line = [('b', Color16::Red); 80];
    let mut text_buffer = [text_line; 60];
    text_buffer[0][1].0 = 'a';
    text_buffer[0][1].1 = Color16::White;

    let mut line2 = 0;
    let mut offset = 0;

    for line in text_buffer.iter() {
        for character in line.iter() {
            GRAPHICS.draw_character(offset * 8, line2 * 8, character.0, character.1);
            offset += 1;
        }
        line2 += 1;
    }
}
