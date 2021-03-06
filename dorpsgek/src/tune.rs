use std::{convert::TryInto, io::Read};

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

    pub fn get(&self, tape: &'a Tape, colour: Colour) -> Var<'a> {
        let score = tape.var(1.0 / 24.0) * ((self.pst_mg * self.phase) + (self.pst_eg * (tape.var(24.0) - self.phase)));
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
    pub fn from_tuning_weights(tape: &'a Tape, weights: &'a [Var<'a>]) -> Self {
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
            phase: [tape.var(0.0), tape.var(1.0), tape.var(1.0), tape.var(2.0), tape.var(4.0), tape.var(0.0)]
        }
    }

    pub fn gradient(&'a self, board: &Board, tape: &'a Tape) -> Var<'a> {
        let mut score = EvalState::new(tape);

        for piece in board.pieces() {
            let square = board.square_of_piece(piece);
            score.add_piece(self, board.piece_from_bit(piece), square, piece.colour());
        }

        (tape.var(0.00255) * score.get(tape, board.side())).tanh()
    }
}

pub struct Tune<'a> {
    learning_rate: f64,
    weights: [Var<'a>; 780],
    m_t: [f64; 780],
    v_t: [f64; 780],
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
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Knights
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Bishops
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Rooks
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Queens
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Kings
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
            // Endgame PST
                // Pawns
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Knights
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Bishops
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Rooks
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Queens
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                // Kings
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
                tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0), tape.var(0.0),
            // Phase
            //tape.var(0_f64), tape.var(1_f64), tape.var(1_f64), tape.var(2_f64), tape.var(4_f64), tape.var(0_f64),
        ];

        Self {
            learning_rate: 0.7,
            weights,
            m_t: [0.0; 780],
            v_t: [0.0; 780],
        }
    }

    pub fn get_state(&self) -> ([f64; 780], [f64; 780], [f64; 780]) {
        let weights = self.weights.iter().map(|var| var.value()).collect::<Vec<_>>();
        (weights.as_slice().try_into().unwrap(), self.m_t, self.v_t)
    }

    pub fn set_state(&mut self, tape: &'a Tape, weights: &[f64], m_t: &[f64], v_t: &[f64]) {
        for i in 0..self.weights.len() {
            self.weights[i] = tape.var(weights[i]);
            self.m_t[i] = m_t[i];
            self.v_t[i] = v_t[i];
        }
    }

    pub fn dump(&self) {
        // Discover and remove means
        let mut mean_mg = [0.0; 6];
        let mut mean_eg = [0.0; 6];

        mean_mg[0] = self.weights[12..75].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_mg[1] = self.weights[75..139].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_mg[2] = self.weights[139..203].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_mg[3] = self.weights[203..267].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_mg[4] = self.weights[267..331].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_mg[5] = self.weights[331..395].iter().map(|v| v.value()).sum::<f64>() / 64.0;

        mean_eg[0] = self.weights[395..459].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_eg[1] = self.weights[459..523].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_eg[2] = self.weights[523..587].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_eg[3] = self.weights[587..651].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_eg[4] = self.weights[651..715].iter().map(|v| v.value()).sum::<f64>() / 64.0;
        mean_eg[5] = self.weights[715..779].iter().map(|v| v.value()).sum::<f64>() / 64.0;

        print!("mat_mg: [");
        for (index, w) in self.weights[0..6].iter().enumerate() {
            print!("{:>4.0}, ", w.value() - mean_mg[index]);
        }
        println!("],");
        print!("mat_eg: [");
        for (index, w) in self.weights[6..12].iter().enumerate() {
            print!("{:>4.0}, ", w.value() - mean_eg[index]);
        }
        println!("],");
        println!("pst_mg: [");
        println!("// Pawns");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[12+rank*8..20+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_mg[0]);
            }
            println!();
        }
        println!("    ],");
        println!("// Knights");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[75+rank*8..83+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_mg[1]);
            }
            println!();
        }
        println!("    ],");
        println!("// Bishops");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[139+rank*8..147+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_mg[2]);
            }
            println!();
        }
        println!("    ],");
        println!("// Rooks");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[203+rank*8..211+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_mg[3]);
            }
            println!();
        }
        println!("    ],");
        println!("// Queens");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[267+rank*8..275+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_mg[4]);
            }
            println!();
        }
        println!("    ],");
        println!("// Kings");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[331+rank*8..339+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_mg[5]);
            }
            println!();
        }
        println!("    ],");
        println!("],");
        println!("pst_eg: [");
        println!("// Pawns");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[395+rank*8..403+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_eg[0]);
            }
            println!();
        }
        println!("    ],");
        println!("// Knights");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[459+rank*8..467+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_eg[1]);
            }
            println!();
        }
        println!("    ],");
        println!("// Bishops");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[523+rank*8..531+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_eg[2]);
            }
            println!();
        }
        println!("    ],");
        println!("// Rooks");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[587+rank*8..595+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_eg[3]);
            }
            println!();
        }
        println!("    ],");
        println!("// Queens");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[651+rank*8..659+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_eg[4]);
            }
            println!();
        }
        println!("    ],");
        println!("// Kings");
        println!("    [");
        for rank in 0_usize..8 {
            print!("        ");
            for w in &self.weights[715+rank*8..723+rank*8] {
                print!("{:>4.0}, ", w.value() - mean_eg[5]);
            }
            println!();
        }
        println!("    ],");
        println!("],");
    }

    pub fn tune(&mut self, tape: &'a Tape, boards: &[Board], epoch: i32) {
        for n in 1..=100 {
            let mut mean_mg = [0.0; 6];
            let mut mean_eg = [0.0; 6];

            mean_mg[0] = self.weights[12..75].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_mg[1] = self.weights[75..139].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_mg[2] = self.weights[139..203].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_mg[3] = self.weights[203..267].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_mg[4] = self.weights[267..331].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_mg[5] = self.weights[331..395].iter().map(|v| v.value()).sum::<f64>() / 64.0;

            mean_eg[0] = self.weights[395..459].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_eg[1] = self.weights[459..523].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_eg[2] = self.weights[523..587].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_eg[3] = self.weights[587..651].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_eg[4] = self.weights[651..715].iter().map(|v| v.value()).sum::<f64>() / 64.0;
            mean_eg[5] = self.weights[715..779].iter().map(|v| v.value()).sum::<f64>() / 64.0;

            print!("Iter {:>5}: ", epoch*100 + n);
            print!("piece values: [");
            for (index, w) in self.weights[0..5].iter().enumerate() {
                print!("{:>4.0} ", w.value() - mean_mg[index]);
            }
            print!("] [");
            for (index, w) in self.weights[6..11].iter().enumerate() {
                print!("{:>4.0} ", w.value() - mean_eg[index]);
            }
            print!("]; ");

            let board = boards.iter().choose(&mut thread_rng()).unwrap();

            // Make a random legal move on the board
            let moves: [Move; 256] = [Move::default(); 256];
            let mut moves = ArrayVec::from(moves);
            moves.set_len(0);
            board.generate(&mut moves);
            let m = *moves.iter().choose(&mut thread_rng()).unwrap();
            let board = board.make(m);

            // Initialise the search.
            let mut weights = Vec::new();
            for w in &mut self.weights {
                weights.push(w.value() as i32);
            }
            let mut s = Search::new();
            s.from_tuning_weights(&weights);

            // Then collect temporal differences.
            let eval = Eval::from_tuning_weights(tape, &self.weights);

            let mut scores = Vec::new();
            let mut diffs = Vec::new();

            let mut last_pv = ArrayVec::new();
            last_pv.set_len(0);

            scores.push(eval.gradient(&board, tape));
            diffs.push(tape.var(0.0));

            for position in 0..12 {
                let mut pv = ArrayVec::new();
                pv.set_len(0);
                let score = s.search_root(&board, 2, &mut pv);

                let mut pv_board = board.clone();
                for m in pv {
                    pv_board = pv_board.make(m);
                }

                if pv.is_empty() {
                    if score == 0 {
                        scores.push(tape.var(0.0));
                    } else if score > 0 {
                        scores.push(tape.var(1.0));
                    } else {
                        scores.push(tape.var(-1.0));
                    }
                } else {
                    scores.push(eval.gradient(&pv_board, tape));
                }

                let diff = scores[scores.len() - 2] - scores[scores.len() - 1];
                if diff.value() > 0.0 && !last_pv.is_empty() && pv[0] != last_pv[1] {
                    // Last move was a blunder; don't learn from it.
                    diffs.push(tape.var(0.0));
                } else {
                    diffs.push(diff);
                }

                if pv.is_empty() {
                    break;
                }

                last_pv = pv;
            }

            let mut sum1 = tape.var(0.0);

            for n in 1..scores.len() {
                let mut sum2 = tape.var(0.0);
                for (m, diff) in diffs.iter().enumerate().take(scores.len()).skip(n) {
                    sum2 = sum2 + tape.var(diff.value() * self.learning_rate.powi((m - n) as i32));
                }
                sum1 = sum1 + sum2 * scores[n];
            }

            print!("err: {} ", sum1.value());
            println!();

            let grad = sum1.grad();

            const BETA1: f64 = 0.9;
            const BETA2: f64 = 0.999;
            const EPSILON: f64 = 1e-8;

            for (index, weight) in self.weights.iter_mut().enumerate().skip(12) {
                if !grad.wrt(*weight).is_nan() {
                    self.m_t[index] = BETA1.mul_add(self.m_t[index], (1.0 - BETA1)*grad.wrt(*weight));
                    self.v_t[index] = BETA2.mul_add(self.v_t[index], (1.0 - BETA2)*grad.wrt(*weight)*grad.wrt(*weight));

                    let m_t = self.m_t[index] / (1.0 - BETA1.powi(n as i32));
                    let v_t = self.v_t[index] / (1.0 - BETA2.powi(n as i32));

                    *weight = tape.var(weight.value() - (100.0 * grad.wrt(*weight)) / (v_t.sqrt() + EPSILON) * m_t);
                }
            }
        }
    }
}
