use unicode_width::UnicodeWidthStr;

use zellij_tile::prelude::*;
use zellij_tile_utils::style;

pub struct View {
    pub blocks: Vec<Block>,
    pub len: usize,
}

#[derive(Default, Clone, Debug)]
pub struct Block {
    pub body: String,
    pub len: usize,
    pub tab_index: Option<usize>,
}

pub struct Bg;

impl Bg {
    pub fn render(cols: usize, palette: Palette) -> Block {
        let text = format!("{: <1$}", "", cols);
        let body = style!(palette.fg, palette.bg).paint(text);

        Block {
            body: body.to_string(),
            len: cols,
            tab_index: None,
        }
    }
}

pub struct Spacer {
    pub space: Option<Block>,
}

impl Spacer {
    pub fn render(
        total_len: usize,
        (left_len, right_len): (usize, usize),
        palette: Palette,
    ) -> Self {
        let room = total_len - left_len - right_len;

        if room > 0 {
            Self {
                space: Some(Bg::render(room, palette)),
            }
        } else {
            // We ran out of space
            Self { space: None }
        }
    }
}

pub struct Error;

impl Error {
    pub fn render(message: &str, palette: Palette) -> Block {
        let text = message;
        let len = text.width();
        let body = style!(palette.white, palette.red).bold().paint(text);

        Block {
            body: body.to_string(),
            len,
            tab_index: None,
        }
    }
}
