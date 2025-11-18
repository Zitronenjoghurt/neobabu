#[derive(Debug, Copy, Clone)]
pub enum EmojiType {
    ArrowBack,
    ArrowLeft,
    ArrowRight,
    ArrowDoubleLeft,
    ArrowDoubleRight,
    BalloonRed,
    Paper,
    Rock,
    Scissors,
    Sparkle,
}

impl EmojiType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ArrowBack => "arrow_back",
            Self::ArrowLeft => "arrow_left",
            Self::ArrowRight => "arrow_right",
            Self::ArrowDoubleLeft => "arrow_double_left",
            Self::ArrowDoubleRight => "arrow_double_right",
            Self::BalloonRed => "balloon_red",
            Self::Paper => "paper",
            Self::Rock => "rock",
            Self::Scissors => "scissors",
            Self::Sparkle => "sparkle",
        }
    }
}
