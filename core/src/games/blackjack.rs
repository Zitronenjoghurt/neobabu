use crate::games::playing_cards::PlayingCardDeck;
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
    pub turn_order: Vec<String>,
    pub last_move: Option<Instant>,
}

impl Default for BlackjackGame {
    fn default() -> Self {
        Self {
            deck: PlayingCardDeck::new_shuffled(),
            dealer: BlackjackPlayer::default(),
            players: HashMap::new(),
            turn_order: Vec::new(),
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
        let name = player.as_ref().to_string();
        if !self.players.contains_key(&name) {
            self.players
                .insert(name.clone(), BlackjackPlayer::default());
            self.turn_order.push(name);
        }
    }

    pub fn start_game(&mut self) -> bool {
        if self.players.is_empty() {
            return false;
        }

        for _ in 0..2 {
            for name in &self.turn_order {
                if let Some(player) = self.players.get_mut(name) {
                    if let Some(card) = self.deck.draw_top() {
                        player.deck.add_bottom(card);
                    }
                }
            }

            if let Some(card) = self.deck.draw_top() {
                self.dealer.deck.add_bottom(card);
            }
        }

        self.current_player = self.turn_order.first().cloned();
        self.last_move = Some(Instant::now());

        if let Some(name) = &self.current_player {
            if let Some(p) = self.players.get(name) {
                if !p.can_move() {
                    self.next_player();
                }
            }
        }

        true
    }

    pub fn has_started(&self) -> bool {
        self.last_move.is_some()
    }

    pub fn is_over(&self) -> bool {
        self.current_player.is_none() && self.dealer.standing
    }

    fn next_player(&mut self) {
        self.last_move = Some(Instant::now());

        let current_index = self
            .current_player
            .as_ref()
            .and_then(|name| self.turn_order.iter().position(|p| p == name));

        let next_index = match current_index {
            Some(i) => i + 1,
            None => 0,
        };

        if next_index < self.turn_order.len() {
            self.current_player = Some(self.turn_order[next_index].clone());
        } else {
            self.current_player = None;
            self.play_dealer();
        }
    }

    fn play_dealer(&mut self) {
        while self.dealer.can_move() && self.dealer.score() < 17 {
            if let Some(card) = self.deck.draw_top() {
                self.dealer.deck.add_bottom(card);
            } else {
                break;
            }
        }
        self.dealer.standing = true;
    }

    pub fn is_to_play(&self, player: impl AsRef<str>) -> bool {
        self.current_player
            .as_ref()
            .map(|current| current == player.as_ref())
            .unwrap_or(false)
    }

    pub fn is_bust(&self, player: impl AsRef<str>) -> bool {
        self.players
            .get(player.as_ref())
            .map(|p| p.is_bust())
            .unwrap_or(false)
    }

    pub fn is_stand(&self, player: impl AsRef<str>) -> bool {
        self.players
            .get(player.as_ref())
            .map(|p| p.standing)
            .unwrap_or(false)
    }

    pub fn play(&mut self, player_name: impl AsRef<str>, move_: BlackjackMove) -> bool {
        let player_name = player_name.as_ref().to_string();

        if !self.is_to_play(&player_name) {
            return false;
        }

        let Some(player) = self.players.get_mut(&player_name) else {
            return false;
        };

        if move_ == BlackjackMove::Hit {
            if let Some(card) = self.deck.draw_top() {
                player.deck.add_bottom(card);

                if player.is_bust() {
                    player.standing = true;
                    self.next_player();
                }
            }
        } else {
            player.standing = true;
            self.next_player();
        }

        self.last_move = Some(Instant::now());
        true
    }

    pub fn iter_players(&self) -> impl Iterator<Item = (&String, &BlackjackPlayer)> {
        self.turn_order
            .iter()
            .filter_map(|name| self.players.get(name).map(|p| (name, p)))
    }

    pub fn get_outcomes(&self) -> Option<HashMap<String, BlackjackOutcome>> {
        if !self.is_over() {
            return None;
        }

        let dealer_score = self.dealer.score();
        let dealer_bust = self.dealer.is_bust();

        let results = self
            .iter_players()
            .map(|(name, player)| {
                let player_score = player.score();
                let player_bust = player.is_bust();

                let outcome = if player_bust {
                    BlackjackOutcome::Loss
                } else if dealer_bust || player_score > dealer_score {
                    BlackjackOutcome::Win
                } else if player_score == dealer_score {
                    BlackjackOutcome::Push
                } else {
                    BlackjackOutcome::Loss
                };

                (name.clone(), outcome)
            })
            .collect();

        Some(results)
    }
}

#[derive(Debug, Default)]
pub struct BlackjackPlayer {
    pub deck: PlayingCardDeck,
    pub standing: bool,
}

impl BlackjackPlayer {
    pub fn deck(&self) -> &PlayingCardDeck {
        &self.deck
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlackjackOutcome {
    Win,
    Loss,
    Push,
}
