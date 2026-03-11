use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;

// Dossier racine du workspace pendant le dev :
// src-tauri/Cargo.toml se trouve dans writer-app/src-tauri
// On remonte donc de deux niveaux pour arriver à ai-blank-ink/
const WORKSPACE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn resolve_path(p: &str) -> PathBuf {
  let candidate = Path::new(p);
  if candidate.is_absolute() {
    return candidate.to_path_buf();
  }

  PathBuf::from(WORKSPACE_ROOT).join(p)
}

#[derive(serde::Serialize)]
struct DocumentNodeDto {
  id: String,
  label: String,
  kind: String,
  path: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  children: Option<Vec<DocumentNodeDto>>,
}

fn scan_stories_dir(base: &Path, rel_prefix: &str) -> Vec<DocumentNodeDto> {
  let mut result = Vec::new();
  let Ok(entries) = fs::read_dir(base) else {
    return result;
  };

  let mut dirs: Vec<(PathBuf, String)> = Vec::new();
  let mut files: Vec<(PathBuf, String)> = Vec::new();

  for entry in entries.flatten() {
    let path = entry.path();
    let name = path
      .file_name()
      .and_then(|n| n.to_str())
      .unwrap_or("")
      .to_string();

    if name.is_empty() || name.starts_with('.') {
      continue;
    }

    if path.is_dir() {
      dirs.push((path, name));
    } else if path.extension().map_or(false, |e| e == "md") {
      files.push((path, name));
    }
  }

  dirs.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
  files.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));

  for (path, name) in dirs {
    let rel_path = format!("{}{}/", rel_prefix, name);
    let path_str = rel_path.trim_end_matches('/');
    let id = format!(
      "folder-{}",
      path_str.replace('/', "-").replace('.', "_")
    );
    let child_rel = format!("{}{}/", rel_prefix, name);
    let children = scan_stories_dir(&path, &child_rel);
    result.push(DocumentNodeDto {
      id,
      label: name,
      kind: "folder".to_string(),
      path: path_str.to_string(),
      children: if children.is_empty() {
        None
      } else {
        Some(children)
      },
    });
  }

  for (_path, name) in files {
    let rel_path = format!("{}{}", rel_prefix, name);
    let id = format!(
      "doc-{}",
      rel_path.replace('/', "-").replace(".md", "")
    );
    result.push(DocumentNodeDto {
      id,
      label: name,
      kind: "document".to_string(),
      path: rel_path,
      children: None,
    });
  }

  result
}

#[tauri::command]
fn list_stories_tree() -> Vec<DocumentNodeDto> {
  let root = PathBuf::from(WORKSPACE_ROOT).join("stories");
  if !root.exists() || !root.is_dir() {
    return Vec::new();
  }
  scan_stories_dir(&root, "stories/")
}

#[tauri::command]
fn open_document(path: String) -> Result<String, String> {
  let full = resolve_path(&path);
  std::fs::read_to_string(&full).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_document(path: String, content: String) -> Result<(), String> {
  let full = resolve_path(&path);
  std::fs::write(&full, content).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
      list_stories_tree,
      open_document,
      save_document
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
