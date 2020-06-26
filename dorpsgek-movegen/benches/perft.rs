use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dorpsgek_movegen::{perft, Board};

pub fn perft_bench(c: &mut Criterion) {
    let mut board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    c.bench_function("perft 4", |b| {
        b.iter(|| {
            black_box(perft(&mut board, 4));
        })
    });
}

criterion_group!(benches, perft_bench);
criterion_main!(benches);
