use sudoku::{Board, BoardTestExt, Solutions, SolverOptions};

#[test]
fn generate_default() {
  let mut board = Board::default();
  assert!(matches!(
    board.test_solve(SolverOptions::Random),
    Solutions::One
  ));
  assert_ne!(board, Board::default());
  println!("Generated: {}", board);
}

#[test]
fn multiple_default() {
  let mut board = Board::default();
  assert!(matches!(
    board.test_solve(SolverOptions::Exhaustive),
    Solutions::Multiple
  ));
  assert_eq!(board, Board::default());
}

#[test]
fn remove_one() {
  let mut board = Board::test_new();
  let original = Board(board.0);
  assert_eq!(board.test_remove_values(1), 1);
  assert_ne!(board, original);
  println!("After removing: 1 Board: {}", board);
  assert!(matches!(
    board.test_solve(SolverOptions::FirstOnly),
    Solutions::One
  ));
  println!("Solution: {}", board);
}

#[test]
fn remove_all() {
  let mut board = Board::test_new();
  let removed = board.test_remove_values(81);
  assert!((2..81).contains(&removed));
  println!("After removing: {} Board: {}", removed, board);
  assert!(matches!(
    board.test_solve(SolverOptions::FirstOnly),
    Solutions::One
  ));
  println!("Solution: {}", board);
}
