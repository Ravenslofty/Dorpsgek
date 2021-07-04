use dorpsgek_movegen::{perft, Board, Move};
use tinyvec::ArrayVec;
use rayon::prelude::*;

pub fn divide(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        1
    } else {
        let moves: [Move; 256] = [Move::default(); 256];
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        moves.par_iter().map(|m| {
            let board = board.make(*m);
            let nodes = perft(&board, depth - 1);
            println!("{} {}", m, nodes);
            nodes
        }).sum()
    }
}

fn main() {
    let startpos = Board::from_fen("8/2p5/3p4/KP3k1r/5p2/8/4P1P1/5R2 w - - 4 3").unwrap();

    let depth = 2;
    let nodes = divide(&startpos, depth);
    println!("Perft {}: {}", depth, nodes);
}
