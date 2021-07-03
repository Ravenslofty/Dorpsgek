use std::mem;

use dorpsgek_movegen::{Board, Move};
use tinyvec::ArrayVec;

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

pub fn divide(board: &Board, depth: u32) -> u64 {
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
            let nodes = perft(&board, depth - 1);
            println!("{} {}", m, nodes);
            count += nodes;
        }
        count
    }
}

fn main() {
    let startpos = Board::from_fen("8/K1p4r/3p4/1P6/1R3p1k/8/4P1P1/8 b - - 3 2").unwrap();

    let depth = 2;
    let nodes = divide(&startpos, depth);
    println!("Perft {}: {}", depth, nodes);
}
