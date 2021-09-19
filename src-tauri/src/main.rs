#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use sudoku::Board;

#[tauri::command]
async fn generate_board() -> Result<String, String> {
  let mut board = Board::default();
  board.solve(false);
  board.remove_random(81);
  Ok(serde_json::to_string(&board).map_err(|err| format!("JSON error: {}", err))?)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![generate_board])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
