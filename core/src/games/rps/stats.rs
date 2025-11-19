pub struct RPSStats {
    pub wins: u64,
    pub losses: u64,
    pub draws: u64,
    pub rock: u64,
    pub paper: u64,
    pub scissors: u64,
}

impl RPSStats {
    pub fn total_played(&self) -> u64 {
        self.wins + self.losses + self.draws
    }

    pub fn win_rate(&self) -> f64 {
        self.wins as f64 / self.total_played() as f64
    }
}
