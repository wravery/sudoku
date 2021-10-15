#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use sudoku::{Board, SolverOptions};

#[tauri::command]
async fn generate_board() -> Result<Board, String> {
  let mut board = Board::new().await;
  board.remove_values(81).await;
  Ok(board)
}

#[tauri::command]
async fn solve_value(board: Board, row: u8, column: u8) -> Result<u8, String> {
  if (0..9).contains(&row) && (0..9).contains(&column) {
    match board.solve(SolverOptions::FirstOnly).await {
      Ok(board) => Ok(board.0[row as usize][column as usize]),
      Err(solution) => Err(format!("Solver error: {:?}", solution)),
    }
  } else {
    Err(format!(
      "Out of bounds: row: {} column: {}",
      row + 1,
      column + 1
    ))
  }
}

#[tauri::command]
async fn get_possible_values(board: Board, row: u8, column: u8) -> Result<Vec<u8>, String> {
  if (0..9).contains(&row) && (0..9).contains(&column) {
    Ok(board.get_all_remaining(row, column))
  } else {
    Err(format!(
      "Out of bounds: row: {} column: {}",
      row + 1,
      column + 1
    ))
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      generate_board,
      solve_value,
      get_possible_values
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
