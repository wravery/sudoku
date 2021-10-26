use std::fmt::Display;

use futures::{
  executor::block_on,
  future::{select_all, BoxFuture},
  prelude::*,
};
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
  Multiple,
}

const MASK_ALL: u16 = 0x1FF;

#[derive(Clone)]
pub enum SolverOptions {
  Random,
  FirstOnly,
  Exhaustive,
}

pub trait BoardRng<R: RngCore> {
  fn get_rng() -> R;
}

impl BoardRng<ThreadRng> for ThreadRng {
  fn get_rng() -> ThreadRng {
    thread_rng()
  }
}

impl BoardRng<StdRng> for StdRng {
  fn get_rng() -> StdRng {
    StdRng::seed_from_u64(0)
  }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board(pub [[u8; 9]; 9]);

impl Board {
  pub async fn new() -> Board {
    Self::create::<ThreadRng>().await
  }

  async fn create<R: RngCore + BoardRng<R>>() -> Board {
    let mut board = Self::default();
    for i in 0..9_u8 {
      board.0[0][usize::from(i)] = i + 1;
    }

    {
      let mut rng = R::get_rng();
      board.0[0].shuffle(&mut rng);
    }

    board
      .solve_recursive::<R>(SolverOptions::Random)
      .await
      .expect("Should always find at least one solution")
  }

  pub async fn solve(self, options: SolverOptions) -> Result<Board, Solutions> {
    self.solve_recursive::<ThreadRng>(options).await
  }

  fn solve_recursive<'a, R: RngCore + BoardRng<R>>(
    self,
    options: SolverOptions,
  ) -> BoxFuture<'a, Result<Board, Solutions>> {
    async move {
      for row in 0..9_u8 {
        for column in 0..9_u8 {
          if self.0[usize::from(row)][usize::from(column)] == 0 {
            let remaining = self.all_remaining(row, column);
            let mut solutions = Some(Solutions::None);
            if remaining != 0 {
              let mut remaining = Self::expand_remaining(remaining);
              if let SolverOptions::Random = options {
                let mut rng = R::get_rng();
                remaining.shuffle(&mut rng);
              }
              let mut test_results: Vec<_> = remaining
                .iter()
                .filter_map(|value| match value {
                  0 => None,
                  _ => Some({
                    let mut test_board = Board(self.0);
                    test_board.0[usize::from(row)][usize::from(column)] = *value;
                    test_board.solve_recursive::<R>(options.clone())
                  }),
                })
                .collect();

              loop {
                if test_results.is_empty() {
                  break;
                }

                let (test_result, _, remaining) = select_all(test_results).await;
                test_results = remaining;
                match (&mut solutions, test_result) {
                  (_, Err(Solutions::None)) => (),
                  (Some(Solutions::None), Ok(board)) => {
                    solutions = None;
                    match options {
                      SolverOptions::Exhaustive => (),
                      _ => {
                        // Recursively solved with that value.
                        return Ok(board);
                      }
                    };
                  }
                  (_, _) => {
                    // Put back the 0 value to restore the board to its original state.
                    return Err(Solutions::Multiple);
                  }
                }
              }
            }

            // Put back the 0 value so the caller can try its next value.
            return match solutions {
              None => Ok(self),
              Some(solutions) => Err(solutions),
            };
          }
        }
      }
      // No more uninitialized values, make sure the initial state was valid.
      match self.check_all() {
        true => Ok(self),
        false => Err(Solutions::None),
      }
    }
    .boxed()
  }

  fn all_remaining(&self, row: u8, column: u8) -> u16 {
    let mut all = MASK_ALL;

    if let Check::Remaining(remaining) = self.check_row(row) {
      all &= remaining;
    }

    if all != 0 {
      if let Check::Remaining(remaining) = self.check_column(column) {
        all &= remaining;
      }
    }

    if all != 0 {
      if let Check::Remaining(remaining) = self.check_square(row, column) {
        all &= remaining;
      }
    }

    all
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

  pub async fn remove_values(&mut self, max_count: u8) -> u8 {
    self.remove_random::<ThreadRng>(max_count).await
  }

  async fn remove_random<R: RngCore + BoardRng<R>>(&mut self, max_count: u8) -> u8 {
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

    {
      let mut rng = R::get_rng();
      choices.shuffle(&mut rng);
    }

    let mut count = 0;
    let mut next = 0_usize;
    let mut binary_search = true;
    while binary_search && count < max_count && next < choices.len() {
      // Perform a binary search to find the next cell that can't be removed.
      let mut low = next;
      let mut high = choices.len();
      let mut test_count = 0_usize;
      let previous = next;
      while low < high && count < max_count {
        let middle = (low + high) / 2;
        let mut test_board: Option<Board> = None;
        for choice in choices[next..=middle].iter() {
          match choice {
            Some((row, column)) => {
              let mut board = test_board.map_or_else(|| Board(self.0), |board| board);
              let test_row = usize::from(*row);
              let test_column = usize::from(*column);
              board.0[test_row][test_column] = 0;
              test_board = Some(board);
            }
            None => (),
          }
        }

        if test_board.is_some() {
          test_count += 1;
        }

        if test_board.is_none()
          || test_board
            .unwrap()
            .solve_recursive::<R>(SolverOptions::Exhaustive)
            .await
            .is_ok()
        {
          while count < max_count && next <= middle {
            if let Some((row, column)) = choices[next].take() {
              let row = usize::from(row);
              let column = usize::from(column);
              self.0[row][column] = 0;
              count += 1;
            }
            next += 1;
          }
          low = next;
        } else {
          high = middle;
        }
      }

      if previous + test_count > next {
        // Stop using the binary search once we no longer break even on tests vs. progress.
        binary_search = false;
      }

      choices[next] = None;
      next += 1;
    }

    if !binary_search {
      // Finish removing candidates using a linear scan.
      while count < max_count && next < choices.len() {
        if let Some((row, column)) = choices[next].take() {
          let test_row = usize::from(row);
          let test_column = usize::from(column);
          let mut test_board = Board(self.0);
          test_board.0[test_row][test_column] = 0;

          if test_board
            .solve_recursive::<R>(SolverOptions::Exhaustive)
            .await
            .is_ok()
          {
            self.0[test_row][test_column] = 0;
            count += 1;
          }
        }

        next += 1;
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

  pub fn get_all_remaining(&self, row: u8, column: u8) -> Vec<u8> {
    Self::expand_remaining(self.all_remaining(row, column))
      .iter()
      .filter_map(|v| match v {
        1..=9 => Some(*v),
        _ => None,
      })
      .collect()
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

pub trait BoardTestExt {
  fn test_new() -> Board;
  fn test_solve(self, options: SolverOptions) -> Result<Board, Solutions>;
  fn test_remove_values(&mut self, max_count: u8) -> u8;
}

impl BoardTestExt for Board {
  fn test_new() -> Board {
    block_on(Self::create::<StdRng>())
  }

  fn test_solve(self, options: SolverOptions) -> Result<Board, Solutions> {
    block_on(Board(self.0).solve_recursive::<StdRng>(options))
  }

  fn test_remove_values(&mut self, max_count: u8) -> u8 {
    block_on(self.remove_random::<StdRng>(max_count))
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
