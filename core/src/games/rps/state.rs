#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RPSState {
    WaitingForBoth,
    WaitingFor1,
    WaitingFor2,
    Winner1,
    Winner2,
    Draw,
}

impl RPSState {
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Winner1 | Self::Winner2 | Self::Draw)
    }

    pub fn is_ongoing(&self) -> bool {
        !self.is_finished()
    }

    pub fn waiting_for_1(&self) -> bool {
        matches!(self, Self::WaitingFor1 | Self::WaitingForBoth)
    }

    pub fn waiting_for_2(&self) -> bool {
        matches!(self, Self::WaitingFor2 | Self::WaitingForBoth)
    }

    pub fn waiting_for(&self, is_1: bool) -> bool {
        if is_1 {
            self.waiting_for_1()
        } else {
            self.waiting_for_2()
        }
    }
}
