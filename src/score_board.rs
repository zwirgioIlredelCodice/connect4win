use crate::board::*;
use std::collections::LinkedList;

#[derive(Debug, Clone, Copy)]
pub struct ScoreSet {
    score: i32,
    p1: i32,
    p2: i32,
}

impl ScoreSet {
    fn fix_score(&mut self) {
        if self.p1 == 0 || self.p2 == 0 {
            if self.p1 > self.p2 {
                self.score = self.p1;
            } else {
                self.score = -self.p2
            }
        } else {
            self.score = 0;
        }
    }
    fn add(&mut self, player: &Player) {
        match player {
            Player::P1 => self.p1 += 1,
            Player::P2 => self.p2 += 1,
        }
        self.fix_score();
    }
    fn sub(&mut self, player: &Player) {
        match player {
            Player::P1 => self.p1 -= 1,
            Player::P2 => self.p2 -= 1,
        }
        self.fix_score();
    }

    pub fn init() -> ScoreSet {
        ScoreSet {
            score: 0,
            p1: 0,
            p2: 0,
        }
    }
}

const NSC: usize = 207; // chissà se è giosto

#[derive(Clone)]
pub struct ScoreBoard {
    total_score: i32,
    scoreboard: [[LinkedList<u32>; COL as usize]; ROW as usize],
    scoresets: [ScoreSet; NSC],
}

impl ScoreBoard {
    pub fn make_move(&mut self, row: usize, col: usize, player: &Player) -> bool {
        let mut delta_score: i32 = 0;
        let mut win: bool = false;

        for i in self.scoreboard[row][col].iter() {
            let mut sc = self.scoresets[*i as usize];
            let ps = sc.score;
            sc.add(player);

            if sc.score == CONNECT as i32{
                win = true;
            }

            delta_score  += sc.score - ps;
        }
        self.total_score += delta_score;
        win
    }

    pub fn unmake_move(&mut self, row: usize, col: usize, player: &Player) {
        let mut delta_score: i32 = 0;
        let mut win: bool = false;

        for i in self.scoreboard[row][col].iter() {
            let mut sc = self.scoresets[*i as usize];
            let ps = sc.score;
            sc.sub(player);

            if sc.score == CONNECT as i32{
                win = true;
            }

            delta_score  += sc.score - ps;
        }
        self.total_score += delta_score;
    }

    pub fn init() -> ScoreBoard {
        let mut sca: [ScoreSet; NSC] = [ScoreSet::init(); NSC];

        let mut sbt: [[LinkedList<u32>; COL as usize]; ROW as usize] = [
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
            [
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
                LinkedList::new(),
            ],
        ];
        let mut n: u32 = 0;
        for i in 0..ROW {
            for j in 0..COL {
                
                if ROW - i >= CONNECT { // orizontal --
                    for k in i..(i+CONNECT) {
                        sbt[k as usize][j as usize].push_back(n);
                    }
                    n += 1;
                }
                
                if COL - j >= CONNECT { // vertical |
                    for k in j..(j+CONNECT) {
                        sbt[i as usize][k as usize].push_back(n);
                    }
                    n += 1;
                }
                
                if (ROW - i >= CONNECT) && (COL - j >= CONNECT) { // diagonal \
                    let mut kk = j;
                    for k in i..(i+CONNECT) {
                        sbt[k as usize][kk as usize].push_back(n);
                        kk += 1;
                    }
                    n += 1;
                }
                
                if (ROW - i >= CONNECT) && (j + 1 >= CONNECT) { // diagonal /
                    let mut kk = j;
                    for k in i..(i+CONNECT) {
                        sbt[k as usize][kk as usize].push_back(n);
                        if kk > 0{
                            kk -= 1;
                        }
                    }
                    n += 1;
                }
            }
        }

        let sb = sbt;
        ScoreBoard {
            total_score: 0,
            scoreboard: sb,
            scoresets: sca,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ScoreBoard;


    #[test]
    fn lel () {
        let mut ss: ScoreBoard = ScoreBoard::init();
        ss.make_move(0, 0, &crate::board::Player::P1);
        ss.make_move(0, 6, &crate::board::Player::P2);
        assert_eq!(ss.total_score, 0)
        
    }
}