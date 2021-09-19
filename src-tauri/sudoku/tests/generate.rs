use sudoku::{Board, Solutions};

#[test]
fn generate_default() {
  let mut board = Board::default();
  assert!(matches!(board.solve(false), Solutions::One));
  assert_ne!(board, Board::default());
}

#[test]
fn multiple_default() {
  let mut board = Board::default();
  assert!(matches!(board.solve(true), Solutions::Multiple));
  assert_eq!(board, Board::default());
}

#[test]
fn remove_one() {
  let mut board = Board::default();
  assert!(matches!(board.solve(false), Solutions::One));
  let original = Board(board.0);
  assert_eq!(board.remove_random(1), 1);
  assert_ne!(board, original);
  assert!(matches!(board.solve(true), Solutions::One));
}

#[test]
fn remove_all() {
  let mut board = Board::default();
  assert!(matches!(board.solve(false), Solutions::One));
  let removed = board.remove_random(81);
  assert!((2..81).contains(&removed));
  println!("After removing: {} Board: {}", removed, board);
  assert!(matches!(board.solve(false), Solutions::One));
  println!("Solution: {}", board);
}
