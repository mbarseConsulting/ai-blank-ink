use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;
use reqwest::Client;
use tauri::Manager;
use uuid::Uuid;

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
#[serde(rename_all = "camelCase")]
struct DiagnosticDto {
  id: String,
  axis: String,
  message: String,
  severity: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  start_offset: Option<usize>,
  #[serde(skip_serializing_if = "Option::is_none")]
  end_offset: Option<usize>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct PatchDto {
  id: String,
  skill_run_id: String,
  start_offset: usize,
  end_offset: usize,
  before_text: String,
  after_text: String,
  status: String,
  axis: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  note: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SkillRunDto {
  id: String,
  document_id: String,
  skill_name: String,
  mode: String,
  scope: String,
  created_at: String,
  diagnostics: Vec<DiagnosticDto>,
  patches: Vec<PatchDto>,
}

fn load_skill_system_prompt(skill_name: &str) -> Result<String, String> {
  // Chemin attendu : deps/ai-write-ink/skills/<skill_name>/SKILL.md
  let skills_root = PathBuf::from(WORKSPACE_ROOT)
    .join("deps")
    .join("ai-write-ink")
    .join("skills")
    .join(skill_name)
    .join("SKILL.md");

  std::fs::read_to_string(&skills_root).map_err(|e| {
    format!(
      "Failed to read SKILL.md for {} at {}: {}",
      skill_name,
      skills_root.display(),
      e
    )
  })
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

/// Ouvre (ou affiche) la fenêtre d'analyse dédiée aux rapports de skills.
/// Important: command async pour éviter le deadlock Webview2 sous Windows.
#[tauri::command]
async fn open_analysis_window(window: tauri::AppHandle, path: Option<String>) -> Result<(), String> {
  // Essayer de récupérer une fenêtre existante.
  if let Some(win) = window.get_webview_window("analysis") {
    if let Err(e) = win.show() {
      return Err(e.to_string());
    }
    if let Err(e) = win.set_focus() {
      return Err(e.to_string());
    }
  } else {
    // Créer une nouvelle fenêtre pointant vers la même app mais avec un hash #analysis.
    // En dev, on pointe explicitement sur le devUrl avec #analysis pour éviter les soucis de chemin.
    #[cfg(debug_assertions)]
    let url = tauri::WebviewUrl::External(
      "http://localhost:4200/#analysis"
        .parse()
        .expect("invalid dev URL for analysis window"),
    );

    #[cfg(not(debug_assertions))]
    let url = tauri::WebviewUrl::App("index.html#analysis".into());

    tauri::WebviewWindowBuilder::new(&window, "analysis", url)
    .title("Ghost Writer – Analysis Desk")
    .inner_size(1200.0, 800.0)
    .resizable(true)
    .build()
    .map_err(|e| e.to_string())?;
  }

  // TODO: utiliser `path` pour envoyer un événement à la fenêtre d'analyse
  // afin qu'elle sache quel document est actif.

  Ok(())
}

// ---------- Gemini integration: qa-reader (diagnostics only) ----------

#[derive(serde::Serialize)]
struct GeminiPart {
  text: String,
}

#[derive(serde::Serialize)]
struct GeminiContent {
  role: String,
  parts: Vec<GeminiPart>,
}

#[derive(serde::Serialize)]
struct GeminiRequestBody {
  contents: Vec<GeminiContent>,
  #[serde(skip_serializing_if = "Option::is_none")]
  system_instruction: Option<GeminiContent>,
}

#[derive(serde::Deserialize)]
struct GeminiResponse {
  #[serde(default)]
  candidates: Vec<GeminiCandidate>,
}

#[derive(serde::Deserialize)]
struct GeminiCandidate {
  content: GeminiContentResponse,
}

#[derive(serde::Deserialize)]
struct GeminiContentResponse {
  parts: Vec<GeminiPartResponse>,
}

#[derive(serde::Deserialize)]
struct GeminiPartResponse {
  #[serde(default)]
  text: String,
}

#[tauri::command]
async fn run_skill(
  skill_name: String,
  path: String,
  mode: Option<String>,
  follow_up: Option<String>,
  previous_message: Option<String>,
) -> Result<SkillRunDto, String> {
  // Pour l’instant on ne supporte que qa-reader en vrai appel modèle.
  if skill_name != "qa-reader" {
    return Err("run_skill currently only supports qa-reader; use run_skill_mock for others."
      .to_string());
  }

  // Clé Gemini (Google AI for Developers)
  let api_key =
    std::env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY is not set".to_string())?;
  // Default to a current Gemini model; can be overridden via GEMINI_MODEL env var.
  let model = std::env::var("GEMINI_MODEL")
    .unwrap_or_else(|_| "gemini-2.5-flash-lite".to_string());

  let full = resolve_path(&path);
  let content =
    std::fs::read_to_string(&full).map_err(|e| format!("Failed to read document: {}", e))?;

  // Charger le vrai SKILL.md de qa-reader comme instructions système.
  let skill_instructions = load_skill_system_prompt(&skill_name)?;

  let system_instruction = GeminiContent {
    role: "user".to_string(),
    parts: vec![GeminiPart {
      text: format!(
        "You are the /{} skill from AI Write Ink. Follow these instructions exactly:\n\n{}",
        skill_name, skill_instructions
      ),
    }],
  };

  let mut contents = Vec::new();

  // Message utilisateur principal incluant toujours le texte complet.
  // En cas de follow-up, on précise l'instruction dans le même message.
  let user_text = if let Some(fu) = &follow_up {
    format!(
      "Texte à analyser :\n\n{}\n\nInstruction spécifique :\n{}\n",
      content, fu
    )
  } else {
    format!("Texte à analyser :\n\n{}\n", content)
  };

  contents.push(GeminiContent {
    role: "user".to_string(),
    parts: vec![GeminiPart { text: user_text }],
  });

  // Optionnellement, le dernier message du skill (diagnostic précédent) pour contexte.
  if let Some(prev) = previous_message {
    contents.push(GeminiContent {
      role: "model".to_string(),
      parts: vec![GeminiPart { text: prev }],
    });
  }

  let body = GeminiRequestBody {
    contents,
    system_instruction: Some(system_instruction),
  };

  let client = Client::new();
  let resp = client
    .post(format!(
      "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
      model, api_key
    ))
    .json(&body)
    .send()
    .await
    .map_err(|e| format!("Gemini HTTP error: {}", e))?;

  if !resp.status().is_success() {
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    return Err(format!("Gemini API returned {}: {}", status, text));
  }

  let parsed: GeminiResponse = resp
    .json()
    .await
    .map_err(|e| format!("Failed to parse Gemini response: {}", e))?;

  let mut diagnostic_text = String::new();
  if let Some(first) = parsed.candidates.first() {
    for part in &first.content.parts {
      diagnostic_text.push_str(&part.text);
    }
  } else {
    diagnostic_text.push_str("Gemini returned no candidates.");
  }

  let run_id = format!("run-{}", Uuid::new_v4());
  let mode_value = mode.unwrap_or_else(|| "analysis".to_string());

  let diag = DiagnosticDto {
    id: format!("diag-{}", Uuid::new_v4()),
    axis: "qa-reader:report".to_string(),
    message: diagnostic_text,
    severity: "info".to_string(),
    start_offset: None,
    end_offset: None,
  };

  let run = SkillRunDto {
    id: run_id,
    document_id: path,
    skill_name,
    mode: mode_value,
    scope: "full".to_string(),
    created_at: Utc::now().to_rfc3339(),
    diagnostics: vec![diag],
    patches: Vec::new(),
  };

  Ok(run)
}

/// Première implémentation très simple d'un "runner" de skill côté Tauri.
/// Pour l'instant, c'est un mock : on lit le document et on fabrique un SkillRunDto
/// avec un diagnostic et un patch basique sur les premiers caractères.
#[tauri::command]
fn run_skill_mock(skill_name: String, path: String, mode: Option<String>) -> Result<SkillRunDto, String> {
  let full = resolve_path(&path);
  let content = std::fs::read_to_string(&full).map_err(|e| e.to_string())?;

  let run_id = format!("run-{}", Uuid::new_v4());
  let mode_value = mode.unwrap_or_else(|| "analysis".to_string());

  // Diagnostic très simple sur le début du texte.
  let diag = DiagnosticDto {
    id: format!("diag-{}", Uuid::new_v4()),
    axis: format!("{}:mock", skill_name),
    message: "Mock diagnostic: début du texte analysé.".to_string(),
    severity: "info".to_string(),
    start_offset: Some(0),
    end_offset: Some(content.len().min(80)),
  };

  // Patch très simple : duplique les 80 premiers caractères avec un marqueur [MOCK].
  let end = content.len().min(80);
  let before = content.chars().take(end).collect::<String>();
  let after = format!("{} [MOCK]", before);

  let patch = PatchDto {
    id: format!("patch-{}", Uuid::new_v4()),
    skill_run_id: run_id.clone(),
    start_offset: 0,
    end_offset: end,
    before_text: before,
    after_text: after,
    status: "pending".to_string(),
    axis: format!("{}:mock", skill_name),
    note: Some("Patch mock généré côté Tauri pour tester le flux.".to_string()),
  };

  let run = SkillRunDto {
    id: run_id,
    document_id: path,
    skill_name,
    mode: mode_value,
    scope: "full".to_string(),
    created_at: chrono::Utc::now().to_rfc3339(),
    diagnostics: vec![diag],
    patches: vec![patch],
  };

  Ok(run)
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
      save_document,
      open_analysis_window,
      run_skill_mock,
      run_skill
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
