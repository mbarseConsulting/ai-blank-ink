use tauri::Manager;

use crate::config::{app_config, load_app_config, AppConfig};
use crate::filesystem::{
  create_new_story_impl, load_skill_run_history_impl, list_stories_tree_impl,
  open_document_impl, save_document_impl, save_skill_run_history_impl,
};
use crate::models::{DocumentNodeDto, SkillRunDto};
use crate::skills::run_skill_impl;

#[tauri::command]
pub(crate) fn get_app_config() -> Result<AppConfig, String> {
  load_app_config()
}

#[tauri::command]
pub(crate) fn list_stories_tree() -> Vec<DocumentNodeDto> {
  list_stories_tree_impl()
}

#[tauri::command]
pub(crate) fn open_document(path: String) -> Result<String, String> {
  open_document_impl(path)
}

#[tauri::command]
pub(crate) fn save_document(path: String, content: String) -> Result<(), String> {
  save_document_impl(path, content)
}

#[tauri::command]
pub(crate) fn create_new_story(title: String) -> Result<String, String> {
  create_new_story_impl(title)
}

/// Open (or show) the analysis window.
/// Important: command async to avoid WebView2 deadlocks on Windows.
#[tauri::command]
pub(crate) async fn open_analysis_window(
  window: tauri::AppHandle,
  _path: Option<String>,
) -> Result<(), String> {
  let cfg = app_config();
  let label = &cfg.window.analysis_label;

  if let Some(win) = window.get_webview_window(label) {
    if let Err(e) = win.show() {
      return Err(e.to_string());
    }
    if let Err(e) = win.set_focus() {
      return Err(e.to_string());
    }
  } else {
    #[cfg(debug_assertions)]
    let url = tauri::WebviewUrl::External(
      cfg.window
        .analysis_dev_url
        .parse()
        .map_err(|_| "invalid analysisDevUrl in config")?,
    );

    #[cfg(not(debug_assertions))]
    let url = tauri::WebviewUrl::App("index.html#analysis".into());

    let [w, h] = cfg.window.analysis_inner_size;
    tauri::WebviewWindowBuilder::new(&window, label, url)
      .title(&cfg.window.analysis_title)
      .inner_size(w, h)
      .resizable(true)
      .build()
      .map_err(|e| e.to_string())?;
  }

  Ok(())
}

/// Load SkillRun history for a given document+skill.
#[tauri::command]
pub(crate) fn load_skill_run_history(
  path: String,
  skill_name: String,
) -> Result<Vec<SkillRunDto>, String> {
  load_skill_run_history_impl(path, skill_name)
}

/// Save SkillRun history for a given document+skill.
#[tauri::command]
pub(crate) fn save_skill_run_history(
  path: String,
  skill_name: String,
  runs: Vec<SkillRunDto>,
) -> Result<(), String> {
  save_skill_run_history_impl(path, skill_name, runs)
}

#[tauri::command]
pub(crate) async fn run_skill(
  skill_name: String,
  path: String,
  mode: Option<String>,
  follow_up: Option<String>,
  previous_message: Option<String>,
) -> Result<SkillRunDto, String> {
  run_skill_impl(skill_name, path, mode, follow_up, previous_message).await
}

