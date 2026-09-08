// 42 — BLIND REPLAY: the same re-derivation with nothing supplied but the
// word itself.
//
//! A court that must resolve a citation to replay a word cannot replay a
//! word it has never seen. Blind replay is what an organ runs on arrival,
//! and what it refuses on is the same list `replay.rs` refuses on.

use super::*;

/// `blind_replay` with no cover behind it: a crystallized form that cites
/// another refuses as unruled, because nothing here can resolve the
/// citation. Hyphae replay through `blind_replay_with` and their ruled
/// cover.
pub fn blind_replay(spoken: &Element) -> Option<&'static str> {
    blind_replay_with(spoken, &|_| None)
}

/// `blind_replay_with` plus the bound family for chapter 23.1 step 6.
pub fn blind_replay_on(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
) -> Option<&'static str> {
    blind_replay_on_inner(spoken, resolve, family, crystal)
}

/// Deterministic re-derivation of a spoken word from its own evidence
/// (the contradiction docket). Returns None when the word replays clean;
/// Some(reason) when the speech was false. A residual word re-derives from
/// its chain (from_residual is deterministic, the chain canonically
/// framed); a CONNECTION re-derives by inversion of its group component,
/// with its remainder chain re-deriving as any germ's; every torsion cycle
/// is walked; and the install must strictly descend (0⃗ ≺_DM μ⃗).
pub fn blind_replay_with(spoken: &Element, resolve: &dyn Fn(&[u8]) -> Option<Form>) -> Option<&'static str> {
    blind_replay_on_inner(spoken, resolve, None, &crate::transducer::family::NoCrystal)
}

pub(crate) fn blind_replay_on_inner(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
) -> Option<&'static str> {
    let chain = spoken.sub();
    // A CONNECTION is a word with two components: the group component (an
    // invertible motion and gauge on the spatial fibre) and the remainder
    // chain. Its chain part re-derives like any germ's; its group part is
    // verified by inversion — a connection the court cannot invert cannot
    // glue and is false speech. Its magnitude carries the volume it speaks
    // across its family on top of the chain's own measure, so it is bounded
    // below by the chain, never equal to it.
    let connection = spoken.is_connection();
    // THE ALPHABET IS THE LAW OF SPEECH: a connection is a word only when
    // its motion is spelled in the seventeen letters and the tape
    // re-derives the block the fibre holds. A tape that disagrees with its
    // fibre is false speech; a connection without a tape was never a word.
    if connection {
        match spoken.letters_rederive() {
            None => return Some("connection has no letters — not a word of the alphabet"),
            Some(false) => return Some("motion does not re-derive from its letters"),
            Some(true) => {}
        }
    }
    if spoken.is_rewrite() {
        return crate::transducer::rewrite::replay(spoken);
    }
    if spoken.is_sentence() {
        // The version byte selects the replayer: a v3 tape is a form and
        // is replayed through the cover; a v2 tape is refused.
        if let Some(reason) = replay_form_with(spoken, resolve, family, crystal) {
            return Some(reason);
        }
        // The sentence MEASURES something. Written as a descent from the
        // zero vector, this read as a lane comparison and was a
        // nonzero test wearing one: `0⃗ ≺ m` is true exactly when some
        // lane of `m` is above zero.
        if spoken.magnitude().iter().all(|&m| m == 0) {
            return Some(REFUSE_NO_DESCENT);
        }
        return None;
    }
    if spoken.is_translation() {
        match spoken.letters_rederive() {
            None => return Some("translation has no letters — not a word of the alphabet"),
            Some(false) => return Some("translation does not re-derive from its letters"),
            Some(true) => {}
        }
        if spoken.solution().is_none() {
            return Some("translation is not invertible");
        }
        let Some((a, b)) = spoken.translation_sections() else {
            return Some("translation does not name two sections");
        };
        if a == b {
            return Some("translation sections coincide");
        }
        // Apply the gauge to the first section (the identity frame of the
        // tape): the letters' affine is the second section. letters_rederive
        // is that replay.
        return None;
    }
    match (chain.is_empty(), connection) {
        (true, false) => {
            // An OPENED AXIS with no chain: the germinated fibres must be
            // group elements whose planar block spells, and the word must
            // stand on a spelled base motion.
            let axes = spoken.opened_axes();
            if axes.is_empty() {
                return Some("empty chain — nothing was derived");
            }
            if spoken.solution().is_none() {
                return Some("opened axis is not invertible");
            }
            for (_, v) in spoken.germinated() {
                if v.versor_inverse().is_none() {
                    return Some("opened axis is not invertible");
                }
            }
        }
        (true, true) => {
            if spoken.solution().is_none() {
                return Some("connection is not invertible");
            }
        }
        (false, _) => {
            let min_r = chain.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
            let min_c = chain.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
            if (min_r, min_c) != (0, 0) {
                return Some("chain not framed to origin");
            }
            if !chain.windows(2).all(|w| w[0] <= w[1]) {
                return Some("chain not canonically ordered");
            }
            let rederived = element::from_residual(&chain.to_vec());
            match connection {
                // The chain lanes (spatial, gauge, shell) re-derive from the
                // chain; the fibre lane is the word's own sections and cycles,
                // which the court counts on the word itself.
                false => {
                    if rederived.address() != spoken.address() {
                        return Some("address does not re-derive from the chain");
                    }
                    // One vector equality on the lanes the chain names.
                    // Sections and torsion a held word gained after it was
                    // ruled (`attach`, `merge_fibre`) live on the word and
                    // are counted by `magnitude`, never re-derived from the
                    // chain; reading them here slashed every deepened word
                    // a peer served.
                    if rederived.base_magnitude() != spoken.chain_magnitude() {
                        return Some("magnitude does not re-derive from the chain");
                    }
                }
                true => {
                    let spoken_c = element::canonical_chain(chain).unwrap_or_else(|| chain.to_vec());
                    if rederived.sub() != spoken_c.as_slice() {
                        return Some("remainder does not re-derive from the chain");
                    }
                    // The connection carries at least its remainder's own
                    // measure: equal, or the remainder strictly descends
                    // from it on the lanes the chain names. Dominance is the
                    // one lane comparison (law 5); no lane is visited by index.
                    let remainder = rederived.base_magnitude();
                    let carried = spoken.chain_magnitude();
                    if !crate::economy::lane_moves(&remainder, &carried).rose().is_empty() {
                        return Some("magnitude below the remainder's own measure");
                    }
                    if spoken.solution().is_none() {
                        return Some("connection is not invertible");
                    }
                }
            }
        }
    }
    if spoken.magnitude().iter().all(|&m| m == 0) {
        return Some("the germ measures nothing on any lane");
    }
    witness_torsion(spoken)
}
