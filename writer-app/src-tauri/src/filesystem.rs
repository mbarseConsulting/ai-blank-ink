use std::fs;
use std::path::{Path, PathBuf};

use crate::config::app_config;
use crate::models::{DocumentNodeDto, SkillRunDto};
use crate::util::{normalize_newlines, resolve_path, slugify_title};

fn history_dir() -> PathBuf {
  PathBuf::from(crate::WORKSPACE_ROOT).join(&app_config().paths.analysis_history_dir)
}

fn history_file_for(path: &str, skill_name: &str) -> PathBuf {
  let mut slug = path.replace(['/', '\\'], "-");
  slug = slug.replace('.', "_");
  history_dir().join(format!("{}.{}.json", slug, skill_name))
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

pub(crate) fn list_stories_tree_impl() -> Vec<DocumentNodeDto> {
  let cfg = app_config();
  let root = PathBuf::from(crate::WORKSPACE_ROOT).join(&cfg.paths.stories_dir);
  if !root.exists() || !root.is_dir() {
    return Vec::new();
  }
  let prefix = format!("{}/", cfg.paths.stories_dir);
  scan_stories_dir(&root, &prefix)
}

pub(crate) fn open_document_impl(path: String) -> Result<String, String> {
  let full = resolve_path(&path);
  std::fs::read_to_string(&full)
    .map(|s| normalize_newlines(&s))
    .map_err(|e| e.to_string())
}

pub(crate) fn save_document_impl(path: String, content: String) -> Result<(), String> {
  let full = resolve_path(&path);
  std::fs::write(&full, content).map_err(|e| e.to_string())
}

pub(crate) fn create_new_story_impl(title: String) -> Result<String, String> {
  let cfg = app_config();
  let stories_root = PathBuf::from(crate::WORKSPACE_ROOT).join(&cfg.paths.stories_dir);

  if !stories_root.exists() {
    fs::create_dir_all(&stories_root)
      .map_err(|e| format!("Failed to create stories dir {}: {}", stories_root.display(), e))?;
  }

  let base_slug = slugify_title(&title);
  let mut slug = base_slug.clone();
  let mut counter = 1usize;

  loop {
    let folder_path = stories_root.join(&slug);
    if !folder_path.exists() {
      fs::create_dir_all(&folder_path)
        .map_err(|e| format!("Failed to create story folder {}: {}", folder_path.display(), e))?;
    }

    let file_name = format!("{}.md", slug);
    let full_path = folder_path.join(&file_name);

    if !full_path.exists() {
      let header = format!("# {}\n\n", title.trim());
      fs::write(&full_path, header)
        .map_err(|e| format!("Failed to write new story at {}: {}", full_path.display(), e))?;

      let rel_path = format!("{}/{}/{}", cfg.paths.stories_dir, slug, file_name);
      return Ok(rel_path);
    }

    counter += 1;
    slug = format!("{}-{}", base_slug, counter);
  }
}

pub(crate) fn load_skill_run_history_impl(
  path: String,
  skill_name: String,
) -> Result<Vec<SkillRunDto>, String> {
  let file = history_file_for(&path, &skill_name);
  if !file.exists() {
    return Ok(Vec::new());
  }
  let data = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;
  let runs: Vec<SkillRunDto> =
    serde_json::from_str(&data).map_err(|e| format!("Failed to parse history: {}", e))?;
  Ok(runs)
}

pub(crate) fn save_skill_run_history_impl(
  path: String,
  skill_name: String,
  runs: Vec<SkillRunDto>,
) -> Result<(), String> {
  let dir = history_dir();
  if let Err(e) = std::fs::create_dir_all(&dir) {
    return Err(e.to_string());
  }
  let file = history_file_for(&path, &skill_name);
  let json = serde_json::to_string_pretty(&runs).map_err(|e| e.to_string())?;
  std::fs::write(&file, json).map_err(|e| e.to_string())
}

