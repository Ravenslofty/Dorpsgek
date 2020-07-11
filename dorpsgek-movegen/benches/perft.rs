use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use criterion_cpu_time::PosixTime;
use dorpsgek_movegen::{perft, Board};

pub fn perft_bench(c: &mut Criterion<PosixTime>) {
    let board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let mut group = c.benchmark_group("perft");

    group.throughput(Throughput::Elements(20));
    group.bench_with_input("1", &board, |b, board| {
        b.iter(|| {
            assert_eq!(perft(board, 1), 20);
        })
    });

    group.throughput(Throughput::Elements(400));
    group.bench_with_input("2", &board, |b, board| {
        b.iter(|| {
            assert_eq!(perft(board, 2), 400);
        })
    });

    group.throughput(Throughput::Elements(8902));
    group.bench_with_input("3", &board, |b, board| {
        b.iter(|| {
            assert_eq!(perft(board, 3), 8902);
        })
    });

    group.throughput(Throughput::Elements(197_281));
    group.bench_with_input("4", &board, |b, board| {
        b.iter(|| {
            assert_eq!(perft(board, 4), 197_281);
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_measurement(PosixTime::UserTime);
    targets = perft_bench
}

criterion_main!(benches);
