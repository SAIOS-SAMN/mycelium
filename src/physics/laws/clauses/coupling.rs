// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-RELEASE §3a steps 3–5 — LAW-COUPLING v1's JOINT-READ FIXTURES AS A
// CONCORDANCE CLAUSE (proposal 78.175 §5, `FORMS-COUPLING` v1, at kernel
// standing): each fixture's input, the object it reaches through
// `forms::coupling`, and both written out in exact bytes.
//
//! The fixtures stand on the turnings about axes 1, 2 and 3, `e₊₂e₊₃`,
//! `e₊₃e₊₁` and `e₊₁e₊₂` (τ₁₂, the field's plane), each a generator of two
//! positive frame units. A fixture is one operation of the form: a parcel's
//! dipole (F1–F2), a body founded on its planes and bonded plane by plane
//! (F1 per plane), the field's pull on a rate (F3), the neighbour reading
//! (F3), light taken on a face and a bell's ledger (F4). The unbalanced
//! ledger reaches its refusal as an object, with its residue; every other
//! refusal of the form is claimed in `forms` alone, and a fixture the form
//! refuses otherwise reconstructs as none. The ratio `ν(t)`, the inversion,
//! the named L1 readings and the claims over generated cases are read in
//! `forms` alone (`forms/src/tests/coupling.rs`).

use super::writing::{put_key, take_key, take_u8};
use crate::transducer::address::{put_key_count, take_key_count};
use crate::transducer::alphabet::{put_q, take_q};
use forms::coordinate::{Axis, Key, Square};
use forms::coupling::{
    field, ledger, meets, pull, take, Alignment, CouplingRefusal, Dipole, Enclosure, Face, Field, Ledger, Meeting, Meets, Neighbour, Parcel, Parcels, Pull,
    Taken,
};
use forms::Q;

/// ONE BODY OF A RANGE AS A FIXTURE STATES IT: its holding, its dipole's
/// pole and open share, its nearness, and its round separation in degrees
/// on the field's plane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Term {
    pub holding: Q,
    pub pole: Q,
    pub open: Q,
    pub nearness: Q,
    pub separation: Q,
}

/// ONE LAW-COUPLING FIXTURE'S INPUT, exact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CouplingFixture {
    /// F1–F2: the dipole of the frames `(z, q)` (`Parcel::dipole`).
    Poled { held: Q, open: Q },
    /// F1 per plane: the kind `(Z, U)` founded on `planes` by the even
    /// split, then each bond in order, a plane and the frames it closes
    /// (`Parcels::founded`, `Parcels::bond`).
    Founded { z: Q, u: Q, planes: Vec<Key>, bonds: Vec<(Key, Q)> },
    /// F3: the field of `range` on `plane`, and its pull on a body of
    /// dipole `(pole, open)` at `rate`, toward the range's `held_rate`
    /// (`field`, `pull`).
    Pulled { rate: Q, held_rate: Q, pole: Q, open: Q, plane: Key, range: Vec<Term> },
    /// F3: whether two bodies' own enclosures meet (`meets`).
    Met { a: Enclosure, b: Enclosure },
    /// F4: light reaching a body of hold `hold` at `reach` on `face`
    /// (`take`).
    Taken { light: Q, face: Face, hold: Q, reach: Q },
    /// F4: one bell's ledger (`ledger`).
    Ledgered { made: Q, received: Q, drained: Q },
}

/// THE EXACT OBJECT A LAW-COUPLING FIXTURE REACHES.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CouplingObject {
    /// A dipole: its pole and open share.
    Dipole(Dipole),
    /// A body's frames per plane, in the order founded: each plane, its
    /// held and its open frames.
    Frames(Vec<(Key, Q, Q)>),
    /// The field and the pull it gives.
    Pull { field: Field, pull: Pull },
    /// The neighbour reading.
    Meeting(Meeting),
    /// Light received and drained.
    Taken(Taken),
    /// A bell's ledger, balanced.
    Ledger(Ledger),
    /// A bell's ledger refused `Unbalanced`: the three as stated and the
    /// residue.
    Unbalanced { made: Q, received: Q, drained: Q, residue: Q },
}

/// THE HYPHA'S RECONSTRUCTION OF ONE FIXTURE through `forms::coupling`;
/// `None` where the form refuses its input otherwise than `Unbalanced`.
pub fn reached(f: &CouplingFixture) -> Option<CouplingObject> {
    Some(match f {
        CouplingFixture::Poled { held, open } => CouplingObject::Dipole(Parcel::of(held.clone(), open.clone()).ok()?.dipole().ok()?),
        CouplingFixture::Founded { z, u, planes, bonds } => {
            let body = bonds.iter().try_fold(Parcels::founded(z, u, planes).ok()?, |b, (k, c)| b.bond(k, c).ok())?;
            CouplingObject::Frames(body.planes().iter().map(|(k, p)| (k.clone(), p.held().clone(), p.open().clone())).collect())
        }
        CouplingFixture::Pulled { rate, held_rate, pole, open, plane, range } => {
            let range = range
                .iter()
                .map(|t| {
                    let d = Dipole::stated(t.pole.clone(), t.open.clone()).ok()?;
                    Neighbour::of(t.holding.clone(), d, t.nearness.clone(), Alignment::on(plane.clone(), t.separation.clone()).ok()?).ok()
                })
                .collect::<Option<Vec<_>>>()?;
            let field = field(plane, &range).ok()?;
            let pull = pull(rate, held_rate, &Dipole::stated(pole.clone(), open.clone()).ok()?, &field).ok()?;
            CouplingObject::Pull { field, pull }
        }
        CouplingFixture::Met { a, b } => CouplingObject::Meeting(meets(a, b).ok()?),
        CouplingFixture::Taken { light, face, hold, reach } => CouplingObject::Taken(take(light, *face, hold, reach).ok()?),
        CouplingFixture::Ledgered { made, received, drained } => match ledger(made, received, drained) {
            Ok(l) => CouplingObject::Ledger(l),
            Err(CouplingRefusal::Unbalanced { made, received, drained, residue }) => {
                CouplingObject::Unbalanced { made: *made, received: *received, drained: *drained, residue: *residue }
            }
            Err(_) => return None,
        },
    })
}

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// The turning `e₊ₖe₊ₗ`.
fn turning(k: u32, l: u32) -> Option<Key> {
    Key::generator(Axis::unit(Square::Positive, k).ok()?, Axis::unit(Square::Positive, l).ok()?).ok()
}

/// A body of a range on τ₁₂: `(w, σ, o, n, Δ)`.
fn term(holding: Q, pole: Q, open: Q, nearness: Q, separation: Q) -> Term {
    Term { holding, pole, open, nearness, separation }
}

/// LAW-COUPLING v1's FIXTURES (78.175 §5) with an input and an object, in
/// the proposal's order: the parcels of F1–F2; the bodies founded and
/// bonded per plane; the nine pulls of F3, every dipole one its frames
/// hold; the three neighbour readings; light on each face, and the three
/// ledgers of F4. `None` where the algebra holds no plane a fixture names.
pub fn coupling_fixtures() -> Option<Vec<CouplingFixture>> {
    let mut f: Vec<CouplingFixture> =
        [(3, 1), (2, 1), (1, 1), (0, 1), (1, 0), (2, 4)].into_iter().map(|(a, o)| CouplingFixture::Poled { held: q(a, 1), open: q(o, 1) }).collect();
    let (tau, planes) = (turning(1, 2)?, vec![turning(2, 3)?, turning(3, 1)?, turning(1, 2)?]);
    let founded = |z: Q, bonds: Vec<(Key, Q)>| CouplingFixture::Founded { z, u: q(1, 1), planes: planes.clone(), bonds };
    f.push(founded(q(1, 1), vec![]));
    f.push(founded(q(4, 3), vec![]));
    f.push(founded(q(1, 1), vec![(tau.clone(), q(1, 3))]));
    f.push(founded(q(1, 1), vec![(tau.clone(), q(1, 3)), (tau.clone(), q(-1, 3))]));
    let pulled = |rate: Q, held_rate: Q, (pole, open): (Q, Q), range: Vec<Term>| CouplingFixture::Pulled { rate, held_rate, pole, open, plane: tau.clone(), range };
    // Every dipole on its frames, `σ = 1 − 2o`: hydrogen `(2, 1)` and the
    // open-heavy kind's frames swapped `(4, 2)` both `(⅓, ⅓)`; the
    // open-heavy kind `(2, 4)` `(−⅓, ⅔)`; the throat `(1, 1)` `(0, ½)`; the
    // body `(3, 1)` `(½, ¼)`; the open pole `(0, 1)` `(−1, 1)`; the pure Z
    // pole `(1, 0)` `(1, 0)`.
    let (hydrogen, heavy, pure_z) = ((q(1, 3), q(1, 3)), (q(-1, 3), q(2, 3)), (q(1, 1), q(0, 1)));
    let near = |(pole, open): (Q, Q), separation: i64| term(q(1, 1), pole, open, q(2, 3), q(separation, 1));
    let (fifth, half) = (q(1, 5), q(1, 2));
    f.extend([
        pulled(fifth.clone(), half.clone(), hydrogen.clone(), vec![near(heavy.clone(), 0)]),
        pulled(fifth.clone(), half.clone(), hydrogen.clone(), vec![near(hydrogen.clone(), 0)]),
        pulled(fifth.clone(), half.clone(), hydrogen.clone(), vec![]),
        pulled(fifth.clone(), half.clone(), hydrogen.clone(), vec![near((q(0, 1), q(1, 2)), 0)]),
        pulled(fifth.clone(), half.clone(), hydrogen.clone(), vec![near(heavy.clone(), 45)]),
        pulled(fifth.clone(), half.clone(), (q(1, 2), q(1, 4)), vec![near(hydrogen.clone(), 0)]),
        pulled(q(99, 100), q(-99, 100), (q(-1, 1), q(1, 1)), vec![term(q(1, 1), q(-1, 1), q(1, 1), q(1, 1), q(0, 1))]),
        pulled(fifth.clone(), half.clone(), hydrogen, vec![near(pure_z.clone(), 0)]),
        pulled(fifth, half, pure_z, vec![near(heavy, 0)]),
    ]);
    let at = |r: i64, m: i64, h: i64| Enclosure { reach: q(r, 1), moment: q(m, 1), hold: q(h, 1) };
    f.extend([at(2, 1, 3), at(2, 1, 4), at(2, 1, 1)].into_iter().map(|b| CouplingFixture::Met { a: at(1, 1, 1), b }));
    f.extend([Face::Continuum, Face::Vacuum].into_iter().map(|face| CouplingFixture::Taken { light: q(8, 1), face, hold: q(1, 1), reach: q(3, 1) }));
    f.extend([(6, 2), (2, 6), (6, 1)].into_iter().map(|(r, d)| CouplingFixture::Ledgered { made: q(8, 1), received: q(r, 1), drained: q(d, 1) }));
    Some(f)
}

fn face_byte(f: Face) -> u8 {
    match f {
        Face::Continuum => 0,
        Face::Vacuum => 1,
    }
}

fn face_of(b: u8) -> Option<Face> {
    match b {
        0 => Some(Face::Continuum),
        1 => Some(Face::Vacuum),
        _ => None,
    }
}

fn meets_byte(m: Meets) -> u8 {
    match m {
        Meets::Within => 0,
        Meets::Bound => 1,
        Meets::Apart => 2,
    }
}

fn meets_of(b: u8) -> Option<Meets> {
    match b {
        0 => Some(Meets::Within),
        1 => Some(Meets::Bound),
        2 => Some(Meets::Apart),
        _ => None,
    }
}

fn put_qs(out: &mut Vec<u8>, qs: &[&Q]) {
    qs.iter().for_each(|x| put_q(out, x));
}

fn put_enclosure(out: &mut Vec<u8>, e: &Enclosure) {
    put_qs(out, &[&e.reach, &e.moment, &e.hold]);
}

fn take_enclosure(b: &[u8], p: &mut usize) -> Option<Enclosure> {
    Some(Enclosure { reach: take_q(b, p)?, moment: take_q(b, p)?, hold: take_q(b, p)? })
}

/// A COUPLING FIXTURE'S INPUT: its operation's byte, then its values.
pub fn put_coupling_fixture(out: &mut Vec<u8>, f: &CouplingFixture) {
    match f {
        CouplingFixture::Poled { held, open } => {
            out.push(0);
            put_qs(out, &[held, open]);
        }
        CouplingFixture::Founded { z, u, planes, bonds } => {
            out.push(1);
            put_qs(out, &[z, u]);
            put_key_count(out, planes.len());
            planes.iter().for_each(|k| put_key(out, k));
            put_key_count(out, bonds.len());
            for (k, c) in bonds {
                put_key(out, k);
                put_q(out, c);
            }
        }
        CouplingFixture::Pulled { rate, held_rate, pole, open, plane, range } => {
            out.push(2);
            put_qs(out, &[rate, held_rate, pole, open]);
            put_key(out, plane);
            put_key_count(out, range.len());
            range.iter().for_each(|t| put_qs(out, &[&t.holding, &t.pole, &t.open, &t.nearness, &t.separation]));
        }
        CouplingFixture::Met { a, b } => {
            out.push(3);
            put_enclosure(out, a);
            put_enclosure(out, b);
        }
        CouplingFixture::Taken { light, face, hold, reach } => {
            out.push(4);
            put_q(out, light);
            out.push(face_byte(*face));
            put_qs(out, &[hold, reach]);
        }
        CouplingFixture::Ledgered { made, received, drained } => {
            out.push(5);
            put_qs(out, &[made, received, drained]);
        }
    }
}

/// A coupling fixture read back by its operation's byte.
pub fn take_coupling_fixture(b: &[u8], p: &mut usize) -> Option<CouplingFixture> {
    Some(match take_u8(b, p)? {
        0 => CouplingFixture::Poled { held: take_q(b, p)?, open: take_q(b, p)? },
        1 => {
            let (z, u) = (take_q(b, p)?, take_q(b, p)?);
            let n = take_key_count(b, p)?;
            let planes = (0..n).map(|_| take_key(b, p)).collect::<Option<Vec<_>>>()?;
            let m = take_key_count(b, p)?;
            let bonds = (0..m).map(|_| Some((take_key(b, p)?, take_q(b, p)?))).collect::<Option<Vec<_>>>()?;
            CouplingFixture::Founded { z, u, planes, bonds }
        }
        2 => {
            let (rate, held_rate, pole, open) = (take_q(b, p)?, take_q(b, p)?, take_q(b, p)?, take_q(b, p)?);
            let plane = take_key(b, p)?;
            let n = take_key_count(b, p)?;
            let range = (0..n)
                .map(|_| Some(Term { holding: take_q(b, p)?, pole: take_q(b, p)?, open: take_q(b, p)?, nearness: take_q(b, p)?, separation: take_q(b, p)? }))
                .collect::<Option<Vec<_>>>()?;
            CouplingFixture::Pulled { rate, held_rate, pole, open, plane, range }
        }
        3 => CouplingFixture::Met { a: take_enclosure(b, p)?, b: take_enclosure(b, p)? },
        4 => CouplingFixture::Taken { light: take_q(b, p)?, face: face_of(take_u8(b, p)?)?, hold: take_q(b, p)?, reach: take_q(b, p)? },
        5 => CouplingFixture::Ledgered { made: take_q(b, p)?, received: take_q(b, p)?, drained: take_q(b, p)? },
        _ => return None,
    })
}

/// A COUPLING OBJECT: its reading's byte, then its values. A field alone
/// is written `0`; a field held `1`, its holding and its value.
pub fn put_coupling_object(out: &mut Vec<u8>, o: &CouplingObject) {
    match o {
        CouplingObject::Dipole(d) => {
            out.push(0);
            put_qs(out, &[&d.pole, &d.open]);
        }
        CouplingObject::Frames(planes) => {
            out.push(1);
            put_key_count(out, planes.len());
            for (k, z, q) in planes {
                put_key(out, k);
                put_qs(out, &[z, q]);
            }
        }
        CouplingObject::Pull { field, pull } => {
            out.push(2);
            match field {
                Field::Alone => out.push(0),
                Field::Held { holding, field } => {
                    out.push(1);
                    put_qs(out, &[holding, field]);
                }
            }
            put_qs(out, &[&pull.gap, &pull.push, &pull.rate]);
        }
        CouplingObject::Meeting(m) => {
            out.push(3);
            put_qs(out, &[&m.separation, &m.reach]);
            out.push(meets_byte(m.meets));
        }
        CouplingObject::Taken(t) => {
            out.push(4);
            put_qs(out, &[&t.received, &t.drained]);
        }
        CouplingObject::Ledger(l) => {
            out.push(5);
            put_qs(out, &[&l.made, &l.received, &l.drained]);
        }
        CouplingObject::Unbalanced { made, received, drained, residue } => {
            out.push(6);
            put_qs(out, &[made, received, drained, residue]);
        }
    }
}

/// A coupling object read back by its reading's byte.
pub fn take_coupling_object(b: &[u8], p: &mut usize) -> Option<CouplingObject> {
    Some(match take_u8(b, p)? {
        0 => CouplingObject::Dipole(Dipole { pole: take_q(b, p)?, open: take_q(b, p)? }),
        1 => {
            let n = take_key_count(b, p)?;
            CouplingObject::Frames((0..n).map(|_| Some((take_key(b, p)?, take_q(b, p)?, take_q(b, p)?))).collect::<Option<Vec<_>>>()?)
        }
        2 => {
            let field = match take_u8(b, p)? {
                0 => Field::Alone,
                1 => Field::Held { holding: take_q(b, p)?, field: take_q(b, p)? },
                _ => return None,
            };
            CouplingObject::Pull { field, pull: Pull { gap: take_q(b, p)?, push: take_q(b, p)?, rate: take_q(b, p)? } }
        }
        3 => CouplingObject::Meeting(Meeting { separation: take_q(b, p)?, reach: take_q(b, p)?, meets: meets_of(take_u8(b, p)?)? }),
        4 => CouplingObject::Taken(Taken { received: take_q(b, p)?, drained: take_q(b, p)? }),
        5 => CouplingObject::Ledger(Ledger { made: take_q(b, p)?, received: take_q(b, p)?, drained: take_q(b, p)? }),
        6 => CouplingObject::Unbalanced { made: take_q(b, p)?, received: take_q(b, p)?, drained: take_q(b, p)?, residue: take_q(b, p)? },
        _ => return None,
    })
}
