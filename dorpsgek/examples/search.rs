use dorpsgek::Search;
use dorpsgek_movegen::Board;
use tinyvec::ArrayVec;

use std::time::Instant;

fn main() {
    let fen = &std::env::args().nth(1).expect("Please provide a FEN string or 'bench'");
    let board = Board::from_fen(if fen == "bench" {
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
    } else {
        fen
    }).unwrap();

    let mut s = Search::new();
    let start = Instant::now();
    for depth in 1..=8 {
        let mut pv = ArrayVec::new();
        pv.set_len(0);
        let score = s.search_root(&board, depth, &mut pv);
        let now = Instant::now().duration_since(start);
        print!(
            "{} {:.2} {} {} ",
            depth,
            score,
            now.as_millis() / 10,
            s.nodes() + s.qnodes()
        );
        for m in pv {
            print!("{} ", m);
        }
        println!();
    }
    println!(
        "# QS: {:.3}%",
        (100 * s.qnodes()) as f64 / (s.nodes() as f64 + s.qnodes() as f64)
    );
}
