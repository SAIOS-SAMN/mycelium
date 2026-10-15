// 78.146 — A TASK BEFORE THE SEAT (wire v1.2): a percept record of kind 4,
// `[Key(citation)]` — the task's 32-byte citation (`registry::citation`) —
// with no counsel, says "this task is before the seat". The mind fetches it
// by its citation on the court's carrier (`Entity::pull_before`, at the
// mind's return), and where it lands extracts and mints its
// rule as the task arm does, and calls fn 1 with the rule's tape, the test
// outputs as the solve act writes grids, and — where the rule is a pure
// motion and hue map — the steps the house's D4 kinds and hue moves name.
//
//! THE STEPS, read exactly off the minted word. A rule is pure where its form
//! states no clause and holds one word vertex whose members all carry one
//! hue map: each member's motion is the word's rotor acting through its
//! carriage (`element::conformal_of_rotor`), and that motion is the D4 kind
//! whose `Step::grid_motion` on the train pair's shapes it EQUALS as an
//! action on the grid's plane (`same_action`, exact), tried in the house's
//! order: 1 Rotate90, 2 Rotate180, 3 Rotate270, 4 FlipH, 5 FlipV, 6
//! Transpose. A motion that is no D4 kind is not pure. Each hue move `from →
//! to` is the count `0x100 + 16·from + to`.

use super::super::rule::{Step, Symmetry};
use crate::anastome::dialogue::seat::{Arg, Percept, Record};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::Vertex;

/// The record kind that puts a task before the seat.
pub const KIND_TASK: u16 = 4;
/// The function a seat solves a task by.
pub const FN_SOLVE: u16 = 1;

/// The D4 kinds in the house's order: kind `i + 1` is `D4[i]`.
pub const D4: [Symmetry; 6] =
    [Symmetry::Rotate90, Symmetry::Rotate180, Symmetry::Rotate270, Symmetry::FlipH, Symmetry::FlipV, Symmetry::Transpose];

/// THE TASKS A PERCEPT PUTS BEFORE THE SEAT: the citations its kind-4
/// records carry — each `[Key(citation)]` exactly — where it carries no
/// counsel (a counselled percept is the house's own mind speaking).
pub fn tasks_before(p: &Percept) -> Vec<[u8; 32]> {
    if p.records.iter().any(|r| matches!(r, Record::Counsel { .. })) {
        return Vec::new();
    }
    p.records
        .iter()
        .filter_map(|r| match r {
            Record::Plain { kind, args } if *kind == KIND_TASK => match args.as_slice() {
                [Arg::Key(cite)] => Some(*cite),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// WHY A RULE IS NO STEPS, by name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotSteps {
    NotASentence,
    /// It states clauses (a selection, region, creation or crop).
    Clauses,
    /// It holds other than one word vertex.
    NotOneWord,
    /// Its members carry different hue maps, or there are none.
    Gauges,
    /// A member's motion is no versor of the word.
    NoMotion,
    /// The motion is no D4 kind on these shapes.
    NotD4,
}

/// TWO MOTIONS ARE ONE ON THE GRID'S PLANE where each carries the plane's
/// points to the same points, exactly: the conformal points of `(0,0)`,
/// `(1,0)`, `(0,1)` and `(1,1)` at frame 0 — three non-collinear points fix
/// a similarity of the plane, the fourth checks it. Compared as actions, so
/// an odd versor (a mirror) and an even one that acts as the same mirror on
/// the plane are one, and a versor's scale and sign never matter.
fn same_action(a: &crate::transducer::cga::Mv, b: &crate::transducer::cga::Mv) -> bool {
    use num_bigint::BigInt;
    use num_rational::BigRational as Q;
    let q = |n: i64| Q::from_integer(BigInt::from(n));
    [(0, 0), (1, 0), (0, 1), (1, 1)].iter().all(|&(r, c)| {
        let x = crate::transducer::cga::point_at(&[q(r), q(c), q(0)]);
        a.sandwich(&x).projective_eq(&b.sandwich(&x))
    })
}

/// A hue move `from → to` as the house counts it.
pub fn hue_step(from: u8, to: u8) -> u64 {
    0x100 + 16 * u64::from(from) + u64::from(to)
}

/// THE STEPS A PURE RULE IS, on a pair of these shapes (the module's rule).
pub fn grid_steps(rule: &Element, in_shape: (usize, usize), out_shape: (usize, usize)) -> Result<Vec<u64>, NotSteps> {
    let held = element::Held::of(rule.clone());
    let form = held.form().ok_or(NotSteps::NotASentence)?;
    if !form.clauses.is_empty() {
        return Err(NotSteps::Clauses);
    }
    let words: Vec<_> = form
        .vertices
        .scoped()
        .filter_map(|(_, v)| match v {
            Vertex::Word { derivation, acts, .. } => Some((derivation, acts)),
            _ => None,
        })
        .collect();
    let [(derivation, acts)] = words.as_slice() else { return Err(NotSteps::NotOneWord) };
    let members: Vec<_> = acts.scoped().map(|(_, a)| a).collect();
    let [first, rest @ ..] = members.as_slice() else { return Err(NotSteps::Gauges) };
    if rest.iter().any(|m| m.gauge != first.gauge) {
        return Err(NotSteps::Gauges);
    }
    let rotor = derivation.rotor();
    let motions: Vec<_> =
        members.iter().map(|m| element::conformal_of_rotor(&rotor, &m.carriage)).collect::<Option<_>>().ok_or(NotSteps::NoMotion)?;
    let identity = element::identity_versor();
    let mut steps = Vec::new();
    if !motions.iter().all(|v| same_action(v, &identity)) {
        let kind = D4
            .iter()
            .position(|k| match Step::grid_motion(*k, in_shape, out_shape) {
                Some(Step::Motion(c)) => motions.iter().all(|v| same_action(v, &c)),
                _ => false,
            })
            .ok_or(NotSteps::NotD4)?;
        steps.push(u64::try_from(kind).map_err(|_| NotSteps::NotD4)? + 1);
    }
    steps.extend(first.gauge.iter().map(|(f, t)| hue_step(*f, *t)));
    Ok(steps)
}

/// THE CALL'S ARGUMENTS: the rule's tape, the test outputs as the solve act
/// writes grids, and the steps where there are any. `None` where a tape
/// passes its carrier.
pub fn solve_args(rule: &Element, outputs: &[crate::arc::Grid], steps: &[u64]) -> Option<Vec<Arg>> {
    let mut args = vec![Arg::Bytes(rule.to_bytes()?), Arg::Bytes(crate::anastome::dialogue::encode_grids(outputs)?)];
    args.extend(steps.iter().map(|s| Arg::Count(*s)));
    Some(args)
}
