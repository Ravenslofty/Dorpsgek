use dorpsgek::Search;
use dorpsgek_movegen::Board;

fn main() {
    let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let mut s = Search::new();
    for depth in 1..=2 {
        let score = s.search(&board, depth, -1000.0, 1000.0);
        println!("{} {:.2} ? {} ?", depth, score, s.nodes());
    }
}