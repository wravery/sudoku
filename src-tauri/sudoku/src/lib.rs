use std::fmt::Display;

use rand::prelude::*;

extern crate serde;
use serde::{Deserialize, Serialize};

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

pub enum SolverOptions {
  Random,
  FailFast,
  Exhaustive,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board(pub [[u8; 9]; 9]);

impl Board {
  pub fn solve(&mut self, options: SolverOptions) -> Solutions {
    let (exhaustive, mut rng) = match options {
      SolverOptions::Random => (false, Some(thread_rng())),
      SolverOptions::FailFast => (false, None),
      SolverOptions::Exhaustive => (true, None),
    };

    self.solve_recursive(exhaustive, &mut rng)
  }

  fn solve_recursive(&mut self, exhaustive: bool, rng: &mut Option<ThreadRng>) -> Solutions {
    for row in 0..9_u8 {
      for column in 0..9_u8 {
        if self.0[usize::from(row)][usize::from(column)] == 0 {
          let remaining = self.all_remaining(row, column);
          let mut solutions = Solutions::None;
          if remaining != 0 {
            let mut remaining = Self::expand_remaining(remaining);
            if let Some(rng) = rng {
              remaining.shuffle(rng);
            }
            for value in remaining {
              if value != 0 {
                self.0[usize::from(row)][usize::from(column)] = value;
                match (&mut solutions, self.solve_recursive(exhaustive, rng)) {
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
                    self.0[usize::from(row)][usize::from(column)] = 0;
                    return Solutions::Multiple;
                  }
                }
              }
            }
            // Put back the 0 value so the caller can try its next value.
            self.0[usize::from(row)][usize::from(column)] = 0;
          }

          return solutions;
        }
      }
    }
    // No more uninitialized values, make sure the initial state was valid.
    match self.check_all() {
      true => Solutions::One,
      false => Solutions::None,
    }
  }

  fn all_remaining(&self, row: u8, column: u8) -> u16 {
    let (mut in_row, mut in_column, mut in_square) = (None, None, None);
    if let Check::Remaining(remaining) = self.check_row(row) {
      if remaining != 0 {
        in_row = Some(remaining);
        if let Check::Remaining(remaining) = self.check_column(column) {
          if remaining != 0 {
            in_column = Some(remaining);
            if let Check::Remaining(remaining) = self.check_square(row, column) {
              if remaining != 0 {
                in_square = Some(remaining);
              }
            }
          }
        }
      }
    }

    match (in_row, in_column, in_square) {
      (Some(in_row), Some(in_column), Some(in_square)) => (in_row & in_column & in_square),
      _ => 0_u16,
    }
  }

  fn expand_remaining(remaining: u16) -> [u8; 9] {
    let mut result = [0; 9];
    if remaining != 0 {
      let mut index = 0;
      for value in 1..=9 {
        if remaining & 0b1 << (value - 1) != 0 {
          result[index] = value;
          index += 1;
        }
      }
    }
    result
  }

  pub fn remove_random(&mut self, max_count: u8) -> u8 {
    let mut choices = [None; 81];
    let mut index = 0;
    for row in 0..9_u8 {
      let filled = self.expand_filled_row(row);
      for column in filled.iter().filter_map(|c| c.as_ref()) {
        let column = *column;
        choices[index] = Some((row, column));
        index += 1;
      }
    }

    let mut rng = thread_rng();
    choices.shuffle(&mut rng);

    let mut count = 0;
    let mut rng = None;
    for (row, column) in choices.iter().filter_map(|c| c.as_ref()) {
      let test_row = usize::from(*row);
      let test_column = usize::from(*column);
      let value = self.0[test_row][test_column];
      self.0[test_row][test_column] = 0;
      if let Solutions::One = self.solve_recursive(true, &mut rng) {
        count += 1;
        if count >= max_count {
          break;
        }
      } else {
        self.0[test_row][test_column] = value;
      }
    }

    count
  }

  fn expand_filled_row(&self, row: u8) -> [Option<u8>; 9] {
    let mut result = [None; 9];
    let mut index = 0;
    for column in 0..9 {
      if self.0[usize::from(row)][usize::from(column)] != 0 {
        result[index] = Some(column);
        index += 1;
      }
    }
    result
  }

  fn check_all(&self) -> bool {
    for i in 0..9_u8 {
      if let Check::Duplicates = self.check_row(i) {
        return false;
      }

      if let Check::Duplicates = self.check_column(i) {
        return false;
      }

      let row = ((i / 3) * 3) + 1;
      let column = ((i % 3) * 3) + 1;
      if let Check::Duplicates = self.check_square(row, column) {
        return false;
      }
    }

    true
  }

  pub fn check_row(&self, row: u8) -> Check {
    if (0..9).contains(&row) {
      Self::check_slice(&self.0[usize::from(row)])
    } else {
      Check::Remaining(MASK_ALL)
    }
  }

  pub fn check_column(&self, column: u8) -> Check {
    if (0..9).contains(&column) {
      Self::check_slice(&[
        self.0[0][usize::from(column)],
        self.0[1][usize::from(column)],
        self.0[2][usize::from(column)],
        self.0[3][usize::from(column)],
        self.0[4][usize::from(column)],
        self.0[5][usize::from(column)],
        self.0[6][usize::from(column)],
        self.0[7][usize::from(column)],
        self.0[8][usize::from(column)],
      ])
    } else {
      Check::Remaining(MASK_ALL)
    }
  }

  pub fn check_square(&self, row: u8, column: u8) -> Check {
    if (0..9).contains(&row) && (0..9).contains(&column) {
      let start_row = (row / 3) * 3;
      let start_column = (column / 3) * 3;
      Self::check_slice(&[
        self.0[usize::from(start_row)][usize::from(start_column)],
        self.0[usize::from(start_row)][usize::from(start_column) + 1],
        self.0[usize::from(start_row)][usize::from(start_column) + 2],
        self.0[usize::from(start_row) + 1][usize::from(start_column)],
        self.0[usize::from(start_row) + 1][usize::from(start_column) + 1],
        self.0[usize::from(start_row) + 1][usize::from(start_column) + 2],
        self.0[usize::from(start_row) + 2][usize::from(start_column)],
        self.0[usize::from(start_row) + 2][usize::from(start_column) + 1],
        self.0[usize::from(start_row) + 2][usize::from(start_column) + 2],
      ])
    } else {
      Check::Remaining(MASK_ALL)
    }
  }

  fn check_slice(slice: &[u8; 9]) -> Check {
    let mut values = 0;
    for value in slice {
      let mask = match value {
        _ if (1..=9).contains(value) => 1_u16 << (value - 1),
        0 => 0,
        _ => unreachable!(),
      };

      if mask & values != 0 {
        return Check::Duplicates;
      }

      values |= mask;
    }

    match values ^ MASK_ALL {
      0 => Check::Full,
      remaining => Check::Remaining(remaining),
    }
  }
}

fn display_value(value: u8) -> String {
  if (1..=9).contains(&value) {
    value.to_string()
  } else {
    " ".to_string()
  }
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
}
