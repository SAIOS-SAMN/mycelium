// A20 — THE PIVOT CERTIFICATE: what a pivot claims, and the axis and side
// it claims it on.
//
//! A certificate re-derives or it is refused (REFUSE_PIVOT_INEXACT): the
//! claim is the move, and the move is replayed.

use super::*;
use crate::transducer::sentence::class::{RankAxis, RankMove};

/// Which side of the chord face a held word is written on: the face word
/// `g` becomes `g · w⁻¹` (Right) or `w · g` (Left). Both are the relation
/// "the loop's holonomy is this held word"; on a non-abelian chart they are
/// two vertices and two candidates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// The move a certificate records (A20 `by`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PivotBy {
    /// A held word written onto the chord face `face` as the relation the
    /// bundle was missing on that loop.
    Word { face: u16, word: Word, side: Side },
    /// The translator residue of the holonomy carriage (A4).
    Placement {
        face: u16,
        translator: Versor,
    },
    /// The hue cycle type, composed as inverse onto the chord gauge (A4).
    Hue { face: u16, gauge: Vec<(u8, u8)> },
    /// A chart axis opened over the loop closing on `face`, seeded by the
    /// loop's holonomy versor (3.7, 16.16 step 3; I6). The move writes no
    /// face: it makes the holonomy a coordinate of the form.
    OpenAxis { face: u16, seed: Versor },
}

impl PivotBy {
    pub(crate) fn face(&self) -> u16 {
        match self {
            PivotBy::Word { face, .. }
            | PivotBy::Placement { face, .. }
            | PivotBy::Hue { face, .. }
            | PivotBy::OpenAxis { face, .. } => *face,
        }
    }

    pub(crate) fn move_key(&self) -> Vec<u8> {
        match self {
            PivotBy::Word { word, .. } => word.rotor().key().into_vec(),
            PivotBy::Placement { translator, .. } => {
                let mut buf = vec![1u8];
                if let Some(k) = translator.key() {
                    let _ = crate::transducer::address::put_addr(&mut buf, k);
                }
                buf
            }
            PivotBy::Hue { gauge, .. } => {
                let mut buf = vec![2u8];
                buf.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
                for &(a, b) in gauge {
                    buf.push(a);
                    buf.push(b);
                }
                buf
            }
            PivotBy::OpenAxis { seed, .. } => {
                let mut buf = vec![3u8];
                if let Some(k) = seed.key() {
                    let _ = crate::transducer::address::put_addr(&mut buf, k);
                }
                buf
            }
        }
    }
}

/// μ⃗ on a certificate: measured over the bundle, or unmeasured.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mu {
    Measured { before: [u64; 4], after: [u64; 4] },
    Unmeasured,
}

/// A certified discharge of one occupied axis (A20). Content-addressed:
/// two minds that discharge one loop by one move produce one key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PivotCertificate {
    /// The loop's index in the walk before the move.
    pub chord: u16,
    pub axis: Axis,
    /// Class key of κ(ℓ) before, and after (zero vector when the loop closed).
    pub before: crate::transducer::address::Address,
    pub after: crate::transducer::address::Address,
    /// WHAT THE PIVOT DID TO THE RANK (S9.1): the move itself, not the two
    /// audits it was read from. The certificate makes a claim — the rank
    /// held, or one unit left this coordinate for `axes` — and the court
    /// re-walks the form and re-derives the same move (A20). Two absolute
    /// ranks would have been data the court derives for itself anyway, and
    /// leaving them here invited a later reader to do its own comparison.
    pub rank_move: RankMove,
    /// μ⃗ of the form over the bundle before and after the move.
    pub mu: Mu,
    pub by: PivotBy,
}

impl PivotCertificate {
    pub fn key(&self) -> Vec<u8> {
        let mut buf = vec![crate::transducer::address::TAG_CERT];
        buf.extend_from_slice(&self.chord.to_le_bytes());
        buf.push(self.axis.to_byte());
        crate::transducer::address::put_bytes(&mut buf, &self.before);
        crate::transducer::address::put_bytes(&mut buf, &self.after);
        put_rank_move(&self.rank_move, &mut buf);
        match &self.by {
            PivotBy::Word { face, word, side } => {
                buf.extend_from_slice(&face.to_le_bytes());
                buf.push(match side {
                    Side::Left => 0,
                    Side::Right => 1,
                });
                crate::transducer::address::put_bytes(&mut buf, &word.key());
            }
            PivotBy::Placement { face, translator } => {
                buf.push(1);
                buf.extend_from_slice(&face.to_le_bytes());
                translator.to_bytes(&mut buf);
            }
            PivotBy::Hue { face, gauge } => {
                buf.push(2);
                buf.extend_from_slice(&face.to_le_bytes());
                buf.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
                for &(a, b) in gauge {
                    buf.push(a);
                    buf.push(b);
                }
            }
            PivotBy::OpenAxis { face, seed } => {
                buf.push(3);
                buf.extend_from_slice(&face.to_le_bytes());
                seed.to_bytes(&mut buf);
            }
        }
        buf
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.chord.to_le_bytes());
        out.push(self.axis.to_byte());
        crate::transducer::address::put_addr(out, &self.before);
        crate::transducer::address::put_addr(out, &self.after);
        put_rank_move(&self.rank_move, out);
        match &self.mu {
            Mu::Unmeasured => out.push(0),
            Mu::Measured { before, after } => {
                out.push(1);
                for m in before.iter().chain(after) {
                    out.extend_from_slice(&m.to_le_bytes());
                }
            }
        }
        match &self.by {
            PivotBy::Word { face, word, side } => {
                out.push(0);
                out.extend_from_slice(&face.to_le_bytes());
                out.push(match side {
                    Side::Left => 0,
                    Side::Right => 1,
                });
                word.to_bytes(out);
            }
            PivotBy::Placement { face, translator } => {
                out.push(1);
                out.extend_from_slice(&face.to_le_bytes());
                translator.to_bytes(out);
            }
            PivotBy::Hue { face, gauge } => {
                out.push(2);
                out.extend_from_slice(&face.to_le_bytes());
                out.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
                for &(a, b) in gauge {
                    out.push(a);
                    out.push(b);
                }
            }
            PivotBy::OpenAxis { face, seed } => {
                out.push(3);
                out.extend_from_slice(&face.to_le_bytes());
                seed.to_bytes(out);
            }
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<PivotCertificate> {
        let chord = take_u16(b, pos)?;
        let axis = Axis::from_byte(*b.get(*pos)?)?;
        *pos += 1;
        let before = crate::transducer::address::take_addr(b, pos)?;
        let after = crate::transducer::address::take_addr(b, pos)?;
        let rank_move = take_rank_move(b, pos)?;
        let mu_tag = *b.get(*pos)?;
        *pos += 1;
        let mu = match mu_tag {
            0 => Mu::Unmeasured,
            1 => {
                let mut before = [0u64; 4];
                let mut after = [0u64; 4];
                for m in before.iter_mut().chain(&mut after) {
                    *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
                Mu::Measured { before, after }
            }
            _ => return None,
        };
        let kind = *b.get(*pos)?;
        *pos += 1;
        let by = match kind {
            0 => {
                let face = take_u16(b, pos)?;
                let side = match *b.get(*pos)? {
                    0 => Side::Left,
                    1 => Side::Right,
                    _ => return None,
                };
                *pos += 1;
                let word = Word::from_bytes(b, pos)?;
                PivotBy::Word { face, word, side }
            }
            1 => {
                let face = take_u16(b, pos)?;
                let translator = Versor::from_bytes(b, pos)?;
                PivotBy::Placement { face, translator }
            }
            2 => {
                let face = take_u16(b, pos)?;
                let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut gauge = Vec::with_capacity(ng);
                for _ in 0..ng {
                    gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                    *pos += 2;
                }
                PivotBy::Hue { face, gauge }
            }
            3 => {
                let face = take_u16(b, pos)?;
                let seed = Versor::from_bytes(b, pos)?;
                PivotBy::OpenAxis { face, seed }
            }
            _ => return None,
        };
        Some(PivotCertificate {
            chord,
            axis,
            before,
            after,
            rank_move,
            mu,
            by,
        })
    }
}

/// THE KIND OF A REFUSAL (50.9, 34.1): what the court found and the
/// contraction move it names. `VertexBody` is `REFUSE_VERTEX_BODY`,
/// `EdgeHolonomy` is `REFUSE_EDGE`, `RemainderUndeclared` is
/// `REFUSE_NOT_CLOSED`; every other refusal of the tape is a
/// `MalformedComplex`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefusalKind {
    VertexBody,
    EdgeHolonomy,
    RemainderUndeclared,
    MalformedComplex,
}

impl RefusalKind {
    pub fn to_byte(self) -> u8 {
        match self {
            RefusalKind::VertexBody => 0,
            RefusalKind::EdgeHolonomy => 1,
            RefusalKind::RemainderUndeclared => 2,
            RefusalKind::MalformedComplex => 3,
        }
    }

    pub fn from_byte(b: u8) -> Option<RefusalKind> {
        Some(match b {
            0 => RefusalKind::VertexBody,
            1 => RefusalKind::EdgeHolonomy,
            2 => RefusalKind::RemainderUndeclared,
            3 => RefusalKind::MalformedComplex,
            _ => return None,
        })
    }

    /// The word of 34.2 and 34.3's `refused {kind}` and `contracted … by {kind}` lines.
    pub fn label(self) -> &'static str {
        match self {
            RefusalKind::VertexBody => "vertex body",
            RefusalKind::EdgeHolonomy => "edge holonomy",
            RefusalKind::RemainderUndeclared => "remainder undeclared",
            RefusalKind::MalformedComplex => "malformed complex",
        }
    }
}

/// THE COORDINATE A REFUSAL NAMES (50.9), as content and never as a
/// position in the sender's tape: the word vertex with the chart it acts
/// on, a face's two vertex addresses, the chart with the cells the court
/// read, or the whole tape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum At {
    Vertex { address: crate::transducer::address::Address, chart: stalk::Invariant },
    Face(crate::transducer::address::Address, crate::transducer::address::Address),
    Chart(stalk::Invariant, Vec<Cell>),
    Tape(crate::transducer::address::Address),
}

impl At {
    /// The coordinate as 34.2 and 34.3 write it.
    pub fn label(&self) -> String {
        let hx = |a: &[u8]| -> String {
            let mut s: String = a.iter().take(2).map(|b| format!("{b:02x}")).collect();
            s.push('…');
            s
        };
        match self {
            At::Vertex { address, chart } => format!("vertex {} chart {}", hx(address.as_bytes()), hx(chart.key().as_bytes())),
            At::Face(a, b) => format!("face {}-{}", hx(a.as_bytes()), hx(b.as_bytes())),
            At::Chart(inv, cells) => format!("chart {} cells {}", hx(inv.key().as_bytes()), cells.len()),
            At::Tape(a) => format!("tape {}", hx(a.as_bytes())),
        }
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        match self {
            At::Vertex { address, chart } => {
                out.push(0);
                let _ = crate::transducer::address::put_addr(out, address);
                let _ = crate::transducer::sentence::tape::put_inv(chart, out);
            }
            At::Face(a, b) => {
                out.push(1);
                let _ = crate::transducer::address::put_addr(out, a);
                let _ = crate::transducer::address::put_addr(out, b);
            }
            At::Chart(inv, cells) => {
                out.push(2);
                let _ = crate::transducer::sentence::tape::put_inv(inv, out);
                out.extend_from_slice(&(cells.len() as u32).to_le_bytes());
                for (r, c, before, after) in cells {
                    out.extend_from_slice(&r.to_le_bytes());
                    out.extend_from_slice(&c.to_le_bytes());
                    out.push(*before);
                    out.push(*after);
                }
            }
            At::Tape(a) => {
                out.push(3);
                let _ = crate::transducer::address::put_addr(out, a);
            }
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<At> {
        let tag = *b.get(*pos)?;
        *pos += 1;
        Some(match tag {
            0 => {
                let address = crate::transducer::address::take_addr(b, pos)?;
                let chart = crate::transducer::sentence::tape::take_inv(b, pos)?;
                At::Vertex { address, chart }
            }
            1 => {
                let a = crate::transducer::address::take_addr(b, pos)?;
                let c = crate::transducer::address::take_addr(b, pos)?;
                At::Face(a, c)
            }
            2 => {
                let inv = crate::transducer::sentence::tape::take_inv(b, pos)?;
                let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
                *pos += 4;
                let n = take_count(b, *pos, n, 18)?;
                let mut cells = Vec::with_capacity(n);
                for _ in 0..n {
                    let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    let c = i64::from_le_bytes(b.get(*pos + 8..*pos + 16)?.try_into().ok()?);
                    let before = *b.get(*pos + 16)?;
                    let after = *b.get(*pos + 17)?;
                    *pos += 18;
                    cells.push((r, c, before, after));
                }
                At::Chart(inv, cells)
            }
            3 => At::Tape(crate::transducer::address::take_addr(b, pos)?),
            _ => return None,
        })
    }
}

/// THE CONTRACTION CERTIFICATE (50.9): the refusal a court issued, the
/// sentence it refused, the form the move produced, and the court's
/// receipt. Filed on the idea beside its pivot certificates; cited on the
/// next utterance, where the court that issued the receipt verifies that
/// the uttered sentence is the contraction at the coordinate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Contraction {
    pub kind: RefusalKind,
    pub at: At,
    /// The refused sentence's address.
    pub before: crate::transducer::address::Address,
    /// The contracted form's vertex key.
    pub after: crate::transducer::address::Address,
    /// The court's replay receipt: its chain hash (47.1), never a digest of content.
    pub receipt: [u8; 32],
}

impl Contraction {
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.push(self.kind.to_byte());
        self.at.to_bytes(out);
        let _ = crate::transducer::address::put_addr(out, &self.before);
        let _ = crate::transducer::address::put_addr(out, &self.after);
        out.extend_from_slice(&self.receipt);
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Contraction> {
        let kind = RefusalKind::from_byte(*b.get(*pos)?)?;
        *pos += 1;
        let at = At::from_bytes(b, pos)?;
        let before = crate::transducer::address::take_addr(b, pos)?;
        let after = crate::transducer::address::take_addr(b, pos)?;
        let receipt: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        Some(Contraction { kind, at, before, after, receipt })
    }
}

/// The rank move on a tape: one tag byte, and for a discharge the
/// coordinate the unit left. `Inadmissible` carries its four deltas so a
/// refused move can still be written down; no trial produces one, and a
/// tape carrying one replays as the refusal it names.
fn put_rank_move(m: &RankMove, out: &mut Vec<u8>) {
    match m {
        RankMove::Equal => out.push(0),
        RankMove::Discharged { from } => {
            out.push(1);
            out.push(match from {
                RankAxis::Torsion => 0,
                RankAxis::Free => 1,
                RankAxis::OffPlane => 2,
                RankAxis::Axes => 3,
            });
        }
        RankMove::Inadmissible { delta } => {
            out.push(2);
            for d in delta {
                out.extend_from_slice(&d.to_le_bytes());
            }
        }
    }
}

fn take_rank_move(b: &[u8], pos: &mut usize) -> Option<RankMove> {
    let tag = *b.get(*pos)?;
    *pos += 1;
    match tag {
        0 => Some(RankMove::Equal),
        1 => {
            let from = match *b.get(*pos)? {
                0 => RankAxis::Torsion,
                1 => RankAxis::Free,
                2 => RankAxis::OffPlane,
                3 => RankAxis::Axes,
                _ => return None,
            };
            *pos += 1;
            Some(RankMove::Discharged { from })
        }
        2 => {
            let mut delta = [0i64; 4];
            for d in &mut delta {
                *d = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                *pos += 8;
            }
            Some(RankMove::Inadmissible { delta })
        }
        _ => None,
    }
}

/// 50.2 STEP 2 — THE PATH a word vertex was reached through: the
/// generators of the speaker's presentation, in application order with
/// their exponents.
///
/// The path is the proof and it travels beside the tape (26, as the
/// contraction certificate does); it is never a coordinate of the form,
/// because the geodesic on the tape quotients it away (1.5) and law 12
/// admits only what the form re-derives. Two minds reach one vertex by
/// two paths — one address, two proofs (13.10, 23.9) — and the ledger
/// tells them apart where the address cannot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    /// The word vertex this path reaches.
    pub vertex: crate::transducer::address::Address,
    /// The generators cited: the form each lives in, the word vertex
    /// itself, and the exponent, in application order. The speaker knows
    /// both addresses, so the receiver does one keyed read on its cover
    /// and one on that form's own index — never a walk to find which form
    /// a vertex belongs to (law 3).
    pub generators: Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        i64,
    )>,
    /// What the presentation did not reach, spelled from the identity in
    /// the seventeen letters (A22). The identity means the generators
    /// alone compose to the vertex.
    pub residue: crate::transducer::alphabet::Word,
}

impl Path {
    /// The presentation reached the whole vertex: nothing was left for A22
    /// to spell.
    pub fn whole(&self) -> bool {
        self.residue.is_identity()
    }
}

impl Path {
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        crate::transducer::address::put_bytes(out, self.vertex.as_bytes());
        out.extend_from_slice(&(self.generators.len() as u32).to_le_bytes());
        for (f, v, e) in &self.generators {
            crate::transducer::address::put_bytes(out, f.as_bytes());
            crate::transducer::address::put_bytes(out, v.as_bytes());
            out.extend_from_slice(&e.to_le_bytes());
        }
        self.residue.to_bytes(out);
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Path> {
        let vertex = crate::transducer::address::Address::from(
            crate::transducer::address::take_bytes(b, pos)?.to_vec(),
        );
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        let mut generators = Vec::with_capacity(n.min(64));
        for _ in 0..n {
            let f = crate::transducer::address::Address::from(
                crate::transducer::address::take_bytes(b, pos)?.to_vec(),
            );
            let v = crate::transducer::address::Address::from(
                crate::transducer::address::take_bytes(b, pos)?.to_vec(),
            );
            let e = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            generators.push((f, v, e));
        }
        let residue = crate::transducer::alphabet::Word::from_bytes(b, pos)?;
        Some(Path { vertex, generators, residue })
    }
}
