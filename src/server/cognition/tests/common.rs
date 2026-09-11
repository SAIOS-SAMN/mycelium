// Fixture constructors for the mind's tests (MU-STANDARDS 3.3): a voice's
// sentence for a pair, a glued pair, and a grid of digits, each built
// through the constructors the production code exports.

use crate::cognition::*;
use crate::transducer::element::Element;
use crate::transducer::family::TrainPair;

/// A pair of cell scenes as the one-pair family a voice reads it as: each
/// scene on its own grid, exactly as the eyes of the voice would (20).
pub fn family_of_pair(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> Vec<TrainPair> {
    use crate::transducer::family::cells_to_grid;
    vec![(cells_to_grid(a), cells_to_grid(b))]
}

/// THE VOICE'S F, IN-PROCESS (50.2, law 8): the rule the form engine
/// extracts over an empty genome and the sentence it mints for the pair —
/// with its demand vertices, uttered whole (50.5). `None` where the eyes
/// admit no body pair: a form with no vertex that acts is a chain (50.6),
/// and a voice speaks nothing for it.
pub fn sentence_of(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> Option<Element> {
    use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};
    use crate::transducer::sentence::Vertex;
    let family = family_of_pair(a, b);
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20))?;
    let acts = rule.form.vertices.iter().any(|v| {
        matches!(v, Vertex::Word { .. } | Vertex::Glued { .. } | Vertex::Rebased { .. } | Vertex::Unfolded { .. })
    });
    if !acts {
        return None;
    }
    mint_sentence(&rule, &family, &NoCrystal)
}

/// A VOICE SPEAKS A PAIR TO THE COURT through the one door an utterance has
/// (law 8, ruled 2026-09-10): the voice's sentence for the pair, bound to a
/// task of that family (the voice's own binding, as the task arm binds it),
/// heard by `perceive_sentence`. `Outcome::Pivot` where the voice has no
/// sentence to speak. No cell of the pair reaches the court.
pub fn speak_pair(k: &mut Kernel, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: [u8; 32]) -> Outcome {
    let Some(e) = sentence_of(a, b) else {
        return Outcome::Pivot;
    };
    let family = family_of_pair(a, b);
    let task = crate::registry::task_id(&crate::arc::Task { train: family.clone(), test: Vec::new() });
    k.bind_task_family(&task, family);
    k.perceive_sentence(voice, task.as_bytes(), &e)
}

/// A sentence with its word vertices STRUCK OUT: a tape of the Frame and
/// its demands alone, which is a chain and not a sentence (50.6), refused
/// by the blind replay as `REFUSE_NOT_A_SENTENCE`. The tamper a forged word
/// carries where a chain-era fixture inflated a magnitude.
pub fn strip_words(spoken: &Element) -> Element {
    use crate::transducer::sentence::{as_element_form, Vertex};
    let mut f = spoken.form().cloned().expect("a sentence tape reads back");
    f.vertices.retain(|v| !matches!(v, Vertex::Word { .. }));
    f.reindex();
    as_element_form(&f, spoken.magnitude()).expect("a tape")
}

/// Two voices speak a pair and `glue_local` glues it: a basis reached
/// through the fixture's two wire identifiers (§3.4's exception), so a test
/// built on this proves nothing about 47.3's witness and says so.
pub fn glue_pair(k: &mut Kernel, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) {
speak_pair(k, a, b, [1; 32]);
let _ = speak_pair(k, a, b, [2; 32]);
k.glue_local();
}

/// 47.5's Λ on a mesh whose measured round trip to every chart's home is
/// one stroke: `from_transport(1, aperture_cycle())`, whatever chart,
/// magnitude and room it is asked with. A test that lapses states the
/// transport it lapses against, because the kernel names no constant and
/// does not lapse against an unmeasured one.
pub fn one_stroke_transport(
    _chart: &crate::transducer::stalk::Invariant,
    _mag: &[u64; 4],
    _room: &[u64; 4],
) -> Option<u64> {
    Some(crate::physics::from_transport(1, crate::physics::aperture_cycle() as u64))
}

/// A home this node has measured no round trip to: nothing lapses.
pub fn unmeasured_transport(
    _chart: &crate::transducer::stalk::Invariant,
    _mag: &[u64; 4],
    _room: &[u64; 4],
) -> Option<u64> {
    None
}

/// A hypha's signing key from a name, for a receipt fixture that must be
/// SIGNED: 47.2 step 2 is a value (`Verified`), and a test reaches the
/// chain checks only through it. The seed is a KDF over the name, which is
/// what a fixture wants and is not an identity (46).
pub fn court_key(name: &str) -> crate::court::identity::HyphaKey {
    use ed25519_dalek::SigningKey;
    let seed = crate::court::identity::HyphaKey::seed_from_name(name);
    let sk = SigningKey::from_bytes(&seed);
    crate::court::identity::HyphaKey { vk: sk.verifying_key(), sk }
}

/// The prefix signed by `key` and verified under its own vk: the one way a
/// test hands `admit_receipt` step 2.
pub fn verified<'a>(
    prefix: &'a crate::court::admission::SignedPrefix,
    key: &crate::court::identity::HyphaKey,
) -> crate::court::admission::Verified<'a> {
    let sig = key.sign(&prefix.bytes());
    crate::court::admission::verify_signature(prefix, &key.vk_bytes(), &sig)
        .expect("a key verifies its own signature over its own prefix")
}

pub fn digits(rows: &[&str]) -> Vec<Vec<u8>> {
rows.iter()
    .map(|r| r.bytes().map(|b| b - b'0').collect())
    .collect()
}
