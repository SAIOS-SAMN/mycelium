use std::fmt;
use std::path::PathBuf;

/// Why the audit could not be taken at all.
#[derive(Debug)]
pub enum MutateRefusal {
    /// The repository root could not be read off the xtask manifest.
    NoRepo,
    /// A git step failed.
    Git { step: &'static str, detail: String },
    /// A file could not be read or written.
    Io { path: PathBuf, detail: String },
    /// cargo could not be started.
    Cargo { detail: String },
    /// A suite does not build on the unmutated tree.
    BaselineUnbuilt { suite: String, detail: String },
    /// A suite fails on the unmutated tree.
    BaselineRed { suite: String, failing: Vec<String> },
    /// A suite runs no test on the unmutated tree.
    BaselineEmpty { suite: String },
}

impl fmt::Display for MutateRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MutateRefusal::NoRepo => write!(f, "mutate refused: the xtask manifest has no parent repository"),
            MutateRefusal::Git { step, detail } => write!(f, "mutate refused: git {step} failed: {detail}"),
            MutateRefusal::Io { path, detail } => write!(f, "mutate refused: {}: {detail}", path.display()),
            MutateRefusal::Cargo { detail } => write!(f, "mutate refused: cargo could not run: {detail}"),
            MutateRefusal::BaselineUnbuilt { suite, detail } => {
                write!(f, "mutate refused: suite `{suite}` does not build at HEAD:\n{detail}")
            }
            MutateRefusal::BaselineRed { suite, failing } => {
                write!(f, "mutate refused: suite `{suite}` fails at HEAD: {}", failing.join(", "))
            }
            MutateRefusal::BaselineEmpty { suite } => {
                write!(f, "mutate refused: suite `{suite}` runs no test at HEAD")
            }
        }
    }
}
