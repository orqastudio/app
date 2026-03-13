use std::path::Path;

use crate::search::embedder::{EmbedError, Embedder};

/// Pre-computed embedding of a skill's description.
struct SkillEmbedding {
    name: String,
    description: String,
    embedding: Vec<f32>,
}

/// Manages skill embeddings and prompt-based matching.
///
/// Loads all skills from `.orqa/process/skills/*/SKILL.md`, extracts their
/// `description:` frontmatter field, embeds them with the ONNX embedder,
/// and caches the results for fast cosine-similarity lookups at prompt time.
pub struct SkillInjector {
    skills: Vec<SkillEmbedding>,
}

/// Error type for skill injection operations.
#[derive(Debug, thiserror::Error)]
pub enum SkillInjectorError {
    #[error("embedding error: {0}")]
    Embed(#[from] EmbedError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl SkillInjector {
    /// Load all skills from the project's `.orqa/process/skills/` directory
    /// and pre-compute their description embeddings.
    ///
    /// Skills without a `description:` frontmatter field are silently skipped.
    pub fn new(project_dir: &Path, embedder: &mut Embedder) -> Result<Self, SkillInjectorError> {
        let skills_dir = project_dir.join(".orqa").join("team").join("skills");
        let skill_metas = discover_skill_descriptions(&skills_dir)?;

        if skill_metas.is_empty() {
            return Ok(Self { skills: Vec::new() });
        }

        let descriptions: Vec<&str> = skill_metas.iter().map(|(_, d)| d.as_str()).collect();
        let embeddings = embedder.embed(&descriptions)?;

        let skills = skill_metas
            .into_iter()
            .zip(embeddings)
            .map(|((name, description), embedding)| SkillEmbedding {
                name,
                description,
                embedding,
            })
            .collect();

        Ok(Self { skills })
    }

    /// Find the top-N skills most relevant to the given prompt embedding.
    ///
    /// Returns skill names sorted by descending similarity, filtered by the
    /// given threshold. At most `top_n` results are returned.
    pub fn match_prompt(
        &self,
        prompt_embedding: &[f32],
        top_n: usize,
        threshold: f32,
    ) -> Vec<SkillMatch> {
        let mut scored: Vec<SkillMatch> = self
            .skills
            .iter()
            .filter_map(|skill| {
                let sim = cosine_similarity(prompt_embedding, &skill.embedding);
                if sim >= threshold {
                    Some(SkillMatch {
                        name: skill.name.clone(),
                        description: skill.description.clone(),
                        score: sim,
                    })
                } else {
                    None
                }
            })
            .collect();

        scored.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored.truncate(top_n);
        scored
    }

    /// Returns the number of skills loaded.
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }
}

/// A matched skill with its similarity score.
#[derive(Debug, Clone)]
pub struct SkillMatch {
    pub name: String,
    pub description: String,
    pub score: f32,
}

/// Compute cosine similarity between two vectors.
///
/// Returns 0.0 if either vector has zero magnitude.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

/// Discover all skill directories and extract their `description:` field
/// from YAML frontmatter.
///
/// Returns a vec of `(skill_name, description)` pairs.
fn discover_skill_descriptions(skills_dir: &Path) -> Result<Vec<(String, String)>, std::io::Error> {
    let mut results = Vec::new();

    let read_dir = match std::fs::read_dir(skills_dir) {
        Ok(rd) => rd,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(results),
        Err(e) => return Err(e),
    };

    for entry in read_dir.flatten() {
        if !entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            continue;
        }

        let skill_md = entry.path().join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }

        let skill_name = entry.file_name().to_string_lossy().to_string();
        if let Some(description) = extract_description(&skill_md) {
            results.push((skill_name, description));
        }
    }

    Ok(results)
}

/// Extract the `description:` field from a SKILL.md's YAML frontmatter.
///
/// Handles both single-line (`description: "text"`) and multi-line
/// (`description: |` followed by indented lines) YAML values.
fn extract_description(path: &Path) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    let frontmatter = extract_frontmatter(&content)?;
    parse_description_field(frontmatter)
}

/// Extract the YAML frontmatter block between `---` delimiters.
fn extract_frontmatter(content: &str) -> Option<&str> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }

    let after_first = &trimmed[3..];
    let end = after_first.find("\n---")?;
    Some(after_first[..end].trim())
}

/// Parse the `description:` value from YAML frontmatter text.
///
/// Supports:
/// - `description: "quoted text"` or `description: unquoted text`
/// - `description: |` followed by indented continuation lines
fn parse_description_field(frontmatter: &str) -> Option<String> {
    let mut lines = frontmatter.lines();
    let mut description_value = None;

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if !trimmed.starts_with("description:") {
            continue;
        }

        let after_key = trimmed.strip_prefix("description:")?.trim();

        // Multi-line block scalar (e.g., `description: |`)
        if after_key == "|" || after_key == ">" {
            let mut parts = Vec::new();
            for cont_line in lines.by_ref() {
                if cont_line.starts_with(' ') || cont_line.starts_with('\t') {
                    parts.push(cont_line.trim());
                } else {
                    break;
                }
            }
            if !parts.is_empty() {
                description_value = Some(parts.join(" "));
            }
        } else {
            // Single-line value — strip surrounding quotes
            let val = after_key
                .trim_start_matches('"')
                .trim_end_matches('"')
                .trim_start_matches('\'')
                .trim_end_matches('\'')
                .trim();
            if !val.is_empty() {
                description_value = Some(val.to_string());
            }
        }
        break;
    }

    description_value
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── cosine_similarity ──

    #[test]
    fn identical_vectors_have_similarity_one() {
        let v = vec![1.0, 0.0, 0.0];
        let sim = cosine_similarity(&v, &v);
        assert!((sim - 1.0).abs() < 1e-6);
    }

    #[test]
    fn orthogonal_vectors_have_similarity_zero() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-6);
    }

    #[test]
    fn opposite_vectors_have_negative_similarity() {
        let a = vec![1.0, 0.0];
        let b = vec![-1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn zero_vector_returns_zero() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 2.0, 3.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
        assert_eq!(cosine_similarity(&b, &a), 0.0);
    }

    #[test]
    fn different_lengths_returns_zero() {
        let a = vec![1.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn empty_vectors_returns_zero() {
        assert_eq!(cosine_similarity(&[], &[]), 0.0);
    }

    #[test]
    fn known_similarity_value() {
        // cos(45°) ≈ 0.7071
        let a = vec![1.0, 0.0];
        let b = vec![1.0, 1.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-5);
    }

    // ── extract_frontmatter ──

    #[test]
    fn extracts_frontmatter_block() {
        let content = "---\ntitle: Test\ndescription: hello\n---\n# Body";
        let fm = extract_frontmatter(content);
        assert_eq!(fm, Some("title: Test\ndescription: hello"));
    }

    #[test]
    fn no_frontmatter_returns_none() {
        let content = "# No frontmatter here";
        assert!(extract_frontmatter(content).is_none());
    }

    #[test]
    fn unclosed_frontmatter_returns_none() {
        let content = "---\ntitle: Test\n";
        assert!(extract_frontmatter(content).is_none());
    }

    // ── parse_description_field ──

    #[test]
    fn parses_single_line_quoted_description() {
        let fm = "title: Test\ndescription: \"Tauri IPC patterns\"";
        assert_eq!(
            parse_description_field(fm),
            Some("Tauri IPC patterns".to_string())
        );
    }

    #[test]
    fn parses_single_line_unquoted_description() {
        let fm = "title: Test\ndescription: Tauri IPC patterns";
        assert_eq!(
            parse_description_field(fm),
            Some("Tauri IPC patterns".to_string())
        );
    }

    #[test]
    fn parses_multiline_block_description() {
        let fm = "title: Test\ndescription: |\n  Line one\n  Line two\nstatus: active";
        assert_eq!(
            parse_description_field(fm),
            Some("Line one Line two".to_string())
        );
    }

    #[test]
    fn parses_folded_block_description() {
        let fm = "title: Test\ndescription: >\n  Line one\n  Line two\nstatus: active";
        assert_eq!(
            parse_description_field(fm),
            Some("Line one Line two".to_string())
        );
    }

    #[test]
    fn no_description_returns_none() {
        let fm = "title: Test\nstatus: active";
        assert!(parse_description_field(fm).is_none());
    }

    #[test]
    fn empty_description_returns_none() {
        let fm = "title: Test\ndescription: \"\"";
        assert!(parse_description_field(fm).is_none());
    }

    // ── match_prompt ──

    #[test]
    fn match_prompt_returns_top_n_sorted_by_score() {
        let injector = SkillInjector {
            skills: vec![
                SkillEmbedding {
                    name: "skill-a".to_string(),
                    description: "A".to_string(),
                    embedding: vec![1.0, 0.0, 0.0],
                },
                SkillEmbedding {
                    name: "skill-b".to_string(),
                    description: "B".to_string(),
                    embedding: vec![0.7, 0.7, 0.0],
                },
                SkillEmbedding {
                    name: "skill-c".to_string(),
                    description: "C".to_string(),
                    embedding: vec![0.0, 0.0, 1.0],
                },
            ],
        };

        let prompt = vec![1.0, 0.0, 0.0];
        let matches = injector.match_prompt(&prompt, 2, 0.3);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].name, "skill-a");
        assert_eq!(matches[1].name, "skill-b");
        assert!(matches[0].score > matches[1].score);
    }

    #[test]
    fn match_prompt_filters_below_threshold() {
        let injector = SkillInjector {
            skills: vec![
                SkillEmbedding {
                    name: "relevant".to_string(),
                    description: "R".to_string(),
                    embedding: vec![1.0, 0.0],
                },
                SkillEmbedding {
                    name: "irrelevant".to_string(),
                    description: "I".to_string(),
                    embedding: vec![0.0, 1.0],
                },
            ],
        };

        let prompt = vec![1.0, 0.0];
        let matches = injector.match_prompt(&prompt, 10, 0.5);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].name, "relevant");
    }

    #[test]
    fn match_prompt_empty_skills_returns_empty() {
        let injector = SkillInjector { skills: Vec::new() };
        let prompt = vec![1.0, 0.0, 0.0];
        assert!(injector.match_prompt(&prompt, 3, 0.3).is_empty());
    }

    #[test]
    fn match_prompt_respects_top_n_limit() {
        let injector = SkillInjector {
            skills: vec![
                SkillEmbedding {
                    name: "a".to_string(),
                    description: "A".to_string(),
                    embedding: vec![1.0, 0.0],
                },
                SkillEmbedding {
                    name: "b".to_string(),
                    description: "B".to_string(),
                    embedding: vec![0.9, 0.1],
                },
                SkillEmbedding {
                    name: "c".to_string(),
                    description: "C".to_string(),
                    embedding: vec![0.8, 0.2],
                },
            ],
        };

        let prompt = vec![1.0, 0.0];
        let matches = injector.match_prompt(&prompt, 1, 0.0);
        assert_eq!(matches.len(), 1);
    }

    // ── skill_count ──

    #[test]
    fn skill_count_reports_loaded_skills() {
        let injector = SkillInjector {
            skills: vec![
                SkillEmbedding {
                    name: "a".to_string(),
                    description: "A".to_string(),
                    embedding: vec![1.0],
                },
                SkillEmbedding {
                    name: "b".to_string(),
                    description: "B".to_string(),
                    embedding: vec![1.0],
                },
            ],
        };
        assert_eq!(injector.skill_count(), 2);
    }
}
