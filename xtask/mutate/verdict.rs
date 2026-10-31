use super::bench::suite_name;
use super::catalogue::Mutant;

/// What one mutant's suite said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutantVerdict {
    /// The suite failed: the law is guarded, by these tests.
    Killed(Vec<String>),
    /// The suite passed with the law broken: a guard gap.
    Survived,
    /// The `before` span was not found exactly once.
    Stale(String),
    /// The mutated tree did not compile: the guard is not proven.
    Unbuilt(String),
}

impl MutantVerdict {
    fn word(&self) -> &'static str {
        match self {
            MutantVerdict::Killed(_) => "KILLED",
            MutantVerdict::Survived => "SURVIVED",
            MutantVerdict::Stale(_) => "STALE",
            MutantVerdict::Unbuilt(_) => "UNBUILT",
        }
    }

    /// Whether this verdict leaves its law unproven.
    fn unproven(&self) -> bool {
        !matches!(self, MutantVerdict::Killed(_))
    }
}

/// One report line per mutant.
pub(super) fn line(m: &Mutant, v: &MutantVerdict) -> String {
    let detail = match v {
        MutantVerdict::Killed(tests) => format!(" — caught by: {}", tests.join(", ")),
        MutantVerdict::Survived => " — NO TEST NOTICED: the law is unguarded".to_string(),
        MutantVerdict::Stale(why) | MutantVerdict::Unbuilt(why) => format!(" — {why}"),
    };
    format!("{:<9}{} [{}] ({}){detail}", v.word(), m.law, m.file, suite_name(m))
}

/// The summary, and whether the audit passes: every mutant KILLED.
pub(super) fn summary(verdicts: &[MutantVerdict]) -> (String, bool) {
    let count = |w: &str| verdicts.iter().filter(|v| v.word() == w).count();
    let text = format!(
        "mutation audit: {} mutants — {} killed, {} survived, {} stale, {} unbuilt",
        verdicts.len(),
        count("KILLED"),
        count("SURVIVED"),
        count("STALE"),
        count("UNBUILT")
    );
    (text, !verdicts.iter().any(MutantVerdict::unproven))
}
