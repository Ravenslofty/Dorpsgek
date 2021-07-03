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

#[cfg(test)]
mod perft {
    use crate::{Board, perft};

    #[test]
    fn startpos_1() {
        let startpos = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 20);
    }

    #[test]
    fn startpos_2() {
        let startpos = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 2), 400);
    }

    #[test]
    fn startpos_3() {
        let startpos = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 3), 8_902);
    }

    #[test]
    fn startpos_4() {
        let startpos = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 4), 197_281);
    }

    #[test]
    fn startpos_5() {
        let startpos = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 5), 4_865_609);
    }

    #[test]
    fn startpos_6() {
        let startpos = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 6), 119_060_324);
    }

    #[test]
    fn kiwipete_1() {
        let startpos = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 48);
    }

    #[test]
    fn kiwipete_2() {
        let startpos = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  0 1").unwrap();
        assert_eq!(perft(&startpos, 2), 2039);
    }

    #[test]
    fn kiwipete_3() {
        let startpos = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  0 1").unwrap();
        assert_eq!(perft(&startpos, 3), 97_862);
    }

    #[test]
    fn kiwipete_4() {
        let startpos = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  0 1").unwrap();
        assert_eq!(perft(&startpos, 4), 4_085_603);
    }

    #[test]
    fn kiwipete_5() {
        let startpos = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  0 1").unwrap();
        assert_eq!(perft(&startpos, 5), 193_690_690);
    }

    #[test]
    fn cpwpos3_1() {
        let startpos = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 14);
    }

    #[test]
    fn cpwpos3_2() {
        let startpos = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 2), 191);
    }

    #[test]
    fn cpwpos3_3() {
        let startpos = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 3), 2_812);
    }

    #[test]
    fn cpwpos3_4() {
        let startpos = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 4), 43_238);
    }

    #[test]
    fn cpwpos3_5() {
        let startpos = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 5), 674_624);
    }

    #[test]
    fn cpwpos3_6() {
        let startpos = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 6), 11_030_083);
    }

    #[test]
    fn cpwpos4_1() {
        let startpos = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 6);
    }

    #[test]
    fn cpwpos4_2() {
        let startpos = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 2), 264);
    }

    #[test]
    fn cpwpos4_3() {
        let startpos = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 3), 9_467);
    }

    #[test]
    fn cpwpos4_4() {
        let startpos = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 4), 422_333);
    }

    #[test]
    fn cpwpos4_5() {
        let startpos = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 5), 15_833_292);
    }

    #[test]
    fn cpwpos5_1() {
        let startpos = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(perft(&startpos, 1), 44);
    }

    #[test]
    fn cpwpos5_2() {
        let startpos = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(perft(&startpos, 2), 1_486);
    }

    #[test]
    fn cpwpos5_3() {
        let startpos = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(perft(&startpos, 3), 62_379);
    }

    #[test]
    fn cpwpos5_4() {
        let startpos = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(perft(&startpos, 4), 2_103_487);
    }

    #[test]
    fn cpwpos5_5() {
        let startpos = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        assert_eq!(perft(&startpos, 5), 89_941_194);
    }

    #[test]
    fn cpwpos6_1() {
        let startpos = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ").unwrap();
        assert_eq!(perft(&startpos, 1), 46);
    }

    #[test]
    fn cpwpos6_2() {
        let startpos = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ").unwrap();
        assert_eq!(perft(&startpos, 2), 2_079);
    }

    #[test]
    fn cpwpos6_3() {
        let startpos = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ").unwrap();
        assert_eq!(perft(&startpos, 3), 89_890);
    }

    #[test]
    fn cpwpos6_4() {
        let startpos = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ").unwrap();
        assert_eq!(perft(&startpos, 4), 3_894_594);
    }

    #[test]
    fn cpwpos6_5() {
        let startpos = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ").unwrap();
        assert_eq!(perft(&startpos, 5), 164_075_551);
    }
}