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
use forms::coordinate::{Bodies, Coordinate, CoordinateRefusal, Lane, Sector, Side};
use forms::Q;

/// THE CLAUSES THIS HYPHA ADJUDICATES, `(law id, version)`, in the
/// register's order: each a law form of the innermost shell with joint-read
/// fixtures (LAW-PASSAGE v2 as `FORMS-PASSAGE`, LAW-COORDINATE v1 as
/// `FORMS-COORDINATE`, the form's version 2).
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
    /// LAW-PASSAGE fixture 1: `v·(v·q)`, the state carried once and the
    /// product with the direction again.
    PassedTwice { v: Clifford, q: Clifford },
    /// LAW-PASSAGE fixture 2: one crossing, `pass(v, q) = v·q`.
    Passed { v: Clifford, q: Clifford },
    /// LAW-PASSAGE fixture 3: the commutator unhalved, `a·b − b·a`.
    Commutator { a: Clifford, b: Clifford },
    /// LAW-PASSAGE fixture 4: the square `x·x`.
    Square { x: Clifford },
    /// LAW-COORDINATE fixture 1a: the bodies, read into the canonical order.
    Ordered { bodies: Vec<Coordinate> },
    /// LAW-COORDINATE fixture 2: the sector at `depth` holding `value`.
    Sector { value: Q, depth: u32 },
    /// LAW-COORDINATE fixtures 4 and 5: where `at` stands on `lane` against
    /// the sector at `depth` holding `other`'s value.
    Within { at: Coordinate, other: Coordinate, lane: Lane, depth: u32 },
    /// LAW-COORDINATE fixture 6: the bodies of `held` in the range
    /// `[lo, hi)` on `lane` at `at`.
    Range { held: Vec<Coordinate>, at: Coordinate, lane: Lane, lo: Q, hi: Q },
}

/// THE EXACT OBJECT A FIXTURE REACHES.
#[derive(Clone, Debug)]
pub enum Object {
    /// A value of `Cl(p, q)`, its signature carried.
    Multivector(Clifford),
    /// Bodies, in the order the reading gives them.
    Bodies(Vec<Coordinate>),
    /// A sector of the sexagesimal reading.
    Sector(Sector),
    /// Where a body stands against a sector.
    Side(Side),
    /// A reading refused because the two origins differ, naming both.
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
            (Fixture::PassedTwice { v, q }, Fixture::PassedTwice { v: w, q: r }) => same_value(v, w) && same_value(q, r),
            (Fixture::Passed { v, q }, Fixture::Passed { v: w, q: r }) => same_value(v, w) && same_value(q, r),
            (Fixture::Commutator { a, b }, Fixture::Commutator { a: c, b: d }) => same_value(a, c) && same_value(b, d),
            (Fixture::Square { x }, Fixture::Square { x: y }) => same_value(x, y),
            (Fixture::Ordered { bodies }, Fixture::Ordered { bodies: other }) => bodies == other,
            (Fixture::Sector { value, depth }, Fixture::Sector { value: v, depth: d }) => value == v && depth == d,
            (Fixture::Within { at, other, lane, depth }, Fixture::Within { at: a, other: b, lane: l, depth: d }) => {
                at == a && other == b && lane == l && depth == d
            }
            (Fixture::Range { held, at, lane, lo, hi }, Fixture::Range { held: h, at: a, lane: l, lo: x, hi: y }) => {
                held == h && at == a && lane == l && lo == x && hi == y
            }
            _ => false,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, o: &Object) -> bool {
        match (self, o) {
            (Object::Multivector(a), Object::Multivector(b)) => same_value(a, b),
            (Object::Bodies(a), Object::Bodies(b)) => a == b,
            (Object::Sector(a), Object::Sector(b)) => a == b,
            (Object::Side(a), Object::Side(b)) => a == b,
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
/// cannot build or reconstruct (a generator the algebra does not hold, or a
/// product the algebra refuses).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterRefusal {
    pub id: ConcordanceClauseId,
}

/// THE HYPHA'S RECONSTRUCTION OF ONE FIXTURE, through the law forms: the
/// object the law reaches on the fixture's input. `None` where the algebra
/// refuses a product of the input (two signatures that disagree, a
/// direction that is not degree 1, a state that is not even).
pub fn reconstruct(fixture: &Fixture) -> Option<Object> {
    let value = |r: Result<Clifford, algebra::SignatureRefusal>| r.ok().map(Object::Multivector);
    match fixture {
        Fixture::PassedTwice { v, q } => value(v.product(&forms::passage::pass(v, q).ok()?)),
        Fixture::Passed { v, q } => forms::passage::pass(v, q).ok().map(Object::Multivector),
        Fixture::Commutator { a, b } => value(a.product(b).and_then(|ab| ab.difference(&b.product(a)?))),
        Fixture::Square { x } => value(x.product(x)),
        Fixture::Ordered { bodies } => {
            let mut ordered = bodies.clone();
            ordered.sort();
            Some(Object::Bodies(ordered))
        }
        Fixture::Sector { value, depth } => Some(Object::Sector(Sector::holding(value, *depth))),
        Fixture::Within { at, other, lane, depth } => Some(match at.within(other, *lane, *depth) {
            Ok(side) => Object::Side(side),
            Err(refusal) => Object::Refused(refusal),
        }),
        Fixture::Range { held, at, lane, lo, hi } => {
            let mut bodies = Bodies::new();
            held.iter().for_each(|c| {
                bodies.hold(c.clone());
            });
            Some(Object::Bodies(bodies.range(at, *lane, lo, hi).bodies.to_vec()))
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
/// clause (draft 5 names 2.11 one; its fixtures were joint-read by every
/// kernel). `FORMS-COORDINATE` stands at kernel standing, signable and
/// vetoable, until 78.173's fixtures are joint-read agreed and its clause
/// is drafted: a clause whose fixtures no other kernel has read is never
/// made unvetoable (coordinator's decision, 2026-11-05).
pub fn register() -> Result<Vec<Registered>, RegisterRefusal> {
    Ok(vec![
        registered(ConcordanceClauseId::of(forms::passage::LAW_ID, forms::passage::LAW_VERSION), Standing::Network, passage_fixtures())?,
        registered(
            ConcordanceClauseId::of(forms::coordinate::LAW_ID, forms::coordinate::LAW_VERSION),
            Standing::Kernel,
            Some(coordinate_fixtures()),
        )?,
    ])
}
