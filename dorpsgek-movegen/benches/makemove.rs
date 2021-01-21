use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use criterion_cpu_time::PosixTime;
use dorpsgek_movegen::{Board, Move, MoveType, Square, perft};

pub fn makemove_bench(c: &mut Criterion<PosixTime>) {
    let startpos =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let kiwipete =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();

    let e2 = unsafe { Square::from_u8_unchecked(12) };
    let e4 = unsafe { Square::from_u8_unchecked(28) };
    let e2e4 = Move::new(e2, e4, MoveType::Normal, None);

    let f3 = unsafe { Square::from_u8_unchecked(21) };
    let d3 = unsafe { Square::from_u8_unchecked(19) };
    let f3d3 = Move::new(f3, d3, MoveType::Normal, None);

    let a6 = unsafe { Square::from_u8_unchecked(40) };
    let e2a6 = Move::new(e2, a6, MoveType::Capture, None);

    let mut group = c.benchmark_group("makemove");

    group.sample_size(5_000);
    group.significance_level(0.005);
    group.noise_threshold(0.025);

    group.throughput(Throughput::Elements(1));
    group.bench_with_input("startpos-e4", &startpos, |b, board| {
        b.iter(|| board.make(e2e4))
    });

    group.throughput(Throughput::Elements(1));
    group.bench_with_input("kiwipete-Qd3", &kiwipete, |b, board| {
        b.iter(|| board.make(f3d3))
    });

    group.throughput(Throughput::Elements(1));
    group.bench_with_input("kiwipete-Bxa6", &kiwipete, |b, board| {
        b.iter(|| board.make(e2a6))
    });

    group.finish();
}

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

pub fn bench(c: &mut Criterion<PosixTime>) {
    makemove_bench(c);
    // perft_bench(c);
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_measurement(PosixTime::UserTime);
    targets = bench
}

criterion_main!(benches);
