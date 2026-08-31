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
use crate::transducer::germinate::ResidualGerm;
use crate::transducer::motor::ConformalMotor;
use num_bigint::BigInt;
use num_traits::{One, Zero};

/// Reserved base fibers (the four strata); germinated fibers take indices ≥ 4.
pub const FIBER_SPATIAL: u16 = 0;
pub const FIBER_GAUGE: u16 = 1;
pub const FIBER_TEMPORAL: u16 = 2;
pub const FIBER_SHELL: u16 = 3;
/// First index available to germinated (residual/new-axis) fibers.
pub const FIBER_GERMINATED: u16 = 4;

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

/// A germ as a sparse multi-axial element: fibers sorted by index, each carrying
/// a `dim×dim` exact rational generator. The element itself is the DESCRIPTION.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    dim: usize,
    fibers: Vec<(u16, QMatrix)>,
}

impl Element {
    /// The identity element on `dim` coordinates — the empty description (no
    /// transition on any axis). `dim` is the representation dimension shared by
    /// every fiber so they compose/bracket in a common GL(dim,ℚ).
    pub fn identity(dim: usize) -> Self {
        Element { dim, fibers: Vec::new() }
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
        self.fibers.is_empty()
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

    /// Canonical ANASTOME bytes: `[fiber:2][dim:2]` then each entry's exact ℚ
    /// numer/denom, in fiber then row-major order. Order-independent (fibers are
    /// kept sorted). WIRE IDENTITY ONLY — never a reasoning operand.
    pub fn canonical(&self) -> Vec<u8> {
        let mut out = Vec::new();
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

    /// The element's canonical ADDRESS — a digest of its ANASTOME form. Wire/cache
    /// identity for matching consilience receipts; never a reasoning operand.
    pub fn address(&self) -> [u8; 32] {
        *blake3::hash(&self.canonical()).as_bytes()
    }
}

/// Realize a conformal motor as a multi-axial element: the spatial fiber holds the
/// exact 3×3 homogeneous similarity `[[λR, t],[0,1]]` acting on `(r,c,1)`. This is
/// the DESCRIPTION of the motion; its [`Element::solution`] is the inverse motor.
pub fn from_conformal(m: &ConformalMotor) -> Element {
    let r = m.rotation.matrix();
    let l = &m.dilation;
    let mat = vec![
        vec![l * q(r[0][0]), l * q(r[0][1]), m.translation.0.clone()],
        vec![l * q(r[1][0]), l * q(r[1][1]), m.translation.1.clone()],
        vec![q(0), q(0), q(1)],
    ];
    Element::identity(3).with_fiber(FIBER_SPATIAL, mat)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::germinate::germinate;
    use crate::transducer::motor::D4;

    fn qm(rows: &[&[i64]]) -> QMatrix {
        rows.iter().map(|r| r.iter().map(|&v| q(v)).collect()).collect()
    }

    /// A shear generator (det 1, so it lifts to an Operator).
    fn shear(a: i64) -> QMatrix {
        qm(&[&[1, a, 0], &[0, 1, 0], &[0, 0, 1]])
    }

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
    #[test]
    fn description_composed_with_its_solution_is_identity() {
        let g = from_conformal(&ConformalMotor {
            dilation: q(1),
            rotation: D4::R90,
            translation: (q(3), q(-2)),
        });
        let sol = g.solution().expect("a motion is reversible — it has a solution");
        assert!(g.compose(&sol).is_identity(), "g ∘ g⁻¹ = e — the coboundary closes");
        assert!(sol.compose(&g).is_identity(), "g⁻¹ ∘ g = e");
    }

    /// The solution is the INVERSE DEDUCTION: applying g⁻¹ to B recovers A. Here
    /// the realized inverse matrix carries the moved point back to its origin.
    #[test]
    fn the_solution_inverts_the_described_motion() {
        let m = ConformalMotor { dilation: q(1), rotation: D4::R180, translation: (q(4), q(4)) };
        let g = from_conformal(&m);
        let inv = g.solution().unwrap().realize();
        // origin point p=(1,2,1) → B under g, then back under g⁻¹.
        let (br, bc) = m.apply_point(&q(1), &q(2));
        let bvec = [br, bc, q(1)];
        let recovered: Vec<Q> =
            (0..3).map(|i| (0..3).map(|k| &inv[i][k] * &bvec[k]).fold(Q::zero(), |s, x| s + x)).collect();
        assert_eq!(recovered, vec![q(1), q(2), q(1)], "g⁻¹ deduces A back from B");
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
        // A rigid rotation (orthogonal, det 1) — an isometry description.
        let g = from_conformal(&ConformalMotor { dilation: q(1), rotation: D4::R90, translation: (q(0), q(0)) });
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
