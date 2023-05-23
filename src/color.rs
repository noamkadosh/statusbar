use zellij_tile::prelude::{InputMode, Palette, PaletteColor};

pub struct ModeColor {
    pub mode_color: PaletteColor,
}

impl ModeColor {
    pub fn new(mode: InputMode, palette: Palette) -> Self {
        let mode_color = match mode {
            InputMode::EnterSearch => palette.cyan,
            InputMode::Locked => palette.red,
            InputMode::Move => palette.green,
            InputMode::Normal => palette.blue,
            InputMode::Pane => palette.orange,
            InputMode::Prompt => palette.blue,
            InputMode::RenamePane => palette.orange,
            InputMode::RenameTab => palette.magenta,
            InputMode::Resize => palette.green,
            InputMode::Scroll => palette.green,
            InputMode::Search => palette.cyan,
            InputMode::Session => palette.red,
            InputMode::Tab => palette.magenta,
            InputMode::Tmux => palette.blue,
        };

        Self { mode_color }
    }
}
