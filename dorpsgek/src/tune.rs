use std::convert::TryInto;

use dorpsgek_movegen::{Board, Colour, Move, Piece, Square};
use rand::prelude::*;
use revad::tape::{Tape, Var};
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
    pub fn from_tuning_weights(weights: &'a [Var<'a>]) -> Self {
        Self {
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
        }
    }

    pub fn gradient(&'a self, board: &Board, tape: &'a Tape) -> Var<'a> {
        let mut score = EvalState::new(tape);

        for piece in board.pieces() {
            let square = board.square_of_piece(piece);
            score.add_piece(self, board.piece_from_bit(piece), square, piece.colour());
        }

        score.get(self, tape, board.side()).abs()
    }
}

pub struct Tune<'a> {
    learning_rate: f64,
    weights: [Var<'a>; 786],
}

impl<'a> Tune<'a> {
    pub fn new(tape: &'a Tape) -> Self {
        let weights = [
            // Midgame Material
            tape.var(100_f64), tape.var(300_f64), tape.var(300_f64), tape.var(500_f64), tape.var(900_f64),  tape.var(0_f64),
            // Endgame Material
            tape.var(100_f64), tape.var(300_f64), tape.var(300_f64), tape.var(500_f64),  tape.var(900_f64),  tape.var(0_f64),
            // Midgame PST
                // Pawns
                tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),   tape.var(random()),   tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),   tape.var(random()),
                // Knights
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),
                // Bishops
                tape.var(random()),   tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),   tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                // Rooks
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()), tape.var(random()),   tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),   tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()),   tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),
                // Queens
                tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()),   tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()), tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),
                tape.var(random()),   tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                // Kings
                tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),   tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),   tape.var(random()), tape.var(random()), tape.var(random()),   tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),   tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),   tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),   tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),
            // Endgame PST
                // Pawns
                tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),
                tape.var(random()),   tape.var(random()),  tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),
                tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),
                // Knights
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                // Bishops
                tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),   tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()), tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),
                tape.var(random()),   tape.var(random()),  tape.var(random()),   tape.var(random()), tape.var(random()),  tape.var(random()),   tape.var(random()),   tape.var(random()),
                tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()), tape.var(random()),   tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()),
                // Rooks
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),   tape.var(random()),   tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),   tape.var(random()),   tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),   tape.var(random()), tape.var(random()),
                // Queens
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()), tape.var(random()),  tape.var(random()),   tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
                // Kings
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),  tape.var(random()),   tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()), tape.var(random()),
                tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()),
                tape.var(random()), tape.var(random()),   tape.var(random()),  tape.var(random()),  tape.var(random()),   tape.var(random()),  tape.var(random()), tape.var(random()),
                tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()), tape.var(random()),
            // Phase
            tape.var(0_f64), tape.var(1_f64), tape.var(1_f64), tape.var(2_f64), tape.var(4_f64), tape.var(0_f64),
        ];

        Self {
            learning_rate: 0.7,
            weights
        }
    }

    pub fn tune(&mut self, tape: &'a Tape) {
        let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();

        for n in 0..5_000 {
            print!("Iter {:>5}: ", n);
            print!("piece values: [");
            for w in &self.weights[0..5] {
                print!("{:>4.0} ", w.value());
            }
            print!("] [");
            for w in &self.weights[6..11] {
                print!("{:>4.0} ", w.value());
            }
            print!("]; ");

            // Make a random legal move on the board
            let moves: [Move; 256] = [Move::default(); 256];
            let mut moves = ArrayVec::from(moves);
            moves.set_len(0);
            board.generate(&mut moves);
            let m = *moves.iter().choose(&mut thread_rng()).unwrap();
            let mut board = board.make(m);

            // Initialise the search.
            let mut weights = Vec::new();
            for w in &mut self.weights {
                weights.push(w.value() as i32);
            }
            let mut s = Search::new();
            s.from_tuning_weights(&weights);

            // Then collect temporal differences.
            let eval = Eval::from_tuning_weights(&self.weights);
            let mut grads = [None, None, None, None, None, None, None, None, None, None, None, None];
            let mut positions = 0;

            for grad in &mut grads {
                let mut pv = ArrayVec::new();
                pv.set_len(0);
                let _score = s.search_root(&board, 2, &mut pv);

                positions += 1;

                if pv.is_empty() {
                    if board.side() == Colour::White {
                        *grad = Some(tape.var(-10_000.0));
                    } else {
                        *grad = Some(tape.var(10_000.0));
                    }
                    break;
                }

                let mut pv_board = board.clone();
                for m in pv {
                    pv_board = pv_board.make(m);
                }

                *grad = Some(eval.gradient(&pv_board, tape));

                board = board.make(pv[0]);
            }

            let mut sum_diff = tape.var(0.0);

            for n in 1..positions {
                sum_diff = sum_diff + (grads[n].unwrap() - grads[n - 1].unwrap()) * tape.var(self.learning_rate.powi(n as i32));
            }

            println!("err: {:<5.1}", sum_diff.value());

            let grad = sum_diff.grad();

            for weight in &mut self.weights {
                *weight = tape.var(weight.value() + 0.1*grad.wrt(*weight));
            }
        }

        print!("mat_mg: [");
        for w in &self.weights[0..6] {
            print!("{:>4.0}, ", w.value());
        }
        println!("],");
        print!("mat_eg: [");
        for w in &self.weights[6..12] {
            print!("{:>4.0}, ", w.value());
        }
        println!("],");
        println!("pst_mg: [");
        println!("// Pawns");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[12+rank*8..20+rank*8] {
                print!("{:>4.0}, ", w.value());
            }
            println!();
        }
        println!("    ],");
        println!("// Knights");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[75+rank*8..83+rank*8] {
                print!("{:>4.0}, ", w.value());
            }
            println!();
        }
        println!("    ],");
        println!("// Bishops");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[139+rank*8..147+rank*8] {
                print!("{:>4.0}, ", w.value());
            }
            println!();
        }
        println!("    ],");
        println!("// Rooks");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[203+rank*8..211+rank*8] {
                print!("{:>4.0}, ", w.value());
            }
            println!();
        }
        println!("    ],");
        println!("// Queens");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[267+rank*8..275+rank*8] {
                print!("{:>4.0}, ", w.value());
            }
            println!();
        }
        println!("    ],");
        println!("// Kings");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[331+rank*8..339+rank*8] {
                print!("{:>4.0}, ", w.value());
            }
            println!();
        }
        println!("    ],");
        println!("]");
    }
}
