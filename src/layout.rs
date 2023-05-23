use unicode_width::UnicodeWidthStr;

use zellij_tile::prelude::*;
use zellij_tile_utils::style;

use crate::{
    color::ModeColor,
    view::{Block, View},
};

pub struct Layout;

impl Layout {
    pub fn render(mode: InputMode, tabs: &[TabInfo], palette: Palette) -> View {
        let mut blocks = vec![];
        let mut total_len = 0;

        let mut active_tab_index = 0;

        for (index, tab) in tabs.iter().enumerate() {
            if tab.active {
                active_tab_index = index
            }
        }

        let layout = tabs[active_tab_index].active_swap_layout_name.clone();

        if let Some(layout) = layout {
            let ModeColor { mode_color } = ModeColor::new(mode, palette);

            let layout = format!(" {:?} ", layout).to_uppercase().replace("\"", "");

            let len = layout.width();
            let body = style!(mode_color, palette.black).bold().paint(layout);

            total_len += len;
            blocks.push(Block {
                body: body.to_string(),
                len,
                tab_index: None,
            });
        }

        View {
            blocks,
            len: total_len,
        }
    }
}
