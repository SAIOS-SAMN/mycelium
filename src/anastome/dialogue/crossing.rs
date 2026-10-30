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
//! universe B is the mouth's other side. Where matter crosses, BOTH reserves
//! sign what crosses, per lane — A's DEBIT and B's CREDIT, exactly equal —
//! so no universe mints matter at a throat; the court checks the two
//! signatures and the equality, never either universe's books.

use super::atlas::{byte, head, key32, u64_at, KIND_TRAVERSE};
use crate::chronometry::tuniversal::TPoint;

const CROSSING: &[u8] = b"mycelium crossing v1";
const DEBIT: &[u8] = b"mycelium crossing debit v1";
const CREDIT: &[u8] = b"mycelium crossing credit v1";
const TRAVERSE: &[u8] = b"mycelium traverse v1";

/// ONE RESERVE'S DECLARATION of what crosses, per lane `S:G:T:H` (its
/// inverse-μ counted on the lanes as the kernel declares it), and its
/// reserve's signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Declaration {
    pub lanes: [u64; 4],
    pub sig: [u8; 64],
}

/// THE TRAVERSE: who pays; the mouth and A's side of it; the final point
/// the crossing reads the map at; both mouths' mapped final points; both
/// universes' latest entries and their `u`; the route (map segment ids,
/// either universe's); the two declarations where matter crosses; a nonce.
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
    pub debit: Option<Declaration>,
    pub credit: Option<Declaration>,
    pub nonce: [u8; 32],
}

fn put_lanes(out: &mut Vec<u8>, l: &[u64; 4]) {
    for x in l {
        out.extend_from_slice(&x.to_le_bytes());
    }
}

fn take_lanes(b: &[u8], p: &mut usize) -> Option<[u64; 4]> {
    Some([u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?])
}

fn put_declaration(out: &mut Vec<u8>, d: &Option<Declaration>) {
    match d {
        None => out.push(0),
        Some(d) => {
            out.push(1);
            put_lanes(out, &d.lanes);
            out.extend_from_slice(&d.sig);
        }
    }
}

fn take_declaration(b: &[u8], p: &mut usize) -> Option<Option<Declaration>> {
    match byte(b, p)? {
        0 => Some(None),
        1 => {
            let lanes = take_lanes(b, p)?;
            let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
            *p += 64;
            Some(Some(Declaration { lanes, sig }))
        }
        _ => None,
    }
}

impl Traverse {
    /// The crossing's coordinates: everything but the declarations.
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
    /// what both declarations and a receiver's applied list name.
    pub fn crossing(&self) -> [u8; 32] {
        let mut b = CROSSING.to_vec();
        b.extend_from_slice(&self.core());
        crate::chronometry::payload_hash(&b)
    }

    /// What A's reserve signs for its DEBIT: `"mycelium crossing debit v1"
    /// ‖ crossing ‖ lanes`.
    pub fn debit_bytes(&self, lanes: &[u64; 4]) -> Vec<u8> {
        let mut out = DEBIT.to_vec();
        out.extend_from_slice(&self.crossing());
        put_lanes(&mut out, lanes);
        out
    }

    /// What B's reserve signs for its CREDIT.
    pub fn credit_bytes(&self, lanes: &[u64; 4]) -> Vec<u8> {
        let mut out = CREDIT.to_vec();
        out.extend_from_slice(&self.crossing());
        put_lanes(&mut out, lanes);
        out
    }

    /// What the payer signs: `"mycelium traverse v1" ‖ core ‖ debit ‖
    /// credit`.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = TRAVERSE.to_vec();
        out.extend_from_slice(&self.core());
        put_declaration(&mut out, &self.debit);
        put_declaration(&mut out, &self.credit);
        out
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedTraverse {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedTraverse { traverse: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedTraverse {
    pub traverse: Traverse,
    pub sig: [u8; 64],
}

/// `[ACT_DOMAIN][from 32][0x80 0x00][16][payer 32][mouth 32][side u8][at
/// 84][point_a 84][point_b 84][entry_a 32][u_a u64][entry_b 32][u_b u64]
/// [route count][segment 32 …][0 | 1 debit lanes 4×u64 sig 64][0 | 1 credit
/// …][nonce 32][sig 64]`.
pub fn encode_traverse(s: &SignedTraverse) -> Vec<u8> {
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
    put_declaration(&mut out, &t.debit);
    put_declaration(&mut out, &t.credit);
    out.extend_from_slice(&t.nonce);
    out.extend_from_slice(&s.sig);
    out
}

pub(crate) fn take_traverse(from: [u8; 32], b: &[u8], p: &mut usize) -> Option<SignedTraverse> {
    let (payer, mouth) = (key32(b, p)?, key32(b, p)?);
    let side = byte(b, p)?;
    let (at, point_a, point_b) = (TPoint::take(b, p)?, TPoint::take(b, p)?, TPoint::take(b, p)?);
    let (entry_a, u_a) = (key32(b, p)?, u64_at(b, p)?);
    let (entry_b, u_b) = (key32(b, p)?, u64_at(b, p)?);
    let route = (0..crate::transducer::address::take_key_count(b, p)?).map(|_| key32(b, p)).collect::<Option<Vec<_>>>()?;
    let (debit, credit) = (take_declaration(b, p)?, take_declaration(b, p)?);
    let nonce = key32(b, p)?;
    let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
    *p += 64;
    let traverse = Traverse { from, payer, mouth, side, at, point_a, point_b, entry_a, u_a, entry_b, u_b, route, debit, credit, nonce };
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
    /// 10 — matter declared by one reserve only.
    OneSided,
    /// 11 — the debit and the credit differ on a lane.
    Mismatched,
    /// 12 — a declaration's signature is not its reserve's.
    DeclarationUnsigned(Side),
    /// 13 — the payer's stake does not cover the map fee, on a lane.
    StakeShort,
    /// 14 — two of the cited points stand at different origins.
    Origins,
    /// 15 — a gas lane or the fee passes its carrier.
    Overflow,
    /// 16 — the court's line could not take it.
    LineRefused,
    /// 17 — the same crossing sealed already.
    Crossed,
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
            CrossRefusal::OneSided => [10, 0],
            CrossRefusal::Mismatched => [11, 0],
            CrossRefusal::DeclarationUnsigned(s) => [12, s.byte()],
            CrossRefusal::StakeShort => [13, 0],
            CrossRefusal::Origins => [14, 0],
            CrossRefusal::Overflow => [15, 0],
            CrossRefusal::LineRefused => [16, 0],
            CrossRefusal::Crossed => [17, 0],
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
            (10, 0) => CrossRefusal::OneSided,
            (11, 0) => CrossRefusal::Mismatched,
            (12, _) => CrossRefusal::DeclarationUnsigned(s()?),
            (13, 0) => CrossRefusal::StakeShort,
            (14, 0) => CrossRefusal::Origins,
            (15, 0) => CrossRefusal::Overflow,
            (16, 0) => CrossRefusal::LineRefused,
            (17, 0) => CrossRefusal::Crossed,
            _ => return None,
        })
    }
}

/// THE ROUTE'S GAS, per lane, exact and never collapsed: S the route's
/// spatial span in geometrized megaparsecs (an exact rational), G the value
/// carried as both reserves declared it, T the TUniversal distance between
/// the two mouths' cited final points (ticks), H the information carried.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gas {
    pub s: (u64, u64),
    pub g: u64,
    pub t: u64,
    pub h: u64,
}

/// WHAT THE COURT ANSWERED a traverse: the crossing's hash, the height and
/// the court's final point it sealed at, the gas, the read price per lane it
/// was priced at, and the fee — `⌊gas · price⌋` per lane — held as a claim
/// on the payer's stake and paid whole to the route's courts.
#[derive(Clone, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "the sealed crossing is the answer's content and its common case; a refusal is the rare one")]
pub enum Crossed {
    Sealed { crossing: [u8; 32], h: u64, point: TPoint, gas: Gas, price: [u64; 4], fee: [u64; 4] },
    Refused(CrossRefusal),
}

/// `[1][crossing 32][h u64][point 84][S n u64][S d u64][G u64][T u64][H
/// u64][price 4×u64][fee 4×u64]` or `[0][code][side]`.
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
            put_lanes(&mut out, price);
            put_lanes(&mut out, fee);
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
            let (price, fee) = (take_lanes(rest, &mut p)?, take_lanes(rest, &mut p)?);
            (p == rest.len()).then_some(Crossed::Sealed { crossing, h, point, gas, price, fee })
        }
        _ => None,
    }
}

/// 78.154 / 78.156 §5 — THE RECONCILIATION EDGE a traverse records on the
/// map: the two universes, the mouth, the crossing's point, `u_a` and
/// `u_entry` (B's latest entry's `u`, cited), and `u_applied` — where B
/// transduced what crossed, committed in B's NEXT entry. Until then the edge
/// reads PENDING, never a disagreement; once applied, its RESIDUAL is
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
    /// The residual between where B applied it and the entry cited; `None`
    /// while pending.
    pub fn residual(&self) -> Option<i128> {
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
