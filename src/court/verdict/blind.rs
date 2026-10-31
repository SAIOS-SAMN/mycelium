// 42 — BLIND REPLAY: the same re-derivation with nothing supplied but the
// word itself.
//
//! A court that must resolve a citation to replay a word cannot replay a
//! word it has never seen. Blind replay is what an organ runs on arrival,
//! and what it refuses on is the same list `replay.rs` refuses on.

use super::*;

/// `blind_replay` with no cover behind it: a crystallized form that cites
/// another refuses as unruled, because nothing here can resolve the
/// citation. Hyphae replay through `blind_replay_with` and each hypha's ruled
/// cover.
///
/// 78.153 — THE OFF-LOCK READERS. `blind_replay`, `blind_replay_with`,
/// `blind_replay_on` and `blind_replay_at` keep the same answer (`None` clean,
/// a refusal otherwise) for the voice and every reader outside a court's
/// lock: the replay is metered and resumable, and these run it to the end
/// over as many strokes as it takes ([`to_completion`]). A court under its
/// lock replays through [`replay_resume`], which answers `Unread` where its
/// one stroke ran out and never refuses for it.
pub fn blind_replay(spoken: &Element, budget: &mut crate::physics::Budget) -> Option<&'static str> {
    blind_replay_with(spoken, &|_| None, budget)
}

/// `blind_replay_with` plus the bound family for chapter 23.1 step 6.
pub fn blind_replay_on(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<&'static str> {
    let owned = |a: &[u8]| resolve(a).map(std::borrow::Cow::Owned);
    to_completion(spoken, budget, &mut |b, cursor| replay_word(spoken, &owned, family, crystal, b, cursor, false))
        .map(|r| r.reason)
}

/// The same replay, refusing WITH THE COORDINATE (50.9): a sentence is
/// replayed by `replay_form_at`, which names the vertex, the face or the
/// chart it refused at; every other word is refused at its tape.
pub fn blind_replay_at(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<Refusal> {
    let owned = |a: &[u8]| resolve(a).map(std::borrow::Cow::Owned);
    to_completion(spoken, budget, &mut |b, cursor| replay_word(spoken, &owned, family, crystal, b, cursor, true))
}

/// 78.153 — THE COURT'S REPLAY ON ITS ONE STROKE, resumed from `cursor`:
/// `Clean`, `Refused` with its coordinate, or `Unread` — the stroke ran out
/// first, the cursor holds where, and nothing was ruled. `at` selects
/// [`blind_replay_at`]'s reading (a sentence straight to its form replay)
/// over [`blind_replay_on`]'s (the word's own checks first).
pub fn replay_resume<'r>(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<std::borrow::Cow<'r, Form>>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    cursor: &mut ReplayCursor,
    at: bool,
) -> Replayed {
    replay_word(spoken, resolve, family, crystal, budget, cursor, at)
}

fn replay_word<'r>(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<std::borrow::Cow<'r, Form>>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    cursor: &mut ReplayCursor,
    at: bool,
) -> Replayed {
    let tape = |reason: Option<&'static str>| match reason {
        None => Replayed::Clean,
        Some(r) => Replayed::Refused(Refusal::of_tape(r, spoken)),
    };
    if at && spoken.is_sentence() && !spoken.is_rewrite() && !spoken.is_connection() {
        return replay_form_resume(spoken, resolve, family, crystal, budget, cursor);
    }
    if let Some(decided) = word_prelude(spoken) {
        return tape(decided);
    }
    if spoken.is_sentence() {
        // The version byte selects the replayer: a v3 tape is a form and
        // is replayed through the cover; a v2 tape is refused.
        //
        // S9.4 — WHAT CERTIFIES IS CLOSURE. `replay_form_resume` is the
        // certificate: the orbits return, every face re-derives, the
        // certificates are re-read, and with a family the form closes it. A
        // sentence is not refused here for carrying a zero magnitude,
        // which is not a defect in the word — μ⃗ is a COST, not a score, and
        // a word that costs nothing to say still says it. The economy
        // prices a zero deflation at zero without the court refusing it.
        return replay_form_resume(spoken, resolve, family, crystal, budget, cursor);
    }
    tape(word_rest(spoken))
}

/// 78.153 — A REPLAY RUN TO ITS END off a court's lock, on strokes of its
/// own while each one moves the cursor. The caller's stroke is left as it
/// stood: the replay drew from it only inside `versor_between` before it was
/// metered, and draining it now would starve the reads the caller makes
/// after. A stroke that moves nothing is a unit no stroke holds — or a
/// process asked to stop (`physics::halt`) — refused by name
/// (`REFUSE_UNREAD`) rather than looped on.
///
/// 78.158a U2 — THE CALLER'S STROKE IS HONOURED, NOT DRAINED. It was read
/// nowhere (`let _ = budget;`), each round a default stroke: a caller that
/// granted a family-sized stroke (`extraction_spend`) had its replay
/// refused UNREAD on a unit its own stroke would have read. Each round is
/// now as wide as the caller's stroke where it grants more than a default
/// one, and a default stroke otherwise; the caller's token is still left
/// as it stood (the reads it makes after are its own).
pub(crate) fn to_completion(
    spoken: &Element,
    budget: &mut crate::physics::Budget,
    run: &mut dyn FnMut(&mut crate::physics::Budget, &mut ReplayCursor) -> Replayed,
) -> Option<Refusal> {
    let default = crate::physics::Physics::default().stroke().remaining();
    let width = match budget.remaining() > default {
        true => budget.remaining(),
        false => default,
    };
    let mut cursor = ReplayCursor::default();
    loop {
        let before = cursor.progress();
        let mut own = crate::physics::Physics::default().stroke_n(width);
        match run(&mut own, &mut cursor) {
            Replayed::Clean => return None,
            Replayed::Refused(r) => return Some(r),
            Replayed::Unread if cursor.progress() == before => {
                return Some(Refusal::of_tape(REFUSE_UNREAD, spoken));
            }
            Replayed::Unread => {}
        }
    }
}

/// The word's own checks before a sentence is replayed: `Some(answer)` where
/// they decide it, `None` where the word goes on.
fn word_prelude(spoken: &Element) -> Option<Option<&'static str>> {
    // 78.53 — A COLOUR WORD moves no body and carries no chain: its action
    // is its fibre relation alone, and what it must be is unitary on the
    // fibre and its reservoir — a partial injection, or a merge whose
    // distinctions it records in e_∅ (`is_fibre_relation`, 78.58). Its
    // evidence against a family is re-derived
    // where it is heard (`adjudicate_one`), never replayed as a chain.
    if spoken.is_colour_word() {
        return Some(match spoken.letters().is_some_and(|l| crate::transducer::element::is_fibre_relation(&l.sigma)) {
            true => None,
            false => Some("colour word's relation is not unitary on the fibre and its reservoir"),
        });
    }
    // A CONNECTION is a word with two components: the group component (an
    // invertible motion and gauge on the spatial fibre) and the remainder
    // chain. Its chain part re-derives like any germ's; its group part is
    // verified by inversion — a connection the court cannot invert cannot
    // glue and is false speech. Its magnitude carries the volume it speaks
    // across its family on top of the chain's own measure, so it is bounded
    // below by the chain, never equal to it.
    //
    // THE ALPHABET IS THE LAW OF SPEECH: a connection is a word only when
    // its motion is spelled in the seventeen letters and the tape
    // re-derives the block the fibre holds. A tape that disagrees with its
    // fibre is false speech; a connection without a tape was never a word.
    if spoken.is_connection() {
        match spoken.letters_rederive() {
            None => return Some(Some("connection has no letters — not a word of the alphabet")),
            Some(false) => return Some(Some("motion does not re-derive from its letters")),
            Some(true) => {}
        }
    }
    if spoken.is_rewrite() {
        return Some(crate::transducer::rewrite::replay(spoken));
    }
    None
}

fn word_rest(spoken: &Element) -> Option<&'static str> {
    word_rest_checks(spoken, spoken.sub(), spoken.is_connection())
}

/// Deterministic re-derivation of a spoken word from its own evidence
/// (the contradiction docket). Returns None when the word replays clean;
/// Some(reason) when the speech was false. A residual word re-derives from
/// its chain (from_residual is deterministic, the chain canonically
/// framed); a CONNECTION re-derives by inversion of its group component,
/// with its remainder chain re-deriving as any germ's; every torsion cycle
/// is traversed; and an install measures something on some lane
/// (`settle_install`). Closure is the certificate, never a measured
/// trajectory (12.6).
pub fn blind_replay_with(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    budget: &mut crate::physics::Budget,
) -> Option<&'static str> {
    blind_replay_on(spoken, resolve, None, &crate::transducer::family::NoCrystal, budget)
}

/// The rest of a word that is not a sentence: a translation, a chain, an
/// opened axis, a connection's remainder, and its torsion witnessed.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — a chain's min corner is its extent, content: the court checks the chain is framed to its own origin"
)]
fn word_rest_checks(spoken: &Element, chain: &[(i64, i64, u8, u8)], connection: bool) -> Option<&'static str> {
    if spoken.is_translation() {
        match spoken.letters_rederive() {
            None => return Some("translation has no letters — not a word of the alphabet"),
            Some(false) => return Some("translation does not re-derive from its letters"),
            Some(true) => {}
        }
        if spoken.reverse().is_none() {
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
            if spoken.reverse().is_none() {
                return Some("opened axis is not invertible");
            }
            for (_, v) in spoken.germinated() {
                if !crate::transducer::cga::is_versor(v) {
                    return Some("opened axis is not invertible");
                }
            }
        }
        (true, true) => {
            if spoken.reverse().is_none() {
                return Some("connection is not invertible");
            }
        }
        (false, _) => {
            // The chain's min corner is its extent, content (rule 4). An
            // empty chain has no corner to be at the origin, and that is
            // not a corner of (0, 0) read for it.
            let (Some(min_r), Some(min_c)) = (
                chain.iter().map(|(r, _, _, _)| *r).min(),
                chain.iter().map(|(_, c, _, _)| *c).min(),
            ) else {
                return Some("chain is empty and frames to no origin");
            };
            if (min_r, min_c) != (0, 0) {
                return Some("chain not framed to origin");
            }
            if !chain.windows(2).all(|w| w[0] <= w[1]) {
                return Some("chain not canonically ordered");
            }
            // A chain with no A5 form re-derives nothing, and a word whose
            // chain says two things at one site is slashed on that.
            let Some(rederived) = element::from_residual(&chain.to_vec()) else {
                return Some("chain has no canonical form");
            };
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
                    // chain; reading them here would slash every deepened
                    // word a peer served.
                    if rederived.base_magnitude() != spoken.chain_magnitude() {
                        return Some("magnitude does not re-derive from the chain");
                    }
                }
                true => {
                    // The chain is the one already accepted above; A5 gives
                    // the same answer, so there is nothing to default to.
                    let Some(spoken_c) = element::canonical_chain(chain) else {
                        return Some("chain has no canonical form");
                    };
                    if rederived.sub() != spoken_c.as_slice() {
                        return Some("remainder does not re-derive from the chain");
                    }
                    // S9.4 — THE CONNECTION CARRIES AT LEAST ITS REMAINDER'S
                    // OWN MEASURE, read per lane: no lane of the remainder
                    // rises above what the chain carries. The refusal names
                    // the lane it was taken on; a `bool` here said only that
                    // some lane, somewhere, was short.
                    let remainder = rederived.base_magnitude();
                    let carried = spoken.chain_magnitude();
                    let risen = crate::economy::lanes_risen(&crate::economy::lane_moves(
                        &remainder, &carried,
                    ));
                    match risen.as_slice() {
                        [] => {}
                        [0, ..] => return Some("magnitude below the remainder's own S"),
                        [1, ..] => return Some("magnitude below the remainder's own G"),
                        [2, ..] => return Some("magnitude below the remainder's own T"),
                        _ => return Some("magnitude below the remainder's own H"),
                    }
                    if spoken.reverse().is_none() {
                        return Some("connection is not invertible");
                    }
                }
            }
        }
    }
    // S9.4 — a zero magnitude is not a refusal here either. Everything above
    // is the re-derivation: the chain canonicalises, the letters re-spell,
    // the versor inverts, the torsion witnesses below. What the word MEASURES
    // is priced by the economy after this returns, and a court that refused
    // a word for measuring nothing was ruling on its price.
    witness_torsion(spoken)
}
