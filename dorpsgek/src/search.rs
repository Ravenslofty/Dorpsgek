use dorpsgek_movegen::{Board, Move};
use tinyvec::ArrayVec;

use crate::eval::{Eval, EvalState};

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

    fn quiesce(&mut self, board: &Board, mut alpha: i32, beta: i32, eval: &EvalState) -> i32 {
        let eval_int = eval.get(board.side());

        if eval_int >= beta {
            return beta;
        }
        alpha = alpha.max(eval_int);

        board.generate_captures_incremental(|m| {
            self.qnodes += 1;

            let eval = self.eval.update_eval(board, &m, eval);

            // Pre-empt stand pat by skipping moves with bad evaluation.
            // One can think of this as delta pruning, with the delta being zero.
            if eval.get(board.side()) <= alpha {
                return true;
            }

            let board = board.make(m);
            alpha = alpha.max(-self.quiesce(&board, -beta, -alpha, &eval));

            if alpha >= beta {
                alpha = beta;
                return false;
            }
            true
        });

        alpha
    }

    fn search(&mut self, board: &Board, depth: i32, mut alpha: i32, beta: i32, eval: &EvalState) -> i32 {
        if depth <= 0 {
            return self.quiesce(board, alpha, beta, eval);
        }

        const R: i32 = 3;

        if !board.in_check() && depth >= R {
            let board = board.make_null();
            let score = -self.search(&board, depth - 1 - R, -beta, -beta + 1, eval);

            if score >= beta {
                return beta;
            }
        }

        let moves: [Move; 256] = [Move::default(); 256];
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        for m in moves {
            self.nodes += 1;

            let eval = self.eval.update_eval(board, &m, eval);
            let board = board.make(m);
            alpha = alpha.max(-self.search(&board, depth - 1, -beta, -alpha, &eval));

            if alpha >= beta {
                return beta;
            }
        }

        alpha
    }

    pub fn search_root(&mut self, board: &Board, depth: i32) -> i32 {
        let eval = self.eval.eval(board);
        self.search(board, depth, -100_000, 100_000, &eval)
    }

    pub fn nodes(&self) -> u64 {
        self.nodes
    }

    pub fn qnodes(&self) -> u64 {
        self.qnodes
    }
}
