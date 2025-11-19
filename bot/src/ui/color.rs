// https://lospec.com/palette-list/glomzy-05
#[derive(Debug, Copy, Clone)]
pub enum UiColor {
    Lime,
    LightGray,
    Gray,
    DarkGray,
    LightRed,
    Orange,
    Yellow,
    Pink,
    Config,
    Success,
    Warning,
    Error,
}

impl UiColor {
    pub fn as_serenity(&self) -> poise::serenity_prelude::Color {
        match self {
            Self::Lime => poise::serenity_prelude::Color::from_rgb(172, 181, 101),
            Self::LightGray => poise::serenity_prelude::Color::from_rgb(217, 211, 217),
            Self::Gray => poise::serenity_prelude::Color::from_rgb(160, 151, 161),
            Self::DarkGray => poise::serenity_prelude::Color::from_rgb(107, 94, 107),
            Self::LightRed => poise::serenity_prelude::Color::from_rgb(184, 92, 84),
            Self::Orange => poise::serenity_prelude::Color::from_rgb(213, 158, 102),
            Self::Yellow => poise::serenity_prelude::Color::from_rgb(221, 201, 132),
            Self::Pink => poise::serenity_prelude::Color::from_rgb(174, 102, 124),
            Self::Config => Self::Gray.as_serenity(),
            Self::Success => Self::Lime.as_serenity(),
            Self::Warning => Self::Yellow.as_serenity(),
            Self::Error => Self::LightRed.as_serenity(),
        }
    }
}
