use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use dorpsgek_movegen::Board;
use dorpsgek::Search;
use tinyvec::ArrayVec;

pub fn search_bench(c: &mut Criterion) {
    let kiwipete =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();

    let mut group = c.benchmark_group("kiwipete");

    group.sample_size(5_000);
    group.significance_level(0.005);
    group.noise_threshold(0.025);

    let nodes = {
        let mut s = Search::new();
        let mut pv = ArrayVec::new();
        s.search_root(&kiwipete, 3, &mut pv);
        s.nodes() + s.qnodes()
    };

    group.throughput(Throughput::Elements(nodes));
    group.bench_with_input("kiwipete-3", &kiwipete, |b, board| {
        let mut s = Search::new();
        let mut pv = ArrayVec::new();
        b.iter(|| {
            s.search_root(board, 3, &mut pv);
        })
    });

    let nodes = {
        let mut s = Search::new();
        let mut pv = ArrayVec::new();
        s.search_root(&kiwipete, 4, &mut pv);
        s.nodes() + s.qnodes()
    };

    group.throughput(Throughput::Elements(nodes));
    group.bench_with_input("kiwipete-4", &kiwipete, |b, board| {
        let mut s = Search::new();
        let mut pv = ArrayVec::new();
        b.iter(|| {
            s.search_root(board, 4, &mut pv);
        })
    });

    group.finish();
}

pub fn bench(c: &mut Criterion) {
    search_bench(c);
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench
}

criterion_main!(benches);
