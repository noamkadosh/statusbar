use unicode_width::UnicodeWidthStr;

use zellij_tile::prelude::*;
use zellij_tile_utils::style;

use crate::{
    color::ModeColor,
    view::{Block, View},
};

pub struct Mode;

impl Mode {
    pub fn render(mode: InputMode, palette: Palette) -> View {
        let mut blocks = vec![];
        let mut total_len = 0;

        let ModeColor { mode_color } = ModeColor::new(mode, palette);

        let mode = match mode {
            InputMode::EnterSearch => "Enter Search",
            InputMode::Locked => "Locked",
            InputMode::Move => "Move",
            InputMode::Normal => "Normal",
            InputMode::Pane => "Pane",
            InputMode::Prompt => "Prompt",
            InputMode::RenamePane => "Rename Pane",
            InputMode::RenameTab => "Rename Tab",
            InputMode::Resize => "Resize",
            InputMode::Scroll => "Scroll",
            InputMode::Search => "Search",
            InputMode::Session => "Session",
            InputMode::Tab => "Tab",
            InputMode::Tmux => "Tmux",
        };

        let mode = format!(" {:?} ", mode).to_uppercase().replace("\"", "");
        let len = mode.width();
        let body = style!(palette.bg, mode_color).bold().paint(mode);

        total_len += len;
        blocks.push(Block {
            body: body.to_string(),
            len,
            tab_index: None,
        });

        View {
            blocks,
            len: total_len,
        }
    }
}
