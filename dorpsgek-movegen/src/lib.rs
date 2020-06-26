/*
 *   This file is part of Dorpsgek.
 *
 *   Dorpsgek is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   Dorpsgek is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *   along with Dorpsgek.  If not, see <http://www.gnu.org/licenses/>.
 */

//#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![warn(warnings, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic, clippy::restriction, clippy::nursery)]
#![allow(
    clippy::integer_arithmetic,
    clippy::float_arithmetic,
    clippy::integer_division,
    clippy::option_expect_used,
    clippy::result_expect_used,
    clippy::shadow_reuse
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions,
    clippy::implicit_return,
    clippy::indexing_slicing
)]

//! Dorpsgek is a chess program.

mod board;
mod chessmove;
mod colour;
mod piece;
mod square;

pub use board::Board;
use chessmove::Move;
use std::mem;
use tinyvec::ArrayVec;

/// Count the number of legal chess positions after N moves.
#[inline]
pub fn perft(mut board: &mut Board, depth: u32) -> u64 {
    if depth == 0 {
        1
    } else {
        let moves: [Move; 256] = unsafe { mem::zeroed() };
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        let mut count = 0;
        for (i, m) in moves.iter().enumerate() {
            let state = board.make(*m);
            count += perft(&mut board, depth - 1);
            board.unmake(*m, state);
        }
        count
    }
}

/// The start point of the program.
fn main() {
    // This is a valid FEN, so it will parse correctly.
    #[allow(clippy::option_unwrap_used)]
    let mut board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    for depth in 1..=5 {
        let mut count = 0;

        let moves: [Move; 256] = unsafe { mem::zeroed() };
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        /*let m = moves[12];
        board.make(m);

        moves.set_len(0);
        board.generate(&mut moves);

        let m = moves[10];
        board.make(m);

        moves.set_len(0);
        board.generate(&mut moves);

        let m = moves[11];
        board.make(m);

        moves.set_len(0);
        board.generate(&mut moves);*/

        for m in moves {
            let state = board.make(m);
            let result = perft(&mut board, depth - 1);
            count += result;
            board.unmake(m, state);
            println!("{}: {}", m, result);
        }

        println!("Perft({}): {}", depth, count);
    }
}
