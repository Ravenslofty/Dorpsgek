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
    //let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let nodes = divide(&board, 6);
    println!("Perft 1: {}", nodes);
}
