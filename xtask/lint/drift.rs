// MU-ANTI-FLATTENING *Drift guards*, ROADMAP track 25 X0 — THE DRIFT
// REGISTER: each regression that recurs, the ruling behind its guard, the
// type that makes the wrong structure unwritable, and the claim that
// proves the type.
//
//! A word list cannot guard these regressions: the same word names a
//! defect at one site and a law at another. So each guard is a type, and
//! this register names it. The gate refuses an entry whose guarding type
//! or claim is missing, named empty or absent from the tree, as the
//! numbers gate refuses a register entry with no site.
//!
//! A claim is proved one of two ways:
//! - [`Proof::Test`]: a `#[test]` fn of that name under `src/`,
//!   `algebra/src/`, `sdk/src/`, `forms/src/` or `tests/`.
//! - [`Proof::CompileFail`]: a doc comment naming the claim in backticks,
//!   followed in the same comment by a fence opened ```` ```compile_fail ````.
//!
//! The report of changed document sentences that put a typed term beside an
//! admission verb is not built: reading which documents changed needs a
//! base commit the gate does not have, so it is listed open (ROADMAP track
//! 25, X0).

use std::path::{Path, PathBuf};

/// How a claim is proved.
#[derive(Debug, Clone, Copy)]
pub enum Proof {
    /// A `#[test]` fn named for the claim.
    Test,
    /// A `compile_fail` doc test named for the claim in its comment.
    CompileFail,
}

/// ONE GUARD: the regression, the ruling, the guarding type and the claim.
#[derive(Debug, Clone, Copy)]
pub struct Guard {
    pub regression: &'static str,
    pub ruling: &'static str,
    pub guard: &'static str,
    pub claim: &'static str,
    pub proof: Proof,
}

/// The roots a guarding type and a claim are read under.
const ROOTS: [&str; 5] = ["src", "algebra/src", "sdk/src", "forms/src", "tests"];

/// THE REGISTER. Only guards that are built stand here; a specified guard
/// enters with its type and its claim.
pub const REGISTER: &[Guard] = &[
    Guard {
        regression: "a digest compared to admit an object, read as an address (MU-ANTI-FLATTENING 13)",
        ruling: "operator ruling of 2026-11-02; MU-HOSTING 5a.10",
        guard: "Locator",
        claim: "address_has_no_equality",
        proof: Proof::CompileFail,
    },
    Guard {
        regression: "a digest compared to admit an object, read as a receipt chain's order (MU-ANTI-FLATTENING 13)",
        ruling: "operator ruling of 2026-11-02; MU-HOSTING 5a.10",
        guard: "Predecessor",
        claim: "predecessor_has_no_equality",
        proof: Proof::CompileFail,
    },
    Guard {
        regression: "a digest compared to admit an object, read as signed bytes (MU-ANTI-FLATTENING 13)",
        ruling: "operator ruling of 2026-11-02; MU-HOSTING 5a.10",
        guard: "Commitment",
        claim: "commitment_has_no_equality",
        proof: Proof::CompileFail,
    },
    Guard {
        regression: "a commitment compared before its object is reconstructed (MU-ANTI-FLATTENING 13)",
        ruling: "operator ruling of 2026-11-02; MU-HOSTING 5a.10",
        guard: "Commitment",
        claim: "commitment_binds_only_a_reconstructed_object",
        proof: Proof::Test,
    },
    Guard {
        regression: "admission without reconstruction (MU-ANTI-FLATTENING 13)",
        ruling: "operator ruling of 2026-11-02; MU-HOSTING 5a.10",
        guard: "Reconstructed",
        claim: "reconstructed_is_built_only_by_the_laws",
        proof: Proof::CompileFail,
    },
    Guard {
        regression: "a crossing admitted at the vertex as claimed, without reconstruction (MU-ANTI-FLATTENING 13)",
        ruling: "operator ruling of 2026-11-02; MU-HOSTING 5a.10; ROADMAP track 25 X2",
        guard: "Reconstructed",
        claim: "vertex_admits_only_a_reconstructed_fold",
        proof: Proof::CompileFail,
    },
    Guard {
        regression: "a site past the sensor read as a grid index rather than lifted into turns of its frame's periods (MU-ANTI-FLATTENING 12)",
        ruling: "operator rulings of 2026-10-31; ROADMAP T3 (#26)",
        guard: "Turn",
        claim: "turn_has_one_constructor",
        proof: Proof::CompileFail,
    },
];

/// Every `.rs` file under `dir`.
fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for e in entries {
        let p = e.map_err(|e| format!("{}: {e}", dir.display()))?.path();
        if p.is_dir() {
            walk(&p, out)?;
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
    Ok(())
}

/// The text of every source file the register reads.
fn sources(repo: &Path) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    for root in ROOTS {
        let dir = repo.join(root);
        if dir.is_dir() {
            walk(&dir, &mut files)?;
        }
    }
    files.iter().map(|f| std::fs::read_to_string(f).map_err(|e| format!("{}: {e}", f.display()))).collect()
}

/// A doc comment line's text, where the line is one.
fn doc(line: &str) -> Option<&str> {
    let t = line.trim_start();
    t.strip_prefix("///").or_else(|| t.strip_prefix("//!"))
}

/// `name` stands as a whole identifier at `at` in `line`.
fn whole(line: &str, at: usize, name: &str) -> bool {
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let before = line[..at].chars().next_back().is_none_or(|c| !ident(c));
    let after = line[at + name.len()..].chars().next().is_none_or(|c| !ident(c));
    before && after
}

/// A struct, enum or trait named `name` is declared in `text`.
fn declares(text: &str, name: &str) -> bool {
    text.lines().filter(|l| doc(l).is_none() && !l.trim_start().starts_with("//")).any(|l| {
        ["struct ", "enum ", "trait "].iter().any(|kw| {
            let item = format!("{kw}{name}");
            l.match_indices(&item).any(|(at, _)| whole(l, at + kw.len(), name))
        })
    })
}

/// A `#[test]` fn named `claim` stands in `text`: the fn line, with
/// `#[test]` among the attributes and doc lines just above it.
fn tests(text: &str, claim: &str) -> bool {
    let lines: Vec<&str> = text.lines().collect();
    let head = format!("fn {claim}(");
    lines.iter().enumerate().any(|(n, l)| {
        doc(l).is_none()
            && l.contains(&head)
            && lines[..n]
                .iter()
                .rev()
                .take_while(|a| {
                    let t = a.trim_start();
                    t.starts_with("#[") || t.starts_with("///")
                })
                .any(|a| a.trim() == "#[test]")
    })
}

/// A doc comment names `claim` in backticks and opens a `compile_fail`
/// fence after it, in the same comment.
fn compile_fails(text: &str, claim: &str) -> bool {
    let lines: Vec<&str> = text.lines().collect();
    let named = format!("`{claim}`");
    lines.iter().enumerate().any(|(n, l)| {
        doc(l).is_some_and(|d| d.contains(&named))
            && lines[n + 1..]
                .iter()
                .map_while(|a| doc(a))
                .any(|d| d.trim_start().starts_with("```compile_fail"))
    })
}

/// THE DRIFT GATE: each entry of `register` read against the tree at
/// `repo`. A hit per entry whose guarding type or claim is missing.
pub fn lint_drift(repo: &Path, register: &[Guard], hits: &mut Vec<String>) -> Result<(), String> {
    let texts = sources(repo)?;
    for g in register {
        let entry = format!("  xtask/lint/drift.rs: the guard of `{}` ({})", g.regression, g.ruling);
        match g.guard.trim() {
            "" => hits.push(format!("{entry} names no guarding type")),
            name if !texts.iter().any(|t| declares(t, name)) => {
                hits.push(format!("{entry} names the type `{name}`, which no struct, enum or trait of the tree declares"))
            }
            _ => {}
        }
        match g.claim.trim() {
            "" => hits.push(format!("{entry} names no claim")),
            claim => {
                let proved = match g.proof {
                    Proof::Test => texts.iter().any(|t| tests(t, claim)),
                    Proof::CompileFail => texts.iter().any(|t| compile_fails(t, claim)),
                };
                if !proved {
                    let how = match g.proof {
                        Proof::Test => "no #[test] fn of the tree",
                        Proof::CompileFail => "named by no compile_fail doc test of the tree",
                    };
                    hits.push(format!("{entry} names the claim `{claim}`, {how}"));
                }
            }
        }
    }
    Ok(())
}
