use crate::games::playing_cards::{PlayingCard, PlayingCardDeck};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlackjackMove {
    Hit,
    Stand,
}

#[derive(Debug)]
pub struct BlackjackGame {
    pub deck: PlayingCardDeck,
    pub dealer: BlackjackPlayer,
    pub players: HashMap<String, BlackjackPlayer>,
    pub current_player: Option<String>,
    pub last_move: Option<Instant>,
}

impl Default for BlackjackGame {
    fn default() -> Self {
        Self {
            deck: PlayingCardDeck::new_shuffled(),
            dealer: BlackjackPlayer::default(),
            players: HashMap::new(),
            current_player: None,
            last_move: None,
        }
    }
}

impl BlackjackGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_player(&mut self, player: impl AsRef<str>) {
        self.players
            .insert(player.as_ref().to_string(), BlackjackPlayer::default());
    }

    pub fn start_game(&mut self) -> bool {
        if self.players.is_empty() {
            return false;
        }

        self.play_dealer_if_needed();
        true
    }

    pub fn has_started(&self) -> bool {
        self.last_move.is_some()
    }

    pub fn is_over(&self) -> bool {
        !self.dealer.can_move() && self.players.values().all(|player| !player.can_move())
    }

    fn next_player(&mut self) {
        self.last_move = Some(Instant::now());

        let player_keys: Vec<String> = self.players.keys().cloned().collect();
        let current_index = self
            .current_player
            .as_ref()
            .map(|current| player_keys.iter().position(|p| p == current).unwrap_or(0))
            .unwrap_or(0);

        self.current_player = player_keys
            .iter()
            .skip(current_index + 1)
            .find(|player| {
                self.players
                    .get(*player)
                    .map(|player| player.can_move())
                    .unwrap_or(false)
            })
            .cloned();
    }

    fn play_dealer_if_needed(&mut self) {
        if self.current_player.is_none() && self.dealer.can_move() {
            if self.dealer.score() < 17 {
                self.dealer
                    .deck
                    .add_bottom(self.deck.draw_top().unwrap_or(PlayingCard::Spades2));
            } else {
                self.dealer.standing = true;
            }
            self.next_player();
        }
    }

    pub fn play(&mut self, player: impl AsRef<str>, move_: BlackjackMove) -> bool {
        let player = player.as_ref().to_string();
        if Some(&player) != self.current_player.as_ref() {
            return false;
        }

        let Some(player) = self.players.get_mut(&player) else {
            return false;
        };

        if move_ == BlackjackMove::Hit {
            player
                .deck
                .add_bottom(self.deck.draw_top().unwrap_or(PlayingCard::Spades2));
        } else {
            player.standing = true;
        }

        self.next_player();

        true
    }
}

#[derive(Debug, Default)]
pub struct BlackjackPlayer {
    deck: PlayingCardDeck,
    standing: bool,
}

impl BlackjackPlayer {
    pub fn score(&self) -> u8 {
        let mut cards = self.deck.iter_cards().collect::<Vec<_>>();
        cards.sort_by_key(|card| card.score());

        let mut score = 0;
        for card in cards {
            if card.is_ace() && score + 11 > 21 {
                score += 1;
            } else {
                score += card.score();
            }
        }

        score
    }

    pub fn is_bust(&self) -> bool {
        self.score() > 21
    }

    pub fn can_move(&self) -> bool {
        !self.standing && !self.is_bust()
    }
}
