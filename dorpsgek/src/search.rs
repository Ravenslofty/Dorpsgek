use tinyvec::ArrayVec;
use dorpsgek_movegen::{Board, Move};

use crate::eval::Eval;

pub struct Search {
    eval: Eval,
    nodes: u64,
    qnodes: u64,
}

impl Default for Search {
    fn default() -> Self {
        Self::new()
    }
}

impl Search {
    pub fn new() -> Self {
        Self {
            eval: Eval::new(),
            nodes: 0,
            qnodes: 0,
        }
    }

    pub fn quiesce(&mut self, board: &Board, mut alpha: f64, beta: f64) -> f64 {
        let eval = self.eval.eval(board);

        if eval > alpha {
            if eval >= beta {
                return beta;
            }
            alpha = eval;
        }

        self.qnodes += 1;

        let moves: [Move; 256] = [Move::default(); 256];
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        for m in moves {
            if !m.is_capture() {
                continue;
            }

            let board = board.make(m);
            let score = -self.quiesce(&board, -beta, -alpha);

            if score > alpha {
                if score >= beta {
                    return beta;
                }
                alpha = score;
            }
        }

        alpha
    }

    pub fn search(&mut self, board: &Board, depth: i32, mut alpha: f64, beta: f64) -> f64 {
        if depth <= 0 {
            return self.quiesce(board, alpha, beta);
        }

        self.nodes += 1;

        let moves: [Move; 256] = [Move::default(); 256];
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        for m in moves {
            let board = board.make(m);

            let score = -self.search(&board, depth - 1, -beta, -alpha);

            if score > alpha {
                if score >= beta {
                    return beta;
                }
                alpha = score;
            }
        }

        alpha
    }

    pub fn nodes(&self) -> u64 {
        self.nodes
    }

    pub fn qnodes(&self) -> u64 {
        self.qnodes
    }
}