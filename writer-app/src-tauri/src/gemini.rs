use reqwest::Client;

#[derive(serde::Serialize)]
pub(crate) struct GeminiPart {
  pub text: String,
}

#[derive(serde::Serialize)]
pub(crate) struct GeminiContent {
  pub role: String,
  pub parts: Vec<GeminiPart>,
}

#[derive(serde::Serialize)]
pub(crate) struct GeminiRequestBody {
  pub contents: Vec<GeminiContent>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub system_instruction: Option<GeminiContent>,
}

#[derive(serde::Deserialize)]
pub(crate) struct GeminiResponse {
  #[serde(default)]
  pub candidates: Vec<GeminiCandidate>,
}

#[derive(serde::Deserialize)]
pub(crate) struct GeminiCandidate {
  pub content: GeminiContentResponse,
}

#[derive(serde::Deserialize)]
pub(crate) struct GeminiContentResponse {
  pub parts: Vec<GeminiPartResponse>,
}

#[derive(serde::Deserialize)]
pub(crate) struct GeminiPartResponse {
  #[serde(default)]
  pub text: String,
}

pub(crate) async fn generate_diagnostic_text(
  api_key: &str,
  model: &str,
  contents: Vec<GeminiContent>,
  system_instruction: Option<GeminiContent>,
) -> Result<String, String> {
  let body = GeminiRequestBody {
    contents,
    system_instruction,
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

  if let Some(first) = parsed.candidates.first() {
    let mut diagnostic_text = String::new();
    for part in &first.content.parts {
      diagnostic_text.push_str(&part.text);
    }
    Ok(diagnostic_text)
  } else {
    Ok("Gemini returned no candidates.".to_string())
  }
}

