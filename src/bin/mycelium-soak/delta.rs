// Y1, 45.9 as amended 2026-09-12 — THE WARM–COLD DELTA: one slice walked
// twice, and the DIFFERENCE is the number.
//
//! A level says how good perception is on a corpus; a difference says
//! whether the engine learned anything from it, and only the second bears
//! on the conjecture. So this walks one fixed slice twice — once by a mind
//! holding nothing, once by a mind that has already stood on every OTHER
//! task of the slice — and reports both walks side by side per task.
//!
//! WHY A DIFFERENCE CANNOT BE GAMED. A level rises when a measurement is
//! loosened, which is the cipher's fault at the instrument. A looser
//! measurement moves BOTH walks alike and the difference is unchanged.
//! What the difference measures is 50.2 step 2 directly: F factors through
//! the generators the mind holds, so a mind that holds more must describe
//! more, and where the two walks agree the factorisation is not reaching
//! the presentation.
//!
//! THIS IS AN INSTRUMENT AND CHANGES NOTHING. It stands nothing into a
//! kernel, opens no wire, reads no ledger and needs no fleet.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::element::{Element, Held};
use mycelium::transducer::family::{extract_rule_with, mint_sentence};
use mycelium::transducer::sentence::Vertex;

/// THE STROKE THIS INSTRUMENT SETS FOR ITSELF, and the reason it is a
/// constant rather than the machine's: a delta read under
/// `Physics::stroke` is a delta between two host calibrations as much as
/// between two minds, and the same slice would read differently on two
/// boxes. L27 is untouched — that row is about what a LIVE organ gets per
/// stroke, and this is not an organ.
///
/// The value is the one measured to exhaust nothing on the reference
/// slice (`unmeasured 0` at 1<<20, against `unmeasured 4` under the
/// machine stroke on the same twelve). `exhausted` is reported per walk
/// anyway, so a slice that outgrows it says so rather than reading as a
/// smaller delta.
const DELTA_STROKE: usize = 1 << 20;

/// What ONE walk of ONE task read, in the engine's own coordinates.
/// Every field is read off the form that was extracted; nothing is
/// counted that was not derived.
#[derive(Default, Clone, PartialEq, Eq)]
pub(crate) struct TaskWalk {
    /// A rule extracted at all. False is a reading — the family did not
    /// close on a rule — and not a zero standing for one (rule 11).
    extracted: bool,
    /// The vertices, by what they are: a word the mind can factor
    /// through, a glued citation of a form it holds, a demand it cannot
    /// spell.
    words: u64,
    glued: u64,
    demands: u64,
    /// 50.5's own order — RotorPrime, Standing, Source, Creation,
    /// TopologicalDeformation — which names what the alphabet could not
    /// spell and the eyes could not pair.
    kinds: [u64; 5],
    /// THE ORBITS BY READING, in the words the court reads them with
    /// (`orbit_reading_replays`): an identity holonomy is FLAT, a finite
    /// order is TORSION the cover can file, an infinite order is OPEN and
    /// files nothing, and an orbit the form opened an axis over is I6's AXIS.
    flat: u64,
    torsion: u64,
    open: u64,
    axis: u64,
    /// Whether the form closed — the one thing that decides whether this
    /// task's rule could be stood at all (13.6, `Form::glue`).
    closed: bool,
    /// The walk ended on its budget rather than on the bundle (L8w), so a
    /// starvation cannot be read as a smaller reading.
    exhausted: bool,
}

impl TaskWalk {
    /// Read one task with the mind it is given, and stand nothing.
    fn of(train: &mycelium::transducer::family::Family, epi: &Epigenome) -> TaskWalk {
        let mut w = TaskWalk::default();
        let mut physics = mycelium::physics::Physics::default();
        let mut budget = physics.stroke_n(DELTA_STROKE);
        let Some(rule) = extract_rule_with(train, epi, &mut budget) else {
            w.exhausted = budget.remaining() == 0;
            return w;
        };
        w.extracted = true;
        w.exhausted = budget.remaining() == 0;
        for (_, v) in rule.form.vertices.scoped() {
            match v {
                Vertex::Word { .. } => w.words += 1,
                Vertex::Glued { .. } => w.glued += 1,
                Vertex::Demand { kind, .. } => {
                    w.demands += 1;
                    if let Some(slot) = w.kinds.get_mut(usize::from(kind.kind_byte())) {
                        *slot += 1;
                    }
                }
                _ => {}
            }
        }
        for lp in &rule.form.orbits {
            if rule.form.opened_axis_of(lp).is_some() {
                w.axis += 1;
                continue;
            }
            // The holonomy is a read along the orbit's overlaps (AT1); an
            // orbit naming an overlap the form does not hold reads none.
            let Some(holonomy) = mycelium::transducer::sentence::orbit_word_on(&rule.form.faces, &rule.form.vertices, lp)
            else {
                continue;
            };
            if holonomy.is_identity() {
                w.flat += 1;
                continue;
            }
            match holonomy.order() {
                mycelium::transducer::alphabet::Order::Finite(_) => w.torsion += 1,
                mycelium::transducer::alphabet::Order::Infinite => w.open += 1,
            }
        }
        w.closed = rule.form.is_closed();
        w
    }
}

/// WHERE THE STAND CHAIN STOPS. A warm mind is built out of what the
/// other tasks yielded, so "nothing was standable" is only half a
/// reading: the slice of sixty gave `closed 5` on the RULE forms and
/// `standable 0`, which says the break is not at extraction and not at
/// the rule's closure but somewhere after them, and an instrument that
/// cannot say where is an instrument that has to be re-run by hand.
///
/// In the chain's own order, each variant being the step that refused.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stand {
    #[default]
    NoRule,
    /// The rule extracted and its form is not closed, so it is not
    /// citable (13.6, `Form::glue`) and there is nothing to mint.
    RuleOpen,
    /// The rule's form closed and `mint_sentence` returned nothing.
    NoSentence,
    /// A sentence was minted and its tape carries no form.
    NoForm,
    /// A sentence was minted, its form read back, and THAT form is not
    /// closed — the rule closed and the sentence did not.
    SentenceOpen,
    /// Closed, citable, and stood.
    Stood,
}

impl Stand {
    fn label(self) -> &'static str {
        match self {
            Stand::NoRule => "no_rule",
            Stand::RuleOpen => "rule_open",
            Stand::NoSentence => "no_sentence",
            Stand::NoForm => "no_form",
            Stand::SentenceOpen => "sentence_open",
            Stand::Stood => "stood",
        }
    }
}

/// The closed sentence a task yields when read COLD, which is what a warm
/// mind stands for the other tasks of the slice, and WHERE it stopped if
/// it yields none. A form that is not closed is not citable (13.6) and
/// standing it would be standing something no mind could factor through.
fn standable(train: &mycelium::transducer::family::Family, epi: &Epigenome) -> (Option<Element>, Stand) {
    let mut physics = mycelium::physics::Physics::default();
    let mut budget = physics.stroke_n(DELTA_STROKE);
    let Some(rule) = extract_rule_with(train, epi, &mut budget) else {
        return (None, Stand::NoRule);
    };
    if !rule.form.is_closed() {
        return (None, Stand::RuleOpen);
    }
    // The sweep mints in the stroke it extracts, so the bundle stood over no
    // receipts: 𝒯 is zero here as a reading, not as an omission.
    let Some(spoken) = mint_sentence(&rule, train, epi, 0) else {
        return (None, Stand::NoSentence);
    };
    let held = Held::of(spoken);
    let Some(f) = held.form() else {
        return (None, Stand::NoForm);
    };
    match f.is_closed() {
        true => (Some(held.element().clone()), Stand::Stood),
        false => (None, Stand::SentenceOpen),
    }
}

/// One task of the slice, read twice.
pub(crate) struct Pair {
    name: String,
    cold: TaskWalk,
    warm: TaskWalk,
}

/// The slice read twice, AND how much the warm mind could be given.
///
/// `standable` is the count of tasks that yielded a closed, citable form
/// — what a warm mind is built out of. It is carried beside the pairs
/// because a delta of zero means two different things and the report must
/// not say them in one line: a warm mind that held eleven and read the
/// slice exactly as an empty one is the row's finding, and a warm mind
/// that could not be built at all is the instrument saying the comparison
/// was never run. On the twelve of 2026-09-12 it is the second.
pub(crate) struct Slice {
    pairs: Vec<Pair>,
    standable: usize,
    /// Where the stand chain stopped, per task of the slice, so a
    /// `standable 0` names the step that refused rather than sending the
    /// reader back to run it by hand.
    stands: Vec<Stand>,
}

/// WALK THE SLICE TWICE. `names` is the slice, in name order, so two runs
/// read the same slice in the same order.
///
/// THE WARM MIND IS LEAVE-ONE-OUT, and holds what the OTHER tasks yielded
/// when they were read COLD. Two choices are being made and both are the
/// difference's, not convenience:
///
///   - Leave-one-out rather than prefix, because "a mind that has already
///     stood on every other task" is the row's own phrase and because a
///     prefix makes the first task's warm mind empty and the last task's
///     full, so the delta would be a property of the slice's ORDER.
///   - The stood forms are the COLD extractions, so what the warm mind
///     holds does not depend on the order the slice was walked in either.
///     A sequential mind holds more (each task extracted with the benefit
///     of the ones before it) and would read a larger delta; it would also
///     read a different one for every permutation, which is not a number.
pub(crate) fn walk_twice(dir: &str, names: &[String]) -> Slice {
    let empty = Epigenome::new();
    // Pass one: every task read cold, and the closed form it yields.
    let mut tasks: Vec<(
        String,
        Vec<mycelium::transducer::family::TrainPair>,
        TaskWalk,
        Option<Element>,
    )> = Vec::new();
    let mut stands: Vec<Stand> = Vec::new();
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else {
            continue;
        };
        let Some(task) = mycelium::arc::parse_task(&json) else {
            continue;
        };
        eprintln!("  delta cold {name} pairs {}", task.train.len());
        let cold = TaskWalk::of(&task.train, &empty);
        let (stand, why) = standable(&task.train, &empty);
        eprintln!("  delta cold {name} stand {}", why.label());
        stands.push(why);
        tasks.push((name.clone(), task.train, cold, stand));
    }
    // Pass two: each task read by a mind holding every OTHER task's form.
    let mut out = Vec::with_capacity(tasks.len());
    for i in 0..tasks.len() {
        let mut epi = Epigenome::new();
        let mut k = 0u64;
        for (j, (_, _, _, stand)) in tasks.iter().enumerate() {
            if j == i {
                continue;
            }
            if let Some(e) = stand {
                k += 1;
                let _ = epi.stand_dialect_at(e.clone(), k, k);
            }
        }
        let Some((name, train, cold, _)) = tasks.get(i) else {
            continue;
        };
        // The other tasks of the slice: this one left out. A slice with no
        // tasks never reaches this loop, so the count is exact and needs
        // no floor (rule 11).
        let others = match tasks.len().checked_sub(1) {
            Some(n) => n,
            None => continue,
        };
        eprintln!("  delta warm {name} holding {k} of {others}");
        let warm = TaskWalk::of(train, &epi);
        out.push(Pair { name: name.clone(), cold: cold.clone(), warm });
    }
    let standable = tasks.iter().filter(|(_, _, _, s)| s.is_some()).count();
    Slice { pairs: out, standable, stands }
}

/// A cold and warm reading side by side. `→` where they differ and `=`
/// where they do not, so a line that moved is visible without arithmetic.
fn side(label: &str, cold: u64, warm: u64) -> String {
    match cold == warm {
        true => format!("{label} {cold}="),
        false => format!("{label} {cold}→{warm}"),
    }
}

/// THE REPORT. Per task both walks, then the totals and the delta, which
/// is the stage's number.
pub(crate) fn report(slice: &Slice) {
    let pairs = &slice.pairs;
    let mut moved = 0u64;
    for p in pairs {
        println!(
            "  delta {} {} {} {} {} | orbits {} {} {} {} | closed {}{} | extracted {}{}",
            p.name,
            side("words", p.cold.words, p.warm.words),
            side("glued", p.cold.glued, p.warm.glued),
            side("demands", p.cold.demands, p.warm.demands),
            side("rotor_prime", p.cold.kinds[0], p.warm.kinds[0]),
            side("flat", p.cold.flat, p.warm.flat),
            side("torsion", p.cold.torsion, p.warm.torsion),
            side("open", p.cold.open, p.warm.open),
            side("axis", p.cold.axis, p.warm.axis),
            u8::from(p.cold.closed),
            match p.cold.closed == p.warm.closed {
                true => "=".to_string(),
                false => format!("→{}", u8::from(p.warm.closed)),
            },
            u8::from(p.cold.extracted),
            match p.cold.extracted == p.warm.extracted {
                true => "=".to_string(),
                false => format!("→{}", u8::from(p.warm.extracted)),
            },
        );
        if p.cold != p.warm {
            moved += 1;
        }
    }
    let sum = |f: &dyn Fn(&TaskWalk) -> u64| -> (u64, u64) {
        pairs.iter().fold((0, 0), |(c, w), p| (c + f(&p.cold), w + f(&p.warm)))
    };
    let (cw, ww) = sum(&|t| t.words);
    let (cg, wg) = sum(&|t| t.glued);
    let (cd, wd) = sum(&|t| t.demands);
    let (cc, wc) = sum(&|t| u64::from(t.closed));
    let (cext, wext) = sum(&|t| u64::from(t.extracted));
    let (cx, wx) = sum(&|t| u64::from(t.exhausted));
    println!(
        "  delta totals words {cw}→{ww} glued {cg}→{wg} demands {cd}→{wd} \
         closed {cc}→{wc} extracted {cext}→{wext}"
    );
    println!(
        "  delta tasks {} moved {moved} standable {} exhausted {cx}→{wx} stroke {DELTA_STROKE}",
        pairs.len(),
        slice.standable
    );
    // WHERE THE STAND CHAIN STOPPED, in the chain's own order. `standable
    // 0` beside `closed 5` says the break is after the rule's closure and
    // this line says which step refused.
    let tally = |s: Stand| slice.stands.iter().filter(|x| **x == s).count();
    println!(
        "  delta stand no_rule {} rule_open {} no_sentence {} no_form {} sentence_open {} stood {}",
        tally(Stand::NoRule),
        tally(Stand::RuleOpen),
        tally(Stand::NoSentence),
        tally(Stand::NoForm),
        tally(Stand::SentenceOpen),
        tally(Stand::Stood),
    );
    // THE FINDING, and the instrument must not claim a comparison it did
    // not make. A warm mind is built out of the OTHER tasks' closed forms;
    // where no task of the slice closes, there is no warm mind, the two
    // walks are the same walk, and a zero here measures nothing about
    // comprehension. That is a reading about the slice and the engine, not
    // a result — and it is the one the twelve gave on 2026-09-12.
    match (slice.standable, moved) {
        (0, _) => println!(
            "  delta UNMEASURED: no task of the slice yields a closed form, so the warm mind \
             holds nothing and both walks are the same walk. The comparison has not been made. \
             The number that must move first is `closed`, which is {cc} of {}",
            pairs.len()
        ),
        (held, 0) => println!(
            "  delta ZERO: a mind holding {held} of {} reads the slice exactly as a mind \
             holding nothing",
            pairs.len()
        ),
        (held, n) => println!(
            "  delta MOVED on {n} of {} tasks, against a mind holding up to {held}",
            pairs.len()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Y1 — THE INSTRUMENT MUST BE ABLE TO SEE A DIFFERENCE, or its zero
    /// on the corpus says nothing about the engine and everything about
    /// itself. The corpus reads `standable 0`, so the twelve exercise only
    /// the arm where no warm mind can be built; this builds the other arm
    /// by hand.
    ///
    /// A mind is given one closed form and asked to read a family. The
    /// same family is read by a mind holding nothing. `TaskWalk::of` is
    /// the whole of what the delta compares, so proving it answers
    /// differently for two different minds on one family is proving the
    /// comparison is live — and proving it answers IDENTICALLY for two
    /// identical minds is proving the reading is a function of the mind
    /// and not of anything else in the walk.
    #[test]
    fn the_reading_is_a_function_of_the_mind_it_is_given() {
        // A recolour: one body, one hue changed. The simplest family there
        // is, built through the instrument's OWN public path so the test
        // exercises what the walk exercises and nothing else.
        let a = vec![vec![1u8, 1], vec![1, 1]];
        let b = vec![vec![2u8, 2], vec![2, 2]];
        let fam = vec![(a, b)];

        let empty = Epigenome::new();
        let cold = TaskWalk::of(&fam, &empty);
        // TWO EMPTY MINDS READ ALIKE. Without this, a difference below
        // could be noise in the walk rather than the mind.
        let cold_again = TaskWalk::of(&fam, &Epigenome::new());
        assert!(cold == cold_again, "one family, two identical minds, one reading");

        // A mind holding this family's own closed form — the same form a
        // warm mind is built out of on a real slice.
        let (stand, why) = standable(&fam, &empty);
        let Some(e) = stand else {
            panic!(
                "the simplest recolour yields a closed form, or nothing can ever be stood; \
                 the chain stopped at {}",
                why.label()
            )
        };
        let mut warm = Epigenome::new();
        assert!(
            warm.stand_dialect_at(e, 1, 1).is_some(),
            "the mind stands the form it is given, or this test proves nothing"
        );
        let hot = TaskWalk::of(&fam, &warm);

        // THE INSTRUMENT SEES IT. What moves is the engine's business —
        // a word vertex where there was a demand, a glued citation, a
        // closure — and the assertion is that SOMETHING does, because a
        // reading that cannot move is not a measurement.
        assert!(
            hot != cold,
            "a mind holding the form reads the family differently from a mind holding nothing; \
             cold words {} glued {} demands {} closed {}, warm words {} glued {} demands {} closed {}",
            cold.words, cold.glued, cold.demands, cold.closed,
            hot.words, hot.glued, hot.demands, hot.closed
        );
    }
}
