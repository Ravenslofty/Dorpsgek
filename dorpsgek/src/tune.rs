use std::convert::TryInto;

use dorpsgek_movegen::{Board, Colour, Move, Piece, Square};
use rand::prelude::*;
use revad::tape::{Grad, Tape, Var};
use tinyvec::ArrayVec;

use crate::Search;

#[derive(Clone)]
pub struct EvalState<'a> {
    pst_mg: Var<'a>,
    pst_eg: Var<'a>,
    phase: Var<'a>
}

impl<'a> EvalState<'a> {
    pub fn new(t: &'a Tape) -> Self {
        Self {
            pst_mg: t.var(0.0),
            pst_eg: t.var(0.0),
            phase: t.var(0.0),
        }
    }

    pub fn get(&self, eval: &'a Eval, tape: &'a Tape, colour: Colour) -> Var<'a> {
        let max_phase = tape.var(16.0)*eval.phase[0] + tape.var(4.0)*eval.phase[1] + tape.var(4.0)*eval.phase[2] + tape.var(4.0)*eval.phase[3] + tape.var(2.0)*eval.phase[4];

        let score = ((self.pst_mg * self.phase) + (self.pst_eg * (max_phase - self.phase))) / max_phase;
        if colour == Colour::White {
            score
        } else {
            -score
        }
    }

    pub fn add_piece(&mut self, eval: &'a Eval, piece: Piece, square: Square, colour: Colour) {
        if colour == Colour::White {
            self.pst_mg = self.pst_mg + eval.pst_mg[piece as usize][square.into_inner() as usize] + eval.mat_mg[piece as usize];
            self.pst_eg = self.pst_eg + eval.pst_eg[piece as usize][square.into_inner() as usize] + eval.mat_eg[piece as usize];
        } else {
            self.pst_mg = self.pst_mg - eval.pst_mg[piece as usize][square.flip().into_inner() as usize] - eval.mat_mg[piece as usize];
            self.pst_eg = self.pst_eg - eval.pst_eg[piece as usize][square.flip().into_inner() as usize] - eval.mat_eg[piece as usize];
        }
        self.phase = self.phase + eval.phase[piece as usize];
    }
}

pub struct Eval<'a> {
    pub mat_mg: [Var<'a>; 6],
    pub mat_eg: [Var<'a>; 6],
    pub pst_mg: [[Var<'a>; 64]; 6],
    pub pst_eg: [[Var<'a>; 64]; 6],
    pub phase: [Var<'a>; 6],
}

impl<'a> Eval<'a> {
    pub fn gradient(board: &Board, tape: &'a Tape, weights: &'a [Var<'a>]) -> (f64, Grad) {
        let weights = Self {
            mat_mg: weights[0..=5].try_into().unwrap(),
            mat_eg: weights[6..=11].try_into().unwrap(),
            pst_mg: [
                // Pawn
                weights[11..75].try_into().unwrap(),
                // Knight
                weights[75..139].try_into().unwrap(),
                // Bishop
                weights[139..203].try_into().unwrap(),
                // Rook
                weights[203..267].try_into().unwrap(),
                // Queen
                weights[267..331].try_into().unwrap(),
                // King
                weights[331..395].try_into().unwrap()
            ],
            pst_eg: [
                // Pawn
                weights[395..459].try_into().unwrap(),
                // Knight
                weights[459..523].try_into().unwrap(),
                // Bishop
                weights[523..587].try_into().unwrap(),
                // Rook
                weights[587..651].try_into().unwrap(),
                // Queen
                weights[651..715].try_into().unwrap(),
                // King
                weights[715..779].try_into().unwrap()
            ],
            phase: weights[779..785].try_into().unwrap()
        };

        let mut score = EvalState::new(tape);

        for piece in board.pieces() {
            let square = board.square_of_piece(piece);
            score.add_piece(&weights, board.piece_from_bit(piece), square, piece.colour());
        }

        let score = score.get(&weights, tape, board.side());
        (score.value(), score.abs().grad())
    }
}

pub struct Tune<'a> {
    learning_rate: f64,
    weights: [Var<'a>; 786],
}

impl<'a> Tune<'a> {
    pub fn new(tape: &'a Tape) -> Self {
        let mut weights = [tape.var(0.0); 786];

        for w in &mut weights {
            *w = tape.var(random());
        }

        w[0] = 100.0;
        w[1] =

        Self {
            learning_rate: 0.7,
            weights
        }
    }

    pub fn tune(&mut self, tape: &'a Tape) {
        let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();

        'main_loop: for n in 0..5_000 {
            println!("Iter {}", n);
            print!("Piece values: ");
            for w in &self.weights[0..6] {
                print!("{:.0} ", w.value());
            }
            println!();

            // Make a random legal move on the board
            let moves: [Move; 256] = [Move::default(); 256];
            let mut moves = ArrayVec::from(moves);
            moves.set_len(0);
            board.generate(&mut moves);
            let m = *moves.iter().choose(&mut thread_rng()).unwrap();
            println!("Trying move {}", m);
            let mut board = board.make(m);

            // Initialise the search.
            let mut weights = Vec::new();
            for w in &mut self.weights {
                weights.push(w.value() as i32);
            }
            let mut s = Search::new();
            s.from_tuning_weights(&weights);

            // Then collect temporal differences.
            let mut scores = [0.0; 12];
            let mut grads = [None, None, None, None, None, None, None, None, None, None, None, None];

            for n in 0..12 {
                let mut pv = ArrayVec::new();
                pv.set_len(0);
                let _score = s.search_root(&board, 4, &mut pv);

                if pv.is_empty() {
                    continue 'main_loop;
                }

                let mut pv_board = board.clone();
                for m in pv {
                    pv_board = pv_board.make(m);
                }
                let (score, g) = Eval::gradient(&pv_board, tape, &self.weights);
                scores[n] = score;
                grads[n] = Some(g);
                board = board.make(pv[0]);
            }

            let mut diffs = [0.0; 12];
            let mut sum_diff = 0.0;

            for n in 1..12 {
                diffs[n] = scores[n] - scores[n - 1];
                sum_diff += scores[n] - scores[n - 1];
            }

            println!("error: {}", sum_diff);

            for (_index, weight) in self.weights.iter_mut().enumerate() {
                let mut sum = 0.0;
                for (n, grad) in grads.iter_mut().enumerate() {
                    let grad = grad.as_ref().unwrap().wrt(*weight);
                    sum += grad * self.learning_rate.powi(n as i32);
                }
                *weight = tape.var(weight.value() + sum);
            }
        }
    }
}
