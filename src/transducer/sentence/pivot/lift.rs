// 78.61 C1 (redefined 2026-10-01, the operator's word: "if the question comes
// in two dimensions, we don't break our higher-dimensional perception to
// rectify our minds in two dimensions") — A WORD LIFTED INTO PERCEPTION, and
// two words compared there, never as they are written.
//
//! A word arrives written in its speaker's charts — vertices keyed by its
//! atlas, faces by its overlaps. Comparing two words as written judges the
//! charts, and a word written in another atlas reads "distinct" though it
//! is the same manifold. So each word is LIFTED: every evaluated member path
//! of every orbit is read by the invariants perception is built from, none
//! of which a chart can change —
//!
//! - the holonomy's conformal class `(I₁, I₂, kind)`, read off the carriage's
//!   own geometric products (conjugation keeps grades, so a change of frame
//!   keeps it);
//! - its MR1 spectrum: each commuting mode's kind, planes and order;
//! - the word rotor's class;
//! - the hue map's cycle type (its cycle lengths: a relabelling keeps them).
//!
//! And at GRADE ZERO, each word vertex by WHAT IT DOES, never where it
//! stands: its rotor's class and each member's hue cycle type — its
//! placement is frame data a change of atlas moves, so it is not perceived
//! as the word's own.
//!
//! The lift is a MULTISET of these invariants per grade, never a sequence
//! and never keyed by a vertex. Two words AGREE where their multisets are
//! one at every grade, something was perceived, and nothing went unread;
//! otherwise the comparison names what each perceives that the other does
//! not — the residue, which is content, never a count of difference. Grades
//! zero and one are read here; the cells' orbit algebras (grade two) and
//! the boundary between grades are owed.

use super::*;
use crate::network::nerve::Coordinate;

/// A WORD AS PERCEIVED: every evaluated member path of every orbit by its
/// invariants' coordinates, with how many paths read it; and the paths whose
/// holonomy could not be read (not a versor, or its modes unsplit over ℚ) —
/// named, never dropped.
#[derive(Clone, Debug, Default)]
pub struct Perceived {
    /// Grade zero: each word vertex's member by its rotor class and hue
    /// cycle type.
    pub vertices: Keyed<u64>,
    /// Grade one: each evaluated member path of each orbit by its
    /// holonomy's invariants.
    pub cycles: Keyed<u64>,
    pub unread: u64,
}

/// WHAT TWO PERCEIVED WORDS SHARE AND WHAT EACH ALONE PERCEIVES: the paths
/// matched invariant for invariant, and each side's residue by its
/// invariant's coordinates with its multiplicity.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Comparison {
    pub matched: u64,
    pub only_ours: Vec<(Vec<u8>, u64)>,
    pub only_theirs: Vec<(Vec<u8>, u64)>,
    pub unread: (u64, u64),
}

impl Comparison {
    /// The two words are perceived as one: something was perceived, no
    /// residue on either side, and nothing unread that could hide one. Two
    /// words of which nothing was perceived do not agree — they were not
    /// compared.
    pub fn agrees(&self) -> bool {
        self.matched > 0 && self.only_ours.is_empty() && self.only_theirs.is_empty() && self.unread == (0, 0)
    }
}

/// LIFT A WORD INTO PERCEPTION, under the stroke's budget (MR1's roots).
pub fn lift(form: &Form, budget: &mut crate::physics::Budget) -> Perceived {
    let mut p = Perceived::default();
    // GRADE ZERO: what each word vertex does, member by member.
    for (_, v) in form.vertices.entries() {
        let Vertex::Word { derivation, acts, .. } = v else { continue };
        let Some(class) = derivation.rotor().holonomy_key() else {
            p.unread += 1;
            continue;
        };
        for (_, a) in acts.entries() {
            let mut k = vec![0u8];
            k.extend_from_slice(class.encode().as_bytes());
            crate::transducer::sentence::CycleType::of(&a.gauge).to_bytes(&mut k);
            bump(&mut p.vertices, &k);
        }
    }
    // GRADE ONE: each orbit's member paths by their holonomy's invariants.
    for lp in &form.orbits {
        let Some(hs) = form.edge_holonomies(lp) else {
            p.unread += 1;
            continue;
        };
        for (_, e) in hs.scoped() {
            match invariant_key(e, budget) {
                Some(k) => bump(&mut p.cycles, &k),
                None => p.unread += 1,
            }
        }
    }
    p
}

/// One more of an invariant in a multiset.
fn bump(set: &mut Keyed<u64>, k: &[u8]) {
    let n = match set.get(k) {
        Some(n) => n + 1,
        None => 1,
    };
    set.insert(k, n);
}

/// ONE HOLONOMY'S INVARIANTS as coordinates: its conformal class, its
/// rotor's class, its modes (kind, planes, order) in the order MR1 reads
/// them off the algebra, and its hue cycle type. `None` where any is unread.
fn invariant_key(e: &Transport, budget: &mut crate::physics::Budget) -> Option<Vec<u8>> {
    let mut k = vec![1u8];
    k.extend_from_slice(e.carriage.holonomy_key()?.encode().as_bytes());
    k.extend_from_slice(e.word.rotor().holonomy_key()?.encode().as_bytes());
    let ModesRead::Split(modes) = ModesRead::of(&e.carriage, budget) else { return None };
    crate::transducer::address::put_key_count(&mut k, modes.len());
    for (kind, planes, order) in modes {
        k.push(kind as u8);
        k.extend_from_slice(&u64::try_from(planes).ok()?.to_le_bytes());
        match order {
            Some(o) => {
                k.push(1);
                k.extend_from_slice(&o.to_le_bytes());
            }
            None => k.push(0),
        }
    }
    crate::transducer::sentence::CycleType::of(&e.gauge).to_bytes(&mut k);
    Some(k)
}

/// COMPARE TWO PERCEIVED WORDS invariant for invariant: the paths matched,
/// and what each perceives that the other does not.
pub fn compare(ours: &Perceived, theirs: &Perceived) -> Comparison {
    let mut c = Comparison { unread: (ours.unread, theirs.unread), ..Comparison::default() };
    for (a, b) in [(&ours.vertices, &theirs.vertices), (&ours.cycles, &theirs.cycles)] {
        compare_grade(a, b, &mut c);
    }
    c
}

/// One grade's multisets, each key leading with its grade.
fn compare_grade(ours: &Keyed<u64>, theirs: &Keyed<u64>, c: &mut Comparison) {
    for (k, &n) in ours.scoped() {
        let m = match theirs.get(&k) {
            Some(&m) => m,
            None => 0,
        };
        c.matched += n.min(m);
        if n > m {
            c.only_ours.push((k.clone(), n - m));
        }
    }
    for (k, &m) in theirs.scoped() {
        let n = match ours.get(&k) {
            Some(&n) => n,
            None => 0,
        };
        if m > n {
            c.only_theirs.push((k.clone(), m - n));
        }
    }
}
