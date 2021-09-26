use sudoku::{Board, Solutions, SolverOptions};

#[test]
fn generate_default() {
  let mut board = Board::default();
  assert!(matches!(board.solve(SolverOptions::Random), Solutions::One));
  assert_ne!(board, Board::default());
  println!("Generated: {}", board);
}

#[test]
fn multiple_default() {
  let mut board = Board::default();
  assert!(matches!(board.solve(SolverOptions::Exhaustive), Solutions::Multiple));
  assert_eq!(board, Board::default());
}

#[test]
fn remove_one() {
  let mut board = Board::new();
  let original = Board(board.0);
  assert_eq!(board.remove_random(1), 1);
  assert_ne!(board, original);
  println!("After removing: 1 Board: {}", board);
  assert!(matches!(board.solve(SolverOptions::FirstOnly), Solutions::One));
  println!("Solution: {}", board);
}

#[test]
fn remove_all() {
  let mut board = Board::new();
  let removed = board.remove_random(81);
  assert!((2..81).contains(&removed));
  println!("After removing: {} Board: {}", removed, board);
  assert!(matches!(board.solve(SolverOptions::FirstOnly), Solutions::One));
  println!("Solution: {}", board);
}
