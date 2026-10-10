// 78.160 P1 — THE PRESENTATION BY HOLES: every word vertex a language holds,
// filed under the holes its whole motion winds about, so a motion it is asked
// of is deconstructed over what is held, hole by hole (78.100 D3), at the
// coordinates the motion itself names — no chart, no lane, no search.
//
//! A court holds one over the words it ruled and answers a mind's residue with
//! it: the mind's pressure is the ask, the residue's own holes select what is
//! sent, and the exact composition of what is cited is the certificate. A
//! word's motion is its least member's carriage composed with its rotor's base
//! action (`conformal_of_rotor`), read in Cl(4,1) — never the rotor alone.

use super::coordinates::{Coordinates, HoleIndex, Unread};
use super::{Form, Vertex};
use crate::network::nerve::Keyed;
use crate::transducer::address::Address;
use crate::transducer::cga::Mv;
use num_rational::BigRational;

/// ONE GENERATOR OF THE PRESENTATION: the form and the word vertex inside it,
/// the vertex's whole motion, and that motion's coordinates.
#[derive(Clone, Debug)]
pub struct Generator {
    pub form: Address,
    pub vertex: Address,
    pub motion: Mv,
    pub coords: Coordinates,
}

/// THE HELD GENERATORS BY THE HOLES THEY STAND ON, keyed by
/// [`super::generator_key`] — the same key the lane covers file a generator
/// under, so one generator is one entry whatever reaches it.
#[derive(Default)]
pub struct Presentation {
    holes: HoleIndex,
    held: Keyed<Generator>,
}

/// A MOTION DECONSTRUCTED OVER A PRESENTATION: the generators cited, each
/// `(form, vertex, exponent)`; whether their product IS the motion, exactly
/// (projectively, 1.5); how many of its holes no held generator reached; and
/// how many holes it has.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Deconstructed {
    pub cites: Vec<(Address, Address, i64)>,
    pub composes: bool,
    pub residue: u64,
    pub holes: u64,
}

/// THE WHOLE MOTION OF A WORD VERTEX: its least member's carriage composed
/// with its rotor's base action — the member the vertex's own key order puts
/// first (content order, rule 15). `None` for any other vertex, or one whose
/// rotor has no base action.
pub fn vertex_motion(v: &Vertex) -> Option<Mv> {
    let Vertex::Word { derivation, acts, .. } = v else { return None };
    let (_, a) = acts.scoped().next()?;
    crate::transducer::element::conformal_of_rotor(&derivation.rotor(), &a.carriage)
}

/// Whether a motion is the identity ray.
pub fn is_still(m: &Mv) -> bool {
    m.projective_eq(&Mv::scalar(BigRational::from_integer(1.into())))
}

/// A MOTION'S CANONICAL KEY: its ray written out (A1), the coordinate a
/// mind files what it was answered for it under.
pub fn motion_key(m: &Mv) -> Option<Vec<u8>> {
    let mut k = Vec::new();
    m.canonical()?.put_canonical(&mut k);
    Some(k)
}

/// `mᵉ` FOR AN EXACT INTEGER EXPONENT, as a ray: a negative power is the
/// reverse's (a versor's inverse up to its weight), squared and multiplied
/// over the bits of `|e|` — the carrier's own width, never a cap.
pub fn motion_power(m: &Mv, e: i64) -> Mv {
    let base = match e < 0 {
        true => m.reverse(),
        false => m.clone(),
    };
    let mut r = Mv::scalar(BigRational::from_integer(1.into()));
    let mut square = base;
    let mut rest = e.unsigned_abs();
    while rest > 0 {
        if rest & 1 == 1 {
            r = r.geo(&square);
        }
        rest >>= 1;
        if rest > 0 {
            square = square.geo(&square);
        }
    }
    r
}

impl Presentation {
    /// How many generators it holds.
    pub fn len(&self) -> usize {
        self.held.len()
    }

    pub fn is_empty(&self) -> bool {
        self.held.is_empty()
    }

    /// FILE A FORM'S WORD VERTICES under the holes their motions wind about,
    /// each motion's coordinates read under the stroke. A vertex that moves
    /// nothing has no hole and is not filed; one whose modes do not read on
    /// this stroke is not filed either, and the count says how many were.
    pub fn file_form(&mut self, form: &Address, f: &Form, budget: &mut crate::physics::Budget) -> usize {
        let mut filed = 0usize;
        for (_, v) in f.vertices.scoped() {
            let Vertex::Word { address, .. } = v else { continue };
            let Some(motion) = vertex_motion(v).filter(|m| !is_still(m)) else { continue };
            let Ok(coords) = Coordinates::of(&motion, budget) else { continue };
            if coords.holes.is_empty() {
                continue;
            }
            let key = super::generator_key(form, address);
            self.holes.file(&key, &coords);
            self.held.insert(&key, Generator { form: form.clone(), vertex: address.clone(), motion, coords });
            filed += 1;
        }
        filed
    }

    /// The mirror of [`Presentation::file_form`], vertex for vertex.
    pub fn unfile_form(&mut self, form: &Address, f: &Form) {
        for (_, v) in f.vertices.scoped() {
            let Vertex::Word { address, .. } = v else { continue };
            let key = super::generator_key(form, address);
            if let Some(g) = self.held.remove(&key) {
                self.holes.unfile(&key, &g.coords);
            }
        }
    }

    /// 78.100 D3 — A MOTION DECONSTRUCTED OVER WHAT IS HELD, hole by hole:
    /// each hole cited to a held generator standing on it whose winding
    /// reaches the motion's by an exponent; one generator covering several
    /// holes at one exponent is one citation. It COMPOSES where every hole is
    /// cited and the citations' powers multiply, in the order of the motion's
    /// holes, to the motion itself — exact, or it does not. `Err` by name
    /// where the motion's own coordinates do not read.
    pub fn deconstruct(&self, motion: &Mv, budget: &mut crate::physics::Budget) -> Result<Deconstructed, Unread> {
        let coords = Coordinates::of(motion, budget)?;
        let d = self.holes.deconstruct(&coords, |k| self.held.get(k).map(|g| &g.coords));
        let mut cites: Vec<(Vec<u8>, i64)> = Vec::new();
        for c in &d.cites {
            match cites.iter().find(|(w, _)| *w == c.word) {
                Some((_, e)) if *e == c.exponent => {}
                _ => cites.push((c.word.clone(), c.exponent)),
            }
        }
        let mut composed = Mv::scalar(BigRational::from_integer(1.into()));
        let mut out = Vec::with_capacity(cites.len());
        for (k, e) in &cites {
            let Some(g) = self.held.get(k) else { continue };
            composed = composed.geo(&motion_power(&g.motion, *e));
            out.push((g.form.clone(), g.vertex.clone(), *e));
        }
        let composes = d.residue.is_empty() && !out.is_empty() && composed.projective_eq(motion);
        Ok(Deconstructed {
            cites: out,
            composes,
            residue: d.residue.len() as u64,
            holes: coords.holes.len() as u64,
        })
    }
}
