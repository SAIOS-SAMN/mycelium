// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.185 (proposed 2026-11-16) — A HYPHA'S ATTACHMENT AS AN ADJUNCTION OF
// THE COVER, ITS DETACHMENT AS A RETRACTION.
//
//! The hyphas cover the network: each is an open set, an overlap is what
//! two both witness, and the nerve is a vertex per hypha and an edge per
//! overlap, each edge carrying the unit motion `g_ij` that takes `i`'s
//! readings onto `j`'s (78.173 §8.5). These are the claims the court checks
//! when a hypha attaches or detaches (MU-RECURSION 15.6, ch. 14):
//!
//! ```text
//! simplex   every two nodes of the link overlap already        else the link opens a cycle: a new class
//! cocycle   g_ij · g_jn = g_in on every triangle (i, j, n)      the readings agree wherever they overlap
//! tether    some link stands at rest: R_p = −σ_p on each plane  the Hopf fibres linked, never touching
//! retract   the nerve without n stays connected                 no class orphaned
//! ```
//!
//! A link that is a simplex makes the new vertex's star a cone over it,
//! which is contractible, so no class is created and none lost. A link two
//! of whose nodes do not overlap opens a new cycle, and is refused by naming
//! that pair, never by a count.

use crate::turning::{compose, reversed, same, TurningRefusal};
use crate::Q;
use algebra::Clifford;

/// A HYPHA'S KEY: the vertex's name.
pub type Key = [u8; 32];

/// ONE OVERLAP: nodes `a` and `b` (by key) and the unit motion carrying
/// `a`'s readings onto `b`'s.
#[derive(Clone, Debug)]
pub struct Overlap {
    pub a: Key,
    pub b: Key,
    pub g: Clifford,
}

/// ONE TETHER an entrant claims: the node it links, the motion carrying that
/// node's readings onto the entrant's, and the planes of their relative
/// phase state, each `(phase R_p, pole σ_p)`.
#[derive(Clone, Debug)]
pub struct Tether {
    pub node: Key,
    pub g: Clifford,
    pub planes: Vec<(Q, Q)>,
}

/// THE NERVE: the hyphas and their overlaps.
#[derive(Clone, Debug, Default)]
pub struct Nerve {
    pub nodes: Vec<Key>,
    pub overlaps: Vec<Overlap>,
}

/// WHY AN ADJUNCTION OR A RETRACTION IS REFUSED, by name.
#[derive(Clone, Debug)]
pub enum CoverRefusal {
    /// The entrant is in the nerve already, or the retracted node is not.
    Held(Key),
    /// A tether names a node the nerve does not hold.
    NoNode(Key),
    /// Two nodes of the link do not overlap: the adjunction would open a
    /// cycle, a new class (ch. 14 runs first).
    OpensCycle { a: Key, b: Key },
    /// The readings disagree on the triangle `(a, b, n)`: `g_ab · g_bn` is
    /// not `g_an`, both stated.
    Cocycle { a: Key, b: Key, around: Box<Clifford>, stated: Box<Clifford> },
    /// No tether stands at rest: the entrant's fibre links none.
    Untethered,
    /// The nerve without the node falls apart: a class orphaned.
    Disconnects,
    /// A motion that is not a unit motion, or two of different signatures.
    Motion(Box<TurningRefusal>),
}

impl From<TurningRefusal> for CoverRefusal {
    fn from(r: TurningRefusal) -> CoverRefusal {
        CoverRefusal::Motion(Box::new(r))
    }
}

impl Nerve {
    /// The motion carrying `a`'s readings onto `b`'s, read off an overlap
    /// stated either way (its reverse where stated from `b`).
    fn carry(&self, a: &Key, b: &Key) -> Result<Option<Clifford>, TurningRefusal> {
        for o in &self.overlaps {
            if o.a == *a && o.b == *b {
                return Ok(Some(o.g.clone()));
            }
            if o.a == *b && o.b == *a {
                return Ok(Some(reversed(&o.g)?));
            }
        }
        Ok(None)
    }

    /// 78.185 §2 — THE ADJUNCTION of `n` by its tethers: admitted where the
    /// link is a simplex of the nerve, the cocycle closes on every triangle
    /// it forms, and some tether stands at rest. Answers the nerve with `n`
    /// and its overlaps adjoined.
    pub fn adjoin(&self, n: Key, link: &[Tether]) -> Result<Nerve, CoverRefusal> {
        if self.nodes.contains(&n) {
            return Err(CoverRefusal::Held(n));
        }
        for t in link {
            if !self.nodes.contains(&t.node) {
                return Err(CoverRefusal::NoNode(t.node));
            }
            // A unit motion: composing it with its reverse reads as one.
            compose(&t.g, &reversed(&t.g)?)?;
        }
        for (i, ti) in link.iter().enumerate() {
            for tj in link.iter().skip(i + 1) {
                let Some(g_ij) = self.carry(&ti.node, &tj.node)? else { return Err(CoverRefusal::OpensCycle { a: ti.node, b: tj.node }) };
                // i → j → n against i → n: g_in stated, g_ij · g_jn read.
                let around = compose(&g_ij, &tj.g)?;
                if !same(&around, &ti.g) {
                    return Err(CoverRefusal::Cocycle { a: ti.node, b: tj.node, around: Box::new(around), stated: Box::new(ti.g.clone()) });
                }
            }
        }
        let at_rest = |t: &Tether| !t.planes.is_empty() && t.planes.iter().all(|(phase, pole)| *phase == -pole.clone());
        if !link.iter().any(at_rest) {
            return Err(CoverRefusal::Untethered);
        }
        let mut out = self.clone();
        out.nodes.push(n);
        out.overlaps.extend(link.iter().map(|t| Overlap { a: t.node, b: n, g: t.g.clone() }));
        Ok(out)
    }

    /// 78.185 §4 — THE RETRACTION of `n`: admitted where the nerve without it
    /// stays connected. What it witnessed stays on the ledger; its overlaps
    /// leave the nerve.
    pub fn retract(&self, n: &Key) -> Result<Nerve, CoverRefusal> {
        if !self.nodes.contains(n) {
            return Err(CoverRefusal::Held(*n));
        }
        let nodes: Vec<Key> = self.nodes.iter().filter(|k| *k != n).copied().collect();
        let overlaps: Vec<Overlap> = self.overlaps.iter().filter(|o| o.a != *n && o.b != *n).cloned().collect();
        // Connected is a property of the whole, so the walk may start from
        // any node; it is bounded by the nodes held, each entered once.
        let mut reached: Vec<Key> = nodes.iter().take(1).copied().collect();
        let mut at = 0;
        while let Some(k) = reached.get(at).copied() {
            for o in &overlaps {
                let next = match (o.a == k, o.b == k) {
                    (true, _) => o.b,
                    (_, true) => o.a,
                    _ => continue,
                };
                if !reached.contains(&next) {
                    reached.push(next);
                }
            }
            at += 1;
        }
        if reached.len() != nodes.len() {
            return Err(CoverRefusal::Disconnects);
        }
        Ok(Nerve { nodes, overlaps })
    }
}
