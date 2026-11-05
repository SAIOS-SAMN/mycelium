// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-HOSTING §5a.4–5a.6, CONCORDANCE 4.1, 6.2a, §0, ROADMAP track 24 B4 —
//! THE BOUNDARY'S REFUSALS at a universe's universal core tube, the vertex
//! where its cones join the hypha's, beside 47.x's admission refusals.
//!
//! Two rules govern the acts that reach a hypha (6.2a). A key's own acts
//! (a solve, speech, an attestation, a wallet read, a stake, a lease, a
//! conversion in) are governed by keys, and nothing here reads them. A
//! universe's acts are governed by topology: each is admitted only from the
//! universe's sitting reserve, or carried in the reserve's act, and only
//! while that reserve converses on every network clause of the register
//! served. Every refusal names the act and the domain, and the clause where
//! one decides it; none is a boolean and none a digest.

use crate::chronometry::Stamp;
use crate::physics::laws::clauses::signing::{put_verdict, take_verdict_at};
use crate::physics::laws::clauses::writing::{put_clause_id, take_clause_id};
use crate::physics::laws::clauses::{AlignmentRefusal, ClauseReading, ConcordanceClauseId};

/// A UNIVERSE'S ACT (Concordance 6.2a, TOPOLOGY FOR A UNIVERSE'S ACTS): what
/// crosses its universal core tube, never a key's own act.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniverseAct {
    /// A tier's fold (78.119, 78.138).
    Fold,
    /// A chronometric anchor, a ledger entry (78.154).
    Anchor,
    /// A map of mouth coordinates (78.154).
    Map,
    /// A mouth, a core tube among them (78.134, 78.139).
    Mouth,
    /// A conversion out of the reserve (78.127).
    ConversionOut,
    /// A manifest of nuclei (78.155b).
    Manifest,
    /// A statement of the universe (78.139).
    Statement,
}

impl UniverseAct {
    /// Every universe act, in 6.2a's order.
    pub const ALL: [UniverseAct; 7] =
        [UniverseAct::Fold, UniverseAct::Anchor, UniverseAct::Map, UniverseAct::Mouth, UniverseAct::ConversionOut, UniverseAct::Manifest, UniverseAct::Statement];

    /// The act's byte on the wire.
    pub const fn byte(self) -> u8 {
        match self {
            UniverseAct::Fold => 0,
            UniverseAct::Anchor => 1,
            UniverseAct::Map => 2,
            UniverseAct::Mouth => 3,
            UniverseAct::ConversionOut => 4,
            UniverseAct::Manifest => 5,
            UniverseAct::Statement => 6,
        }
    }

    /// The act a byte names, or none.
    pub fn of(b: u8) -> Option<UniverseAct> {
        UniverseAct::ALL.into_iter().find(|a| a.byte() == b)
    }

    /// WHETHER A KERNEL-STANDING CLAUSE GOVERNS THE ACT: the clause's law
    /// form states the objects the act carries. LAW-COORDINATE
    /// (`FORMS-COORDINATE`) governs the acts that state TUniversal
    /// coordinates, `origin/h:S:G:T:H:K`: an anchor (its point) and a map
    /// (its mouths' points). A clause that governs no act bounds nothing at
    /// the tube.
    pub fn governed_by(self, law: &str) -> bool {
        law == forms::coordinate::LAW_ID && matches!(self, UniverseAct::Anchor | UniverseAct::Map)
    }
}

/// WHY THE UNIVERSAL CORE TUBE REFUSED A UNIVERSE'S ACT.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Boundary {
    /// CONCORDANCE 4.1 — the act reached the hypha other than from the
    /// domain's sitting reserve, and was not carried in the reserve's act:
    /// no past cone through the vertex places it in the universe's cone.
    NotThroughCore { act: UniverseAct, domain: [u8; 32] },
    /// CONCORDANCE §0 — the domain's sitting reserve does not converse on
    /// the network clause `clause` of the register served; `reading` is
    /// where it stands on it (unread, or colliding with the residue).
    NotSignatory { act: UniverseAct, domain: [u8; 32], clause: ConcordanceClauseId, reading: Box<ClauseReading> },
    /// CONCORDANCE §0 — the reserve vetoed the kernel clause `clause`,
    /// which governs this act: the veto bounds what the clause governs,
    /// named, and nothing else.
    Bounded { act: UniverseAct, domain: [u8; 32], clause: ConcordanceClauseId },
    /// The reserve's alignment could not be read whole (`StrokeSpent`, or
    /// a register the hypha cannot read): the crossing waits, neither
    /// admitted nor refused on a reading cut short.
    Unaligned { act: UniverseAct, domain: [u8; 32], why: AlignmentRefusal },
}

impl Boundary {
    /// The act refused.
    pub fn act(&self) -> UniverseAct {
        match self {
            Boundary::NotThroughCore { act, .. } | Boundary::NotSignatory { act, .. } | Boundary::Bounded { act, .. } | Boundary::Unaligned { act, .. } => *act,
        }
    }

    /// The domain whose act it was.
    pub fn domain(&self) -> [u8; 32] {
        match self {
            Boundary::NotThroughCore { domain, .. }
            | Boundary::NotSignatory { domain, .. }
            | Boundary::Bounded { domain, .. }
            | Boundary::Unaligned { domain, .. } => *domain,
        }
    }

    /// THE REFUSAL WRITTEN: `[tag][act][domain 32]`, then, by tag: `0`
    /// nothing; `1` the clause and the reading (`0` converses, `1` vetoed,
    /// `2` collides, `3` unread; a stamp, and for a collision the refusal
    /// as a signing verdict writes it); `2` the clause; `3` `[0]` the
    /// stroke spent or `[1][clause]` a register unread.
    pub fn put(&self, out: &mut Vec<u8>) {
        let tag = match self {
            Boundary::NotThroughCore { .. } => 0,
            Boundary::NotSignatory { .. } => 1,
            Boundary::Bounded { .. } => 2,
            Boundary::Unaligned { .. } => 3,
        };
        out.push(tag);
        out.push(self.act().byte());
        out.extend_from_slice(&self.domain());
        match self {
            Boundary::NotThroughCore { .. } => {}
            Boundary::NotSignatory { clause, reading, .. } => {
                put_clause_id(out, clause);
                put_reading(out, reading);
            }
            Boundary::Bounded { clause, .. } => put_clause_id(out, clause),
            Boundary::Unaligned { why: AlignmentRefusal::StrokeSpent, .. } => out.push(0),
            Boundary::Unaligned { why: AlignmentRefusal::Unregistered { id }, .. } => {
                out.push(1);
                put_clause_id(out, id);
            }
        }
    }

    /// The refusal read back from `p` on, or none.
    pub fn take(b: &[u8], p: &mut usize) -> Option<Boundary> {
        let tag = byte(b, p)?;
        let act = UniverseAct::of(byte(b, p)?)?;
        let domain: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        Some(match tag {
            0 => Boundary::NotThroughCore { act, domain },
            1 => Boundary::NotSignatory { act, domain, clause: take_clause_id(b, p)?, reading: Box::new(take_reading(b, p)?) },
            2 => Boundary::Bounded { act, domain, clause: take_clause_id(b, p)? },
            3 => Boundary::Unaligned {
                act,
                domain,
                why: match byte(b, p)? {
                    0 => AlignmentRefusal::StrokeSpent,
                    1 => AlignmentRefusal::Unregistered { id: take_clause_id(b, p)? },
                    _ => return None,
                },
            },
            _ => return None,
        })
    }
}

fn byte(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

fn put_reading(out: &mut Vec<u8>, r: &ClauseReading) {
    match r {
        ClauseReading::Converses { at } => {
            out.push(0);
            at.put_canonical(out);
        }
        ClauseReading::Vetoed { at } => {
            out.push(1);
            at.put_canonical(out);
        }
        ClauseReading::Collides { at, residue } => {
            out.push(2);
            at.put_canonical(out);
            put_verdict(out, &Err((**residue).clone()));
        }
        ClauseReading::Unread => out.push(3),
    }
}

fn take_reading(b: &[u8], p: &mut usize) -> Option<ClauseReading> {
    Some(match byte(b, p)? {
        0 => ClauseReading::Converses { at: Stamp::take_canonical(b, p)? },
        1 => ClauseReading::Vetoed { at: Stamp::take_canonical(b, p)? },
        2 => {
            let at = Stamp::take_canonical(b, p)?;
            let Err(residue) = take_verdict_at(b, p)? else { return None };
            ClauseReading::Collides { at, residue: Box::new(residue) }
        }
        3 => ClauseReading::Unread,
        _ => return None,
    })
}
