// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE CONFORMANCE REPORT: one finding per act checked, each named, read
// against the profiles the client declared.

use super::{Need, Profile, DEFERRAL, STATEMENT};

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
    /// The hypha's worker pool was full and it answered the act with the
    /// deferral (see [`crate::deferral`]): nothing was ruled, and the act is
    /// spoken again later. Never a pass, and never a failure to decode.
    Deferred {
        /// The act's name in the statement.
        act: &'static str,
        /// The door's reserve deficit on the lanes S, G, T, H.
        deficit: [u64; 4],
    },
}

impl Finding {
    /// The act the finding names.
    pub fn act(&self) -> &'static str {
        match self {
            Finding::Passed { act, .. } | Finding::Failed { act, .. } | Finding::Unread { act, .. } | Finding::Deferred { act, .. } => act,
        }
    }

    /// Whether the act passed.
    pub fn passed(&self) -> bool {
        matches!(self, Finding::Passed { .. })
    }
}

/// THE REPORT: the profiles the client declared, and every finding, in the
/// order the acts were checked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    /// The profiles the client declared: the MUST acts of all of them,
    /// together, are what it is held to.
    pub profiles: Vec<Profile>,
    /// Every finding, one per act checked.
    pub findings: Vec<Finding>,
}

impl Report {
    /// An empty report for a client declaring `profiles`.
    pub fn of(profiles: &[Profile]) -> Report {
        Report { profiles: profiles.to_vec(), findings: Vec::new() }
    }

    /// The need the declared profiles give an act, or none where none of
    /// them names it. The [`DEFERRAL`] read is MUST for every profile.
    pub fn need(&self, act: &str) -> Option<Need> {
        if act == DEFERRAL && !self.profiles.is_empty() {
            return Some(Need::Must);
        }
        STATEMENT.union(&self.profiles).into_iter().find(|a| a.name == act).map(|a| a.need)
    }

    /// Whether the client conforms: it declared a profile, every MUST act of
    /// the profiles it declared passed, and no act failed.
    pub fn conforms(&self) -> bool {
        let none_failed = !self.findings.iter().any(|f| matches!(f, Finding::Failed { .. }));
        let musts = STATEMENT.union(&self.profiles);
        let musts_passed = musts.iter().filter(|a| a.need == Need::Must).all(|a| self.findings.iter().any(|f| f.act() == a.name && f.passed()));
        !self.profiles.is_empty() && none_failed && musts_passed
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

    pub(crate) fn deferred(&mut self, act: &'static str, deficit: [u64; 4]) {
        self.findings.push(Finding::Deferred { act, deficit });
    }

    /// Every act the hypha deferred, `(act, deficit)`, in order: each is
    /// spoken again once the door has room.
    pub fn deferrals(&self) -> impl Iterator<Item = (&'static str, [u64; 4])> + '_ {
        self.findings.iter().filter_map(|f| match f {
            Finding::Deferred { act, deficit } => Some((*act, *deficit)),
            _ => None,
        })
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
            let need = match self.need(finding.act()) {
                Some(Need::Must) => "MUST",
                Some(Need::May) => "MAY ",
                None => "    ",
            };
            match finding {
                Finding::Passed { act, read } => writeln!(f, "passed  {need} {act}: {read}")?,
                Finding::Failed { act, reason } => writeln!(f, "failed  {need} {act}: {reason}")?,
                Finding::Unread { act, reason } => writeln!(f, "unread  {need} {act}: {reason}")?,
                Finding::Deferred { act, deficit } => writeln!(f, "deferred {need} {act}: the hypha's pool is full, deficit S:G:T:H {deficit:?}; speak it again")?,
            }
        }
        let verdict = match self.conforms() {
            true => "conforms",
            false => "does not conform",
        };
        let profiles: Vec<&str> = self.profiles.iter().map(|p| p.name()).collect();
        writeln!(
            f,
            "{verdict} (profile {}): wire 0x{:04x}, law version {}",
            profiles.join(" + "),
            STATEMENT.wire_version,
            STATEMENT.law_version
        )
    }
}
