use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku::{Board, Solutions};

fn remove_all(c: &mut Criterion) {
  let mut board = Board::default();
  assert!(matches!(board.solve(false), Solutions::One));
  c.bench_function("remove all", |b| {
    b.iter(|| board.remove_random(black_box(81)))
  });
}

criterion_group!(benches, remove_all);
criterion_main!(benches);
