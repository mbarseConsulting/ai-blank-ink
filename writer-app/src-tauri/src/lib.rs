use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;
use once_cell::sync::OnceCell;
use regex::Regex;
use reqwest::Client;
use tauri::Manager;
use uuid::Uuid;

// Dossier racine du workspace pendant le dev :
// src-tauri/Cargo.toml se trouve dans writer-app/src-tauri
// On remonte donc de deux niveaux pour arriver à ai-blank-ink/
const WORKSPACE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

// ---------- Config (writer-app/config.json) ----------

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppConfig {
  paths: PathsConfig,
  window: WindowConfig,
  gemini: GeminiConfig,
  skills: SkillsConfig,
  ui: UiConfig,
  i18n: I18nConfig,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PathsConfig {
  stories_dir: String,
  analysis_history_dir: String,
  env_file: String,
  skills_base_dir: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct WindowConfig {
  analysis_label: String,
  analysis_title: String,
  analysis_dev_url: String,
  #[serde(default = "default_analysis_inner_size")]
  analysis_inner_size: [f64; 2],
}

fn default_analysis_inner_size() -> [f64; 2] {
  [1200.0, 800.0]
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiConfig {
  default_model: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SkillsConfig {
  supported_for_run: Vec<String>,
  supported_for_analysis_desk: Vec<String>,
  default_order: Vec<String>,
  list: Vec<SkillEntry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SkillEntry {
  name: String,
  label: String,
  description: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct UiConfig {
  analysis_init_delay_ms: u64,
  skill_footer_max_height_percent: u32,
  skill_footer_min_height_px: u32,
  skill_footer_resize_max_percent_of_window: u32,
  editor_min_height_px: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct I18nConfig {
  empty_tree_message: String,
  analysis_desk_title: String,
  analysis_history_title: String,
  no_report_placeholder: String,
  no_report_hint: String,
  no_doc_subtitle: String,
}

fn load_app_config() -> Result<AppConfig, String> {
  let path = PathBuf::from(WORKSPACE_ROOT).join("writer-app").join("config.json");
  let data = std::fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;
  serde_json::from_str(&data).map_err(|e| format!("Failed to parse config: {}", e))
}

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

fn app_config() -> &'static AppConfig {
  APP_CONFIG.get_or_init(|| {
    load_app_config().unwrap_or_else(|e| {
      panic!("writer-app/config.json required: {}", e);
    })
  })
}

/// Returns app config for the frontend (single source of truth: writer-app/config.json).
#[tauri::command]
fn get_app_config() -> Result<AppConfig, String> {
  load_app_config()
}

/// Normalise les fins de lignes en LF (`\n`) pour rester aligné avec CodeMirror.
fn normalize_newlines(input: &str) -> String {
  input.replace("\r\n", "\n").replace('\r', "\n")
}

fn resolve_path(p: &str) -> PathBuf {
  let candidate = Path::new(p);
  if candidate.is_absolute() {
    return candidate.to_path_buf();
  }

  PathBuf::from(WORKSPACE_ROOT).join(p)
}

fn history_dir() -> PathBuf {
  PathBuf::from(WORKSPACE_ROOT).join(&app_config().paths.analysis_history_dir)
}

fn history_file_for(path: &str, skill_name: &str) -> PathBuf {
  let mut slug = path.replace(['/', '\\'], "-");
  slug = slug.replace('.', "_");
  history_dir().join(format!("{}.{}.json", slug, skill_name))
}

/// Convert byte offsets (UTF-8) to character offsets for CodeMirror / JS (code points).
/// French fiction (é, à, «, », —) and most BMP characters: 1 code point = 1 JS index.
fn calculate_char_offsets(content: &str, byte_start: usize, byte_end: usize) -> (usize, usize) {
  let char_start = content[..byte_start].chars().count();
  let char_count = content[byte_start..byte_end].chars().count();
  (char_start, char_start + char_count)
}

/// Cherche `original` dans `content` : d’abord correspondance exacte (après normalisation
/// des fins de ligne), puis correspondance flexible où toute séquence de blancs (espace,
/// saut de ligne) dans l’original peut matcher n’importe quelle séquence de blancs dans
/// le document (ex. Gemini renvoie "yeux. —" alors que le doc a "yeux.\n—").
/// Retourne (byte_start, byte_end, slice_matched) pour créer le patch avec le texte réel du doc.
fn find_original_in_content(content: &str, original: &str) -> Option<(usize, usize, String)> {
  let normalized = normalize_newlines(original);
  // 1) Correspondance exacte (normalisée)
  if let Some(byte_pos) = content.find(&normalized) {
    let byte_end = byte_pos + normalized.len();
    let matched = content[byte_pos..byte_end].to_string();
    return Some((byte_pos, byte_end, matched));
  }
  // 2) Correspondance flexible : blancs → \s+
  let pattern = build_flexible_whitespace_pattern(&normalized);
  if let Ok(re) = Regex::new(&pattern) {
    if let Some(m) = re.find(content) {
      let (byte_start, byte_end) = (m.start(), m.end());
      let matched = content[byte_start..byte_end].to_string();
      return Some((byte_start, byte_end, matched));
    }
  }
  None
}

/// Construit une regex où chaque séquence de caractères blancs est remplacée par \s+.
fn build_flexible_whitespace_pattern(normalized: &str) -> String {
  let mut out = String::new();
  let mut run = String::new();
  for c in normalized.chars() {
    if c.is_whitespace() {
      if !run.is_empty() {
        out.push_str(&regex::escape(&run));
        run.clear();
      }
      out.push_str("\\s+");
    } else {
      run.push(c);
    }
  }
  if !run.is_empty() {
    out.push_str(&regex::escape(&run));
  }
  out
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
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

fn load_skill_bundle(skill_name: &str) -> Result<String, String> {
  let skill_dir = PathBuf::from(WORKSPACE_ROOT)
    .join(&app_config().paths.skills_base_dir)
    .join(skill_name);

  let skill_md = skill_dir.join("SKILL.md");
  let agent_md = skill_dir.join(format!("agent-{}.md", skill_name));

  let mut bundle = String::new();

  // SKILL.md est obligatoire
  match std::fs::read_to_string(&skill_md) {
    Ok(s) => {
      bundle.push_str(&s);
      bundle.push_str("\n\n---\n\n");
    }
    Err(e) => {
      return Err(format!(
        "Failed to read SKILL.md for {} at {}: {}",
        skill_name,
        skill_md.display(),
        e
      ))
    }
  }

  // agent-<skill>.md est optionnel mais fortement recommandé
  if let Ok(agent) = std::fs::read_to_string(&agent_md) {
    bundle.push_str("# Agent instructions\n\n");
    bundle.push_str(&agent);
    bundle.push('\n');
  } else {
    bundle.push_str(&format!(
      "_Warning: agent file {} not found; using SKILL.md only._\n",
      agent_md.display()
    ));
  }

  Ok(bundle)
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
  let cfg = app_config();
  let root = PathBuf::from(WORKSPACE_ROOT).join(&cfg.paths.stories_dir);
  if !root.exists() || !root.is_dir() {
    return Vec::new();
  }
  let prefix = format!("{}/", cfg.paths.stories_dir);
  scan_stories_dir(&root, &prefix)
}

#[tauri::command]
fn open_document(path: String) -> Result<String, String> {
  let full = resolve_path(&path);
  std::fs::read_to_string(&full)
    .map(|s| normalize_newlines(&s))
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn save_document(path: String, content: String) -> Result<(), String> {
  let full = resolve_path(&path);
  std::fs::write(&full, content).map_err(|e| e.to_string())
}

/// Ouvre (ou affiche) la fenêtre d'analyse dédiée aux rapports de skills.
/// Important: command async pour éviter le deadlock Webview2 sous Windows.
#[tauri::command]
async fn open_analysis_window(window: tauri::AppHandle, _path: Option<String>) -> Result<(), String> {
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
      cfg.window.analysis_dev_url
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

  // TODO: utiliser `path` pour envoyer un événement à la fenêtre d'analyse
  // afin qu'elle sache quel document est actif.

  Ok(())
}

/// Charge l’historique des SkillRun pour un document + skill donnés depuis le disque.
#[tauri::command]
fn load_skill_run_history(path: String, skill_name: String) -> Result<Vec<SkillRunDto>, String> {
  let file = history_file_for(&path, &skill_name);
  if !file.exists() {
    return Ok(Vec::new());
  }
  let data = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;
  let runs: Vec<SkillRunDto> =
    serde_json::from_str(&data).map_err(|e| format!("Failed to parse history: {}", e))?;
  Ok(runs)
}

/// Sauvegarde l’historique des SkillRun pour un document + skill donnés sur le disque.
#[tauri::command]
fn save_skill_run_history(path: String, skill_name: String, runs: Vec<SkillRunDto>) -> Result<(), String> {
  let dir = history_dir();
  if let Err(e) = std::fs::create_dir_all(&dir) {
    return Err(e.to_string());
  }
  let file = history_file_for(&path, &skill_name);
  let json = serde_json::to_string_pretty(&runs).map_err(|e| e.to_string())?;
  std::fs::write(&file, json).map_err(|e| e.to_string())
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

#[derive(serde::Deserialize)]
struct EditAiSuggestionItem {
  #[serde(default)]
  id: Option<String>,
  #[serde(default)]
  axis: Option<String>,
  #[serde(default)]
  note: Option<String>,
  #[serde(default)]
  diagnostic: Option<String>,
  original: String,
  replacement: String,
  #[serde(default)]
  #[allow(dead_code)]
  severity: Option<String>,
}

#[derive(serde::Deserialize)]
struct EditAiSuggestionJson {
  #[serde(default)]
  diagnostics: Vec<DiagnosticDto>,
  #[serde(default)]
  suggestions: Vec<EditAiSuggestionItem>,
}

#[tauri::command]
async fn run_skill(
  skill_name: String,
  path: String,
  mode: Option<String>,
  follow_up: Option<String>,
  previous_message: Option<String>,
) -> Result<SkillRunDto, String> {
  // Pour l’instant on ne supporte que qa-reader, qa-originality, qa-prose et edit-ai-fr en vrai appel modèle.
  let cfg = app_config();
  if !cfg.skills.supported_for_run.iter().any(|s| s.as_str() == skill_name.as_str()) {
    return Err(format!(
      "run_skill supports: {}.",
      cfg.skills.supported_for_run.join(", ")
    ));
  }

  let env_path = PathBuf::from(WORKSPACE_ROOT).join(&cfg.paths.env_file);
  let _ = dotenvy::from_path(&env_path);

  let api_key =
    std::env::var("GEMINI_API_KEY").map_err(|_| format!("GEMINI_API_KEY is not set (add it to {})", cfg.paths.env_file))?;
  let model = std::env::var("GEMINI_MODEL")
    .unwrap_or_else(|_| cfg.gemini.default_model.clone());

  let full = resolve_path(&path);
  let raw_content = std::fs::read_to_string(&full)
    .map_err(|e| format!("Failed to read document: {}", e))?;
  // IMPORTANT : on normalise les fins de lignes pour rester aligné avec CodeMirror.
  let content = normalize_newlines(&raw_content);

  // Charger le bundle SKILL.md + agent-<skill>.md comme instructions système.
  let skill_instructions = load_skill_bundle(&skill_name)?;

  let extra_constraints = match skill_name.as_str() {
    "qa-reader"
    | "qa-originality"
    | "qa-prose"
    | "qa-characters"
    | "qa-consistency"
    | "cowrite-ink" => {
      "IMPORTANT CONSTRAINTS:\n\
- Never paste or quote the original story text in your answer.\n\
- Do NOT include large excerpts from the input; refer to scenes/paragraphs descriptively.\n\
- Output only your analysis and conclusions."
    }
    "write-ink" => {
      "IMPORTANT: When generating new prose (continuation, rewrite), output the generated text directly.\n\
When giving feedback or analysis, avoid pasting large excerpts from the input."
    }
    "edit-ai-fr" => "MANDATORY OUTPUT FORMAT:\n\
1. You may start with a short prose summary (optional).\n\
2. You MUST end your entire response with exactly ONE code block: ```json ... ```\n\
3. Inside that single json block, output a single object with this exact structure:\n\
   { \"suggestions\": [ { \"diagnostic\": \"brief explanation of WHY the change is needed\", \"original\": \"exact phrase from the text to replace\", \"replacement\": \"corrected phrase\" } ] }\n\
4. \"diagnostic\" must always be filled with a short explanation (reason for the change).\n\
5. \"original\" must be an exact substring copied from the input text (so the app can find and highlight it).\n\
6. \"replacement\" is the corrected French text.\n\
7. Add as many suggestion objects as needed. If no concrete edits, use \"suggestions\": [].\n\
8. Do not put any other ```json or ``` blocks elsewhere. Only one ```json block at the very end.\n",
    _ => "",
  };

  let system_instruction = GeminiContent {
    role: "user".to_string(),
    parts: vec![GeminiPart {
      text: format!(
        "You are the /{} skill from AI Write Ink.\n\
Follow these instructions exactly.\n\
{}\n\n{}",
        skill_name, extra_constraints, skill_instructions
      ),
    }],
  };

  let mut contents = Vec::new();

  // Message utilisateur principal incluant toujours le texte complet.
  // En cas de follow-up, on précise l'instruction dans le même message.
  let user_text = if skill_name == "edit-ai-fr" {
    let reminder = "Réponds puis termine OBLIGATOIREMENT par un seul bloc ```json contenant un objet {\"suggestions\": [...]} avec pour chaque entrée: \"diagnostic\", \"original\" (sous-chaîne exacte du texte), \"replacement\".";
    if let Some(fu) = &follow_up {
      format!(
        "Texte à analyser :\n\n{}\n\nInstruction : {}\n\n{}\n",
        content, fu, reminder
      )
    } else {
      format!("Texte à analyser :\n\n{}\n\n{}\n", content, reminder)
    }
  } else if let Some(fu) = &follow_up {
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

  let mut diagnostics_vec: Vec<DiagnosticDto> = Vec::new();
  let mut patches: Vec<PatchDto> = Vec::new();

  // Si edit-ai-fr, essayer d'extraire un bloc ```json ... ``` de la réponse
  if skill_name == "edit-ai-fr" {
    let json_marker = "```json";
    let json_marker_alt = "```\njson"; // variante possible
    let start = diagnostic_text
      .find(json_marker)
      .or_else(|| diagnostic_text.find(json_marker_alt));
    if let Some(start) = start {
      let skip = if diagnostic_text[start..].starts_with(json_marker) {
        json_marker.len()
      } else {
        json_marker_alt.len()
      };
      if let Some(rel_end) = diagnostic_text[start + skip..].find("```") {
        let json_start = start + skip;
        let json_end = json_start + rel_end;
        let json_str = diagnostic_text[json_start..json_end].trim();
        if let Ok(env) = serde_json::from_str::<EditAiSuggestionJson>(json_str) {
          // diagnostics éventuels renvoyés par le JSON
          diagnostics_vec.extend(env.diagnostics.into_iter());

          // suggestions -> patches inline
          let mut debug_lines: Vec<String> = Vec::new();
          for item in env.suggestions {
            if item.original.is_empty() {
              continue;
            }
            // Conserver une trace lisible dans le rapport pour aider au debug
            let explanation = item
              .diagnostic
              .clone()
              .or(item.note.clone())
              .unwrap_or_else(|| "".to_string());
            debug_lines.push(format!(
              "- ORIGINAL: \"{}\"\n  REPLACEMENT: \"{}\"\n  EXPLANATION: {}",
              item.original, item.replacement, explanation
            ));

            if let Some((byte_pos, byte_end, matched_slice)) =
              find_original_in_content(&content, &item.original)
            {
              let (char_from, char_to) =
                calculate_char_offsets(&content, byte_pos, byte_end);
              let patch_id =
                item.id.unwrap_or_else(|| format!("edit-{}", Uuid::new_v4()));
              patches.push(PatchDto {
                id: patch_id,
                skill_run_id: run_id.clone(),
                start_offset: char_from,
                end_offset: char_to,
                before_text: matched_slice,
                after_text: item.replacement,
                status: "pending".to_string(),
                axis: item.axis.unwrap_or_else(|| "edit-ai-fr".to_string()),
                note: item.note.or(item.diagnostic),
              });
            }
          }

          if !debug_lines.is_empty() {
            diagnostic_text.push_str(
              "\n\n---\nSuggestions détectées (mapping original → replacement):\n",
            );
            diagnostic_text.push_str(&debug_lines.join("\n"));
          }
        }
        // Retirer le bloc JSON du texte de diagnostic pour l'affichage
        let remove_end = json_end + 3; // inclure les ```
        diagnostic_text.replace_range(start..remove_end, "");
      }
    } else if !diagnostic_text.is_empty() {
      // Pas de bloc JSON trouvé : informer l'utilisateur
      diagnostic_text.push_str("\n\n---\n*Aucun bloc JSON de suggestions n'a été retourné ; les corrections inline ne sont pas disponibles. Réessayez ou reformulez pour demander des corrections concrètes avec le format attendu.*");
    }
  }

  // Diagnostic "rapport" principal
  let summary_diag = DiagnosticDto {
    id: format!("diag-{}", Uuid::new_v4()),
    axis: format!("{}:report", skill_name),
    message: diagnostic_text,
    severity: "info".to_string(),
    start_offset: None,
    end_offset: None,
  };

  if diagnostics_vec.is_empty() {
    diagnostics_vec.push(summary_diag);
  } else {
    diagnostics_vec.insert(0, summary_diag);
  }

  let run = SkillRunDto {
    id: run_id,
    document_id: path,
    skill_name,
    mode: mode_value,
    scope: "full".to_string(),
    created_at: Utc::now().to_rfc3339(),
    diagnostics: diagnostics_vec,
    patches,
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
      get_app_config,
      list_stories_tree,
      open_document,
      save_document,
      open_analysis_window,
      load_skill_run_history,
      save_skill_run_history,
      run_skill
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
