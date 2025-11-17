#[derive(Debug, Copy, Clone)]
pub enum Emoji {
    BalloonRed,
    Sparkle,
}

impl Emoji {
    pub fn name(&self) -> &'static str {
        match self {
            Self::BalloonRed => "balloon_red",
            Self::Sparkle => "sparkle",
        }
    }
}
