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
//! composite has nowhere to be registered. `cech::complex::orbits_of` can
//! already produce `Fill::Congruent` at a triple overlap and nothing
//! converts it into a coordinate of the cover: `H²` has a detector and no
//! carrier. [`NerveRound`] is that carrier — and it is a CONFORMAL ROUND
//! rather than a simplex, because a junction of `k` charts is one blade
//! `p₀ ∧ … ∧ p_{k-1}` and not `C(k,3)` triangles.
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
//! the same dimensional collapse that made every intra-form orbit read flat.
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
//! So [`NerveRound::attached`] is the one stored field on a round.

use crate::transducer::address::Address;
use crate::transducer::alphabet::{Cited, Derivation, Rotor, Word, Q};
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
    /// `A_j` — the cited form. This is the coordinate that leaves the key:
    /// which form a boundary was instantiated against is the COVER's fact,
    /// not the chart's.
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

/// THE COVER IN ITS OWN ORDER, and there is ONE writer of it (law 12).
///
/// ORDERED BY SITE AND THEN BY TARGET, AND BOTH ARE THE KEY (54.7 B5a).
/// `of_form` filed edges under `host()` alone, which held while the target
/// was still in the key: two citations of different forms were two sites,
/// so one entry per site lost nothing.
///
/// Once the target leaves, two citations of two forms under one φ from
/// charts of one rank stand at ONE site — that is gauge universality and it
/// is the point — and a carrier keyed by site alone keeps ONE of them and
/// drops the rest, silently. The form would attest one incidence where it
/// has two, and the cover would be wrong in the direction nothing reports.
///
/// The site orders the listing and the target separates the edges standing
/// at it. Both are coordinates the edge already holds, and the trie's walk
/// IS their order — nothing compares anything, and no comparator decides.
///
/// IT IS CALLED FROM BOTH DECODERS. The tape writes citations in the
/// form's vertex order and the envelope files them here, so before this was
/// one function the two carriers held the same edges in two orders and the
/// pre-B6 parity oracle refused at reader 3 — correctly, since a reader
/// that took the first edge of a cover would have taken different edges
/// from the two. Order is not content, and the way to say so is one writer
/// of the order rather than two producers that happen to agree.
pub fn canonical_nerve(edges: impl IntoIterator<Item = NerveEdge>) -> Vec<NerveEdge> {
    let mut by_incidence: crate::network::nerve::Keyed<NerveEdge> =
        crate::network::nerve::Keyed::new();
    for e in edges {
        let mut k = e.host().into_vec();
        crate::transducer::address::put_bytes(&mut k, e.target.as_bytes());
        by_incidence.insert(k, e);
    }
    by_incidence.scoped().map(|(_, e)| e.clone()).collect()
}

impl NerveEdge {
    /// THE SITE IN THE CITING FORM, by its own coordinates (A21) — the key
    /// contribution of the `Vertex::Glued` this edge came off.
    ///
    /// An edge re-attaches to its vertex by this key. It is NOT the vertex's
    /// index: the vertex list is canonically ordered, but an ordering is a
    /// position and a position is not a coordinate (rule 15), and an edge
    /// ledger keyed by position would break under any re-ordering the form
    /// is entitled to.
    ///
    /// IT GOES THROUGH THE ONE WRITER, and that is law 2 and not tidiness.
    /// This hand-wrote the record — `[2]`, placement, depth, carriage,
    /// chart — beside `put_vertex`'s `Glued` arm, and the two agreed only
    /// because they were written on one afternoon to agree. What that makes
    /// the site is a byte run that MATCHES another byte run, and identity is
    /// content, never bytes (law 2; byte exactness is never what is checked,
    /// §1.5). `put_vertex` carries the same ruling in its own doc comment —
    /// two writers of one fact let the order disagree with what is written —
    /// and this was the second writer.
    ///
    /// So the edge reconstitutes the vertex it denotes, from its OWN
    /// coordinates and no placeholder, and asks the key's writer what that
    /// vertex's contribution is. There is then no agreement to maintain and
    /// nothing to assert about bytes: the site of an edge and the site of a
    /// vertex are one fact with one writer, and a coordinate the `Glued` arm
    /// starts or stops writing moves both at once.
    pub fn host(&self) -> Address {
        let mut buf = Vec::new();
        crate::transducer::sentence::put_vertex(
            &mut buf,
            &crate::transducer::sentence::Vertex::Glued {
                chart: self.chart.clone(),
                magnitude: self.magnitude,
                depth: self.depth,
                placement: self.placement.clone(),
                carriage: self.carriage.clone(),
            },
        );
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

/// A CONFORMAL ROUND OF THE NERVE (54.7 B8, as ruled 2026-09-21): the joint
/// NON-EUCLIDEAN geometry of the charts that meet, carried as ONE blade.
///
/// A junction of `m` charts was a simplex — `NerveTriangle`, three edge
/// indices and a composite walked around them — so a junction of `k` charts
/// was `C(k,3)` independent objects whose indices named positions in this
/// envelope's own list rather than coordinates of anything (rule 15).
/// Replaced by the object the physics already has: a round is not
/// assembled, it is a blade.
///
/// ```text
///     S = p₀ ∧ p₁ ∧ … ∧ pₘ ,   pᵢ = x₁e₁ + x₂e₂ + x₃e₃ + x₀e₋ + R e₊ ,  pᵢ² = 0
/// ```
///
/// THE ABSOLUTE IS NON-DEGENERATE. Each `pᵢ` is a chart's base point on the
/// null cone of `Cl(4,1)` under the HYPERBOLIC embedding: `x` lies on
/// `⟨x,x⟩_{3,1} = −R²` and `{e₊, e₋}` parameterise the intrinsic curvature
/// scale rather than a flat origin and a parabolic infinity. The flat
/// embedding `x + ½‖x‖²e_∞ + e_o` would put a privileged planar boundary
/// and a Cartesian norm into a manifold whose sectional curvature is not
/// zero, and its translations would be parabolic `T(a) = 1 + ½a e_∞`;
/// here a geodesic motion is a hyperbolic transvection and `τ` lives in the
/// Killing algebra `𝔰𝔬(3,1)` as displacement along a geodesic axis, not as
/// a translation across a flat vector space.
///
/// The inner product of two base points is then the invariant itself:
/// `pᵢ · pⱼ = R²(1 − cosh(d_ℍ(xᵢ,xⱼ)/R))`, so the round's geometry is the
/// manifold's and not a chart of it.
///
/// THE LADDER, and its ceiling is arithmetic:
///
/// ```text
///     grade 2   two points     geodesic point pair   the 1-skeleton seam
///     grade 3   three points   hyperbolic circle     a triple junction
///     grade 4   four points    2-sphere / horosphere a quadruple junction
///     grade 5   five points    the volume element I₅ orientation and density
///     grade 6   six points     ≡ 0
/// ```
///
/// `Cl(4,1)` has five generators and 32 blades, so `⋀⁶(ℚ^{4,1}) ≡ 0` and a
/// sixth point wedges to nothing; conformally dependent charts do the same
/// at any grade. There is no `if grade > MAX` anywhere — exhaustion is
/// Grassmann's.
///
/// WHY THIS IS THE ANSWER TO MULTI-INCIDENCE. Once the citation's target
/// left the key, two forms mating one boundary under one φ at one chart
/// rank stand at ONE site, and the citing chart cannot tell them apart
/// LOCALLY — that is gauge universality and it is correct. What separates
/// them is the joint geometry of the charts that meet, which is this blade:
/// a multi-sheeted seam of `k` sheets is ONE grade-`(k+1)` round, not
/// `C(k,3)` triangles.
///
/// THE BLADE IS DERIVED, NOT STORED (law 12). It is the outer product of
/// the base points and follows from them; what the round CARRIES is the
/// attachment, because under T21(iii) a dimension is never declared, it is
/// paid for, and attaching a cell along a stable round kills a class and
/// yields an inter-form axis. That act is a mind's, against a budget, and
/// does not factor through the geometry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NerveRound {
    /// The charts this round passes through, by their own addresses — a
    /// coordinate set, never indices into a list (rule 15).
    pub charts: Vec<Address>,
    /// Each chart's base point on the hyperboloid, in the same order:
    /// `(x₁, x₂, x₃, x₀)` with `x₁² + x₂² + x₃² − x₀² = −R²`. Four
    /// coordinates because `x₀` IS one: solving for it would need a root
    /// that is not rational in general, and law 4 does not round.
    pub points: Vec<[Q; 4]>,
    /// The curvature scale `R`, with `κ = −1/R²`. One per round: the charts
    /// of a junction lie on one absolute.
    pub radius: Q,
    /// The axis the attachment opened, where a cell was attached along this
    /// round. The one stored fact.
    pub attached: Option<Box<Versor>>,
}

impl NerveRound {
    /// `S = p₀ ∧ p₁ ∧ … ∧ pₘ`, wedged progressively.
    ///
    /// `None` for fewer than two points — a round needs an overlap — and for
    /// a point off the absolute, which `hyperbolic_point` refuses rather
    /// than projecting onto it (rule 11). The ZERO multivector where the
    /// points are conformally dependent or exceed the algebra: that is
    /// Grassmann's own answer and not a refusal.
    pub fn blade(&self) -> Option<crate::transducer::cga::Mv> {
        let at = |x: &[Q; 4]| {
            // The round's points travel as three base coordinates and `x₀`
            // (the n = 3 wire layout of `NerveRound`, recut with BQ5b).
            crate::transducer::cga::hyperbolic_point(&x[..3], &x[3], &self.radius)
        };
        let mut it = self.points.iter();
        let first = it.next()?;
        let _ = self.points.get(1)?;
        let mut s = at(first)?;
        // THE GRADE IS HOW MANY POINTS ARE IN THE BLADE, so it is read off
        // the walk rather than counted beside it (law 12): after `i` further
        // points the blade has grade `i + 1`.
        for (i, x) in it.enumerate() {
            s = s.wedge_vector(i as u32 + 1, &at(x)?);
            if s.is_zero() {
                return Some(s);
            }
        }
        Some(s)
    }

    /// The grade the round actually reached — `0` where the wedge vanished.
    /// A reading, never a field.
    pub fn grade(&self) -> u32 {
        match self.blade() {
            Some(s) if !s.is_zero() => self.points.len() as u32,
            _ => 0,
        }
    }

    /// DOES THE COMPOSITE TRANSITION STABILISE THE ROUND?
    ///
    /// ```text
    ///     S' = Ω S Ω̃        coherent ⟺ S' ∧ S = 0  (S' ∥ S)
    /// ```
    ///
    /// The simplicial reading walked `δφ_ijk = φ_jk φ_ik⁻¹ φ_ij` around three
    /// edges and asked whether it was the identity. This asks whether the
    /// versor's adjoint action is an isometry OF THE ROUND, which is the
    /// same question at every grade and does not need the junction
    /// decomposed into triples first.
    ///
    /// Projectively (§1.5): a round is a homogeneous blade, so `λS` and `S`
    /// are one round and the scale is not the reading.
    pub fn stabilises(&self, omega: &crate::transducer::cga::Mv) -> Option<bool> {
        let s = self.blade()?;
        if s.is_zero() {
            return None;
        }
        let moved = omega.geo(&s).geo(&omega.reverse());
        Some(crate::transducer::cga::projective_ratio(&moved, &s).is_some())
    }

    /// THE REJECTION `ΔS = S' − proj_S(S')`, the part of the moved round
    /// that does not lie along it.
    ///
    /// ```text
    ///     proj_S(S') = (⟨S' S̃⟩₀ / ⟨S S̃⟩₀) S ,     ΔS = S' − proj_S(S')
    /// ```
    ///
    /// It is CONTENT and not an error (rule 7, "curvature is content"): the
    /// exact non-Euclidean shear across the junction, which a mind may
    /// declare as an open boundary and a court refuses only when it is
    /// undeclared. At grade 3 it is the `H²` obstruction that separates two
    /// mates the citing chart sees as one boundary.
    ///
    /// `None` when the round is stabilised — there is no defect to carry —
    /// and when `⟨S S̃⟩₀ = 0`, where the projection is not defined and a
    /// zero would be a fabricated reading (rule 11).
    pub fn shear(&self, omega: &crate::transducer::cga::Mv) -> Option<crate::transducer::cga::Mv> {
        let s = self.blade()?;
        if s.is_zero() {
            return None;
        }
        let moved = omega.geo(&s).geo(&omega.reverse());
        if crate::transducer::cga::projective_ratio(&moved, &s).is_some() {
            return None;
        }
        let denom = s.geo(&s.reverse()).scalar_part();
        if denom == num_traits::Zero::zero() {
            return None;
        }
        let lambda = moved.geo(&s.reverse()).scalar_part() / denom;
        Some(moved.sub(&s.scale(&lambda)))
    }

    /// The coordinate this round is filed under: its chart addresses in
    /// order, length-delimited (A21). Two minds that met the same charts
    /// reach one key without anyone choosing an order.
    pub fn key(&self) -> Vec<u8> {
        let mut k = Vec::new();
        for c in &self.charts {
            crate::transducer::address::put_bytes(&mut k, c.as_bytes());
        }
        k
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.charts.len() as u64).to_le_bytes());
        for c in &self.charts {
            crate::transducer::address::put_bytes(out, c.as_bytes());
        }
        out.extend_from_slice(&(self.points.len() as u64).to_le_bytes());
        for x in &self.points {
            for q in x {
                crate::transducer::alphabet::put_q(out, q);
            }
        }
        crate::transducer::alphabet::put_q(out, &self.radius);
        match &self.attached {
            Some(v) => {
                out.push(1);
                v.to_bytes(out);
            }
            None => out.push(0),
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<NerveRound> {
        let n = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?) as usize;
        *pos += 8;
        let mut charts = Vec::new();
        for _ in 0..n {
            charts.push(Address::new(crate::transducer::address::take_bytes(b, pos)?));
        }
        let m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?) as usize;
        *pos += 8;
        let mut points = Vec::new();
        for _ in 0..m {
            points.push([
                crate::transducer::alphabet::take_q(b, pos)?,
                crate::transducer::alphabet::take_q(b, pos)?,
                crate::transducer::alphabet::take_q(b, pos)?,
                crate::transducer::alphabet::take_q(b, pos)?,
            ]);
        }
        let radius = crate::transducer::alphabet::take_q(b, pos)?;
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
        Some(NerveRound { charts, points, radius, attached })
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
    pub rounds: Vec<NerveRound>,
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
    /// the same envelope. `certificates` comes off a `Keyed`, which is
    /// already in key order.
    ///
    /// NO TRIANGLES ARE EMITTED. A triple overlap is derivable from the
    /// edges; what a 2-simplex carries is an ATTACHMENT, and nothing attaches
    /// an inter-form 2-cell yet. Emitting empty triangles for every triple
    /// would be a coordinate standing for no theorem (T21(iii)).
    pub fn of_form(f: &crate::transducer::sentence::Form) -> ReceiptsEnvelope {
        use crate::transducer::sentence::Vertex;
        // THE 1-SKELETON IS THE FORM'S COVER, taken whole (54.7 B5a). It
        // was rebuilt here by walking the `Glued` vertices for their target
        // addresses, and a vertex no longer names the form it mates with:
        // the boundary is the chart's, the incidence is the cover's.
        let mut env = ReceiptsEnvelope { nerve: f.cover.clone(), ..ReceiptsEnvelope::default() };
        for v in &f.vertices {
            match v {
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
        env.nerve = canonical_nerve(env.nerve.drain(..));
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
            && self.rounds.is_empty()
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
    /// Duplicates are kept. A form cited at two SITES is two citations and
    /// one USE; collapsing them here would decide that at the reading
    /// instead of at the ledger that keys by citer address, which is where
    /// `use_credit` actually counts — `cited_by` inserts the citer into a
    /// `Keyed`, so a form cited twice by one sentence is one entry.
    ///
    /// 54.7 B5a — LABOUR IS CREDITED PER DISTINCT GEOMETRIC OVERLAP, NOT
    /// PER STATEMENT. A multi-sheeted seam credits each sheet, because each
    /// distinct target is an independent interface in the cover. The same
    /// target twice at one site is ONE overlap declared as two: it cannot
    /// arise from `glue_append_under`, which refuses a duplicate edge, and
    /// an envelope that presents one is refused at the court with
    /// `REFUSE_SHEET_REPEATED` rather than quietly collapsed here — a
    /// speaker must not be able to inflate yield by restating an interface.
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
        out.extend_from_slice(&(self.rounds.len() as u64).to_le_bytes());
        for t in &self.rounds {
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
            env.rounds.push(NerveRound::from_bytes(b, pos)?);
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
