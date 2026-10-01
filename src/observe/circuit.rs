// 78.12(4), PL1 — THE CIRCUIT'S TRAJECTORY, TRANSITION BY TRANSITION: the bricks, not the
// house. Every kept circuit of the last percept read at each member transition —
// the face crossed and which way, whether the face refracts, the transition's own
// ray and its class and modes, and the class of the transport composed so
// far — served to the console on demand and never printed per epoch.
//
//! The census line `modes [none]` on thirty orbits says the house has no
//! walls and nothing about the bricks. This reading is the bricks: a
//! trivial holonomy reads as the sequence of transitions whose product
//! telescopes, and the first transition whose face refracts is named where one
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

/// ONE CIRCUIT: the chord that closes it and its transitions in boundary order,
/// and the reuse ladder's test where one ran (CT1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Circuit {
    /// The chord that closes it, by its overlap (AT1).
    pub chord: crate::network::nerve::coord::Overlap,
    pub transitions: Vec<CircuitTransition>,
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
    /// `Unrefracted`, open-180°: the coset edge (78.6(1)).
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
    /// The overlap the constructed route crosses in the circuit's place.
    pub face: crate::network::nerve::coord::Overlap,
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
    /// Counts of routes, so `u64` (STD1 3b's rule, the provenance ruling).
    Reads(u64),
    /// Other routes read against the circuit's own: agreeing, disagreeing.
    Routes { agree: u64, disagree: u64 },
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
            .any(|(a, r)| a.constructs() && matches!(r, ArmRead::Routes { agree, disagree } if *agree > 0 || *disagree > 0));
        let class = match (disagree, trivial, reads, constructed) {
            (true, _, _, _) => CircuitClass::Curved,
            (false, true, _, _) => CircuitClass::Flat,
            (false, false, true, _) => CircuitClass::C,
            (false, false, false, true) => CircuitClass::Candidate,
            (false, false, false, false) => CircuitClass::P,
        };
        CircuitTest { arms, class, offers }
    }

    #[must_use]
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        #[expect(
            clippy::cast_possible_truncation,
            reason = "STD1 C1 — a constant table: a position in CircuitClass::ALL, 6 entries"
        )]
        out.push(CircuitClass::ALL.iter().position(|&c| c == self.class).map_or(0xff, |i| i as u8));
        crate::transducer::address::put_u32_count(out, self.arms.len())?;
        for (arm, read) in &self.arms {
            #[expect(
                clippy::cast_possible_truncation,
                reason = "STD1 C1 — a constant table: a position in TestArm::ALL, 8 entries"
            )]
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
        crate::transducer::address::put_u32_count(out, self.offers.len())?;
        for o in &self.offers {
            #[expect(
                clippy::cast_possible_truncation,
                reason = "STD1 C1 — a constant table: a position in TestArm::ALL, 8 entries"
            )]
            out.push(TestArm::ALL.iter().position(|a| *a == o.arm).map_or(0xff, |i| i as u8));
            put_overlap(out, &o.face);
            put_ray(out, &o.constructor)?;
            out.push(u8::from(o.flat));
            put_ray(out, &o.holonomy)?;
            put_modes(out, &o.modes)?;
        }
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<CircuitTest> {
        let class = *CircuitClass::ALL.get(usize::from(take_u8(b, p)?))?;
        let m = take_len(b, p)?;
        if !holds(b, *p, m, 2) {
            return None;
        }
        let mut arms = Vec::with_capacity(m);
        for _ in 0..m {
            let arm = *TestArm::ALL.get(usize::from(take_u8(b, p)?))?;
            let read = match take_u8(b, p)? {
                0 => ArmRead::Empty,
                1 => ArmRead::Reads(take_u64(b, p)?),
                2 => ArmRead::Routes { agree: take_u64(b, p)?, disagree: take_u64(b, p)? },
                3 => ArmRead::Permanent,
                _ => return None,
            };
            arms.push((arm, read));
        }
        let n = take_len(b, p)?;
        if !holds(b, *p, n, 12) {
            return None;
        }
        let mut offers = Vec::with_capacity(n);
        for _ in 0..n {
            let arm = *TestArm::ALL.get(usize::from(take_u8(b, p)?))?;
            let face = take_overlap(b, p)?;
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
#[must_use]
fn put_ray(out: &mut Vec<u8>, ray: &[(u16, BigRational)]) -> Option<()> {
    crate::transducer::address::put_u32_count(out, ray.len())?;
    for (blade, c) in ray {
        out.extend_from_slice(&blade.to_le_bytes());
        put_int(out, c.numer())?;
        put_int(out, c.denom())?;
    }
    Some(())
}

fn take_ray(b: &[u8], p: &mut usize) -> Option<Vec<(u16, BigRational)>> {
    let m = take_len(b, p)?;
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
    /// THE FIRST TRANSITION WHERE A REFRACTION ENTERS: the first face crossed that
    /// carries an interface. `None` where no transition refracts, which is a
    /// reading and not a refusal.
    pub fn first_refraction(&self) -> Option<usize> {
        self.transitions.iter().position(|s| s.refracted)
    }
}

/// ONE MEMBER TRANSITION.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitTransition {
    /// The overlap crossed, by its two charts (AT1).
    pub face: crate::network::nerve::coord::Overlap,
    /// Along the face's own orientation, or against it.
    pub forward: bool,
    /// The face carries an interface: two bodies in contact, a refraction.
    pub refracted: bool,
    pub carried: Carried,
}

/// WHAT ONE TRANSITION CARRIED, oriented the way the circuit runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Carried {
    /// The transition's transport was not read (rule 11: unread is not flat).
    Unread,
    /// The transition's transport inverts on no ray: it is on the light cone.
    Cone,
    Read {
        /// The transition's versor ray in its canonical coordinates: each non-zero
        /// blade and its exact coefficient, the first `+1` (A4).
        ray: Vec<(u16, BigRational)>,
        /// The transition's own class, `None` where it is unread.
        kind: Option<BivectorKind>,
        /// The transition's modes, by the one reader of modes.
        modes: ModesRead,
        /// The class of the transport composed through this transition, `None`
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

/// An overlap framed, and read back (AT1).
fn put_overlap(out: &mut Vec<u8>, o: &crate::network::nerve::coord::Overlap) {
    use crate::network::nerve::Coordinate;
    crate::transducer::address::put_key_bytes(out, o.encode().as_bytes());
}

fn take_overlap(b: &[u8], p: &mut usize) -> Option<crate::network::nerve::coord::Overlap> {
    use crate::network::nerve::Coordinate;
    crate::network::nerve::coord::Overlap::decode(&crate::transducer::address::take_key_bytes(b, p)?).ok()
}

pub(super) fn take_u16(b: &[u8], p: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
    *p += 2;
    Some(v)
}

pub(super) fn take_u32(b: &[u8], p: &mut usize) -> Option<u32> {
    let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    Some(v)
}

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

/// A `u32` read as a count or a length, exact in its reader's own width
/// (STD1 3c: the value is read at the width it was written at, never cast
/// back and forth).
pub(super) fn take_len(b: &[u8], p: &mut usize) -> Option<usize> {
    usize::try_from(take_u32(b, p)?).ok()
}

/// A count the body cannot hold at `least` bytes an entry is refused before
/// anything is reserved for it.
pub(super) fn holds(b: &[u8], p: usize, m: usize, least: usize) -> bool {
    match (b.len().checked_sub(p), m.checked_mul(least)) {
        (Some(rest), Some(need)) => rest >= need,
        _ => false,
    }
}

/// An exact integer behind its byte length; `None` where the length passes
/// four bytes (STD1 3c) — an integer is unbounded by construction.
#[must_use]
pub(super) fn put_int(out: &mut Vec<u8>, v: &BigInt) -> Option<()> {
    crate::transducer::address::put_bytes(out, &v.to_signed_bytes_le())
}

pub(super) fn take_int(b: &[u8], p: &mut usize) -> Option<BigInt> {
    let len = take_len(b, p)?;
    let v = BigInt::from_signed_bytes_le(b.get(*p..p.checked_add(len)?)?);
    *p += len;
    Some(v)
}

#[must_use]
fn put_modes(out: &mut Vec<u8>, m: &ModesRead) -> Option<()> {
    use crate::transducer::address::put_u32_count;
    match m {
        ModesRead::Split(ms) => {
            out.push(0);
            put_u32_count(out, ms.len())?;
            for &(kind, planes, order) in ms {
                out.push(kind.as_u8());
                put_u32_count(out, planes)?;
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
            put_u32_count(out, *d)?;
        }
        ModesRead::Undecomposed(Undecomposed::Defective) => out.push(2),
        ModesRead::Unread => out.push(3),
        ModesRead::Undecomposed(Undecomposed::Unbounded(d)) => {
            out.push(4);
            put_u32_count(out, *d)?;
        }
    }
    Some(())
}

fn take_modes(b: &[u8], p: &mut usize) -> Option<ModesRead> {
    match take_u8(b, p)? {
        0 => {
            let m = take_len(b, p)?;
            if !holds(b, *p, m, 6) {
                return None;
            }
            let mut ms = Vec::with_capacity(m);
            for _ in 0..m {
                let kind = BivectorKind::from_u8(take_u8(b, p)?)?;
                let planes = take_len(b, p)?;
                let order = match take_u8(b, p)? {
                    0 => None,
                    1 => Some(take_u32(b, p)?),
                    _ => return None,
                };
                ms.push((kind, planes, order));
            }
            Some(ModesRead::Split(ms))
        }
        1 => Some(ModesRead::Undecomposed(Undecomposed::Irrational(take_len(b, p)?))),
        2 => Some(ModesRead::Undecomposed(Undecomposed::Defective)),
        3 => Some(ModesRead::Unread),
        4 => Some(ModesRead::Undecomposed(Undecomposed::Unbounded(take_len(b, p)?))),
        _ => None,
    }
}

impl CircuitTransition {
    #[must_use]
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        put_overlap(out, &self.face);
        out.push(u8::from(self.forward) | (u8::from(self.refracted) << 1));
        match &self.carried {
            Carried::Unread => out.push(0),
            Carried::Cone => out.push(1),
            Carried::Read { ray, kind, modes, after } => {
                out.push(2);
                // The ray's one writer (law 12): this restated it.
                put_ray(out, ray)?;
                put_kind(out, *kind);
                put_modes(out, modes)?;
                put_kind(out, *after);
            }
        }
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<CircuitTransition> {
        let face = take_overlap(b, p)?;
        let flags = take_u8(b, p)?;
        if flags > 3 {
            return None;
        }
        let carried = match take_u8(b, p)? {
            0 => Carried::Unread,
            1 => Carried::Cone,
            2 => {
                let m = take_len(b, p)?;
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
        Some(CircuitTransition { face, forward: flags & 1 == 1, refracted: flags & 2 == 2, carried })
    }
}

impl Circuits {
    /// `None` where a count or a length passes its field (STD1 3c).
    #[must_use]
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        use crate::transducer::address::put_u32_count;
        put_u32_count(out, self.circuits.len())?;
        for c in &self.circuits {
            put_overlap(out, &c.chord);
            put_u32_count(out, c.transitions.len())?;
            for s in &c.transitions {
                s.put(out)?;
            }
            match &c.test {
                Some(t) => {
                    out.push(1);
                    t.put(out)?;
                }
                None => out.push(0),
            }
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Circuits> {
        let m = take_len(b, p)?;
        // A circuit is at least its chord, its transition count and its test flag.
        if !holds(b, *p, m, 7) {
            return None;
        }
        let mut circuits = Vec::with_capacity(m);
        for _ in 0..m {
            let chord = take_overlap(b, p)?;
            let k = take_len(b, p)?;
            // A transition is at least its face, its flags and its arm.
            if !holds(b, *p, k, 4) {
                return None;
            }
            let mut transitions = Vec::with_capacity(k);
            for _ in 0..k {
                transitions.push(CircuitTransition::take(b, p)?);
            }
            let test = match take_u8(b, p)? {
                0 => None,
                1 => Some(CircuitTest::take(b, p)?),
                _ => return None,
            };
            circuits.push(Circuit { chord, transitions, test });
        }
        Some(Circuits { circuits })
    }
}
