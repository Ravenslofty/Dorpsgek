use dorpsgek_movegen::{Board, Move};
use tinyvec::ArrayVec;

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

    pub fn quiesce(&mut self, board: &Board, mut alpha: i32, beta: i32) -> i32 {
        let eval = self.eval.eval(board);

        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }

        let moves: [Move; 256] = [Move::default(); 256];
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate_captures(&mut moves);

        for m in moves {
            if !m.is_capture() {
                continue;
            }

            self.qnodes += 1;

            let board = board.make(m);
            let score = -self.quiesce(&board, -beta, -alpha);

            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    pub fn search(&mut self, board: &Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
        if depth <= 0 {
            return self.quiesce(board, alpha, beta);
        }

        let moves: [Move; 256] = [Move::default(); 256];
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        for m in moves {
            self.nodes += 1;

            let board = board.make(m);
            let score = -self.search(&board, depth - 1, -beta, -alpha);

            if score >= beta {
                return beta;
            }
            if score > alpha {
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
