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

/// Normalise les fins de lignes en LF (`\n`) pour rester aligné avec CodeMirror.
fn normalize_newlines(input: &str) -> String {
  input.replace("\r\n", "\n")
}

fn resolve_path(p: &str) -> PathBuf {
  let candidate = Path::new(p);
  if candidate.is_absolute() {
    return candidate.to_path_buf();
  }

  PathBuf::from(WORKSPACE_ROOT).join(p)
}

fn history_dir() -> PathBuf {
  PathBuf::from(WORKSPACE_ROOT).join("writer-app").join(".analysis-history")
}

fn history_file_for(path: &str, skill_name: &str) -> PathBuf {
  let mut slug = path.replace(['/', '\\'], "-");
  slug = slug.replace('.', "_");
  history_dir().join(format!("{}.{}.json", slug, skill_name))
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
  // Dossier du skill : deps/ai-write-ink/skills/<skill_name>/
  let skill_dir = PathBuf::from(WORKSPACE_ROOT)
    .join("deps")
    .join("ai-write-ink")
    .join("skills")
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
  let root = PathBuf::from(WORKSPACE_ROOT).join("stories");
  if !root.exists() || !root.is_dir() {
    return Vec::new();
  }
  scan_stories_dir(&root, "stories/")
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
  match skill_name.as_str() {
    "qa-reader"
    | "qa-originality"
    | "qa-prose"
    | "qa-characters"
    | "qa-consistency"
    | "write-ink"
    | "cowrite-ink"
    | "edit-ai-fr" => {}
    _ => {
      return Err(
        "run_skill supports qa-reader, qa-originality, qa-prose, qa-characters, qa-consistency, write-ink, cowrite-ink, edit-ai-fr; use run_skill_mock for others."
          .to_string(),
      )
    }
  }

  // Clé Gemini : env var ou fichier writer-app/.env (non committé)
  let env_path = PathBuf::from(WORKSPACE_ROOT).join("writer-app").join(".env");
  let _ = dotenvy::from_path(&env_path);

  let api_key =
    std::env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY is not set (add it to writer-app/.env)".to_string())?;
  // Default to a current Gemini model; can be overridden via GEMINI_MODEL env var.
  let model = std::env::var("GEMINI_MODEL")
    .unwrap_or_else(|_| "gemini-2.5-flash-lite".to_string());

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

            if let Some(pos) = content.find(&item.original) {
              let from = pos;
              let to = pos + item.original.len();
              let patch_id =
                item.id.unwrap_or_else(|| format!("edit-{}", Uuid::new_v4()));
              patches.push(PatchDto {
                id: patch_id,
                skill_run_id: run_id.clone(),
                start_offset: from,
                end_offset: to,
                before_text: item.original.clone(),
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
      load_skill_run_history,
      save_skill_run_history,
      run_skill_mock,
      run_skill
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
