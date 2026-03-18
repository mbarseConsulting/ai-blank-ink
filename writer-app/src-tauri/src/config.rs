use std::path::PathBuf;

use once_cell::sync::OnceCell;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AppConfig {
  pub paths: PathsConfig,
  pub window: WindowConfig,
  pub gemini: GeminiConfig,
  pub skills: SkillsConfig,
  pub ui: UiConfig,
  pub i18n: I18nConfig,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PathsConfig {
  pub stories_dir: String,
  pub analysis_history_dir: String,
  pub env_file: String,
  pub skills_base_dir: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WindowConfig {
  pub analysis_label: String,
  pub analysis_title: String,
  pub analysis_dev_url: String,
  #[serde(default = "default_analysis_inner_size")]
  pub analysis_inner_size: [f64; 2],
}

fn default_analysis_inner_size() -> [f64; 2] {
  [1200.0, 800.0]
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeminiConfig {
  pub default_model: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SkillsConfig {
  pub supported_for_run: Vec<String>,
  pub supported_for_analysis_desk: Vec<String>,
  pub default_order: Vec<String>,
  pub list: Vec<SkillEntry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct SkillEntry {
  pub name: String,
  pub label: String,
  pub description: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UiConfig {
  pub analysis_init_delay_ms: u64,
  pub skill_footer_max_height_percent: u32,
  pub skill_footer_min_height_px: u32,
  pub skill_footer_resize_max_percent_of_window: u32,
  pub editor_min_height_px: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct I18nConfig {
  pub empty_tree_message: String,
  pub analysis_desk_title: String,
  pub analysis_history_title: String,
  pub no_report_placeholder: String,
  pub no_report_hint: String,
  pub no_doc_subtitle: String,
}

pub(crate) fn load_app_config() -> Result<AppConfig, String> {
  let path = PathBuf::from(crate::WORKSPACE_ROOT)
    .join("writer-app")
    .join("config.json");
  let data = std::fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;
  serde_json::from_str(&data).map_err(|e| format!("Failed to parse config: {}", e))
}

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

pub(crate) fn app_config() -> &'static AppConfig {
  APP_CONFIG.get_or_init(|| {
    load_app_config().unwrap_or_else(|e| {
      panic!("writer-app/config.json required: {}", e);
    })
  })
}

