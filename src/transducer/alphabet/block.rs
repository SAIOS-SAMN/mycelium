// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 37.7 P2, A3 — THE PLANE BLOCK AS A GAUSSIAN INTEGER, and the rotor a
//! block is the action of.
//!
//! A rotation block over ℚ is a Gaussian integer up to a rational scale,
//! and a rotor is that integer's square root, because a rotor is a
//! half-angle. Both readings are exact or refused; nothing is rounded and
//! nothing is searched for.
//!
//! Moved here from `family/ring.rs`, unchanged (§1.6). They were private
//! to the ring branch, which made them unreachable from the two other
//! places that need the same arithmetic: the versor readers in `element`,
//! which classify a meet's transition by its rotor part, and the court's
//! complex, which re-derives that classification. Arithmetic on a rotor's
//! block belongs beside the rotor, not inside one caller.

use super::{SpellCore, Word};
use crate::core::gaussian::Gauss;
use crate::core::Q;

/// A plane block `[[s, −t], [t, s]]` as the Gaussian integer `s + t·i`,
/// with the common denominator cleared EXACTLY. A rotation block over ℚ
/// is `π/ρ` for a Gaussian `π`; clearing the denominator recovers `π` up
/// to a rational scale, and scale is what the ring divides out anyway —
/// identity is projective (1.5). `None` when the block is not a rotation
/// block, which is not an arc at all.
pub(crate) fn gauss_of_block(block: &[[Q; 2]; 2]) -> Option<Gauss> {
    use num_integer::Integer;
    let (s, t) = (&block[0][0], &block[1][0]);
    if &block[1][1] != s || block[0][1] != -t.clone() {
        return None;
    }
    let d = s.denom().lcm(t.denom());
    let a = (s * num_rational::BigRational::from_integer(d.clone())).to_integer();
    let b = (t * num_rational::BigRational::from_integer(d)).to_integer();
    let g = Gauss::new(a, b);
    match g.is_zero() {
        true => None,
        false => Some(g),
    }
}

// `block_of_gauss` was here: the inverse reading, `s + t·i` written back
// as its plane block. Its one caller was the ring's cofactor step, which
// wrote the remainder back to a block so it could be peeled a second
// time. The peel now divides by the adjoined generators in its own pass,
// so nothing ever leaves the peel and returns to it, and there is nothing
// to write back.

// `rotor_of_block` — the rotor a plane block is the action of, its arc's
// square root in ℤ[i] — stood here with `arc_half`. A versor's rotation is
// read whole on the base's three planes by `element::base_rotor_of`
// (51.2), and a rotor recovered from one plane's block is a projection of
// it (checklist (m)); the two had no other caller.

/// THE WORD A CLOSED NEST SPELLS, or `None` if any shell was a citation.
/// A `Word` is letters: it has nowhere to put a generator the alphabet
/// does not contain, and returning the letters alone would return a
/// DIFFERENT rotation from the one that was peeled (rule 11).
///
/// The cores are outer-to-inner and the word is built inner-to-outer, so
/// the walk is reversed. This shape was written out at five call sites;
/// it is one reader now.
pub(crate) fn word_of_cores(cores: &[SpellCore]) -> Option<Word> {
    let mut w = Word::identity();
    for core in cores.iter().rev() {
        match core {
            SpellCore::Letters(syllable) => {
                for &(id, inv) in syllable {
                    w.push(id, inv);
                }
            }
            SpellCore::Cited { .. } => return None,
        }
    }
    Some(w)
}

/// THE CITATIONS AND THE RESIDUE a closed nest carries (50.2 step 2): the
/// adjoined shells with their exponents summed per generator, and the
/// letters that remain as a word. This is a `Derivation`'s two halves.
pub(crate) fn cited_of_cores(cores: &[SpellCore]) -> Option<(Vec<super::Cited>, Word)> {
    use crate::network::nerve::Keyed;
    let mut w = Word::identity();
    let mut by_gen: Keyed<super::Cited> = Keyed::new();
    for core in cores.iter().rev() {
        match core {
            SpellCore::Letters(syllable) => {
                for &(id, inv) in syllable {
                    w.push(id, inv);
                }
            }
            SpellCore::Cited { form, vertex, exponent } => {
                // Keyed by the citation's own two addresses written out, so
                // the shells of one generator sum into one exponent and the
                // order is the key's and not the peel's arrival (rule 15).
                let mut k = Vec::with_capacity(form.as_bytes().len() + vertex.as_bytes().len() + 8);
                crate::transducer::address::put_addr(&mut k, form);
                crate::transducer::address::put_addr(&mut k, vertex);
                match by_gen.get_mut(&k) {
                    Some((_, _, e)) => *e += exponent,
                    None => {
                        by_gen.insert(k, (form.clone(), vertex.clone(), *exponent));
                    }
                }
            }
        }
    }
    // The generators this word cites, accumulated here by (form, vertex):
    // the whole of that index is the citation list, and a zero exponent
    // drops out of it.
    let cited: Vec<_> = by_gen
        .scoped()
        .map(|(_, v)| v.clone())
        .filter(|(_, _, e)| *e != 0)
        .collect();
    Some((cited, w))
}

/// THE WORD A PLANE BLOCK SPELLS, where A22's peel closes on it: the
/// cores in reverse, which is the one shape three callers had each written
/// out for themselves (`peel_versor`, the ring's cofactor, and the cover's
/// deleted `rotor_of_transition`). `None` where the peel stands, which is
/// a block the seventeen do not reach — a refusal, never a shorter word.
pub(crate) fn word_of_block(block: &[[Q; 2]; 2], plane: (u8, u8)) -> Option<super::Word> {
    match super::spell_peel_on(plane, block)? {
        (crate::transducer::peel::PeelNest::Closed { cores }, _) => super::word_of_cores(&cores),
        _ => None,
    }
}

/// The word a ROTOR spells on the sensor plane, through its own block.
/// `None` where the seventeen do not reach it, which is exactly 37.7's
/// condition and not an error.
pub(crate) fn word_of_rotor(rotor: &super::Rotor) -> Option<super::Word> {
    let plane = (super::PLANE_ROW, super::PLANE_COL);
    word_of_block(&super::rotor_block_on(rotor, plane)?, plane)
}

