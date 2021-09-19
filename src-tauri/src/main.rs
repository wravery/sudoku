#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate serde;

mod board;
use board::Board;

use rand::prelude::*;

#[tauri::command]
async fn generate_board() -> Result<String, String> {
  let mut board = Board::default();
  board.solve(false);
  let mut rng = thread_rng();
  loop {
    if !board.remove_random(&mut rng) {
      break;
    }
  }
  Ok(serde_json::to_string(&board).map_err(|err| format!("JSON error: {}", err))?)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![generate_board])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
