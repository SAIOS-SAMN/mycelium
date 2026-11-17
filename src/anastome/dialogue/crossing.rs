// 78.154 — TRAVERSE: a COORDINATE RETRIEVAL AND MAPPING act (MU-ECONOMY
// §7.1; ruled 2026-10-09: every wormhole traversal needs coordinates
// from the hypha; 78.156 §4–5; refinements (1)–(3)). A
// sub-kind of ACT_DOMAIN, never of the 78.128 handoff family: the handoff
// family's TRAVERSE moves matter through the court, and the crossing — matter,
// a ship, a fleet — happens in its own universe, never through the hypha.
// What a traverser buys here is the map: both mouths' final points and both
// universes' latest ledger entries, reconciled on one edge; it pays the MAP
// FEE and nothing else, to the courts keeping the route's map.
//
//! The payer is any key (it pays from its STAKE, 78.154 (4)). The frame's
//! domain is the SENDING universe A, at its side of the mouth; the receiving
//! universe B is the mouth's other side. Where matter crosses, each
//! universe at the tube signs its MOTION (78.184 §6, ruled 2026-11-16): the
//! nuclei its side releases and the nuclei its side takes in, in the
//! manifest's own writing. The transport is multi-axial, so the crossing has
//! no direction and one tube carries both ways. The court conserves
//! nucleons across the motions (dissipation admitted) and moves the μ held
//! on the nuclei with them.

use super::atlas::{byte, head, key32, u64_at, KIND_TRAVERSE};
use crate::chronometry::tuniversal::TPoint;

const CROSSING: &[u8] = b"mycelium crossing v1";
const MOTION: &[u8] = b"mycelium crossing motion v1";
const TRAVERSE: &[u8] = b"mycelium traverse v1";

/// 78.184 §6 — ONE UNIVERSE'S MOTION at the tube: the nuclei its side
/// releases and the nuclei its side takes in, each `(Z, Q, R, n)` as the
/// manifest v3 writes it, signed by the domain's sitting reserve over
/// [`Traverse::motion_bytes`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Motion {
    pub domain: [u8; 32],
    pub released: super::endowment::Nuclei,
    pub taken: super::endowment::Nuclei,
    pub sig: [u8; 64],
}

/// THE TRAVERSE: who pays; the mouth and A's side of it; the final point
/// the crossing reads the map at; both mouths' mapped final points; both
/// universes' latest entries and each entry's `u`; the route (map segment ids,
/// either universe's); each universe's motion where matter crosses; a nonce.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Traverse {
    pub from: [u8; 32],
    pub payer: [u8; 32],
    pub mouth: [u8; 32],
    pub side: u8,
    pub at: TPoint,
    pub point_a: TPoint,
    pub point_b: TPoint,
    pub entry_a: [u8; 32],
    pub u_a: u64,
    pub entry_b: [u8; 32],
    pub u_b: u64,
    pub route: Vec<[u8; 32]>,
    pub motions: Vec<Motion>,
    pub nonce: [u8; 32],
}

fn put_motion(out: &mut Vec<u8>, m: &Motion) -> Option<()> {
    out.extend_from_slice(&m.domain);
    super::endowment::put_nuclei(out, &m.released)?;
    super::endowment::put_nuclei(out, &m.taken)?;
    out.extend_from_slice(&m.sig);
    Some(())
}

fn take_motion(b: &[u8], p: &mut usize) -> Option<Motion> {
    let domain = key32(b, p)?;
    let released = super::endowment::take_nuclei(b, p)?;
    let taken = super::endowment::take_nuclei(b, p)?;
    let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
    *p += 64;
    Some(Motion { domain, released, taken, sig })
}

/// `[n u8][motion …]`; `None` past 255 motions or a count past its carrier.
fn put_motions(out: &mut Vec<u8>, ms: &[Motion]) -> Option<()> {
    out.push(u8::try_from(ms.len()).ok()?);
    for m in ms {
        put_motion(out, m)?;
    }
    Some(())
}

fn take_motions(b: &[u8], p: &mut usize) -> Option<Vec<Motion>> {
    (0..byte(b, p)?).map(|_| take_motion(b, p)).collect()
}

impl Traverse {
    /// The crossing's coordinates: everything but the motions.
    fn core(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.from);
        out.extend_from_slice(&self.payer);
        out.extend_from_slice(&self.mouth);
        out.push(self.side);
        for p in [&self.at, &self.point_a, &self.point_b] {
            p.put(&mut out);
        }
        out.extend_from_slice(&self.entry_a);
        out.extend_from_slice(&self.u_a.to_le_bytes());
        out.extend_from_slice(&self.entry_b);
        out.extend_from_slice(&self.u_b.to_le_bytes());
        crate::transducer::address::put_key_count(&mut out, self.route.len());
        for s in &self.route {
            out.extend_from_slice(s);
        }
        out.extend_from_slice(&self.nonce);
        out
    }

    /// THE CROSSING'S HASH — the reconciliation edge's id on the map, and
    /// what every motion and a receiver's applied list name.
    pub fn crossing(&self) -> [u8; 32] {
        let mut b = CROSSING.to_vec();
        b.extend_from_slice(&self.core());
        crate::chronometry::payload_hash(&b)
    }

    /// What a universe's sitting reserve signs for its MOTION: `"mycelium
    /// crossing motion v1" ‖ crossing ‖ domain ‖ released ‖ taken`.
    pub fn motion_bytes(&self, domain: &[u8; 32], released: &super::endowment::Nuclei, taken: &super::endowment::Nuclei) -> Option<Vec<u8>> {
        let mut out = MOTION.to_vec();
        out.extend_from_slice(&self.crossing());
        out.extend_from_slice(domain);
        super::endowment::put_nuclei(&mut out, released)?;
        super::endowment::put_nuclei(&mut out, taken)?;
        Some(out)
    }

    /// What the payer signs: `"mycelium traverse v1" ‖ core ‖ motions`;
    /// `None` where the motions do not write.
    pub fn signed_bytes(&self) -> Option<Vec<u8>> {
        let mut out = TRAVERSE.to_vec();
        out.extend_from_slice(&self.core());
        put_motions(&mut out, &self.motions)?;
        Some(out)
    }

    /// The traverse signed by the payer; `None` where its motions do not
    /// write.
    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> Option<SignedTraverse> {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()?).to_bytes();
        Some(SignedTraverse { traverse: self, sig })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedTraverse {
    pub traverse: Traverse,
    pub sig: [u8; 64],
}

/// `[ACT_DOMAIN][from 32][0x80 0x00][16][payer 32][mouth 32][side u8][at
/// 84][point_a 84][point_b 84][entry_a 32][u_a u64][entry_b 32][u_b u64]
/// [route count][segment 32 …][n u8][motion: domain 32, released nuclei,
/// taken nuclei, sig 64 …][nonce 32][sig 64]`; `None` where the motions do
/// not write.
pub fn encode_traverse(s: &SignedTraverse) -> Option<Vec<u8>> {
    let t = &s.traverse;
    let mut out = head(&t.from, KIND_TRAVERSE);
    out.extend_from_slice(&t.payer);
    out.extend_from_slice(&t.mouth);
    out.push(t.side);
    for p in [&t.at, &t.point_a, &t.point_b] {
        p.put(&mut out);
    }
    out.extend_from_slice(&t.entry_a);
    out.extend_from_slice(&t.u_a.to_le_bytes());
    out.extend_from_slice(&t.entry_b);
    out.extend_from_slice(&t.u_b.to_le_bytes());
    crate::transducer::address::put_key_count(&mut out, t.route.len());
    for r in &t.route {
        out.extend_from_slice(r);
    }
    put_motions(&mut out, &t.motions)?;
    out.extend_from_slice(&t.nonce);
    out.extend_from_slice(&s.sig);
    Some(out)
}

pub(crate) fn take_traverse(from: [u8; 32], b: &[u8], p: &mut usize) -> Option<SignedTraverse> {
    let (payer, mouth) = (key32(b, p)?, key32(b, p)?);
    let side = byte(b, p)?;
    let (at, point_a, point_b) = (TPoint::take(b, p)?, TPoint::take(b, p)?, TPoint::take(b, p)?);
    let (entry_a, u_a) = (key32(b, p)?, u64_at(b, p)?);
    let (entry_b, u_b) = (key32(b, p)?, u64_at(b, p)?);
    let route = (0..crate::transducer::address::take_key_count(b, p)?).map(|_| key32(b, p)).collect::<Option<Vec<_>>>()?;
    let motions = take_motions(b, p)?;
    let nonce = key32(b, p)?;
    let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
    *p += 64;
    let traverse = Traverse { from, payer, mouth, side, at, point_a, point_b, entry_a, u_a, entry_b, u_b, route, motions, nonce };
    Some(SignedTraverse { traverse, sig })
}

/// WHICH SIDE a refusal names: A, the sending universe, or B, the receiving.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    A,
    B,
}

impl Side {
    fn byte(self) -> u8 {
        match self {
            Side::A => 0,
            Side::B => 1,
        }
    }

    fn of(b: u8) -> Option<Side> {
        match b {
            0 => Some(Side::A),
            1 => Some(Side::B),
            _ => None,
        }
    }
}

/// WHY A TRAVERSE WAS NOT SEALED.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossRefusal {
    /// 1 — the payer's signature does not verify.
    Unsigned,
    /// 2 — a universe is not founded on this court.
    UnknownDomain,
    /// 3 — no open tube of that id with A at the side named.
    NoMouth,
    /// 4 — the mouth's side is not on its universe's map: a wormhole is
    /// untraversable until its reserve maps it.
    Unmapped(Side),
    /// 5 — the crossing's point is not final in this court's view.
    NotFinal,
    /// 6 — the mouth's cited point is not the one it was mapped at, or it
    /// was mapped above the crossing's point.
    MapPoint(Side),
    /// 7 — the entry cited is not the universe's latest, or it has none.
    NotLatest(Side),
    /// 8 — NOT YET AT h: the universe's latest entry stands more than its
    /// mouth's N final-point advances behind the crossing's point. Commit
    /// an entry and ask again; never a slash.
    NotYetAtH(Side),
    /// 9 — a route segment is on neither universe's map.
    Route,
    /// 10 — 78.184 §6: a motion of a universe that is neither side of the
    /// tube, or two motions of one universe.
    NotAtTube,
    /// 11 — 78.184 §6: the motions take in more nucleons than they release.
    NotConserved,
    /// 12 — a motion's signature is not its universe's sitting reserve's.
    MotionUnsigned(Side),
    /// 13 — the payer's stake does not cover the map fee, on a lane.
    Short,
    /// 14 — two of the cited points stand at different origins.
    Origins,
    /// 15 — a gas lane or the fee passes its carrier.
    Overflow,
    /// 16 — the court's line could not take it.
    LineRefused,
    /// 17 — the same crossing sealed already.
    Crossed,
    /// 18 — 78.184 §6: a universe releases more nucleons than its element
    /// account holds.
    PastAccount(Side),
    /// 19 — 78.184 §5: a motion's entry states a phase off its rest point.
    NotAtRest(Side),
    /// 20 — 78.184 §6: a universe's sitting reserve cannot cover the
    /// delivered μ its release carries.
    Uncovered(Side),
}

impl CrossRefusal {
    fn bytes(self) -> [u8; 2] {
        match self {
            CrossRefusal::Unsigned => [1, 0],
            CrossRefusal::UnknownDomain => [2, 0],
            CrossRefusal::NoMouth => [3, 0],
            CrossRefusal::Unmapped(s) => [4, s.byte()],
            CrossRefusal::NotFinal => [5, 0],
            CrossRefusal::MapPoint(s) => [6, s.byte()],
            CrossRefusal::NotLatest(s) => [7, s.byte()],
            CrossRefusal::NotYetAtH(s) => [8, s.byte()],
            CrossRefusal::Route => [9, 0],
            CrossRefusal::NotAtTube => [10, 0],
            CrossRefusal::NotConserved => [11, 0],
            CrossRefusal::MotionUnsigned(s) => [12, s.byte()],
            CrossRefusal::Short => [13, 0],
            CrossRefusal::Origins => [14, 0],
            CrossRefusal::Overflow => [15, 0],
            CrossRefusal::LineRefused => [16, 0],
            CrossRefusal::Crossed => [17, 0],
            CrossRefusal::PastAccount(s) => [18, s.byte()],
            CrossRefusal::NotAtRest(s) => [19, s.byte()],
            CrossRefusal::Uncovered(s) => [20, s.byte()],
        }
    }

    fn of(code: u8, side: u8) -> Option<CrossRefusal> {
        let s = || Side::of(side);
        Some(match (code, side) {
            (1, 0) => CrossRefusal::Unsigned,
            (2, 0) => CrossRefusal::UnknownDomain,
            (3, 0) => CrossRefusal::NoMouth,
            (4, _) => CrossRefusal::Unmapped(s()?),
            (5, 0) => CrossRefusal::NotFinal,
            (6, _) => CrossRefusal::MapPoint(s()?),
            (7, _) => CrossRefusal::NotLatest(s()?),
            (8, _) => CrossRefusal::NotYetAtH(s()?),
            (9, 0) => CrossRefusal::Route,
            (10, 0) => CrossRefusal::NotAtTube,
            (11, 0) => CrossRefusal::NotConserved,
            (12, _) => CrossRefusal::MotionUnsigned(s()?),
            (13, 0) => CrossRefusal::Short,
            (14, 0) => CrossRefusal::Origins,
            (15, 0) => CrossRefusal::Overflow,
            (16, 0) => CrossRefusal::LineRefused,
            (17, 0) => CrossRefusal::Crossed,
            (18, _) => CrossRefusal::PastAccount(s()?),
            (19, _) => CrossRefusal::NotAtRest(s()?),
            (20, _) => CrossRefusal::Uncovered(s()?),
            _ => return None,
        })
    }
}

/// THE ROUTE'S GAS, per lane, exact and never collapsed: S the route's
/// spatial span in geometrized megaparsecs (an exact rational), G the
/// mapping's value terms, T the TUniversal distance between the two
/// mouths' cited final points (ticks), H the information carried
/// (Concordance 5.8). A motion of nuclei is matter, which crosses in its
/// own universe, so it puts nothing on G or H (2.7).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gas {
    pub s: (u64, u64),
    pub g: u64,
    pub t: u64,
    pub h: u64,
}

/// WHAT THE COURT ANSWERED a traverse: the crossing's hash, the height and
/// the court's final point it sealed at, the gas, the read price per lane it
/// was priced at (μ per unit of gas), and the fee — `⌊gas · price⌋` per lane
/// — held as a claim on the payer's stake and paid whole to the route's
/// courts. Price and fee are exact amounts of μ (`Mu`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Crossed {
    Sealed { crossing: [u8; 32], h: u64, point: TPoint, gas: Gas, price: crate::economy::Mu, fee: crate::economy::Mu },
    Refused(CrossRefusal),
}

/// `[1][crossing 32][h u64][point 84][S n u64][S d u64][G u64][T u64][H
/// u64][price μ⃗][fee μ⃗]` or `[0][code][side]`, each μ⃗ four exact rational
/// lanes (`Mu::put`).
pub fn encode_traverse_reply(c: &Crossed) -> Vec<u8> {
    match c {
        Crossed::Sealed { crossing, h, point, gas, price, fee } => {
            let mut out = vec![1];
            out.extend_from_slice(crossing);
            out.extend_from_slice(&h.to_le_bytes());
            point.put(&mut out);
            for x in [gas.s.0, gas.s.1, gas.g, gas.t, gas.h] {
                out.extend_from_slice(&x.to_le_bytes());
            }
            price.put(&mut out);
            fee.put(&mut out);
            out
        }
        Crossed::Refused(r) => {
            let mut out = vec![0];
            out.extend_from_slice(&r.bytes());
            out
        }
    }
}

pub fn decode_traverse_reply(b: &[u8]) -> Option<Crossed> {
    match b {
        [0, code, side] => Some(Crossed::Refused(CrossRefusal::of(*code, *side)?)),
        [1, rest @ ..] => {
            let mut p = 0;
            let crossing = key32(rest, &mut p)?;
            let h = u64_at(rest, &mut p)?;
            let point = TPoint::take(rest, &mut p)?;
            let s = (u64_at(rest, &mut p)?, u64_at(rest, &mut p)?);
            let gas = Gas { s, g: u64_at(rest, &mut p)?, t: u64_at(rest, &mut p)?, h: u64_at(rest, &mut p)? };
            let (price, fee) = (crate::economy::Mu::take_from(rest, &mut p)?, crate::economy::Mu::take_from(rest, &mut p)?);
            (p == rest.len()).then_some(Crossed::Sealed { crossing, h, point, gas, price, fee })
        }
        _ => None,
    }
}

/// 78.154 / 78.156 §5 — THE RECONCILIATION EDGE a traverse records on the
/// map: the two universes, the mouth, the crossing's point, `u_a` and
/// `u_entry` (B's latest entry's `u`, cited), and `u_applied` — where B
/// transduced what crossed, committed in B's NEXT entry. Until then the edge
/// reads PENDING, never a disagreement; once applied, its RESIDUE is
/// `u_applied − u_entry`, exact and kept.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub mouth: [u8; 32],
    pub at: TPoint,
    pub u_a: u64,
    pub u_entry: u64,
    pub u_applied: Option<u64>,
}

impl Edge {
    /// The residue between where B applied it and the entry cited; `None`
    /// while pending.
    pub fn residue(&self) -> Option<i128> {
        self.u_applied.map(|a| i128::from(a) - i128::from(self.u_entry))
    }

    pub fn put(&self, out: &mut Vec<u8>) {
        for k in [&self.from, &self.to, &self.mouth] {
            out.extend_from_slice(k);
        }
        self.at.put(out);
        out.extend_from_slice(&self.u_a.to_le_bytes());
        out.extend_from_slice(&self.u_entry.to_le_bytes());
        match self.u_applied {
            None => out.push(0),
            Some(u) => {
                out.push(1);
                out.extend_from_slice(&u.to_le_bytes());
            }
        }
    }

    pub fn take(b: &[u8], p: &mut usize) -> Option<Edge> {
        let (from, to, mouth) = (key32(b, p)?, key32(b, p)?, key32(b, p)?);
        let at = TPoint::take(b, p)?;
        let (u_a, u_entry) = (u64_at(b, p)?, u64_at(b, p)?);
        let u_applied = match byte(b, p)? {
            0 => None,
            1 => Some(u64_at(b, p)?),
            _ => return None,
        };
        Some(Edge { from, to, mouth, at, u_a, u_entry, u_applied })
    }
}
