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

/// THE CARRIAGE A WORD VERTEX WAS MEASURED WITH, by address, so a step
/// that alters one can be named.
///
/// The court refuses a sentence whose word vertex does not carry its body
/// onto its image, and at epoch 11 that was 83% of its refusals on PAIR 0
/// — the pair the rule was read from. A vertex that satisfied pair 0 when
/// it was measured and fails it when the court replays was altered in
/// between, and the steps that alter one are the glue, the rebase and the
/// contact.
///
/// WHAT IS COMPARED IS THE COORDINATE, NOT THE KINEMATICS. Re-running
/// `vertex_carries_pair` after each step would be the court's grid replay
/// done again in the voice, per family, on the hot path (law 12, rule 10)
/// — and keyed by ordinal it would report every reindexed vertex as lost,
/// which is a fault of the probe and not of the manifold. Both carriages
/// are already held as exact versors; `same_versor` on them is the whole
/// question, O(1) over ℚ, and an address survives every reindex (law 2).
pub(crate) fn carriage_of(
    form: &crate::transducer::sentence::Form,
) -> crate::network::nerve::Keyed<crate::transducer::element::Versor> {
    let mut out = crate::network::nerve::Keyed::new();
    if !on() {
        return out;
    }
    // EVERY MEMBER'S CARRIAGE, keyed by the vertex's address and the chart it
    // acts on there: a step that alters one member's placement and leaves
    // another's is exactly what this trace exists to name, and one carriage per
    // address could not say it.
    for (_, &i) in form.by_address.scoped() {
        let Some(w @ crate::transducer::sentence::Vertex::Word { address, .. }) =
            form.vertices.get(i as usize)
        else {
            continue;
        };
        let Some(acts) = w.acts() else {
            continue;
        };
        for (ck, a) in acts.scoped() {
            let mut k = address.as_bytes().to_vec();
            crate::transducer::address::put_bytes(&mut k, &ck);
            out.insert(&k, (*a.carriage).clone());
        }
    }
    out
}

/// Name the step at which a vertex's carriage stopped being the one it was
/// measured with. Silence is the reading that nothing moved.
pub(crate) fn trace_carriage(
    step: &str,
    before: &crate::network::nerve::Keyed<crate::transducer::element::Versor>,
    form: &crate::transducer::sentence::Form,
) {
    if !on() {
        return;
    }
    let after = carriage_of(form);
    for (addr, was) in before.scoped() {
        let at: String = addr.iter().take(2).map(|b| format!("{b:02x}")).collect();
        match after.get(&addr) {
            Some(now) if crate::transducer::element::same_versor(was, now) => {}
            // A vertex the step removed is not a carriage it altered: the
            // glue retracts what it subsumes, and that is the move, not a
            // corruption of it.
            None => eprintln!("  trace {step} vertex {at}… retracted"),
            Some(_) => eprintln!("  trace {step} vertex {at}… carriage replaced"),
        }
    }
}
