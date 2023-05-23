use unicode_width::UnicodeWidthStr;

use zellij_tile::prelude::*;
use zellij_tile_utils::style;

use crate::{
    color::ModeColor,
    view::{Block, View},
};

pub struct Session;

impl Session {
    pub fn render(name: Option<&str>, mode: InputMode, palette: Palette) -> View {
        let mut blocks = vec![];
        let mut total_len = 0;

        // name
        if let Some(name) = name {
            let ModeColor { mode_color } = ModeColor::new(mode, palette);

            let text = format!(" {} ", name.to_uppercase());
            let len = text.width();
            let body = style!(mode_color, palette.black).bold().paint(text);

            total_len += len;
            blocks.push(Block {
                body: body.to_string(),
                len,
                tab_index: None,
            })
        }

        View {
            blocks,
            len: total_len,
        }
    }
}
