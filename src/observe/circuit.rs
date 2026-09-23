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

/// EVERY KEPT CIRCUIT OF THE LAST PERCEPT, in the order the traversal read them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Circuits {
    pub circuits: Vec<Circuit>,
}

/// ONE CIRCUIT: the chord that closes it and its steps in boundary order,
/// and the reuse ladder's test where one ran (CT1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Circuit {
    pub chord: u16,
    pub steps: Vec<CircuitStep>,
    /// `None` where the circuit read a class of its own that is not trivial,
    /// or rides a held coset: nothing triggered the ladder (79.3).
    pub test: Option<CircuitTest>,
}

/// CT1, 79.3 — ONE ARM OF THE REUSE LADDER, chosen by the refusal's own type:
/// a typed conditional, each arm one keyed read, never a search.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestArm {
    /// Any refusal: the other chart pairs of the face the circuit lost.
    OtherPairs,
    /// `NotRigid`: the tether carrier `g = (θ_bθ_a⁻¹)·V` (BQ5a).
    Tether,
    /// `Unrefracted`, half-open: the coset edge (78.6(1)).
    Coset,
    /// `Unrefracted`, static: `V` with the refraction static (78.6(2)).
    Static,
    /// Read and trivial: the other circuits crossing the circuit's faces.
    OtherCircuits,
    /// `Cone`: nothing, permanent at this grade by name.
    Cone,
    /// CT2 — every reuse arm empty: a tether constructed between the lost
    /// face's two bodies from the cell's nuclei.
    ConstructedTether,
    /// CT2 — the same tether carried by a constructor the cell holds, `Ω v Ω̃`:
    /// a fibre landing the pair on its image chart pair.
    ConstructedFibre,
}

impl TestArm {
    /// Whether the arm CONSTRUCTS what it reads (CT2), never reuses it.
    pub fn constructs(self) -> bool {
        matches!(self, TestArm::ConstructedTether | TestArm::ConstructedFibre)
    }
}

/// CT2, 79.3 — ONE CONSTRUCTED ROUTE, AS OFFERED: a candidate of the thought,
/// volatile, ratified only by a court. Its constructor and the face it lands
/// on, the verdict of T1 on the two routes with the holonomy of the loop they
/// close, and whether that orbit closes or opens, by mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offer {
    pub arm: TestArm,
    /// The face the constructed route crosses in the circuit's place.
    pub face: u16,
    /// The constructed carrier's ray: the tether, or the constructor that
    /// carried it.
    pub constructor: Vec<(u16, BigRational)>,
    /// T1: the two routes agree — the completed loop is the identity ray.
    pub flat: bool,
    pub holonomy: Vec<(u16, BigRational)>,
    pub modes: ModesRead,
}

/// WHAT ONE ARM READ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmRead {
    /// No such route: nothing for the arm to reuse.
    Empty,
    /// Routes carried by a reused fibre or carrier where the circuit's own
    /// reading refused: how many.
    Reads(u32),
    /// Other routes read against the circuit's own: agreeing, disagreeing.
    Routes { agree: u32, disagree: u32 },
    /// Refused by name, permanent at this grade.
    Permanent,
}

/// HOW THE ARMS DISAGREE (79.3), in 78.9's arms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircuitClass {
    /// Every readable route agrees: the corollary holds here.
    Flat,
    /// Two routes disagree: the enclosed cell's holonomy is certified content.
    Curved,
    /// Only a reused fibre or carrier reads it: cover-relative.
    C,
    /// Only another mind's charts read it (CV4).
    D,
    /// Only a constructed fibre reads it (CT2).
    Candidate,
    /// Every arm refuses, each by name: a wall.
    P,
}

impl CircuitClass {
    pub const ALL: [CircuitClass; 6] =
        [CircuitClass::Flat, CircuitClass::Curved, CircuitClass::C, CircuitClass::D, CircuitClass::Candidate, CircuitClass::P];

    pub fn label(self) -> &'static str {
        match self {
            CircuitClass::Flat => "flat",
            CircuitClass::Curved => "curved",
            CircuitClass::C => "C",
            CircuitClass::D => "D",
            CircuitClass::Candidate => "candidate",
            CircuitClass::P => "P",
        }
    }
}

impl TestArm {
    pub const ALL: [TestArm; 8] = [
        TestArm::OtherPairs,
        TestArm::Tether,
        TestArm::Coset,
        TestArm::Static,
        TestArm::OtherCircuits,
        TestArm::Cone,
        TestArm::ConstructedTether,
        TestArm::ConstructedFibre,
    ];

    pub fn label(self) -> &'static str {
        match self {
            TestArm::OtherPairs => "other chart pairs",
            TestArm::Tether => "tether carrier",
            TestArm::Coset => "coset edge",
            TestArm::Static => "static tether",
            TestArm::OtherCircuits => "other circuits",
            TestArm::Cone => "cone",
            TestArm::ConstructedTether => "constructed tether",
            TestArm::ConstructedFibre => "constructed fibre",
        }
    }
}

/// CT1 — THE LADDER'S RECORD FOR ONE CIRCUIT: every arm that ran and what it
/// read, and the class their disagreement gives. No arm is the route
/// (checklist (r)).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitTest {
    pub arms: Vec<(TestArm, ArmRead)>,
    pub class: CircuitClass,
    /// CT2 — the constructed routes offered, one per construction read.
    pub offers: Vec<Offer>,
}

impl CircuitTest {
    /// THE CLASS BY HOW THE ARMS DISAGREE (79.3): two REUSED routes
    /// disagreeing are curved; a trivial circuit whose readable routes agree
    /// is flat; a refused one that a reused fibre or carrier reads is C; one
    /// only a CONSTRUCTED route reads is a CANDIDATE (CT2) — flat or curved
    /// by T1 in its offer, C once a court ratifies; one every arm refuses is
    /// P. D is read across the mesh (CV4), never by this ladder.
    pub fn classify(arms: Vec<(TestArm, ArmRead)>, trivial: bool, offers: Vec<Offer>) -> CircuitTest {
        let reused = |a: &TestArm| !a.constructs();
        let disagree =
            arms.iter().any(|(a, r)| reused(a) && matches!(r, ArmRead::Routes { disagree, .. } if *disagree > 0));
        let reads = arms.iter().any(|(a, r)| reused(a) && matches!(r, ArmRead::Reads(n) if *n > 0));
        let constructed = arms
            .iter()
            .any(|(a, r)| a.constructs() && matches!(r, ArmRead::Routes { agree, disagree } if agree + disagree > 0));
        let class = match (disagree, trivial, reads, constructed) {
            (true, _, _, _) => CircuitClass::Curved,
            (false, true, _, _) => CircuitClass::Flat,
            (false, false, true, _) => CircuitClass::C,
            (false, false, false, true) => CircuitClass::Candidate,
            (false, false, false, false) => CircuitClass::P,
        };
        CircuitTest { arms, class, offers }
    }

    fn put(&self, out: &mut Vec<u8>) {
        out.push(CircuitClass::ALL.iter().position(|&c| c == self.class).map_or(0xff, |i| i as u8));
        out.extend_from_slice(&(self.arms.len() as u32).to_le_bytes());
        for (arm, read) in &self.arms {
            out.push(TestArm::ALL.iter().position(|a| a == arm).map_or(0xff, |i| i as u8));
            match read {
                ArmRead::Empty => out.push(0),
                ArmRead::Reads(n) => {
                    out.push(1);
                    out.extend_from_slice(&n.to_le_bytes());
                }
                ArmRead::Routes { agree, disagree } => {
                    out.push(2);
                    out.extend_from_slice(&agree.to_le_bytes());
                    out.extend_from_slice(&disagree.to_le_bytes());
                }
                ArmRead::Permanent => out.push(3),
            }
        }
        out.extend_from_slice(&(self.offers.len() as u32).to_le_bytes());
        for o in &self.offers {
            out.push(TestArm::ALL.iter().position(|a| *a == o.arm).map_or(0xff, |i| i as u8));
            out.extend_from_slice(&o.face.to_le_bytes());
            put_ray(out, &o.constructor);
            out.push(u8::from(o.flat));
            put_ray(out, &o.holonomy);
            put_modes(out, &o.modes);
        }
    }

    fn take(b: &[u8], p: &mut usize) -> Option<CircuitTest> {
        let class = *CircuitClass::ALL.get(usize::from(take_u8(b, p)?))?;
        let m = take_u32(b, p)?;
        if !holds(b, *p, m, 2) {
            return None;
        }
        let mut arms = Vec::with_capacity(m);
        for _ in 0..m {
            let arm = *TestArm::ALL.get(usize::from(take_u8(b, p)?))?;
            let read = match take_u8(b, p)? {
                0 => ArmRead::Empty,
                1 => ArmRead::Reads(take_u32(b, p)? as u32),
                2 => ArmRead::Routes { agree: take_u32(b, p)? as u32, disagree: take_u32(b, p)? as u32 },
                3 => ArmRead::Permanent,
                _ => return None,
            };
            arms.push((arm, read));
        }
        let n = take_u32(b, p)?;
        if !holds(b, *p, n, 12) {
            return None;
        }
        let mut offers = Vec::with_capacity(n);
        for _ in 0..n {
            let arm = *TestArm::ALL.get(usize::from(take_u8(b, p)?))?;
            let face = take_u16(b, p)?;
            let constructor = take_ray(b, p)?;
            let flat = match take_u8(b, p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            let holonomy = take_ray(b, p)?;
            let modes = take_modes(b, p)?;
            offers.push(Offer { arm, face, constructor, flat, holonomy, modes });
        }
        Some(CircuitTest { arms, class, offers })
    }
}

/// A ray in its canonical coordinates, each blade and its exact coefficient.
fn put_ray(out: &mut Vec<u8>, ray: &[(u16, BigRational)]) {
    out.extend_from_slice(&(ray.len() as u32).to_le_bytes());
    for (blade, c) in ray {
        out.extend_from_slice(&blade.to_le_bytes());
        put_int(out, c.numer());
        put_int(out, c.denom());
    }
}

fn take_ray(b: &[u8], p: &mut usize) -> Option<Vec<(u16, BigRational)>> {
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
    Some(ray)
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

pub(super) fn take_u8(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

pub(super) fn take_u16(b: &[u8], p: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
    *p += 2;
    Some(v)
}

pub(super) fn take_u32(b: &[u8], p: &mut usize) -> Option<usize> {
    let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(v as usize)
}

/// A count the body cannot hold at `least` bytes an entry is refused before
/// anything is reserved for it.
pub(super) fn holds(b: &[u8], p: usize, m: usize, least: usize) -> bool {
    match (b.len().checked_sub(p), m.checked_mul(least)) {
        (Some(rest), Some(need)) => rest >= need,
        _ => false,
    }
}

pub(super) fn put_int(out: &mut Vec<u8>, v: &BigInt) {
    let bytes = v.to_signed_bytes_le();
    out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(&bytes);
}

pub(super) fn take_int(b: &[u8], p: &mut usize) -> Option<BigInt> {
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
            match &c.test {
                Some(t) => {
                    out.push(1);
                    t.put(out);
                }
                None => out.push(0),
            }
        }
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Circuits> {
        let m = take_u32(b, p)?;
        // A circuit is at least its chord, its step count and its test flag.
        if !holds(b, *p, m, 7) {
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
            let test = match take_u8(b, p)? {
                0 => None,
                1 => Some(CircuitTest::take(b, p)?),
                _ => return None,
            };
            circuits.push(Circuit { chord, steps, test });
        }
        Some(Circuits { circuits })
    }
}
