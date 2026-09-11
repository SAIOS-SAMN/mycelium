// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! L20 — THE TRACE THAT NAMES THE LOOP. One line at the entry of each move
//! of `extract_rule_with` and of `pair_form_uncached`'s stages, carrying
//! the budget remaining at that entry. On a task that does not end, THE
//! LAST LINE WRITTEN NAMES THE LOOP; on one that does, the line where the
//! budget stops falling names where the work is not paid for.
//!
//! IT IS OFF UNLESS ASKED FOR. `MYC_TRACE=1` in the environment turns it
//! on, read ONCE at the first call and held — a boot-time reading like
//! `reserve_max`, not a value consulted per line. Left on, it would write
//! a line per move per family and the corpus reading would be a reading of
//! the tracing; left compiled out, the task that does not end could not be
//! traced in the build that does not end.
//!
//! It reads and never decides: no branch in the engine consults it, and
//! `trace` returns `()`.

use std::sync::OnceLock;

static ON: OnceLock<bool> = OnceLock::new();

fn on() -> bool {
    *ON.get_or_init(|| std::env::var("MYC_TRACE").is_ok_and(|v| v != "0"))
}

/// One trace line: the stage by name, and the budget remaining at its
/// entry. The budget is the measurement — a stage that enters twice with
/// the same remaining spent nothing between them, and a stage that is the
/// last line written is the one that did not return.
pub(crate) fn trace(stage: &str, budget: &crate::physics::Budget) {
    if !on() {
        return;
    }
    eprintln!("  trace {stage} budget {}", budget.remaining());
}

/// The same line with one count the stage measured, so a loop that is
/// inside a stage says how far it got. `n` is what the stage itself
/// counted — bodies, pairs, vertices — and it is never a decision.
pub(crate) fn trace_n(stage: &str, n: usize, budget: &crate::physics::Budget) {
    if !on() {
        return;
    }
    eprintln!("  trace {stage} n {n} budget {}", budget.remaining());
}

/// A line from INSIDE a move that holds no budget — which is itself the
/// reading: a stage that cannot say what it has left is a stage nothing
/// bounds. Every such line is a place the fix has to thread the caller's
/// `Budget` through.
pub(crate) fn trace_at(stage: &str, n: usize) {
    if !on() {
        return;
    }
    eprintln!("  trace {stage} n {n} budget -");
}
