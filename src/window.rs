use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

pub struct Window<'a> {
    title: &'a str,
    offset: (isize, isize),
    size: (usize, usize),
}

pub fn windows() {
    let window = Window {
        title: "AbleOS Window Example",
        offset: (0, 0),
        size: (400, 80),
    };

    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);
    // Left line
    mode.draw_line(
        (0 + window.offset.0, 0 + window.offset.1),
        (
            0 + window.offset.0,
            window.size.1 as isize + window.offset.1,
        ),
        Color16::White,
    );
    // Uppermost line
    mode.draw_line(
        (0 + window.offset.0, 0 + window.offset.1),
        (
            window.size.0 as isize + window.offset.0,
            0 + window.offset.1,
        ),
        Color16::White,
    );
    // Lowest line
    mode.draw_line(
        (
            0 + window.offset.0,
            window.size.1 as isize + window.offset.1,
        ),
        (
            window.size.0 as isize + window.offset.0,
            window.size.1 as isize + window.offset.1,
        ),
        Color16::White,
    );

    //right most line
    mode.draw_line(
        (
            window.size.0 as isize + window.offset.0,
            window.size.1 as isize + window.offset.1,
        ),
        (window.size.0 as isize + window.offset.0, window.offset.1),
        Color16::White,
    );
    // Second top line
    mode.draw_line(
        (0 + window.offset.0, 30 + window.offset.1),
        (
            window.size.0 as isize + window.offset.0,
            30 + window.offset.1,
        ),
        Color16::White,
    );

    {
        // A simple colored window display that I think should be fully implemented
        for y in 1..30 {
            for x in 1..window.size.0 {
                mode.set_pixel(
                    x + window.offset.0 as usize,
                    y + window.offset.1 as usize,
                    Color16::Green,
                );
            }
        }
    }

    let title_width = window.title.len() * 8;
    for (offset, character) in window.title.chars().enumerate() {
        mode.draw_character(
            // TODO: Get length of character size and then do math
            (window.offset.0 as usize + ((window.size.0 - title_width) / 2)) as usize + offset * 8,
            (12 + window.offset.1) as usize,
            character,
            Color16::White,
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
