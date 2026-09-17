// 50.4, A22 — THE ORDERED FACTORIZATION: a measured rotor spelled into the
// letters, one commuting plane pair at a time, or the typed demand it leaves.
//
//! `Spin(4,0)` is not abelian across planes sharing an axis, so no single
//! lattice decides a rotor. The even subalgebra splits by its central
//! idempotents `(1 ± e0123)/2` into two quaternion factors (15.3); the
//! three complementary plane pairs of 1.2 are the three axes of each
//! factor, so a rotor is a product of three pair blocks, read in the fixed
//! order by an exact Tait–Bryan peel of each factor, and every pair block
//! is two commuting plane rotors, each spelled by its plane's Gaussian
//! peel. What a step cannot read exactly — an irrational half-angle, a
//! prime outside a plane's letters — is a demand, never a chain.

use crate::transducer::alphabet::{rational_sqrt, rotor_block_on, spell_peel_on, LetterId, Rotor, Word, Q};
use crate::transducer::peel::PeelNest;
use crate::transducer::sentence::Demand;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

/// The order of 50.4: `(Π_01 × Π_23) → (Π_02 × Π_13) → (Π_03 × Π_12)`.
pub const BLOCK_ORDER: [((u8, u8), (u8, u8)); 3] = [((0, 1), (2, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2))];

/// Spell `r` in the letters by the ordered peel. `None` when `r` is not a
/// versor (never a vertex, 16.1); `Err` with the demand of 50.5 when a
/// pair block or a plane does not spell in this order; `Ok` with the word
/// whose rotor is projectively `r`.
pub fn factor_rotor(r: &Rotor) -> Option<Result<Word, Demand>> {
    r.weight()?;
    // EVEN, OR NOT A ROTOR (Expansion 3, 16.1). The quaternion factors
    // below read the even grades and nothing else; an element with an odd
    // grade — a reflection has weight too — would be factored by what it
    // carries on grades 0, 2 and 4 and spelled as the rotation it is not.
    // It is no element of `Spin(4,0)` and never a vertex: refused, not
    // truncated.
    if !r.is_even() {
        return None;
    }
    let (qp, qm) = factors(r);
    let (Some(tp), Some(tm)) = (tait_bryan(&qp), tait_bryan(&qm)) else {
        return Some(Err(standing(r, BLOCK_ORDER[0].0)?));
    };
    let mut word = Word::identity();
    for (k, (pa, pb)) in BLOCK_ORDER.iter().enumerate() {
        let Some((ra, rb)) = pair_rotors(k, &tp[k], &tm[k]) else {
            return Some(Err(standing(r, *pa)?));
        };
        for (plane, rotor) in [(*pa, ra), (*pb, rb)] {
            match spell_plane(plane, &rotor, r)? {
                Err(demand) => return Some(Err(demand)),
                Ok(None) => {}
                Ok(Some(w)) => word = word.compose(&w),
            }
        }
    }
    // The word re-derives the rotor, or it spelled nothing (A12 step 4).
    if !word.rotor().projective_eq(r) {
        return Some(Err(standing(r, BLOCK_ORDER[0].0)?));
    }
    // THE SPELLING IS THE GEODESIC (Expansion 1). The peel writes the
    // plane rotors in block order, which is a spelling and not the
    // shortest one: the presentation's relators — the commutators of
    // complementary planes, the sheet — shorten it, and this is the one
    // construction every spelling the alphabet produces passes through.
    Some(Ok(word.geodesic()))
}

pub(crate) type Quat = [Q; 4];

/// A rotation about one axis of a quaternion factor, read projectively as
/// the cosine and sine of its half-angle: `c + s·f_k` up to scale.
pub(crate) struct HalfAngle {
    c: Q,
    s: Q,
}

/// The two quaternion factors of an even element: `R E±` in the bases
/// `(E±, E± e01, E± e02, E± e03)`, the minus factor's third axis negated so
/// both are right-handed (`i j = k`).
pub(crate) fn factors(r: &Rotor) -> (Quat, Quat) {
    // THE GRADES, EACH THROUGH ITS OWN PROJECTOR (Expansion 3): `⟨r⟩₀` is
    // the scalar, `⟨r⟩₂` the six plane bivectors, `⟨r⟩₄` the 4-volume the
    // central idempotents split on. The coefficients are the same numbers
    // read off `coeff` directly; reading them through the graded
    // decomposition makes this the one consumer of that decomposition
    // rather than a second reader of the same blades (law 12).
    let (g0, g2) = (r.grade(0), r.grade(2));
    let c = |m: usize| g2.coeff(m).clone();
    let (a, p) = (g0.coeff(0).clone(), r.pseudoscalar().clone());
    let (b01, b02, b03, b12, b13, b23) = (c(3), c(5), c(9), c(6), c(10), c(12));
    let qp = [&a + &p, &b01 - &b23, &b02 + &b13, &b03 - &b12];
    let qm = [&a - &p, &b01 + &b23, &b02 - &b13, -(&b03 + &b12)];
    (qp, qm)
}

/// The exact Tait–Bryan peel of one factor: `q = q1 · q2 · q3`, each `q_k`
/// a rotation about axis `k`, read projectively as `(cos : sin)` of its
/// half-angle. The third step needs one rational square root — the
/// half-angle of a rational rotation is rational or it is not — and the
/// others none; `None` when the root is irrational or the factor is zero.
pub(crate) fn tait_bryan(q: &Quat) -> Option<[HalfAngle; 3]> {
    let two = Q::from_integer(BigInt::from(2));
    let [w, x, y, z] = q.clone();
    // Axis 3: after removing q3 the rest is q1·q2, whose coordinates
    // satisfy w·z = x·y; the angle that makes it so is read exactly.
    let a = &w * &z - &x * &y;
    let b = (&w * &w + &x * &x - &y * &y - &z * &z) / &two;
    let (c3, s3) = if a.is_zero() {
        (Q::one(), Q::zero())
    } else {
        let root = rational_sqrt(&(&a * &a + &b * &b))?;
        (root + b, a)
    };
    let (w, x, y, z) = (
        &w * &c3 + &z * &s3,
        &x * &c3 - &y * &s3,
        &y * &c3 + &x * &s3,
        &z * &c3 - &w * &s3,
    );
    // Axis 2: the rest is q1, with no second or third component.
    let (c2, s2) = if w.is_zero() && y.is_zero() { (Q::one(), Q::zero()) } else { (w.clone(), y.clone()) };
    let (w, x, y, z) = (
        &w * &c2 + &y * &s2,
        &x * &c2 + &z * &s2,
        &y * &c2 - &w * &s2,
        &z * &c2 - &x * &s2,
    );
    if !y.is_zero() || !z.is_zero() || (w.is_zero() && x.is_zero()) {
        return None;
    }
    Some([HalfAngle { c: w, s: x }, HalfAngle { c: c2, s: s2 }, HalfAngle { c: c3, s: s3 }])
}

/// The pair block `P_k` from the two factors' half-angles on axis `k`, as
/// the two commuting plane rotors it is the product of. The factors' scales
/// are matched by the square root of their norms' ratio; `None` when that
/// root is irrational — the block has no rational plane rotors.
pub(crate) fn pair_rotors(k: usize, plus: &HalfAngle, minus: &HalfAngle) -> Option<(Rotor, Rotor)> {
    let (cp, sp) = (&plus.c, &plus.s);
    let (cm, sm) = (&minus.c, &minus.s);
    let np2 = cp * cp + sp * sp;
    let nm2 = cm * cm + sm * sm;
    if np2.is_zero() || nm2.is_zero() {
        return None;
    }
    let lambda = rational_sqrt(&(&nm2 / &np2))?;
    let two = Q::from_integer(BigInt::from(2));
    let lcp = &lambda * cp;
    let lsp = &lambda * sp;
    let scalar = (&lcp + cm) / &two;
    let pseudo = (&lcp - cm) / &two;
    // The bivector halves: f+ = (e_a ∓ e_b)/2 and f− = ±(e_a ± e_b)/2 per
    // pair, and e_a · e_b = σ · e0123.
    let (ea, eb, sigma) = match k {
        0 => ((&lsp + sm) / &two, (-&lsp + sm) / &two, Q::one()),
        1 => ((&lsp + sm) / &two, (&lsp - sm) / &two, -Q::one()),
        _ => ((&lsp - sm) / &two, (-&lsp - sm) / &two, Q::one()),
    };
    // P = cα cβ + sα cβ e_a + cα sβ e_b + σ sα sβ e0123.
    let (ca, sa) = if !scalar.is_zero() || !ea.is_zero() { (scalar.clone(), ea.clone()) } else { (eb.clone(), &sigma * &pseudo) };
    let (cb, sb) = if !scalar.is_zero() || !eb.is_zero() { (scalar.clone(), eb.clone()) } else { (ea.clone(), &sigma * &pseudo) };
    let (pa, pb) = BLOCK_ORDER[k];
    let mask = |p: (u8, u8)| (1usize << p.0) | (1usize << p.1);
    let ra = Rotor::scalar(ca).add(&Rotor::blade(mask(pa), sa));
    let rb = Rotor::scalar(cb).add(&Rotor::blade(mask(pb), sb));
    let block = Rotor::scalar(scalar)
        .add(&Rotor::blade(mask(pa), ea))
        .add(&Rotor::blade(mask(pb), eb))
        .add(&Rotor::blade(15, pseudo));
    if !ra.geo(&rb).projective_eq(&block) {
        return None;
    }
    Some((ra, rb))
}

/// One plane rotor spelled by its plane's peel. `Ok(None)` for the
/// identity; `Err` the demand the plane leaves, carrying `target` — the
/// rotor being factored — as the witness a standing block cannot be on
/// its own (16.16 step 6); `None` when the rotor is not a versor of that
/// plane.
fn spell_plane(plane: (u8, u8), rotor: &Rotor, target: &Rotor) -> Option<Result<Option<Word>, Demand>> {
    let block = rotor_block_on(rotor, plane)?;
    if is_identity_block(&block) {
        return Some(Ok(None));
    }
    Some(match spell_peel_on(plane, &block) {
        Some((PeelNest::Closed { cores }, rho)) if rho.is_one() => {
            let w = crate::transducer::alphabet::word_of_cores(&cores)?;
            if w.block_on(plane).as_ref() != Some(&block) {
                Err(Demand::Standing { block: Box::new(block), rotor: Box::new(target.clone()) })
            } else {
                Ok(Some(w))
            }
        }
        Some((PeelNest::Closed { .. }, _)) => {
            Err(Demand::Standing { block: Box::new(block), rotor: Box::new(target.clone()) })
        }
        Some((PeelNest::Standing { arc, .. }, _)) => {
            let norm: BigInt = &arc.p * &arc.p + &arc.q * &arc.q;
            match prime_outside_the_alphabet(&norm, plane) {
                // 37.7 P2: the witness is the block left AFTER the base
                // shells — `R = s + t·e_ij` written out — never the block
                // the peel started from. The court reads `π_p` off it by
                // Euclid over ℤ[i]; a witness whose gcd with `p` is a
                // unit names a prime it does not carry and is refused.
                Some(prime) => Err(Demand::RotorPrime {
                    plane,
                    prime,
                    witness: Box::new(arc_block(&arc.p, &arc.q)),
                }),
                None => Err(Demand::Standing { block: Box::new(block), rotor: Box::new(target.clone()) }),
            }
        }
        None => Err(Demand::Standing { block: Box::new(block), rotor: Box::new(target.clone()) }),
    })
}

/// The Gaussian arc `s + t·i` written out as the plane block it is:
/// `[[s, −t], [t, s]]`, the rotation the residue still stands on (37.7
/// P2). Exact over ℚ, nothing normalised away.
pub(crate) fn arc_block(s: &BigInt, t: &BigInt) -> [[Q; 2]; 2] {
    let (s, t) = (Q::from_integer(s.clone()), Q::from_integer(t.clone()));
    [[s.clone(), -t.clone()], [t, s]]
}

/// The standing reading of a rotor that did not decompose: its own action
/// on `plane`, the in-plane components of the plane's axes' images.
pub(crate) fn standing(r: &Rotor, plane: (u8, u8)) -> Option<Demand> {
    let (i, j) = (plane.0 as usize, plane.1 as usize);
    let unit = |k: usize| -> [Q; 4] { std::array::from_fn(|t| if t == k { Q::one() } else { Q::zero() }) };
    let a = r.carry(&unit(i))?;
    let b = r.carry(&unit(j))?;
    Some(Demand::Standing {
        block: Box::new([[a[i].clone(), b[i].clone()], [a[j].clone(), b[j].clone()]]),
        rotor: Box::new(r.clone()),
    })
}

pub(crate) fn is_identity_block(b: &[[Q; 2]; 2]) -> bool {
    b[0][0].is_one() && b[1][1].is_one() && b[0][1].is_zero() && b[1][0].is_zero()
}

/// The least prime factor of a Gaussian norm after the plane's own primes
/// leave it — 5 on every plane, 13 where the (5,12,13) letter exists — or
/// `None` when only units or half letters remain (a standing block, not a
/// missing prime).
pub fn prime_outside_the_alphabet(norm: &BigInt, plane: (u8, u8)) -> Option<u64> {
    let mut n = norm.clone();
    let mut own = vec![5u64];
    if LetterId::pythagorean_13(plane).is_some() {
        own.push(13);
    }
    for p in own {
        let bp = BigInt::from(p);
        loop {
            let (d, m) = n.div_rem(&bp);
            if !m.is_zero() {
                break;
            }
            n = d;
        }
    }
    if n <= BigInt::one() {
        return None;
    }
    // Trial division on an exact integer: the norm of one block's arc,
    // bounded by the block's own denominators, never a search over anything
    // the sensor did not present.
    let mut p = BigInt::from(2u64);
    while &p * &p <= n {
        if n.is_multiple_of(&p) {
            return u64::try_from(p).ok();
        }
        p += BigInt::one();
    }
    u64::try_from(n).ok()
}
