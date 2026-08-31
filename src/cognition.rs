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
    /// The demand is the COMPOSITION of two installed generators (a depth-2
    /// sentence) — the algebra already SPANS it, so nothing is minted. The two
    /// cited basis indices (R6: reach new transforms by composition).
    Composed(usize, usize),
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
        // R6 — COMPOSITIONAL RECALL: before minting, ask whether the algebra already
        // SPANS this demand — i.e. it is the composition of installed generators (a
        // depth-2 sentence). If so, recognise it as that sentence and mint NOTHING;
        // both cited generators are USED, so their discoverers are paid (the corpus
        // this composition unlocked). "Don't mint what the basis already spans."
        if g.solution().is_some() {
            if let Some((i, j)) = self.factor_pair(&g) {
                self.treasury.settle_recall(voice);
                self.treasury.settle_use(self.discoverers[i]);
                self.treasury.settle_use(self.discoverers[j]);
                self.physics.syntropy(lanes); // a spanned sentence builds order
                return Outcome::Composed(i, j);
            }
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

    /// DEDUCE whether `g` factors as the composition `basis[i] ∘ basis[j]` of two
    /// installed GROUP generators (a depth-2 Cayley spelling). Exact — matched by
    /// realized action, no fuzzy search — and restricted to invertible (group)
    /// generators so the realization is injective on the description. Bounded O(n²)
    /// over the basis; deeper depths iterate the same reduction (future).
    fn factor_pair(&self, g: &Element) -> Option<(usize, usize)> {
        let groupish: Vec<usize> = (0..self.basis.len()).filter(|&i| self.basis[i].solution().is_some()).collect();
        for &i in &groupish {
            for &j in &groupish {
                if self.basis[i].compose(&self.basis[j]).same_action(g) {
                    return Some((i, j));
                }
            }
        }
        None
    }

    /// A read-only view of the germinated basis (for the CCL display seam).
    pub fn basis(&self) -> &[Element] {
        &self.basis
    }

    /// P7 — SNAPSHOT the durable cognitive state (the germinated basis + each
    /// germ's discoverer + the settlement ledger) so a hypha resumes its learned
    /// language and earned balances across restart, rather than re-germinating from
    /// zero. Format: `MYC1 | [n][elem_len][elem][discoverer 32]… | treasury`.
    pub fn snapshot(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"MYC1");
        out.extend_from_slice(&(self.basis.len() as u32).to_le_bytes());
        for (g, who) in self.basis.iter().zip(&self.discoverers) {
            let eb = g.to_bytes();
            out.extend_from_slice(&(eb.len() as u32).to_le_bytes());
            out.extend_from_slice(&eb);
            out.extend_from_slice(who);
        }
        out.extend_from_slice(&self.treasury.snapshot());
        out
    }

    /// P7 — RESTORE a snapshot into a fresh kernel. Returns `false` (leaving the
    /// kernel empty) on a bad magic or corruption — a corrupt ledger never crashes
    /// a boot, it just starts clean.
    pub fn restore(&mut self, b: &[u8]) -> bool {
        let mut pos = 0usize;
        if b.get(0..4) != Some(b"MYC1") {
            return false;
        }
        pos = 4;
        let read_u32 = |b: &[u8], p: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v)
        };
        let load = |kernel: &mut Kernel, b: &[u8], pos: &mut usize| -> Option<()> {
            let n = read_u32(b, pos)? as usize;
            for _ in 0..n {
                let elen = read_u32(b, pos)? as usize;
                let mut ep = *pos;
                let elem = Element::from_bytes(b, &mut ep)?;
                *pos += elen;
                let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                kernel.basis.push(elem);
                kernel.discoverers.push(who);
            }
            kernel.treasury = crate::economy::Treasury::restore(b, pos)?;
            Some(())
        };
        if load(self, b, &mut pos).is_none() {
            self.basis.clear();
            self.discoverers.clear();
            return false;
        }
        true
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
        Outcome::Composed(_, _) => 6,
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
    /// P7 — the germinated language survives restart: install two germs, snapshot,
    /// restore into a fresh kernel, and the restored kernel RECALLS them (no
    /// re-germination) with balances intact.
    #[test]
    fn the_basis_survives_a_restart() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let t3 = vec![(0, 3, 1), (0, 4, 1), (1, 3, 1)];
        let bar = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)];
        k.ingest(&a, &t3, [1; 32]);
        assert_eq!(k.ingest(&a, &t3, [2; 32]), Outcome::Installed(0)); // a motion germ
        k.ingest(&a, &bar, [1; 32]);
        assert_eq!(k.ingest(&a, &bar, [2; 32]), Outcome::Installed(1)); // a residual germ
        let balance = k.treasury().balance(&[2; 32]);
        let snap = k.snapshot();

        // A fresh hypha boots from the snapshot.
        let mut booted = Kernel::new();
        assert!(booted.restore(&snap), "clean restore");
        assert_eq!(booted.basis_len(), 2, "both germs came back");
        assert_eq!(booted.treasury().balance(&[2; 32]), balance, "earned balance survived");
        // Both are RECALLED, not re-germinated (the language persisted).
        assert_eq!(booted.ingest(&a, &t3, [9; 32]), Outcome::Recognised(0), "motion recalled");
        assert_eq!(booted.ingest(&a, &bar, [9; 32]), Outcome::Recognised(1), "residual recalled");
        assert_eq!(booted.basis_len(), 2, "no re-germination");
    }

    /// R6 — COMPOSITIONAL RECALL: install a translate(+0,+3) generator, then a
    /// translate(+0,+6) demand is DEDUCED as g₀∘g₀ (a depth-2 sentence). The algebra
    /// spans it, so NOTHING is minted (basis stays 1), and the generator's
    /// discoverer is paid for the composition's use.
    #[test]
    fn a_composite_demand_is_deduced_not_minted() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let t3 = vec![(0, 3, 1), (0, 4, 1), (1, 3, 1)]; // +0,+3
        let t6 = vec![(0, 6, 1), (0, 7, 1), (1, 6, 1)]; // +0,+6 = (+0,+3) ∘ (+0,+3)
        // Install the atom on two voices.
        assert_eq!(k.ingest(&a, &t3, [1; 32]), Outcome::GermPending);
        assert_eq!(k.ingest(&a, &t3, [2; 32]), Outcome::Installed(0));
        assert_eq!(k.basis_len(), 1);
        // The discoverer is the installing (quorum-reaching) voice.
        let before = k.treasury().balance(&[2; 32]);
        // The composite is recognised as g₀∘g₀ — no new germ minted.
        assert_eq!(k.ingest(&a, &t6, [3; 32]), Outcome::Composed(0, 0), "deduced as a sentence");
        assert_eq!(k.basis_len(), 1, "the algebra spanned it — nothing minted");
        // The cited generator's discoverer earned utility (used twice in the sentence).
        assert!(k.treasury().balance(&[2; 32]) > before, "composition pays the cited generator");
    }

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
