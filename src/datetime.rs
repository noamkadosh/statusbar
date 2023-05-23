use chrono::{Datelike, FixedOffset, Utc};

use zellij_tile::prelude::*;
use zellij_tile_utils::style;

use crate::{
    color::ModeColor,
    view::{Block, View},
};

#[derive(PartialEq)]
pub struct DateTime {
    pub date: String,
    pub time: String,
}

impl DateTime {
    // NOTE: `time` crate can't get UTC offset in multithreaded binaries on macos:
    // https://github.com/time-rs/time/issues/293
    // so using chrono (which is also broken here) :shrugs:
    pub fn now() -> Self {
        // FIXME: UTC offset is always +00:00. Hardcoding TZ for now.
        let utc = Utc::now();
        let offset = FixedOffset::east_opt(-7 * 3600 /* hours */).unwrap();
        let now = utc.with_timezone(&offset);

        let day_suffix = match now.day() {
            1 | 21 | 31 => "st",
            2 | 22 => "nd",
            3 | 23 => "rd",
            _ => "th",
        };

        let date = now.format("%a, %b %-d").to_string() + day_suffix;
        let time = now.format("%-I:%M %p").to_string();

        Self {
            date: date.to_owned(),
            time: time.to_owned(),
        }
    }

    pub fn render(&self, mode: InputMode, palette: Palette) -> View {
        use unicode_width::UnicodeWidthStr;

        let mut blocks = vec![];
        let mut total_len = 0;

        let ModeColor { mode_color } = ModeColor::new(mode, palette);

        // date
        {
            let text = format!("  {} ", self.date);
            let len = text.width();
            let body = style!(palette.bg, mode_color).bold().paint(text);

            total_len += len;
            blocks.push(Block {
                body: body.to_string(),
                len,
                tab_index: None,
            });
        }

        // time
        {
            let text = format!(" 󱑇 {} ", self.time);
            let len = text.width();
            let body = style!(palette.bg, mode_color).bold().paint(text);

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

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}
