// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE MULTI-AXIAL ELEMENT — the uniform carrier of an ANASTOME germ, and the
//! embodiment of the asymmetry/symmetry law: an element IS the DESCRIPTION of a
//! transition (the directed coboundary A→B the client perceived); the SOLUTION is
//! its inverse deduction `g⁻¹` — computed by [`Element::solution`], never stored
//! as a second word. Description ∘ solution = identity (the coboundary closes:
//! consilience).
//!
//! It is NOT an enum of "kinds" (that caps capacity and forces kind-branching —
//! the flat-taxonomy sin). It is a sparse element over an EXTENSIBLE set of fibers
//! `(fiber_index → generator)`, and every fiber carries the SAME kind of exact
//! object: a `dim×dim` rational matrix — a generator of GL(dim,ℚ) acting on the
//! core Δ field by conjugation (`Operator::apply`, A·Δ·A⁻¹). Conformal motion,
//! gauge relabel, and germinated residual axes are all just FIBERS — never
//! variants. A pivot germinates by allocating a NEW fiber index (dimensional
//! resolution grows the axis count); no new variant, no branch.
//!
//! The operations are the real algebra, so the gate is the algebra LAWS, not hash
//! determinism:
//!   compose  — per-axis group product (matrix ·); associative.
//!   solution — per-axis group inverse (exact ℚ Gauss-Jordan); g∘g⁻¹ = e.
//!   bracket  — per-axis Lie commutator XY−YX; [X,Y]=−[Y,X], Jacobi.
//!   δ        — the antisymmetric coboundary a residual realizes over the core Δ.
//! English never appears here; addresses are wire identity, never reasoned on.

use crate::core::engine::{Delta, Operator, Q, QMatrix};
use crate::core::matrix::{det, identity, invert, mul};
use crate::transducer::germinate::{germ_bytes as residual_word, ResidualGerm};
use num_bigint::BigInt;
use num_traits::{One, Zero};

/// Reserved base fibers (the four strata); germinated fibers take indices ≥ 4.
pub const FIBER_SPATIAL: u16 = 0;
pub const FIBER_GAUGE: u16 = 1;
pub const FIBER_TEMPORAL: u16 = 2;
pub const FIBER_SHELL: u16 = 3;
/// First index available to germinated (residual/new-axis) fibers.
pub const FIBER_GERMINATED: u16 = 4;

#[cfg(test)]
fn q(v: i64) -> Q {
    Q::from(BigInt::from(v))
}

/// A − B for two `dim×dim` rational matrices (the Lie-algebra difference).
fn mat_sub(a: &QMatrix, b: &QMatrix) -> QMatrix {
    a.iter()
        .zip(b)
        .map(|(ra, rb)| ra.iter().zip(rb).map(|(x, y)| x - y).collect())
        .collect()
}

/// The commutator XY − YX — the Lie bracket of two generators of gl(dim,ℚ).
fn commutator(x: &QMatrix, y: &QMatrix) -> QMatrix {
    mat_sub(&mul(x, y), &mul(y, x))
}

/// Is `m` the `dim×dim` identity? (An identity fiber carries no description and
/// is dropped to keep the element canonical/sparse.)
fn is_identity(m: &QMatrix) -> bool {
    let n = m.len();
    m.iter().enumerate().all(|(i, row)| {
        row.iter().enumerate().all(|(j, x)| if i == j { x.is_one() } else { x.is_zero() })
    }) && m.iter().all(|r| r.len() == n)
}

/// A germ as a sparse multi-axial element carrying TWO complementary lenses of the
/// same description, so nothing is collapsed:
///   - `fibers` — the ALGEBRA lens: `dim×dim` exact rational generators for
///     compose/bracket/solution/δ. A projection (a residual's hue tally is not
///     injective — bidirectional swaps cancel), which is FINE for algebra.
///   - `word` — the coordinate-EXACT ANASTOME description (the banked language):
///     the exact transform bytes or the positional residual `(r,c,before,after)`.
///     This is what drives identity/equality/address, so distinct descriptions
///     never collide even where the tally does.
///   - `mag` — per-lane support (‖Δ‖'s |supp|), computed EXACTLY from the
///     description at germination, so economy/physics read a coordinate-exact cost
///     (a residual's changed-cell count, never a canceling tally).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    dim: usize,
    fibers: Vec<(u16, QMatrix)>,
    /// Coordinate-exact ANASTOME description (empty for pure-algebra elements).
    word: Vec<u8>,
    /// Per-lane support, exact from the description (0⃗ for pure-algebra elements).
    pub(crate) mag: [u64; 4],
    /// The CENTERED COBOUNDARY CHAIN — the residual's substance with geometry
    /// preserved up to translation: `(row, col, before, after)` over the changed
    /// cells, min-corner framed to the origin (the substrate's transposition
    /// projection), sorted. This is what the confluent chain-reduction peels and
    /// what `same_action` compares for residual germs — the position-free hue
    /// tally (the `fibers` δ-lens) is a projection and never decides identity or
    /// substance. Empty for pure-algebra (group) elements.
    pub(crate) sub: Vec<(i64, i64, u8, u8)>,
    /// THE PUNCTURED AXIS — sections along the germinated fiber pierced through
    /// this thought: alternative per-cell transition ASSIGNMENTS over the SAME
    /// base positions. A recurring family (same geometry, different transitions)
    /// collapses into sections of this one bundle instead of sibling cards —
    /// growth as +axis (volume), not +card (area). The base `sub`'s own
    /// assignment is the implicit zeroth section. Vec-of-relations; empty until
    /// the axis is pierced. (The FREE part of the fiber; cycles condense into
    /// `torsion` below.)
    sections: Vec<Vec<(u8, u8)>>,
    /// THE TORSION OF THE AXIS — CE's law reaching the germinated fiber ("colors
    /// are torsion fields, not lookup tables"; the torsion_order is the period of
    /// the generating fold). When sections form the ORBIT of one hue-map
    /// generator σ (derived from the witnessed family cell-wise, exactly as peel
    /// derives offsets — never an enumerated plane), the fiber condenses from a
    /// free list to a TORSION CYCLE: `(σ relations from↦to, period ℓ)`. The
    /// orbit {base, σ·base, σ²·base, …} of length ℓ is spoken in O(1) storage —
    /// and a CLOSED cycle (σ^ℓ = id on its support) speaks orbit members never
    /// individually witnessed: capacity earned by exact structure. The fiber is
    /// thus ℤ^r ⊕ ⊕ᵢ ℤ/tᵢ — free sections ⊕ torsion cycles.
    torsion: Vec<(Vec<(u8, u8)>, u32)>,
    /// THE ANCHOR FIBER (Step 7) — γ = (π_anchor, Δ_substance): the recursive
    /// selection term restricting WHERE this word's substance applies on a
    /// scene. None = ⊤, the unconstrained carrier (pure topological actions,
    /// and every word born before selection entered the language).
    pub(crate) anchor: Option<crate::transducer::anchor::Sel>,
}

/// Apply a hue-map σ (functional from↦to relations) to a transition assignment:
/// every hue present must be in σ's domain, else the orbit ends (None).
pub fn apply_hue_map(sigma: &[(u8, u8)], asg: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let map = |h: u8| sigma.iter().find(|(f, _)| *f == h).map(|(_, t)| *t);
    asg.iter().map(|(b, a)| Some((map(*b)?, map(*a)?))).collect()
}

/// Derive the hue-map carrying assignment A onto assignment B cell-wise —
/// `None` if the relation is not functional (one hue demanding two images).
/// This is DERIVATION from the data, never a hypothesized plane.
pub fn derive_hue_map(a: &[(u8, u8)], b: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut sigma: Vec<(u8, u8)> = Vec::new();
    let mut bind = |from: u8, to: u8| -> bool {
        match sigma.iter().find(|(f, _)| *f == from) {
            Some((_, t)) => *t == to,
            None => {
                sigma.push((from, to));
                true
            }
        }
    };
    if a.len() != b.len() {
        return None;
    }
    for ((ab, aa), (bb, ba)) in a.iter().zip(b) {
        if !bind(*ab, *bb) || !bind(*aa, *ba) {
            return None;
        }
    }
    sigma.sort_unstable();
    Some(sigma)
}

/// The positions skeleton of a centered chain (its base geometry).
pub fn positions(chain: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64)> {
    chain.iter().map(|(r, c, _, _)| (*r, *c)).collect()
}

/// The transition assignment of a centered chain (its fiber coordinate),
/// index-aligned with [`positions`] (chains sort by position first).
pub fn assignment(chain: &[(i64, i64, u8, u8)]) -> Vec<(u8, u8)> {
    chain.iter().map(|(_, _, b, a)| (*b, *a)).collect()
}

impl Element {
    /// The identity element on `dim` coordinates — the empty description (no
    /// transition on any axis). `dim` is the representation dimension shared by
    /// every fiber so they compose/bracket in a common GL(dim,ℚ).
    pub fn identity(dim: usize) -> Self {
        Element {
            dim,
            fibers: Vec::new(),
            word: Vec::new(),
            mag: [0; 4],
            sub: Vec::new(),
            sections: Vec::new(),
            torsion: Vec::new(),
            anchor: None,
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    /// Set (or replace) a fiber's generator. Must be `dim×dim`. An identity
    /// generator clears the fiber (sparse). Kept sorted by fiber index.
    pub fn set_fiber(&mut self, index: u16, mat: QMatrix) {
        assert_eq!(mat.len(), self.dim, "generator dimension mismatch");
        assert!(mat.iter().all(|r| r.len() == self.dim), "generator not square");
        self.fibers.retain(|(i, _)| *i != index);
        if !is_identity(&mat) {
            self.fibers.push((index, mat));
            self.fibers.sort_by_key(|(i, _)| *i);
        }
    }

    /// Builder-style set.
    pub fn with_fiber(mut self, index: u16, mat: QMatrix) -> Self {
        self.set_fiber(index, mat);
        self
    }

    pub fn fiber(&self, index: u16) -> Option<&QMatrix> {
        self.fibers.iter().find(|(i, _)| *i == index).map(|(_, m)| m)
    }

    /// The fibers this element carries a description on — its occupied axes.
    pub fn axes(&self) -> Vec<u16> {
        self.fibers.iter().map(|(i, _)| *i).collect()
    }

    /// The highest fiber in use (for allocating the next germinated axis).
    pub fn max_axis(&self) -> Option<u16> {
        self.fibers.iter().map(|(i, _)| *i).max()
    }

    pub fn is_identity(&self) -> bool {
        // A residual with a canceling hue tally (a bidirectional swap) has empty
        // FIBERS but a real chain — the chain decides; the tally never does.
        self.fibers.is_empty() && self.sub.is_empty()
    }

    /// The centered coboundary chain (empty for pure-algebra elements).
    /// The anchor fiber — None is ⊤ (the unconstrained carrier).
    pub fn anchor(&self) -> Option<&crate::transducer::anchor::Sel> {
        self.anchor.as_ref()
    }

    /// Bind the anchor fiber (germination writes it at birth; the court's
    /// refinement rebinds it).
    pub fn with_anchor(mut self, pi: crate::transducer::anchor::Sel) -> Self {
        self.anchor = Some(pi);
        self
    }

    pub fn sub(&self) -> &[(i64, i64, u8, u8)] {
        &self.sub
    }

    /// The free sections along the punctured axis (cycles live in `torsion`).
    pub fn sections(&self) -> &[Vec<(u8, u8)>] {
        &self.sections
    }

    /// The torsion cycles of the axis: `(generator σ, period ℓ)` each.
    pub fn torsion(&self) -> &[(Vec<(u8, u8)>, u32)] {
        &self.torsion
    }

    /// The fiber's total spoken span — base + free sections + every torsion-orbit
    /// member (the volume metric; cycles count their full period).
    pub fn fiber_span(&self) -> usize {
        1 + self.sections.len()
            + self.torsion.iter().map(|(_, l)| (*l as usize).saturating_sub(1)).sum::<usize>()
    }

    /// Every concrete rewrite chain this word can speak — the base chain plus
    /// each section/orbit assignment materialized over the base geometry
    /// (positions held, hue-transition classes substituted class-for-class).
    /// The single source for application/projection (the gate and the SDK's
    /// `Entity::project` both speak through this).
    pub fn variant_chains(&self) -> Vec<Vec<(i64, i64, u8, u8)>> {
        let base = self.sub.clone();
        if base.is_empty() {
            return Vec::new();
        }
        let base_pairs = assignment(&base);
        let mut out = vec![base.clone()];
        for asg in self.assignments() {
            if asg.len() != base_pairs.len() || asg == base_pairs {
                continue;
            }
            let variant: Vec<_> = base
                .iter()
                .map(|(r, c, b, a)| {
                    let i = base_pairs.iter().position(|p| *p == (*b, *a)).unwrap_or(0);
                    (*r, *c, asg[i].0, asg[i].1)
                })
                .collect();
            out.push(variant);
        }
        out
    }

    /// Every transition assignment this thought speaks: base, free sections, and
    /// each torsion cycle's ORBIT expanded exactly to its period (bounded by ℓ —
    /// a closed cycle speaks members never individually witnessed).
    pub fn assignments(&self) -> Vec<Vec<(u8, u8)>> {
        let base = assignment(&self.sub);
        let mut out = vec![base.clone()];
        out.extend(self.sections.iter().cloned());
        for (sigma, l) in &self.torsion {
            let mut cur = base.clone();
            for _ in 1..*l {
                match apply_hue_map(sigma, &cur) {
                    Some(next) => {
                        if !out.contains(&next) {
                            out.push(next.clone());
                        }
                        cur = next;
                    }
                    None => break,
                }
            }
        }
        out
    }

    /// Does a centered chain SIT on this thought — same base positions, and its
    /// assignment is the base's, a free section's, or a member of a torsion
    /// cycle's orbit (walked to the period, O(ℓ·cells) — the closed-form
    /// membership that makes the compressed fiber CHEAPER than the list it
    /// replaced)?
    pub fn matches_chain(&self, chain: &[(i64, i64, u8, u8)]) -> bool {
        if self.sub.is_empty() || positions(&self.sub) != positions(chain) {
            return false;
        }
        let asg = assignment(chain);
        let base = assignment(&self.sub);
        if asg == base || self.sections.contains(&asg) {
            return true;
        }
        for (sigma, l) in &self.torsion {
            let mut cur = base.clone();
            for _ in 1..*l {
                match apply_hue_map(sigma, &cur) {
                    Some(next) => {
                        if next == asg {
                            return true;
                        }
                        cur = next;
                    }
                    None => break,
                }
            }
        }
        false
    }

    /// THE TORSE MOVE — extract the torsion of the punctured axis. Candidate
    /// generators are DERIVED from the witnessed family (base → each section,
    /// cell-wise — the same way peel derives offsets; nothing enumerated); the
    /// first σ whose orbit spans enough of the family condenses those sections
    /// into ONE cycle `(σ, ℓ)`. A closure found within the walk (σ^t·base = base)
    /// sets ℓ = t — the FULL cycle, including unwitnessed members: capacity by
    /// exact structure. Returns the number of sections condensed (0 = no torsion
    /// found; the free part stays as it was).
    pub fn torse(&mut self) -> usize {
        const MAX_ORBIT: usize = 32;
        if self.sections.is_empty() {
            return 0;
        }
        let base = assignment(&self.sub);
        for k in 0..self.sections.len() {
            let Some(sigma) = derive_hue_map(&base, &self.sections[k]) else { continue };
            // Walk the orbit: collect members, note closure.
            let mut orbit: Vec<Vec<(u8, u8)>> = Vec::new();
            let mut cur = base.clone();
            let mut period = 0u32;
            for step in 1..=MAX_ORBIT {
                match apply_hue_map(&sigma, &cur) {
                    Some(next) => {
                        if next == base {
                            period = step as u32; // closed: σ^t = id on its support
                            break;
                        }
                        orbit.push(next.clone());
                        cur = next;
                    }
                    None => break,
                }
            }
            let spanned: Vec<usize> = (0..self.sections.len())
                .filter(|&i| orbit.contains(&self.sections[i]))
                .collect();
            // Worth condensing: the orbit spans ≥2 sections, or it CLOSES (a true
            // ℤ/t cycle — structure even at one witnessed section, e.g. a swap).
            if spanned.len() >= 2 || (period > 0 && !spanned.is_empty()) {
                let l = if period > 0 { period } else { (orbit.len() + 1) as u32 };
                for &i in spanned.iter().rev() {
                    self.sections.remove(i);
                }
                self.torsion.push((sigma, l));
                return spanned.len();
            }
        }
        0
    }

    /// PUNCTURE — pierce (or extend) the germinated axis with a new section: a
    /// fresh transition assignment over the same base positions. The Z through
    /// the paper: the thought gains VOLUME; the family member becomes a section,
    /// not a card. Returns false if the assignment is already spoken (or does
    /// not fit the base geometry).
    pub fn puncture(&mut self, asg: Vec<(u8, u8)>) -> bool {
        if asg.len() != self.sub.len()
            || asg == assignment(&self.sub)
            || self.sections.contains(&asg)
            || self.assignments().contains(&asg)
        {
            return false; // already spoken — by base, a section, or a torsion orbit
        }
        self.sections.push(asg);
        true
    }

    /// The composite operator this description applies to the field — the product
    /// of its per-axis generators, in fiber order. (A convenience realization; the
    /// group laws below are per-axis, where they hold exactly.)
    pub fn realize(&self) -> QMatrix {
        let mut acc = identity(self.dim);
        for (_, m) in &self.fibers {
            acc = mul(&acc, m);
        }
        acc
    }

    /// Realize as a core [`Operator`] (acts on Δ by conjugation A·Δ·A⁻¹). `None`
    /// if the description is orientation-reversing or singular (det ≤ 0) — such a
    /// description is not in the connected group and does not lift to an Operator.
    pub fn as_operator(&self, id: usize) -> Option<Operator> {
        let m = self.realize();
        let inv = invert(&m)?;
        if det(&m) <= Q::zero() {
            return None;
        }
        Some(Operator { id, matrix: m, matrix_inv: inv, det_sign_positive: true })
    }

    /// Do two elements describe the SAME transformation, position quotiented? For
    /// residual germs (non-empty chains) the CENTERED CHAIN decides — geometry
    /// preserved up to translation, so equal-tally/different-geometry residuals
    /// never conflate (the Step-1 finding, cured). For pure-algebra (group)
    /// elements the realized matrix decides, where the realization is injective.
    /// A chain-carrying and a chain-free element are never the same action.
    pub fn same_action(&self, other: &Element) -> bool {
        if !self.sub.is_empty() || !other.sub.is_empty() {
            return self.sub == other.sub;
        }
        self.realize() == other.realize()
    }

    /// COMPOSE — the per-axis group product `self` then `other`. Shared axes
    /// multiply (matrix ·); disjoint axes are carried. Associative (the group law).
    pub fn compose(&self, other: &Element) -> Element {
        assert_eq!(self.dim, other.dim, "dimension mismatch");
        let mut out = Element::identity(self.dim);
        // Union of axes, in sorted order.
        let mut axes: Vec<u16> = self.axes();
        for a in other.axes() {
            if !axes.contains(&a) {
                axes.push(a);
            }
        }
        axes.sort_unstable();
        for a in axes {
            let combined = match (self.fiber(a), other.fiber(a)) {
                (Some(x), Some(y)) => mul(x, y),
                (Some(x), None) => x.clone(),
                (None, Some(y)) => y.clone(),
                (None, None) => continue,
            };
            out.set_fiber(a, combined);
        }
        out
    }

    /// SOLUTION — the inverse deduction `g⁻¹`, derived (never stored) from the
    /// description: per-axis exact group inverse. `self.compose(self.solution())`
    /// is the identity — the coboundary closes (consilience). `None` if any axis
    /// generator is singular (an irreversible description has no group solution;
    /// it stays a pivot).
    pub fn solution(&self) -> Option<Element> {
        let mut out = Element::identity(self.dim);
        for (i, m) in &self.fibers {
            out.set_fiber(*i, invert(m)?);
        }
        Some(out)
    }

    /// Group inverse — an alias for [`Element::solution`] at the algebra level.
    pub fn inverse(&self) -> Option<Element> {
        self.solution()
    }

    /// BRACKET — the per-axis Lie commutator `[self,other] = XY − YX`. The result's
    /// fibers are gl(dim,ℚ) elements (generally not invertible — they live in the
    /// algebra, not the group). Nontrivial only on shared axes (`[X,I]=0`). This is
    /// the anti-redundant-germination probe: a bracket that closes into the span is
    /// no new axis (R6).
    pub fn bracket(&self, other: &Element) -> Element {
        assert_eq!(self.dim, other.dim, "dimension mismatch");
        let mut out = Element::identity(self.dim);
        for (i, x) in &self.fibers {
            if let Some(y) = other.fiber(*i) {
                let c = commutator(x, y);
                // A zero bracket means the axes commute — carry nothing.
                if c.iter().any(|r| r.iter().any(|v| !v.is_zero())) {
                    // Store directly (a bracket may be identity-shaped yet nonzero
                    // as an algebra element; bypass the group-identity drop).
                    out.fibers.retain(|(j, _)| j != i);
                    out.fibers.push((*i, c));
                }
            }
        }
        out.fibers.sort_by_key(|(i, _)| *i);
        out
    }

    /// The antisymmetric coboundary of a fiber's generator, `M − Mᵀ` — the δ that
    /// realizes this axis's directed description as an antisymmetric field.
    pub fn coboundary(&self, index: u16) -> Option<QMatrix> {
        let m = self.fiber(index)?;
        let n = m.len();
        let mut d = vec![vec![Q::zero(); n]; n];
        for i in 0..n {
            for j in 0..n {
                d[i][j] = &m[i][j] - &m[j][i];
            }
        }
        Some(d)
    }

    /// Canonical ANASTOME bytes: the BASE identity (word, dim, fibers) followed
    /// by the punctured axis's sections — the FULL bundle, so a taught thought
    /// carries its interior volume across the nerve.
    pub fn canonical(&self) -> Vec<u8> {
        let mut out = self.base_canonical();
        out.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        for s in &self.sections {
            out.extend_from_slice(&(s.len() as u32).to_le_bytes());
            for (b, a) in s {
                out.push(*b);
                out.push(*a);
            }
        }
        // The torsion cycles: (generator relations, period).
        out.extend_from_slice(&(self.torsion.len() as u32).to_le_bytes());
        for (sigma, l) in &self.torsion {
            out.extend_from_slice(&(sigma.len() as u32).to_le_bytes());
            for (f, t) in sigma {
                out.push(*f);
                out.push(*t);
            }
            out.extend_from_slice(&l.to_le_bytes());
        }
        out
    }

    /// The BASE canonical form: the coordinate-exact `word` (the banked
    /// description — what makes identity injective), then `[dim:2]` and each
    /// fiber's exact ℚ entries in fiber then row-major order. Order-independent
    /// (fibers are kept sorted). WIRE IDENTITY ONLY — never a reasoning operand.
    /// Sections are EXCLUDED: puncturing deepens a thought's volume without
    /// changing which word it is (address stability across the pierced axis).
    fn base_canonical(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(self.word.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.word);
        out.extend_from_slice(&(self.dim as u16).to_le_bytes());
        for (i, m) in &self.fibers {
            out.extend_from_slice(&i.to_le_bytes());
            for row in m {
                for v in row {
                    let (ns, nb) = v.numer().to_bytes_le();
                    let db = v.denom().to_bytes_le().1;
                    out.push(match ns {
                        num_bigint::Sign::Minus => 2,
                        num_bigint::Sign::NoSign => 0,
                        num_bigint::Sign::Plus => 1,
                    });
                    out.extend_from_slice(&(nb.len() as u32).to_le_bytes());
                    out.extend_from_slice(&nb);
                    out.extend_from_slice(&(db.len() as u32).to_le_bytes());
                    out.extend_from_slice(&db);
                }
            }
        }
        out
    }

    /// The element's canonical ADDRESS — a digest of its BASE form (sections
    /// excluded: a pierced axis deepens the thought's volume without changing
    /// which word it is). Wire/cache identity; never a reasoning operand.
    pub fn address(&self) -> [u8; 32] {
        *blake3::hash(&self.base_canonical()).as_bytes()
    }

    /// Serialize the FULL element (dim, coordinate-exact word, mag, exact-ℚ fibers)
    /// for persistence — so a germinated basis survives restart (P7). Lossless: a
    /// round trip reproduces address, algebra, identity, and magnitude exactly.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(self.dim as u32).to_le_bytes());
        out.extend_from_slice(&(self.word.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.word);
        for m in &self.mag {
            out.extend_from_slice(&m.to_le_bytes());
        }
        out.extend_from_slice(&(self.fibers.len() as u32).to_le_bytes());
        for (i, mat) in &self.fibers {
            out.extend_from_slice(&i.to_le_bytes());
            for row in mat {
                for v in row {
                    let nb = v.numer().to_signed_bytes_le();
                    let db = v.denom().to_signed_bytes_le();
                    out.extend_from_slice(&(nb.len() as u32).to_le_bytes());
                    out.extend_from_slice(&nb);
                    out.extend_from_slice(&(db.len() as u32).to_le_bytes());
                    out.extend_from_slice(&db);
                }
            }
        }
        // The centered coboundary chain (the substance geometry).
        out.extend_from_slice(&(self.sub.len() as u32).to_le_bytes());
        for (r, c, before, after) in &self.sub {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*before);
            out.push(*after);
        }
        // The punctured axis's sections (the thought's interior volume).
        out.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        for s in &self.sections {
            out.extend_from_slice(&(s.len() as u32).to_le_bytes());
            for (b, a) in s {
                out.push(*b);
                out.push(*a);
            }
        }
        // The torsion cycles: (generator relations, period ℓ).
        out.extend_from_slice(&(self.torsion.len() as u32).to_le_bytes());
        for (sigma, l) in &self.torsion {
            out.extend_from_slice(&(sigma.len() as u32).to_le_bytes());
            for (f, t) in sigma {
                out.push(*f);
                out.push(*t);
            }
            out.extend_from_slice(&l.to_le_bytes());
        }
        // THE ANCHOR FIBER: [1][term] present, [0] the ⊤ carrier. Appended
        // last so pre-anchor bytes parse with an unconstrained anchor.
        match &self.anchor {
            Some(t) => {
                out.push(1);
                t.to_bytes(&mut out);
            }
            None => out.push(0),
        }
        out
    }

    /// Deserialize an element written by [`Element::to_bytes`], advancing `pos`.
    /// `None` on a malformed ledger (never a panic on disk corruption).
    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Element> {
        let u32_at = |b: &[u8], p: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v)
        };
        let dim = u32_at(b, pos)? as usize;
        if dim > 64 {
            return None; // untrusted: no sane element exceeds this (GERM_DIM = 13)
        }
        let wlen = u32_at(b, pos)? as usize;
        let word = b.get(*pos..*pos + wlen)?.to_vec();
        *pos += wlen;
        let mut mag = [0u64; 4];
        for m in &mut mag {
            *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        // Counts are UNTRUSTED (disk corruption, old formats, the wire): validate
        // against the remaining bytes BEFORE any allocation — a garbage count must
        // return None, never abort the process on a phantom-capacity request.
        let remaining = |b: &[u8], p: &usize| b.len().saturating_sub(*p);
        let nfib = u32_at(b, pos)? as usize;
        if nfib > remaining(b, pos) / 2 {
            return None; // each fiber needs at least its index bytes
        }
        let mut fibers = Vec::with_capacity(nfib);
        for _ in 0..nfib {
            let idx = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
            *pos += 2;
            let mut mat = vec![vec![Q::zero(); dim]; dim];
            for row in mat.iter_mut() {
                for cell in row.iter_mut() {
                    let nl = u32_at(b, pos)? as usize;
                    let nb = b.get(*pos..*pos + nl)?;
                    *pos += nl;
                    let numer = BigInt::from_signed_bytes_le(nb);
                    let dl = u32_at(b, pos)? as usize;
                    let db = b.get(*pos..*pos + dl)?;
                    *pos += dl;
                    let denom = BigInt::from_signed_bytes_le(db);
                    *cell = Q::new(numer, denom);
                }
            }
            fibers.push((idx, mat));
        }
        let nsub = u32_at(b, pos)? as usize;
        if nsub > remaining(b, pos) / 18 {
            return None; // 18 bytes per chain cell
        }
        let mut sub = Vec::with_capacity(nsub);
        for _ in 0..nsub {
            let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            let c = i64::from_le_bytes(b.get(*pos + 8..*pos + 16)?.try_into().ok()?);
            let before = *b.get(*pos + 16)?;
            let after = *b.get(*pos + 17)?;
            *pos += 18;
            sub.push((r, c, before, after));
        }
        let nsec = u32_at(b, pos)? as usize;
        if nsec > remaining(b, pos) / 4 {
            return None; // each section needs at least its length prefix
        }
        let mut sections = Vec::with_capacity(nsec);
        for _ in 0..nsec {
            let slen = u32_at(b, pos)? as usize;
            if slen > remaining(b, pos) / 2 {
                return None; // 2 bytes per section entry
            }
            let mut s = Vec::with_capacity(slen);
            for _ in 0..slen {
                let bb = *b.get(*pos)?;
                let aa = *b.get(*pos + 1)?;
                *pos += 2;
                s.push((bb, aa));
            }
            sections.push(s);
        }
        let ntor = u32_at(b, pos)? as usize;
        if ntor > remaining(b, pos) / 8 {
            return None; // untrusted count: each cycle needs relations-len + period
        }
        let mut torsion = Vec::with_capacity(ntor);
        for _ in 0..ntor {
            let slen = u32_at(b, pos)? as usize;
            if slen > remaining(b, pos) / 2 {
                return None;
            }
            let mut sigma = Vec::with_capacity(slen);
            for _ in 0..slen {
                let f = *b.get(*pos)?;
                let t = *b.get(*pos + 1)?;
                *pos += 2;
                sigma.push((f, t));
            }
            let l = u32_at(b, pos)?;
            torsion.push((sigma, l));
        }
        // The anchor fiber tail (absent in pre-anchor serializations → ⊤).
        let anchor = match b.get(*pos) {
            Some(1) => {
                *pos += 1;
                Some(crate::transducer::anchor::Sel::from_bytes(b, pos)?)
            }
            Some(0) => {
                *pos += 1;
                None
            }
            _ => None,
        };
        Some(Element { dim, fibers, word, mag, sub, sections, torsion, anchor })
    }
}

/// Realize a residual germ as the antisymmetric hue-transition field over the core
/// Δ — the δ (coboundary) of the description. States are hues `0..=maxhue`; for
/// each changed cell `before→after` the net directed flow accumulates
/// `Δ[before][after] += 1` (and antisymmetrically `−1` the other way). This is the
/// residual's exact description as an antisymmetric field; its reverse germ negates
/// it, so description ⊕ solution closes to the zero field.
pub fn germ_delta(g: &ResidualGerm) -> Delta {
    let maxhue = g.iter().flat_map(|(_, _, b, a)| [*b, *a]).max().unwrap_or(0) as usize;
    let n = maxhue + 1;
    let mut delta = Delta::zero(n, 1);
    for (_, _, before, after) in g {
        let (a, b) = (*before as usize, *after as usize);
        if a == b {
            continue;
        }
        delta.entries[a][b][0] += Q::one();
        delta.entries[b][a][0] -= Q::one();
    }
    delta
}

// ─── THE GERM CARRIER — a demand's description as ONE block-diagonal operator ───
//
// A germ is realized at a common dimension `GERM_DIM = 3 + PALETTE`: the spatial
// homogeneous plane (rows 0..3) ⊕ the hue axis (rows 3..13). Every germ — whether
// a solved motion∘recolour or an irreducible residual — is the SAME shape, so the
// basis is one uniform species (no kind-branch) and germs are cross-comparable
// (the R6 Lie structure will compose them block-wise). The block-diagonal form
// composes/inverts block-wise for free, and the two blocks read off the per-lane
// physics/economy vectors below.

/// Hue states carried on the gauge/germinated axis (ARC palette 0..9; 0 = absent).
pub const PALETTE: usize = 10;
/// The common germ dimension: spatial homogeneous plane (3) ⊕ hue axis (PALETTE).
pub const GERM_DIM: usize = 3 + PALETTE;

/// Copy a small square `block` into `dst` at diagonal offset `off`.
fn place_block(dst: &mut QMatrix, off: usize, block: &QMatrix) {
    let n = block.len();
    for i in 0..n {
        for j in 0..n {
            dst[off + i][off + j] = block[i][j].clone();
        }
    }
}

/// The DESCRIPTION of a perceived residual as a germ — post-excision, the ONLY way
/// cognition mints anything: every non-transposition demand IS a coboundary the
/// eye measured, and every "rotation"/"recolour" the mesh will ever speak is a
/// germinated pattern, never a named solver.
///
/// The chain is CENTERED — the changed-cell set framed with its min-corner at the
/// origin (the substrate's transposition projection, hypha-side per the ruling) —
/// so the same deformation at any position is ONE word, while geometry (the
/// relative offsets between changed cells) is preserved exactly: equal-tally,
/// different-geometry residuals stay distinct. The word/identity IS the centered
/// chain; the antisymmetric hue tally is kept only as the δ-algebra lens.
pub fn from_residual(g: &ResidualGerm) -> Element {
    // The substrate framing projection: min-corner of the changed set → origin.
    let min_r = g.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
    let min_c = g.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
    let mut centered: ResidualGerm =
        g.iter().map(|(r, c, before, after)| (r - min_r, c - min_c, *before, *after)).collect();
    centered.sort_unstable();

    // The δ-lens: the position-free antisymmetric hue-transition tally (a
    // projection — algebra only, never identity/substance).
    let mut m = identity(GERM_DIM);
    let mut hue = vec![vec![Q::zero(); PALETTE]; PALETTE];
    for (_, _, before, after) in &centered {
        let (a, b) = (*before as usize, *after as usize);
        if a == b || a >= PALETTE || b >= PALETTE {
            continue;
        }
        hue[a][b] += Q::one();
        hue[b][a] -= Q::one();
    }
    place_block(&mut m, 3, &hue);

    let supp = centered.len() as u64;
    let mut e = Element::identity(GERM_DIM).with_fiber(FIBER_SPATIAL, m);
    e.word = residual_word(&centered);
    e.mag = [0, 0, supp, 0];
    e.sub = centered;
    e
}

impl Element {
    /// The four lanes this germ carries content on — 𝒮 spatial, 𝒢 gauge (a group
    /// recolour), residual (a coboundary/new axis), 𝒽 shell — exactly where the
    /// coordinate-exact magnitude is nonzero (never the lossy tally).
    pub fn lanes(&self) -> [bool; 4] {
        std::array::from_fn(|l| self.mag[l] > 0)
    }

    /// The germ's polytonic MAGNITUDE — per-lane support size (‖Δ‖'s |supp|
    /// component), never summed, computed EXACTLY from the description at
    /// germination. A settled demand's residual magnitude is 0⃗; a real germ's is
    /// nonzero — the descent the POW++ gate reads.
    pub fn magnitude(&self) -> [u64; 4] {
        self.mag
    }

    /// THE TRANSPOSITION QUOTIENT — factor out the translation (the ℤ² orbit) so
    /// the element becomes its position-independent SUBSTANCE
    /// (dilation·rotation·gauge·residual). Two transposition-equivalent demands
    /// (same substance, different position) share ONE substance — the object of
    /// confluent reduction, not the coordinate-exact word. (Gauge S₁₀ canonical
    /// relabel is a further quotient, still to land.)
    pub fn substance(&self) -> Element {
        let mut out = self.clone();
        out.word = Vec::new(); // the substance is translation-invariant, not the exact word
        for (i, m) in out.fibers.iter_mut() {
            if *i == FIBER_SPATIAL && m.len() == GERM_DIM {
                m[0][2] = Q::zero(); // zero the spatial translation column
                m[1][2] = Q::zero();
            }
        }
        // a pure transposition (translate only) becomes the identity — empty substance
        out.fibers.retain(|(_, m)| !is_identity(m));
        // A residual's substance IS its centered chain (already position-quotiented
        // by construction); its magnitude reads the CHAIN, never the cancelable tally.
        out.mag = if out.sub.is_empty() { out.reduction_magnitude() } else { [0, 0, out.sub.len() as u64, 0] };
        out
    }

    /// THE REDUCTION MAGNITUDE ‖Δ‖ read from the MATRIX (drives the confluent
    /// peeling's ≺_DM descent): per-lane count of non-identity structural entries —
    /// 𝒮 the spatial block, 𝒢 the hue block. Distinct from the description-derived
    /// germ [`magnitude`]: this is the well-founded order the reduction descends.
    pub fn reduction_magnitude(&self) -> [u64; 4] {
        match self.fiber(FIBER_SPATIAL) {
            Some(m) if m.len() == GERM_DIM => {
                let id = identity(GERM_DIM);
                let spatial = (0..3)
                    .flat_map(|i| (0..3).map(move |j| (i, j)))
                    .filter(|&(i, j)| m[i][j] != id[i][j])
                    .count();
                let hue = (3..GERM_DIM)
                    .flat_map(|i| (3..GERM_DIM).map(move |j| (i, j)))
                    .filter(|&(i, j)| m[i][j] != id[i][j])
                    .count();
                [spatial as u64, hue as u64, 0, 0]
            }
            _ => [0; 4],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// γ = (π, Δ): the anchor fiber survives serde exactly; pre-anchor bytes
    /// (no tail) parse to the ⊤ carrier — full backward compatibility.
    #[test]
    fn anchor_fiber_round_trips_and_top_is_default() {
        use crate::transducer::anchor::Sel;
        let g: ResidualGerm = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        let plain = from_residual(&g);
        assert!(plain.anchor().is_none(), "born ⊤ until germination binds");
        let pi = Sel::Minus(Box::new(Sel::Hue(1)), Box::new(Sel::Delta(Box::new(Sel::Hue(1)))));
        let anchored = plain.clone().with_anchor(pi.clone());
        let bytes = anchored.to_bytes();
        let mut pos = 0;
        let back = Element::from_bytes(&bytes, &mut pos).expect("parse");
        assert_eq!(back.anchor(), Some(&pi), "π survives the wire");
        assert_eq!(back.sub(), anchored.sub(), "Δ untouched");
        assert_eq!(back.address(), anchored.address(), "address is BASE-only — π never renames");
        // A pre-anchor serialization (tail stripped) parses to ⊤.
        let plain_bytes = plain.to_bytes();
        let mut pos2 = 0;
        let old = Element::from_bytes(&plain_bytes[..plain_bytes.len() - 1], &mut pos2).expect("old form");
        assert!(old.anchor().is_none(), "absent tail = the unconstrained carrier");
    }

    use super::*;
    use crate::transducer::germinate::germinate;

    fn qm(rows: &[&[i64]]) -> QMatrix {
        rows.iter().map(|r| r.iter().map(|&v| q(v)).collect()).collect()
    }

    /// A shear generator (det 1, so it lifts to an Operator).
    fn shear(a: i64) -> QMatrix {
        qm(&[&[1, a, 0], &[0, 1, 0], &[0, 0, 1]])
    }

    // NOTE post-excision: tests fabricate exact matrices to VERIFY the algebra
    // laws (associativity, inverses, brackets, conjugation). They name nothing in
    // cognition — no solver exists; the mesh germinates its own generators.

    // ─── THE GATE: algebra laws, not hash determinism ───────────────────────

    /// COMPOSE IS ASSOCIATIVE — the group law, per axis. (A∘B)∘C = A∘(B∘C).
    #[test]
    fn compose_is_associative() {
        let a = Element::identity(3).with_fiber(FIBER_SPATIAL, shear(1));
        let b = Element::identity(3).with_fiber(FIBER_SPATIAL, shear(2));
        let c = Element::identity(3).with_fiber(FIBER_SPATIAL, shear(3));
        let lhs = a.compose(&b).compose(&c);
        let rhs = a.compose(&b.compose(&c));
        assert_eq!(lhs.realize(), rhs.realize(), "composition is associative");
    }

    /// THE ASYMMETRY↔SYMMETRY LAW: description ∘ solution = identity. The germ is
    /// the description; solution() deduces g⁻¹; composed, the coboundary CLOSES.
    /// (An exact homogeneous isometry matrix, fabricated to verify the group law.)
    #[test]
    fn description_composed_with_its_solution_is_identity() {
        let g = Element::identity(3).with_fiber(FIBER_SPATIAL, qm(&[&[0, -1, 3], &[1, 0, -2], &[0, 0, 1]]));
        let sol = g.solution().expect("an invertible description has a solution");
        assert!(g.compose(&sol).is_identity(), "g ∘ g⁻¹ = e — the coboundary closes");
        assert!(sol.compose(&g).is_identity(), "g⁻¹ ∘ g = e");
    }

    /// The solution is the INVERSE DEDUCTION: applying g⁻¹ to the image recovers
    /// the origin — the realized inverse matrix carries the moved point back.
    #[test]
    fn the_solution_inverts_the_described_motion() {
        let g = Element::identity(3).with_fiber(FIBER_SPATIAL, qm(&[&[-1, 0, 4], &[0, -1, 4], &[0, 0, 1]]));
        let fwd = g.realize();
        let inv = g.solution().unwrap().realize();
        // p = (1,2,1) → g·p, then g⁻¹ carries it back.
        let apply = |m: &QMatrix, v: &[Q; 3]| -> Vec<Q> {
            (0..3).map(|i| (0..3).map(|k| &m[i][k] * &v[k]).fold(Q::zero(), |s, x| s + x)).collect()
        };
        let p = [q(1), q(2), q(1)];
        let moved = apply(&fwd, &p);
        let recovered = apply(&inv, &[moved[0].clone(), moved[1].clone(), moved[2].clone()]);
        assert_eq!(recovered, vec![q(1), q(2), q(1)], "g⁻¹ deduces the origin back from the image");
    }

    /// BRACKET IS ANTISYMMETRIC: [X,Y] = −[Y,X] (XY−YX).
    #[test]
    fn bracket_is_antisymmetric() {
        let x = Element::identity(3).with_fiber(FIBER_SPATIAL, qm(&[&[0, 1, 0], &[0, 0, 0], &[0, 0, 0]]));
        let y = Element::identity(3).with_fiber(FIBER_SPATIAL, qm(&[&[0, 0, 0], &[1, 0, 0], &[0, 0, 0]]));
        let xy = x.bracket(&y);
        let yx = y.bracket(&x);
        // [X,Y] + [Y,X] = 0 ⇒ the two commutator generators are exact negatives.
        let a = xy.fiber(FIBER_SPATIAL).expect("[X,Y] is nonzero here");
        let b = yx.fiber(FIBER_SPATIAL).expect("[Y,X] is nonzero here");
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(a[i][j], -&b[i][j], "[X,Y] = −[Y,X] at ({i},{j})");
            }
        }
    }

    /// BRACKET SATISFIES JACOBI: [[X,Y],Z] + [[Y,Z],X] + [[Z,X],Y] = 0.
    #[test]
    fn bracket_satisfies_jacobi() {
        let e = |m: QMatrix| Element::identity(3).with_fiber(FIBER_SPATIAL, m);
        let x = e(qm(&[&[0, 1, 0], &[0, 0, 0], &[0, 0, 0]]));
        let y = e(qm(&[&[0, 0, 0], &[1, 0, 0], &[0, 0, 0]]));
        let z = e(qm(&[&[1, 0, 0], &[0, -1, 0], &[0, 0, 0]]));
        let term = |p: &Element, q_: &Element, r: &Element| p.bracket(q_).bracket(r);
        let t1 = term(&x, &y, &z);
        let t2 = term(&y, &z, &x);
        let t3 = term(&z, &x, &y);
        // Sum the three commutator generators; Jacobi ⇒ zero matrix.
        let g1 = t1.fiber(FIBER_SPATIAL).cloned().unwrap_or_else(|| vec![vec![Q::zero(); 3]; 3]);
        let g2 = t2.fiber(FIBER_SPATIAL).cloned().unwrap_or_else(|| vec![vec![Q::zero(); 3]; 3]);
        let g3 = t3.fiber(FIBER_SPATIAL).cloned().unwrap_or_else(|| vec![vec![Q::zero(); 3]; 3]);
        for i in 0..3 {
            for j in 0..3 {
                let s = &g1[i][j] + &g2[i][j] + &g3[i][j];
                assert!(s.is_zero(), "Jacobi identity holds at ({i},{j})");
            }
        }
    }

    /// δ: a residual realizes an ANTISYMMETRIC field over the core Δ, and its
    /// solution (the reverse germ) negates it — description ⊕ solution = 0 (closure).
    #[test]
    fn the_residual_coboundary_is_antisymmetric_and_closes() {
        let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
        let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)]; // (0,1): 2→9
        let g = germinate(&a, &b).unwrap();
        let d = germ_delta(&g);
        for i in 0..d.dim {
            for j in 0..d.dim {
                assert_eq!(d.entries[i][j][0], -&d.entries[j][i][0], "Δ_ij = −Δ_ji");
            }
        }
        // The reverse description negates the field → they sum to the zero field.
        let rev = germinate(&b, &a).unwrap();
        let dr = germ_delta(&rev);
        for i in 0..d.dim {
            for j in 0..d.dim {
                assert!((&d.entries[i][j][0] + &dr.entries[i][j][0]).is_zero(), "δ ⊕ δ⁻¹ = 0");
            }
        }
    }

    /// Conjugation by an ISOMETRY over the CORE Δ preserves the antisymmetric
    /// field — the element realized as an Operator acts by A·Δ·A⁻¹ and the
    /// coboundary structure is invariant under the symmetry group (orthogonal A;
    /// a general shear would NOT preserve it — antisymmetry is an isometry
    /// invariant, which is exactly the physically meaningful case).
    #[test]
    fn conjugation_preserves_the_antisymmetric_field() {
        // A rigid rotation (orthogonal, det 1) — an exact isometry matrix fixture.
        let g = Element::identity(3).with_fiber(FIBER_SPATIAL, qm(&[&[0, -1, 0], &[1, 0, 0], &[0, 0, 1]]));
        let op = g.as_operator(0).expect("an isometry lifts to an Operator");
        let mut field = Delta::zero(3, 1);
        field.set_antisym(0, 1, vec![q(5)]);
        field.set_antisym(0, 2, vec![q(-3)]);
        let out = op.apply(&field).expect("conjugation");
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(out.entries[i][j][0], -&out.entries[j][i][0], "A·Δ·A⁻¹ stays antisymmetric");
            }
        }
    }

    // ─── THE GERM CARRIER: centered coboundary chains (post-excision) ───────

    /// THE RESIDUAL GERMINATES: a pivot that no motor closes becomes a first-class
    /// germ on the residual lane — the description that used to be discarded. Its
    /// magnitude is nonzero there, so the POW++ gate admits it.
    #[test]
    fn a_residual_germinates_as_a_coboundary_germ() {
        // (0,1): 2→9 with the rest fixed — not a rigid motion; a pure residual.
        let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
        let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)];
        let g = germinate(&a, &b).unwrap();
        let germ = from_residual(&g);
        assert_eq!(germ.lanes(), [false, false, true, false], "the residual lane, not 𝒮/𝒢");
        let mag = germ.magnitude();
        assert!(mag[2] >= 1 && mag[0] == 0 && mag[1] == 0, "cost on the residual lane only");
        // A residual has no group inverse (its block is antisymmetric/singular);
        // its solution is the reverse coboundary (from_residual of b→a), not a
        // matrix inverse — the group solution() correctly declines.
        assert!(germ.solution().is_none(), "no single-matrix inverse for a coboundary");
        let reverse = from_residual(&germinate(&b, &a).unwrap());
        assert_ne!(germ, reverse, "the reverse description is a distinct germ");
    }

    /// The transposition quotient is the CENTERED chain: the same deformation at
    /// any position is ONE substance (min-corner framed to origin), while a
    /// deformation with the SAME hue tally but DIFFERENT geometry stays DISTINCT —
    /// the Step-1 conflation, cured (the tally never decides substance).
    #[test]
    fn substance_is_the_centered_chain() {
        let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
        let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)]; // (0,1): 2→9
        // The same deformation transposed +7 rows, +5 cols.
        let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 7, c + 5, *h)).collect();
        let b_far: Vec<_> = b.iter().map(|(r, c, h)| (r + 7, c + 5, *h)).collect();
        let here = from_residual(&germinate(&a, &b).unwrap());
        let far = from_residual(&germinate(&a_far, &b_far).unwrap());
        assert_eq!(here, far, "same deformation, any position — ONE word");
        assert!(here.same_action(&far), "one substance");
        // Equal tally, different geometry — the cured conflation: {1→0 here, 0→4
        // there} at two different relative offsets must stay DISTINCT.
        let g1 = from_residual(&vec![(0, 0, 1, 0), (1, 1, 0, 4)]);
        let g2 = from_residual(&vec![(1, 0, 1, 0), (0, 2, 0, 4)]);
        assert!(!g1.same_action(&g2), "equal tally, different geometry — distinct substance");
        assert_ne!(g1.address(), g2.address(), "and distinct identity");
    }

    /// P7: a germ round-trips through bytes losslessly — chain, address, magnitude,
    /// and action all survive, so a persisted basis reloads identically.
    #[test]
    fn a_germ_round_trips_through_bytes() {
        let residual = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap());
        let swap = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1), (1, 0, 1)], &[(0, 0, 1u8), (0, 1, 1), (0, 2, 1)]).unwrap());
        for g in [&residual, &swap] {
            let bytes = g.to_bytes();
            let mut pos = 0;
            let back = Element::from_bytes(&bytes, &mut pos).expect("reload");
            assert_eq!(pos, bytes.len(), "consumed exactly");
            assert_eq!(&back, g, "full equality (dim, word, mag, fibers, chain)");
            assert_eq!(back.address(), g.address(), "address survives");
            assert_eq!(back.sub(), g.sub(), "the centered chain survives");
            assert!(back.same_action(g), "the substance survives");
        }
    }

    /// THE TORSE CAPACITY THEOREM: a 3-cycle hue rotation derived from ONE
    /// witnessed section closes (σ³ = id), so the cycle speaks the THIRD orbit
    /// member that was never individually witnessed — capacity earned by exact
    /// structure, in O(1) storage where the list held O(n).
    #[test]
    fn a_closed_cycle_speaks_unwitnessed_members() {
        // base: transitions 1→2, 2→3, 3→1 over three cells.
        let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
        let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
        let mut t = from_residual(&germinate(&a, &b).unwrap());
        // One witnessed sibling: σ·base where σ = {1↦2, 2↦3, 3↦1}.
        assert!(t.puncture(vec![(2, 3), (3, 1), (1, 2)]));
        assert_eq!(t.torse(), 1, "the section condenses into the cycle");
        assert_eq!(t.torsion().len(), 1, "one torsion cycle (σ, 3)");
        assert_eq!(t.torsion()[0].1, 3, "the period is the torsion_order");
        assert!(t.sections().is_empty(), "the free list drained into the cycle");
        // THE UNWITNESSED MEMBER: σ²·base = transitions 3→1, 1→2, 2→3 — never
        // punctured, never seen — yet the cycle speaks it.
        let unwitnessed = vec![(0, 0, 3, 1), (0, 1, 1, 2), (0, 2, 2, 3)];
        assert!(t.matches_chain(&unwitnessed), "capacity beyond the evidence, exactly");
        assert!(!t.matches_chain(&vec![(0, 0, 5, 6), (0, 1, 1, 2), (0, 2, 2, 3)]), "no hallucination");
        assert_eq!(t.fiber_span(), 3, "the volume counts the full orbit");
    }

    /// A SWAP is the smallest torsion (ℤ/2): one witnessed section closes at
    /// period 2 and condenses. An orbit that neither closes nor spans two
    /// sections extracts NOTHING — no false structure from thin evidence.
    #[test]
    fn a_swap_torses_and_thin_evidence_does_not() {
        let mut swap = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 9)], &[(0, 0, 2u8), (0, 1, 9)]).unwrap());
        assert!(swap.puncture(vec![(2, 1)]));
        assert_eq!(swap.torse(), 1, "sigma 1↦2, 2↦1 closes at period 2");
        assert_eq!(swap.torsion()[0].1, 2);
        // Thin evidence: sections whose hues never recur — no orbit spans two,
        // nothing closes — the free part stays free.
        let mut open = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 9)], &[(0, 0, 2u8), (0, 1, 9)]).unwrap());
        assert!(open.puncture(vec![(3, 4)]));
        assert!(open.puncture(vec![(5, 6)]));
        assert_eq!(open.torse(), 0, "no torsion invented from unrelated sections");
        assert_eq!(open.sections().len(), 2, "the free part is untouched");
    }

    /// Torsion survives serde: the cycle (σ, ℓ) round-trips, and the reloaded
    /// thought still speaks the unwitnessed orbit member.
    #[test]
    fn torsion_round_trips_through_bytes() {
        let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
        let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
        let mut t = from_residual(&germinate(&a, &b).unwrap());
        t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
        t.torse();
        let bytes = t.to_bytes();
        let mut pos = 0;
        let back = Element::from_bytes(&bytes, &mut pos).expect("reload");
        assert_eq!(pos, bytes.len());
        assert_eq!(back, t, "full equality including the torsion");
        assert!(back.matches_chain(&vec![(0, 0, 3, 1), (0, 1, 1, 2), (0, 2, 2, 3)]), "capacity survives restart");
    }

    /// THE CANCELING TALLY IS NOT IDENTITY: L→bar's bidirectional hue swap cancels
    /// the tally to zero — but the CHAIN keeps it a real, non-identity germ with
    /// exact magnitude, and its reverse is a distinct word.
    #[test]
    fn a_bidirectional_swap_does_not_collapse_to_zero() {
        let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)]; // L
        let b = [(0, 0, 1u8), (0, 1, 1), (0, 2, 1)]; // bar
        let l_to_bar = from_residual(&germinate(&a, &b).unwrap());
        assert_eq!(l_to_bar.magnitude(), [0, 0, 2, 0], "two changed cells, exact — not a canceled 0");
        assert_eq!(l_to_bar.lanes(), [false, false, true, false], "a real residual germ");
        assert!(!l_to_bar.is_identity(), "the chain decides — a canceled tally is not identity");
        assert!(!l_to_bar.substance().is_identity(), "and its substance is real");
        let bar_to_l = from_residual(&germinate(&b, &a).unwrap());
        assert_ne!(l_to_bar.address(), bar_to_l.address(), "distinct descriptions, distinct address");
        assert!(!l_to_bar.same_action(&bar_to_l), "and distinct substance");
    }

    /// Multi-axial: distinct axes compose INDEPENDENTLY (no cross-talk, no cap on
    /// how many axes germinate). A germinated fiber extends without disturbing others.
    #[test]
    fn axes_are_independent_and_extensible() {
        let spatial = Element::identity(3).with_fiber(FIBER_SPATIAL, shear(1));
        let germ = Element::identity(3).with_fiber(FIBER_GERMINATED, shear(7));
        let both = spatial.compose(&germ);
        assert_eq!(both.axes(), vec![FIBER_SPATIAL, FIBER_GERMINATED], "both axes present");
        assert_eq!(both.fiber(FIBER_SPATIAL), spatial.fiber(FIBER_SPATIAL), "spatial axis intact");
        assert_eq!(both.max_axis(), Some(FIBER_GERMINATED));
        // A new axis changes identity but not the existing axes' generators.
        assert_ne!(both.address(), spatial.address());
    }
}
