use std::cmp::Ordering;
use std::fmt::Display;

use crate::board::*;
use crate::score::*;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    col: u8,
    player: Player,
    score: Score,
    #[allow(dead_code)]
    depth: u8,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Move {
    pub fn new(col: u8, player: Player, score: Score, depth: u8) -> Self {
        Self {
            col,
            player,
            score,
            depth,
        }
    }

    pub fn col(&self) -> u8 {
        self.col
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn player(&self) -> Player {
        self.player
    }
}

impl Eq for Move {}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}
