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
#[derive(Debug, Clone)]
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
    torsion: Vec<(Vec<(u8, u8)>, u32, u16)>,
    /// THE ANCHOR FIBER (Step 7) — γ = (π_anchor, Δ_substance): the recursive
    /// selection term restricting WHERE this word's substance applies on a
    /// scene. None = ⊤, the unconstrained carrier (pure topological actions,
    /// and every word born before selection entered the language).
    pub(crate) anchor: Option<crate::transducer::anchor::Sel>,
    /// THE CONNECTION — for every assignment spoken on this fibre (the base's
    /// own, a section, a torsion-orbit member), the versors V ∈ Spin(4,1)(ℚ)
    /// under which a sibling was observed and carried into this frame:
    /// `(assignment, V)`. The parallel-transport record of the bundle; the
    /// set of V reached for one assignment is its holonomy. Vec-of-relations;
    /// excluded from the address (a connection deepens the thought, it does
    /// not rename it).
    transitions: Vec<(Vec<(u8, u8)>, Versor, u16)>,
    /// The memoized address — a measurement of the base (word, dim, fibers),
    /// taken once and cleared wherever the base mutates. Never serialized.
    address: std::sync::OnceLock<[u8; 32]>,
    /// THE GAUGE AXIS KEYED BY PROFILE: every assignment this word speaks
    /// (its own, its sections, its torsion orbits) reached by the key of
    /// its gauge charge — one read names the 0 / 1 / many assignments a
    /// body could be spoken with. Memoized; cleared wherever the fibre
    /// mutates. Never serialized.
    profiles: std::sync::OnceLock<Vec<([u8; 32], Vec<usize>)>>,
    /// Decoded sentence Form, memoized from the description tape. Cleared
    /// wherever the tape mutates. Never serialized.
    form: std::sync::OnceLock<Option<crate::transducer::sentence::Form>>,
}

/// THE GAUGE PROFILE of an assignment: its transition classes with their
/// counts reduced to proportions (the counts divided by their common
/// factor), so a dilation by ρ — which scales every count by ρ² — has the
/// same profile. Sorted; one run-length pass over the sorted assignment.
pub fn gauge_profile(asg: &[(u8, u8)]) -> Vec<((u8, u8), u32)> {
    let mut sorted = asg.to_vec();
    sorted.sort_unstable();
    let mut out: Vec<((u8, u8), u32)> = Vec::new();
    for t in sorted {
        match out.last_mut() {
            Some((k, n)) if *k == t => *n += 1,
            _ => out.push((t, 1)),
        }
    }
    let g = out.iter().fold(0u32, |g, (_, n)| gcd(g, *n));
    if g > 1 {
        for (_, n) in out.iter_mut() {
            *n /= g;
        }
    }
    out
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// The key of a gauge profile: one coordinate for the whole charge.
pub fn gauge_key(asg: &[(u8, u8)]) -> [u8; 32] {
    let mut buf = Vec::new();
    for ((b, a), n) in gauge_profile(asg) {
        buf.push(b);
        buf.push(a);
        buf.extend_from_slice(&n.to_le_bytes());
    }
    *blake3::hash(&buf).as_bytes()
}

/// THE TRANSPORT of the bundle: a rational conformal versor V ∈ Spin(4,1)(ℚ)
/// (translation, rotation by any rational rotor, reflection, exact dilation,
/// and their products), acting by the sandwich V X Ṽ on the null points of
/// a support. The lattice isometries are the special case; no table of
/// images exists anywhere.
pub type Versor = crate::transducer::cga::Mv;

pub fn identity_versor() -> Versor {
    Versor::scalar(num_rational::BigRational::from_integer(BigInt::from(1)))
}

/// The legacy lattice transport (a D4 index over the eight isometries in
/// their historical order, then an integer offset) lifted to its versor:
/// the word of the alphabet whose action on the row and column steps is
/// the isometry, realised as a conformal versor, then the translator. The
/// index table is the legacy wire's convention; decoding only.
pub fn legacy_versor(d4: u8, tr: i64, tc: i64) -> Option<Versor> {
    use num_rational::BigRational;
    let images: [fn(i64, i64) -> (i64, i64); 8] = [
        |r, c| (r, c),
        |r, c| (c, -r),
        |r, c| (-r, -c),
        |r, c| (-c, r),
        |r, c| (r, -c),
        |r, c| (-r, c),
        |r, c| (c, r),
        |r, c| (-c, -r),
    ];
    let image = images.get(d4 as usize)?;
    let q = |v: i64| BigRational::from_integer(BigInt::from(v));
    // The isometry is the images of the row and column steps, spelled in
    // the alphabet; the versor is realised from the word, never tabulated.
    let spelling = crate::transducer::alphabet::spell_images(image(1, 0), image(0, 1))?;
    conformal_of(&spelling, &(q(tr), q(tc)))
}

/// THE CONFORMAL REALISATION OF A WORD: the versor of Spin(4,1)(ℚ) whose
/// action on the plane is the spelling at its dilation, placed by `shift`.
/// The rotation rotor is `(1 + cos) − sin·e₁₂` (the half-angle form, exact
/// whenever the rotation is rational; the half turn is `e₁₂` itself), the
/// orientation sheet is the odd versor `e₁`, the dilation and translation
/// their versors. Verified against the word's own block; `None` when the
/// word does not preserve the plane.
pub fn conformal_of(spelling: &crate::transducer::alphabet::Spelling, shift: &(Q, Q)) -> Option<Versor> {
    use crate::transducer::cga;
    let block = spelling.block()?;
    let rho = &spelling.rho;
    if rho.is_zero() {
        return None;
    }
    let r: [[Q; 2]; 2] = std::array::from_fn(|i| std::array::from_fn(|j| &block[i][j] / rho));
    let det = &r[0][0] * &r[1][1] - &r[0][1] * &r[1][0];
    let flipped = if det.is_one() {
        false
    } else if (-&det).is_one() {
        true
    } else {
        return None;
    };
    let (c, s) = (&r[0][0], &r[1][0]);
    let rot = if (-c).is_one() {
        cga::rotator(&Q::zero(), &Q::one())
    } else {
        cga::rotator(&(Q::one() + c), &(-s))
    };
    let mut v = cga::translator(&shift.0, &shift.1).geo(&cga::dilator(rho)).geo(&rot);
    if flipped {
        // The sheet the alphabet spells negates the column step; the odd
        // versor `e₁` negates the row step, so the half turn composes it.
        v = v.geo(&cga::rotator(&Q::zero(), &Q::one())).geo(&cga::reflector());
    }
    let target = vec![
        vec![block[0][0].clone(), block[0][1].clone(), shift.0.clone()],
        vec![block[1][0].clone(), block[1][1].clone(), shift.1.clone()],
        vec![Q::zero(), Q::zero(), Q::one()],
    ];
    if affine_of(&v)? != target {
        return None;
    }
    Some(v)
}

/// Projective equality of two versors: the same transport up to a nonzero
/// rational scale (versors act projectively on the cone).
pub fn same_versor(a: &Versor, b: &Versor) -> bool {
    crate::transducer::cga::projective_ratio(a, b).is_some()
}

/// The translator by an integer lattice offset — the lattice peel's transport
/// as a rational versor.
pub fn translator_versor(r: i64, c: i64) -> Versor {
    crate::transducer::cga::translator(
        &num_rational::BigRational::from_integer(BigInt::from(r)),
        &num_rational::BigRational::from_integer(BigInt::from(c)),
    )
}

/// The versor carrying `from`'s support onto `to`'s, solved in closed form
/// on the two shapes (see `blade::versor_between`) and verified corner by
/// corner. None when no rational versor carries one onto the other — the
/// exact congruence test on the conformal orbit, returning the transport
/// operator rather than a verdict.
pub fn transition(from: &[(i64, i64, u8, u8)], to: &[(i64, i64, u8, u8)]) -> Option<Versor> {
    let from_shape = shape_of_chain(from)?;
    transition_from_shape(&from_shape, to)
}

/// The shape of a chain's support (None for an empty chain) — lifted once
/// and reused across the candidates a lookup compares it with.
pub fn shape_of_chain(chain: &[(i64, i64, u8, u8)]) -> Option<crate::transducer::blade::Shape> {
    let mut a = positions(chain);
    a.sort_unstable();
    a.dedup();
    if a.is_empty() {
        return None;
    }
    Some(crate::transducer::blade::shape_of(&a))
}

/// The versor carrying an already-lifted shape onto `to`'s support.
pub fn transition_from_shape(
    from: &crate::transducer::blade::Shape,
    to: &[(i64, i64, u8, u8)],
) -> Option<Versor> {
    let to_shape = shape_of_chain(to)?;
    crate::transducer::blade::versor_between(from, &to_shape)
}

/// The cell whose unit square contains a rational point — exact
/// containment on the sensor's own lattice (the floor of a rational is an
/// exact integer, not a rounding).
pub fn cell_of(p: &(num_rational::BigRational, num_rational::BigRational)) -> (i64, i64) {
    let f = |x: &num_rational::BigRational| -> i64 { x.floor().to_integer().try_into().unwrap_or(i64::MAX) };
    (f(&p.0), f(&p.1))
}

/// Carry a cell's center by V and read the base point back (None at
/// infinity).
pub fn carry_center(v: &Versor, r: i64, c: i64) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    let x = num_rational::BigRational::from_integer(BigInt::from(r)) + &half;
    let y = num_rational::BigRational::from_integer(BigInt::from(c)) + &half;
    crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y)))
}

/// Carry a whole cell by V as a blade — its unit square — and read back the
/// cells whose centers the image square holds: one cell under an
/// isometry, an s×s block under a dilation by s, the one shared cell (or
/// none) under a shrink. The support is carried forward; nothing outside
/// it is visited. None at infinity.
pub fn carry_cell(v: &Versor, r: i64, c: i64) -> Option<Vec<(i64, i64)>> {
    let corner = |dr: i64, dc: i64| -> Option<(num_rational::BigRational, num_rational::BigRational)> {
        let x = num_rational::BigRational::from_integer(BigInt::from(r + dr));
        let y = num_rational::BigRational::from_integer(BigInt::from(c + dc));
        crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y)))
    };
    let (p0, p1) = (corner(0, 0)?, corner(1, 1)?);
    let half = num_rational::BigRational::new(BigInt::from(1), BigInt::from(2));
    // The cells whose center (i + ½) lies in the half-open image range
    // [lo, hi): i ≥ ⌈lo − ½⌉ and i + ½ < hi.
    let span = |a: &num_rational::BigRational, b: &num_rational::BigRational| -> (i64, i64) {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let first: i64 = (lo - &half).ceil().to_integer().try_into().unwrap_or(i64::MAX);
        let last: i64 = (hi - &half).ceil().to_integer().try_into().unwrap_or(i64::MIN); // exclusive
        (first, last)
    };
    let (r0, r1) = span(&p0.0, &p1.0);
    let (c0, c1) = span(&p0.1, &p1.1);
    let mut out = Vec::new();
    for i in r0..r1 {
        for j in c0..c1 {
            out.push((i, j));
        }
    }
    Some(out)
}

/// THE PULLBACK of a sibling's assignment onto a base support through the
/// versor V carrying the sibling onto the base: each base cell's center is
/// carried back by V⁻¹ to the sibling's frame, and the sibling cell whose
/// square contains it speaks the transition. Exact at every scale (a
/// dilated sibling's block, a shrunken sibling's shared cell); None when a
/// base cell lands outside the sibling.
pub fn pullback(
    base: &[(i64, i64)],
    sibling: &[(i64, i64, u8, u8)],
    v: &Versor,
) -> Option<Vec<(u8, u8)>> {
    let inv = v.versor_inverse()?;
    let mut sorted: Vec<(i64, i64, u8, u8)> = sibling.to_vec();
    sorted.sort_unstable();
    base.iter()
        .map(|&(r, c)| {
            let (sr, sc) = cell_of(&carry_center(&inv, r, c)?);
            sorted
                .binary_search_by(|x| (x.0, x.1).cmp(&(sr, sc)))
                .ok()
                .map(|i| (sorted[i].2, sorted[i].3))
        })
        .collect()
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

/// Canonical key of one hue pair. The pair is the chart coordinate; the
/// 32-byte form is the PATRICIA key, never a scan handle.
pub fn hue_pair_key(before: u8, after: u8) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[0] = before;
    k[1] = after;
    k
}

/// Canonical key of a whole assignment (the exact list of pairs).
pub fn assignment_key(asg: &[(u8, u8)]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(asg.len() * 2);
    for &(b, a) in asg {
        buf.push(b);
        buf.push(a);
    }
    *blake3::hash(&buf).as_bytes()
}

/// One transition class, or several keyed by the pair.
pub enum GaugeFibers {
    One((u8, u8), Vec<(i64, i64, u8, u8)>),
    Many(crate::network::nerve::Keyed<Vec<(i64, i64, u8, u8)>>),
}

/// THE GAUGE FIBERS of a chain: the chain opened along its fiber
/// coordinate, one sub-chain per transition class. Indexed by the pair
/// key. One class is `One`; two or more is `Many`. Empty is `None`.
pub fn gauge_fibers(chain: &[(i64, i64, u8, u8)]) -> Option<GaugeFibers> {
    if chain.is_empty() {
        return None;
    }
    let mut idx: crate::network::nerve::Keyed<Vec<(i64, i64, u8, u8)>> =
        crate::network::nerve::Keyed::new();
    for &cell in chain {
        let k = hue_pair_key(cell.2, cell.3);
        match idx.get_mut(&k) {
            Some(fiber) => fiber.push(cell),
            None => {
                idx.insert(k, vec![cell]);
            }
        }
    }
    let mut first = None;
    let mut many = false;
    for (k, fiber) in idx.fiber(&crate::network::nerve::ROOT) {
        match first {
            None => first = Some(((k[0], k[1]), fiber.clone())),
            Some(_) => {
                many = true;
                break;
            }
        }
    }
    match (many, first) {
        (false, Some((class, cells))) => Some(GaugeFibers::One(class, cells)),
        _ => Some(GaugeFibers::Many(idx)),
    }
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
            transitions: Vec::new(),
            address: std::sync::OnceLock::new(),
            profiles: std::sync::OnceLock::new(),
            form: std::sync::OnceLock::new(),
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
        self.address.take();
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

    /// Set the per-lane support this word carries (an opened axis carries
    /// the listing's measure and the charts it joined).
    pub fn set_magnitude(&mut self, mag: [u64; 4]) {
        self.mag = mag;
    }

    pub fn is_identity(&self) -> bool {
        // A residual with a canceling hue tally (a bidirectional swap) has empty
        // FIBERS but a real chain — the chain decides; the tally never does.
        // A word with a description tape (a sentence form, a rewrite, a
        // translation) is never the identity: its meaning is on the tape,
        // not in a fibre, and a closed form has no chain by construction.
        self.fibers.is_empty()
            && self.sub.is_empty()
            && !self.is_sentence()
            && !self.is_rewrite()
            && !self.is_translation()
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

    /// The ANASTOME description tape (connection or sentence).
    pub fn description(&self) -> &[u8] {
        &self.word
    }

    /// Replace the description tape and drop the memoized address and form.
    pub fn set_description(&mut self, word: Vec<u8>) {
        self.word = word;
        self.address.take();
        self.form.take();
    }

    /// The sentence Form this word carries, decoded once from the tape.
    pub fn form(&self) -> Option<&crate::transducer::sentence::Form> {
        self.form
            .get_or_init(|| crate::transducer::sentence::from_element_form(self))
            .as_ref()
    }

    /// The free sections along the punctured axis (cycles live in `torsion`).
    pub fn sections(&self) -> &[Vec<(u8, u8)>] {
        &self.sections
    }

    /// The torsion cycles of the axis: `(generator σ, period ℓ)` each.
    pub fn torsion(&self) -> &[(Vec<(u8, u8)>, u32, u16)] {
        &self.torsion
    }

    /// The connection: every (assignment, g) under which a sibling was
    /// transported into this frame.
    pub fn transitions(&self) -> &[(Vec<(u8, u8)>, Versor, u16)] {
        &self.transitions
    }

    /// Record a dislocation versor on the fibre: (assignment, V, fibre)
    /// excluded from the address. Duplicate (asg, V, fibre) not stored twice.
    pub fn record_dislocation(&mut self, asg: Vec<(u8, u8)>, g: Versor, fibre: u16) {
        if !self.transitions.iter().any(|(a, h, f)| a == &asg && same_versor(h, &g) && *f == fibre) {
            self.transitions.push((asg, g, fibre));
        }
    }

    /// File a torsion cycle `(σ, ℓ)` on fibre `fibre`. Duplicate generators
    /// on the same fibre are not stored twice.
    pub fn record_torsion(&mut self, sigma: Vec<(u8, u8)>, period: u32, fibre: u16) {
        if period == 0 {
            return;
        }
        if !self.torsion.iter().any(|(s, _, f)| s == &sigma && *f == fibre) {
            self.torsion.push((sigma, period, fibre));
            self.profiles.take();
        }
    }

    /// Take a form's (or nerve's) torsion carrier whole. The element's
    /// torsion is that carrier, not a second list re-recorded cycle by cycle.
    pub fn adopt_torsion(&mut self, torsion: &crate::network::nerve::Keyed<(Vec<(u8, u8)>, u32, u16)>) {
        self.torsion.clear();
        for (_, (sigma, ell, _)) in torsion.fiber(&crate::network::nerve::ROOT) {
            if *ell > 0 {
                self.torsion.push((sigma.clone(), *ell, FIBER_SPATIAL));
            }
        }
        self.profiles.take();
    }

    /// The fiber's total spoken span — base + free sections + every torsion-orbit
    /// member (the volume metric; cycles count their full period).
    pub fn fiber_span(&self) -> usize {
        1 + self.sections.len()
            + self.torsion.iter().map(|(_, l, _)| (*l as usize).saturating_sub(1)).sum::<usize>()
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
    /// The assignments this word speaks with a given gauge charge, read by
    /// the charge's key: 0 is no such gauge, 1 is the citation, many is
    /// multiplicity on the fibre. One read; the axis is never walked.
    pub fn assignments_by_profile(&self, key: &[u8; 32]) -> Vec<usize> {
        let table = self.profiles.get_or_init(|| {
            let mut rows: Vec<([u8; 32], Vec<usize>)> = Vec::new();
            let mut keyed: Vec<([u8; 32], usize)> =
                self.assignments().iter().enumerate().map(|(i, a)| (gauge_key(a), i)).collect();
            keyed.sort_unstable();
            for (k, i) in keyed {
                match rows.last_mut() {
                    Some((last, is)) if *last == k => is.push(i),
                    _ => rows.push((k, vec![i])),
                }
            }
            rows
        });
        match table.binary_search_by(|(k, _)| k.cmp(key)) {
            Ok(i) => table[i].1.clone(),
            Err(_) => Vec::new(),
        }
    }

    pub fn assignments(&self) -> Vec<Vec<(u8, u8)>> {
        let base = assignment(&self.sub);
        let mut out = vec![base.clone()];
        out.extend(self.sections.iter().cloned());
        for (sigma, l, _) in &self.torsion {
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
    /// Which assignment (in [`assignments`] order: base, sections, torsion
    /// orbit) the centered chain sits on, if any — the gauge coordinate a
    /// citation names.
    pub fn matching_assignment(&self, chain: &[(i64, i64, u8, u8)]) -> Option<usize> {
        if self.sub.is_empty() || positions(&self.sub) != positions(chain) {
            return None;
        }
        let asg = assignment(chain);
        self.assignments().iter().position(|a| *a == asg)
    }

    pub fn matches_chain(&self, chain: &[(i64, i64, u8, u8)]) -> bool {
        if self.sub.is_empty() || positions(&self.sub) != positions(chain) {
            return false;
        }
        let asg = assignment(chain);
        let base = assignment(&self.sub);
        if asg == base || self.sections.contains(&asg) {
            return true;
        }
        for (sigma, l, _) in &self.torsion {
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
            // Only a CLOSED orbit is a cycle (σ^t = id on its support, the loop
            // witnessed at the mint): it condenses the sections it spans, even
            // one (a swap). An orbit that leaves its domain is not filed under
            // a period — those sections stay free; the open walk is the
            // remainder the word carries, never a winding it claims.
            if period > 0 && !spanned.is_empty() {
                for &i in spanned.iter().rev() {
                    self.sections.remove(i);
                }
                self.torsion.push((sigma, period, FIBER_SPATIAL));
                self.profiles.take();
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
        self.profiles.take();
        true
    }

    /// ATTACH — connect a congruent sibling to this bundle. Its chain is
    /// transported by g into this frame (g · supp(sibling) = supp(self)), the
    /// transported assignment becomes a section (or is recognised as one the
    /// fibre already speaks), and (assignment, g) joins the connection.
    /// Returns the sections the fibre gained (0 or 1); a g that does not
    /// carry the sibling onto this base changes nothing.
    pub fn attach(&mut self, sibling: &Element, g: &Versor) -> usize {
        let Some(asg) = pullback(&positions(&self.sub), &sibling.sub, g) else { return 0 };
        let gained = self.puncture(asg.clone()) as usize;
        if !self.transitions.iter().any(|(a, h, _)| *a == asg && same_versor(h, g)) {
            self.transitions.push((asg, g.clone(), FIBER_SPATIAL));
        }
        gained
    }

    /// Merge another placement's fibre into this bundle (adoption of a word
    /// already held at this address): its sections puncture here and its
    /// connection joins. Returns the count of sections newly spoken.
    pub fn merge_fibre(&mut self, other: &Element) -> usize {
        let mut added = 0usize;
        for s in &other.sections {
            if self.puncture(s.clone()) {
                added += 1;
            }
        }
        for (a, g, f) in &other.transitions {
            if !self.transitions.iter().any(|(aa, gg, ff)| aa == a && same_versor(gg, g) && ff == f) {
                self.transitions.push((a.clone(), g.clone(), *f));
            }
        }
        added
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

    /// THE CARRIAGE between two thoughts — the versor carrying this one's
    /// base geometry onto the other's, when they sit on one orbit. The chain
    /// is the boundary of a thought, never its identity: two germs are the
    /// same thought exactly when a versor relates their supports (equal
    /// tallies at different geometries never conflate; same-geometry
    /// siblings are sections of one word, not two cards). For pure-algebra
    /// (group) elements the realized matrix decides, where the realization
    /// is injective, and the carriage is the identity. A chain-carrying and
    /// a chain-free element are never carried onto each other.
    pub fn carriage(&self, other: &Element) -> Option<Versor> {
        match (self.sub.as_slice(), other.sub.as_slice()) {
            ([], []) => (self.realize() == other.realize()).then(identity_versor),
            ([], _) | (_, []) => None,
            (a, b) => transition(a, b),
        }
    }

    /// Group elements only: the same action iff the realized matrices agree.
    pub fn same_action(&self, other: &Element) -> bool {
        self.sub.is_empty() && other.sub.is_empty() && self.realize() == other.realize()
    }

    /// Same vertex of the Cayley graph: the M2 address, which is geodesic,
    /// ρ, shift, gauge, and the A5 chain. Two spellings of one vertex have
    /// one address; a different dilation, hue map, or placement is another
    /// vertex.
    pub fn same_vertex(&self, other: &Element) -> bool {
        self.address() == other.address()
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
        for (sigma, l, fibre) in &self.torsion {
            out.extend_from_slice(&(sigma.len() as u32).to_le_bytes());
            for (f, t) in sigma {
                out.push(*f);
                out.push(*t);
            }
            out.extend_from_slice(&l.to_le_bytes());
            out.extend_from_slice(&fibre.to_le_bytes());
        }
        // The connection: (assignment, g, fibre) relations.
        out.extend_from_slice(&(self.transitions.len() as u32).to_le_bytes());
        for (a, g, fibre) in &self.transitions {
            out.extend_from_slice(&(a.len() as u32).to_le_bytes());
            for (b, f) in a {
                out.push(*b);
                out.push(*f);
            }
            g.to_bytes(&mut out);
            out.extend_from_slice(&fibre.to_le_bytes());
        }
        out
    }

    /// The BASE canonical form: tape, dilation, placement, gauge, chain.
    /// The spatial fibre is derived and never hashed.
    fn base_canonical(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(self.word.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.word);
        out.extend_from_slice(&(self.sub.len() as u32).to_le_bytes());
        for (r, c, before, after) in &self.sub {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*before);
            out.push(*after);
        }
        if self.letters().is_none() {
            if let Some(m) = self.fibers.iter().find(|(i, _)| *i == FIBER_SPATIAL).map(|(_, m)| m) {
                if m.len() >= 2 && m[0].len() >= 2 {
                    let id = m[0][0].is_one() && m[1][1].is_one() && m[0][1].is_zero() && m[1][0].is_zero();
                    if !id {
                        for r in 0..2 {
                            for c in 0..2 {
                                let nb = m[r][c].numer().to_signed_bytes_le();
                                let db = m[r][c].denom().to_signed_bytes_le();
                                out.extend_from_slice(&(nb.len() as u32).to_le_bytes());
                                out.extend_from_slice(&nb);
                                out.extend_from_slice(&(db.len() as u32).to_le_bytes());
                                out.extend_from_slice(&db);
                            }
                        }
                    }
                }
            }
        }
        for (i, _) in &self.fibers {
            if *i >= FIBER_GERMINATED {
                out.extend_from_slice(&i.to_le_bytes());
            }
        }
        out
    }

    /// Vertex key of the word form: geodesic normal form plus frame
    /// coordinates as exact rationals. A sentence is keyed by its form
    /// (M1, ρ, agreed shift relative to the Frame, contact, gauge, faces).
    /// The body's absolute position is already quotiented on the chain.
    pub fn address(&self) -> [u8; 32] {
        *self.address.get_or_init(|| {
            if self.is_sentence() {
                if let Some(k) = crate::transducer::sentence::vertex_address(self) {
                    return k;
                }
            }
            *blake3::hash(&self.form_key_bytes()).as_bytes()
        })
    }

    /// Meaning of a non-sentence word: geodesic + (ρ, placement, gauge) +
    /// the centered remainder chain. The raw description bytes are not here.
    fn form_key_bytes(&self) -> Vec<u8> {
        use crate::transducer::alphabet::put_q;
        let mut out = Vec::new();
        if let Some(rw) = crate::transducer::rewrite::Rewrite::from_element(self) {
            out.push(REWRITE_TAG);
            out.push(rw.kind as u8);
            out.extend_from_slice(&(rw.before.len() as u16).to_le_bytes());
            for a in &rw.before {
                out.extend_from_slice(a);
            }
            out.extend_from_slice(&rw.after);
            for m in &rw.claimed {
                out.extend_from_slice(&m.to_le_bytes());
            }
            return out;
        }
        if let Some(letters) = self.letters() {
            out.extend_from_slice(&letters.spelling.word.key());
            put_q(&mut out, &letters.spelling.rho);
            put_q(&mut out, &letters.shift.0);
            put_q(&mut out, &letters.shift.1);
            let mut sigma = letters.sigma.clone();
            sigma.sort_unstable();
            out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
            for (f, t) in sigma {
                out.push(f);
                out.push(t);
            }
        } else if let Some(m) = self.fibers.iter().find(|(i, _)| *i == FIBER_SPATIAL).map(|(_, m)| m) {
            // Residual with a non-identity frame fibre: ρ is meaning, not a
            // listing coordinate. Translation is already quotiented in the chain.
            if m.len() >= 2 && m[0].len() >= 2 {
                let id = m[0][0].is_one() && m[1][1].is_one() && m[0][1].is_zero() && m[1][0].is_zero();
                if !id {
                    for r in 0..2 {
                        for c in 0..2 {
                            put_q(&mut out, &m[r][c]);
                        }
                    }
                }
            }
        }
        let chain = canonical_chain(&self.sub).unwrap_or_else(|| self.sub.clone());
        out.extend_from_slice(&(chain.len() as u32).to_le_bytes());
        for (r, c, b, a) in &chain {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*b);
            out.push(*a);
        }
        out
    }

    /// GEN2: tapes only. Magic `0xA2`, then dim, word, mag, chain, sections,
    /// torsion, anchor, transitions as letter tapes. The spatial fibre is
    /// never written.
    pub const GEN2_MAGIC: u8 = 0xA2;

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![Self::GEN2_MAGIC];
        out.extend_from_slice(&(self.dim as u32).to_le_bytes());
        out.extend_from_slice(&(self.word.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.word);
        for m in &self.mag {
            out.extend_from_slice(&m.to_le_bytes());
        }
        out.extend_from_slice(&(self.sub.len() as u32).to_le_bytes());
        for (r, c, before, after) in &self.sub {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*before);
            out.push(*after);
        }
        out.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        for s in &self.sections {
            out.extend_from_slice(&(s.len() as u32).to_le_bytes());
            for (b, a) in s {
                out.push(*b);
                out.push(*a);
            }
        }
        out.extend_from_slice(&(self.torsion.len() as u32).to_le_bytes());
        for (sigma, l, fibre) in &self.torsion {
            out.extend_from_slice(&(sigma.len() as u32).to_le_bytes());
            for (f, t) in sigma {
                out.push(*f);
                out.push(*t);
            }
            out.extend_from_slice(&l.to_le_bytes());
            out.extend_from_slice(&fibre.to_le_bytes());
        }
        match &self.anchor {
            Some(t) => {
                out.push(1);
                t.to_bytes(&mut out);
            }
            None => out.push(0),
        }
        out.extend_from_slice(&(self.transitions.len() as u32).to_le_bytes());
        for (a, g, fibre) in &self.transitions {
            out.extend_from_slice(&(a.len() as u32).to_le_bytes());
            for (b, f) in a {
                out.push(*b);
                out.push(*f);
            }
            let tape = tape_of_versor(g);
            let back = versor_of_tape(&tape);
            if back == *g {
                out.push(0);
                tape.to_bytes(&mut out);
            } else {
                out.push(1);
                g.to_bytes(&mut out);
            }
            out.extend_from_slice(&fibre.to_le_bytes());
        }
        out
    }

    /// Deserialize GEN2, else a GEN1 reader (fibres on the wire) for
    /// migration. Rewrites on first load: the next `to_bytes` is GEN2.
    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Element> {
        if b.get(*pos) == Some(&Self::GEN2_MAGIC) {
            return Self::from_bytes_gen2(b, pos);
        }
        let start = *pos;
        let mut p_new = start;
        let mut new = Self::from_bytes_layout(b, &mut p_new, true);
        if let Some(e) = new.as_mut() {
            e.fibers.retain(|(i, _)| *i != FIBER_SPATIAL);
            e.derive_spatial();
        }
        if new.is_some() && p_new == b.len() {
            *pos = p_new;
            return new;
        }
        let mut p_old = start;
        let mut old = Self::from_bytes_layout(b, &mut p_old, false);
        if let Some(e) = old.as_mut() {
            e.fibers.retain(|(i, _)| *i != FIBER_SPATIAL);
            e.derive_spatial();
        }
        if old.is_some() && (new.is_none() || p_old == b.len()) {
            *pos = p_old;
            return old;
        }
        if new.is_some() {
            *pos = p_new;
            return new;
        }
        None
    }

    fn from_bytes_gen2(b: &[u8], pos: &mut usize) -> Option<Element> {
        if b.get(*pos) != Some(&Self::GEN2_MAGIC) {
            return None;
        }
        *pos += 1;
        let u32_at = |b: &[u8], p: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v)
        };
        let dim = u32_at(b, pos)? as usize;
        if dim > 64 {
            return None;
        }
        let wlen = u32_at(b, pos)? as usize;
        let word = b.get(*pos..*pos + wlen)?.to_vec();
        *pos += wlen;
        let mut mag = [0u64; 4];
        for m in &mut mag {
            *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        let remaining = |b: &[u8], p: &usize| b.len().saturating_sub(*p);
        let nsub = u32_at(b, pos)? as usize;
        if nsub > remaining(b, pos) / 18 {
            return None;
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
            return None;
        }
        let mut sections = Vec::with_capacity(nsec);
        for _ in 0..nsec {
            let slen = u32_at(b, pos)? as usize;
            if slen > remaining(b, pos) / 2 {
                return None;
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
            return None;
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
            let fibre = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
            *pos += 2;
            torsion.push((sigma, l, fibre));
        }
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
        let ntr = u32_at(b, pos).unwrap_or(0) as usize;
        let mut transitions = Vec::with_capacity(ntr.min(64));
        for _ in 0..ntr {
            let alen = u32_at(b, pos)? as usize;
            if alen > remaining(b, pos) / 2 {
                return None;
            }
            let mut a = Vec::with_capacity(alen);
            for _ in 0..alen {
                let bb = *b.get(*pos)?;
                let f = *b.get(*pos + 1)?;
                *pos += 2;
                a.push((bb, f));
            }
            let tag = *b.get(*pos)?;
            *pos += 1;
            let g = if tag == 0 {
                let w = crate::transducer::alphabet::Word::from_bytes(b, pos)?;
                versor_of_tape(&w)
            } else {
                Versor::from_bytes(b, pos)?
            };
            let fibre = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
            *pos += 2;
            transitions.push((a, g, fibre));
        }
        let mut e = Element {
            dim,
            fibers: Vec::new(),
            word,
            mag,
            sub,
            sections,
            torsion,
            transitions,
            address: std::sync::OnceLock::new(),
            profiles: std::sync::OnceLock::new(),
            form: std::sync::OnceLock::new(),
            anchor,
        };
        e.derive_spatial();
        Some(e)
    }

    fn derive_spatial(&mut self) {
        if self.fibers.iter().any(|(i, _)| *i == FIBER_SPATIAL) {
            return;
        }
        let Some(letters) = self.letters() else {
            return;
        };
        let Some(a) = letters.affine() else {
            return;
        };
        let dim = self.dim.max(GERM_DIM);
        let mut m = identity(dim);
        place_block(&mut m, 0, &a);
        for &(f, t) in &letters.sigma {
            let i = 3 + f as usize;
            let j = 3 + t as usize;
            if i < dim && j < dim && f != t {
                let mut c = 3;
                while c < dim {
                    m[i][c] = Q::zero();
                    c += 1;
                }
                m[i][j] = Q::one();
            }
        }
        self.fibers.push((FIBER_SPATIAL, m));
        self.fibers.sort_by_key(|(i, _)| *i);
    }

    fn from_bytes_layout(b: &[u8], pos: &mut usize, with_fibre: bool) -> Option<Element> {
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
                    if denom == BigInt::from(0) {
                        return None;
                    }
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
            let fibre = if with_fibre {
                let f = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
                *pos += 2;
                f
            } else {
                FIBER_SPATIAL
            };
            torsion.push((sigma, l, fibre));
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
        // The magnitude is a MEASURED projection of the chain, not stored
        // identity (the address covers word/dim/fibers only): re-measure on
        // load so persisted pre-projection words upgrade transparently and
        // the blind replay compares measurement against measurement.
        let mag = if sub.is_empty() { mag } else { measure_lanes(&sub) };
        // The connection tail (absent in pre-bundle serializations → none).
        let mut transitions = Vec::new();
        if *pos < b.len() {
            let nt = u32_at(b, pos)? as usize;
            if nt > remaining(b, pos) / 21 {
                return None; // untrusted count: length prefix + g per relation
            }
            for _ in 0..nt {
                let alen = u32_at(b, pos)? as usize;
                if alen > remaining(b, pos) / 2 {
                    return None;
                }
                let mut a = Vec::with_capacity(alen);
                for _ in 0..alen {
                    let bb = *b.get(*pos)?;
                    let ff = *b.get(*pos + 1)?;
                    *pos += 2;
                    a.push((bb, ff));
                }
                // The connection's transport: a versor, or the legacy lattice
                // form (a D4 index and an integer offset) lifted to its
                // versor — judicial continuity across the format change.
                let save = *pos;
                let g = match Versor::from_bytes(b, pos) {
                    Some(v) => v,
                    None => {
                        *pos = save;
                        let d4 = *b.get(*pos)?;
                        let tr = i64::from_le_bytes(b.get(*pos + 1..*pos + 9)?.try_into().ok()?);
                        let tc = i64::from_le_bytes(b.get(*pos + 9..*pos + 17)?.try_into().ok()?);
                        *pos += 17;
                        legacy_versor(d4, tr, tc)?
                    }
                };
                let fibre = if with_fibre {
                    let f = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
                    *pos += 2;
                    f
                } else {
                    FIBER_SPATIAL
                };
                transitions.push((a, g, fibre));
            }
        }
        Some(Element {
            dim,
            fibers,
            word,
            mag,
            sub,
            sections,
            torsion,
            anchor,
            transitions,
            address: std::sync::OnceLock::new(),
            profiles: std::sync::OnceLock::new(),
            form: std::sync::OnceLock::new(),
        })
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
/// A5: translate min-corner to origin, sort, dedup. None when two
/// distinct transitions occupy one site.
pub fn canonical_chain(cells: &[(i64, i64, u8, u8)]) -> Option<Vec<(i64, i64, u8, u8)>> {
    if cells.is_empty() {
        return Some(Vec::new());
    }
    let min_r = cells.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
    let min_c = cells.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
    let mut out: Vec<(i64, i64, u8, u8)> = cells
        .iter()
        .map(|(r, c, b, a)| (r - min_r, c - min_c, *b, *a))
        .collect();
    out.sort_unstable();
    out.dedup();
    for w in out.windows(2) {
        if w[0].0 == w[1].0 && w[0].1 == w[1].1 && (w[0].2, w[0].3) != (w[1].2, w[1].3) {
            return None;
        }
    }
    Some(out)
}

pub fn from_residual(g: &ResidualGerm) -> Element {
    let centered: ResidualGerm = canonical_chain(g).unwrap_or_else(|| {
        let min_r = g.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
        let min_c = g.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
        let mut c: ResidualGerm = g.iter().map(|(r, col, b, a)| (r - min_r, col - min_c, *b, *a)).collect();
        c.sort_unstable();
        c
    });

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

    let mut e = Element::identity(GERM_DIM).with_fiber(FIBER_SPATIAL, m);
    e.word = residual_word(&centered);
    e.address.take();
    e.form.take();
    e.mag = measure_lanes(&centered);
    e.sub = centered;
    e
}

/// A residual germ with its FRAME FIBER: the exact frame map
/// ρ = (h_out/h_in, w_out/w_in) ∈ ℚ² enters the spatial generator's
/// diagonal, so a word at ρ ≠ (1,1) is a different word from its planar
/// shadow (identity and substance both carry the fiber; only translation is
/// quotiented). `frame` is (h_in, w_in, h_out, w_out).
/// The unique listing key of a demand: the address of the framed residual.
/// Intake, the book, quote, attest, probe, settle, task, and the nerve
/// route all use this, never an unframed shadow.
pub fn listing_key(chain: &[(i64, i64, u8, u8)], frame: &(usize, usize, usize, usize)) -> [u8; 32] {
    from_residual_framed(&chain.to_vec(), frame).address()
}

/// Identity frame: no scale. Unframed speech uses this so its key matches
/// a planar intake.
pub const IDENTITY_FRAME: (usize, usize, usize, usize) = (1, 1, 1, 1);

pub fn from_residual_framed(g: &ResidualGerm, frame: &(usize, usize, usize, usize)) -> Element {
    let mut e = from_residual(g);
    let (hi, wi, ho, wo) = *frame;
    if hi > 0 && wi > 0 && (hi, wi) != (ho, wo) {
        if let Some(m) = e.fiber(FIBER_SPATIAL) {
            let mut m = m.clone();
            m[0][0] = Q::new(BigInt::from(ho as u64), BigInt::from(hi as u64));
            m[1][1] = Q::new(BigInt::from(wo as u64), BigInt::from(wi as u64));
            e.set_fiber(FIBER_SPATIAL, m);
        }
    }
    e
}

/// THE AFFINE BLOCK of a versor: the exact 3×3 homogeneous matrix of its
/// action on the plane, read by carrying three points (the origin and the
/// two unit steps) and checked on a fourth. None when the versor is not
/// affine (a conformal action that bends lines has no such block) or
/// reaches infinity.
pub fn affine_of(v: &Versor) -> Option<QMatrix> {
    let carry = |r: i64, c: i64| -> Option<(Q, Q)> {
        let x = Q::from(BigInt::from(r));
        let y = Q::from(BigInt::from(c));
        crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&x, &y)))
    };
    let (t, er, ec, d) = (carry(0, 0)?, carry(1, 0)?, carry(0, 1)?, carry(1, 1)?);
    let (a, c_) = (&er.0 - &t.0, &er.1 - &t.1);
    let (b, d_) = (&ec.0 - &t.0, &ec.1 - &t.1);
    // Affine iff the fourth point is the sum of the two steps.
    if d.0 != &t.0 + &a + &b || d.1 != &t.1 + &c_ + &d_ {
        return None;
    }
    Some(vec![
        vec![a, b, t.0.clone()],
        vec![c_, d_, t.1.clone()],
        vec![Q::zero(), Q::zero(), Q::one()],
    ])
}

/// MINT THE CONNECTION: one element with two components. The group
/// component is the comprehended transport — the versor's affine block on
/// the motion rows of the spatial fibre and the gauge map σ as a
/// permutation on the hue rows (when σ, extended by the identity on the
/// hues it does not touch, is a bijection; otherwise the hue rows stay the
/// identity and the gauge is carried by the chain). The additive component
/// is the remainder — what the connection could not speak — as the chain;
/// an empty remainder is a closed word. `carried` is the volume the
/// connection speaks across its family, added to the chain's own measure
/// per lane, so a closed connection has magnitude on the lanes it moves.
pub fn from_connection(affine: &QMatrix, sigma: &[(u8, u8)], remainder: &ResidualGerm, carried: [u64; 4]) -> Option<Element> {
    if affine.len() != 3 || affine.iter().any(|r| r.len() != 3) {
        return None;
    }
    if !(affine[2][0].is_zero() && affine[2][1].is_zero() && affine[2][2].is_one()) {
        return None;
    }
    // THE ALPHABET SPEAKS THE MOTION: the linear block is spelled into the
    // seventeen letters (a peel, never a search); a block the alphabet
    // cannot spell is not a word of the language — the caller holds it as
    // thought. The tape is the word's ANASTOME description, so the letters
    // enter the address and the court re-derives the block from them.
    let block = [[affine[0][0].clone(), affine[0][1].clone()], [affine[1][0].clone(), affine[1][1].clone()]];
    let spelling = crate::transducer::alphabet::spell(&block)?;
    let shift = (affine[0][2].clone(), affine[1][2].clone());
    let mut e = from_residual(remainder);
    e.word = connection_word(&spelling, &shift, sigma, &e.sub);
    let mut m = identity(GERM_DIM);
    place_block(&mut m, 0, affine);
    let mut image: Vec<usize> = (0..PALETTE).collect();
    for &(from, to) in sigma {
        if (from as usize) < PALETTE && (to as usize) < PALETTE {
            image[from as usize] = to as usize;
        }
    }
    let mut seen = vec![false; PALETTE];
    let bijective = image.iter().all(|&t| {
        let fresh = !seen[t];
        seen[t] = true;
        fresh
    });
    if bijective {
        let mut perm = vec![vec![Q::zero(); PALETTE]; PALETTE];
        for (from, &to) in image.iter().enumerate() {
            perm[from][to] = Q::one();
        }
        place_block(&mut m, 3, &perm);
    }
    e.set_fiber(FIBER_SPATIAL, m);
    e.address.take();
    for l in 0..4 {
        e.mag[l] = e.mag[l].saturating_add(carried[l]);
    }
    Some(e)
}

/// The leading byte of a connection's description: what follows is a
/// letter tape, not a residual germ.
fn tape_of_versor(g: &Versor) -> crate::transducer::alphabet::Word {
    let Some(a) = affine_of(g) else {
        return crate::transducer::alphabet::Word::identity();
    };
    if a.len() < 2 || a[0].len() < 2 {
        return crate::transducer::alphabet::Word::identity();
    }
    let block = [[a[0][0].clone(), a[0][1].clone()], [a[1][0].clone(), a[1][1].clone()]];
    crate::transducer::alphabet::spell(&block)
        .map(|s| s.word)
        .unwrap_or_else(crate::transducer::alphabet::Word::identity)
}

fn versor_of_tape(w: &crate::transducer::alphabet::Word) -> Versor {
    let one = Q::from(BigInt::from(1));
    conformal_of(
        &crate::transducer::alphabet::Spelling { word: w.clone(), rho: one },
        &(Q::from(BigInt::from(0)), Q::from(BigInt::from(0))),
    )
    .unwrap_or_else(identity_versor)
}

pub const CONNECTION_TAG: u8 = 0x11;
/// A sentence: a nerve of germs. The tape after this tag is the base
/// connection, then the germ addresses, faces and cycles. Distinct from
/// CONNECTION_TAG so an opened-axis connection is not replayed as a sentence.
pub const SENTENCE_TAG: u8 = 0x12;
/// A translation: the gauge between two sections, not a motion of a body.
/// The court replays it by inversion of the gauge; the syllabus teaches it
/// by incidence. Same payload as a connection, then the two section addresses.
pub const TRANSLATION_TAG: u8 = 0x13;
/// An optimisation: a Rewrite the court replays both ways.
pub const REWRITE_TAG: u8 = 0x14;

/// The description of a connection: its spelling in the alphabet at its
/// dilation, the placement, the gauge map, and the centered remainder.
pub(crate) fn connection_word(
    spelling: &crate::transducer::alphabet::Spelling,
    shift: &(Q, Q),
    sigma: &[(u8, u8)],
    remainder: &ResidualGerm,
) -> Vec<u8> {
    use crate::transducer::alphabet::put_q;
    let mut out = vec![CONNECTION_TAG];
    spelling.to_bytes(&mut out);
    put_q(&mut out, &shift.0);
    put_q(&mut out, &shift.1);
    out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
    for &(f, t) in sigma {
        out.push(f);
        out.push(t);
    }
    let germ = residual_word(remainder);
    out.extend_from_slice(&(germ.len() as u32).to_le_bytes());
    out.extend_from_slice(&germ);
    out
}

/// THE LETTERS OF A CONNECTION as read off its description: the word and
/// its dilation, the placement, and the gauge map it was spoken with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Letters {
    pub spelling: crate::transducer::alphabet::Spelling,
    pub shift: (Q, Q),
    pub sigma: Vec<(u8, u8)>,
}

impl Letters {
    /// The homogeneous affine block the letters speak.
    pub fn affine(&self) -> Option<QMatrix> {
        let b = self.spelling.block()?;
        Some(vec![
            vec![b[0][0].clone(), b[0][1].clone(), self.shift.0.clone()],
            vec![b[1][0].clone(), b[1][1].clone(), self.shift.1.clone()],
            vec![Q::zero(), Q::zero(), Q::one()],
        ])
    }
}

impl Element {
    /// The letters of this word — `None` for a residual germ or a word
    /// spoken before the alphabet (no tape: not a word of the language).
    pub fn letters(&self) -> Option<Letters> {
        use crate::transducer::alphabet::{take_q, Spelling};
        let b = &self.word;
        let tag = b.first().copied()?;
        if tag != CONNECTION_TAG && tag != SENTENCE_TAG && tag != TRANSLATION_TAG {
            return None;
        }
        let mut pos = 1usize;
        let spelling = Spelling::from_bytes(b, &mut pos)?;
        let shift = (take_q(b, &mut pos)?, take_q(b, &mut pos)?);
        let n = u16::from_le_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
        pos += 2;
        let mut sigma = Vec::with_capacity(n);
        for _ in 0..n {
            sigma.push((*b.get(pos)?, *b.get(pos + 1)?));
            pos += 2;
        }
        let len = u32::from_le_bytes(b.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        if pos + len > b.len() {
            return None;
        }
        Some(Letters { spelling, shift, sigma })
    }

    /// The affine block the spatial fibre holds on its motion rows.
    pub fn held_affine(&self) -> Option<QMatrix> {
        let m = self.fiber(FIBER_SPATIAL)?;
        if m.len() < 3 {
            return None;
        }
        Some((0..3).map(|r| (0..3).map(|c| m[r][c].clone()).collect()).collect())
    }

    /// The affine block derived from the tape, never a stored fibre.
    pub fn affine(&self) -> Option<QMatrix> {
        self.letters()?.affine()
    }

    /// Tape re-derives vertex: the geodesic of the stored word is the same
    /// transport. `None` when there are no letters.
    pub fn letters_rederive(&self) -> Option<bool> {
        let letters = self.letters()?;
        Some(letters.spelling.word.matrix()? == letters.spelling.word.geodesic().matrix()?)
    }

    /// The axes opened above the four strata: the germinated fibres.
    pub fn opened_axes(&self) -> Vec<u16> {
        self.fibers.iter().map(|(i, _)| *i).filter(|&i| i >= FIBER_GERMINATED).collect()
    }

    /// Is this word a CONNECTION — a group component the court can invert
    /// and glue? Read on the spatial fibre: the motion rows are a
    /// homogeneous affine block and the hue rows are a permutation (the
    /// identity included). A residual germ's antisymmetric hue tally is
    /// never a permutation, so the two kinds of word never conflate.
    pub fn is_connection(&self) -> bool {
        let Some(m) = self.fiber(FIBER_SPATIAL) else { return false };
        if m.len() != GERM_DIM {
            return false;
        }
        let homogeneous = m[2][0].is_zero() && m[2][1].is_zero() && m[2][2].is_one();
        let permutation = (3..GERM_DIM).all(|r| {
            let ones = (3..GERM_DIM).filter(|&c| m[r][c].is_one()).count();
            let zeros = (3..GERM_DIM).filter(|&c| m[r][c].is_zero()).count();
            ones == 1 && zeros == GERM_DIM - 4
        }) && (3..GERM_DIM).all(|c| (3..GERM_DIM).filter(|&r| m[r][c].is_one()).count() == 1);
        let no_mixing = (0..3).all(|r| (3..GERM_DIM).all(|c| m[r][c].is_zero())) && (3..GERM_DIM).all(|r| (0..3).all(|c| m[r][c].is_zero()));
        homogeneous && permutation && no_mixing
    }

    /// A sentence: its own tape tag, never merely an opened-axis connection.
    pub fn is_sentence(&self) -> bool {
        self.word.first().copied() == Some(SENTENCE_TAG)
    }

    /// A translation: the gauge between two sections.
    pub fn is_translation(&self) -> bool {
        self.word.first().copied() == Some(TRANSLATION_TAG)
    }

    /// An optimisation spoken as a word.
    pub fn is_rewrite(&self) -> bool {
        self.word.first().copied() == Some(REWRITE_TAG)
    }

    /// Mint a Rewrite: the address measures the tape.
    pub fn from_rewrite_tape(word: Vec<u8>, mag: [u64; 4]) -> Element {
        let mut e = Element::identity(GERM_DIM);
        e.word = word;
        e.mag = mag;
        e.address.take();
        e.form.take();
        e
    }

    /// The two sections a translation gauges, if the tape carries them.
    pub fn translation_sections(&self) -> Option<([u8; 32], [u8; 32])> {
        if !self.is_translation() {
            return None;
        }
        let b = self.description();
        if b.len() < 64 {
            return None;
        }
        let a: [u8; 32] = b[b.len() - 64..b.len() - 32].try_into().ok()?;
        let c: [u8; 32] = b[b.len() - 32..].try_into().ok()?;
        Some((a, c))
    }
}

/// A gauge between two sections, spelled as a connection with identity
/// gauge map and tagged so the court replays it as a translation.
pub fn from_translation(
    rotor: &crate::transducer::alphabet::Rotor,
    a: [u8; 32],
    b: [u8; 32],
) -> Option<Element> {
    let one = Q::one();
    let zero = Q::zero();
    let e0 = rotor.carry(&[one.clone(), zero.clone(), zero.clone(), zero.clone()])?;
    let e1 = rotor.carry(&[zero.clone(), one.clone(), zero.clone(), zero.clone()])?;
    let affine = vec![
        vec![e0[0].clone(), e1[0].clone(), zero.clone()],
        vec![e0[1].clone(), e1[1].clone(), zero.clone()],
        vec![zero.clone(), zero.clone(), one],
    ];
    let mut e = from_connection(&affine, &[], &Vec::new(), [1, 0, 0, 0])?;
    let mut d = e.description().to_vec();
    if d.first() == Some(&CONNECTION_TAG) {
        d[0] = TRANSLATION_TAG;
    }
    d.extend_from_slice(&a);
    d.extend_from_slice(&b);
    e.set_description(d);
    Some(e)
}

/// The perceptual lane projection of a centered coboundary chain — the
/// per-lane magnitude MEASURED from the chain's geometry, never funneled
/// onto one axis:
///   𝒮 spatial:  occupancy transformation — cells where support appears or
///               vanishes (mass moved, scaled, or reshaped).
///   𝒢 gauge:    chromatic content — hue transitions on cells where the
///               support stands on both sides (recolour, palette orbit).
///   𝒯 temporal: standing extent — TUniversal ticks a listing has stood,
///               or the family's scene-state span at mint. The chain has
///               none; 0 here. `Element::magnitude` and the kernel overlay
///               the live T coordinate.
///   𝒽 holonomic: boundary topology — the connected-component and
///               boundary-cochain deltas between the before-support and the
///               after-support (merges, splits, enclosures). Sections and
///               torsion cycles the word holds are added on this lane by
///               `Element::magnitude`.
/// A recolour word is gauge-lane work; a move is spatial; a hole closing is
/// shell. Without these projections the four-lane manifold degenerates to a
/// scalar on 𝒯 and every gradient downstream flatlines.
pub fn measure_lanes(chain: &[(i64, i64, u8, u8)]) -> [u64; 4] {
    let mut spatial = 0u64;
    let mut gauge = 0u64;
    let mut before_pos: Vec<(i64, i64)> = Vec::new();
    let mut after_pos: Vec<(i64, i64)> = Vec::new();
    for &(r, c, b, a) in chain {
        if b != 0 {
            before_pos.push((r, c));
        }
        if a != 0 {
            after_pos.push((r, c));
        }
        match (b != 0, a != 0) {
            (true, true) if b != a => gauge += 1,
            (true, false) | (false, true) => spatial += 1,
            _ => {}
        }
    }
    let sb = crate::transducer::spectral::signature(&before_pos, &[]);
    let sa = crate::transducer::spectral::signature(&after_pos, &[]);
    let shell = sb.components.abs_diff(sa.components) + sb.boundary.abs_diff(sa.boundary);
    // Lane 2 is temporal extent — TUniversal ticks or scene-state span —
    // and is measured on the standing carrier or at mint, never on the chain.
    [spatial, gauge, 0, shell]
}

impl Element {
    /// The four lanes this germ carries content on — 𝒮 spatial, 𝒢 gauge, 𝒯
    /// temporal, 𝒽 holonomic — exactly where the coordinate-exact magnitude
    /// is nonzero (never the lossy tally).
    pub fn lanes(&self) -> [bool; 4] {
        let m = self.magnitude();
        std::array::from_fn(|l| m[l] > 0)
    }

    /// The germ's polytonic MAGNITUDE — per-lane support size (‖Δ‖'s |supp|
    /// component), never summed, computed EXACTLY from the description at
    /// germination. A settled demand's residual magnitude is 0⃗; a real germ's is
    /// nonzero — the descent the POW++ gate reads.
    /// Spatial, gauge, and shell from the chain (and what a connection
    /// carries). Lane 2 is temporal extent at mint (scene-state span).
    /// Lane 3 gains the holonomic volume — sections and torsion cycles.
    pub fn magnitude(&self) -> [u64; 4] {
        let mut m = self.mag;
        m[3] = m[3].saturating_add((self.sections.len() + self.torsion.len()) as u64);
        m
    }

    /// The lanes the chain itself carries, before the sections and torsion
    /// cycles the word gained after it was spoken are counted. This is what
    /// re-derives from the chain; a word deepened by `attach` or
    /// `merge_fibre` keeps this fixed while `magnitude` grows on lane 3.
    pub fn base_magnitude(&self) -> [u64; 4] {
        self.mag
    }

    /// The base magnitude on the lanes a chain names: 𝒮 spatial, 𝒢 gauge,
    /// 𝒽 shell. Lane 𝒯 is temporal extent, measured on the standing
    /// carrier and never on the chain (`measure_lanes` writes it zero), so
    /// it is quotiented here by the same law. This is the vector the court
    /// compares with `from_residual(chain).base_magnitude()`, as one
    /// equality, never lane by lane.
    pub fn chain_magnitude(&self) -> [u64; 4] {
        let [spatial, gauge, _, shell] = self.mag;
        [spatial, gauge, 0, shell]
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
        out.address.take();
        out.form.take();
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
        out.mag = if out.sub.is_empty() { out.reduction_magnitude() } else { measure_lanes(&out.sub) };
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

    /// The legacy lattice transport decodes to the versor with the same
    /// action: index 1 is (r, c) → (c, −r), then the offset.
    #[test]
    fn the_legacy_transport_lifts_to_its_versor() {
        let v = legacy_versor(1, 2, 3).expect("a lattice isometry is a rational versor");
        let q = |n: i64| num_rational::BigRational::from_integer(BigInt::from(n));
        let at = |r: i64, c: i64| crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&q(r), &q(c))));
        assert_eq!(at(0, 0), Some((q(2), q(3))));
        assert_eq!(at(1, 0), Some((q(2), q(2))), "(1,0) → (0,−1) + (2,3)");
        assert_eq!(at(0, 1), Some((q(3), q(3))), "(0,1) → (1,0) + (2,3)");
        assert!(legacy_versor(9, 0, 0).is_none());
    }

    /// Congruence on the conformal orbit: translations, rotations,
    /// reflections, and exact dilations of a support are carried by a
    /// rational versor found in closed form; a genuinely different shape
    /// refuses; the pullback reads a sibling's assignment onto the base at
    /// any scale exactly.
    #[test]
    fn the_versor_identifies_conformal_images_and_pulls_back_sections() {
        let l = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2), (1, 0, 1, 2)];
        let shifted: Vec<_> = l.iter().map(|(r, c, b, a)| (r + 7, c + 3, *b, *a)).collect();
        let rotated = vec![(0i64, 0i64, 1u8, 2u8), (1, 0, 1, 2), (0, -1, 1, 2)];
        let bar = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2), (0, 2, 1, 2)];
        assert!(transition(&l, &shifted).is_some(), "translation is a versor");
        assert!(transition(&l, &rotated).is_some(), "rotation is a versor");
        assert!(transition(&l, &bar).is_none(), "distinct shapes refuse");
        // A 2× dilated L with other hues: the versor exists and the pullback
        // reads its assignment onto the base's three cells.
        let big: Vec<(i64, i64, u8, u8)> = l
            .iter()
            .flat_map(|&(r, c, _, _)| {
                [(2 * r, 2 * c), (2 * r, 2 * c + 1), (2 * r + 1, 2 * c), (2 * r + 1, 2 * c + 1)]
                    .into_iter()
                    .map(|(rr, cc)| (rr, cc, 5u8, 6u8))
            })
            .collect();
        let v = transition(&big, &l).expect("a dilation by 1/2 carries the big L onto the L");
        assert_eq!(pullback(&positions(&l), &big, &v), Some(vec![(5, 6); 3]));
        assert_ne!(positions(&l), positions(&shifted), "the raw lens sees only coordinates");
    }

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
        // A pre-anchor serialization (anchor byte and connection count
        // stripped) parses to ⊤ with an empty connection.
        let plain_bytes = plain.to_bytes();
        let mut pos2 = 0;
        let old = Element::from_bytes(&plain_bytes[..plain_bytes.len() - 5], &mut pos2).expect("old form");
        assert!(old.anchor().is_none(), "absent tail = the unconstrained carrier");
        assert!(old.transitions().is_empty(), "absent connection = none recorded");
    }

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
        // A hue transition on a standing cell measures as GAUGE content
        // alongside the residual defect (the four-lane projection).
        assert_eq!(germ.lanes(), [false, true, false, false], "a recolour is 𝒢 content; temporal and holonomic are empty until standing or punctured");
        let mag = germ.magnitude();
        assert!(mag[2] == 0 && mag[1] >= 1 && mag[0] == 0, "gauge only: no spatial, no temporal extent yet");
        assert_eq!(mag[3], 0, "holonomic volume is empty until sections or torsion");
        // A residual has no group inverse (its block is antisymmetric/singular);
        // its solution is the reverse coboundary (from_residual of b→a), not a
        // matrix inverse — the group solution() correctly declines.
        assert!(germ.solution().is_none(), "no single-matrix inverse for a coboundary");
        let reverse = from_residual(&germinate(&b, &a).unwrap());
        assert_ne!(germ.address(), reverse.address(), "the reverse description is a distinct germ");
    }

    /// The transposition quotient is the CENTERED chain: the same deformation at
    /// any position is ONE substance (min-corner framed to origin), while a
    /// deformation with the SAME hue tally but DIFFERENT geometry stays DISTINCT —
    /// the Step-1 conflation, cured (the tally never decides substance).
    /// A connection minted from a transport is a word with two components:
    /// the identity motion with a hue swap is a permutation on the hue rows
    /// (invertible, a connection), the remainder is its chain, and its
    /// address differs from the bare residual's. A non-bijective gauge
    /// leaves the hue rows the identity.
    #[test]
    fn a_connection_is_one_element_with_two_components() {
        let affine = affine_of(&identity_versor()).expect("the identity versor is affine");
        assert_eq!(affine, crate::core::matrix::identity(3));
        let remainder = vec![(0, 0, 3u8, 4u8)];
        let w = from_connection(&affine, &[(1, 2), (2, 1)], &remainder, [5, 2, 0, 0]).expect("the identity spells");
        assert!(w.is_connection(), "affine motion rows and a permutation on the hue rows");
        assert_eq!(w.letters_rederive(), Some(true), "the tape re-derives the held motion");
        assert_eq!(w.letters().map(|l| l.sigma), Some(vec![(1, 2), (2, 1)]), "the gauge rides the tape");
        let mut pos = 0;
        let back = Element::from_bytes(&w.to_bytes(), &mut pos).expect("round trip");
        assert_eq!(back.letters(), w.letters(), "the letters survive the wire");
        // A shear is no similarity: the alphabet cannot spell it, so it is
        // not a word — held as thought, never minted.
        let shear = vec![vec![q(1), q(1), q(0)], vec![q(0), q(1), q(0)], vec![q(0), q(0), q(1)]];
        assert!(from_connection(&shear, &[], &Vec::new(), [1, 0, 0, 0]).is_none(), "an unspellable motion is not a word");
        assert!(w.solution().is_some(), "the group component inverts");
        assert_eq!(w.sub(), remainder.as_slice(), "the remainder is the chain");
        assert_ne!(w.address(), from_residual(&remainder).address(), "the connection is not the bare residual");
        let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
        let long = crate::transducer::alphabet::Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
        assert_eq!(long.key(), crate::transducer::alphabet::Word::identity().key());
        let mut alt = w.clone();
        if let Some(letters) = alt.letters() {
            let spelling = crate::transducer::alphabet::Spelling { word: long, rho: letters.spelling.rho };
            let rem = alt.sub().to_vec();
            alt.word = connection_word(&spelling, &letters.shift, &letters.sigma, &rem);
            alt.address.take();
            assert_eq!(alt.address(), w.address(), "q^4 and 1 are one vertex; the tape is not the address");
            assert!(alt.same_vertex(&w), "q^4 and 1 are one vertex; equality is the rotor, not a digest");
        }
        assert!(w.magnitude()[0] >= 5 && w.magnitude()[1] >= 2, "carried volume rides on the lanes");
        let closed = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [5, 2, 0, 0]).expect("spells");
        assert!(closed.is_connection() && closed.sub().is_empty(), "an empty remainder is a closed connection");
        let merge = from_connection(&affine, &[(1, 2)], &Vec::new(), [1, 1, 0, 0]).expect("spells");
        assert!(!merge.is_connection(), "a merging gauge with the identity motion carries no group component");
        assert!(!from_residual(&remainder).is_connection(), "a residual's tally is never a permutation");
        // Identity motion and identity gauge drop the fibre: the unit, not a
        // connection, not speech — the 5839 empty-chain refusal.
        let unit = from_connection(&affine, &[], &Vec::new(), [77, 0, 0, 0]).expect("spells");
        assert!(unit.is_identity(), "identity motion and gauge leave no fibre and no chain");
        assert!(!unit.is_connection(), "the unit is not a connection");
        assert!(unit.sub().is_empty(), "nothing residual to hold as thought");
    }

    #[test]
    fn gen2_round_trips_and_gen1_migrates() {
        let affine = affine_of(&identity_versor()).expect("affine");
        let w = from_connection(&affine, &[(1, 2), (2, 1)], &vec![(0, 0, 3u8, 4)], [5, 2, 0, 0]).expect("spells");
        let bytes = w.to_bytes();
        assert_eq!(bytes.first().copied(), Some(Element::GEN2_MAGIC));
        let mut pos = 0;
        let back = Element::from_bytes(&bytes, &mut pos).expect("GEN2");
        assert_eq!(back.address(), w.address());
        assert_eq!(back.letters().map(|l| l.sigma), w.letters().map(|l| l.sigma));
        assert!(back.letters_rederive() == Some(true));
    }

    #[test]
    fn substance_is_the_centered_chain() {
        let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
        let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)]; // (0,1): 2→9
        // The same deformation transposed +7 rows, +5 cols.
        let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 7, c + 5, *h)).collect();
        let b_far: Vec<_> = b.iter().map(|(r, c, h)| (r + 7, c + 5, *h)).collect();
        let here = from_residual(&germinate(&a, &b).unwrap());
        let far = from_residual(&germinate(&a_far, &b_far).unwrap());
        assert_eq!(here.address(), far.address(), "same deformation, any position — ONE word");
        assert!(here.carriage(&far).is_some(), "one thought: a versor carries one onto the other");
        // Equal tally, different geometry — the cured conflation: {1→0 here, 0→4
        // there} at two different relative offsets must stay DISTINCT.
        let g1 = from_residual(&vec![(0, 0, 1, 0), (1, 1, 0, 4)]);
        let g2 = from_residual(&vec![(1, 0, 1, 0), (0, 2, 0, 4)]);
        assert!(g1.carriage(&g2).is_none(), "equal tally, different geometry — no versor relates them");
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
            assert_eq!(back.address(), g.address(), "the reloaded word is the same citation");
            assert_eq!(back.sub(), g.sub(), "the centered chain survives");
            assert!(back.carriage(g).is_some(), "the thought survives: the identity carriage relates them");
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
        assert_eq!(back.address(), t.address(), "the reloaded word is the same citation");
        assert!(back.matches_chain(&vec![(0, 0, 3, 1), (0, 1, 1, 2), (0, 2, 2, 3)]), "capacity survives restart");
        assert!(t.transitions().is_empty());
    }

    /// THE CANCELING TALLY IS NOT IDENTITY: L→bar's bidirectional hue swap cancels
    /// the tally to zero — but the CHAIN keeps it a real, non-identity germ with
    /// exact magnitude, and its reverse is a distinct word.
    #[test]
    fn a_bidirectional_swap_does_not_collapse_to_zero() {
        let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)]; // L
        let b = [(0, 0, 1u8), (0, 1, 1), (0, 2, 1)]; // bar
        let l_to_bar = from_residual(&germinate(&a, &b).unwrap());
        // One cell vacates and one appears: SPATIAL content (mass moved)
        // alongside the two-cell residual defect — measured, not defaulted.
        assert_eq!(l_to_bar.magnitude(), [2, 0, 0, 0], "two moved cells, exact — not a canceled 0; the fibre lane is empty until punctured");
        assert_eq!(l_to_bar.lanes(), [true, false, false, false], "a real spatial germ");
        assert!(!l_to_bar.is_identity(), "the chain decides — a canceled tally is not identity");
        assert!(!l_to_bar.substance().is_identity(), "and its substance is real");
        let bar_to_l = from_residual(&germinate(&b, &a).unwrap());
        assert_ne!(l_to_bar.address(), bar_to_l.address(), "distinct descriptions, distinct address");
        // The two descriptions share one base geometry: a versor carries one
        // support onto the other, and the reversed gauge is a second section
        // of the same word, never a second card.
        assert!(l_to_bar.carriage(&bar_to_l).is_some(), "one base geometry, two sections");
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
        assert!(!both.same_action(&spatial), "a germinated axis is a new generator, not a new digest");
    }

    #[test]
    fn a5_chain_canonical_idempotent_and_translation_free() {
        let a = vec![(3, 4, 1u8, 2u8), (3, 5, 1, 2)];
        let b = vec![(0, 1, 1u8, 2u8), (0, 0, 1, 2)];
        let ca = canonical_chain(&a).unwrap();
        let cb = canonical_chain(&b).unwrap();
        assert_eq!(ca, cb);
        assert_eq!(canonical_chain(&ca).unwrap(), ca);
        assert_eq!(from_residual(&a).address(), from_residual(&b).address());
    }

    #[test]
    fn same_vertex_is_address_rho_sigma_and_shift() {
        let id = affine_of(&identity_versor()).expect("affine");
        let swap = from_connection(&id, &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("swap");
        let cycle = from_connection(&id, &[(1, 2), (2, 3), (3, 1)], &Vec::new(), [1, 0, 0, 0]).expect("cycle");
        let qn = |n: i64| Q::from(BigInt::from(n));
        let shifted = vec![
            vec![qn(1), qn(0), qn(5)],
            vec![qn(0), qn(1), qn(0)],
            vec![qn(0), qn(0), qn(1)],
        ];
        let place = from_connection(&shifted, &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("shift");
        let mut dilated = swap.clone();
        if let Some(letters) = dilated.letters() {
            let two = Q::from(BigInt::from(2));
            let spelling = crate::transducer::alphabet::Spelling {
                word: letters.spelling.word,
                rho: two,
            };
            let rem = dilated.sub().to_vec();
            dilated.word = connection_word(&spelling, &letters.shift, &letters.sigma, &rem);
            dilated.address.take();
        }
        assert!(!swap.same_vertex(&dilated), "ρ is identity");
        assert!(!swap.same_vertex(&cycle), "σ is identity");
        assert!(!swap.same_vertex(&place), "shift is identity");
        assert_ne!(swap.address(), dilated.address());
        assert_ne!(swap.address(), cycle.address());
        assert_ne!(swap.address(), place.address());
    }

    #[test]
    fn m2_two_spellings_one_address() {
        let affine = affine_of(&identity_versor()).expect("affine");
        let w = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [5, 2, 0, 0]).expect("spells");
        let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
        let long = crate::transducer::alphabet::Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
        let mut alt = w.clone();
        if let Some(letters) = alt.letters() {
            let spelling = crate::transducer::alphabet::Spelling { word: long, rho: letters.spelling.rho };
            let rem = alt.sub().to_vec();
            alt.word = connection_word(&spelling, &letters.shift, &letters.sigma, &rem);
            alt.address.take();
            assert_eq!(alt.address(), w.address());
            assert!(alt.same_vertex(&w));
        }
    }
}
