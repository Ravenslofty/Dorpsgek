use dorpsgek_movegen::{perft, Board, Move};
use tinyvec::ArrayVec;

pub fn divide(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        1
    } else {
        let moves: [Move; 256] = [Move::default(); 256];
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
    let startpos = Board::from_fen("rnBq1k1r/pp3ppp/2pb4/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 9").unwrap();

    let depth = 3;
    let nodes = divide(&startpos, depth);
    println!("Perft {}: {}", depth, nodes);
}
