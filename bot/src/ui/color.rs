use neobabu_core::games::farming::season::Season;

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
    Nasa,
    Youtube,
    Config,
    Success,
    Warning,
    Error,
    FarmingSpring,
    FarmingSummer,
    FarmingAutumn,
    FarmingWinter,
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
            Self::Nasa => poise::serenity_prelude::Color::from_rgb(26, 59, 140),
            Self::Youtube => poise::serenity_prelude::Color::from_rgb(255, 0, 51),
            Self::FarmingSpring => poise::serenity_prelude::Color::from_rgb(225, 175, 233),
            Self::FarmingSummer => poise::serenity_prelude::Color::from_rgb(240, 196, 79),
            Self::FarmingAutumn => poise::serenity_prelude::Color::from_rgb(165, 65, 55),
            Self::FarmingWinter => poise::serenity_prelude::Color::from_rgb(255, 255, 255),
            Self::Config => Self::Gray.as_serenity(),
            Self::Success => Self::Lime.as_serenity(),
            Self::Warning => Self::Yellow.as_serenity(),
            Self::Error => Self::LightRed.as_serenity(),
        }
    }
}

impl From<Season> for UiColor {
    fn from(season: Season) -> Self {
        match season {
            Season::Spring => Self::FarmingSpring,
            Season::Summer => Self::FarmingSummer,
            Season::Autumn => Self::FarmingAutumn,
            Season::Winter => Self::FarmingWinter,
        }
    }
}
