// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0, SIGNING BY ADJUDICATED READING — the Concordance clauses
// the hypha adjudicates (never the rule language's clause): each one's
// content (a law form and its joint-read fixtures), its writing, a
// kernel's signature by reconstruction, the alignment read from the signing
// events on the ledger, and the signings of the past carried as history.

pub mod alignment;
pub mod coupling;
pub mod fixtures;
pub mod history;
pub mod register;
pub mod signing;
pub mod writing;

pub use alignment::{read_alignment, Alignment, AlignmentRefusal, ClauseAlignment, ClauseReading, KernelAlignment, SigningEvent};
pub use coupling::{CouplingFixture, CouplingObject, Term};
pub use history::{history, HistoricalSigning, Mechanism};
pub use register::{reconstruct, register, ConcordanceClause, ConcordanceClauseId, Fixture, Object, RegisterRefusal, Registered, Standing, CLAUSES};
pub use signing::{admit, admit_bytes, judge, Admitted, Assent, Choice, SignedClause, Signing, SigningRefusal, Verdict};

#[cfg(test)]
mod tests;
