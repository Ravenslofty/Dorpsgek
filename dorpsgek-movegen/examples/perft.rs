use dorpsgek_movegen::{perft, Board, Move};
use rayon::prelude::*;
use tinyvec::ArrayVec;

pub fn divide(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        1
    } else {
        let moves: [Move; 256] = [Move::default(); 256];
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        moves
            .par_iter()
            .map(|m| {
                let board = board.make(*m);
                let nodes = perft(&board, depth - 1);
                println!("{} {}", m, nodes);
                nodes
            })
            .sum()
    }
}

fn main() {
    let startpos = Board::from_fen("n1n5/1Pk5/8/8/8/8/8/4KNrN w - - 0 2").unwrap();

    let depth = 1;
    let nodes = divide(&startpos, depth);
    println!("Perft {}: {}", depth, nodes);
}
