use rand::prelude::IndexedRandom;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    pub fn random() -> Self {
        let choice = [Self::Rock, Self::Paper, Self::Scissors]
            .choose(&mut rand::rng())
            .copied();
        choice.unwrap_or(Self::Rock)
    }
}

impl Display for RPSChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "Rock"),
            Self::Paper => write!(f, "Paper"),
            Self::Scissors => write!(f, "Scissors"),
        }
    }
}
