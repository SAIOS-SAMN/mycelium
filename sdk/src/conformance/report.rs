// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE CONFORMANCE REPORT: one finding per act checked, each named.

use super::{Need, STATEMENT};

/// WHAT ONE ACT'S CHECK CAME TO, named by the act's statement name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Finding {
    /// The act was spoken and its reply decoded to the exact values checked.
    Passed {
        /// The act's name in the statement.
        act: &'static str,
        /// What was read back.
        read: String,
    },
    /// The act was not spoken, or its reply did not decode, or decoded to
    /// values the statement does not admit.
    Failed {
        /// The act's name in the statement.
        act: &'static str,
        /// Why the act did not pass.
        reason: String,
    },
    /// The act could not be exercised (no task to solve, no in-process
    /// check for it): never a pass, and never a failure of the client.
    Unread {
        /// The act's name in the statement.
        act: &'static str,
        /// Why the act was not exercised.
        reason: String,
    },
}

impl Finding {
    /// The act the finding names.
    pub fn act(&self) -> &'static str {
        match self {
            Finding::Passed { act, .. } | Finding::Failed { act, .. } | Finding::Unread { act, .. } => act,
        }
    }

    /// Whether the act passed.
    pub fn passed(&self) -> bool {
        matches!(self, Finding::Passed { .. })
    }
}

/// THE REPORT: every finding, in the order the acts were checked.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Every finding, one per act checked.
    pub findings: Vec<Finding>,
}

impl Report {
    /// Whether the client conforms: every MUST act of the statement passed,
    /// and no act failed.
    pub fn conforms(&self) -> bool {
        let none_failed = !self.findings.iter().any(|f| matches!(f, Finding::Failed { .. }));
        let musts_passed = STATEMENT.of_need(Need::Must).all(|a| self.findings.iter().any(|f| f.act() == a.name && f.passed()));
        none_failed && musts_passed
    }

    /// Every failure, `(act, reason)`, in order.
    pub fn failures(&self) -> impl Iterator<Item = (&'static str, &str)> {
        self.findings.iter().filter_map(|f| match f {
            Finding::Failed { act, reason } => Some((*act, reason.as_str())),
            _ => None,
        })
    }

    pub(crate) fn pass(&mut self, act: &'static str, read: impl Into<String>) {
        self.findings.push(Finding::Passed { act, read: read.into() });
    }

    pub(crate) fn fail(&mut self, act: &'static str, reason: impl Into<String>) {
        self.findings.push(Finding::Failed { act, reason: reason.into() });
    }

    pub(crate) fn unread(&mut self, act: &'static str, reason: impl Into<String>) {
        self.findings.push(Finding::Unread { act, reason: reason.into() });
    }

    /// One check's outcome: `Ok` the read, `Err` the reason.
    pub(crate) fn record(&mut self, act: &'static str, outcome: Result<String, String>) {
        match outcome {
            Ok(read) => self.pass(act, read),
            Err(reason) => self.fail(act, reason),
        }
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for finding in &self.findings {
            let need = match STATEMENT.act(finding.act()).map(|a| a.need) {
                Some(Need::Must) => "MUST",
                Some(Need::May) => "MAY ",
                None => "    ",
            };
            match finding {
                Finding::Passed { act, read } => writeln!(f, "passed  {need} {act}: {read}")?,
                Finding::Failed { act, reason } => writeln!(f, "failed  {need} {act}: {reason}")?,
                Finding::Unread { act, reason } => writeln!(f, "unread  {need} {act}: {reason}")?,
            }
        }
        let verdict = match self.conforms() {
            true => "conforms",
            false => "does not conform",
        };
        writeln!(f, "{verdict}: wire 0x{:04x}, law version {}", STATEMENT.wire_version, STATEMENT.law_version)
    }
}
