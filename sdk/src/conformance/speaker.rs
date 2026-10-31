// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE SUITE'S SESSION: every act the conformance read speaks on a network
// goes through one session, which tells the hypha's deferral apart from a
// reply that did not decode and reports it as deferred.

use super::Report;
use crate::deferral::{deferral_of, Deferral};
use crate::session::Session;
use crate::voice::{SigningKey, Voice};
use std::cell::Cell;

/// A session that holds the deferral its last act was answered with, until
/// the act's finding is recorded.
pub(super) struct Speaker {
    session: Session,
    deferral: Cell<Option<Deferral>>,
}

impl Speaker {
    pub(super) fn open(endpoint: &str, key: &SigningKey) -> Speaker {
        Speaker { session: Session::open(endpoint, Voice::from_key(key.clone())), deferral: Cell::new(None) }
    }

    /// One act spoken and its reply read. A deferral is held for the
    /// finding (the act is reported deferred, with the door's deficit) and
    /// answered here as the reason the check stops; any other refusal is
    /// "no answer".
    pub(super) fn speak(&self, act: Vec<u8>) -> Result<Vec<u8>, String> {
        self.session.speak(act).map_err(|e| match deferral_of(&e) {
            Some(d) => {
                self.deferral.set(Some(d));
                d.to_string()
            }
            None => format!("no answer: {e}"),
        })
    }

    /// One check's outcome, or the deferral its act was answered with.
    pub(super) fn record(&self, report: &mut Report, act: &'static str, outcome: Result<String, String>) {
        match self.deferral.take() {
            Some(d) => report.deferred(act, d.deficit),
            None => report.record(act, outcome),
        }
    }

    /// A failure, or the deferral the act was answered with.
    pub(super) fn fail(&self, report: &mut Report, act: &'static str, reason: impl Into<String>) {
        self.record(report, act, Err(reason.into()));
    }

    /// An act not exercised, or the deferral it was answered with.
    pub(super) fn unread(&self, report: &mut Report, act: &'static str, reason: impl Into<String>) {
        match self.deferral.take() {
            Some(d) => report.deferred(act, d.deficit),
            None => report.unread(act, reason),
        }
    }
}
