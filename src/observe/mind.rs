// MU-CLIENT §5.3, §8 — WHAT A MIND'S HEARING CAME TO, AND WHAT A COURT
// ATTESTED OF IT: two typed readings served straight from the running
// binaries, never read off a log.
//
//! [`Mind`] — ONE MIND'S CONVERSATION AND WHAT IT BUILT: the things other
//! minds said that it heard, the thoughts understood both ways, each
//! hearing's cell as the field priced it (closed, held, open, unread), the demands
//! open cells raised and what the demand taker made of every demand, the
//! HOLDs it sent and the agreed language it spoke, the deviation of its
//! closed and of its open hearings — exact, lane by lane, never a count per
//! lane — and what its ear and its conversation hold now. Written where the
//! mind acts (`note_mind`), by the carrier and the field on their own
//! circuits; read only at a question, on the mind's post, so the observer
//! never waits on cognition and nothing in cognition reads it back.
//!
//! [`Agreements`] — THE COUNTERPARTY'S WITNESS: per speaker → hearer, the
//! agreed language a court attested, how much of it heard whole, and the
//! deviation it carried, summed exact per lane. Written where the court
//! verifies the hearer's signed HOLD (`note_agreement`).

use crate::network::nerve::Keyed;
use num_rational::BigRational as Q;
use std::sync::{LazyLock, Mutex};

/// WHAT A HEARING'S CELL CLOSED AS, in the order the counts are kept: every
/// echo returned; some precessing, none escaping (held); some escaping
/// (open); unread.
pub const PRICED: [&str; 4] = ["closes", "held", "open", "unread"];

/// WHAT THE DEMAND TAKER MADE OF A DEMAND, in the order the counts are kept:
/// its residue did not construct; a turn adjoined as the mind's letter; a
/// motion with no turn stood as a motion letter; constructed with no chart
/// to stand on; a residue it already holds a letter of.
pub const CONSTRUCTED: [&str; 5] = ["refused", "letter", "motion letter", "no chart", "held"];

/// WHAT ITS FIBRES DID (78.74, MU-CLIENT §4.5), each flex read off the
/// fibre's own split: RIGID, TORSION (a twist, no work), FIELD (charge
/// between its levels, the stretch), LIFT (its pivots moved, the reshape;
/// also a flex on thoughts with no pivots); the flexes a hole PINNED; the
/// fibres the field CARRIED (frozen-in flux); the tethers the field released
/// and the mind LET GO (a return vented, or a stretch that broke); and the
/// shortcuts FORMED through a shared pivot, and those the fibration placed
/// on the far pivot's side of the switch, which form nothing — in the order
/// the counts are kept.
pub const FLEX: [&str; 9] = ["rigid", "torsion", "field", "lift", "pinned", "carried", "let go", "formed", "far side"];

/// WHAT TWO BODIES WHOSE SPHERES MET IN AN ADMITTED SCENE WERE (78.52 Row
/// 2), in the order the counts are kept.
pub const BODIES: [&str; 3] = ["fused", "precessing", "contact"];

/// One slot of [`CONSTRUCTED`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Constructed {
    Refused,
    Letter,
    MotionLetter,
    NoChart,
    Held,
}

fn zero4() -> [Q; 4] {
    std::array::from_fn(|_| Q::from_integer(0.into()))
}

fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn put_lanes(out: &mut Vec<u8>, lanes: &[Q; 4]) {
    for q in lanes {
        crate::transducer::alphabet::put_q(out, q);
    }
}

fn take_lanes(b: &[u8], p: &mut usize) -> Option<[Q; 4]> {
    let mut lane = || crate::transducer::alphabet::take_q(b, p);
    Some([lane()?, lane()?, lane()?, lane()?])
}

/// The four lanes as the console writes them, `S:G:T:H`.
fn lanes_line(lanes: &[Q; 4]) -> String {
    lanes.iter().map(|q| q.to_string()).collect::<Vec<_>>().join(":")
}

fn short(b: &[u8]) -> String {
    b.iter().take(4).map(|x| format!("{x:02x}")).collect()
}

/// ONE MIND'S CONVERSATION AND WHAT IT BUILT (module doc).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mind {
    /// The mind's own K at its last tick.
    pub k: u64,
    /// Things other minds said, heard.
    pub heard: u64,
    /// Thoughts this mind heard whose ACK read as its own reading, every word
    /// understood — handed to the field to price as the hearer.
    pub understood: u64,
    /// Thoughts this mind said whose HOLD read as its ACK — handed to the
    /// field to price as the speaker.
    pub returned: u64,
    /// Each hearing's cell, by [`PRICED`].
    pub priced: [u64; 4],
    /// Demands an open hearing's cell raised.
    pub demanded: u64,
    /// Every demand the taker took, by [`CONSTRUCTED`].
    pub constructed: [u64; 5],
    /// Its fibres' flexes, by [`FLEX`].
    pub flex: [u64; 9],
    /// Bodies whose spheres met in an admitted scene, by [`BODIES`].
    pub bodies: [u64; 3],
    /// Spin coupled to orbit: each lock `p:q` read, with how often, in
    /// ratio order; and the drifts held as torsion.
    pub locks: Vec<(u64, u64, u64)>,
    pub drifts: u64,
    /// Perception's cells: rings held as shears (precessing), and rings
    /// escaping, docketed as demands.
    pub shears: u64,
    pub escapes: u64,
    /// HOLDs this mind sent.
    pub held: u64,
    /// Agreed language this mind spoke to its court.
    pub agreed: u64,
    /// The deviation of the hearings whose cell closed, and of those whose
    /// cell did not (open or unread), each summed exact lane by lane.
    pub closed: [Q; 4],
    pub open: [Q; 4],
    /// What the ear and the conversation hold now: the ear's generators,
    /// the thoughts awaiting an ACK, the things said awaiting a HOLD, the
    /// letters awaiting a union.
    pub ear: u64,
    pub thoughts: u64,
    pub unions: u64,
    pub waiting: u64,
}

impl Default for Mind {
    fn default() -> Mind {
        Mind {
            k: 0,
            heard: 0,
            understood: 0,
            returned: 0,
            priced: [0; 4],
            demanded: 0,
            constructed: [0; 5],
            flex: [0; 9],
            bodies: [0; 3],
            locks: Vec::new(),
            drifts: 0,
            shears: 0,
            escapes: 0,
            held: 0,
            agreed: 0,
            closed: zero4(),
            open: zero4(),
            ear: 0,
            thoughts: 0,
            unions: 0,
            waiting: 0,
        }
    }
}

impl Mind {
    /// One lock `p:q` read.
    pub fn lock(&mut self, p: u64, q: u64) {
        match self.locks.binary_search_by_key(&(p, q), |&(a, b, _)| (a, b)) {
            Ok(at) => self.locks[at].2 += 1,
            Err(at) => self.locks.insert(at, (p, q, 1)),
        }
    }

    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        for v in [self.k, self.heard, self.understood, self.returned] {
            put_u64(out, v);
        }
        for v in self.priced {
            put_u64(out, v);
        }
        put_u64(out, self.demanded);
        for v in self.constructed.iter().chain(&self.flex).chain(&self.bodies) {
            put_u64(out, *v);
        }
        for v in [self.held, self.agreed] {
            put_u64(out, v);
        }
        put_lanes(out, &self.closed);
        put_lanes(out, &self.open);
        for v in [self.ear, self.thoughts, self.unions, self.waiting, self.drifts, self.shears, self.escapes] {
            put_u64(out, v);
        }
        crate::transducer::address::put_u32_count(out, self.locks.len())?;
        for &(p, q, n) in &self.locks {
            for v in [p, q, n] {
                put_u64(out, v);
            }
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Mind> {
        let [k, heard, understood, returned] = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let priced = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let demanded = take_u64(b, p)?;
        let constructed = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let mut flex = [0u64; 9];
        for v in &mut flex {
            *v = take_u64(b, p)?;
        }
        let bodies = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let [held, agreed] = [take_u64(b, p)?, take_u64(b, p)?];
        let closed = take_lanes(b, p)?;
        let open = take_lanes(b, p)?;
        let [ear, thoughts, unions, waiting] = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let [drifts, shears, escapes] = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let n = usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
        *p += 4;
        // Each lock is three counts.
        if n > b.len().checked_sub(*p)? / 24 {
            return None;
        }
        let mut locks = Vec::with_capacity(n);
        for _ in 0..n {
            locks.push((take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?));
        }
        Some(Mind {
            k,
            heard,
            understood,
            returned,
            priced,
            demanded,
            constructed,
            flex,
            bodies,
            locks,
            drifts,
            shears,
            escapes,
            held,
            agreed,
            closed,
            open,
            ear,
            thoughts,
            unions,
            waiting,
        })
    }

    /// The reading as the console prints it.
    pub fn lines(&self) -> Vec<String> {
        let priced: Vec<String> = PRICED.iter().zip(&self.priced).map(|(n, v)| format!("{n} {v}")).collect();
        let constructed: Vec<String> = CONSTRUCTED.iter().zip(&self.constructed).map(|(n, v)| format!("{n} {v}")).collect();
        vec![
            format!(
                "mind K {} · heard {} · understood {} (hearer) {} (speaker) · held {} · agreed {}",
                self.k, self.heard, self.understood, self.returned, self.held, self.agreed
            ),
            format!("  cells {} · demanded {}", priced.join(" "), self.demanded),
            format!("  constructed {}", constructed.join(" ")),
            format!(
                "  fibres {} · bodies met {}",
                FLEX.iter().zip(&self.flex).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                BODIES.iter().zip(&self.bodies).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" ")
            ),
            format!(
                "  torsion: shears {} · escapes {} · drifts {} · locks {}",
                self.shears,
                self.escapes,
                self.drifts,
                match self.locks.is_empty() {
                    true => "none".to_string(),
                    false => self.locks.iter().map(|(p, q, n)| format!("{p}:{q}×{n}")).collect::<Vec<_>>().join(" "),
                }
            ),
            format!("  deviation closed {} · open {}", lanes_line(&self.closed), lanes_line(&self.open)),
            format!("  holds ear {} · thoughts {} · unions {} · waiting {}", self.ear, self.thoughts, self.unions, self.waiting),
        ]
    }
}

static MIND: LazyLock<Mutex<Mind>> = LazyLock::new(|| Mutex::new(Mind::default()));

/// WRITE what the mind just did into its reading, where it did it.
pub fn note_mind(f: impl FnOnce(&mut Mind)) {
    if let Ok(mut m) = MIND.lock() {
        f(&mut m);
    }
}

/// The mind's reading as it stands.
pub fn mind() -> Mind {
    match MIND.lock() {
        Ok(m) => m.clone(),
        Err(_) => Mind::default(),
    }
}

/// ONE SPEAKER → HEARER PAIR AT A COURT: the agreed language attested, how
/// much of it heard whole (every lane zero), and the deviation summed exact
/// per lane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agreed {
    pub speaker: [u8; 32],
    pub hearer: [u8; 32],
    pub agreed: u64,
    pub whole: u64,
    pub deviation: [Q; 4],
}

/// THE AGREEMENTS A COURT ATTESTED since boot, per pair in key order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Agreements {
    pub pairs: Vec<Agreed>,
}

static AGREEMENTS: LazyLock<Mutex<Keyed<Agreed>>> = LazyLock::new(|| Mutex::new(Keyed::new()));

/// ONE ATTESTATION, where the court verified the hearer's signed HOLD.
pub fn note_agreement(speaker: &[u8; 32], hearer: &[u8; 32], deviation: &[Q; 4]) {
    let Ok(mut a) = AGREEMENTS.lock() else { return };
    let mut key = speaker.to_vec();
    key.extend_from_slice(hearer);
    if a.get(&key).is_none() {
        a.insert(&key, Agreed { speaker: *speaker, hearer: *hearer, agreed: 0, whole: 0, deviation: zero4() });
    }
    let Some(pair) = a.get_mut(&key) else { return };
    pair.agreed += 1;
    if deviation.iter().all(|d| *d == Q::from_integer(0.into())) {
        pair.whole += 1;
    }
    for (sum, d) in pair.deviation.iter_mut().zip(deviation) {
        *sum += d;
    }
}

/// The agreements this court attested.
pub fn agreements() -> Agreements {
    match AGREEMENTS.lock() {
        Ok(a) => Agreements { pairs: a.scoped().map(|(_, p)| p.clone()).collect() },
        Err(_) => Agreements::default(),
    }
}

impl Agreements {
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        crate::transducer::address::put_u32_count(out, self.pairs.len())?;
        for p in &self.pairs {
            out.extend_from_slice(&p.speaker);
            out.extend_from_slice(&p.hearer);
            put_u64(out, p.agreed);
            put_u64(out, p.whole);
            put_lanes(out, &p.deviation);
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Agreements> {
        let n = usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
        *p += 4;
        // Each pair is two keys, two counts and four lanes of two lengths at
        // least.
        if n > b.len().checked_sub(*p)? / (64 + 16 + 8) {
            return None;
        }
        let mut pairs = Vec::with_capacity(n);
        for _ in 0..n {
            let speaker: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            let hearer: [u8; 32] = b.get(*p + 32..*p + 64)?.try_into().ok()?;
            *p += 64;
            let agreed = take_u64(b, p)?;
            let whole = take_u64(b, p)?;
            let deviation = take_lanes(b, p)?;
            pairs.push(Agreed { speaker, hearer, agreed, whole, deviation });
        }
        Some(Agreements { pairs })
    }

    /// The reading as the console prints it: one line per pair.
    pub fn lines(&self) -> Vec<String> {
        let total: u64 = self.pairs.iter().map(|p| p.agreed).sum();
        let whole: u64 = self.pairs.iter().map(|p| p.whole).sum();
        let mut out = vec![format!("agreements {total} over {} pairs · heard whole {whole}", self.pairs.len())];
        for p in &self.pairs {
            out.push(format!(
                "  {}… → {}… agreed {} whole {} deviation {}",
                short(&p.speaker),
                short(&p.hearer),
                p.agreed,
                p.whole,
                lanes_line(&p.deviation)
            ));
        }
        out
    }
}
