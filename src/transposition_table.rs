use crate::bit_board::*;
use crate::score::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::hash::*;

#[derive(Clone, Default)]
pub struct Entry {
    score: Score,
    key: BitBoard,
}

pub struct Table {
    size: usize,
    table: Box<[Entry]>,
}

impl Table {
    pub fn new(size: usize) -> Table {
        Table {
            size,
            table: vec![Default::default(); 1].into_boxed_slice(),
        }
    }

    pub fn alloc(&mut self) {
        self.table = vec![Default::default(); self.size].into_boxed_slice();
    }

    pub fn clean(&mut self) {
        self.alloc();
    }

    pub fn get_ready(&mut self) {
        self.alloc();
    }

    fn get_index(&self, key: &BitBoard) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let n: usize = s.finish() as usize;
        n % self.size
    }

    pub fn get(&self, key: &BitBoard) -> Option<Score> {
        let index = self.get_index(key);
        if self.table[index].key == *key {
            Some(self.table[index].score)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: BitBoard, score: Score) {
        let index = self.get_index(&key);
        let entry: Entry = Entry { score, key };
        self.table[index] = entry;
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getset() {
        let mut table = Table::new(100);
        let mut bitboard = BitBoard::new();
        bitboard.make_move(0, &crate::board::Player::P1);
        table.set(bitboard, EQUAL);
        let sc = table.get(&bitboard).unwrap();
        assert_eq!(sc, EQUAL);
    }
}
