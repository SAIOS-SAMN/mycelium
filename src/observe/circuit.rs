// 78.12(4), PL1 — THE CIRCUIT'S TRAJECTORY, STEP BY STEP: the bricks, not the
// house. Every kept circuit of the last percept read at each member step —
// the face crossed and which way, whether the face refracts, the step's own
// ray and its class and modes, and the class of the transport composed so
// far — served to the console on demand and never printed per epoch.
//
//! The census line `modes [none]` on thirty orbits says the house has no
//! walls and nothing about the bricks. This reading is the bricks: a
//! trivial holonomy reads as the sequence of steps whose product
//! telescopes, and the first step whose face refracts is named where one
//! exists — where a wall could stand.
//!
//! Three of 78.12(4)'s fields have no producer yet, each named by the row
//! that owes it in [`CIRCUIT_OWED`], never printed as a zero.

use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cga::Undecomposed;
use crate::transducer::sentence::ModesRead;
use num_bigint::BigInt;
use num_rational::BigRational;

/// The fields 78.12(4) names that no producer reads yet, each with the row
/// that owes it. The ONE statement of what is owed (law 12), read by every
/// renderer.
pub const CIRCUIT_OWED: [(&str, &str); 3] = [
    ("constructors composed", "CS1: the reflections a step's transport is the product of"),
    ("round carried", "CG1: M_i = g_i M_{i-1} g̃_i, the orbit algebra's round"),
    ("lane drawn", "E5 (MS1): mass per lane per layer per grade"),
];

/// EVERY KEPT CIRCUIT OF THE LAST PERCEPT, in the order the walk read them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Circuits {
    pub circuits: Vec<Circuit>,
}

/// ONE CIRCUIT: the chord that closes it and its steps in boundary order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Circuit {
    pub chord: u16,
    pub steps: Vec<CircuitStep>,
}

impl Circuit {
    /// THE FIRST STEP WHERE A REFRACTION ENTERS: the first face crossed that
    /// carries an interface. `None` where no step refracts, which is a
    /// reading and not a refusal.
    pub fn first_refraction(&self) -> Option<usize> {
        self.steps.iter().position(|s| s.refracted)
    }
}

/// ONE MEMBER STEP.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitStep {
    /// The face crossed, by its index in the form.
    pub face: u16,
    /// Along the face's own orientation, or against it.
    pub forward: bool,
    /// The face carries an interface: two bodies in contact, a refraction.
    pub refracted: bool,
    pub carried: Carried,
}

/// WHAT ONE STEP CARRIED, oriented the way the circuit runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Carried {
    /// The step's transport was not read (rule 11: unread is not flat).
    Unread,
    /// The step's transport inverts on no ray: it is on the light cone.
    Cone,
    Read {
        /// The step's versor ray in its canonical coordinates: each non-zero
        /// blade and its exact coefficient, the first `+1` (A4).
        ray: Vec<(u16, BigRational)>,
        /// The step's own class, `None` where it is unread.
        kind: Option<BivectorKind>,
        /// The step's modes, by the one reader of modes.
        modes: ModesRead,
        /// The class of the transport composed through this step, `None`
        /// where it is unread.
        after: Option<BivectorKind>,
    },
}

const NO_KIND: u8 = 0xff;

fn put_kind(out: &mut Vec<u8>, k: Option<BivectorKind>) {
    out.push(match k {
        Some(k) => k.as_u8(),
        None => NO_KIND,
    });
}

fn take_kind(b: &[u8], p: &mut usize) -> Option<Option<BivectorKind>> {
    match take_u8(b, p)? {
        NO_KIND => Some(None),
        k => Some(Some(BivectorKind::from_u8(k)?)),
    }
}

fn take_u8(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

fn take_u16(b: &[u8], p: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
    *p += 2;
    Some(v)
}

fn take_u32(b: &[u8], p: &mut usize) -> Option<usize> {
    let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(v as usize)
}

/// A count the body cannot hold at `least` bytes an entry is refused before
/// anything is reserved for it.
fn holds(b: &[u8], p: usize, m: usize, least: usize) -> bool {
    match (b.len().checked_sub(p), m.checked_mul(least)) {
        (Some(rest), Some(need)) => rest >= need,
        _ => false,
    }
}

fn put_int(out: &mut Vec<u8>, v: &BigInt) {
    let bytes = v.to_signed_bytes_le();
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(&bytes);
}

fn take_int(b: &[u8], p: &mut usize) -> Option<BigInt> {
    let len = take_u32(b, p)?;
    let v = BigInt::from_signed_bytes_le(b.get(*p..p.checked_add(len)?)?);
    *p += len;
    Some(v)
}

fn put_modes(out: &mut Vec<u8>, m: &ModesRead) {
    match m {
        ModesRead::Split(ms) => {
            out.push(0);
            out.extend_from_slice(&(ms.len() as u32).to_le_bytes());
            for &(kind, planes, order) in ms {
                out.push(kind.as_u8());
                out.extend_from_slice(&(planes as u32).to_le_bytes());
                match order {
                    Some(l) => {
                        out.push(1);
                        out.extend_from_slice(&l.to_le_bytes());
                    }
                    None => out.push(0),
                }
            }
        }
        ModesRead::Undecomposed(Undecomposed::Irrational(d)) => {
            out.push(1);
            out.extend_from_slice(&(*d as u32).to_le_bytes());
        }
        ModesRead::Undecomposed(Undecomposed::Defective) => out.push(2),
        ModesRead::Unread => out.push(3),
        ModesRead::Undecomposed(Undecomposed::Unbounded(d)) => {
            out.push(4);
            out.extend_from_slice(&(*d as u32).to_le_bytes());
        }
    }
}

fn take_modes(b: &[u8], p: &mut usize) -> Option<ModesRead> {
    match take_u8(b, p)? {
        0 => {
            let m = take_u32(b, p)?;
            if !holds(b, *p, m, 6) {
                return None;
            }
            let mut ms = Vec::with_capacity(m);
            for _ in 0..m {
                let kind = BivectorKind::from_u8(take_u8(b, p)?)?;
                let planes = take_u32(b, p)?;
                let order = match take_u8(b, p)? {
                    0 => None,
                    1 => Some(take_u32(b, p)? as u32),
                    _ => return None,
                };
                ms.push((kind, planes, order));
            }
            Some(ModesRead::Split(ms))
        }
        1 => Some(ModesRead::Undecomposed(Undecomposed::Irrational(take_u32(b, p)?))),
        2 => Some(ModesRead::Undecomposed(Undecomposed::Defective)),
        3 => Some(ModesRead::Unread),
        4 => Some(ModesRead::Undecomposed(Undecomposed::Unbounded(take_u32(b, p)?))),
        _ => None,
    }
}

impl CircuitStep {
    fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.face.to_le_bytes());
        out.push(u8::from(self.forward) | (u8::from(self.refracted) << 1));
        match &self.carried {
            Carried::Unread => out.push(0),
            Carried::Cone => out.push(1),
            Carried::Read { ray, kind, modes, after } => {
                out.push(2);
                out.extend_from_slice(&(ray.len() as u32).to_le_bytes());
                for (blade, c) in ray {
                    out.extend_from_slice(&blade.to_le_bytes());
                    put_int(out, c.numer());
                    put_int(out, c.denom());
                }
                put_kind(out, *kind);
                put_modes(out, modes);
                put_kind(out, *after);
            }
        }
    }

    fn take(b: &[u8], p: &mut usize) -> Option<CircuitStep> {
        let face = take_u16(b, p)?;
        let flags = take_u8(b, p)?;
        if flags > 3 {
            return None;
        }
        let carried = match take_u8(b, p)? {
            0 => Carried::Unread,
            1 => Carried::Cone,
            2 => {
                let m = take_u32(b, p)?;
                if !holds(b, *p, m, 10) {
                    return None;
                }
                let mut ray = Vec::with_capacity(m);
                for _ in 0..m {
                    let blade = take_u16(b, p)?;
                    let (numer, denom) = (take_int(b, p)?, take_int(b, p)?);
                    if num_traits::Zero::is_zero(&denom) {
                        return None;
                    }
                    ray.push((blade, BigRational::new(numer, denom)));
                }
                let kind = take_kind(b, p)?;
                let modes = take_modes(b, p)?;
                let after = take_kind(b, p)?;
                Carried::Read { ray, kind, modes, after }
            }
            _ => return None,
        };
        Some(CircuitStep { face, forward: flags & 1 == 1, refracted: flags & 2 == 2, carried })
    }
}

impl Circuits {
    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.circuits.len() as u32).to_le_bytes());
        for c in &self.circuits {
            out.extend_from_slice(&c.chord.to_le_bytes());
            out.extend_from_slice(&(c.steps.len() as u32).to_le_bytes());
            for s in &c.steps {
                s.put(out);
            }
        }
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Circuits> {
        let m = take_u32(b, p)?;
        // A circuit is at least its chord and its step count.
        if !holds(b, *p, m, 6) {
            return None;
        }
        let mut circuits = Vec::with_capacity(m);
        for _ in 0..m {
            let chord = take_u16(b, p)?;
            let k = take_u32(b, p)?;
            // A step is at least its face, its flags and its arm.
            if !holds(b, *p, k, 4) {
                return None;
            }
            let mut steps = Vec::with_capacity(k);
            for _ in 0..k {
                steps.push(CircuitStep::take(b, p)?);
            }
            circuits.push(Circuit { chord, steps });
        }
        Some(Circuits { circuits })
    }
}
