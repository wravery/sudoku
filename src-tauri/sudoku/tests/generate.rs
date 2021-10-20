use sudoku::{Board, BoardTestExt, Solutions, SolverOptions};

#[test]
fn generate_default() {
  let board = Board::default()
    .test_solve(SolverOptions::Random)
    .expect("should succeed");
  assert_ne!(board, Board::default());
  println!("Generated: {}", board);
}

#[test]
fn multiple_default() {
  assert!(matches!(
    Board::default().test_solve(SolverOptions::Exhaustive),
    Err(Solutions::Multiple)
  ));
}

#[test]
fn remove_one() {
  let mut board = Board::test_new();
  let original = Board(board.0);
  assert_eq!(board.test_remove_values(1), 1);
  assert_ne!(board, original);
  println!("After removing: 1 Board: {}", board);
  let board = board.test_solve(SolverOptions::FirstOnly).expect("should succeed");
  println!("Solution: {}", board);
}

#[test]
fn remove_all() {
  let mut board = Board::test_new();
  let removed = board.test_remove_values(81);
  assert!((2..81).contains(&removed));
  println!("After removing: {} Board: {}", removed, board);
  let board = board.test_solve(SolverOptions::FirstOnly).expect("should succeed");
  println!("Solution: {}", board);
}
