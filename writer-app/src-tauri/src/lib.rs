#![allow(dead_code)]

mod api;
mod config;
mod filesystem;
mod gemini;
mod models;
mod skills;
mod util;

// Workspace root during dev.
// `src-tauri/Cargo.toml` lives in `writer-app/src-tauri`, so we go up two levels.
pub(crate) const WORKSPACE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  use api::{
    create_new_story, get_app_config, load_skill_run_history, list_stories_tree,
    open_analysis_window, open_document, run_skill, save_document, save_skill_run_history,
  };

  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_app_config,
      list_stories_tree,
      open_document,
      save_document,
      create_new_story,
      open_analysis_window,
      load_skill_run_history,
      save_skill_run_history,
      run_skill
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}