// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A12, M6 — COMPREHENSION BY FACTORISATION: an incoming word is understood
//! iff it factors through the words the mind already holds, and when it does
//! not, the remainder it leaves is exact — the demand, with no choice in it.

use super::alphabet::{PlaneForm, Rotor, Word, CAYLEY_ALPHABET_LEN};

/// What a mind concludes about an incoming word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Factored {
    /// The word is a product of held words: the indices into the basis, each
    /// with the sign of its exponent, in application order. An empty tape
    /// means the identity, which every mind already speaks.
    Understood(Vec<(usize, i64)>),
    /// The word is not in the span of what is held. `remainder` is the exact
    /// element still missing — the canonical HNF residue on the plane, the
    /// whole word off it — and `understood` is the part that did factor.
    Novel { remainder: PlaneForm, understood: Vec<(usize, i64)> },
    /// Off the plane, with no plane form to reduce and the budget spent
    /// before the traversal reached the target: unmeasured, never "novel".
    /// A refusal is not a demand (rule 11).
    Unmeasured,
}

/// One row of the even lattice, and what it was made of. A generator is
/// either one basis word with `s = 0`, or the DIFFERENCE of two words with
/// `s = 1` — since two odd elements compose to an even one — or the
/// relator `4·e_u`, which belongs to no word and is spent by nobody.
#[derive(Clone, Debug)]
enum Gen {
    One(usize),
    Diff(usize, usize),
    Relator,
}

/// The even sublattice generators of a basis, per 16.12 step 1.
fn generators(basis: &[PlaneForm]) -> (Vec<[i64; CAYLEY_ALPHABET_LEN]>, Vec<Gen>) {
    let mut rows = Vec::new();
    let mut from = Vec::new();
    let odd: Vec<usize> = (0..basis.len()).filter(|&i| basis[i].orientation).collect();
    for (i, b) in basis.iter().enumerate() {
        if !b.orientation {
            rows.push(b.rot);
            from.push(Gen::One(i));
        }
    }
    // Differences of the odd words: the odd set's own members, not a
    // product over the basis — every pair here shares the one property
    // that makes it a generator, and the read is over that set alone.
    for (a, &i) in odd.iter().enumerate() {
        for &j in odd.iter().skip(a + 1) {
            let r: [i64; CAYLEY_ALPHABET_LEN] =
                std::array::from_fn(|k| basis[i].rot[k] - basis[j].rot[k]);
            rows.push(r);
            from.push(Gen::Diff(i, j));
        }
    }
    // 4·e_u for each unit turn: a 90° turn has order four (1.2), so the
    // relator is part of the lattice and is not anyone's word.
    // The six 90° turns, slots 1..=6 (MU-ALPHABET.md): slot 0 is the
    // identity, 7..=16 are the primes, which have infinite order and carry
    // no relator (1.2).
    for u in 1..=6usize {
        let mut r = [0i64; CAYLEY_ALPHABET_LEN];
        r[u] = 4;
        rows.push(r);
        from.push(Gen::Relator);
    }
    (rows, from)
}

/// The tape a set of generator exponents names, in basis indices. A
/// relator contributes nothing to spend: it is a fact about the alphabet,
/// not a word anyone holds. The lattice is exact at any size (STD1); the
/// tape writes an exponent as `i64`, and one past it is refused (`None`) —
/// never truncated, as the `as` cast that stood here silently did.
fn tape_of(coeffs: &[num_bigint::BigInt], from: &[Gen]) -> Option<Vec<(usize, i64)>> {
    use num_traits::Zero;
    let mut out = Vec::new();
    for (c, g) in coeffs.iter().zip(from) {
        if c.is_zero() {
            continue;
        }
        let e = i64::try_from(c).ok()?;
        match g {
            Gen::One(i) => out.push((*i, e)),
            Gen::Diff(i, j) => {
                out.push((*i, e));
                out.push((*j, e.checked_neg()?));
            }
            Gen::Relator => {}
        }
    }
    Some(out)
}

/// A12 on the sensor plane. `g` and every basis word have plane forms
/// (A3-reduced); membership in the even lattice is decided exactly, and the
/// residue of a failure is canonical.
pub fn factor_on_plane(g: &PlaneForm, basis: &[PlaneForm]) -> Factored {
    let (rows, from) = generators(basis);
    let (h, u) = crate::core::exponents::hnf_with_transform(&rows);
    // s = 1: the target is odd, so one odd basis word is divided out first
    // and the rest is an even membership question. The least such index is
    // taken — an ordering over the basis, canonical on every node, never a
    // preference between candidates.
    let (target, odd_word) = if g.orientation {
        let Some(i) = (0..basis.len()).find(|&i| basis[i].orientation) else {
            return Factored::Novel { remainder: *g, understood: Vec::new() };
        };
        let r: [i64; CAYLEY_ALPHABET_LEN] =
            std::array::from_fn(|k| g.rot[k] - basis[i].rot[k]);
        (r, Some(i))
    } else {
        (g.rot, None)
    };
    let (coeffs, residue) = crate::core::exponents::reduce_against(&h, &target);
    // Pull the reduction back through U to exponents over the ORIGINAL
    // generators: H's rows are combinations of them, and that is what the
    // mind actually holds.
    use num_traits::Zero;
    let mut over_gens = vec![num_bigint::BigInt::zero(); rows.len()];
    for (i, c) in coeffs.iter().enumerate() {
        if c.is_zero() {
            continue;
        }
        for (j, x) in u[i].iter().enumerate() {
            over_gens[j] += c * x;
        }
    }
    // An exponent or a residue coordinate past the tape's `i64` is not a
    // tape this mind can write: unmeasured, by name, never truncated.
    let Some(mut tape) = tape_of(&over_gens, &from) else {
        return Factored::Unmeasured;
    };
    if let Some(i) = odd_word {
        tape.insert(0, (i, 1));
    }
    if residue.iter().all(|x| x.is_zero()) {
        return Factored::Understood(tape);
    }
    // The residue is even by construction: the odd part was divided out.
    let rot: Option<Vec<i64>> = residue.iter().map(|x| i64::try_from(x).ok()).collect();
    let Some(Ok(rot)) = rot.map(<[i64; CAYLEY_ALPHABET_LEN]>::try_from) else {
        return Factored::Unmeasured;
    };
    let rem = PlaneForm { rot, ..Default::default() };
    Factored::Novel { remainder: rem, understood: tape }
}

/// A12 off the plane: a budgeted traversal whose states are A1 keys, applying
/// each held rotor and its inverse in index order. There is no HNF and no
/// metric to say which partial is nearer, so a failure returns the whole
/// word as the demand (16.12) — and an exhausted budget returns neither, it
/// refuses.
pub fn factor_off_plane(
    g: &Rotor,
    basis: &[Rotor],
    budget: &mut crate::physics::Budget,
) -> Factored {
    let target = g.key();
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    // (rotor, tape) frontier. The traversal is bounded by the Budget alone.
    let mut frontier: Vec<(Rotor, Vec<(usize, i64)>)> = vec![(Rotor::identity(), Vec::new())];
    seen.insert(Rotor::identity().key(), ());
    while budget.spend() {
        let Some((cur, tape)) = frontier.pop() else {
            // The traversal closed without reaching it: everything reachable
            // from what is held has been visited, so the word is genuinely
            // outside the span rather than merely unreached.
            return Factored::Novel { remainder: PlaneForm::default(), understood: Vec::new() };
        };
        if cur.key() == target {
            return Factored::Understood(tape);
        }
        for (i, b) in basis.iter().enumerate() {
            for e in [1i64, -1] {
                // The inverse letter is the reverse, a ray (78.32(1)).
                let step = if e == 1 { b.clone() } else {
                    match b.is_versor() { true => b.reverse(), false => continue }
                };
                let next = cur.geo(&step);
                let k = next.key();
                if seen.contains(k.as_bytes()) {
                    continue;
                }
                seen.insert(k.as_bytes(), ());
                let mut t = tape.clone();
                t.push((i, e));
                frontier.push((next, t));
            }
        }
    }
    Factored::Unmeasured
}

/// Verify a factorisation by re-derivation: multiply the tape's rotors and
/// compare by A1. "A mind recognises what it can re-derive, and nothing
/// else" — so the answer is checked, never trusted from the algebra that
/// produced it (16.12 step 4).
// 50.2 step 2 as ruled 2026-09-10: `replays_with_residue` was here, the
// check a `Path` beside the tape needed. `Derivation::verify_with` is the
// re-derivation now, and it lives with the carrier it checks rather than
// beside the algebra that produced it.
pub fn replays(tape: &[(usize, i64)], basis: &[Rotor], g: &Rotor) -> bool {
    let mut p = Rotor::identity();
    for (i, e) in tape {
        let Some(b) = basis.get(*i) else { return false };
        let step = if *e >= 0 {
            b.clone()
        } else {
            match b.is_versor() {
                true => b.reverse(),
                false => return false,
            }
        };
        for _ in 0..e.unsigned_abs() {
            p = p.geo(&step);
        }
    }
    p.projective_eq(g)
}

/// The word a factorisation spells, for the expression the mind records.
pub fn word_of(tape: &[(usize, i64)], basis: &[Word]) -> Option<Word> {
    let mut w = Word::identity();
    for (i, e) in tape {
        let b = basis.get(*i)?;
        let step = if *e >= 0 { b.clone() } else { b.inverse() };
        for _ in 0..e.unsigned_abs() {
            w = w.compose(&step);
        }
    }
    Some(w.normal_form())
}
