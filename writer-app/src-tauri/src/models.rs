#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagnosticDto {
  pub id: String,
  pub axis: String,
  pub message: String,
  pub severity: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub start_offset: Option<usize>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub end_offset: Option<usize>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PatchDto {
  pub id: String,
  pub skill_run_id: String,
  pub start_offset: usize,
  pub end_offset: usize,
  pub before_text: String,
  pub after_text: String,
  pub status: String,
  pub axis: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub note: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SkillRunDto {
  pub id: String,
  pub document_id: String,
  pub skill_name: String,
  pub mode: String,
  pub scope: String,
  pub created_at: String,
  pub diagnostics: Vec<DiagnosticDto>,
  pub patches: Vec<PatchDto>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DocumentNodeDto {
  pub id: String,
  pub label: String,
  pub kind: String,
  pub path: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub children: Option<Vec<DocumentNodeDto>>,
}

