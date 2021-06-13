#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use generic_error::Result;

// Defining commands is no longer a multi-file mess
#[tauri::command]
async fn my_custom_command() -> String {
  println!("Im called");
  let s = match my_custom_fn().await {
    Ok(a) => a,
    Err(e) => format!("error = {}", e),
  };
  //println!("Returning {}", s);
  s
}

async fn my_custom_fn() -> Result<String> {
  let body = reqwest::get("https://www.rust-lang.org")
      .await?
      .text()
      .await?;
  Ok(format!("body = {:?}", body))
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
