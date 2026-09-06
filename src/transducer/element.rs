// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ELEMENT — the uniform carrier of an ANASTOME germ. An element IS the
//! DESCRIPTION of a transition; the SOLUTION is its inverse deduction `g⁻¹`,
//! never stored as a second word. Description ∘ solution = identity.
//!
//! The action is a versor, never a matrix (18.3). `Element { tape, action,
//! sub, mag }`: S is one versor `V = carriage · linear_versor(word)`, G the
//! hue permutation, T an extent in receipts, H the shell coordinates.
//! compose is the geometric product on S and permutation composition on G;
//! inverse is the versor inverse and refuses when it does not exist;
//! same_action is projective equality on V and equality of the permutation.
//! A cell is carried by the sandwich `V p Ṽ` and read at the sensor only.
//! English never appears here; addresses are wire identity, never reasoned on.

use crate::core::engine::Q;
use crate::transducer::germinate::{germ_bytes as residual_word, ResidualGerm};
use num_bigint::BigInt;
use num_traits::{One, Zero};

/// Stratum tags on torsion and transitions (not matrix fibre indices).
pub const FIBER_SPATIAL: u16 = 0;
pub const FIBER_GAUGE: u16 = 1;
pub const FIBER_TEMPORAL: u16 = 2;
pub const FIBER_SHELL: u16 = 3;
/// First index of a germinated axis (a versor, never a matrix).
pub const FIBER_GERMINATED: u16 = 4;

/// Hue states on the gauge (ARC palette 0..9; 0 = absent).
pub const PALETTE: usize = 10;

#[cfg(test)]
fn q(v: i64) -> Q {
    Q::from(BigInt::from(v))
}

/// The four strata of a word's action. S is one versor; G a hue permutation;
/// T an extent in receipts; H the shell coordinates. Germinated axes are
/// further versors. Nothing here is a matrix.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Action {
    /// S: `V = carriage · linear_versor(word)`. None is the identity motion.
    spatial: Option<Versor>,
    /// G: hue permutation as pairs. Empty is identity.
    gauge: Vec<(u8, u8)>,
    /// T: extent in receipts. Zero is none.
    temporal: u64,
    /// H: shell coordinate. Zero is none.
    shell: u64,
    /// Axes opened above the four strata, each a versor, tagged ≥ 4.
    germinated: Vec<(u16, Versor)>,
}

/// A germ as tape, action, chain, and magnitude.
#[derive(Debug, Clone)]
pub struct Element {
    /// Coordinate-exact ANASTOME description (empty for a pure residual chain).
    word: Vec<u8>,
    action: Action,
    /// Per-lane support, exact from the description.
    pub(crate) mag: [u64; 4],
    /// The CENTERED COBOUNDARY CHAIN — the residual's substance with geometry
    /// preserved up to translation: `(row, col, before, after)` over the changed
    /// cells, min-corner framed to the origin, sorted. Empty for a closed word.
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
    /// The memoized address — a measurement of the tape and the action,
    /// taken once and cleared wherever the base mutates. Never serialized.
    address: std::sync::OnceLock<crate::transducer::a21::Address>,
    /// THE GAUGE AXIS KEYED BY PROFILE: every assignment this word speaks
    /// (its own, its sections, its torsion orbits) reached by the key of
    /// its gauge charge — one read names the 0 / 1 / many assignments a
    /// body could be spoken with. Memoized; cleared wherever the fibre
    /// mutates. Never serialized.
    profiles: std::sync::OnceLock<Vec<(Vec<u8>, Vec<usize>)>>,
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

/// The key of a gauge profile: the charge coordinates written out.
pub fn gauge_key(asg: &[(u8, u8)]) -> Vec<u8> {
    let mut buf = vec![crate::transducer::a21::TAG_GAUGE];
    for ((b, a), n) in gauge_profile(asg) {
        buf.push(b);
        buf.push(a);
        buf.extend_from_slice(&n.to_le_bytes());
    }
    buf
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
    // The isometry is the images of the row and column steps, spelled in
    // the alphabet; the versor is realised from the word, never tabulated.
    let spelling = crate::transducer::alphabet::spell_images(image(1, 0), image(0, 1))?;
    conformal_of(&spelling.word, &translator_versor(tr, tc))
}

/// Linear word as a CGA versor, no translation. The identity translator is
/// formed here, not at the call site. `None` when the word does not keep
/// the sensor plane.
pub fn versor_of_word(word: &crate::transducer::alphabet::Word) -> Option<Versor> {
    linear_versor(word)
}

/// THE CONFORMAL REALISATION OF A WORD: the linear versor of the word
/// composed with the carriage by the geometric product. `None` when the
/// word does not preserve the plane.
pub fn conformal_of(word: &crate::transducer::alphabet::Word, carriage: &Versor) -> Option<Versor> {
    Some(carriage.geo(&linear_versor(word)?))
}

fn linear_versor(word: &crate::transducer::alphabet::Word) -> Option<Versor> {
    use crate::transducer::cga;
    let block = word.plane_block()?;
    let rho2 = &block[0][0] * &block[0][0] + &block[1][0] * &block[1][0];
    if rho2.is_zero() {
        return None;
    }
    let r: [[Q; 2]; 2] = std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            // The linear word is the rotation/orientation; dilation lives
            // on the carriage.
            &block[i][j] / Q::from_integer(BigInt::from(1))
        })
    });
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
    let mut v = rot;
    if flipped {
        // The half-turn on a third axis is the orientation character;
        // there is no reflection letter.
        v = v.geo(&cga::rotator(&Q::zero(), &Q::one())).geo(&cga::reflector());
    }
    Some(v)
}

/// Projective equality of two versors: the same transport up to a nonzero
/// rational scale (versors act projectively on the cone).
pub fn same_versor(a: &Versor, b: &Versor) -> bool {
    crate::transducer::cga::projective_ratio(a, b).is_some()
}

/// A4 carriage: dilator(ρ) · translator. Sensor pairs stay in `pair_form`,
/// `speak_rule`, and `orbit`.
pub fn carriage(rho: &Q, t: &(Q, Q)) -> Versor {
    crate::transducer::cga::dilator(rho).geo(&crate::transducer::cga::translator(&t.0, &t.1))
}

/// Translator component of a carriage: the origin image under the sandwich,
/// as a translator versor. None when the origin is sent to infinity.
pub fn translator_of(v: &Versor) -> Option<Versor> {
    let z = Q::zero();
    let img = crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&z, &z)))?;
    if img.0.is_zero() && img.1.is_zero() {
        Some(identity_versor())
    } else {
        Some(crate::transducer::cga::translator(&img.0, &img.1))
    }
}

/// Dilator component of a carriage: `v · T⁻¹` when `v = D · T`. None when
/// the translator cannot be read or inverted.
pub fn dilator_of(v: &Versor) -> Option<Versor> {
    let t = translator_of(v)?;
    let inv = t.versor_inverse()?;
    let d = v.geo(&inv);
    if same_versor(&d, &identity_versor()) {
        Some(identity_versor())
    } else {
        Some(d)
    }
}

fn compose_perm(a: &[(u8, u8)], b: &[(u8, u8)]) -> Vec<(u8, u8)> {
    if a.is_empty() {
        return b.to_vec();
    }
    if b.is_empty() {
        return a.to_vec();
    }
    let apply = |sigma: &[(u8, u8)], h: u8| -> u8 {
        sigma.iter().find(|(f, _)| *f == h).map(|(_, t)| *t).unwrap_or(h)
    };
    let mut out: Vec<(u8, u8)> = Vec::new();
    let mut seen: Vec<u8> = Vec::new();
    for &(f, _) in a.iter().chain(b.iter()) {
        if seen.contains(&f) {
            continue;
        }
        seen.push(f);
        let t = apply(b, apply(a, f));
        if t != f {
            out.push((f, t));
        }
    }
    out.sort_unstable();
    out
}

fn gauge_is_perm(sigma: &[(u8, u8)]) -> bool {
    let mut image: [u8; PALETTE] = std::array::from_fn(|i| i as u8);
    for &(f, t) in sigma {
        if (f as usize) >= PALETTE || (t as usize) >= PALETTE {
            return false;
        }
        image[f as usize] = t;
    }
    let mut seen = [false; PALETTE];
    image.iter().all(|&t| {
        let i = t as usize;
        if i >= PALETTE || seen[i] {
            false
        } else {
            seen[i] = true;
            true
        }
    })
}

fn invert_perm(a: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut out: Vec<(u8, u8)> = Vec::new();
    let mut tos: Vec<u8> = Vec::new();
    for &(f, t) in a {
        if tos.contains(&t) {
            return None;
        }
        tos.push(t);
        if f != t {
            out.push((t, f));
        }
    }
    out.sort_unstable();
    Some(out)
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

/// Canonical key of one hue pair.
pub fn hue_pair_key(before: u8, after: u8) -> Vec<u8> {
    vec![crate::transducer::a21::TAG_HUE, before, after]
}

/// The hues a pair key names. The tag is not a colour.
pub fn hue_pair_of(key: &[u8]) -> Option<(u8, u8)> {
    match key {
        [crate::transducer::a21::TAG_HUE, b, a] => Some((*b, *a)),
        _ => None,
    }
}

/// Canonical key of a whole assignment (the exact list of pairs).
pub fn assignment_key(asg: &[(u8, u8)]) -> Vec<u8> {
    let mut buf = vec![crate::transducer::a21::TAG_ASSIGN];
    buf.extend_from_slice(&(asg.len() as u32).to_le_bytes());
    for &(b, a) in asg {
        buf.push(b);
        buf.push(a);
    }
    buf
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
    for (k, fiber) in idx.fiber(crate::network::nerve::ROOT) {
        match first {
            None => first = Some((hue_pair_of(&k)?, fiber.clone())),
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
    /// The identity element — empty tape, identity action, no chain.
    pub fn identity() -> Self {
        Element {
            word: Vec::new(),
            action: Action::default(),
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

    /// The action's spatial versor: composed from the tape when letters
    /// exist, otherwise the stored S stratum.
    pub fn versor(&self) -> Option<Versor> {
        if let Some(letters) = self.letters() {
            return conformal_of(&letters.spelling.word, &letters.carriage);
        }
        self.action.spatial.clone()
    }

    /// Open a germinated axis holding `v`. Returns the axis index.
    pub fn open_axis(&mut self, v: Versor) -> u16 {
        let i = self
            .action
            .germinated
            .iter()
            .map(|(i, _)| *i)
            .max()
            .map(|m| m.saturating_add(1).max(FIBER_GERMINATED))
            .unwrap_or(FIBER_GERMINATED);
        self.action.germinated.push((i, v));
        self.address.take();
        i
    }

    pub fn germinated(&self) -> &[(u16, Versor)] {
        &self.action.germinated
    }

    pub fn gauge(&self) -> &[(u8, u8)] {
        &self.action.gauge
    }

    /// Occupied strata: S when the versor is not identity, G when the
    /// permutation is not empty, germinated axes by index.
    pub fn axes(&self) -> Vec<u16> {
        let mut a = Vec::new();
        if self.versor().is_some_and(|v| !same_versor(&v, &identity_versor())) {
            a.push(FIBER_SPATIAL);
        }
        let g = if let Some(letters) = self.letters() {
            letters.sigma
        } else {
            self.action.gauge.clone()
        };
        if !g.is_empty() {
            a.push(FIBER_GAUGE);
        }
        if self.action.temporal != 0 {
            a.push(FIBER_TEMPORAL);
        }
        if self.action.shell != 0 {
            a.push(FIBER_SHELL);
        }
        for (i, _) in &self.action.germinated {
            a.push(*i);
        }
        a
    }

    pub fn max_axis(&self) -> Option<u16> {
        self.axes().into_iter().max()
    }

    /// Set the per-lane support this word carries (an opened axis carries
    /// the listing's measure and the charts it joined).
    pub fn set_magnitude(&mut self, mag: [u64; 4]) {
        self.mag = mag;
    }

    pub fn is_identity(&self) -> bool {
        // A residual with a canceling hue tally has identity action but a
        // real chain — the chain decides. A word with a description tape
        // is never the identity.
        self.axes().is_empty()
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
        for (_, (sigma, ell, _)) in torsion.fiber(crate::network::nerve::ROOT) {
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
    pub fn assignments_by_profile(&self, key: &[u8]) -> Vec<usize> {
        let table = self.profiles.get_or_init(|| {
            let mut rows: Vec<(Vec<u8>, Vec<usize>)> = Vec::new();
            let mut keyed: Vec<(Vec<u8>, usize)> =
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
        match table.binary_search_by(|(k, _)| k.as_slice().cmp(key)) {
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

    /// THE CARRIAGE between two thoughts — the versor carrying this one's
    /// base geometry onto the other's, when they sit on one orbit. The chain
    /// is the boundary of a thought, never its identity. A chain-carrying and
    /// a chain-free element are never carried onto each other.
    pub fn carriage(&self, other: &Element) -> Option<Versor> {
        match (self.sub.as_slice(), other.sub.as_slice()) {
            ([], []) => self.same_action(other).then(identity_versor),
            ([], _) | (_, []) => None,
            (a, b) => transition(a, b),
        }
    }

    /// Group elements: the same action iff the versors agree projectively
    /// and the permutations are equal.
    pub fn same_action(&self, other: &Element) -> bool {
        self.sub.is_empty()
            && other.sub.is_empty()
            && match (self.versor(), other.versor()) {
                (None, None) => true,
                (Some(a), Some(b)) => same_versor(&a, &b),
                _ => false,
            }
            && self.action.gauge == other.action.gauge
            && self.germinated_eq(other)
    }

    fn germinated_eq(&self, other: &Element) -> bool {
        if self.action.germinated.len() != other.action.germinated.len() {
            return false;
        }
        self.action
            .germinated
            .iter()
            .zip(&other.action.germinated)
            .all(|((i, a), (j, b))| i == j && same_versor(a, b))
    }

    /// Same vertex of the Cayley graph: the M2 address, which is geodesic,
    /// ρ, shift, gauge, and the A5 chain. Two spellings of one vertex have
    /// one address; a different dilation, hue map, or placement is another
    /// vertex.
    pub fn same_vertex(&self, other: &Element) -> bool {
        self.address() == other.address()
    }

    /// COMPOSE — geometric product on S, permutation composition on G,
    /// geometric product on matching germinated axes. Associative.
    pub fn compose(&self, other: &Element) -> Element {
        let mut out = Element::identity();
        out.action.spatial = match (self.versor(), other.versor()) {
            (Some(a), Some(b)) => Some(a.geo(&b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
        out.action.gauge = compose_perm(&self.action.gauge, &other.action.gauge);
        out.action.temporal = self.action.temporal.saturating_add(other.action.temporal);
        out.action.shell = self.action.shell.saturating_add(other.action.shell);
        let mut germ = self.action.germinated.clone();
        for (j, w) in &other.action.germinated {
            match germ.iter_mut().find(|(i, _)| i == j) {
                Some((_, held)) => *held = held.geo(w),
                None => germ.push((*j, w.clone())),
            }
        }
        germ.sort_by_key(|(i, _)| *i);
        out.action.germinated = germ;
        out
    }

    /// SOLUTION — the inverse deduction `g⁻¹`. Versor inverse on S and on
    /// each germinated axis; inverse permutation on G. `None` when a versor
    /// is not invertible.
    pub fn solution(&self) -> Option<Element> {
        if !self.sub.is_empty() && self.versor().is_none() && self.action.germinated.is_empty() {
            return None;
        }
        let mut out = Element::identity();
        if let Some(v) = self.versor() {
            out.action.spatial = Some(v.versor_inverse()?);
        }
        out.action.gauge = invert_perm(&self.action.gauge)?;
        for (i, w) in &self.action.germinated {
            out.action.germinated.push((*i, w.versor_inverse()?));
        }
        Some(out)
    }

    /// Inverse: versor inverse when the action is a versor; the reverse
    /// coboundary chain when the substance is a residual germ. `None` when
    /// a versor is not invertible.
    pub fn inverse(&self) -> Option<Element> {
        if !self.sub.is_empty() && self.versor().is_none() && self.action.germinated.is_empty() {
            let rev: ResidualGerm = self
                .sub
                .iter()
                .map(|&(r, c, b, a)| (r, c, a, b))
                .collect();
            return Some(from_residual(&rev));
        }
        self.solution()
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

    /// The BASE canonical form: tape and chain. The action is on the tape
    /// or composed from it; germinated axes are listed by index.
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
        for (i, _) in &self.action.germinated {
            out.extend_from_slice(&i.to_le_bytes());
        }
        out
    }

    /// Vertex key of the word form: geodesic normal form plus frame
    /// coordinates as exact rationals. A sentence is keyed by its form
    /// (M1, ρ, agreed shift relative to the Frame, contact, gauge, faces).
    /// The body's absolute position is already quotiented on the chain.
    pub fn address(&self) -> crate::transducer::a21::Address {
        self.address
            .get_or_init(|| {
                if self.is_sentence() {
                    if let Some(k) = crate::transducer::sentence::vertex_address(self) {
                        return k;
                    }
                }
                let mut buf = vec![crate::transducer::a21::TAG_FORM];
                crate::transducer::a21::put_bytes(&mut buf, &self.form_key_bytes().unwrap_or_default());
                crate::transducer::a21::Address::new(buf)
            })
            .clone()
    }

    /// Meaning of a non-sentence word: M1 ‖ carriage ‖ σ ‖ chain (17.2).
    /// ρ is in the carriage, never beside it.
    fn form_key_bytes(&self) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        if let Some(rw) = crate::transducer::rewrite::Rewrite::from_element(self) {
            out.push(REWRITE_TAG);
            out.push(rw.kind as u8);
            out.extend_from_slice(&(rw.before.len() as u16).to_le_bytes());
            for a in &rw.before {
                crate::transducer::a21::put_addr(&mut out, a)?;
            }
            crate::transducer::a21::put_addr(&mut out, &rw.after)?;
            for m in &rw.claimed {
                out.extend_from_slice(&m.to_le_bytes());
            }
            return Some(out);
        }
        if let Some(letters) = self.letters() {
            crate::transducer::a21::put_bytes(&mut out, letters.spelling.word.key().as_bytes());
            letters.carriage.to_bytes(&mut out);
            let mut sigma = letters.sigma.clone();
            sigma.sort_unstable();
            out.extend_from_slice(&(sigma.len() as u16).to_le_bytes());
            for (f, t) in sigma {
                out.push(f);
                out.push(t);
            }
        } else if let Some(v) = self.action.spatial.as_ref() {
            v.to_bytes(&mut out);
        }
        let chain = canonical_chain(&self.sub).unwrap_or_else(|| self.sub.clone());
        out.extend_from_slice(&(chain.len() as u32).to_le_bytes());
        for (r, c, b, a) in &chain {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(*b);
            out.push(*a);
        }
        Some(out)
    }

    /// GEN3: tapes only. Magic `0xA3`, then word, mag, chain, sections,
    /// torsion, anchor, transitions as letter tapes. Addresses on the tape
    /// are `len16 + bytes`. An older file is refused.
    pub const GEN3_MAGIC: u8 = 0xA3;
    pub const GEN2_MAGIC: u8 = 0xA2;

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![Self::GEN3_MAGIC];
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
        out.extend_from_slice(&(self.action.germinated.len() as u32).to_le_bytes());
        for (i, v) in &self.action.germinated {
            out.extend_from_slice(&i.to_le_bytes());
            v.to_bytes(&mut out);
        }
        match &self.action.spatial {
            Some(v) if self.letters().is_none() => {
                out.push(1);
                v.to_bytes(&mut out);
            }
            _ => out.push(0),
        }
        out
    }

    /// Deserialize GEN3 only. An older file is refused, never migrated.
    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Element> {
        if b.get(*pos) == Some(&Self::GEN3_MAGIC) {
            return Self::from_bytes_gen2(b, pos);
        }
        None
    }

    fn from_bytes_gen2(b: &[u8], pos: &mut usize) -> Option<Element> {
        if b.get(*pos) != Some(&Self::GEN3_MAGIC) {
            return None;
        }
        *pos += 1;
        let u32_at = |b: &[u8], p: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v)
        };
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
        let mut germinated = Vec::new();
        if let Some(ng) = u32_at(b, pos) {
            for _ in 0..ng as usize {
                let i = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
                *pos += 2;
                let v = Versor::from_bytes(b, pos)?;
                germinated.push((i, v));
            }
        }
        let spatial = match b.get(*pos) {
            Some(1) => {
                *pos += 1;
                Some(Versor::from_bytes(b, pos)?)
            }
            Some(0) => {
                *pos += 1;
                None
            }
            _ => None,
        };
        let mut e = Element {
            word,
            action: Action {
                spatial,
                gauge: Vec::new(),
                temporal: mag[2],
                shell: mag[3],
                germinated,
            },
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
        e.sync_action_from_tape();
        Some(e)
    }

    fn sync_action_from_tape(&mut self) {
        let Some(letters) = self.letters() else {
            return;
        };
        self.action.spatial = conformal_of(&letters.spelling.word, &letters.carriage);
        self.action.gauge = letters.sigma;
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

    let mut e = Element::identity();
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
pub fn listing_key(chain: &[(i64, i64, u8, u8)], frame: &(usize, usize, usize, usize)) -> crate::transducer::a21::Address {
    from_residual_framed(&chain.to_vec(), frame).address()
}

/// Identity frame: no scale. Unframed speech uses this so its key matches
/// a planar intake.
pub const IDENTITY_FRAME: (usize, usize, usize, usize) = (1, 1, 1, 1);

pub fn from_residual_framed(g: &ResidualGerm, frame: &(usize, usize, usize, usize)) -> Element {
    let mut e = from_residual(g);
    let (hi, wi, ho, wo) = *frame;
    if hi > 0 && wi > 0 && (hi, wi) != (ho, wo) {
        let rh = Q::new(BigInt::from(ho as u64), BigInt::from(hi as u64));
        let rw = Q::new(BigInt::from(wo as u64), BigInt::from(wi as u64));
        if rh == rw && !rh.is_one() {
            e.action.spatial = Some(crate::transducer::cga::dilator(&rh));
            e.address.take();
        }
    }
    e
}

/// MINT THE CONNECTION: the linear word, the A4 carriage, the gauge map,
/// and the remainder chain. `V = carriage · linear_versor(word)`. A word
/// that does not keep the plane is not a connection.
pub fn from_connection(
    linear: &crate::transducer::alphabet::Word,
    carriage: &Versor,
    sigma: &[(u8, u8)],
    remainder: &ResidualGerm,
    carried: [u64; 4],
) -> Option<Element> {
    let v = conformal_of(linear, carriage)?;
    let spelling = crate::transducer::alphabet::Spelling { word: linear.clone() };
    let mut e = from_residual(remainder);
    e.word = connection_word(&spelling, carriage, sigma, &e.sub);
    e.action.spatial = Some(v);
    e.action.gauge = sigma.to_vec();
    e.address.take();
    for l in 0..4 {
        e.mag[l] = e.mag[l].saturating_add(carried[l]);
    }
    Some(e)
}

fn tape_of_versor(_g: &Versor) -> crate::transducer::alphabet::Word {
    crate::transducer::alphabet::Word::identity()
}

fn versor_of_tape(w: &crate::transducer::alphabet::Word) -> Versor {
    match versor_of_word(w) {
        Some(v) => v,
        None => identity_versor(),
    }
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
    carriage: &Versor,
    sigma: &[(u8, u8)],
    remainder: &ResidualGerm,
) -> Vec<u8> {
    let mut out = vec![CONNECTION_TAG];
    spelling.to_bytes(&mut out);
    carriage.to_bytes(&mut out);
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

/// THE LETTERS OF A CONNECTION as read off its description: the word,
/// the A4 carriage, and the gauge map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Letters {
    pub spelling: crate::transducer::alphabet::Spelling,
    pub carriage: Versor,
    pub sigma: Vec<(u8, u8)>,
}

impl Element {
    /// The letters of this word — `None` for a residual germ or a word
    /// spoken before the alphabet (no tape: not a word of the language).
    pub fn letters(&self) -> Option<Letters> {
        use crate::transducer::alphabet::Spelling;
        let b = &self.word;
        let tag = b.first().copied()?;
        if tag != CONNECTION_TAG && tag != SENTENCE_TAG && tag != TRANSLATION_TAG {
            return None;
        }
        let mut pos = 1usize;
        let spelling = Spelling::from_bytes(b, &mut pos)?;
        let carriage = Versor::from_bytes(b, &mut pos)?;
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
        Some(Letters { spelling, carriage, sigma })
    }

    /// Tape re-derives vertex: the geodesic of the stored word is the same
    /// transport, and V reconstructed from the tape agrees with the action.
    /// `None` when there are no letters.
    pub fn letters_rederive(&self) -> Option<bool> {
        let letters = self.letters()?;
        let v = conformal_of(&letters.spelling.word, &letters.carriage)?;
        let same_word = letters.spelling.word.matrix()? == letters.spelling.word.geodesic().matrix()?;
        Some(same_word && self.versor().is_some_and(|held| same_versor(&held, &v)))
    }

    /// The axes opened above the four strata: germinated versors.
    pub fn opened_axes(&self) -> Vec<u16> {
        self.action.germinated.iter().map(|(i, _)| *i).collect()
    }

    /// Is this word a CONNECTION — letters on the tape and an invertible
    /// versor. A residual chain is never a connection.
    pub fn is_connection(&self) -> bool {
        let Some(letters) = self.letters() else {
            return false;
        };
        if self.word.first() != Some(&CONNECTION_TAG) {
            return false;
        }
        conformal_of(&letters.spelling.word, &letters.carriage)
            .and_then(|v| v.versor_inverse())
            .is_some()
            && gauge_is_perm(&letters.sigma)
            && !(letters.spelling.word.is_identity()
                && same_versor(&letters.carriage, &identity_versor())
                && letters.sigma.is_empty()
                && self.sub.is_empty())
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
        let mut e = Element::identity();
        e.word = word;
        e.mag = mag;
        e.address.take();
        e.form.take();
        e
    }

    /// The two sections a translation gauges, if the tape carries them.
    pub fn translation_sections(&self) -> Option<(crate::transducer::a21::Address, crate::transducer::a21::Address)> {
        if !self.is_translation() {
            return None;
        }
        let b = self.description();
        // Walk back two length-delimited addresses at the tail.
        // The tape is prefix + addr_a + addr_b; decode from the start of the pair
        // by scanning: find the first put_addr that consumes the suffix.
        let mut q = 0usize;
        while q + 2 <= b.len() {
            let mut t = q;
            if let (Some(a), Some(c)) = (
                crate::transducer::a21::take_addr(b, &mut t),
                crate::transducer::a21::take_addr(b, &mut t),
            ) {
                if t == b.len() {
                    return Some((a, c));
                }
            }
            q += 1;
        }
        None
    }
}

/// A gauge between two sections, spelled as a connection with identity
/// gauge map and tagged so the court replays it as a translation.
pub fn from_translation(
    rotor: &crate::transducer::alphabet::Rotor,
    a: impl AsRef<[u8]>,
    b: impl AsRef<[u8]>,
) -> Option<Element> {
    let word = rotor.spelled_word()?;
    let mut e = from_connection(&word, &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0])?;
    let mut d = e.description().to_vec();
    if d.first() == Some(&CONNECTION_TAG) {
        d[0] = TRANSLATION_TAG;
    }
    crate::transducer::a21::put_addr(&mut d, a)?;
    crate::transducer::a21::put_addr(&mut d, b)?;
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
        out.word = Vec::new();
        out.address.take();
        out.form.take();
        if let Some(v) = out.versor() {
            if let Some(t) = translator_of(&v) {
                if !same_versor(&t, &identity_versor()) {
                    if let Some(inv) = t.versor_inverse() {
                        let d = v.geo(&inv);
                        out.action.spatial = if same_versor(&d, &identity_versor()) {
                            None
                        } else {
                            Some(d)
                        };
                    }
                }
            }
        }
        out.mag = if out.sub.is_empty() {
            out.reduction_magnitude()
        } else {
            measure_lanes(&out.sub)
        };
        out
    }

    /// Descent order: a non-identity versor occupies 𝒮, a nonempty
    /// permutation occupies 𝒢.
    pub fn reduction_magnitude(&self) -> [u64; 4] {
        let spatial = u64::from(self.versor().is_some_and(|v| !same_versor(&v, &identity_versor())));
        let gauge = u64::from(!self.action.gauge.is_empty());
        [spatial, gauge, 0, 0]
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

    fn elem_from_versor(v: Versor) -> Element {
        let mut e = Element::identity();
        e.action.spatial = Some(v);
        e
    }

    #[test]
    fn compose_is_the_geometric_product() {
        let a = elem_from_versor(translator_versor(1, 0));
        let b = elem_from_versor(translator_versor(0, 2));
        let c = elem_from_versor(translator_versor(3, 0));
        let lhs = a.compose(&b).compose(&c);
        let rhs = a.compose(&b.compose(&c));
        assert!(
            same_versor(&lhs.versor().expect("lhs"), &rhs.versor().expect("rhs")),
            "compose is the geometric product, associative"
        );
        let ab = a.compose(&b).versor().expect("ab");
        let want = translator_versor(1, 0).geo(&translator_versor(0, 2));
        assert!(same_versor(&ab, &want));
    }

    #[test]
    fn description_composed_with_its_solution_is_identity() {
        let g = elem_from_versor(translator_versor(3, -2));
        let sol = g.solution().expect("an invertible description has a solution");
        assert!(g.compose(&sol).is_identity(), "g ∘ g⁻¹ = e — the coboundary closes");
        assert!(sol.compose(&g).is_identity(), "g⁻¹ ∘ g = e");
    }

    #[test]
    fn inverse_refuses_a_non_versor() {
        let mut e = Element::identity();
        e.action.spatial = Some(Versor::zero());
        assert!(e.solution().is_none(), "a non-versor has no inverse");
        assert!(e.inverse().is_none(), "a non-versor has no inverse");
    }

    #[test]
    fn reverse_germ_is_the_inverse_chain() {
        let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
        let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)];
        let g = from_residual(&germinate(&a, &b).unwrap());
        assert!(g.solution().is_none(), "a residual has no versor inverse");
        let inv = g.inverse().expect("the reverse chain exists");
        let want: Vec<_> = g.sub().iter().map(|&(r, c, b, a)| (r, c, a, b)).collect();
        assert_eq!(inv.sub(), want.as_slice(), "inverse swaps before/after on the chain");
        let again = inv.inverse().expect("reverse of reverse");
        assert_eq!(again.sub(), g.sub(), "inverse ∘ inverse is the chain");
    }

    #[test]
    fn same_action_is_projective_versor_equality() {
        let t = translator_versor(1, 0);
        let a = elem_from_versor(t.clone());
        let scaled = elem_from_versor(t.scale(&q(3)));
        let other = elem_from_versor(translator_versor(0, 1));
        assert!(a.same_action(&scaled), "projective equality of V");
        assert!(!a.same_action(&other), "distinct versors are distinct actions");
    }

    #[test]
    fn the_solution_inverts_the_described_motion() {
        let g = elem_from_versor(translator_versor(4, 4));
        let v = g.versor().expect("V");
        let inv = g.solution().expect("inverse").versor().expect("V⁻¹");
        let qn = |n: i64| Q::from(BigInt::from(n));
        let p = crate::transducer::cga::point(&qn(1), &qn(2));
        let moved = v.sandwich(&p);
        let back = inv.sandwich(&moved);
        assert_eq!(
            crate::transducer::cga::base(&back),
            crate::transducer::cga::base(&p),
            "g⁻¹ carries the image back to the origin point"
        );
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
        let id = crate::transducer::alphabet::Word::identity();
        let remainder = vec![(0, 0, 3u8, 4u8)];
        let w = from_connection(&id, &identity_versor(), &[(1, 2), (2, 1)], &remainder, [5, 2, 0, 0]).expect("the identity spells");
        assert!(w.is_connection(), "invertible versor and a permutation on the hues");
        assert_eq!(w.letters_rederive(), Some(true), "the tape re-derives V");
        assert_eq!(w.letters().map(|l| l.sigma), Some(vec![(1, 2), (2, 1)]), "the gauge rides the tape");
        let mut pos = 0;
        let back = Element::from_bytes(&w.to_bytes(), &mut pos).expect("round trip");
        assert_eq!(back.letters(), w.letters(), "the letters survive the wire");
        assert!(w.solution().is_some(), "the group component inverts");
        assert_eq!(w.sub(), remainder.as_slice(), "the remainder is the chain");
        assert_ne!(w.address(), from_residual(&remainder).address(), "the connection is not the bare residual");
        let qid = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
        let long = crate::transducer::alphabet::Word::from_letters(&[(qid, false), (qid, false), (qid, false), (qid, false)]);
        assert_eq!(long.key(), crate::transducer::alphabet::Word::identity().key());
        let mut alt = w.clone();
        if let Some(letters) = alt.letters() {
            let spelling = crate::transducer::alphabet::Spelling { word: long };
            let rem = alt.sub().to_vec();
            alt.word = connection_word(&spelling, &letters.carriage, &letters.sigma, &rem);
            alt.sync_action_from_tape();
            alt.address.take();
            assert_eq!(alt.address(), w.address(), "q^4 and 1 are one vertex; the tape is not the address");
            assert!(alt.same_vertex(&w), "q^4 and 1 are one vertex; equality is the rotor, not a digest");
        }
        assert!(w.magnitude()[0] >= 5 && w.magnitude()[1] >= 2, "carried volume rides on the lanes");
        let closed = from_connection(&id, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [5, 2, 0, 0]).expect("spells");
        assert!(closed.is_connection() && closed.sub().is_empty(), "an empty remainder is a closed connection");
        let merge = from_connection(&id, &identity_versor(), &[(1, 2)], &Vec::new(), [1, 1, 0, 0]).expect("spells");
        assert!(!merge.is_connection(), "a merging gauge with the identity motion carries no group component");
        assert!(!from_residual(&remainder).is_connection(), "a residual's tally is never a permutation");
        let unit = from_connection(&id, &identity_versor(), &[], &Vec::new(), [77, 0, 0, 0]).expect("spells");
        assert!(unit.is_identity(), "identity motion and gauge leave no action and no chain");
        assert!(!unit.is_connection(), "the unit is not a connection");
        assert!(unit.sub().is_empty(), "nothing residual to hold as thought");
    }

    #[test]
    fn gen3_round_trips_and_gen2_is_refused() {
        let w = from_connection(&crate::transducer::alphabet::Word::identity(), &identity_versor(), &[(1, 2), (2, 1)], &vec![(0, 0, 3u8, 4)], [5, 2, 0, 0]).expect("spells");
        let bytes = w.to_bytes();
        assert_eq!(bytes.first().copied(), Some(Element::GEN3_MAGIC));
        let mut pos = 0;
        let back = Element::from_bytes(&bytes, &mut pos).expect("GEN3");
        assert_eq!(back.address(), w.address());
        assert_eq!(back.letters().map(|l| l.sigma), w.letters().map(|l| l.sigma));
        assert!(back.letters_rederive() == Some(true));
        let mut old = bytes.clone();
        old[0] = Element::GEN2_MAGIC;
        let mut p = 0;
        assert!(Element::from_bytes(&old, &mut p).is_none(), "GEN2 is refused");
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
        let mut spatial = elem_from_versor(translator_versor(1, 0));
        let mut germ = Element::identity();
        germ.open_axis(translator_versor(0, 1));
        let both = spatial.compose(&germ);
        assert_eq!(both.axes(), vec![FIBER_SPATIAL, FIBER_GERMINATED], "both axes present");
        assert!(same_versor(
            &both.versor().expect("S"),
            &spatial.versor().expect("S")
        ));
        assert_eq!(both.max_axis(), Some(FIBER_GERMINATED));
        assert!(!both.same_action(&spatial), "a germinated axis is a new generator, not a new digest");
        spatial.open_axis(translator_versor(2, 0));
        assert_eq!(spatial.opened_axes(), vec![FIBER_GERMINATED]);
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
        let id = crate::transducer::alphabet::Word::identity();
        let swap = from_connection(&id, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("swap");
        let cycle = from_connection(&id, &identity_versor(), &[(1, 2), (2, 3), (3, 1)], &Vec::new(), [1, 0, 0, 0]).expect("cycle");
        let place = from_connection(&id, &translator_versor(5, 0), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("shift");
        let mut dilated = swap.clone();
        if let Some(letters) = dilated.letters() {
            let two = Q::from(BigInt::from(2));
            let car = crate::transducer::cga::dilator(&two).geo(&letters.carriage);
            let rem = dilated.sub().to_vec();
            dilated.word = connection_word(&letters.spelling, &car, &letters.sigma, &rem);
            dilated.sync_action_from_tape();
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
        let w = from_connection(&crate::transducer::alphabet::Word::identity(), &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [5, 2, 0, 0]).expect("spells");
        let qid = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
        let long = crate::transducer::alphabet::Word::from_letters(&[(qid, false), (qid, false), (qid, false), (qid, false)]);
        let mut alt = w.clone();
        if let Some(letters) = alt.letters() {
            let spelling = crate::transducer::alphabet::Spelling { word: long };
            let rem = alt.sub().to_vec();
            alt.word = connection_word(&spelling, &letters.carriage, &letters.sigma, &rem);
            alt.sync_action_from_tape();
            alt.address.take();
            assert_eq!(alt.address(), w.address());
            assert!(alt.same_vertex(&w));
        }
    }

    #[test]
    fn rotated_connection_survives_the_tape() {
        let qid = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let rot = crate::transducer::alphabet::Word::from_letters(&[(qid, false)]);
        let two = Q::from(BigInt::from(2));
        let car = crate::transducer::cga::dilator(&two).geo(&translator_versor(3, 5));
        let w = from_connection(&rot, &car, &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
        let v0 = w.versor().expect("V");
        let bytes = w.to_bytes();
        let mut pos = 0;
        let back = Element::from_bytes(&bytes, &mut pos).expect("reload");
        let v1 = back.versor().expect("V'");
        assert!(same_versor(&v0, &v1), "tape → Element → tape is projective_eq of V");
        let qn = |n: i64| Q::from(BigInt::from(n));
        let p = crate::transducer::cga::point(&qn(1), &qn(0));
        assert_eq!(
            crate::transducer::cga::base(&v0.sandwich(&p)),
            crate::transducer::cga::base(&v1.sandwich(&p)),
            "equal spoken cells on a rotated, dilated, translated body"
        );
    }

    #[test]
    fn carriage_holds_rho_once() {
        let two = Q::from(BigInt::from(2));
        let car = crate::transducer::cga::dilator(&two);
        let w = from_connection(&crate::transducer::alphabet::Word::identity(), &car, &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
        let letters = w.letters().expect("letters");
        let d = dilator_of(&letters.carriage).expect("dilator");
        assert!(same_versor(&d, &car), "ρ lives in the carriage");
        let key = w.address();
        // form_key is M1 ‖ carriage ‖ σ ‖ chain: no second ρ beside the carriage.
        assert!(key.as_bytes().len() > 1);
    }

    #[test]
    fn no_qmatrix_in_cognition() {
        let src = include_str!("element.rs");
        let prod = src.split("#[cfg(test)]").next().expect("prod");
        assert!(
            !prod.contains("fibers: Vec") && !prod.contains("as_operator") && !prod.contains("realize("),
            "the Element's action is a versor, never a matrix"
        );
    }

    #[test]
    fn connection_address_keys_the_carriage_not_a_pair() {
        let car = translator_versor(3, 5);
        let w = from_connection(&crate::transducer::alphabet::Word::identity(), &car, &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
        let letters = w.letters().expect("letters");
        let mut bytes = Vec::new();
        letters.carriage.to_bytes(&mut bytes);
        assert!(
            w.address().as_bytes().windows(bytes.len()).any(|s| s == bytes.as_slice()),
            "M2 keys the A4 carriage, not a coordinate pair"
        );
        let w2 = from_connection(&crate::transducer::alphabet::Word::identity(), &translator_versor(7, 0), &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
        assert_ne!(w.address(), w2.address());
    }
}
