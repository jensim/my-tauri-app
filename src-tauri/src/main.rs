#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use generic_error::Result;

// Defining commands is no longer a multi-file mess
#[tauri::command]
async fn my_custom_command() -> Option<String> {
  println!("Im called");
  match my_custom_fn().await {
    Ok(a) => Some(a),
    Err(e) => {
      println!("error = {}", e);
      None
    }
  }
}

#[derive(serde::Deserialize, Debug)]
struct CrateResponse {
  #[serde(rename(deserialize = "crate"))]
  krate: CrateData,
}

#[derive(serde::Deserialize, Debug)]
struct CrateData {
  max_stable_version: String,
}

async fn my_custom_fn() -> Result<String> {
  let client = reqwest::Client::new();
  let body: CrateResponse = client
    .get("https://crates.io/api/v1/crates/tauri")
    .header(
      "User-Agent",
      "my-tauri-app https://github.com/jensim/my-tauri-app",
    )
    .send()
    .await?
    .json::<CrateResponse>()
    .await?;
  Ok(body.krate.max_stable_version)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
