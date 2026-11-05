// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0, MU-RELEASE §3a steps 3–5 — THE CONCORDANCE CLAUSE REGISTER:
// each Concordance clause the hypha adjudicates, identified by its content
// (its law form's id and version, and its joint-read fixtures as exact
// inputs), with the hypha's own reconstruction of each fixture's object.
//
//! "Clause" here is always a clause of the Concordance, never the rule
//! language's clause (`transducer::sentence::form::clause`). A Concordance
//! clause is never identified by a digest of text (A21): its address is its
//! content written out (`writing::put_clause`), so a change of the
//! Concordance's wording changes no clause, and a change of the law is a new
//! version of the form. The fixtures are the inputs every kernel
//! reconstructs to sign the clause; [`reconstruct`] is the hypha's own
//! reconstruction of each, through the law forms (`forms`).

use super::fixtures::{coordinate_fixtures, passage_fixtures};
use algebra::Clifford;
use forms::coordinate::{Bodies, Bounds, Coordinate, CoordinateRefusal, Interval, Key, Lane, Sector, Sides};
use forms::passage::Poles;
use forms::Q;

/// THE CLAUSES THIS HYPHA ADJUDICATES, `(law id, version)`, in the
/// register's order: each a law form of the innermost shell with joint-read
/// fixtures (LAW-PASSAGE v3 as `FORMS-PASSAGE`, the form's version 4;
/// LAW-COORDINATE v3 as `FORMS-COORDINATE`, the form's version 3).
pub const CLAUSES: &[(&str, u16)] = &[
    (forms::passage::LAW_ID, forms::passage::LAW_VERSION),
    (forms::coordinate::LAW_ID, forms::coordinate::LAW_VERSION),
];

/// A CONCORDANCE CLAUSE'S NAME: its law form's id and version. With the fixtures it is
/// the clause's content address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConcordanceClauseId {
    pub law: String,
    pub version: u16,
}

impl ConcordanceClauseId {
    /// The clause of the form `(law, version)`.
    pub fn of(law: &str, version: u16) -> ConcordanceClauseId {
        ConcordanceClauseId { law: law.to_string(), version }
    }
}

/// WHAT A CLAUSE BINDS (Concordance 0.1): a network clause, which no kernel
/// may veto, or a clause a kernel may enable or veto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standing {
    /// Enforced by the hyphae for every kernel (0.1's Network row): never
    /// vetoed.
    Network,
    /// Kernel physics or game design: enabled or vetoed by each kernel.
    Kernel,
}

impl Standing {
    /// The standing's byte on the wire.
    pub const fn byte(self) -> u8 {
        match self {
            Standing::Network => 0,
            Standing::Kernel => 1,
        }
    }

    /// The standing a byte names, or none.
    pub const fn of(b: u8) -> Option<Standing> {
        match b {
            0 => Some(Standing::Network),
            1 => Some(Standing::Kernel),
            _ => None,
        }
    }
}

/// ONE JOINT-READ FIXTURE'S INPUT, exact: the operation the law states and
/// the objects it is read on.
#[derive(Clone, Debug)]
pub enum Fixture {
    /// LAW-PASSAGE fixtures 1, 2, 3 and 5: one crossing of the frame state
    /// `frame` on the plane of the units `positive` (`e₊`) and `negative`
    /// (`e₋`), with `flux` on `T` (`forms::passage::Poles::cross`).
    Crossed { positive: Clifford, negative: Clifford, frame: Clifford, flux: Q },
    /// LAW-PASSAGE fixture 4: the riding state's pass, `e₊·state`, on the
    /// plane of the units `positive` and `negative`
    /// (`forms::passage::Poles::ride`).
    Ridden { positive: Clifford, negative: Clifford, state: Clifford },
    /// LAW-COORDINATE fixtures 1a and 11: the bodies, read into the
    /// canonical order.
    Ordered { bodies: Vec<Coordinate> },
    /// LAW-COORDINATE fixture 2: the sector at `depth` holding `value`, the
    /// display reading.
    Sector { value: Q, depth: u32 },
    /// LAW-COORDINATE fixtures 4 and 5: where `at` stands against the
    /// bounds the caller states, a side per component.
    Within { at: Coordinate, bounds: Bounds },
    /// LAW-COORDINATE fixtures 6 and 12: the bodies of `held` in the range
    /// `interval` on component `key` of `lane` at `at`.
    Range { held: Vec<Coordinate>, at: Coordinate, lane: Lane, key: Key, interval: Interval },
}

/// THE EXACT OBJECT A FIXTURE REACHES.
#[derive(Clone, Debug)]
pub enum Object {
    /// A value of `Cl(p, q)`, its signature carried.
    Multivector(Clifford),
    /// One crossing's result: the frame state past the swap, its span
    /// (`S`) and the flux written on `T`.
    Crossing { frame: Clifford, span: Q, flux: Q },
    /// Bodies, in the order the reading gives them.
    Bodies(Vec<Coordinate>),
    /// A sector of the sexagesimal reading, display only.
    Sector(Sector),
    /// Where a body stands against stated bounds, a side per component.
    Sides(Sides),
    /// A reading refused by name: two origins, two shapes, or a component
    /// not held.
    Refused(CoordinateRefusal),
}

/// THE SAME VALUE OF `Cl(p, q)`: the one signature, and the same terms,
/// each blade with its exact coefficient.
fn same_value(a: &Clifford, b: &Clifford) -> bool {
    a.negatives() == b.negatives() && a.terms().eq(b.terms())
}

impl PartialEq for Fixture {
    fn eq(&self, o: &Fixture) -> bool {
        match (self, o) {
            (Fixture::Crossed { positive, negative, frame, flux }, Fixture::Crossed { positive: p, negative: n, frame: f, flux: t }) => {
                same_value(positive, p) && same_value(negative, n) && same_value(frame, f) && flux == t
            }
            (Fixture::Ridden { positive, negative, state }, Fixture::Ridden { positive: p, negative: n, state: s }) => {
                same_value(positive, p) && same_value(negative, n) && same_value(state, s)
            }
            (Fixture::Ordered { bodies }, Fixture::Ordered { bodies: other }) => bodies == other,
            (Fixture::Sector { value, depth }, Fixture::Sector { value: v, depth: d }) => value == v && depth == d,
            (Fixture::Within { at, bounds }, Fixture::Within { at: a, bounds: b }) => at == a && bounds == b,
            (Fixture::Range { held, at, lane, key, interval }, Fixture::Range { held: h, at: a, lane: l, key: k, interval: i }) => {
                held == h && at == a && lane == l && key == k && interval == i
            }
            _ => false,
        }
    }
}

/// The fixture's equality is exact on every input (the same values, the
/// same coordinates), so it is an equivalence: a fixture named in a
/// refusal on the ledger compares as the ledger's citations do.
impl Eq for Fixture {}

/// The object's equality is exact, as the fixture's.
impl Eq for Object {}

impl PartialEq for Object {
    fn eq(&self, o: &Object) -> bool {
        match (self, o) {
            (Object::Multivector(a), Object::Multivector(b)) => same_value(a, b),
            (Object::Crossing { frame, span, flux }, Object::Crossing { frame: f, span: s, flux: t }) => same_value(frame, f) && span == s && flux == t,
            (Object::Bodies(a), Object::Bodies(b)) => a == b,
            (Object::Sector(a), Object::Sector(b)) => a == b,
            (Object::Sides(a), Object::Sides(b)) => a == b,
            (Object::Refused(a), Object::Refused(b)) => a == b,
            _ => false,
        }
    }
}

/// A CONCORDANCE CLAUSE AS SERVED: its name, its standing, and its
/// fixtures' inputs.
#[derive(Clone, Debug, PartialEq)]
pub struct ConcordanceClause {
    pub id: ConcordanceClauseId,
    pub standing: Standing,
    pub fixtures: Vec<Fixture>,
}

/// A CONCORDANCE CLAUSE AS THE HYPHA HOLDS IT: the Concordance clause and
/// the hypha's own reconstruction of each fixture's object, in the
/// fixtures' order.
#[derive(Clone, Debug, PartialEq)]
pub struct Registered {
    pub clause: ConcordanceClause,
    pub reached: Vec<Object>,
}

/// WHY THE REGISTER STANDS UNREAD: the Concordance clause whose fixtures this hypha
/// cannot build or reconstruct (a generator the algebra does not hold, a
/// product the algebra refuses, or a lane set the coordinate refuses).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterRefusal {
    pub id: ConcordanceClauseId,
}

/// THE HYPHA'S RECONSTRUCTION OF ONE FIXTURE, through the law forms: the
/// object the law reaches on the fixture's input. `None` where the passage
/// refuses the input (a unit that is not one, two signatures that
/// disagree, a frame state off the plane, a riding state of both
/// parities).
pub fn reconstruct(fixture: &Fixture) -> Option<Object> {
    match fixture {
        Fixture::Crossed { positive, negative, frame, flux } => {
            let crossed = Poles::of(positive, negative).ok()?.cross(frame, flux).ok()?;
            Some(Object::Crossing { frame: crossed.frame, span: crossed.span, flux: crossed.flux })
        }
        Fixture::Ridden { positive, negative, state } => Poles::of(positive, negative).ok()?.ride(state).ok().map(Object::Multivector),
        Fixture::Ordered { bodies } => {
            let mut ordered = bodies.clone();
            ordered.sort();
            Some(Object::Bodies(ordered))
        }
        Fixture::Sector { value, depth } => Some(Object::Sector(Sector::holding(value, *depth))),
        Fixture::Within { at, bounds } => Some(match at.within(bounds) {
            Ok(sides) => Object::Sides(sides),
            Err(refusal) => Object::Refused(refusal),
        }),
        Fixture::Range { held, at, lane, key, interval } => {
            let mut bodies = Bodies::new();
            held.iter().for_each(|c| {
                bodies.hold(c.clone());
            });
            Some(match bodies.range(at, lane, key, interval) {
                Ok(found) => Object::Bodies(found.bodies.to_vec()),
                Err(refusal) => Object::Refused(refusal),
            })
        }
    }
}

/// One Concordance clause registered at its standing: its fixtures built,
/// and each reconstructed.
fn registered(id: ConcordanceClauseId, standing: Standing, fixtures: Option<Vec<Fixture>>) -> Result<Registered, RegisterRefusal> {
    let refused = || RegisterRefusal { id: id.clone() };
    let fixtures = fixtures.ok_or_else(refused)?;
    let reached = fixtures.iter().map(reconstruct).collect::<Option<Vec<_>>>().ok_or_else(refused)?;
    Ok(Registered { clause: ConcordanceClause { id: id.clone(), standing, fixtures }, reached })
}

/// THE REGISTER: every Concordance clause of [`CLAUSES`], in order, its
/// fixtures built and reconstructed by this hypha; refused by the clause
/// whose fixtures the hypha cannot read. `FORMS-PASSAGE` is a network
/// clause (held draft 2.11 names it one; LAW-PASSAGE v3 was adopted by
/// both operators on 2026-11-05, its fixtures served for the joint read).
/// `FORMS-COORDINATE` stands at kernel standing, signable and
/// vetoable, until 78.173's fixtures are joint-read agreed and its clause
/// is drafted: a clause whose fixtures no other kernel has read is never
/// made unvetoable (coordinator's decision, 2026-11-05).
pub fn register() -> Result<Vec<Registered>, RegisterRefusal> {
    Ok(vec![
        registered(ConcordanceClauseId::of(forms::passage::LAW_ID, forms::passage::LAW_VERSION), Standing::Network, passage_fixtures())?,
        registered(
            ConcordanceClauseId::of(forms::coordinate::LAW_ID, forms::coordinate::LAW_VERSION),
            Standing::Kernel,
            coordinate_fixtures(),
        )?,
    ])
}
