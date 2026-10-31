// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE MUTATION AUDIT: a law whose breakage no test notices is unguarded.
//!
//! Each mutant of [`catalogue::CATALOGUE`] is applied, one at a time, by
//! exact string replacement inside a scratch git worktree at HEAD — never
//! the working tree — and its narrowest suite is run there. The suite must
//! FAIL. The file is restored before the next mutant, and the worktree is
//! removed at the end, on every path out.
//!
//! Before any mutant, every distinct suite is run on the unmutated tree: a
//! suite red or empty at HEAD proves nothing by failing, and the audit is
//! refused by name rather than read.

mod audit;
mod bench;
pub mod catalogue;
mod refusal;
mod verdict;
mod worktree;

#[cfg(test)]
mod tests;

pub use audit::run;
