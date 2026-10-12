//! Core LeanSpec validation rules and pure parser logic.
//!
//! Evaluates specifications against formal architectural rules (R01-R12).
//! Zero I/O is performed in this module.

use crate::document::{
    SpecDocument, SpecFrontmatter, SpecPriority, SpecSection, SpecStatus, ValidationMessage,
    ValidationReport,
};

/// Modern LPSL canonical section order.
pub const MODERN_SECTIONS: &[&str] = &["Requirements", "Design", "Tasks", "Verification", "Notes"];

/// Legacy LeanSpec section order.
pub const LEGACY_SECTIONS: &[&str] = &["Overview", "Design", "Plan", "Test", "Notes"];

/// Parse and validate the specification number and slug from a file name.
///
/// Returns `Some((number, slug))` if format matches `NNNN-kebab-slug.md`.
#[must_use]
pub fn parse_file_name(file_name: &str) -> Option<(String, String)> {
    let name = file_name.strip_suffix(".md")?;
    let (num_part, slug_part) = name.split_once('-')?;

    if num_part.len() == 4 && num_part.chars().all(|c| c.is_ascii_digit()) && !slug_part.is_empty() {
        let is_kebab = slug_part
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
        if is_kebab {
            return Some((num_part.to_owned(), slug_part.to_owned()));
        }
    }
    None
}

/// Parse raw markdown text into a `SpecDocument` without filesystem access.
///
/// # Errors
/// Emits errors in `ValidationReport` if frontmatter or structural syntax is broken.
pub fn parse_spec(file_name: &str, text: &str, report: &mut ValidationReport) -> Option<SpecDocument> {
    let (number, slug) = match parse_file_name(file_name) {
        Some(pair) => pair,
        None => {
            report.errors.push(ValidationMessage {
                file: file_name.to_owned(),
                line: Some(1),
                rule: "R01".to_owned(),
                message: "file name must match NNNN-kebab-slug.md".to_owned(),
            });
            return None;
        }
    };

    let stem = file_name.strip_suffix(".md").unwrap_or(file_name).to_owned();
    let lines: Vec<&str> = text.lines().collect();
    let line_count = lines.len();

    // Line limit checks (R10)
    if line_count > 400 {
        report.errors.push(ValidationMessage {
            file: file_name.to_owned(),
            line: Some(line_count),
            rule: "R10".to_owned(),
            message: format!("file exceeds maximum limit: {line_count} lines > 400"),
        });
    } else if line_count > 300 {
        report.warnings.push(ValidationMessage {
            file: file_name.to_owned(),
            line: Some(line_count),
            rule: "R10".to_owned(),
            message: format!("file is long: {line_count} lines > 300; consider splitting"),
        });
    }

    // Parse Frontmatter (R02)
    let frontmatter = match parse_frontmatter(file_name, &lines, report) {
        Some(fm) => fm,
        None => return None,
    };

    // Find H1 (R04)
    let mut h1 = None;
    let mut start_idx = 0;
    for (idx, &line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix("# ") {
            h1 = Some((idx + 1, heading.trim().to_owned()));
            start_idx = idx + 1;
            break;
        }
    }

    let h1_title = match h1 {
        Some((line_num, title)) => {
            if title != stem {
                report.errors.push(ValidationMessage {
                    file: file_name.to_owned(),
                    line: Some(line_num),
                    rule: "R04".to_owned(),
                    message: format!("H1 '{title}' must equal file stem '{stem}'"),
                });
            }
            title
        }
        None => {
            report.errors.push(ValidationMessage {
                file: file_name.to_owned(),
                line: None,
                rule: "R04".to_owned(),
                message: format!("missing top-level H1 heading equal to '{stem}'"),
            });
            return None;
        }
    };

    // Parse H2 Sections (R05, R06)
    let sections = parse_sections(file_name, &lines[start_idx..], start_idx + 1, report);

    Some(SpecDocument {
        stem,
        number,
        slug,
        h1: h1_title,
        frontmatter,
        sections,
        line_count,
    })
}

fn parse_frontmatter(
    file_name: &str,
    lines: &[&str],
    report: &mut ValidationReport,
) -> Option<SpecFrontmatter> {
    if lines.is_empty() || lines[0].trim() != "---" {
        report.errors.push(ValidationMessage {
            file: file_name.to_owned(),
            line: Some(1),
            rule: "R02".to_owned(),
            message: "file must begin with frontmatter fence '---'".to_owned(),
        });
        return None;
    }

    let mut end_idx = None;
    for (idx, &line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            end_idx = Some(idx);
            break;
        }
    }

    let end_idx = match end_idx {
        Some(idx) => idx,
        None => {
            report.errors.push(ValidationMessage {
                file: file_name.to_owned(),
                line: Some(1),
                rule: "R02".to_owned(),
                message: "unclosed frontmatter: missing closing '---'".to_owned(),
            });
            return None;
        }
    };

    let mut status = None;
    let mut created = None;
    let mut tags = Vec::new();
    let mut priority = None;
    let mut depends_on = Vec::new();
    let mut scope = None;

    for &line in &lines[1..end_idx] {
        let trimmed = line.trim();
        if let Some((key, val)) = trimmed.split_once(':') {
            let key = key.trim();
            let val = val.trim();
            match key {
                "status" => status = SpecStatus::from_str_lossy(val),
                "created" => created = Some(val.trim_matches('"').to_owned()),
                "priority" => priority = SpecPriority::from_str_lossy(val),
                "scope" => scope = Some(val.trim_matches('"').to_owned()),
                "tags" => {
                    let cleaned = val.trim_matches(|c| c == '[' || c == ']' || c == '"');
                    tags = cleaned
                        .split(',')
                        .map(|s| s.trim().to_owned())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                "depends_on" => {
                    let cleaned = val.trim_matches(|c| c == '[' || c == ']' || c == '"');
                    depends_on = cleaned
                        .split(',')
                        .map(|s| s.trim().to_owned())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                _ => {}
            }
        }
    }

    let status = match status {
        Some(s) => s,
        None => {
            report.errors.push(ValidationMessage {
                file: file_name.to_owned(),
                line: Some(1),
                rule: "R02".to_owned(),
                message: "frontmatter must include valid 'status'".to_owned(),
            });
            return None;
        }
    };

    let created = created.unwrap_or_else(|| "1970-01-01".to_owned());

    Some(SpecFrontmatter {
        status,
        created,
        tags,
        priority,
        depends_on,
        scope,
    })
}

fn parse_sections(
    file_name: &str,
    lines: &[&str],
    line_offset: usize,
    report: &mut ValidationReport,
) -> Vec<SpecSection> {
    let mut sections = Vec::new();
    let mut current_title: Option<String> = None;
    let mut current_start = line_offset;
    let mut current_body = Vec::new();

    for (idx, &line) in lines.iter().enumerate() {
        let abs_line = line_offset + idx;
        let trimmed = line.trim();

        if let Some(title) = trimmed.strip_prefix("## ") {
            if let Some(prev_title) = current_title.take() {
                sections.push(SpecSection {
                    title: prev_title,
                    start_line: current_start,
                    body: current_body.join("\n"),
                });
                current_body.clear();
            }
            current_title = Some(title.trim().to_owned());
            current_start = abs_line;
        } else if current_title.is_some() {
            current_body.push(line);
        }
    }

    if let Some(last_title) = current_title {
        sections.push(SpecSection {
            title: last_title,
            start_line: current_start,
            body: current_body.join("\n"),
        });
    }

    // Validate section order (R05)
    let titles: Vec<&str> = sections.iter().map(|s| s.title.as_str()).collect();
    let is_modern = titles.contains(&"Requirements") || titles.contains(&"Tasks");
    let expected = if is_modern {
        MODERN_SECTIONS
    } else {
        LEGACY_SECTIONS
    };

    for title in &titles {
        if !expected.contains(title) {
            report.warnings.push(ValidationMessage {
                file: file_name.to_owned(),
                line: None,
                rule: "R05".to_owned(),
                message: format!("custom or unexpected H2 section: '{title}'"),
            });
        }
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_spec_parse() {
        let raw = r#"---
status: complete
created: 2026-10-11
tags: [test, spec]
priority: high
depends_on: [0001-template-architecture]
---

# 0002-test-spec

## Overview

Overview body.

## Design

Design body.

## Plan

Plan body.

## Test

```bash
cargo test
```

## Notes

Notes body.
"#;
        let mut report = ValidationReport::default();
        let doc = parse_spec("0002-test-spec.md", raw, &mut report);
        assert!(doc.is_some());
        assert!(report.is_clean());
        let doc = doc.unwrap();
        assert_eq!(doc.number, "0002");
        assert_eq!(doc.slug, "test-spec");
        assert_eq!(doc.frontmatter.status, SpecStatus::Complete);
    }
}
