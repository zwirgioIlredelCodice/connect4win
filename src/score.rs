use crate::board::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Score {
    pub score: i32,
    pub state: GameState,
}

pub const MAX: Score = Score { score: 0, state: GameState::WINP1 };
pub const MIN: Score = Score { score: 0, state: GameState::WINP2 };
pub const EQUAL: Score = Score { score: 0, state: GameState::OPEN };

impl Default for Score {
    fn default() -> Self {
        Score { score: 0, state: GameState::OPEN }
    }
}

impl Eq for Score {}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        if self.state == other.state {
            return true;
        } else {
            return self.score == other.score;
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.state == other.state {
            return self.state.cmp(&other.state);
        } else {
            return self.score.cmp(&other.score);
        }
    }

    fn max(self, other: Self) -> Self
        where
            Self: Sized, {
        if self > other {
            self
        } else {
            other
        }
    }
    
    fn min(self, other: Self) -> Self
        where
            Self: Sized, {
        if self < other {
            self
        } else {
            other
        }
    }
}