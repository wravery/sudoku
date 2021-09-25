use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku::Board;

fn remove_all(c: &mut Criterion) {
  let mut board = Board::new();
  c.bench_function("remove all", |b| {
    b.iter(|| board.remove_random(black_box(81)))
  });
}

fn remove_50(c: &mut Criterion) {
  let mut board = Board::new();
  c.bench_function("remove 50", |b| {
    b.iter(|| board.remove_random(black_box(50)))
  });
}

fn remove_40(c: &mut Criterion) {
  let mut board = Board::new();
  c.bench_function("remove 40", |b| {
    b.iter(|| board.remove_random(black_box(40)))
  });
}

fn remove_30(c: &mut Criterion) {
  let mut board = Board::new();
  c.bench_function("remove 30", |b| {
    b.iter(|| board.remove_random(black_box(30)))
  });
}

criterion_group!(benches, remove_all, remove_50, remove_40, remove_30);
criterion_main!(benches);
