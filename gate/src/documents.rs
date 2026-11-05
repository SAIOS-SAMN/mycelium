// AI-GENERATED CONTENT — see DISCLAIMER.md
//
// MU-STANDARDS 7.1 — THE DOCUMENTS NAME WHAT EXISTS: every repository path a
// document cites in backticks stands in the tree, or its line says the
// thing it names is gone.
//
//! A cited path is an inline code span (outside a fenced block) written in
//! path characters alone (`[A-Za-z0-9_./-]`, a `:<line>` or `::<item>`
//! suffix read off), holding a `/`, whose first component is a top-level
//! directory of the repository, or one that was. A span with a glob, a
//! brace, a placeholder or a space (`src/bin/{a,b}.rs`, `<module>/tests/`)
//! is a pattern, not a path, and is not read.
//!
//! A path that names something gone stands only on a line that says so: a
//! line holding the word `retired`, `removed` or `was`. Every other cited
//! path that does not exist is refused, with its document and line.

use std::path::{Path, PathBuf};

/// The top-level directories a cited path may open with: the tree's own,
/// and the retired ones a document may still name as gone.
pub const ROOTS: [&str; 14] = [
    ".github", "algebra", "changes", "docs", "examples", "forms", "gate", "ops", "scripts", "sdk", "sources", "src", "tests",
    "xtask",
];

/// The words that let a line name a path that is gone.
const GONE: [&str; 3] = ["retired", "removed", "was"];

/// The documents whose citations are read: every `.md` under `docs/`, and
/// the root documents but the CHANGELOG, which is history.
pub fn documents(repo: &Path) -> Result<Vec<PathBuf>, String> {
    let mut out: Vec<PathBuf> =
        ["README.md", "AGENTS.md", "CONTRIBUTING.md", "ROADMAP.md", "GLOSSARY.md"].iter().map(|d| repo.join(d)).collect();
    walk(&repo.join("docs"), &mut out)?;
    Ok(out)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for e in entries {
        let p = e.map_err(|e| format!("{}: {e}", dir.display()))?.path();
        if p.is_dir() {
            walk(&p, out)?;
        } else if p.extension().is_some_and(|x| x == "md") {
            out.push(p);
        }
    }
    Ok(())
}

/// One path a document cites, with its line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cited {
    /// The 1-based line of the document.
    pub line: usize,
    /// The path as cited, its suffix read off.
    pub path: String,
    /// The line says the thing cited is gone.
    pub gone: bool,
}

/// A code span read as a repository path, or `None` where it is no path.
fn path_of(span: &str) -> Option<String> {
    let span = span.trim();
    let path = match span.split_once(':') {
        Some((head, _)) => head,
        None => span,
    };
    let lawful = |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '/' | '-');
    if path.is_empty() || !path.chars().all(lawful) || !path.contains('/') || path.contains("..") {
        return None;
    }
    let first = path.split('/').next()?;
    ROOTS.contains(&first).then(|| path.to_string())
}

/// Every path `text` cites in an inline code span, outside fenced blocks.
pub fn cited(text: &str) -> Vec<Cited> {
    let mut out = Vec::new();
    let mut fenced = false;
    for (n, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let lower = line.to_ascii_lowercase();
        let gone = lower.split(|c: char| !c.is_ascii_alphabetic()).any(|w| GONE.contains(&w));
        // The odd pieces between backticks are the code spans.
        for span in line.split('`').skip(1).step_by(2) {
            if let Some(path) = path_of(span) {
                out.push(Cited { line: n + 1, path, gone });
            }
        }
    }
    out
}

/// THE CHANGELOG FRAGMENTS (AGENTS.md, *Definition of done*): every file
/// under `dir` is a markdown file (`.md`) holding one entry, opened by its
/// `### ` heading with a title. A hit per file that is not. Returns the
/// count of fragments read.
pub fn lint_fragments(dir: &Path, hits: &mut Vec<String>) -> Result<usize, String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut read = 0;
    for e in entries {
        let p = e.map_err(|e| format!("{}: {e}", dir.display()))?.path();
        read += 1;
        let name = p.display();
        if p.extension().is_none_or(|x| x != "md") || !p.is_file() {
            hits.push(format!("  {name}: a fragment is a markdown file, `<branch or topic>.md`"));
            continue;
        }
        let text = std::fs::read_to_string(&p).map_err(|e| format!("{name}: {e}"))?;
        let titled = text.strip_prefix("### ").is_some_and(|rest| rest.lines().next().is_some_and(|t| !t.trim().is_empty()));
        if !titled {
            hits.push(format!("  {name}: a fragment opens with its entry's `### ` heading and title"));
        }
    }
    Ok(read)
}

/// THE DOCUMENTS' PATHS: a hit per cited path of `files` (read relative to
/// `repo`) that does not exist and whose line does not say it is gone.
/// Returns the count of cited paths read.
pub fn lint_documents(repo: &Path, files: &[PathBuf], hits: &mut Vec<String>) -> Result<usize, String> {
    let mut read = 0;
    for f in files {
        let text = std::fs::read_to_string(f).map_err(|e| format!("{}: {e}", f.display()))?;
        let name = match f.strip_prefix(repo) {
            Ok(rel) => rel.display().to_string(),
            Err(_) => f.display().to_string(),
        };
        for c in cited(&text) {
            read += 1;
            if !c.gone && !repo.join(c.path.trim_end_matches('/')).exists() {
                hits.push(format!(
                    "  {name}:{}: `{}` names no path of the tree (MU-STANDARDS 7.1); rebind it, or say on its line that it was removed",
                    c.line, c.path
                ));
            }
        }
    }
    Ok(read)
}
