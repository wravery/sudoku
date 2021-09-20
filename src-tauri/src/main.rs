#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use sudoku::{Board, Solutions};

#[tauri::command]
async fn generate_board() -> Result<String, String> {
  let mut board = Board::default();
  board.solve(false);
  board.remove_random(81);
  Ok(serde_json::to_string(&board).map_err(|err| format!("JSON error: {}", err))?)
}

#[tauri::command]
async fn solve_value(board: Board, row: u8, column: u8) -> Result<String, String> {
  if (1..=9).contains(&row) && (1..=9).contains(&column) {
    let mut solution = Board(board.0);
    match solution.solve(false) {
      Solutions::One => {
        let value = solution.0[row as usize - 1][column as usize - 1];
        solution.0 = board.0;
        solution.0[row as usize - 1][column as usize - 1] = value;
        Ok(serde_json::to_string(&solution).map_err(|err| format!("JSON error: {}", err))?)
      }
      solution => Err(format!("Solver error: {:?}", solution)),
    }
  } else {
    Err(format!("Out of bounds: row: {} column: {}", row, column))
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![generate_board, solve_value])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
