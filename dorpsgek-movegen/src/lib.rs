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

pub use board::{Board, PieceIndex};
pub use chessmove::{Move, MoveType};
pub use colour::Colour;
pub use piece::Piece;
pub use square::Square;
use tinyvec::ArrayVec;

/// Count the number of legal chess positions after N moves.
#[inline]
#[must_use]
pub fn perft(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        1
    } else if depth == 1 {
        let moves: [Move; 256] = [Move::default(); 256];
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);
        moves.len() as u64
    } else {
        let moves: [Move; 256] = [Move::default(); 256];
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
    use crate::{perft, Board};

    #[test]
    fn perft_test1() {
        let startpos =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 20);
        assert_eq!(perft(&startpos, 2), 400);
        assert_eq!(perft(&startpos, 3), 8902);
        assert_eq!(perft(&startpos, 4), 197_281);
        assert_eq!(perft(&startpos, 5), 4_865_609);
        assert_eq!(perft(&startpos, 6), 119_060_324);
    }

    #[test]
    fn perft_test2() {
        let startpos =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
                .unwrap();
        assert_eq!(perft(&startpos, 1), 48);
        assert_eq!(perft(&startpos, 2), 2039);
        assert_eq!(perft(&startpos, 3), 97862);
        assert_eq!(perft(&startpos, 4), 4_085_603);
        assert_eq!(perft(&startpos, 5), 193_690_690);
    }

    #[test]
    fn perft_test3() {
        let startpos = Board::from_fen("4k3/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 15);
        assert_eq!(perft(&startpos, 2), 66);
        assert_eq!(perft(&startpos, 3), 1197);
        assert_eq!(perft(&startpos, 4), 7059);
        assert_eq!(perft(&startpos, 5), 133987);
        assert_eq!(perft(&startpos, 6), 764643);
    }

    #[test]
    fn perft_test4() {
        let startpos = Board::from_fen("4k3/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 16);
        assert_eq!(perft(&startpos, 2), 71);
        assert_eq!(perft(&startpos, 3), 1287);
        assert_eq!(perft(&startpos, 4), 7626);
        assert_eq!(perft(&startpos, 5), 145232);
        assert_eq!(perft(&startpos, 6), 846648);
    }

    #[test]
    fn perft_test5() {
        let startpos = Board::from_fen("4k2r/8/8/8/8/8/8/4K3 w k - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 75);
        assert_eq!(perft(&startpos, 3), 459);
        assert_eq!(perft(&startpos, 4), 8290);
        assert_eq!(perft(&startpos, 5), 47635);
        assert_eq!(perft(&startpos, 6), 899442);
    }

    #[test]
    fn perft_test6() {
        let startpos = Board::from_fen("r3k3/8/8/8/8/8/8/4K3 w q - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 80);
        assert_eq!(perft(&startpos, 3), 493);
        assert_eq!(perft(&startpos, 4), 8897);
        assert_eq!(perft(&startpos, 5), 52710);
        assert_eq!(perft(&startpos, 6), 1001523);
    }

    #[test]
    fn perft_test7() {
        let startpos = Board::from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 26);
        assert_eq!(perft(&startpos, 2), 112);
        assert_eq!(perft(&startpos, 3), 3189);
        assert_eq!(perft(&startpos, 4), 17945);
        assert_eq!(perft(&startpos, 5), 532933);
        assert_eq!(perft(&startpos, 6), 2788982);
    }

    #[test]
    fn perft_test8() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/4K3 w kq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 130);
        assert_eq!(perft(&startpos, 3), 782);
        assert_eq!(perft(&startpos, 4), 22180);
        assert_eq!(perft(&startpos, 5), 118882);
        assert_eq!(perft(&startpos, 6), 3517770);
    }

    #[test]
    fn perft_test9() {
        let startpos = Board::from_fen("8/8/8/8/8/8/6k1/4K2R w K - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 12);
        assert_eq!(perft(&startpos, 2), 38);
        assert_eq!(perft(&startpos, 3), 564);
        assert_eq!(perft(&startpos, 4), 2219);
        assert_eq!(perft(&startpos, 5), 37735);
        assert_eq!(perft(&startpos, 6), 185867);
    }

    #[test]
    fn perft_test10() {
        let startpos = Board::from_fen("8/8/8/8/8/8/1k6/R3K3 w Q - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 15);
        assert_eq!(perft(&startpos, 2), 65);
        assert_eq!(perft(&startpos, 3), 1018);
        assert_eq!(perft(&startpos, 4), 4573);
        assert_eq!(perft(&startpos, 5), 80619);
        assert_eq!(perft(&startpos, 6), 413018);
    }

    #[test]
    fn perft_test11() {
        let startpos = Board::from_fen("4k2r/6K1/8/8/8/8/8/8 w k - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 32);
        assert_eq!(perft(&startpos, 3), 134);
        assert_eq!(perft(&startpos, 4), 2073);
        assert_eq!(perft(&startpos, 5), 10485);
        assert_eq!(perft(&startpos, 6), 179869);
    }

    #[test]
    fn perft_test12() {
        let startpos = Board::from_fen("r3k3/1K6/8/8/8/8/8/8 w q - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 49);
        assert_eq!(perft(&startpos, 3), 243);
        assert_eq!(perft(&startpos, 4), 3991);
        assert_eq!(perft(&startpos, 5), 20780);
        assert_eq!(perft(&startpos, 6), 367724);
    }

    #[test]
    fn perft_test13() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 26);
        assert_eq!(perft(&startpos, 2), 568);
        assert_eq!(perft(&startpos, 3), 13744);
        assert_eq!(perft(&startpos, 4), 314346);
        assert_eq!(perft(&startpos, 5), 7594526);
        assert_eq!(perft(&startpos, 6), 179862938);
    }

    #[test]
    fn perft_test14() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 567);
        assert_eq!(perft(&startpos, 3), 14095);
        assert_eq!(perft(&startpos, 4), 328965);
        assert_eq!(perft(&startpos, 5), 8153719);
        assert_eq!(perft(&startpos, 6), 195629489);
    }

    #[test]
    fn perft_test15() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/2R1K2R w Kkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 548);
        assert_eq!(perft(&startpos, 3), 13502);
        assert_eq!(perft(&startpos, 4), 312835);
        assert_eq!(perft(&startpos, 5), 7736373);
        assert_eq!(perft(&startpos, 6), 184411439);
    }

    #[test]
    fn perft_test16() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 547);
        assert_eq!(perft(&startpos, 3), 13579);
        assert_eq!(perft(&startpos, 4), 316214);
        assert_eq!(perft(&startpos, 5), 7878456);
        assert_eq!(perft(&startpos, 6), 189224276);
    }

    #[test]
    fn perft_test17() {
        let startpos = Board::from_fen("1r2k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 26);
        assert_eq!(perft(&startpos, 2), 583);
        assert_eq!(perft(&startpos, 3), 14252);
        assert_eq!(perft(&startpos, 4), 334705);
        assert_eq!(perft(&startpos, 5), 8198901);
        assert_eq!(perft(&startpos, 6), 198328929);
    }

    #[test]
    fn perft_test18() {
        let startpos = Board::from_fen("2r1k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 560);
        assert_eq!(perft(&startpos, 3), 13592);
        assert_eq!(perft(&startpos, 4), 317324);
        assert_eq!(perft(&startpos, 5), 7710115);
        assert_eq!(perft(&startpos, 6), 185959088);
    }

    #[test]
    fn perft_test19() {
        let startpos = Board::from_fen("r3k1r1/8/8/8/8/8/8/R3K2R w KQq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 560);
        assert_eq!(perft(&startpos, 3), 13607);
        assert_eq!(perft(&startpos, 4), 320792);
        assert_eq!(perft(&startpos, 5), 7848606);
        assert_eq!(perft(&startpos, 6), 190755813);
    }

    #[test]
    fn perft_test20() {
        let startpos = Board::from_fen("4k3/8/8/8/8/8/8/4K2R b K - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 75);
        assert_eq!(perft(&startpos, 3), 459);
        assert_eq!(perft(&startpos, 4), 8290);
        assert_eq!(perft(&startpos, 5), 47635);
        assert_eq!(perft(&startpos, 6), 899442);
    }

    #[test]
    fn perft_test21() {
        let startpos = Board::from_fen("4k3/8/8/8/8/8/8/R3K3 b Q - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 80);
        assert_eq!(perft(&startpos, 3), 493);
        assert_eq!(perft(&startpos, 4), 8897);
        assert_eq!(perft(&startpos, 5), 52710);
        assert_eq!(perft(&startpos, 6), 1001523);
    }

    #[test]
    fn perft_test22() {
        let startpos = Board::from_fen("4k2r/8/8/8/8/8/8/4K3 b k - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 15);
        assert_eq!(perft(&startpos, 2), 66);
        assert_eq!(perft(&startpos, 3), 1197);
        assert_eq!(perft(&startpos, 4), 7059);
        assert_eq!(perft(&startpos, 5), 133987);
        assert_eq!(perft(&startpos, 6), 764643);
    }

    #[test]
    fn perft_test23() {
        let startpos = Board::from_fen("r3k3/8/8/8/8/8/8/4K3 b q - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 16);
        assert_eq!(perft(&startpos, 2), 71);
        assert_eq!(perft(&startpos, 3), 1287);
        assert_eq!(perft(&startpos, 4), 7626);
        assert_eq!(perft(&startpos, 5), 145232);
        assert_eq!(perft(&startpos, 6), 846648);
    }

    #[test]
    fn perft_test24() {
        let startpos = Board::from_fen("4k3/8/8/8/8/8/8/R3K2R b KQ - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 130);
        assert_eq!(perft(&startpos, 3), 782);
        assert_eq!(perft(&startpos, 4), 22180);
        assert_eq!(perft(&startpos, 5), 118882);
        assert_eq!(perft(&startpos, 6), 3517770);
    }

    #[test]
    fn perft_test25() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 26);
        assert_eq!(perft(&startpos, 2), 112);
        assert_eq!(perft(&startpos, 3), 3189);
        assert_eq!(perft(&startpos, 4), 17945);
        assert_eq!(perft(&startpos, 5), 532933);
        assert_eq!(perft(&startpos, 6), 2788982);
    }

    #[test]
    fn perft_test26() {
        let startpos = Board::from_fen("8/8/8/8/8/8/6k1/4K2R b K - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 32);
        assert_eq!(perft(&startpos, 3), 134);
        assert_eq!(perft(&startpos, 4), 2073);
        assert_eq!(perft(&startpos, 5), 10485);
        assert_eq!(perft(&startpos, 6), 179869);
    }

    #[test]
    fn perft_test27() {
        let startpos = Board::from_fen("8/8/8/8/8/8/1k6/R3K3 b Q - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 49);
        assert_eq!(perft(&startpos, 3), 243);
        assert_eq!(perft(&startpos, 4), 3991);
        assert_eq!(perft(&startpos, 5), 20780);
        assert_eq!(perft(&startpos, 6), 367724);
    }

    #[test]
    fn perft_test28() {
        let startpos = Board::from_fen("4k2r/6K1/8/8/8/8/8/8 b k - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 12);
        assert_eq!(perft(&startpos, 2), 38);
        assert_eq!(perft(&startpos, 3), 564);
        assert_eq!(perft(&startpos, 4), 2219);
        assert_eq!(perft(&startpos, 5), 37735);
        assert_eq!(perft(&startpos, 6), 185867);
    }

    #[test]
    fn perft_test29() {
        let startpos = Board::from_fen("r3k3/1K6/8/8/8/8/8/8 b q - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 15);
        assert_eq!(perft(&startpos, 2), 65);
        assert_eq!(perft(&startpos, 3), 1018);
        assert_eq!(perft(&startpos, 4), 4573);
        assert_eq!(perft(&startpos, 5), 80619);
        assert_eq!(perft(&startpos, 6), 413018);
    }

    #[test]
    fn perft_test30() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 26);
        assert_eq!(perft(&startpos, 2), 568);
        assert_eq!(perft(&startpos, 3), 13744);
        assert_eq!(perft(&startpos, 4), 314346);
        assert_eq!(perft(&startpos, 5), 7594526);
        assert_eq!(perft(&startpos, 6), 179862938);
    }

    #[test]
    fn perft_test31() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/1R2K2R b Kkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 26);
        assert_eq!(perft(&startpos, 2), 583);
        assert_eq!(perft(&startpos, 3), 14252);
        assert_eq!(perft(&startpos, 4), 334705);
        assert_eq!(perft(&startpos, 5), 8198901);
        assert_eq!(perft(&startpos, 6), 198328929);
    }

    #[test]
    fn perft_test32() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/2R1K2R b Kkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 560);
        assert_eq!(perft(&startpos, 3), 13592);
        assert_eq!(perft(&startpos, 4), 317324);
        assert_eq!(perft(&startpos, 5), 7710115);
        assert_eq!(perft(&startpos, 6), 185959088);
    }

    #[test]
    fn perft_test33() {
        let startpos = Board::from_fen("r3k2r/8/8/8/8/8/8/R3K1R1 b Qkq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 560);
        assert_eq!(perft(&startpos, 3), 13607);
        assert_eq!(perft(&startpos, 4), 320792);
        assert_eq!(perft(&startpos, 5), 7848606);
        assert_eq!(perft(&startpos, 6), 190755813);
    }

    #[test]
    fn perft_test34() {
        let startpos = Board::from_fen("1r2k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 567);
        assert_eq!(perft(&startpos, 3), 14095);
        assert_eq!(perft(&startpos, 4), 328965);
        assert_eq!(perft(&startpos, 5), 8153719);
        assert_eq!(perft(&startpos, 6), 195629489);
    }

    #[test]
    fn perft_test35() {
        let startpos = Board::from_fen("2r1k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 548);
        assert_eq!(perft(&startpos, 3), 13502);
        assert_eq!(perft(&startpos, 4), 312835);
        assert_eq!(perft(&startpos, 5), 7736373);
        assert_eq!(perft(&startpos, 6), 184411439);
    }

    #[test]
    fn perft_test36() {
        let startpos = Board::from_fen("r3k1r1/8/8/8/8/8/8/R3K2R b KQq - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 25);
        assert_eq!(perft(&startpos, 2), 547);
        assert_eq!(perft(&startpos, 3), 13579);
        assert_eq!(perft(&startpos, 4), 316214);
        assert_eq!(perft(&startpos, 5), 7878456);
        assert_eq!(perft(&startpos, 6), 189224276);
    }

    #[test]
    fn perft_test37() {
        let startpos = Board::from_fen("8/1n4N1/2k5/8/8/5K2/1N4n1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 14);
        assert_eq!(perft(&startpos, 2), 195);
        assert_eq!(perft(&startpos, 3), 2760);
        assert_eq!(perft(&startpos, 4), 38675);
        assert_eq!(perft(&startpos, 5), 570726);
        assert_eq!(perft(&startpos, 6), 8107539);
    }

    #[test]
    fn perft_test38() {
        let startpos = Board::from_fen("8/1k6/8/5N2/8/4n3/8/2K5 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 11);
        assert_eq!(perft(&startpos, 2), 156);
        assert_eq!(perft(&startpos, 3), 1636);
        assert_eq!(perft(&startpos, 4), 20534);
        assert_eq!(perft(&startpos, 5), 223507);
        assert_eq!(perft(&startpos, 6), 2594412);
    }

    #[test]
    fn perft_test39() {
        let startpos = Board::from_fen("8/8/4k3/3Nn3/3nN3/4K3/8/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 19);
        assert_eq!(perft(&startpos, 2), 289);
        assert_eq!(perft(&startpos, 3), 4442);
        assert_eq!(perft(&startpos, 4), 73584);
        assert_eq!(perft(&startpos, 5), 1198299);
        assert_eq!(perft(&startpos, 6), 19870403);
    }

    #[test]
    fn perft_test40() {
        let startpos = Board::from_fen("K7/8/2n5/1n6/8/8/8/k6N w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 51);
        assert_eq!(perft(&startpos, 3), 345);
        assert_eq!(perft(&startpos, 4), 5301);
        assert_eq!(perft(&startpos, 5), 38348);
        assert_eq!(perft(&startpos, 6), 588695);
    }

    #[test]
    fn perft_test41() {
        let startpos = Board::from_fen("k7/8/2N5/1N6/8/8/8/K6n w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 17);
        assert_eq!(perft(&startpos, 2), 54);
        assert_eq!(perft(&startpos, 3), 835);
        assert_eq!(perft(&startpos, 4), 5910);
        assert_eq!(perft(&startpos, 5), 92250);
        assert_eq!(perft(&startpos, 6), 688780);
    }

    #[test]
    fn perft_test42() {
        let startpos = Board::from_fen("8/1n4N1/2k5/8/8/5K2/1N4n1/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 15);
        assert_eq!(perft(&startpos, 2), 193);
        assert_eq!(perft(&startpos, 3), 2816);
        assert_eq!(perft(&startpos, 4), 40039);
        assert_eq!(perft(&startpos, 5), 582642);
        assert_eq!(perft(&startpos, 6), 8503277);
    }

    #[test]
    fn perft_test43() {
        let startpos = Board::from_fen("8/1k6/8/5N2/8/4n3/8/2K5 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 16);
        assert_eq!(perft(&startpos, 2), 180);
        assert_eq!(perft(&startpos, 3), 2290);
        assert_eq!(perft(&startpos, 4), 24640);
        assert_eq!(perft(&startpos, 5), 288141);
        assert_eq!(perft(&startpos, 6), 3147566);
    }

    #[test]
    fn perft_test44() {
        let startpos = Board::from_fen("8/8/3K4/3Nn3/3nN3/4k3/8/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 68);
        assert_eq!(perft(&startpos, 3), 1118);
        assert_eq!(perft(&startpos, 4), 16199);
        assert_eq!(perft(&startpos, 5), 281190);
        assert_eq!(perft(&startpos, 6), 4405103);
    }

    #[test]
    fn perft_test45() {
        let startpos = Board::from_fen("K7/8/2n5/1n6/8/8/8/k6N b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 17);
        assert_eq!(perft(&startpos, 2), 54);
        assert_eq!(perft(&startpos, 3), 835);
        assert_eq!(perft(&startpos, 4), 5910);
        assert_eq!(perft(&startpos, 5), 92250);
        assert_eq!(perft(&startpos, 6), 688780);
    }

    #[test]
    fn perft_test46() {
        let startpos = Board::from_fen("k7/8/2N5/1N6/8/8/8/K6n b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 51);
        assert_eq!(perft(&startpos, 3), 345);
        assert_eq!(perft(&startpos, 4), 5301);
        assert_eq!(perft(&startpos, 5), 38348);
        assert_eq!(perft(&startpos, 6), 588695);
    }

    #[test]
    fn perft_test47() {
        let startpos = Board::from_fen("B6b/8/8/8/2K5/4k3/8/b6B w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 17);
        assert_eq!(perft(&startpos, 2), 278);
        assert_eq!(perft(&startpos, 3), 4607);
        assert_eq!(perft(&startpos, 4), 76778);
        assert_eq!(perft(&startpos, 5), 1320507);
        assert_eq!(perft(&startpos, 6), 22823890);
    }

    #[test]
    fn perft_test48() {
        let startpos = Board::from_fen("8/8/1B6/7b/7k/8/2B1b3/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 21);
        assert_eq!(perft(&startpos, 2), 316);
        assert_eq!(perft(&startpos, 3), 5744);
        assert_eq!(perft(&startpos, 4), 93338);
        assert_eq!(perft(&startpos, 5), 1713368);
        assert_eq!(perft(&startpos, 6), 28861171);
    }

    #[test]
    fn perft_test49() {
        let startpos = Board::from_fen("k7/B7/1B6/1B6/8/8/8/K6b w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 21);
        assert_eq!(perft(&startpos, 2), 144);
        assert_eq!(perft(&startpos, 3), 3242);
        assert_eq!(perft(&startpos, 4), 32955);
        assert_eq!(perft(&startpos, 5), 787524);
        assert_eq!(perft(&startpos, 6), 7881673);
    }

    #[test]
    fn perft_test50() {
        let startpos = Board::from_fen("K7/b7/1b6/1b6/8/8/8/k6B w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 7);
        assert_eq!(perft(&startpos, 2), 143);
        assert_eq!(perft(&startpos, 3), 1416);
        assert_eq!(perft(&startpos, 4), 31787);
        assert_eq!(perft(&startpos, 5), 310862);
        assert_eq!(perft(&startpos, 6), 7382896);
    }

    #[test]
    fn perft_test51() {
        let startpos = Board::from_fen("B6b/8/8/8/2K5/5k2/8/b6B b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 6);
        assert_eq!(perft(&startpos, 2), 106);
        assert_eq!(perft(&startpos, 3), 1829);
        assert_eq!(perft(&startpos, 4), 31151);
        assert_eq!(perft(&startpos, 5), 530585);
        assert_eq!(perft(&startpos, 6), 9250746);
    }

    #[test]
    fn perft_test52() {
        let startpos = Board::from_fen("8/8/1B6/7b/7k/8/2B1b3/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 17);
        assert_eq!(perft(&startpos, 2), 309);
        assert_eq!(perft(&startpos, 3), 5133);
        assert_eq!(perft(&startpos, 4), 93603);
        assert_eq!(perft(&startpos, 5), 1591064);
        assert_eq!(perft(&startpos, 6), 29027891);
    }

    #[test]
    fn perft_test53() {
        let startpos = Board::from_fen("k7/B7/1B6/1B6/8/8/8/K6b b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 7);
        assert_eq!(perft(&startpos, 2), 143);
        assert_eq!(perft(&startpos, 3), 1416);
        assert_eq!(perft(&startpos, 4), 31787);
        assert_eq!(perft(&startpos, 5), 310862);
        assert_eq!(perft(&startpos, 6), 7382896);
    }

    #[test]
    fn perft_test54() {
        let startpos = Board::from_fen("K7/b7/1b6/1b6/8/8/8/k6B b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 21);
        assert_eq!(perft(&startpos, 2), 144);
        assert_eq!(perft(&startpos, 3), 3242);
        assert_eq!(perft(&startpos, 4), 32955);
        assert_eq!(perft(&startpos, 5), 787524);
        assert_eq!(perft(&startpos, 6), 7881673);
    }

    #[test]
    fn perft_test55() {
        let startpos = Board::from_fen("7k/RR6/8/8/8/8/rr6/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 19);
        assert_eq!(perft(&startpos, 2), 275);
        assert_eq!(perft(&startpos, 3), 5300);
        assert_eq!(perft(&startpos, 4), 104342);
        assert_eq!(perft(&startpos, 5), 2161211);
        assert_eq!(perft(&startpos, 6), 44956585);
    }

    #[test]
    fn perft_test56() {
        let startpos = Board::from_fen("R6r/8/8/2K5/5k2/8/8/r6R w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 36);
        assert_eq!(perft(&startpos, 2), 1027);
        assert_eq!(perft(&startpos, 3), 29215);
        assert_eq!(perft(&startpos, 4), 771461);
        assert_eq!(perft(&startpos, 5), 20506480);
        assert_eq!(perft(&startpos, 6), 525169084);
    }

    #[test]
    fn perft_test57() {
        let startpos = Board::from_fen("7k/RR6/8/8/8/8/rr6/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 19);
        assert_eq!(perft(&startpos, 2), 275);
        assert_eq!(perft(&startpos, 3), 5300);
        assert_eq!(perft(&startpos, 4), 104342);
        assert_eq!(perft(&startpos, 5), 2161211);
        assert_eq!(perft(&startpos, 6), 44956585);
    }

    #[test]
    fn perft_test58() {
        let startpos = Board::from_fen("R6r/8/8/2K5/5k2/8/8/r6R b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 36);
        assert_eq!(perft(&startpos, 2), 1027);
        assert_eq!(perft(&startpos, 3), 29227);
        assert_eq!(perft(&startpos, 4), 771368);
        assert_eq!(perft(&startpos, 5), 20521342);
        assert_eq!(perft(&startpos, 6), 524966748);
    }

    #[test]
    fn perft_test59() {
        let startpos = Board::from_fen("6kq/8/8/8/8/8/8/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 2);
        assert_eq!(perft(&startpos, 2), 36);
        assert_eq!(perft(&startpos, 3), 143);
        assert_eq!(perft(&startpos, 4), 3637);
        assert_eq!(perft(&startpos, 5), 14893);
        assert_eq!(perft(&startpos, 6), 391507);
    }

    #[test]
    fn perft_test60() {
        let startpos = Board::from_fen("6KQ/8/8/8/8/8/8/7k b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 2);
        assert_eq!(perft(&startpos, 2), 36);
        assert_eq!(perft(&startpos, 3), 143);
        assert_eq!(perft(&startpos, 4), 3637);
        assert_eq!(perft(&startpos, 5), 14893);
        assert_eq!(perft(&startpos, 6), 391507);
    }

    #[test]
    fn perft_test61() {
        let startpos = Board::from_fen("K7/8/8/3Q4/4q3/8/8/7k w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 6);
        assert_eq!(perft(&startpos, 2), 35);
        assert_eq!(perft(&startpos, 3), 495);
        assert_eq!(perft(&startpos, 4), 8349);
        assert_eq!(perft(&startpos, 5), 166741);
        assert_eq!(perft(&startpos, 6), 3370175);
    }

    #[test]
    fn perft_test62() {
        let startpos = Board::from_fen("6qk/8/8/8/8/8/8/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 22);
        assert_eq!(perft(&startpos, 2), 43);
        assert_eq!(perft(&startpos, 3), 1015);
        assert_eq!(perft(&startpos, 4), 4167);
        assert_eq!(perft(&startpos, 5), 105749);
        assert_eq!(perft(&startpos, 6), 419369);
    }

    #[test]
    fn perft_test63() {
        let startpos = Board::from_fen("6KQ/8/8/8/8/8/8/7k b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 2);
        assert_eq!(perft(&startpos, 2), 36);
        assert_eq!(perft(&startpos, 3), 143);
        assert_eq!(perft(&startpos, 4), 3637);
        assert_eq!(perft(&startpos, 5), 14893);
        assert_eq!(perft(&startpos, 6), 391507);
    }

    #[test]
    fn perft_test64() {
        let startpos = Board::from_fen("K7/8/8/3Q4/4q3/8/8/7k b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 6);
        assert_eq!(perft(&startpos, 2), 35);
        assert_eq!(perft(&startpos, 3), 495);
        assert_eq!(perft(&startpos, 4), 8349);
        assert_eq!(perft(&startpos, 5), 166741);
        assert_eq!(perft(&startpos, 6), 3370175);
    }

    #[test]
    fn perft_test65() {
        let startpos = Board::from_fen("8/8/8/8/8/K7/P7/k7 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 7);
        assert_eq!(perft(&startpos, 3), 43);
        assert_eq!(perft(&startpos, 4), 199);
        assert_eq!(perft(&startpos, 5), 1347);
        assert_eq!(perft(&startpos, 6), 6249);
    }

    #[test]
    fn perft_test66() {
        let startpos = Board::from_fen("8/8/8/8/8/7K/7P/7k w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 7);
        assert_eq!(perft(&startpos, 3), 43);
        assert_eq!(perft(&startpos, 4), 199);
        assert_eq!(perft(&startpos, 5), 1347);
        assert_eq!(perft(&startpos, 6), 6249);
    }

    #[test]
    fn perft_test67() {
        let startpos = Board::from_fen("K7/p7/k7/8/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 1);
        assert_eq!(perft(&startpos, 2), 3);
        assert_eq!(perft(&startpos, 3), 12);
        assert_eq!(perft(&startpos, 4), 80);
        assert_eq!(perft(&startpos, 5), 342);
        assert_eq!(perft(&startpos, 6), 2343);
    }

    #[test]
    fn perft_test68() {
        let startpos = Board::from_fen("7K/7p/7k/8/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 1);
        assert_eq!(perft(&startpos, 2), 3);
        assert_eq!(perft(&startpos, 3), 12);
        assert_eq!(perft(&startpos, 4), 80);
        assert_eq!(perft(&startpos, 5), 342);
        assert_eq!(perft(&startpos, 6), 2343);
    }

    #[test]
    fn perft_test69() {
        let startpos = Board::from_fen("8/2k1p3/3pP3/3P2K1/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 7);
        assert_eq!(perft(&startpos, 2), 35);
        assert_eq!(perft(&startpos, 3), 210);
        assert_eq!(perft(&startpos, 4), 1091);
        assert_eq!(perft(&startpos, 5), 7028);
        assert_eq!(perft(&startpos, 6), 34834);
    }

    #[test]
    fn perft_test70() {
        let startpos = Board::from_fen("8/8/8/8/8/K7/P7/k7 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 1);
        assert_eq!(perft(&startpos, 2), 3);
        assert_eq!(perft(&startpos, 3), 12);
        assert_eq!(perft(&startpos, 4), 80);
        assert_eq!(perft(&startpos, 5), 342);
        assert_eq!(perft(&startpos, 6), 2343);
    }

    #[test]
    fn perft_test71() {
        let startpos = Board::from_fen("8/8/8/8/8/7K/7P/7k b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 1);
        assert_eq!(perft(&startpos, 2), 3);
        assert_eq!(perft(&startpos, 3), 12);
        assert_eq!(perft(&startpos, 4), 80);
        assert_eq!(perft(&startpos, 5), 342);
        assert_eq!(perft(&startpos, 6), 2343);
    }

    #[test]
    fn perft_test72() {
        let startpos = Board::from_fen("K7/p7/k7/8/8/8/8/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 7);
        assert_eq!(perft(&startpos, 3), 43);
        assert_eq!(perft(&startpos, 4), 199);
        assert_eq!(perft(&startpos, 5), 1347);
        assert_eq!(perft(&startpos, 6), 6249);
    }

    #[test]
    fn perft_test73() {
        let startpos = Board::from_fen("7K/7p/7k/8/8/8/8/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 7);
        assert_eq!(perft(&startpos, 3), 43);
        assert_eq!(perft(&startpos, 4), 199);
        assert_eq!(perft(&startpos, 5), 1347);
        assert_eq!(perft(&startpos, 6), 6249);
    }

    #[test]
    fn perft_test74() {
        let startpos = Board::from_fen("8/2k1p3/3pP3/3P2K1/8/8/8/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 35);
        assert_eq!(perft(&startpos, 3), 182);
        assert_eq!(perft(&startpos, 4), 1091);
        assert_eq!(perft(&startpos, 5), 5408);
        assert_eq!(perft(&startpos, 6), 34822);
    }

    #[test]
    fn perft_test75() {
        let startpos = Board::from_fen("8/8/8/8/8/4k3/4P3/4K3 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 2);
        assert_eq!(perft(&startpos, 2), 8);
        assert_eq!(perft(&startpos, 3), 44);
        assert_eq!(perft(&startpos, 4), 282);
        assert_eq!(perft(&startpos, 5), 1814);
        assert_eq!(perft(&startpos, 6), 11848);
    }

    #[test]
    fn perft_test76() {
        let startpos = Board::from_fen("4k3/4p3/4K3/8/8/8/8/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 2);
        assert_eq!(perft(&startpos, 2), 8);
        assert_eq!(perft(&startpos, 3), 44);
        assert_eq!(perft(&startpos, 4), 282);
        assert_eq!(perft(&startpos, 5), 1814);
        assert_eq!(perft(&startpos, 6), 11848);
    }

    #[test]
    fn perft_test77() {
        let startpos = Board::from_fen("8/8/7k/7p/7P/7K/8/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 9);
        assert_eq!(perft(&startpos, 3), 57);
        assert_eq!(perft(&startpos, 4), 360);
        assert_eq!(perft(&startpos, 5), 1969);
        assert_eq!(perft(&startpos, 6), 10724);
    }

    #[test]
    fn perft_test78() {
        let startpos = Board::from_fen("8/8/k7/p7/P7/K7/8/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 9);
        assert_eq!(perft(&startpos, 3), 57);
        assert_eq!(perft(&startpos, 4), 360);
        assert_eq!(perft(&startpos, 5), 1969);
        assert_eq!(perft(&startpos, 6), 10724);
    }

    #[test]
    fn perft_test79() {
        let startpos = Board::from_fen("8/8/3k4/3p4/3P4/3K4/8/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 25);
        assert_eq!(perft(&startpos, 3), 180);
        assert_eq!(perft(&startpos, 4), 1294);
        assert_eq!(perft(&startpos, 5), 8296);
        assert_eq!(perft(&startpos, 6), 53138);
    }

    #[test]
    fn perft_test80() {
        let startpos = Board::from_fen("8/3k4/3p4/8/3P4/3K4/8/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 8);
        assert_eq!(perft(&startpos, 2), 61);
        assert_eq!(perft(&startpos, 3), 483);
        assert_eq!(perft(&startpos, 4), 3213);
        assert_eq!(perft(&startpos, 5), 23599);
        assert_eq!(perft(&startpos, 6), 157093);
    }

    #[test]
    fn perft_test81() {
        let startpos = Board::from_fen("8/8/3k4/3p4/8/3P4/3K4/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 8);
        assert_eq!(perft(&startpos, 2), 61);
        assert_eq!(perft(&startpos, 3), 411);
        assert_eq!(perft(&startpos, 4), 3213);
        assert_eq!(perft(&startpos, 5), 21637);
        assert_eq!(perft(&startpos, 6), 158065);
    }

    #[test]
    fn perft_test82() {
        let startpos = Board::from_fen("k7/8/3p4/8/3P4/8/8/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 15);
        assert_eq!(perft(&startpos, 3), 90);
        assert_eq!(perft(&startpos, 4), 534);
        assert_eq!(perft(&startpos, 5), 3450);
        assert_eq!(perft(&startpos, 6), 20960);
    }

    #[test]
    fn perft_test83() {
        let startpos = Board::from_fen("8/8/7k/7p/7P/7K/8/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 9);
        assert_eq!(perft(&startpos, 3), 57);
        assert_eq!(perft(&startpos, 4), 360);
        assert_eq!(perft(&startpos, 5), 1969);
        assert_eq!(perft(&startpos, 6), 10724);
    }

    #[test]
    fn perft_test84() {
        let startpos = Board::from_fen("8/8/k7/p7/P7/K7/8/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 9);
        assert_eq!(perft(&startpos, 3), 57);
        assert_eq!(perft(&startpos, 4), 360);
        assert_eq!(perft(&startpos, 5), 1969);
        assert_eq!(perft(&startpos, 6), 10724);
    }

    #[test]
    fn perft_test85() {
        let startpos = Board::from_fen("8/8/3k4/3p4/3P4/3K4/8/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 25);
        assert_eq!(perft(&startpos, 3), 180);
        assert_eq!(perft(&startpos, 4), 1294);
        assert_eq!(perft(&startpos, 5), 8296);
        assert_eq!(perft(&startpos, 6), 53138);
    }

    #[test]
    fn perft_test86() {
        let startpos = Board::from_fen("8/3k4/3p4/8/3P4/3K4/8/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 8);
        assert_eq!(perft(&startpos, 2), 61);
        assert_eq!(perft(&startpos, 3), 411);
        assert_eq!(perft(&startpos, 4), 3213);
        assert_eq!(perft(&startpos, 5), 21637);
        assert_eq!(perft(&startpos, 6), 158065);
    }

    #[test]
    fn perft_test87() {
        let startpos = Board::from_fen("8/8/3k4/3p4/8/3P4/3K4/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 8);
        assert_eq!(perft(&startpos, 2), 61);
        assert_eq!(perft(&startpos, 3), 483);
        assert_eq!(perft(&startpos, 4), 3213);
        assert_eq!(perft(&startpos, 5), 23599);
        assert_eq!(perft(&startpos, 6), 157093);
    }

    #[test]
    fn perft_test88() {
        let startpos = Board::from_fen("k7/8/3p4/8/3P4/8/8/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 15);
        assert_eq!(perft(&startpos, 3), 89);
        assert_eq!(perft(&startpos, 4), 537);
        assert_eq!(perft(&startpos, 5), 3309);
        assert_eq!(perft(&startpos, 6), 21104);
    }

    #[test]
    fn perft_test89() {
        let startpos = Board::from_fen("7k/3p4/8/8/3P4/8/8/K7 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 19);
        assert_eq!(perft(&startpos, 3), 117);
        assert_eq!(perft(&startpos, 4), 720);
        assert_eq!(perft(&startpos, 5), 4661);
        assert_eq!(perft(&startpos, 6), 32191);
    }

    #[test]
    fn perft_test90() {
        let startpos = Board::from_fen("7k/8/8/3p4/8/8/3P4/K7 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 19);
        assert_eq!(perft(&startpos, 3), 116);
        assert_eq!(perft(&startpos, 4), 716);
        assert_eq!(perft(&startpos, 5), 4786);
        assert_eq!(perft(&startpos, 6), 30980);
    }

    #[test]
    fn perft_test91() {
        let startpos = Board::from_fen("k7/8/8/7p/6P1/8/8/K7 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 22);
        assert_eq!(perft(&startpos, 3), 139);
        assert_eq!(perft(&startpos, 4), 877);
        assert_eq!(perft(&startpos, 5), 6112);
        assert_eq!(perft(&startpos, 6), 41874);
    }

    #[test]
    fn perft_test92() {
        let startpos = Board::from_fen("k7/8/7p/8/8/6P1/8/K7 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4354);
        assert_eq!(perft(&startpos, 6), 29679);
    }

    #[test]
    fn perft_test93() {
        let startpos = Board::from_fen("k7/8/8/6p1/7P/8/8/K7 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 22);
        assert_eq!(perft(&startpos, 3), 139);
        assert_eq!(perft(&startpos, 4), 877);
        assert_eq!(perft(&startpos, 5), 6112);
        assert_eq!(perft(&startpos, 6), 41874);
    }

    #[test]
    fn perft_test94() {
        let startpos = Board::from_fen("k7/8/6p1/8/8/7P/8/K7 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4354);
        assert_eq!(perft(&startpos, 6), 29679);
    }

    #[test]
    fn perft_test95() {
        let startpos = Board::from_fen("k7/8/8/3p4/4p3/8/8/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 3);
        assert_eq!(perft(&startpos, 2), 15);
        assert_eq!(perft(&startpos, 3), 84);
        assert_eq!(perft(&startpos, 4), 573);
        assert_eq!(perft(&startpos, 5), 3013);
        assert_eq!(perft(&startpos, 6), 22886);
    }

    #[test]
    fn perft_test96() {
        let startpos = Board::from_fen("k7/8/3p4/8/8/4P3/8/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4271);
        assert_eq!(perft(&startpos, 6), 28662);
    }

    #[test]
    fn perft_test97() {
        let startpos = Board::from_fen("7k/3p4/8/8/3P4/8/8/K7 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 19);
        assert_eq!(perft(&startpos, 3), 117);
        assert_eq!(perft(&startpos, 4), 720);
        assert_eq!(perft(&startpos, 5), 5014);
        assert_eq!(perft(&startpos, 6), 32167);
    }

    #[test]
    fn perft_test98() {
        let startpos = Board::from_fen("7k/8/8/3p4/8/8/3P4/K7 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 19);
        assert_eq!(perft(&startpos, 3), 117);
        assert_eq!(perft(&startpos, 4), 712);
        assert_eq!(perft(&startpos, 5), 4658);
        assert_eq!(perft(&startpos, 6), 30749);
    }

    #[test]
    fn perft_test99() {
        let startpos = Board::from_fen("k7/8/8/7p/6P1/8/8/K7 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 22);
        assert_eq!(perft(&startpos, 3), 139);
        assert_eq!(perft(&startpos, 4), 877);
        assert_eq!(perft(&startpos, 5), 6112);
        assert_eq!(perft(&startpos, 6), 41874);
    }

    #[test]
    fn perft_test100() {
        let startpos = Board::from_fen("k7/8/7p/8/8/6P1/8/K7 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4354);
        assert_eq!(perft(&startpos, 6), 29679);
    }

    #[test]
    fn perft_test101() {
        let startpos = Board::from_fen("k7/8/8/6p1/7P/8/8/K7 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 22);
        assert_eq!(perft(&startpos, 3), 139);
        assert_eq!(perft(&startpos, 4), 877);
        assert_eq!(perft(&startpos, 5), 6112);
        assert_eq!(perft(&startpos, 6), 41874);
    }

    #[test]
    fn perft_test102() {
        let startpos = Board::from_fen("k7/8/6p1/8/8/7P/8/K7 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4354);
        assert_eq!(perft(&startpos, 6), 29679);
    }

    #[test]
    fn perft_test103() {
        let startpos = Board::from_fen("k7/8/8/3p4/4p3/8/8/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 15);
        assert_eq!(perft(&startpos, 3), 102);
        assert_eq!(perft(&startpos, 4), 569);
        assert_eq!(perft(&startpos, 5), 4337);
        assert_eq!(perft(&startpos, 6), 22579);
    }

    #[test]
    fn perft_test104() {
        let startpos = Board::from_fen("k7/8/3p4/8/8/4P3/8/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4271);
        assert_eq!(perft(&startpos, 6), 28662);
    }

    #[test]
    fn perft_test105() {
        let startpos = Board::from_fen("7k/8/8/p7/1P6/8/8/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 22);
        assert_eq!(perft(&startpos, 3), 139);
        assert_eq!(perft(&startpos, 4), 877);
        assert_eq!(perft(&startpos, 5), 6112);
        assert_eq!(perft(&startpos, 6), 41874);
    }

    #[test]
    fn perft_test106() {
        let startpos = Board::from_fen("7k/8/p7/8/8/1P6/8/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4354);
        assert_eq!(perft(&startpos, 6), 29679);
    }

    #[test]
    fn perft_test107() {
        let startpos = Board::from_fen("7k/8/8/1p6/P7/8/8/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 22);
        assert_eq!(perft(&startpos, 3), 139);
        assert_eq!(perft(&startpos, 4), 877);
        assert_eq!(perft(&startpos, 5), 6112);
        assert_eq!(perft(&startpos, 6), 41874);
    }

    #[test]
    fn perft_test108() {
        let startpos = Board::from_fen("7k/8/1p6/8/8/P7/8/7K w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4354);
        assert_eq!(perft(&startpos, 6), 29679);
    }

    #[test]
    fn perft_test109() {
        let startpos = Board::from_fen("k7/7p/8/8/8/8/6P1/K7 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 25);
        assert_eq!(perft(&startpos, 3), 161);
        assert_eq!(perft(&startpos, 4), 1035);
        assert_eq!(perft(&startpos, 5), 7574);
        assert_eq!(perft(&startpos, 6), 55338);
    }

    #[test]
    fn perft_test110() {
        let startpos = Board::from_fen("k7/6p1/8/8/8/8/7P/K7 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 25);
        assert_eq!(perft(&startpos, 3), 161);
        assert_eq!(perft(&startpos, 4), 1035);
        assert_eq!(perft(&startpos, 5), 7574);
        assert_eq!(perft(&startpos, 6), 55338);
    }

    #[test]
    fn perft_test111() {
        let startpos = Board::from_fen("3k4/3pp3/8/8/8/8/3PP3/3K4 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 7);
        assert_eq!(perft(&startpos, 2), 49);
        assert_eq!(perft(&startpos, 3), 378);
        assert_eq!(perft(&startpos, 4), 2902);
        assert_eq!(perft(&startpos, 5), 24122);
        assert_eq!(perft(&startpos, 6), 199002);
    }

    #[test]
    fn perft_test112() {
        let startpos = Board::from_fen("7k/8/8/p7/1P6/8/8/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 22);
        assert_eq!(perft(&startpos, 3), 139);
        assert_eq!(perft(&startpos, 4), 877);
        assert_eq!(perft(&startpos, 5), 6112);
        assert_eq!(perft(&startpos, 6), 41874);
    }

    #[test]
    fn perft_test113() {
        let startpos = Board::from_fen("7k/8/p7/8/8/1P6/8/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4354);
        assert_eq!(perft(&startpos, 6), 29679);
    }

    #[test]
    fn perft_test114() {
        let startpos = Board::from_fen("7k/8/8/1p6/P7/8/8/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 22);
        assert_eq!(perft(&startpos, 3), 139);
        assert_eq!(perft(&startpos, 4), 877);
        assert_eq!(perft(&startpos, 5), 6112);
        assert_eq!(perft(&startpos, 6), 41874);
    }

    #[test]
    fn perft_test115() {
        let startpos = Board::from_fen("7k/8/1p6/8/8/P7/8/7K b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 4);
        assert_eq!(perft(&startpos, 2), 16);
        assert_eq!(perft(&startpos, 3), 101);
        assert_eq!(perft(&startpos, 4), 637);
        assert_eq!(perft(&startpos, 5), 4354);
        assert_eq!(perft(&startpos, 6), 29679);
    }

    #[test]
    fn perft_test116() {
        let startpos = Board::from_fen("k7/7p/8/8/8/8/6P1/K7 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 25);
        assert_eq!(perft(&startpos, 3), 161);
        assert_eq!(perft(&startpos, 4), 1035);
        assert_eq!(perft(&startpos, 5), 7574);
        assert_eq!(perft(&startpos, 6), 55338);
    }

    #[test]
    fn perft_test117() {
        let startpos = Board::from_fen("k7/6p1/8/8/8/8/7P/K7 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 5);
        assert_eq!(perft(&startpos, 2), 25);
        assert_eq!(perft(&startpos, 3), 161);
        assert_eq!(perft(&startpos, 4), 1035);
        assert_eq!(perft(&startpos, 5), 7574);
        assert_eq!(perft(&startpos, 6), 55338);
    }

    #[test]
    fn perft_test118() {
        let startpos = Board::from_fen("3k4/3pp3/8/8/8/8/3PP3/3K4 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 7);
        assert_eq!(perft(&startpos, 2), 49);
        assert_eq!(perft(&startpos, 3), 378);
        assert_eq!(perft(&startpos, 4), 2902);
        assert_eq!(perft(&startpos, 5), 24122);
        assert_eq!(perft(&startpos, 6), 199002);
    }

    #[test]
    fn perft_test119() {
        let startpos = Board::from_fen("8/Pk6/8/8/8/8/6Kp/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 11);
        assert_eq!(perft(&startpos, 2), 97);
        assert_eq!(perft(&startpos, 3), 887);
        assert_eq!(perft(&startpos, 4), 8048);
        assert_eq!(perft(&startpos, 5), 90606);
        assert_eq!(perft(&startpos, 6), 1030499);
    }

    #[test]
    fn perft_test120() {
        let startpos = Board::from_fen("n1n5/1Pk5/8/8/8/8/5Kp1/5N1N w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 24);
        assert_eq!(perft(&startpos, 2), 421);
        assert_eq!(perft(&startpos, 3), 7421);
        assert_eq!(perft(&startpos, 4), 124608);
        assert_eq!(perft(&startpos, 5), 2193768);
        assert_eq!(perft(&startpos, 6), 37665329);
    }

    #[test]
    fn perft_test121() {
        let startpos = Board::from_fen("8/PPPk4/8/8/8/8/4Kppp/8 w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 18);
        assert_eq!(perft(&startpos, 2), 270);
        assert_eq!(perft(&startpos, 3), 4699);
        assert_eq!(perft(&startpos, 4), 79355);
        assert_eq!(perft(&startpos, 5), 1533145);
        assert_eq!(perft(&startpos, 6), 28859283);
    }

    #[test]
    fn perft_test122() {
        let startpos = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N w - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 24);
        assert_eq!(perft(&startpos, 2), 496);
        assert_eq!(perft(&startpos, 3), 9483);
        assert_eq!(perft(&startpos, 4), 182838);
        assert_eq!(perft(&startpos, 5), 3605103);
        assert_eq!(perft(&startpos, 6), 71179139);
    }

    #[test]
    fn perft_test123() {
        let startpos = Board::from_fen("8/Pk6/8/8/8/8/6Kp/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 11);
        assert_eq!(perft(&startpos, 2), 97);
        assert_eq!(perft(&startpos, 3), 887);
        assert_eq!(perft(&startpos, 4), 8048);
        assert_eq!(perft(&startpos, 5), 90606);
        assert_eq!(perft(&startpos, 6), 1030499);
    }

    #[test]
    fn perft_test124() {
        let startpos = Board::from_fen("n1n5/1Pk5/8/8/8/8/5Kp1/5N1N b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 24);
        assert_eq!(perft(&startpos, 2), 421);
        assert_eq!(perft(&startpos, 3), 7421);
        assert_eq!(perft(&startpos, 4), 124608);
        assert_eq!(perft(&startpos, 5), 2193768);
        assert_eq!(perft(&startpos, 6), 37665329);
    }

    #[test]
    fn perft_test125() {
        let startpos = Board::from_fen("8/PPPk4/8/8/8/8/4Kppp/8 b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 18);
        assert_eq!(perft(&startpos, 2), 270);
        assert_eq!(perft(&startpos, 3), 4699);
        assert_eq!(perft(&startpos, 4), 79355);
        assert_eq!(perft(&startpos, 5), 1533145);
        assert_eq!(perft(&startpos, 6), 28859283);
    }

    #[test]
    fn perft_test126() {
        let startpos = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap();
        assert_eq!(perft(&startpos, 1), 24);
        assert_eq!(perft(&startpos, 2), 496);
        assert_eq!(perft(&startpos, 3), 9483);
        assert_eq!(perft(&startpos, 4), 182838);
        assert_eq!(perft(&startpos, 5), 3605103);
        assert_eq!(perft(&startpos, 6), 71179139);
    }
}
