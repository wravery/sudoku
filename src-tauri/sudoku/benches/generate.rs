use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku::{Board, BoardTestExt};

fn remove_all(c: &mut Criterion) {
  c.bench_function("remove all", |b| {
    b.iter(|| {
      let mut board = Board::test_new();
      board.test_remove_values(black_box(81));
    })
  });
}

fn remove_50(c: &mut Criterion) {
  c.bench_function("remove 50", |b| {
    b.iter(|| {
      let mut board = Board::test_new();
      board.test_remove_values(black_box(50));
    })
  });
}

fn remove_40(c: &mut Criterion) {
  c.bench_function("remove 40", |b| {
    b.iter(|| {
      let mut board = Board::test_new();
      board.test_remove_values(black_box(40));
    })
  });
}

fn remove_30(c: &mut Criterion) {
  c.bench_function("remove 30", |b| {
    b.iter(|| {
      let mut board = Board::test_new();
      board.test_remove_values(black_box(30));
    })
  });
}

criterion_group!(benches, remove_all, remove_50, remove_40, remove_30);
criterion_main!(benches);
