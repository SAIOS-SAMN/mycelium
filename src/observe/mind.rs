// MU-CLIENT §5.3, §8 — WHAT A MIND'S HEARING CAME TO, AND WHAT A COURT
// ATTESTED OF IT: two typed readings served straight from the running
// binaries, never read off a log.
//
//! [`Mind`] — ONE MIND'S CONVERSATION AND WHAT IT BUILT: the things other
//! minds said that it heard, the thoughts understood both ways, each
//! hearing's chamber as the field priced it (closed, held, open, unread), the
//! demands open chambers raised and what the demand taker made of every
//! demand, the HOLDs it sent and the agreed language it spoke, the deviation
//! of its closed and of its open hearings — exact, lane by lane, never a count
//! per lane — its vocabulary and what it remembers hearing, and what its
//! conversation's carrier holds now. Written where the
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
/// to stand on; a residue it already holds a letter of; COMPOSED whole of the
/// words it knows (MU-CLIENT §5.4, D4: nothing constructed); composed IN
/// PART, only what its words left constructed.
pub const CONSTRUCTED: [&str; 7] = ["refused", "letter", "motion letter", "no chart", "held", "composed", "in part"];

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

/// THE BREATH'S AXES (MU-CLIENT §4.3): the breaths that moved radially —
/// contraction or dilation — and the planes their commutators split into
/// by kind: pinches (elliptic, each twisted), null carries, boosts — in the
/// order the counts are kept.
pub const BREATH: [&str; 4] = ["radial", "pinched", "null", "boost"];

/// WHAT TWO BODIES WHOSE SPHERES MET IN AN ADMITTED SCENE WERE (78.52 Row
/// 2), in the order the counts are kept.
pub const BODIES: [&str; 3] = ["fused", "precessing", "contact"];

/// WHAT ITS TASKS' PERSPECTIVES SAID (MU-CLIENT §4.4): tasks seen; rules
/// known (every held-out pair re-derived), unknown (fitted, not re-derived),
/// of one perspective, unread; held-out pairs lifted into the mind; solves
/// sent; known by inversion; and undetermined by their perspectives (a
/// two-pair clause rule, sent for the court to judge) — in the order the
/// counts are kept.
pub const PERSPECTIVE: [&str; 9] = ["tasks", "known", "unknown", "one perspective", "unread", "lifted", "sent", "by inversion", "undetermined"];

/// WHAT A HELD-OUT PERSPECTIVE DISAGREED ABOUT (MU-CLIENT §4.4): its
/// siblings derived no rule; the rule spoke nothing on it; spoke with a
/// remainder; laid down another frame; or coloured the right cells
/// otherwise — and, where it put cells in the wrong places (78.167 A1, the
/// occupancy split by cause, each cause present counted): ERASED a body the
/// truth keeps that no vertex reached; did not CREATE cells the truth has
/// and the input has not; DISPLACED — left ground where the truth keeps a
/// body the speech did reach; did not CLEAR cells the truth has as ground —
/// in the order the counts are kept.
pub const DISAGREE: [&str; 9] = ["no rule", "unspoken", "remainder", "frame", "erased", "uncreated", "displaced", "uncleared", "hue"];

/// 78.167 A2 — THE DEMANDS A FAILED PERSPECTIVE'S RULE SPOKE NOTHING FOR, by
/// kind (`Demand::kind_byte`'s order).
pub const DEMANDED: [&str; 6] = ["rotor prime", "standing", "source", "creation", "deformation", "dislocation"];

/// 78.167 A3 — WHERE A BOOST MODE COMES FROM: its word scales (ρ ≠ 1, a
/// change of size), transverses (the special conformal half), or neither.
pub const BOOSTS: [&str; 3] = ["scaled", "transverse", "neither"];

/// 78.167 B — WHY A TEST'S SPEECH WAS HELD, NOT SENT AS A SOLVE: a body it
/// did not reach, or a creation, merge or reshape its form could not speak.
pub const WITHHELD: [&str; 2] = ["a body unreached", "occupancy unspoken"];

/// 78.167 C — A FAMILY'S ACTS READ OFF THEIR EFFECTS where its vertices did
/// not say it: stated as clauses, or refused by [`crate::transducer::family::acts::Unread`].
pub const ACTS: [&str; 8] = ["stated", "cited", "unfolded", "overwritten", "inseparable", "uncreated", "unsized", "spent"];

/// ITS QUESTIONS (MU-CLIENT §5.3): words it did not know and asked of their
/// residue; questions of others it answered by factoring; rings whose meaning
/// was DERIVED (from words it knew, or from factors it came to know); rings
/// whose meaning was GIVEN (asked whole, answered as an atom); given meanings
/// RE-DERIVED once the mind knew the words beside them (78.167 D1) — in the
/// order the counts are kept.
pub const ASKS: [&str; 5] = ["asked", "clarified", "derived", "given", "re-derived"];

/// WHAT BECAME OF THE ANSWERS IT HEARD (MU-CLIENT §5.3): taken (their factors
/// re-derived the residue exactly), inexact (they did not), or said under a
/// question's coordinates it has no question open at.
pub const ANSWERS: [&str; 3] = ["taken", "inexact", "no question"];

/// FROM WHERE ITS HEARERS READ WHAT IT SAID (MU-CLIENT §5.3), word by word at
/// the speaker: from a vantage of the hearer's OWN, distinct from the one
/// offered (a second derivation); from the OFFERED vantage (one derivation
/// seen twice); or from NONE (the hearer knows it only whole).
pub const VANTAGES: [&str; 3] = ["own", "offered", "none"];

/// THE LAWS ITS WORDS STAND ON (MU-CLIENT §5.4, 78.99): every mode of every
/// word its own perception newly taught it, by the law that mode returns on —
/// a turn (periodic), a null count (nilpotent), a rosette, a boost.
pub const LAWS: [&str; 4] = ["periodic", "nilpotent", "rosette", "boost"];

/// WHAT IT DID NOT REPEAT (compaction): a thing it had said, not said to its
/// peers again; a residue it had a question open on, not asked again.
pub const REPEATS: [&str; 2] = ["not said again", "not asked again"];

/// HOW EACH HEARD RING WAS TAKEN (MU-CLIENT §4.9), by its two readings —
/// SOUND (does it sound like something heard) and WORD (is it a word known):
/// familiar and known UNDERSTOOD; familiar and unknown ASKED; new and known
/// SHIFTED; new and unknown LISTENED (noise not yet signal); modes unread —
/// in the order the counts are kept.
pub const SEATS: [&str; 5] = ["understood", "asked", "shifted", "listened", "unheard"];

/// MUTUAL BALANCE at the speaker: the hearer's re-derivation resonating in
/// this mind, or not — in the order the counts are kept.
pub const MUTUAL: [&str; 2] = ["balanced", "unbalanced"];

/// One slot of [`CONSTRUCTED`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Constructed {
    Refused,
    Letter,
    MotionLetter,
    NoChart,
    Held,
    Composed,
    InPart,
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
    pub constructed: [u64; 7],
    /// Its fibres' flexes, by [`FLEX`].
    pub flex: [u64; 9],
    /// Bodies whose spheres met in an admitted scene, by [`BODIES`].
    pub bodies: [u64; 3],
    /// Its tasks' perspectives, by [`PERSPECTIVE`].
    pub perspective: [u64; 9],
    /// What its failed perspectives disagreed about, by [`DISAGREE`].
    pub disagree: [u64; 9],
    /// 78.167 A2 — what their rules' demands spoke nothing for, by
    /// [`DEMANDED`].
    pub demanded_kinds: [u64; 6],
    /// 78.167 A3 — the boost modes of its words, by [`BOOSTS`].
    pub boosts: [u64; 3],
    /// 78.167 B — tests whose speech was held, by [`WITHHELD`].
    pub withheld_tests: [u64; 2],
    /// 78.167 C — families' acts, by [`ACTS`].
    pub acts: [u64; 8],
    /// Its questions, by [`ASKS`].
    pub asks: [u64; 5],
    /// Heard rings by how they were understood, by [`SEATS`]; mutual
    /// balances at the speaker, by [`MUTUAL`]; its lexicon's size, and what
    /// heard speech grew it by; its vocabulary's words; and the rings it
    /// remembers hearing.
    pub seats: [u64; 5],
    pub mutual: [u64; 2],
    pub lexicon: u64,
    pub grown: u64,
    pub vocabulary: u64,
    pub remembered: u64,
    /// The answers it heard, by [`ANSWERS`]; and its questions open now, and
    /// of those the ones answered whose factors it does not yet all know.
    pub answers: [u64; 3],
    pub unanswered: u64,
    pub awaiting: u64,
    /// Its hearers' readings of what it said, by [`VANTAGES`].
    pub vantages: [u64; 3],
    /// What compaction held back, by [`REPEATS`]: things it would have said
    /// again, residues it would have asked about again.
    pub repeats: [u64; 2],
    /// The modes of the words its perception taught it, by [`LAWS`].
    pub laws: [u64; 4],
    /// Spin coupled to orbit: each lock `p:q` read, with how often, in
    /// ratio order; and the drifts held as torsion.
    pub locks: Vec<(u64, u64, u64)>,
    pub drifts: u64,
    /// Perception's chambers: rings held as shears (precessing), and rings
    /// escaping, docketed as demands.
    pub shears: u64,
    pub escapes: u64,
    /// HOLDs this mind sent.
    pub held: u64,
    /// Agreed words this mind spoke to its court.
    pub agreed: u64,
    /// Compositions of ruled words this mind spoke to its court (D4), each
    /// a word whose derivation cites them.
    pub composed: u64,
    /// Sentences shared with another mind: agreed between the two, never
    /// carried to a court, which adjudicates words.
    pub shared: u64,
    /// The deviation of the hearings whose chamber closed, and of those whose
    /// chamber did not (open or unread), each summed exact lane by lane.
    pub closed: [Q; 4],
    pub open: [Q; 4],
    /// What the conversation's carrier holds now: the thoughts awaiting an
    /// ACK, the things said awaiting a HOLD, the letters awaiting a union.
    pub thoughts: u64,
    pub unions: u64,
    pub waiting: u64,
    /// The holes standing on the mind's complex — classes the cross kept and
    /// questions carried — as the last vent left them.
    pub holes: u64,
    /// The scars on it: holes cauterized where their class could not be read
    /// back.
    pub scars: u64,
    /// The classes its vacuum holds, frozen in place.
    pub frozen: u64,
    /// Its breaths (MU-CLIENT §4.3): those that moved on the radial axis, and
    /// their planes by kind, by [`BREATH`].
    pub breath: [u64; 4],
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
            constructed: [0; 7],
            flex: [0; 9],
            bodies: [0; 3],
            perspective: [0; 9],
            disagree: [0; 9],
            demanded_kinds: [0; 6],
            boosts: [0; 3],
            withheld_tests: [0; 2],
            acts: [0; 8],
            asks: [0; 5],
            seats: [0; 5],
            mutual: [0; 2],
            lexicon: 0,
            grown: 0,
            vocabulary: 0,
            remembered: 0,
            answers: [0; 3],
            unanswered: 0,
            awaiting: 0,
            vantages: [0; 3],
            repeats: [0; 2],
            laws: [0; 4],
            locks: Vec::new(),
            drifts: 0,
            shears: 0,
            escapes: 0,
            held: 0,
            agreed: 0,
            composed: 0,
            shared: 0,
            closed: zero4(),
            open: zero4(),
            thoughts: 0,
            unions: 0,
            waiting: 0,
            holes: 0,
            scars: 0,
            frozen: 0,
            breath: [0; 4],
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
        for v in self.constructed.iter().chain(&self.flex).chain(&self.bodies).chain(&self.perspective).chain(&self.disagree).chain(&self.asks).chain(&self.seats).chain(&self.mutual) {
            put_u64(out, *v);
        }
        for v in [self.held, self.agreed, self.shared] {
            put_u64(out, v);
        }
        put_lanes(out, &self.closed);
        put_lanes(out, &self.open);
        for v in [self.thoughts, self.unions, self.waiting, self.drifts, self.shears, self.escapes, self.lexicon, self.grown, self.vocabulary, self.remembered]
            .into_iter()
            .chain(self.answers)
            .chain([self.unanswered, self.awaiting])
            .chain(self.vantages)
            .chain(self.laws)
            .chain(self.repeats)
            .chain([self.composed])
            .chain(self.demanded_kinds)
            .chain(self.boosts)
            .chain(self.withheld_tests)
            .chain(self.acts)
            .chain([self.holes, self.scars, self.frozen])
            .chain(self.breath)
        {
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
        let mut constructed = [0u64; 7];
        for v in &mut constructed {
            *v = take_u64(b, p)?;
        }
        let mut flex = [0u64; 9];
        for v in &mut flex {
            *v = take_u64(b, p)?;
        }
        let bodies = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let mut perspective = [0u64; 9];
        for v in &mut perspective {
            *v = take_u64(b, p)?;
        }
        let mut disagree = [0u64; 9];
        for v in &mut disagree {
            *v = take_u64(b, p)?;
        }
        let asks = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let seats = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let mutual = [take_u64(b, p)?, take_u64(b, p)?];
        let [held, agreed, shared] = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let closed = take_lanes(b, p)?;
        let open = take_lanes(b, p)?;
        let [thoughts, unions, waiting] = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let [drifts, shears, escapes, lexicon, grown] = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let [vocabulary, remembered] = [take_u64(b, p)?, take_u64(b, p)?];
        let answers = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let [unanswered, awaiting] = [take_u64(b, p)?, take_u64(b, p)?];
        let vantages = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let laws = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let repeats = [take_u64(b, p)?, take_u64(b, p)?];
        let composed = take_u64(b, p)?;
        let mut demanded_kinds = [0u64; 6];
        for v in &mut demanded_kinds {
            *v = take_u64(b, p)?;
        }
        let boosts = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let withheld_tests = [take_u64(b, p)?, take_u64(b, p)?];
        let mut acts = [0u64; 8];
        for a in &mut acts {
            *a = take_u64(b, p)?;
        }
        let [holes, scars, frozen] = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let breath = [take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?, take_u64(b, p)?];
        let n =usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
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
            perspective,
            disagree,
            demanded_kinds,
            boosts,
            withheld_tests,
            acts,
            asks,
            seats,
            mutual,
            lexicon,
            grown,
            vocabulary,
            remembered,
            answers,
            unanswered,
            awaiting,
            vantages,
            repeats,
            laws,
            locks,
            drifts,
            shears,
            escapes,
            held,
            agreed,
            composed,
            shared,
            closed,
            open,
            thoughts,
            unions,
            waiting,
            holes,
            scars,
            frozen,
            breath,
        })
    }

    /// The reading as the console prints it.
    pub fn lines(&self) -> Vec<String> {
        let priced: Vec<String> = PRICED.iter().zip(&self.priced).map(|(n, v)| format!("{n} {v}")).collect();
        let constructed: Vec<String> = CONSTRUCTED.iter().zip(&self.constructed).map(|(n, v)| format!("{n} {v}")).collect();
        vec![
            format!(
                "mind K {} · heard {} · understood {} (hearer) {} (speaker) · held {} · words agreed to the court {} · compositions spoken {} · sentences shared {}",
                self.k, self.heard, self.understood, self.returned, self.held, self.agreed, self.composed, self.shared
            ),
            format!("  chambers {} · demanded {}", priced.join(" "), self.demanded),
            format!("  constructed {}", constructed.join(" ")),
            format!(
                "  perspectives {} · tests withheld {} · acts {}",
                PERSPECTIVE.iter().zip(&self.perspective).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                WITHHELD.iter().zip(&self.withheld_tests).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                ACTS.iter().zip(&self.acts).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" ")
            ),
            format!(
                "  disagreed {} · demands unspoken {} · questions {} · answers {} · unanswered {} (awaiting factors {}) · {}",
                DISAGREE.iter().zip(&self.disagree).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                DEMANDED.iter().zip(&self.demanded_kinds).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                ASKS.iter().zip(&self.asks).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                ANSWERS.iter().zip(&self.answers).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                self.unanswered,
                self.awaiting,
                REPEATS.iter().zip(&self.repeats).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" · ")
            ),
            format!(
                "  rings heard {} · mutual {} · read from {} · words' laws {} (boosts {}) · lexicon {} (grown {} by speech) · vocabulary {} · remembers hearing {}",
                SEATS.iter().zip(&self.seats).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                MUTUAL.iter().zip(&self.mutual).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                VANTAGES.iter().zip(&self.vantages).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                LAWS.iter().zip(&self.laws).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                BOOSTS.iter().zip(&self.boosts).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                self.lexicon,
                self.grown,
                self.vocabulary,
                self.remembered
            ),
            format!(
                "  fibres {} · bodies met {} · holes standing {} · scars {} · frozen {} · breath {}",
                FLEX.iter().zip(&self.flex).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                BODIES.iter().zip(&self.bodies).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" "),
                self.holes,
                self.scars,
                self.frozen,
                BREATH.iter().zip(&self.breath).map(|(n, v)| format!("{n} {v}")).collect::<Vec<_>>().join(" ")
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
            format!("  carrier holds thoughts {} · unions {} · waiting {}", self.thoughts, self.unions, self.waiting),
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
