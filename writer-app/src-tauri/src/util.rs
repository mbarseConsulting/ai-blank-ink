use std::path::{Path, PathBuf};

use regex::Regex;
use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

pub(crate) fn normalize_newlines(input: &str) -> String {
  input.replace("\r\n", "\n").replace('\r', "\n")
}

pub(crate) fn resolve_path(p: &str) -> PathBuf {
  let candidate = Path::new(p);
  if candidate.is_absolute() {
    return candidate.to_path_buf();
  }

  PathBuf::from(crate::WORKSPACE_ROOT).join(p)
}

pub(crate) fn calculate_char_offsets(
  content: &str,
  byte_start: usize,
  byte_end: usize,
) -> (usize, usize) {
  let char_start = content[..byte_start].chars().count();
  let char_count = content[byte_start..byte_end].chars().count();
  (char_start, char_start + char_count)
}

pub(crate) fn find_original_in_content(
  content: &str,
  original: &str,
) -> Option<(usize, usize, String)> {
  let normalized = normalize_newlines(original);

  // 1) Exact match (normalized)
  if let Some(byte_pos) = content.find(&normalized) {
    let byte_end = byte_pos + normalized.len();
    let matched = content[byte_pos..byte_end].to_string();
    return Some((byte_pos, byte_end, matched));
  }

  // 2) Flexible whitespace match: every whitespace run in the original becomes \s+.
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

pub(crate) fn slugify_title(title: &str) -> String {
  // Strip accents by decomposing to NFD and dropping combining marks.
  let mut without_accents = String::new();
  for ch in title.nfd() {
    if is_combining_mark(ch) {
      continue;
    }
    without_accents.push(ch);
  }

  // Keep case, collapse non-alphanumerics into single dashes.
  let mut slug = String::new();
  let mut last_was_dash = false;

  for ch in without_accents.chars() {
    if ch.is_ascii_alphanumeric() {
      slug.push(ch);
      last_was_dash = false;
    } else if !last_was_dash {
      slug.push('-');
      last_was_dash = true;
    }
  }

  let slug = slug.trim_matches('-').to_string();
  if slug.is_empty() {
    "untitled-story".to_string()
  } else {
    slug
  }
}

