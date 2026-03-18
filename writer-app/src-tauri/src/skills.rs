use std::path::PathBuf;

use chrono::Utc;
use dotenvy;
use uuid::Uuid;

use crate::config::app_config;
use crate::gemini::{generate_diagnostic_text, GeminiContent, GeminiPart};
use crate::models::{DiagnosticDto, PatchDto, SkillRunDto};
use crate::util::{calculate_char_offsets, find_original_in_content, normalize_newlines, resolve_path};

fn load_skill_bundle(skill_name: &str) -> Result<String, String> {
  let skill_dir = PathBuf::from(crate::WORKSPACE_ROOT)
    .join(&app_config().paths.skills_base_dir)
    .join(skill_name);

  let skill_md = skill_dir.join("SKILL.md");
  let agent_md = skill_dir.join(format!("agent-{}.md", skill_name));

  let mut bundle = String::new();

  // SKILL.md is mandatory
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
      ));
    }
  }

  // agent-<skill>.md is optional but recommended
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

pub(crate) async fn run_skill_impl(
  skill_name: String,
  path: String,
  mode: Option<String>,
  follow_up: Option<String>,
  previous_message: Option<String>,
) -> Result<SkillRunDto, String> {
  let cfg = app_config();
  if !cfg.skills.supported_for_run.iter().any(|s| s.as_str() == skill_name.as_str()) {
    return Err(format!(
      "run_skill supports: {}.",
      cfg.skills.supported_for_run.join(", ")
    ));
  }

  let env_path = PathBuf::from(crate::WORKSPACE_ROOT).join(&cfg.paths.env_file);
  let _ = dotenvy::from_path(&env_path);

  let api_key =
    std::env::var("GEMINI_API_KEY").map_err(|_| {
      format!(
        "GEMINI_API_KEY is not set (add it to {})",
        cfg.paths.env_file
      )
    })?;
  let model = std::env::var("GEMINI_MODEL")
    .unwrap_or_else(|_| cfg.gemini.default_model.clone());

  let full = resolve_path(&path);
  let raw_content = std::fs::read_to_string(&full)
    .map_err(|e| format!("Failed to read document: {}", e))?;
  let content = normalize_newlines(&raw_content);

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

  // Main user message always including full document text.
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

  // Optional previous message for context.
  if let Some(prev) = previous_message {
    contents.push(GeminiContent {
      role: "model".to_string(),
      parts: vec![GeminiPart { text: prev }],
    });
  }

  let mut diagnostic_text = generate_diagnostic_text(
    &api_key,
    &model,
    contents,
    Some(system_instruction),
  )
  .await?;

  let run_id = format!("run-{}", Uuid::new_v4());
  let mode_value = mode.unwrap_or_else(|| "analysis".to_string());

  let mut diagnostics_vec: Vec<DiagnosticDto> = Vec::new();
  let mut patches: Vec<PatchDto> = Vec::new();

  // edit-ai-fr: extract ```json ... ``` block and map suggestions -> patches.
  if skill_name == "edit-ai-fr" {
    let json_marker = "```json";
    let json_marker_alt = "```\njson"; // variant possible

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
          diagnostics_vec.extend(env.diagnostics.into_iter());

          let mut debug_lines: Vec<String> = Vec::new();
          for item in env.suggestions {
            if item.original.is_empty() {
              continue;
            }

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

          // Remove the JSON block from the displayed diagnostic output.
          let remove_end = json_end + 3; // includes trailing ```
          diagnostic_text.replace_range(start..remove_end, "");
        }
      }
    } else if !diagnostic_text.is_empty() {
      diagnostic_text.push_str(
        "\n\n---\n*Aucun bloc JSON de suggestions n'a été retourné ; les corrections inline ne sont pas disponibles. Réessayez ou reformulez pour demander des corrections concrètes avec le format attendu.*",
      );
    }
  }

  // Main report diagnostic
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

  Ok(SkillRunDto {
    id: run_id,
    document_id: path,
    skill_name,
    mode: mode_value,
    scope: "full".to_string(),
    created_at: Utc::now().to_rfc3339(),
    diagnostics: diagnostics_vec,
    patches,
  })
}

