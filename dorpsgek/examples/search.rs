use dorpsgek::Search;
use dorpsgek_movegen::Board;

use std::time::Instant;

fn main() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    let mut s = Search::new();
    let start = Instant::now();
    for depth in 1..=9 {
        let score = s.search(&board, depth, -100_000, 100_000);
        let now = Instant::now().duration_since(start);
        println!(
            "{} {:.2} {} {}",
            depth,
            score,
            now.as_millis() / 10,
            s.nodes() + s.qnodes()
        );
    }
    println!(
        "# QS: {:.3}%",
        (100 * s.qnodes()) as f64 / (s.nodes() as f64 + s.qnodes() as f64)
    );
}
