// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! COGNITION — the kernel loop. A demand (scene A → scene B) enters as raw
//! cells; the transducer lifts blades, the unified solve extracts one conformal
//! motor, conception classifies it against the installed basis, and a novel
//! germ installs ONLY on two-voice consilience (`consensus`). State is
//! Vec-of-relations throughout — the basis is a Vec of motors, admission and the
//! nerve are Vec-backed. No digest-keyed cognition (Anti-Flattening Law); the
//! motor digest below is a WIRE identity for matching receipts, never reasoned on.

use crate::network::consensus::{Admission, Receipt};
use crate::network::nerve::Nerve;
use crate::transducer::blade::{components, lift};
use crate::transducer::element::{self, Element};
use crate::transducer::germinate::germinate;
use crate::transducer::transform::solve_transform;

/// Mycelium's heritage floor: demands larger than this fragment (the 48% score
/// depended on decomposing large corpus). Here the topological trigger is
/// "no single motor closes the scene AND ≥2 bodies" — size-independent — with
/// the floor retained as the guard for whole-scene attempts.
pub const FRAGMENT_FLOOR: usize = 9;

/// The verdict of ingesting one demand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The motor is already an installed primitive — recall.
    Recognised(usize),
    /// A novel germ awaiting a second voice (this voice recorded).
    GermPending,
    /// A novel germ that just reached quorum and installed to the basis.
    Installed(usize),
    /// The scene had no single motor; it fragmented into per-body sub-demands,
    /// each resolved independently and recomposed (the large-corpus path).
    Fragmented(Vec<Outcome>),
    /// No motor closes the demand — an irreducible residual (a pivot).
    Pivot,
}

/// The kernel's evolving state: the installed generator basis + the germ
/// admission ledger + the Čech peer table.
#[derive(Default)]
pub struct Kernel {
    /// The germinated basis: anonymous ANASTOME elements — one uniform species
    /// (block-diagonal descriptions with a coordinate-exact word), whether a solved
    /// motion∘recolour or a residual coboundary. The kernel never branches on their
    /// "kind"; they are opaque, addressed by canonical form. English lives only in
    /// the `ccl` seam.
    basis: Vec<Element>,
    /// The discoverer (installing voice) of each basis germ, parallel to `basis`
    /// — so novelty yield flows to who germinated it, on real use.
    discoverers: Vec<[u8; 32]>,
    admission: Admission,
    pub nerve: Nerve,
    /// Polytonic kinematic pressure + torsion (per lane).
    physics: crate::physics::Physics,
    /// The settlement ledger (POW++-gated bounties, consilience splits).
    treasury: crate::economy::Treasury,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel::default()
    }

    pub fn basis_len(&self) -> usize {
        self.basis.len()
    }

    /// The polytonic kinematic pressure + torsion spectrum (per lane).
    pub fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    /// The settlement ledger (bounties, consilience, recall yields).
    pub fn treasury(&self) -> &crate::economy::Treasury {
        &self.treasury
    }

    /// Ingest a demand from `voice`: transduce → solve → conceive → admit.
    /// A whole-scene coherent motor settles directly; otherwise the demand
    /// FRAGMENTS into per-body sub-demands (bodies corresponded by invariant),
    /// each solved and admitted independently, then recomposed — the large-corpus
    /// path. An irreducible body makes the whole a pivot.
    /// Convenience: PERCEIVE (segment) then ADJUDICATE. In the mesh, perception
    /// belongs to the client (`client::perceive`) — this path is for local use
    /// and tests. The server calls `adjudicate` on the client's reported percept.
    pub fn ingest(&mut self, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: [u8; 32]) -> Outcome {
        self.adjudicate(&components(a), &components(b), voice)
    }

    /// THE JUDGE — mathematics + ANASTOME only. Given a client's PERCEPT (already
    /// segmented bodies of A and B), correspond by invariant, derive the exact
    /// element per body, and answer valid (Recognised/Installed/Fragmented) or
    /// invalid (Pivot). No sensing here — the client perceived; the judge reasons.
    pub fn adjudicate(
        &mut self,
        ca: &[Vec<(i64, i64, u8)>],
        cb: &[Vec<(i64, i64, u8)>],
        voice: [u8; 32],
    ) -> Outcome {
        if ca.is_empty() || ca.len() != cb.len() {
            self.physics.pivot([false; 4]);
            return Outcome::Pivot;
        }
        // Correspond bodies by rigid-motion invariant (zero search) — the pairing,
        // keeping the raw cells so the full description can be read per body.
        let mut ai: Vec<usize> = (0..ca.len()).collect();
        let mut bi: Vec<usize> = (0..cb.len()).collect();
        ai.sort_by(|&x, &y| lift(&ca[x], 0).invariant().cmp(&lift(&ca[y], 0).invariant()));
        bi.sort_by(|&x, &y| lift(&cb[x], 0).invariant().cmp(&lift(&cb[y], 0).invariant()));
        // Derive each body's anonymous element. A body a motor CLOSES gives a solved
        // description (from_transform). A body that RESISTS the group is NOT a
        // dead-end — its residual IS the description, so it GERMINATES as a
        // coordinate-exact coboundary germ (from_residual), a new axis of
        // perception. Only a truly empty demand (no change at all) stays a pivot.
        let mut germs: Vec<Element> = Vec::with_capacity(ca.len());
        for k in 0..ca.len() {
            let (a, b) = (&ca[ai[k]], &cb[bi[k]]);
            let e = match solve_transform(a, b) {
                Some(t) => element::from_transform(&t),
                None => match germinate(a, b) {
                    Some(rg) => element::from_residual(&rg),
                    None => {
                        self.physics.pivot([false; 4]);
                        return Outcome::Pivot;
                    }
                },
            };
            germs.push(e);
        }
        // One element closing every body is a single germ; otherwise the demand
        // fragmented into independent per-body germs.
        if germs.iter().all(|g| *g == germs[0]) {
            self.conceive_germ(germs.swap_remove(0), voice)
        } else {
            Outcome::Fragmented(germs.into_iter().map(|g| self.conceive_germ(g, voice)).collect())
        }
    }

    /// Classify one anonymous germ against the basis and run the two-voice gate.
    /// Opaque: matched by canonical address, never by "kind".
    fn conceive_germ(&mut self, g: Element, voice: [u8; 32]) -> Outcome {
        let lanes = g.lanes();
        if let Some(i) = self.basis.iter().position(|p| *p == g) {
            self.treasury.settle_recall(voice); // the recaller's small yield
            self.treasury.settle_use(self.discoverers[i]); // UTILITY → the germ's discoverer
            self.physics.syntropy(lanes); // recognition builds order (negentropy)
            return Outcome::Recognised(i);
        }
        let addr = g.address();
        if self.admission.record(Receipt { voice, germ: addr }) {
            // Two-voice consilience: POW++ gate + consilience split (anti-
            // hallucination). Installing pays NO flat novelty bounty — reward
            // flows later, on real use (see the Recognised arm).
            let _ = self.treasury.settle_install(&g, voice); // gate only
            self.treasury.settle_consilience(&self.admission.voices_of(&addr));
            self.physics.relieve(lanes);
            self.physics.syntropy(lanes); // a germinated fiber is order built
            self.basis.push(g);
            self.discoverers.push(voice);
            Outcome::Installed(self.basis.len() - 1)
        } else {
            self.physics.bump(lanes); // an open germ raises per-lane pressure
            Outcome::GermPending
        }
    }

    /// A read-only view of the germinated basis (for the CCL display seam).
    pub fn basis(&self) -> &[Element] {
        &self.basis
    }
}

// ── the ANASTOME address of a germ (canonical, opaque; transport identity) ────

/// Canonical bytes of an anonymous germ — its coordinate-exact ANASTOME form (the
/// banked word ⊕ the block-diagonal algebra; carries no English, no "kind"). This
/// is the language the hypha teaches back across the nerve; the client treats it
/// opaquely (display only).
pub fn germ_bytes(g: &Element) -> Vec<u8> {
    g.canonical()
}

/// The germ's canonical ADDRESS — a digest of its ANASTOME form. Used to match
/// consilience receipts across voices; a wire/cache identity, never a reasoning
/// operand (the kernel reasons on the element, addresses by this).
pub fn germ_address(g: &Element) -> [u8; 32] {
    g.address()
}

// ── demand wire codec (server ⇄ client) ──────────────────────────────────────

fn push_cells(out: &mut Vec<u8>, cells: &[(i64, i64, u8)]) {
    out.extend_from_slice(&(cells.len() as u32).to_le_bytes());
    for (r, c, h) in cells {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(*h);
    }
}

fn read_cells(b: &[u8], p: &mut usize) -> Option<Vec<(i64, i64, u8)>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let r = i64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        let c = i64::from_le_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
        let h = *b.get(*p + 16)?;
        *p += 17;
        out.push((r, c, h));
    }
    Some(out)
}

/// Encode a demand body: `voice(32) | scene A | scene B`.
pub fn encode_demand(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(voice);
    push_cells(&mut out, a);
    push_cells(&mut out, b);
    out
}

#[allow(clippy::type_complexity)]
pub fn decode_demand(body: &[u8]) -> Option<([u8; 32], Vec<(i64, i64, u8)>, Vec<(i64, i64, u8)>)> {
    let voice: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32;
    let a = read_cells(body, &mut p)?;
    let b = read_cells(body, &mut p)?;
    Some((voice, a, b))
}

/// CLIENT PERCEPTION — segment each scene into bodies. This is the sensing act;
/// it lives with the client, which reports the resulting percept to its hypha.
pub fn perceive(
    a: &[(i64, i64, u8)],
    b: &[(i64, i64, u8)],
) -> (Vec<Vec<(i64, i64, u8)>>, Vec<Vec<(i64, i64, u8)>>) {
    (components(a), components(b))
}

fn push_bodies(out: &mut Vec<u8>, bodies: &[Vec<(i64, i64, u8)>]) {
    out.extend_from_slice(&(bodies.len() as u32).to_le_bytes());
    for body in bodies {
        push_cells(out, body);
    }
}

fn read_bodies(b: &[u8], p: &mut usize) -> Option<Vec<Vec<(i64, i64, u8)>>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(read_cells(b, p)?);
    }
    Some(out)
}

/// Encode a client PERCEPT: `voice(32) | A bodies | B bodies` — segmented
/// bodies, not raw cells: the client already perceived.
pub fn encode_percept(
    a_bodies: &[Vec<(i64, i64, u8)>],
    b_bodies: &[Vec<(i64, i64, u8)>],
    voice: &[u8; 32],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(voice);
    push_bodies(&mut out, a_bodies);
    push_bodies(&mut out, b_bodies);
    out
}

#[allow(clippy::type_complexity)]
pub fn decode_percept(
    body: &[u8],
) -> Option<([u8; 32], Vec<Vec<(i64, i64, u8)>>, Vec<Vec<(i64, i64, u8)>>)> {
    let voice: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32;
    let a = read_bodies(body, &mut p)?;
    let b = read_bodies(body, &mut p)?;
    Some((voice, a, b))
}

/// One byte on the wire summarising an outcome (server → client reply).
pub fn outcome_code(o: &Outcome) -> u8 {
    match o {
        Outcome::Recognised(_) => 1,
        Outcome::GermPending => 2,
        Outcome::Installed(_) => 3,
        Outcome::Fragmented(_) => 5,
        Outcome::Pivot => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scene(sr: i64, sc: i64) -> Vec<(i64, i64, u8)> {
        vec![(sr, sc, 1), (sr, sc + 1, 1), (sr + 1, sc, 1)]
    }

    /// A novel translation germinates: one voice pends, a second installs it;
    /// afterward the same demand recalls.
    #[test]
    fn a_germ_installs_on_two_voices_then_recalls() {
        let mut k = Kernel::new();
        let (a, b) = (scene(0, 0), scene(0, 3)); // +0,+3 translate
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        assert_eq!(k.basis_len(), 0, "one voice does not install");
        assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::Installed(0), "two voices install");
        assert_eq!(k.basis_len(), 1);
        assert_eq!(k.ingest(&a, &b, [3; 32]), Outcome::Recognised(0), "now recalled");
    }

    /// An irreducible residual is a pivot, never installed.
    /// A shape deformation (L → bar) that NO motor closes is not a dead-end: its
    /// residual IS the description, so it germinates a coordinate-exact coboundary
    /// germ and installs on two-voice consilience (priced on USE per R3, so a
    /// one-off earns ~0 — no basis flood). The reframe made operational: germinate
    /// the description, never discard the residual.
    #[test]
    fn a_shape_deformation_germinates_a_residual() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
        let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]; // bar — no rigid motor
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "one voice awaits consilience");
        assert_eq!(k.basis_len(), 0, "not yet installed");
        assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::Installed(0), "two voices install the residual germ");
        assert_eq!(k.basis_len(), 1, "the deformation germinated a new axis");
        assert_eq!(k.ingest(&a, &b, [3; 32]), Outcome::Recognised(0), "and is recalled");
    }

    /// Two distinct objects moving by DIFFERENT motors: the whole scene has no
    /// single motor, so the demand FRAGMENTS per body and resolves each — the
    /// large-corpus path. Two voices install both germs (two distinct motors).
    #[test]
    fn independent_objects_fragment_and_resolve() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1), (5, 5, 2), (5, 6, 2), (6, 5, 2), (6, 6, 2)];
        let b = vec![(0, 3, 1), (0, 4, 1), (1, 3, 1), (5, 12, 2), (5, 13, 2), (6, 12, 2), (6, 13, 2)];
        match k.ingest(&a, &b, [1; 32]) {
            Outcome::Fragmented(parts) => assert_eq!(parts.len(), 2, "one sub-demand per body"),
            o => panic!("expected Fragmented, got {o:?}"),
        }
        match k.ingest(&a, &b, [2; 32]) {
            Outcome::Fragmented(parts) => {
                assert!(parts.iter().all(|p| matches!(p, Outcome::Installed(_))));
                assert_eq!(k.basis_len(), 2, "two distinct motors germinated");
            }
            o => panic!("expected Fragmented installs, got {o:?}"),
        }
    }

    /// The demand codec roundtrips.
    #[test]
    fn the_demand_codec_roundtrips() {
        let (a, b) = (scene(2, 2), scene(2, 5));
        let (v, ra, rb) = decode_demand(&encode_demand(&a, &b, &[9; 32])).unwrap();
        assert_eq!((v, ra, rb), ([9; 32], a, b));
    }
}
