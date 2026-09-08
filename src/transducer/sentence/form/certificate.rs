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
}

impl PivotBy {
    pub(crate) fn face(&self) -> u16 {
        match self {
            PivotBy::Word { face, .. }
            | PivotBy::Placement { face, .. }
            | PivotBy::Hue { face, .. } => *face,
        }
    }

    pub(crate) fn move_key(&self) -> Vec<u8> {
        match self {
            PivotBy::Word { word, .. } => word.rotor().key().into_vec(),
            PivotBy::Placement { translator, .. } => {
                let mut buf = vec![1u8];
                if let Some(k) = translator.key() {
                    let _ = crate::transducer::a21::put_addr(&mut buf, k);
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
    pub before: crate::transducer::a21::Address,
    pub after: crate::transducer::a21::Address,
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
        let mut buf = vec![crate::transducer::a21::TAG_CERT];
        buf.extend_from_slice(&self.chord.to_le_bytes());
        buf.push(self.axis.to_byte());
        crate::transducer::a21::put_bytes(&mut buf, &self.before);
        crate::transducer::a21::put_bytes(&mut buf, &self.after);
        put_rank_move(&self.rank_move, &mut buf);
        match &self.by {
            PivotBy::Word { face, word, side } => {
                buf.extend_from_slice(&face.to_le_bytes());
                buf.push(match side {
                    Side::Left => 0,
                    Side::Right => 1,
                });
                crate::transducer::a21::put_bytes(&mut buf, &word.key());
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
        }
        buf
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.chord.to_le_bytes());
        out.push(self.axis.to_byte());
        crate::transducer::a21::put_addr(out, &self.before);
        crate::transducer::a21::put_addr(out, &self.after);
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
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<PivotCertificate> {
        let chord = take_u16(b, pos)?;
        let axis = Axis::from_byte(*b.get(*pos)?)?;
        *pos += 1;
        let before = crate::transducer::a21::take_addr(b, pos)?;
        let after = crate::transducer::a21::take_addr(b, pos)?;
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
