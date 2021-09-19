use std::fmt::Display;

use rand::prelude::*;
use serde::Serialize;

#[derive(Debug)]
pub enum Check {
  Full,
  Remaining(u16),
  Duplicates,
}

#[derive(Debug)]
pub enum Solutions {
  None,
  One,
  Multiple,
}

const MASK_ALL: u16 = 0b111111111;

#[derive(Debug, Default, PartialEq, Eq, Serialize)]
pub struct Board(pub [[u8; 9]; 9]);

impl Board {
  pub fn solve(&mut self, exhaustive: bool) -> Solutions {
    let mut rng = thread_rng();
    for row in 1..=9 as u8 {
      for column in 1..=9 as u8 {
        if self.0[row as usize - 1][column as usize - 1] == 0 {
          if let (
            Check::Remaining(in_row),
            Check::Remaining(in_column),
            Check::Remaining(in_square),
          ) = (
            self.check_row(row),
            self.check_column(column),
            self.check_square(row, column),
          ) {
            let mut solutions = Solutions::None;
            let mut remaining = Self::expand_remaining(in_row & in_column & in_square);
            remaining.shuffle(&mut rng);
            for value in remaining {
              self.0[row as usize - 1][column as usize - 1] = value;
              match (&mut solutions, self.solve(exhaustive)) {
                (_, Solutions::None) => (),
                (Solutions::None, Solutions::One) => {
                  solutions = Solutions::One;
                  if !exhaustive {
                    // Recursively solved with that value.
                    return solutions;
                  }
                }
                (_, Solutions::One | Solutions::Multiple) => {
                  // Put back the 0 value to restore the board to its original state.
                  self.0[row as usize - 1][column as usize - 1] = 0;
                  return Solutions::Multiple;
                }
              }
            }

            // Put back the 0 value so the caller can try its next value.
            self.0[row as usize - 1][column as usize - 1] = 0;
            return solutions;
          }
        }
      }
    }
    // No more uninitialized values, make sure the initial state was valid.
    match self.check_all() {
      true => Solutions::One,
      false => Solutions::None,
    }
  }

  fn expand_remaining(remaining: u16) -> Vec<u8> {
    let mut result = Vec::new();
    for value in 1..=9 {
      if remaining & 0b1 << (value - 1) != 0 {
        result.push(value);
      }
    }
    result
  }

  pub fn remove_random(&mut self, rng: &mut ThreadRng) -> bool {
    let mut choices = Vec::new();
    for row in 1..=9 as u8 {
      let filled = self.expand_filled_row(row);
      if let Some(column) = filled.choose(rng) {
        choices.push((row, *column));
      }
    }

    choices.shuffle(rng);
    for (row, column) in choices {
      let value = self.0[row as usize - 1][column as usize - 1];
      self.0[row as usize - 1][column as usize - 1] = 0;
      if let Solutions::One = self.solve(true) {
        return true;
      }
      self.0[row as usize - 1][column as usize - 1] = value;
    }

    false
  }

  fn expand_filled_row(&self, row: u8) -> Vec<u8> {
    let mut result = Vec::new();
    for column in 1..=9 {
      if self.0[row as usize - 1][column as usize - 1] != 0 {
        result.push(column);
      }
    }
    result
  }

  pub fn check_all(&self) -> bool {
    for i in 1..=9 as u8 {
      if let Check::Duplicates = self.check_row(i) {
        return false;
      }

      if let Check::Duplicates = self.check_column(i) {
        return false;
      }

      let row = (((i - 1) / 3) * 3) + 2;
      let column = (((i - 1) % 3) * 3) + 2;
      if let Check::Duplicates = self.check_square(row, column) {
        return false;
      }
    }

    true
  }
  pub fn check_row(&self, row: u8) -> Check {
    if row < 1 || row > 9 {
      Check::Remaining(MASK_ALL)
    } else {
      Self::check_slice(&self.0[row as usize - 1])
    }
  }

  pub fn check_column(&self, column: u8) -> Check {
    if column < 1 || column > 9 {
      Check::Remaining(MASK_ALL)
    } else {
      Self::check_slice(&[
        self.0[0][column as usize - 1],
        self.0[1][column as usize - 1],
        self.0[2][column as usize - 1],
        self.0[3][column as usize - 1],
        self.0[4][column as usize - 1],
        self.0[5][column as usize - 1],
        self.0[6][column as usize - 1],
        self.0[7][column as usize - 1],
        self.0[8][column as usize - 1],
      ])
    }
  }

  pub fn check_square(&self, row: u8, column: u8) -> Check {
    if row < 1 || row > 9 || column < 1 || column > 9 {
      Check::Remaining(MASK_ALL)
    } else {
      let start_row = ((row - 1) / 3) * 3;
      let start_column = ((column - 1) / 3) * 3;
      Self::check_slice(&[
        self.0[start_row as usize][start_column as usize],
        self.0[start_row as usize][start_column as usize + 1],
        self.0[start_row as usize][start_column as usize + 2],
        self.0[start_row as usize + 1][start_column as usize],
        self.0[start_row as usize + 1][start_column as usize + 1],
        self.0[start_row as usize + 1][start_column as usize + 2],
        self.0[start_row as usize + 2][start_column as usize],
        self.0[start_row as usize + 2][start_column as usize + 1],
        self.0[start_row as usize + 2][start_column as usize + 2],
      ])
    }
  }

  fn check_slice(slice: &[u8; 9]) -> Check {
    let mut values = 0;
    for value in slice {
      let mask = match value {
        _ if *value > 0 && *value < 10 => (1 as u16) << (value - 1),
        0 => 0,
        _ => unreachable!(),
      };

      if mask & values != 0 {
        return Check::Duplicates;
      }

      if *value > 0 && *value < 10 {
        values |= mask;
      }
    }

    match values ^ MASK_ALL {
      0 => Check::Full,
      remaining => Check::Remaining(remaining),
    }
  }
}

fn display_value(value: u8) -> String {
  format!(
    "{}",
    if value < 1 || value > 9 {
      " ".to_string()
    } else {
      value.to_string()
    }
  )
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f)?;
    writeln!(f, "=========================================")?;
    for row in 0..9 {
      writeln!(
        f,
        "|| {} | {} | {} || {} | {} | {} || {} | {} | {} ||",
        display_value(self.0[row][0]),
        display_value(self.0[row][1]),
        display_value(self.0[row][2]),
        display_value(self.0[row][3]),
        display_value(self.0[row][4]),
        display_value(self.0[row][5]),
        display_value(self.0[row][6]),
        display_value(self.0[row][7]),
        display_value(self.0[row][8]),
      )?;
      if row % 3 == 2 {
        writeln!(f, "=========================================")?;
      } else {
        writeln!(f, "|+---+---+---++---+---+---++---+---+---+|")?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn check_full() {
    match Board::check_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9]) {
      Check::Full => (),
      result => panic!("{:?}", result),
    };
  }

  #[test]
  fn check_remaining() {
    match Board::check_slice(&[1, 0, 3, 0, 5, 0, 7, 0, 9]) {
      Check::Remaining(0b010101010) => (),
      result => panic!("{:?}", result),
    };
  }

  #[test]
  fn check_duplicate() {
    match Board::check_slice(&[1, 2, 3, 4, 5, 4, 3, 2, 1]) {
      Check::Duplicates => (),
      result => panic!("{:?}", result),
    };
  }

  #[test]
  fn check_empty() {
    let board = Board([
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
    assert!(board.check_all());
  }

  #[test]
  fn check_non_overlapping() {
    let board = Board([
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
    assert!(board.check_all());
  }

  #[test]
  fn fail_overlapping_row() {
    let board = Board([
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 1, 0, 0, 0, 0, 0, 1, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
    assert!(!board.check_all());
  }

  #[test]
  fn fail_overlapping_column() {
    let board = Board([
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 1, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 1, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
    assert!(!board.check_all());
  }

  #[test]
  fn fail_overlapping_square() {
    let board = Board([
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 1, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 1, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
    assert!(!board.check_all());
  }

  #[test]
  fn generate_default() {
    let mut board = Board::default();
    assert!(matches!(board.solve(false), Solutions::One));
    assert_ne!(board, Board::default());
    assert!(board.check_all());
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
    let mut rng = thread_rng();
    assert!(board.remove_random(&mut rng));
    assert_ne!(board, original);
    assert!(matches!(board.solve(true), Solutions::One));
  }

  #[test]
  fn remove_all() {
    let mut board = Board::default();
    assert!(matches!(board.solve(false), Solutions::One));
    let mut removed = 0;
    let mut rng = thread_rng();
    loop {
      let previous = Board(board.0);
      if !board.remove_random(&mut rng) {
        break;
      }
      assert_ne!(board, previous);
      removed += 1;
    }
    assert!(removed > 1);
    println!("After removing: {} Board: {}", removed, board);
    assert!(matches!(board.solve(false), Solutions::One));
    println!("Solution: {}", board);
  }
}
