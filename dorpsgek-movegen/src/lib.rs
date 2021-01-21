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
//#![forbid(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(clippy::pedantic, clippy::nursery, clippy::perf)]
#![feature(const_fn)]

//! Dorpsgek is a chess program.

mod board;
mod chessmove;
mod colour;
mod piece;
mod square;

pub use board::Board;
pub use chessmove::{Move, MoveType};
pub use square::Square;
use std::mem;
use tinyvec::ArrayVec;

/// Count the number of legal chess positions after N moves.
#[inline]
#[must_use]
pub fn perft(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        1
    } else {
        let moves: [Move; 256] = unsafe { mem::zeroed() };
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        let mut count = 0;
        for m in moves {
            let board = board.make(m);
            count += perft(&board, depth - 1);
        }
        count
    }
}
