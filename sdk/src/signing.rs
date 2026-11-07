// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0, SIGNING BY ADJUDICATED READING — THE KERNEL'S SIDE: a
// kernel reconstructs each served clause's fixtures itself, through the law
// forms it links (`forms`), and signs its readings clause by clause.

//! A kernel's signing helper.
//!
//! A hypha serves its clause register on `ACT_LAWS` mode 7
//! ([`encode_concordance_request`], [`decode_concordance_reply`]): each
//! clause's name (law id and version), its standing and its joint-read
//! fixtures' inputs. A kernel signs a clause by reconstructing every
//! fixture's object in its own process and presenting the objects, signed
//! under its key, on mode 8 ([`encode_signature_request`]). The hypha
//! reconstructs the same fixtures and admits the signature only where the
//! kernel reached the same exact objects; otherwise it refuses
//! `NotSignatory`, naming the clause and the fixture
//! ([`decode_signature_reply`]). A network clause is never vetoed.
//!
//! [`reconstruct_fixture`] is this SDK's own reconstruction of each fixture
//! the forms clauses carry, through `forms::passage`,
//! `forms::coordinate`, `forms::coupling` and `forms::turning`; a kernel on another stack reconstructs with its own
//! code. [`sign_concordance`] signs a served register with it.
//!
//! ```no_run
//! use mycelium_sdk::signing::{decode_concordance_reply, encode_concordance_request, encode_signature_request, sign_concordance, Assent};
//! use mycelium_sdk::prelude::*;
//!
//! let (voice, key) = mycelium_sdk::voice::load_or_mint("home/voice.key");
//! let session = Session::open("127.0.0.1:7500", voice);
//! let Some(Ok(served)) = session.speak(encode_concordance_request()).ok().and_then(|r| decode_concordance_reply(&r)) else { return };
//! if let Some(signed) = sign_concordance(&key, &served, |_| Assent::Enabled) {
//!     let verdict = session.speak(encode_signature_request(&signed));
//!     println!("{verdict:?}");
//! }
//! ```

use forms::coordinate::{Bodies, Sector};
use forms::passage::Poles;

/// The Concordance clause register's types: a Concordance clause as
/// served (never the rule language's clause, `crate::rule`), its name and
/// standing, a fixture's input and the object it reaches.
pub use mycelium::physics::laws::clauses::{ConcordanceClause, ConcordanceClauseId, Fixture, Object, RegisterRefusal, Standing};
/// The LAW-COUPLING fixture's input and object (78.175 §5), and one body of
/// a range as a fixture states it.
pub use mycelium::physics::laws::clauses::{CouplingFixture, CouplingObject, Term};
/// The fixture 11b input of LAW-COORDINATE's composition of turnings
/// (78.173 §5), each reaching a value of `Cl(p, q)`.
pub use mycelium::physics::laws::clauses::TurningFixture;
/// The signing act, a clause's choice, the assent admitted and each refusal
/// by name.
pub use mycelium::physics::laws::clauses::{Admitted, Assent, Choice, SignedClause, Signing, SigningRefusal};
/// The register's read (`ACT_LAWS` mode 7), the signing act presented,
/// judged and recorded on the hypha's ledger (mode 8), and the alignment
/// reading derived from the recorded signings (mode 9). Each mode is in
/// [`crate::laws`].
pub use mycelium::physics::laws::codec::{
    decode_alignment_reply, decode_concordance_reply, decode_signature_reply, encode_alignment_request, encode_concordance_request, encode_signature_request,
};
/// The alignment reading: per kernel and per clause version, converses,
/// vetoed, collides (with the residue) or unread, derived from the signing
/// events; and the signings of the past, served beside it as history.
pub use mycelium::physics::laws::clauses::{
    read_alignment, Alignment, AlignmentRefusal, ClauseAlignment, ClauseReading, HistoricalSigning, KernelAlignment, Mechanism, SigningEvent,
};

/// THE KERNEL'S RECONSTRUCTION OF ONE FIXTURE, through the law forms: the
/// passage's crossing (the pole swap, the span and the flux) and the riding
/// state's pass in the exact algebra, the coordinate's order, sector
/// (display), sides on stated bounds and range on one component. `None`
/// where the passage refuses the input.
pub fn reconstruct_fixture(fixture: &Fixture) -> Option<Object> {
    Some(match fixture {
        Fixture::Crossed { positive, negative, frame, flux } => {
            let crossed = Poles::of(positive, negative).ok()?.cross(frame, flux).ok()?;
            Object::Crossing { frame: crossed.frame, span: crossed.span, flux: crossed.flux }
        }
        Fixture::Ridden { positive, negative, state } => Object::Multivector(Poles::of(positive, negative).ok()?.ride(state).ok()?),
        Fixture::Ordered { bodies } => {
            let mut ordered = bodies.clone();
            ordered.sort();
            Object::Bodies(ordered)
        }
        Fixture::Sector { value, depth } => Object::Sector(Sector::holding(value, *depth)),
        Fixture::Within { at, bounds } => match at.within(bounds) {
            Ok(sides) => Object::Sides(sides),
            Err(refusal) => Object::Refused(refusal),
        },
        Fixture::Range { held, at, lane, key, interval } => {
            let mut bodies = Bodies::new();
            for c in held {
                bodies.hold(c.clone());
            }
            match bodies.range(at, lane, key, interval) {
                Ok(found) => Object::Bodies(found.bodies.to_vec()),
                Err(refusal) => Object::Refused(refusal),
            }
        }
        Fixture::Coupling(f) => Object::Coupling(reconstruct_coupling(f)?),
        Fixture::Turning(f) => Object::Multivector(reconstruct_turning(f)?),
    })
}

/// THE KERNEL'S RECONSTRUCTION OF ONE FIXTURE 11b (78.173 §5), through
/// `forms::turning`: a rate's motion, a motion's generator, two motions
/// composed and two rates composed. `None` where the law refuses the input.
fn reconstruct_turning(f: &TurningFixture) -> Option<forms::algebra::Clifford> {
    use forms::turning as t;
    match f {
        TurningFixture::Motion { rate } => t::motion(rate).ok(),
        TurningFixture::Generator { motion } => t::generator(motion).ok(),
        TurningFixture::Composed { first, then } => t::compose(first, then).ok(),
        TurningFixture::Rates { first, then } => t::compose_rates(first, then).ok(),
    }
}

/// THE KERNEL'S RECONSTRUCTION OF ONE LAW-COUPLING FIXTURE (78.175 §5),
/// through `forms::coupling`: a dipole, a body's frames per plane, the
/// field's pull, the neighbour reading, light taken and a bell's ledger
/// (its `Unbalanced` refusal an object). `None` where the form refuses the
/// input otherwise.
fn reconstruct_coupling(f: &CouplingFixture) -> Option<CouplingObject> {
    use forms::coupling as c;
    Some(match f {
        CouplingFixture::Poled { held, open } => CouplingObject::Dipole(c::Parcel::of(held.clone(), open.clone()).ok()?.dipole().ok()?),
        CouplingFixture::Founded { z, u, planes, bonds } => {
            let mut body = c::Parcels::founded(z, u, planes).ok()?;
            for (plane, closed) in bonds {
                body = body.bond(plane, closed).ok()?;
            }
            CouplingObject::Frames(body.planes().iter().map(|(k, p)| (k.clone(), p.held().clone(), p.open().clone())).collect())
        }
        CouplingFixture::Pulled { rate, held_rate, pole, open, plane, range } => {
            let mut held = Vec::with_capacity(range.len());
            for t in range {
                let along = c::Alignment::on(plane.clone(), t.separation.clone()).ok()?;
                held.push(c::Neighbour::of(t.holding.clone(), c::Dipole::stated(t.pole.clone(), t.open.clone()).ok()?, t.nearness.clone(), along).ok()?);
            }
            let field = c::field(plane, &held).ok()?;
            let pull = c::pull(rate, held_rate, &c::Dipole::stated(pole.clone(), open.clone()).ok()?, &field).ok()?;
            CouplingObject::Pull { field, pull }
        }
        CouplingFixture::Met { a, b } => CouplingObject::Meeting(c::meets(a, b).ok()?),
        CouplingFixture::Taken { light, face, hold, reach } => CouplingObject::Taken(c::take(light, *face, hold, reach).ok()?),
        CouplingFixture::Ledgered { made, received, drained } => match c::ledger(made, received, drained) {
            Ok(l) => CouplingObject::Ledger(l),
            Err(c::CouplingRefusal::Unbalanced { made, received, drained, residue }) => {
                CouplingObject::Unbalanced { made: *made, received: *received, drained: *drained, residue: *residue }
            }
            Err(_) => return None,
        },
    })
}

/// A KERNEL'S SIGNING ACT OVER `served`, by its key: each clause with the
/// assent `assent` gives it, an enabled clause carrying the kernel's own
/// reconstruction of each of its fixtures. `None` where a fixture of an
/// enabled clause does not reconstruct here.
pub fn sign_concordance(key: &ed25519_dalek::SigningKey, served: &[ConcordanceClause], assent: impl Fn(&ConcordanceClause) -> Assent) -> Option<Signing> {
    let clauses = served
        .iter()
        .map(|c| {
            let choice = match assent(c) {
                Assent::Enabled => Choice::Enabled { reached: c.fixtures.iter().map(reconstruct_fixture).collect::<Option<Vec<_>>>()? },
                Assent::Vetoed => Choice::Vetoed,
            };
            Some(SignedClause { id: c.id.clone(), fixtures: c.fixtures.clone(), choice })
        })
        .collect::<Option<Vec<_>>>()?;
    Some(Signing::sign(key, clauses))
}
