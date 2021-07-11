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
    let startpos = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();

    let depth = 5;
    let nodes = divide(&startpos, depth);
    println!("size of board: {}", std::mem::size_of::<Board>());
    println!("Perft {}: {}", depth, nodes);
}
