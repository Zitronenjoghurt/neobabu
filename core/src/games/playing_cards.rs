use num_enum::TryFromPrimitive;
use rand::prelude::{Rng, SliceRandom};
use std::collections::VecDeque;
use strum::IntoEnumIterator;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    sea_orm::EnumIter,
    TryFromPrimitive,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(u8)]
pub enum PlayingCard {
    Spades2 = 0,
    Spades3 = 1,
    Spades4 = 2,
    Spades5 = 3,
    Spades6 = 4,
    Spades7 = 5,
    Spades8 = 6,
    Spades9 = 7,
    Spades10 = 8,
    SpadesJack = 9,
    SpadesQueen = 10,
    SpadesKing = 11,
    SpadesAce = 12,
    Hearts2 = 13,
    Hearts3 = 14,
    Hearts4 = 15,
    Hearts5 = 16,
    Hearts6 = 17,
    Hearts7 = 18,
    Hearts8 = 19,
    Hearts9 = 20,
    Hearts10 = 21,
    HeartsJack = 22,
    HeartsQueen = 23,
    HeartsKing = 24,
    HeartsAce = 25,
    Clubs2 = 26,
    Clubs3 = 27,
    Clubs4 = 28,
    Clubs5 = 29,
    Clubs6 = 30,
    Clubs7 = 31,
    Clubs8 = 32,
    Clubs9 = 33,
    Clubs10 = 34,
    ClubsJack = 35,
    ClubsQueen = 36,
    ClubsKing = 37,
    ClubsAce = 38,
    Diamonds2 = 39,
    Diamonds3 = 40,
    Diamonds4 = 41,
    Diamonds5 = 42,
    Diamonds6 = 43,
    Diamonds7 = 44,
    Diamonds8 = 45,
    Diamonds9 = 46,
    Diamonds10 = 47,
    DiamondsJack = 48,
    DiamondsQueen = 49,
    DiamondsKing = 50,
    DiamondsAce = 51,
}

impl PlayingCard {
    pub fn score(&self) -> u8 {
        match self {
            PlayingCard::Spades2
            | PlayingCard::Hearts2
            | PlayingCard::Clubs2
            | PlayingCard::Diamonds2 => 2,
            PlayingCard::Spades3
            | PlayingCard::Hearts3
            | PlayingCard::Clubs3
            | PlayingCard::Diamonds3 => 3,
            PlayingCard::Spades4
            | PlayingCard::Hearts4
            | PlayingCard::Clubs4
            | PlayingCard::Diamonds4 => 4,
            PlayingCard::Spades5
            | PlayingCard::Hearts5
            | PlayingCard::Clubs5
            | PlayingCard::Diamonds5 => 5,
            PlayingCard::Spades6
            | PlayingCard::Hearts6
            | PlayingCard::Clubs6
            | PlayingCard::Diamonds6 => 6,
            PlayingCard::Spades7
            | PlayingCard::Hearts7
            | PlayingCard::Clubs7
            | PlayingCard::Diamonds7 => 7,
            PlayingCard::Spades8
            | PlayingCard::Hearts8
            | PlayingCard::Clubs8
            | PlayingCard::Diamonds8 => 8,
            PlayingCard::Spades9
            | PlayingCard::Hearts9
            | PlayingCard::Clubs9
            | PlayingCard::Diamonds9 => 9,
            PlayingCard::Spades10
            | PlayingCard::Hearts10
            | PlayingCard::Clubs10
            | PlayingCard::Diamonds10 => 10,
            PlayingCard::SpadesJack
            | PlayingCard::HeartsJack
            | PlayingCard::ClubsJack
            | PlayingCard::DiamondsJack => 10,
            PlayingCard::SpadesQueen
            | PlayingCard::HeartsQueen
            | PlayingCard::ClubsQueen
            | PlayingCard::DiamondsQueen => 10,
            PlayingCard::SpadesKing
            | PlayingCard::HeartsKing
            | PlayingCard::ClubsKing
            | PlayingCard::DiamondsKing => 10,
            PlayingCard::SpadesAce
            | PlayingCard::HeartsAce
            | PlayingCard::ClubsAce
            | PlayingCard::DiamondsAce => 11,
        }
    }

    pub fn is_ace(&self) -> bool {
        matches!(
            self,
            PlayingCard::SpadesAce
                | PlayingCard::HeartsAce
                | PlayingCard::ClubsAce
                | PlayingCard::DiamondsAce
        )
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayingCardDeck {
    cards: VecDeque<PlayingCard>,
}

impl PlayingCardDeck {
    pub fn new() -> Self {
        Self {
            cards: PlayingCard::iter().collect(),
        }
    }

    pub fn new_shuffled() -> Self {
        let mut deck = Self::new();
        deck.shuffle(&mut rand::rng());
        deck
    }

    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        self.cards.make_contiguous().shuffle(rng);
    }

    pub fn add_top(&mut self, card: PlayingCard) {
        self.cards.push_back(card);
    }

    pub fn add_bottom(&mut self, card: PlayingCard) {
        self.cards.push_front(card);
    }

    pub fn add_multiple_top(&mut self, cards: &[PlayingCard]) {
        self.cards.extend(cards);
    }

    pub fn add_multiple_bottom(&mut self, cards: &[PlayingCard]) {
        for card in cards.iter().rev() {
            self.cards.push_front(*card);
        }
    }

    pub fn add_random<R: Rng>(&mut self, card: PlayingCard, rng: &mut R) {
        if self.cards.is_empty() {
            self.cards.push_back(card);
        } else {
            let index = rng.random_range(0..=self.cards.len());
            self.cards.insert(index, card);
        }
    }

    pub fn draw_top(&mut self) -> Option<PlayingCard> {
        self.cards.pop_back()
    }

    pub fn draw_bottom(&mut self) -> Option<PlayingCard> {
        self.cards.pop_front()
    }

    pub fn draw_random<R: Rng>(&mut self, rng: &mut R) -> Option<PlayingCard> {
        if self.cards.is_empty() {
            return None;
        }
        let index = rng.random_range(0..self.cards.len());
        self.cards.remove(index)
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn iter_cards(&self) -> impl Iterator<Item = PlayingCard> + '_ {
        self.cards.iter().copied()
    }
}
