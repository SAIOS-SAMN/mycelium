// 54.5 Road (d) — THE RECEIPTS ENVELOPE: what a mind ATTESTS about a form,
// carried beside the coordinate that NAMES it.
//
//! An address is a form's coordinates written out (law 2) and it carries
//! nothing else. Three things a court reads are not coordinates of the
//! geometry, and `evidence_is_not_identity` says so deliberately:
//!
//!   **The citations a vertex factored through.** Two minds reaching one
//!   rotor by different derivations must reach ONE address (§1.5, 13.10),
//!   so the presentation cannot be in the key. `derivations_compose`,
//!   `generator_pay` and `unruled_citation` all read it.
//!
//!   **The pivots a crystallization rests on** (A20). `replay_certificates`
//!   un-pivots each against the axis it names.
//!
//!   **The filing**, which moved to `Element.filing` at `df1eeaa` and is
//!   the one of the three already landed.
//!
//! All three rode `Element.description` — the tape — which is why the tape
//! could not go. This is where they ride instead: per mind, signed with the
//! word that makes them, indexed by `Address × Signer`, and already crossing
//! the wire with the element.
//!
//! # The nerve is GRADED, and that is not a convenience
//!
//! A citation is a 1-simplex of `N(𝒰)`. Recording only `⟨A_i, A_j, φ⟩`
//! would make the nerve a directed graph — generators with no relations, a
//! free groupoid with no topology — and cohomological homotopy does not
//! live in 1-morphisms. The Čech differential
//!
//! ```text
//!     (δφ)_ijk = φ_jk · φ_ik⁻¹ · φ_ij = Ω_ijk
//! ```
//!
//! evaluates on a TRIPLE overlap, and with no carrier for a 2-simplex that
//! composite has nowhere to be registered. `cech::complex::loops_of` can
//! already produce `Fill::Congruent` at a triple overlap and nothing
//! converts it into a coordinate of the cover: `H²` has a detector and no
//! carrier. [`NerveTriangle`] is that carrier, so the structure does not
//! have to be redesigned the moment an attachment exists.
//!
//! # A 1-simplex carries the FULL affine transition
//!
//! An inter-chart transition is a space-group element, not a rotation:
//!
//! ```text
//!     g_ij = T(τ_ij) · R_ij
//! ```
//!
//! `R_ij ∈ Spin(4,0)` is the bend, `τ_ij` the relative displacement of the
//! two charts' origins. An edge carrying a bare rotor forces every chart
//! overlap to be CONCENTRIC and throws away the translational solder form —
//! the same dimensional collapse that made every intra-form loop read flat.
//! So an edge carries `placement` (the rotor half) and `carriage` (`D(ρ)·T(t)`,
//! the placement and dilation half), which is exactly what `Vertex::Glued`
//! already holds and exactly how [`crate::transducer::cech::Simplex`] splits
//! the same object.
//!
//! The CARTAN SPLIT of that transition — where it bends and how far it slips
//! — is [`NerveEdge::screw`] and is NOT a field. It is a function of `R` and
//! `t` (`screw_of` solves `(I − R)p = t` by the normal equations, exactly
//! over ℚ), and law 12 keeps a theorem rather than a fact stored beside the
//! coordinates it follows from. The same holds for a triangle's holonomy,
//! its net slip, and its `Fill`: all three are readings of the three edges,
//! and all three are readers here.
//!
//! # What is NOT derivable, and is therefore carried
//!
//! A triangle's EXISTENCE is derivable — three edges pairwise present is a
//! triple overlap. What is not derivable is whether a 2-CELL WAS ATTACHED
//! along it: that is an act of a mind against a budget, the inter-form
//! analogue of `open_axis_on`, and it pays for a dimension under T21(iii).
//! So [`NerveTriangle::attached`] is the one stored field on a 2-simplex.

use crate::transducer::address::Address;
use crate::transducer::alphabet::{Cited, Derivation, Rotor, Word};
use crate::transducer::element::{Screw, Versor};
use crate::transducer::sentence::PivotCertificate;
use crate::transducer::stalk::Invariant;

/// A 1-SIMPLEX OF THE NERVE: one chart citing another, with the affine
/// transition it is embedded under.
///
/// It is `Vertex::Glued` with the target address moved out of the key and
/// into the ledger. Every other field is the citation's own bounded
/// coordinate and stays in `form_vertex_key`, which is why [`Self::host`]
/// can re-attach an edge to its vertex by COORDINATE and never by index
/// (rule 15).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NerveEdge {
    /// `A_j` — the cited form. This is the byte run that leaves the key.
    pub target: Address,
    /// The cited form's rank: what KIND of chart is being cited.
    pub chart: Invariant,
    /// μ⃗ of the cited form as it was cited. Not keyed, and not re-derivable
    /// without the cited form in hand.
    pub magnitude: [u64; 4],
    /// The cited form's depth WHEN IT WAS CITED. A cited form goes on being
    /// cited after, so its depth moves and this coordinate does not — which
    /// is why 54.2 deleted the court's re-derivation of it.
    pub depth: u16,
    /// `R_ij` — the rotational half of the transition, over all six planes
    /// of `Cl(4,0)`.
    pub placement: Word,
    /// `D(ρ)·T(t)` — the translation and dilation half, in `Cl(3,1)`. A
    /// conformal versor (A4): `O(1)` per citation whatever the cited form's
    /// size, and exactly invertible by its reversion, so the incidence is a
    /// groupoid edge — `φ` one way and `φ̃` the other.
    pub carriage: Box<Versor>,
}

impl NerveEdge {
    /// THE SITE IN THE CITING FORM, by its own coordinates (A21) — the
    /// bytes `form_vertex_key`'s `Glued` arm keeps once the target address
    /// leaves it.
    ///
    /// An edge re-attaches to its vertex by this key. It is NOT the vertex's
    /// index: the vertex list is canonically ordered, but an ordering is a
    /// position and a position is not a coordinate (rule 15), and an edge
    /// ledger keyed by position would break under any re-ordering the form
    /// is entitled to.
    pub fn host(&self) -> Address {
        let mut buf = vec![2u8];
        self.placement.put_canonical(&mut buf);
        buf.extend_from_slice(&self.depth.to_le_bytes());
        self.carriage.put_canonical(&mut buf);
        crate::transducer::address::put_bytes(&mut buf, self.chart.key().as_bytes());
        Address::new(buf)
    }

    /// THE CARTAN SPLIT of this transition: the point the bend holds still,
    /// and the translation no choice of origin absorbs.
    ///
    /// Derived, never stored (law 12). `None` where the reading cannot be
    /// taken in this chart — a dilation, or a turn that carries the base off
    /// itself — which is a refusal and never a zero slip (rule 11).
    pub fn screw(&self) -> Option<Screw> {
        crate::transducer::element::screw_of(&self.placement.rotor(), &self.carriage)
    }

    /// `g_ij = T(τ)·R` as the two halves a composite needs. The rotor and
    /// the carriage compose in their own algebras and are never mixed.
    pub fn transition(&self) -> (Rotor, Versor) {
        (self.placement.rotor(), (*self.carriage).clone())
    }

    /// EVERY LENGTH HERE IS A `u32`, AND THAT IS THE FIX FOR A REAL
    /// REFUSAL. `put_addr` is the WIRE embedding — `len16 + bytes` — and it
    /// refuses an address over 65 535 bytes rather than truncating it. A
    /// form's address is its coordinates written out, and a 1 655-vertex
    /// form's address measures 10 MB, so `put_addr` would have refused the
    /// serialization of every citation of a real form.
    ///
    /// Refused, not corrupted — but the failure would have surfaced as an
    /// envelope that did not write, read back as a word attesting NOTHING.
    /// That is the empty-carrier reading that misled five times on
    /// 2026-09-20, and it is designed out here rather than caught later:
    /// `put_bytes` is the KEY-FIELD embedding (u32) and every address in
    /// this envelope is a key field.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        crate::transducer::address::put_bytes(out, self.target.as_bytes());
        crate::transducer::address::put_bytes(out, self.chart.key().as_bytes());
        for m in &self.magnitude {
            out.extend_from_slice(&m.to_le_bytes());
        }
        out.extend_from_slice(&self.depth.to_le_bytes());
        self.placement.to_bytes(out);
        self.carriage.to_bytes(out);
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<NerveEdge> {
        let target = Address::new(crate::transducer::address::take_bytes(b, pos)?);
        let chart = Invariant::from_key(&crate::transducer::address::take_bytes(b, pos)?)?;
        let mut magnitude = [0u64; 4];
        for m in &mut magnitude {
            *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        let depth = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
        *pos += 2;
        let placement = Word::from_bytes(b, pos)?;
        let carriage = Box::new(Versor::from_bytes(b, pos)?);
        Some(NerveEdge { target, chart, magnitude, depth, placement, carriage })
    }
}

/// A 2-SIMPLEX OF THE NERVE: a triple overlap, and whether a cell has been
/// attached along it.
///
/// The three edges are named by their index in the envelope's own `nerve`
/// list, which is this envelope's internal ordering and not a coordinate of
/// anything outside it.
///
/// EVERYTHING ELSE IS A READING of those three edges — the holonomy `Ω`, the
/// net slip around the circuit, and the `Fill` that classifies it — so none
/// of them is a field (law 12). What a triangle CARRIES is the attachment:
/// under T21(iii) a dimension is never declared, it is paid for by retiring
/// an obstruction, and attaching a 2-cell along a resonant `Ω` kills a class
/// in `H²` and yields an inter-form axis. That act is a mind's, against a
/// budget, and does not factor through the geometry.
///
/// NOTHING POPULATES THIS YET, and it is not pretending to. `cech::classify`
/// can already answer `Fill::Congruent` at a triple overlap; the attachment
/// that would convert one into a coordinate of the cover does not exist. The
/// type is here so the envelope is a graded simplicial structure from the
/// start rather than a 1-skeleton that has to be redesigned around.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NerveTriangle {
    /// `e_ij, e_jk, e_ki` by index into [`ReceiptsEnvelope::nerve`].
    pub edges: [u32; 3],
    /// The axis the attachment opened, where a 2-cell was attached.
    pub attached: Option<Box<Versor>>,
}

impl NerveTriangle {
    /// `Ω_ijk = g_ki ∘ g_jk ∘ g_ij`, both halves composed in their own
    /// algebras. `None` when an edge index names nothing.
    pub fn holonomy(&self, nerve: &[NerveEdge]) -> Option<(Rotor, Versor)> {
        let mut r = Rotor::identity();
        let mut c = crate::transducer::element::identity_versor();
        for &e in &self.edges {
            let (er, ec) = nerve.get(e as usize)?.transition();
            r = r.geo(&er);
            c = c.geo(&ec);
        }
        Some((r, c))
    }

    /// The NET DISLOCATION around the circuit: the Burgers vector of the
    /// triple overlap. `None` where the composite is not a transition this
    /// chart can split (rule 11).
    pub fn screw(&self, nerve: &[NerveEdge]) -> Option<Screw> {
        let (r, c) = self.holonomy(nerve)?;
        crate::transducer::element::screw_of(&r, &c)
    }

    /// 12.6 at a triple overlap: flat, filed at a finite period, standing,
    /// or a rotation the alphabet does not spell.
    pub fn fill(&self, nerve: &[NerveEdge]) -> Option<crate::transducer::cech::Fill> {
        let (r, c) = self.holonomy(nerve)?;
        Some(crate::transducer::cech::classify(
            &r,
            &c,
            crate::transducer::sentence::CycleType::of(&[]),
        ))
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        for e in self.edges {
            out.extend_from_slice(&e.to_le_bytes());
        }
        match &self.attached {
            Some(v) => {
                out.push(1);
                v.to_bytes(out);
            }
            None => out.push(0),
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<NerveTriangle> {
        let mut edges = [0u32; 3];
        for e in &mut edges {
            *e = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
        }
        let attached = match *b.get(*pos)? {
            0 => {
                *pos += 1;
                None
            }
            1 => {
                *pos += 1;
                Some(Box::new(Versor::from_bytes(b, pos)?))
            }
            _ => return None,
        };
        Some(NerveTriangle { edges, attached })
    }
}

/// ONE VERTEX'S DERIVATION, as evidence rather than as identity.
///
/// The vertex is named by its own address, which §1.5 makes the canonical
/// ROTOR — so the cite re-attaches to the vertex the key rebuilt, and two
/// minds whose derivations differ still reach one address.
///
/// The RESIDUE travels because `verify_with` needs it: the check is
/// `∏ g^e · r ≟ rotor`, the rotor is the key and the generators are here, so
/// without `r` the third term is missing and the identity would stand in for
/// it — which is a scalar collapse with a different name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivationCite {
    /// The `Word` vertex in the citing form.
    pub vertex: Address,
    /// The generators composed, in application order.
    pub generators: Vec<Cited>,
    /// What no held generator reached, in letters.
    pub residue: Word,
}

impl DerivationCite {
    /// THE DERIVATION REASSEMBLED against the action the key carries. The
    /// rotor comes from the FORM — it is the vertex's coordinate — and the
    /// citations from here, so neither side can forge the other: a mind that
    /// lies about its citations is caught by `verify_with` composing them
    /// against a rotor it did not write.
    pub fn against(&self, carried: &Rotor) -> Derivation {
        Derivation::factored(self.generators.clone(), self.residue.clone(), carried.clone())
    }

    /// `u32` lengths and a `u64` count, for [`NerveEdge::to_bytes`]'s
    /// reason: a cited form's address is not bounded by 64 KB.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        crate::transducer::address::put_bytes(out, self.vertex.as_bytes());
        out.extend_from_slice(&(self.generators.len() as u64).to_le_bytes());
        for (form, vertex, e) in &self.generators {
            crate::transducer::address::put_bytes(out, form.as_bytes());
            crate::transducer::address::put_bytes(out, vertex.as_bytes());
            out.extend_from_slice(&e.to_le_bytes());
        }
        self.residue.to_bytes(out);
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<DerivationCite> {
        let vertex = Address::new(crate::transducer::address::take_bytes(b, pos)?);
        let n = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
        *pos += 8;
        // The loop's boundary is the tape's own bytes (rule 10): every
        // iteration consumes two length-prefixed addresses and an i64, and a
        // byte that is not there refuses the whole read. Nothing is reserved
        // on the strength of the count, so a malformed count allocates
        // nothing and decides nothing (rules 2, 11).
        let mut generators = Vec::new();
        for _ in 0..n {
            let form = Address::new(crate::transducer::address::take_bytes(b, pos)?);
            let vx = Address::new(crate::transducer::address::take_bytes(b, pos)?);
            let e = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
            generators.push((form, vx, e));
        }
        let residue = Word::from_bytes(b, pos)?;
        Some(DerivationCite { vertex, generators, residue })
    }
}

/// THE ENVELOPE: the graded nerve incidence this word stands in, and the
/// evidence a court replays it with.
///
/// It rides the element — `Element.receipts` — because an element is a word
/// AS ONE MIND MINTED IT, which is the smallest carrier that has both
/// indices of `Address × Signer`. The form is geometry and is the same
/// complex whatever mind holds it; none of this factors through it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReceiptsEnvelope {
    /// 1-simplices: the citations, with their transitions.
    pub nerve: Vec<NerveEdge>,
    /// 2-simplices: triple overlaps with a cell attached.
    pub triangles: Vec<NerveTriangle>,
    /// The derivations, one per citing vertex.
    pub derivations: Vec<DerivationCite>,
    /// The pivots this crystallization rests on (A20).
    pub certificates: Vec<PivotCertificate>,
}

impl ReceiptsEnvelope {
    /// THE ENVELOPE A FORM LEAVES, taken at the mint (54.5 Road (d), B2).
    ///
    /// The mint has the form in hand, so this is where the three evidential
    /// carriers come off it — once, at the one place a word is made — rather
    /// than being read back off a tape at every court that hears it.
    ///
    /// EVERY LIST IS IN ITS OWN CONTENT ORDER (rule 15). The nerve is sorted
    /// by each edge's site key and the derivations by their vertex address,
    /// so two minds that walked a form's vertices in different orders emit
    /// the same envelope, and a `NerveTriangle`'s edge indices name the same
    /// edges on every node. `certificates` comes off a `Keyed`, which is
    /// already in key order.
    ///
    /// NO TRIANGLES ARE EMITTED. A triple overlap is derivable from the
    /// edges; what a 2-simplex carries is an ATTACHMENT, and nothing attaches
    /// an inter-form 2-cell yet. Emitting empty triangles for every triple
    /// would be a coordinate standing for no theorem (T21(iii)).
    pub fn of_form(f: &crate::transducer::sentence::Form) -> ReceiptsEnvelope {
        use crate::transducer::sentence::Vertex;
        let mut env = ReceiptsEnvelope::default();
        for v in &f.vertices {
            match v {
                Vertex::Glued { address, chart, magnitude, depth, placement, carriage } => {
                    env.nerve.push(NerveEdge {
                        target: address.clone(),
                        chart: chart.clone(),
                        magnitude: *magnitude,
                        depth: *depth,
                        placement: placement.clone(),
                        carriage: carriage.clone(),
                    });
                }
                // A SPELLED VERTEX LEAVES NO CITE. Its whole word is the
                // residue and the key carries the rotor, so an entry for it
                // would be evidence of nothing (law 12).
                Vertex::Word { address, derivation, .. } if !derivation.is_spelled() => {
                    env.derivations.push(DerivationCite {
                        vertex: address.clone(),
                        generators: derivation.generators.clone(),
                        residue: derivation.residue.clone(),
                    });
                }
                _ => {}
            }
        }
        // THE ORDER IS THE TRIE'S, NOT A SORT'S (rule 15, law 2). A `sort_by`
        // asks a comparator to decide an order; a `Keyed` IS the order,
        // because a PATRICIA trie walks its keys in their own byte order and
        // the key is the object's coordinates written out. Two minds that
        // walked a form's vertices differently reach one listing without
        // anyone choosing a comparison.
        let mut by_site: crate::network::nerve::Keyed<NerveEdge> =
            crate::network::nerve::Keyed::new();
        for e in env.nerve.drain(..) {
            by_site.insert(e.host(), e);
        }
        env.nerve = by_site.scoped().map(|(_, e)| e.clone()).collect();
        let mut by_vertex: crate::network::nerve::Keyed<DerivationCite> =
            crate::network::nerve::Keyed::new();
        for d in env.derivations.drain(..) {
            by_vertex.insert(d.vertex.clone(), d);
        }
        env.derivations = by_vertex.scoped().map(|(_, d)| d.clone()).collect();
        env.certificates = f.certificates.scoped().map(|(_, c)| c.clone()).collect();
        env
    }

    pub fn is_empty(&self) -> bool {
        self.nerve.is_empty()
            && self.triangles.is_empty()
            && self.derivations.is_empty()
            && self.certificates.is_empty()
    }

    /// The edge at a citation site, by the site's own coordinates.
    pub fn edge_at(&self, host: &Address) -> Option<&NerveEdge> {
        self.nerve.iter().find(|e| &e.host() == host)
    }

    /// The derivation cite for a vertex address.
    pub fn cite_for(&self, vertex: &[u8]) -> Option<&DerivationCite> {
        self.derivations.iter().find(|d| d.vertex.as_bytes() == vertex)
    }

    /// EVERY FORM THIS WORD CITES, at both levels (26 as ruled 2026-09-10):
    /// a chart citing another chart, and a vertex factored through a
    /// generator inside one. They are one mechanism at two levels and the
    /// economy counts them the same way, so they are read together.
    ///
    /// Duplicates are kept. A form cited by two vertices is two citations
    /// and one USE; collapsing them here would decide that at the reading
    /// instead of at the ledger that keys by citer address, which is where
    /// `use_credit` actually counts.
    pub fn cited_addresses(&self) -> Vec<Address> {
        let mut out = Vec::new();
        for e in &self.nerve {
            out.push(e.target.clone());
        }
        for cite in &self.derivations {
            for (form, _, _) in &cite.generators {
                out.push(form.clone());
            }
        }
        out
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.nerve.len() as u64).to_le_bytes());
        for e in &self.nerve {
            e.to_bytes(out);
        }
        out.extend_from_slice(&(self.triangles.len() as u64).to_le_bytes());
        for t in &self.triangles {
            t.to_bytes(out);
        }
        out.extend_from_slice(&(self.derivations.len() as u64).to_le_bytes());
        for d in &self.derivations {
            d.to_bytes(out);
        }
        out.extend_from_slice(&(self.certificates.len() as u64).to_le_bytes());
        for c in &self.certificates {
            c.to_bytes(out);
        }
    }

    /// THE ENVELOPE AS A SELF-CONTAINED BLOCK: `len u32 ‖ bytes`.
    ///
    /// It is FRAMED so that the block's own length bounds the parse: a
    /// malformed envelope refuses the record instead of consuming the fields
    /// written after it, and a later field can be added to the envelope
    /// without moving anything else on the record.
    ///
    /// THE WRITER IS TOTAL, and that is deliberate. It used to return
    /// `Option` and fall back to a zero-length block, which made "the
    /// envelope did not serialize" read back as "this word attests nothing"
    /// — the empty-carrier collapse, designed in. With `u32` lengths and
    /// `u64` counts the only unwritable envelope is one holding more than
    /// 2⁶⁴ entries, which cannot be constructed, so there is no refusal path
    /// left to confuse with emptiness. An EMPTY envelope writes four zero
    /// counts and is 32 bytes, never 0.
    pub fn to_block(&self, out: &mut Vec<u8>) {
        let mut body = Vec::new();
        self.to_bytes(&mut body);
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
    }

    /// Read a framed envelope. A ZERO-LENGTH block is refused rather than
    /// read as the empty envelope: the writer never produces one, so its
    /// presence is a record this build cannot read and admitting it would
    /// restore exactly the collapse [`Self::to_block`] removes (rule 11).
    pub fn from_block(b: &[u8], pos: &mut usize) -> Option<ReceiptsEnvelope> {
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        let body = b.get(*pos..*pos + n)?;
        *pos += n;
        let mut p = 0usize;
        ReceiptsEnvelope::from_bytes(body, &mut p)
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<ReceiptsEnvelope> {
        let count = |b: &[u8], p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let mut env = ReceiptsEnvelope::default();
        let n = count(b, pos)?;
        for _ in 0..n {
            env.nerve.push(NerveEdge::from_bytes(b, pos)?);
        }
        let n = count(b, pos)?;
        for _ in 0..n {
            env.triangles.push(NerveTriangle::from_bytes(b, pos)?);
        }
        let n = count(b, pos)?;
        for _ in 0..n {
            env.derivations.push(DerivationCite::from_bytes(b, pos)?);
        }
        let n = count(b, pos)?;
        for _ in 0..n {
            env.certificates.push(PivotCertificate::from_bytes(b, pos)?);
        }
        Some(env)
    }
}
