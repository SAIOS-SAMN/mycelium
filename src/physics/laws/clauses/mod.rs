// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0, SIGNING BY ADJUDICATED READING — the Concordance clauses
// the hypha adjudicates (never the rule language's clause): each one's
// content (a law form and its joint-read fixtures), its writing, and a
// kernel's signature by reconstruction.

pub mod fixtures;
pub mod register;
pub mod signing;
pub mod writing;

pub use register::{reconstruct, register, ConcordanceClause, ConcordanceClauseId, Fixture, Object, RegisterRefusal, Registered, Standing, CLAUSES};
pub use signing::{admit, admit_bytes, Admitted, Assent, Choice, SignedClause, Signing, SigningRefusal};

#[cfg(test)]
mod tests;
