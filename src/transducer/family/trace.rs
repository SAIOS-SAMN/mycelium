// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! L20, D1 — THE TRACE THAT NAMES THE READER. One line per reader entered,
//! carrying the five things a timeline needs and nothing else: the
//! READER'S NAME, the PREFIX LENGTH of the key it wrote (or `demand`, or
//! `-`), the SPEND since the previous line on this token, the BYTES the
//! reader moved where it can say, and the WALL CLOCK.
//!
//! THE TRACE REACHED ONLY EXTRACTION, which is why the `1019…` stall had
//! no instrument: a family is extracted, and then it is SPOKEN, MINTED and
//! HELD, and those three readers wrote no line. Extended to them at the
//! coordinator's ruling of 2026-09-19. Each is a reader under rule 17 and
//! each is now asked the same two questions as the seven moves — what
//! coordinate did you write, and what did it cost.
//!
//! A READER THAT CANNOT SAY WHAT IT SPENT PRINTS `-`, AND THAT IS THE
//! READING. `speak`, `mint` and `hold` carry no `Budget` between them: the
//! extraction's token is spent by the time a form reaches the wire, and
//! nothing bounds what happens after. Every `spend -` on those three lines
//! is a place rule 10 is not met — a loop that neither spends a Budget nor
//! reads a boundary — and the instrument says so rather than inventing a
//! number to fill the column.
//!
//! THE `-` IN THE SECOND SLOT IS ITSELF A READING. Rule 17 (2026-09-19)
//! says a reader writes a coordinate or returns a demand; a line that
//! writes neither names a site that is not reading anything, and the
//! timeline shows those beside the moves that cost seconds. The instrument
//! is built so the rule is visible in it, not so the rule is assumed.
//!
//! THE SPEND IS TAKEN HERE, not by the engine. `Budget` carries only
//! `remaining`, so the spend between two lines is the fall between them;
//! this module holds the last `remaining` it saw and prints the drop. A
//! remaining that ROSE is a new token — a new family's budget — and prints
//! `-` rather than a negative dressed as a number (rule 11).
//!
//! THE CLOCK IS THE TRACE'S, NEVER A KERNEL'S. Law 9 and rule 5 forbid a
//! kernel reading a wall clock; this is not a kernel. No branch in the
//! engine consults a trace line, `trace` returns `()`, and the origin is a
//! monotonic `Instant` taken at the first line so every line carries
//! seconds since the trace began — comparable across lines, and never an
//! absolute time anything could key on.
//!
//! IT IS OFF UNLESS ASKED FOR. `MYC_TRACE=1` in the environment turns it
//! on, read ONCE at the first call and held — a boot-time reading like
//! `reserve_max`, not a value consulted per line. Left on, it would write
//! a line per reader per family and the corpus reading would be a reading
//! of the tracing; left compiled out, the task that does not end could not
//! be traced in the build that does not end.

// THE GATE REFUSED THIS SHAPE AND THE EXEMPTION IS SCOPED TO THIS FILE.
// `clippy.toml` disallows `std::time::Instant` with the reason "law 9: the
// clock is the receipt chain's k and the height, never wall time", and that
// ban is right for the engine. Law 9's scope, and rule 5's "looks like"
// column, is a KERNEL reading a clock; D1 (2026-09-19) requires the trace
// line to carry the wall clock and says "never by a kernel". This module is
// not a kernel and the file's header says why: no branch in the engine
// consults a trace line, `trace` returns `()`, and with `MYC_TRACE` unset
// not a single read happens. A timeline cannot be rendered from a carrier
// that has no seconds in it, and the alternative — a second clock in each
// instrument binary — cannot reach the line, which is written here.
//
// It is an inner attribute so the exemption is the FILE's and cannot leak
// into a kernel by someone moving a function next to it.
#![allow(
    clippy::disallowed_types,
    reason = "D1 2026-09-19: the trace's own monotonic origin; law 9 binds kernels and this is an instrument"
)]

use std::cell::Cell;
use std::sync::OnceLock;
use std::time::Instant;

static ON: OnceLock<bool> = OnceLock::new();
static ORIGIN: OnceLock<Instant> = OnceLock::new();

thread_local! {
    /// The `remaining` the last line on this thread carried, so the spend
    /// is the fall between two lines and no caller has to compute it.
    static LAST: Cell<Option<usize>> = const { Cell::new(None) };
}

fn on() -> bool {
    *ON.get_or_init(|| std::env::var("MYC_TRACE").is_ok_and(|v| v != "0"))
}

/// WHETHER THE TRACE IS LISTENING, so a reader whose coordinate costs
/// something to derive does not derive it into a line nobody asked for.
/// With `MYC_TRACE` unset this is one atomic read and the caller skips the
/// work entirely, which is what the module promises above.
pub fn tracing() -> bool {
    on()
}

/// WHAT A READER WROTE, which under rule 17 is the whole of what it did.
///
/// Public because a reader is traced WHERE IT RUNS, and `speak` runs in the
/// voice's own binary. An instrument that can only be called from inside
/// one module traces that module, which is how the trace came to cover the
/// seven moves and nothing downstream of them.
pub enum Wrote<'a> {
    /// The coordinate it wrote. The line carries the key's PREFIX LENGTH —
    /// the depth of the stratum the read landed on (law 2) — and not the
    /// key itself, which would put content in a log line.
    Key(&'a [u8]),
    /// IT WROTE AT AN ORDINAL, not a coordinate (rule 15). A vertex's
    /// position in its own form is an index; the line carries the index
    /// because there is no stratum depth to report for one.
    Ordinal(u16),
    /// It returned a typed demand and wrote no coordinate. That is a
    /// complete answer under rule 17 and the line says so.
    Demand,
    /// It wrote neither. Rule 17 says this is not a reader; the line is
    /// kept so the timeline can name the site.
    Nothing,
}

/// One trace line. `budget` is absent where the reader holds none — which
/// is itself the reading: a stage that cannot say what it has left is a
/// stage nothing bounds, and every such line is a place a fix has to
/// thread the caller's `Budget` through.
pub fn trace(reader: &str, wrote: Wrote<'_>, budget: Option<&crate::physics::Budget>) {
    line(reader, wrote, budget, None);
}

/// The same line from a reader that can also say HOW MANY BYTES it moved —
/// the tape a mint wrote, the form a hold took in, the description a speech
/// put on the wire. `spend` and `bytes` are two different questions and a
/// reader may answer either, both, or neither; law 5 keeps them in separate
/// columns rather than summed into one cost.
///
/// SIGNED, BECAUSE A HOLD CAN GIVE BYTES BACK. `hold_form` reads the
/// allocator before and after, and a hold that retracts more than it takes
/// in reads negative — a true reading of what the process kept, which law 7
/// says is kept and not clamped to zero.
pub fn trace_bytes(
    reader: &str,
    wrote: Wrote<'_>,
    budget: Option<&crate::physics::Budget>,
    bytes: i128,
) {
    line(reader, wrote, budget, Some(bytes));
}

fn line(
    reader: &str,
    wrote: Wrote<'_>,
    budget: Option<&crate::physics::Budget>,
    bytes: Option<i128>,
) {
    if !on() {
        return;
    }
    let t = ORIGIN.get_or_init(Instant::now).elapsed();
    let wrote = match wrote {
        Wrote::Key(k) => format!("key {}", k.len()),
        Wrote::Ordinal(i) => format!("ordinal {i}"),
        Wrote::Demand => "demand".to_string(),
        Wrote::Nothing => "-".to_string(),
    };
    let (spend, remaining) = match budget {
        None => ("-".to_string(), "-".to_string()),
        Some(b) => {
            let now = b.remaining();
            // A remaining that rose is a NEW TOKEN, not a negative spend.
            let spend = match LAST.with(|l| l.get()) {
                Some(was) if was >= now => (was - now).to_string(),
                _ => "-".to_string(),
            };
            LAST.with(|l| l.set(Some(now)));
            (spend, now.to_string())
        }
    };
    let bytes = bytes.map_or_else(|| "-".to_string(), |b| b.to_string());
    eprintln!(
        "  trace {reader} {wrote} spend {spend} bytes {bytes} budget {remaining} t {}.{:03}",
        t.as_secs(),
        t.subsec_millis()
    );
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
    // The Word vertices are one fibre of the atlas, read at their kind's tag.
    for (_, w) in form.vertices.fiber_at(&[crate::network::nerve::coord::VertexAt::WORD]).into_iter().flatten() {
        let crate::transducer::sentence::Vertex::Word { address, .. } = w else {
            continue;
        };
        let Some(acts) = w.acts() else {
            continue;
        };
        for (ck, a) in acts.scoped() {
            let mut k = address.as_bytes().to_vec();
            crate::transducer::address::put_key_bytes(&mut k, &ck);
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
