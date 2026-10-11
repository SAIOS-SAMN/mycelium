// 78.99 (1), 78.68, WN1 — A WORD'S COORDINATES ON THE MANIFOLD IT CONSTRUCTS:
// per mode, its HOLE — the generator its orbit winds about, a canonical ray —
// and its WINDING there; the word rebuilt from them, and comprehended by them.
//
//! A word applied to its ground sweeps an orbit, and the orbit winds about
//! the fixed locus of each of its modes (MR1, `motion_modes`). That locus is
//! the mode's own generator `K_j` — the word's plane, never a chart's axis —
//! held as its canonical ray `K̂_j`, and where the mode stands on it is read
//! against `K̂_j` exactly:
//!
//! ```text
//!     elliptic, order ℓ    Turn  t/ℓ        the turn 360°·t/ℓ (Niven: ℓ ∈ {2,3,4,6})
//!     parabolic            Count λ          K_j = λ·K̂_j, one count of one direction
//!     rosette, boost       Named (c, λ)     the rational point and the side
//! ```
//!
//! THE COORDINATES ARE THE WORD, NEVER A FIELD BESIDE IT (law 12, 78.68's
//! turnstile): each mode's ray is `Σ_k (−K_j)^{∧k} / (k!(1 + c_j)^k)` with
//! `K_j = (1 + c_j)·λ_j·K̂_j` — the 180° turn its space's blade `K̂_j` itself
//! — and the rays multiply back to the word (EQ1), proved by [`Coordinates::word`].
//!
//! COMPREHENSION BY COORDINATES (78.100 D3). A held word is found by its
//! holes, a keyed fibre per hole; a new word is DECONSTRUCTED mode by mode —
//! each hole cited to a held word on it whose winding reaches the word's by
//! an exponent — and what no held word covers is its residue. Every mode
//! cited, it is a composition of what is held (a power of one word the case
//! where one word covers every hole): derivable, never held twice.

use crate::network::nerve::coord::BivectorKind;
use crate::network::nerve::Keyed;
use crate::transducer::cga::{self, Mv, MotionMode, Undecomposed};
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

type Q = BigRational;

/// WHERE ONE MODE STANDS ON ITS HOLE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Winding {
    /// An elliptic mode of finite order: the turn `t/ℓ` of 360°, reduced,
    /// `0 < t < ℓ`.
    Turn { t: u32, ell: u32 },
    /// A parabolic mode: `K = λ·K̂`, one count along one direction.
    Count(Q),
    /// A rosette or a boost: never returns and does not condense (78.68),
    /// held by its rational point `c` and its side `λ`.
    Named { c: Q, lambda: Q },
}

/// ONE HOLE OF A WORD'S CONSTRUCTION AND ITS WINDING THERE: the canonical
/// generator `K̂` (its key the canonical writing), the mode's class and
/// planes, its rational point `c` and its side `λ` — what rebuilds the mode
/// — and the winding they read as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hole {
    pub key: Vec<u8>,
    pub generator: Mv,
    pub kind: BivectorKind,
    pub planes: usize,
    pub c: Q,
    pub lambda: Q,
    pub winding: Winding,
}

/// A WORD'S COORDINATES: its holes in key order (content order, law 2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coordinates {
    pub holes: Vec<Hole>,
}

/// WHY A WORD'S COORDINATES DID NOT READ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unread {
    /// The word is no versor: it has no action, so no modes.
    NoAction,
    /// Its modes did not split over ℚ, or the stroke was spent reading them.
    Modes(Undecomposed),
    /// A mode's generator has no canonical ray.
    Generator,
}

/// THE ORDER OF A TURN WHOSE POINT IS `c` (Niven): 2 at −1, 3 at −½, 4 at 0,
/// 6 at ½ — the only rational cosines of a finite order; `None` elsewhere.
fn niven_order(c: &Q) -> Option<u32> {
    let half = Q::new(1.into(), 2.into());
    match () {
        _ if *c == -Q::one() => Some(2),
        _ if *c == -half.clone() => Some(3),
        _ if c.is_zero() => Some(4),
        _ if *c == half => Some(6),
        _ => None,
    }
}

/// The turn `t/ℓ` a mode of rational point `c` and side `λ` stands at: the
/// base turn of `c` (Niven's points, the only rational cosines of a finite
/// order), `t = 1` on the side `λ > 0` and `t = ℓ − 1` on the other. At the
/// 180° turn the side is void and `t = 1`.
fn turn_of(c: &Q, lambda: &Q, ell: u32) -> Option<Winding> {
    if niven_order(c)? != ell {
        return None;
    }
    let t = match (ell, lambda.is_negative()) {
        (2, _) => 1,
        (_, false) => 1,
        (_, true) => ell - 1,
    };
    Some(Winding::Turn { t, ell })
}

impl Hole {
    /// ONE MODE READ AS ITS HOLE AND WINDING. The generator is the ray's
    /// bivector part with its sign turned (`ray = 1 − K/(1 + c) + …`), the
    /// 180° turn's the ray itself (its space's blade). `λ` is the generator
    /// against its canonical ray — the coefficient at the canonical's first
    /// non-zero slot, where the canonical holds `+1`.
    fn of(mode: &MotionMode) -> Result<Hole, Unread> {
        let generator = match mode.c == -Q::one() {
            true => mode.ray.clone(),
            false => mode.ray.grade(2).scale(&-Q::one()),
        };
        Hole::from_parts(&generator, mode.kind, mode.planes, mode.c.clone(), Q::one(), mode.order).ok_or(Unread::Generator)
    }

    /// A HOLE FROM WHAT REBUILDS IT — its generator (any scale), its rational
    /// point and its side — canonicalized and its winding read, exactly as
    /// [`Hole::of`] reads a mode. `None` where the generator is zero or a
    /// turn's point is no finite order's.
    fn from_parts(generator: &Mv, kind: BivectorKind, planes: usize, c: Q, lambda: Q, order: Option<u32>) -> Option<Hole> {
        let canonical = generator.canonical()?;
        let (at, _) = canonical.terms().next()?;
        let half_turn = c == -Q::one();
        // λ against the canonical ray: the generator is `μ·K̂` with `μ` its
        // coefficient at the canonical's first slot, so `λ·generator = λμ·K̂`.
        let lambda = match half_turn {
            true => Q::one(),
            false => lambda * generator.coeff(at),
        };
        let mut key = Vec::new();
        canonical.put_canonical(&mut key);
        let winding = match (kind, order) {
            (BivectorKind::Elliptic, Some(ell)) => turn_of(&c, &lambda, ell)?,
            (BivectorKind::Parabolic, _) => Winding::Count(lambda.clone()),
            _ => Winding::Named { c: c.clone(), lambda: lambda.clone() },
        };
        Some(Hole { key, generator: canonical, kind, planes, c, lambda, winding })
    }

    /// THE ORDER this hole's turn closes at, read off its winding.
    fn order(&self) -> Option<u32> {
        match self.winding {
            Winding::Turn { ell, .. } => Some(ell),
            _ => None,
        }
    }

    /// 78.99 (5) — THE HOLE CARRIED BY A CHART TRANSPORT `Γ`: its generator
    /// conjugated, `Γ K̂ Γ̃ / (Γ Γ̃)` (an automorphism of the algebra, so the
    /// rebuilt ray is the conjugated ray and the point `c` is unchanged), and
    /// its side and winding re-read against the new canonical ray. `None`
    /// where `Γ` is no versor.
    pub fn transported(&self, gamma: &Mv) -> Option<Hole> {
        let w = cga::weight_of(gamma).filter(|w| !w.is_zero())?;
        let carried = gamma.sandwich(&self.generator).scale(&(Q::one() / w));
        Hole::from_parts(&carried, self.kind, self.planes, self.c.clone(), self.lambda.clone(), self.order())
    }

    /// THE MODE'S RAY, rebuilt from the hole: `Σ_k (−K)^{∧k} / (k!(1 + c)^k)`
    /// with `K = (1 + c)·λ·K̂`, and the 180° turn's blade itself.
    pub fn ray(&self) -> Mv {
        let lift = Q::one() + &self.c;
        if lift.is_zero() {
            return self.generator.clone();
        }
        let minus = self.generator.scale(&-(&lift * &self.lambda));
        let mut ray = Mv::scalar(Q::one());
        let mut power = Mv::scalar(Q::one());
        let mut k = 1u32;
        loop {
            power = power.geo(&minus).grade(2 * k);
            if power.is_zero() {
                break;
            }
            let factorial = (1..=k).fold(Q::one(), |f, i| f * Q::from_integer(i.into()));
            let weight = Q::one() / (factorial * num_traits::pow(lift.clone(), k as usize));
            ray = ray.add(&power.scale(&weight));
            k += 1;
        }
        ray
    }
}

impl Coordinates {
    /// A WORD'S COORDINATES, read off its own modes under the stroke. `Err`
    /// by name where they do not read — never an empty reading (rule 11);
    /// the identity has no hole and reads empty.
    pub fn of(word: &Mv, budget: &mut crate::physics::Budget) -> Result<Coordinates, Unread> {
        let modes = match cga::motion_modes(word, budget) {
            None => return Err(Unread::NoAction),
            Some(Err(why)) => return Err(Unread::Modes(why)),
            Some(Ok(modes)) => modes,
        };
        let mut holes = modes.iter().map(Hole::of).collect::<Result<Vec<Hole>, Unread>>()?;
        #[expect(clippy::disallowed_methods, reason = "rule 4 — a sort by a canonical coordinate: the hole's key, content order")]
        holes.sort_by(|a, b| a.key.cmp(&b.key));
        Ok(Coordinates { holes })
    }

    /// THE WORD THE COORDINATES ARE: the holes' rays multiplied back, as a
    /// ray. The modes commute (MR1 verified it), so the product's order is
    /// the key order and any other gives the same ray.
    pub fn word(&self) -> Mv {
        self.holes.iter().fold(Mv::scalar(Q::one()), |acc, h| acc.geo(&h.ray()))
    }

    /// 78.99 (5) — INFLECTION: the coordinates carried by the chart transport
    /// `Γ` between two minds, hole by hole, re-sorted by their new keys. The
    /// receiver's word is `Γ W Γ̃`, rebuilt from these. `None` where `Γ` is no
    /// versor.
    pub fn transported(&self, gamma: &Mv) -> Option<Coordinates> {
        let mut holes = self.holes.iter().map(|h| h.transported(gamma)).collect::<Option<Vec<Hole>>>()?;
        #[expect(clippy::disallowed_methods, reason = "rule 4 — a sort by a canonical coordinate: the hole's key, content order")]
        holes.sort_by(|a, b| a.key.cmp(&b.key));
        Some(Coordinates { holes })
    }

    /// 78.100 — THE WORD'S HARMONIC SIGNATURE, read off its holes: each
    /// mode's tone, its plane left behind (the node's own stratum).
    pub fn signature(&self) -> Signature {
        Signature::of_tones(self.holes.iter().map(|h| Tone { kind: h.kind, planes: h.planes, c: h.c.clone(), order: h.order() }).collect())
    }

    /// THE HOLES' KEYS, in order: the fibre a word is found under.
    pub fn hole_keys(&self) -> impl Iterator<Item = &[u8]> {
        self.holes.iter().map(|h| h.key.as_slice())
    }

    /// IS THIS WORD `held`'s POWER `k`: the same holes, and on every one the
    /// winding `k` times the held word's — `t/ℓ ≡ k·t₀/ℓ₀ (mod 1)` on a turn,
    /// `λ = k·λ₀` on a count, a named point equal only at `k = 1`. The `k` is
    /// read off the windings: a count fixes it; turns alone admit it modulo
    /// the least common multiple of their orders (at most twelve), and the
    /// least such `k ≥ 2` is returned. `None` where no `k ≥ 2` holds —
    /// the same word is recognized by its key, never here.
    pub fn power_of(&self, held: &Coordinates) -> Option<u64> {
        if self.holes.len() != held.holes.len() || self.holes.iter().zip(&held.holes).any(|(a, b)| a.key != b.key) {
            return None;
        }
        let pairs: Vec<(&Winding, &Winding)> = self.holes.iter().map(|h| &h.winding).zip(held.holes.iter().map(|h| &h.winding)).collect();
        // A count fixes `k`: one ratio, integral and at least two.
        let fixed = pairs.iter().find_map(|(w, w0)| match (w, w0) {
            (Winding::Count(l), Winding::Count(l0)) if !l0.is_zero() => Some(l / l0),
            _ => None,
        });
        let holds = |k: u64| pairs.iter().all(|(w, w0)| winds_k(w, w0, k));
        match fixed {
            Some(r) => {
                let k = r.is_integer().then(|| r.to_integer())?;
                let k = u64::try_from(k).ok().filter(|k| *k >= 2)?;
                holds(k).then_some(k)
            }
            None => {
                let period = pairs.iter().try_fold(1u64, |l, (w, _)| match w {
                    Winding::Turn { ell, .. } => Some(lcm(l, u64::from(*ell))),
                    _ => None,
                })?;
                (2..=period.max(2)).find(|k| holds(*k))
            }
        }
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        (x, y) = (y, x % y);
    }
    a / x * b
}

/// Does `w` stand at `k` times `w0` on one hole.
fn winds_k(w: &Winding, w0: &Winding, k: u64) -> bool {
    match (w, w0) {
        (Winding::Turn { t, ell }, Winding::Turn { t: t0, ell: ell0 }) => {
            // `t/ℓ − k·t₀/ℓ₀ ∈ ℤ`, cross-multiplied: `t·ℓ₀ ≡ k·t₀·ℓ (mod ℓ·ℓ₀)`.
            let m = u64::from(*ell) * u64::from(*ell0);
            let lhs = u64::from(*t) * u64::from(*ell0) % m;
            let rhs = (k % m) * u64::from(*t0) % m * u64::from(*ell) % m;
            lhs == rhs
        }
        (Winding::Count(l), Winding::Count(l0)) => *l == l0 * Q::from_integer(k.into()),
        (Winding::Named { .. }, Winding::Named { .. }) => false,
        _ => false,
    }
}

/// 78.100 — ONE MODE'S TONE: what every node hears of it — its class by the
/// sign of its square, the planes it spans, its rational point `c` (`cos` of
/// a turn, `cosh` of a boost, `1` on a null flow) and the order it closes at.
/// Each is unchanged by any chart transport `Γ` (conjugation keeps the
/// spectrum of the action); the plane the mode turns in is the node's own
/// (78.44(2)) and is not a tone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tone {
    pub kind: BivectorKind,
    pub planes: usize,
    pub c: Q,
    pub order: Option<u32>,
}

/// 78.100 — A WORD'S HARMONIC SIGNATURE: its tones in their own content
/// order. It LOCATES the word in a hearer's language — the stratum every
/// node shares — and never identifies it (78.60): a slide by one and a
/// slide by two sound alike, as two turns of one angle about different
/// axes do; what tells them apart travels as the word's deconstruction over
/// the syllabus (78.100 D5).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature {
    pub tones: Vec<Tone>,
}

fn kind_byte(k: BivectorKind) -> u8 {
    match k {
        BivectorKind::Trivial => 0,
        BivectorKind::Elliptic => 1,
        BivectorKind::Hyperbolic => 2,
        BivectorKind::Parabolic => 3,
    }
}

impl Signature {
    /// The tones in their content order: by class, planes, point, order —
    /// the reading's own coordinates, never an arrival.
    fn of_tones(mut tones: Vec<Tone>) -> Signature {
        #[expect(clippy::disallowed_methods, reason = "rule 4 — a sort by canonical coordinates: the tone's class, planes, point and order")]
        tones.sort_by(|a, b| (kind_byte(a.kind), a.planes, &a.c, a.order).cmp(&(kind_byte(b.kind), b.planes, &b.c, b.order)));
        Signature { tones }
    }

    /// A WORD'S SIGNATURE read off its own modes under the stroke; `Err` by
    /// name where they do not read (rule 11).
    pub fn of(word: &Mv, budget: &mut crate::physics::Budget) -> Result<Signature, Unread> {
        Ok(Coordinates::of(word, budget)?.signature())
    }

    /// THE SIGNATURE WRITTEN BY ITS OWN COORDINATES: per tone its class, its
    /// planes, its point (an exact rational) — the order re-read from the
    /// point (Niven), never carried (law 12).
    pub fn put(&self, out: &mut Vec<u8>) {
        crate::transducer::address::put_key_count(out, self.tones.len());
        for t in &self.tones {
            out.push(kind_byte(t.kind));
            crate::transducer::address::put_key_count(out, t.planes);
            Mv::scalar(t.c.clone()).to_bytes(out);
        }
    }

    /// [`Signature::put`]'s inverse, whole or refused.
    pub fn take(b: &[u8], pos: &mut usize) -> Option<Signature> {
        let count = crate::transducer::address::take_key_count(b, pos)?;
        let mut tones = Vec::with_capacity(count.min(16));
        for _ in 0..count {
            let kind = match b.get(*pos)? {
                0 => BivectorKind::Trivial,
                1 => BivectorKind::Elliptic,
                2 => BivectorKind::Hyperbolic,
                3 => BivectorKind::Parabolic,
                _ => return None,
            };
            *pos += 1;
            let planes = crate::transducer::address::take_key_count(b, pos)?;
            let c = Mv::from_bytes(b, pos)?.scalar_part();
            let order = match kind {
                BivectorKind::Elliptic => niven_order(&c),
                _ => None,
            };
            tones.push(Tone { kind, planes, c, order });
        }
        Some(Signature::of_tones(tones))
    }
}

/// 78.99 (1) — THE HOLES A LEXICON HOLDS: per hole key, the words standing on
/// it, by their own keys — the fibre a new word's comprehension reads.
#[derive(Default)]
pub struct HoleIndex {
    on: Keyed<Keyed<()>>,
}

/// 78.100 D3 — ONE MODE OF A WORD CITED BY ITS MOTION: the held word that
/// stands on the same hole, by its key, and the exponent its winding there is
/// raised to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cite {
    pub word: Vec<u8>,
    pub hole: Vec<u8>,
    pub exponent: i64,
}

/// 78.100 D3, D4 — A WORD DECONSTRUCTED OVER WHAT IS HELD, MODE BY MODE: each
/// of its holes cited to a held word on that hole at an exponent, and the
/// holes no held word covers — the RESIDUE, the demand. A word is a product
/// over its dimensions, never a chain: its modes commute (MR1), so the
/// cited modes and the residue multiply back to it in any order
/// (`Coordinates::word`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Deconstruction {
    pub cites: Vec<Cite>,
    pub residue: Vec<Hole>,
}

impl Deconstruction {
    /// Every mode cited: the word is a composition of what is held — grammar,
    /// in the span at zero rank (78.100 S2).
    pub fn derived(&self) -> bool {
        self.residue.is_empty() && !self.cites.is_empty()
    }
}

/// THE EXPONENT `e` WITH `h = h₀ᵉ` ON ONE HOLE, read exactly off the windings:
/// a count by the integer ratio of its sides (any non-zero integer, a
/// reverse slide negative); a turn by the least `e ≥ 1` with `t/ℓ ≡ e·t₀/ℓ₀`
/// (mod 1), searched within the orders' least common multiple; a named
/// point only as itself, `e = 1`. `None` where no exponent reaches it.
fn exponent_on(h: &Hole, h0: &Hole) -> Option<i64> {
    match (&h.winding, &h0.winding) {
        (Winding::Count(l), Winding::Count(l0)) if !l0.is_zero() => {
            let r = l / l0;
            let e = r.is_integer().then(|| r.to_integer())?;
            let e = i64::try_from(e).ok().filter(|e| *e != 0)?;
            Some(e)
        }
        (Winding::Turn { ell, .. }, Winding::Turn { ell: ell0, .. }) => {
            let period = lcm(u64::from(*ell), u64::from(*ell0));
            let e = (1..=period).find(|k| winds_k(&h.winding, &h0.winding, *k))?;
            i64::try_from(e).ok()
        }
        (Winding::Named { .. }, Winding::Named { .. }) => (h.c == h0.c && h.lambda == h0.lambda).then_some(1),
        _ => None,
    }
}

impl HoleIndex {
    /// 78.100 D3 — DECONSTRUCT A WORD BY ITS MOTION over the held words, each
    /// held word's coordinates read by its key through `held`: every hole of
    /// the word cited to the first held word, in key order (content, law 2),
    /// that stands on the same hole and whose winding there reaches the
    /// word's by an exponent; a hole none reaches is residue. Exact: no
    /// winding is approximated, and a hole is cited only where the exponent
    /// holds.
    pub fn deconstruct<'a>(&self, coords: &Coordinates, held: impl Fn(&[u8]) -> Option<&'a Coordinates>) -> Deconstruction {
        let mut out = Deconstruction::default();
        for h in &coords.holes {
            let cite = self.on.get(&h.key).and_then(|fibre| {
                fibre.scoped().find_map(|(k, ())| {
                    let h0 = held(&k)?.holes.iter().find(|h0| h0.key == h.key)?;
                    let exponent = exponent_on(h, h0)?;
                    Some(Cite { word: k, hole: h.key.clone(), exponent })
                })
            });
            match cite {
                Some(c) => out.cites.push(c),
                None => out.residue.push(h.clone()),
            }
        }
        out
    }

    /// Whether any held word stands on `hole`.
    pub fn stands_on(&self, hole: &[u8]) -> bool {
        self.on.get(hole).is_some()
    }

    /// FILE A HELD WORD under each of its holes.
    pub fn file(&mut self, word: &[u8], coords: &Coordinates) {
        for h in &coords.holes {
            match self.on.get_mut(&h.key) {
                Some(f) => {
                    f.insert(word, ());
                }
                None => {
                    let mut f = Keyed::new();
                    f.insert(word, ());
                    self.on.insert(&h.key, f);
                }
            }
        }
    }

    /// The mirror of [`HoleIndex::file`]: the word leaves each of its holes'
    /// fibres, and a fibre left empty leaves with it.
    pub fn unfile(&mut self, word: &[u8], coords: &Coordinates) {
        for h in &coords.holes {
            let empty = match self.on.get_mut(&h.key) {
                Some(f) => {
                    f.remove(word);
                    f.is_empty()
                }
                None => false,
            };
            if empty {
                self.on.remove(&h.key);
            }
        }
    }
}
