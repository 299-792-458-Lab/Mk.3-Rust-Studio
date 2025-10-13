use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nation {
    Tera,
    Sora,
    Aqua,
}

impl Nation {
    pub fn name(&self) -> &'static str {
        match self {
            Nation::Tera => "Tera",
            Nation::Sora => "Sora",
            Nation::Aqua => "Aqua",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Nation::Tera => Color::Blue,
            Nation::Sora => Color::Red,
            Nation::Aqua => Color::Green,
        }
    }
}
