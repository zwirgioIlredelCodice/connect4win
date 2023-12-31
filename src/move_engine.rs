use crate::board::*;
#[cfg(not(target_family = "wasm"))]
use crate::game_database::*;
use crate::r#move::*;
use crate::score::*;
use crate::timer::*;
use crate::transposition_table::*;

pub struct Engine {
    timer: Timer,
    table: Table,
    #[cfg(not(target_family = "wasm"))]
    database: GameDatabase,
}

impl Engine {
    pub fn new(seconds: u64, table_size: usize) -> Engine {
        Engine {
            timer: Timer::new(seconds),
            table: Table::new(table_size),
            #[cfg(not(target_family = "wasm"))]
            database: GameDatabase::new(),
        }
    }

    #[allow(dead_code)]
    pub fn set_time(&mut self, seconds: u64) {
        self.timer.set_duration(seconds);
    }

    #[allow(dead_code)]
    pub fn set_table(&mut self, table_size: usize) {
        self.table.set_size(table_size);
    }

    fn move_sort(board: &mut Board) -> Vec<u8> {
        let v = board.legal_moves();
        let mut mv: Vec<(Score, u8)> = Vec::with_capacity(COL as usize);
        let mut out: Vec<u8> = Vec::with_capacity(COL as usize);

        for m in v {
            board.make_move(m);
            mv.push((board.evaluate(), m));
            board.unmake_move();
        }
        mv.sort_by_key(|mv| mv.0);
        if board.player() == Player::P1 {
            mv.reverse();
        }

        for m in mv {
            out.push(m.1);
        }
        out
    }

    pub fn alpha_beta(
        &mut self,
        board: &mut Board,
        mut alpha: Score,
        mut beta: Score,
        depth: u8,
    ) -> Result<Score, TimeoutError> {
        #[cfg(not(target_family = "wasm"))]
        let saved_score: Option<Score> = if board.nmoves() == PLY {
            self.database.get(&board.bitboard())
        } else if depth >= 1 {
            self.table.get(&board.bitboard())
        } else {
            None
        };

        #[cfg(target_family = "wasm")]
        let saved_score: Option<Score> = if depth >= 1 {
            self.table.get(&board.bitboard())
        } else {
            None
        };

        match saved_score {
            Some(score) => Ok(score),
            None => {
                let moves: Vec<u8>;
                let mut eval: Score;

                if depth == 0 || board.gamestate() != GameState::Open {
                    return Ok(board.evaluate());
                } else {
                    if depth >= 1 {
                        moves = Self::move_sort(board);
                    } else {
                        moves = board.legal_moves();
                    }

                    match board.player() {
                        Player::P1 => {
                            eval = MIN;
                            for m in moves {
                                self.timer.check()?;

                                board.make_move(m);
                                let newscore = self.alpha_beta(board, alpha, beta, depth - 1)?;
                                board.unmake_move();

                                eval = eval.max(newscore);
                                alpha = alpha.max(eval);
                                if alpha > beta {
                                    break;
                                }
                            }
                        }
                        Player::P2 => {
                            eval = MAX;
                            for m in moves {
                                self.timer.check()?;

                                board.make_move(m);
                                let newscore = self.alpha_beta(board, alpha, beta, depth - 1)?;
                                board.unmake_move();

                                eval = eval.min(newscore);
                                beta = beta.min(eval);
                                if alpha > beta {
                                    break;
                                }
                            }
                        }
                    }
                }
                #[cfg(not(target_family = "wasm"))]
                if depth >= 1 && board.nmoves() != PLY {
                    self.table.set(board.bitboard(), eval);
                }
                #[cfg(target_family = "wasm")]
                if depth >= 1 {
                    self.table.set(board.bitboard(), eval);
                }
                Ok(eval)
            }
        }
    }

    fn move_list(
        &mut self,
        board: &mut Board,
        prev_ml: &Vec<Move>,
        depth: u8,
    ) -> Result<Vec<Move>, TimeoutError> {
        let mut alpha = MIN;
        let mut beta = MAX;
        let mut out: Vec<Move> = Vec::with_capacity(COL as usize);

        match board.player() {
            Player::P1 => {
                for m in prev_ml {
                    self.timer.check()?;
                    match m.score().gamestate() {
                        GameState::Open => {
                            board.make_move(m.col());
                            let newscore = self.alpha_beta(board, alpha, beta, depth - 1)?;
                            board.unmake_move();

                            out.push(Move::new(m.col(), m.player(), newscore, depth));
                            alpha = alpha.max(newscore);
                        }
                        _ => {
                            out.push(*m);
                        }
                    }
                }
            }
            Player::P2 => {
                for m in prev_ml {
                    self.timer.check()?;
                    match m.score().gamestate() {
                        GameState::Open => {
                            board.make_move(m.col());
                            let newscore = self.alpha_beta(board, alpha, beta, depth - 1)?;
                            board.unmake_move();

                            out.push(Move::new(m.col(), m.player(), newscore, depth));
                            beta = beta.min(newscore);
                        }
                        _ => {
                            out.push(*m);
                        }
                    }
                }
            }
        }
        Ok(out)
    }

    fn init_move_array(board: &Board) -> Vec<Move> {
        let mut out: Vec<Move> = Vec::with_capacity(COL as usize);
        let cols = board.legal_moves();
        for c in cols {
            out.push(Move::new(c, board.player(), EQUAL, 0));
        }
        out
    }

    fn get_ready(&mut self) {
        self.table.get_ready();
        self.timer.start();
    }

    pub fn iterative_depening(&mut self, board: &Board) -> Move {
        self.get_ready();

        let mut tb: Board = board.clone();
        let mut movelist = Self::init_move_array(&tb);
        let mut bestmove: Move = movelist[0];

        if board.is_empty() {
            return Move::new(3, board.player(), EQUAL, 0);
        }

        let max_depth: u8 = board.free_cells();

        #[cfg(not(target_family = "wasm"))]
        let min_depth = if board.nmoves() <= PLY {
            GOOD_QUERY - board.nmoves() + 2
        } else {
            1
        };
        #[cfg(target_family = "wasm")]
        let min_depth = 1;

        for i in min_depth..max_depth {
            self.table.clean();
            match self.move_list(&mut tb, &movelist, i) {
                Ok(mut ml) => {
                    ml.sort();
                    if board.player() == Player::P1 {
                        ml.reverse();
                    }
                    movelist = ml;
                    bestmove = movelist[0];

                    if bestmove.score().gamestate() != GameState::Open {
                        break;
                    }
                }
                Err(TimeoutError) => {
                    return bestmove;
                }
            }
        }
        bestmove
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ml() {
        let mut b = Board::new();
        let ml = Engine::move_sort(&mut b);
        assert_eq!(ml, [3, 4, 2, 5, 1, 6, 0])
    }

    #[test]
    fn timeengine() {
        use std::time::Instant;

        let mut board = Board::new();
        board.make_move(3);
        let mut e = Engine::new(3, 100_000);
        e.get_ready();

        let start = Instant::now();
        _ = e.alpha_beta(&mut board, MIN, MAX, 12);
        let duration = start.elapsed();
        println!("Time elapsed in alpha_beta is: {:?}", duration);
    }

    #[test]
    fn dtb() {
        let mut board = Board::new();
        let mut e = Engine::new(3, 100_000);
        e.get_ready();
        let s: Score = e.alpha_beta(&mut board, MIN, MAX, GOOD_QUERY).unwrap();
        println!("{}", ScoreMethods::to_string(&s))
    }
}
