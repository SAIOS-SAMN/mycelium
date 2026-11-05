// Fixture constructors for the mind's tests (MU-STANDARDS 3.3): a voice's
// sentence for a pair, a glued pair, and a sample of digits, each built
// through the constructors the production code exports.

use crate::cognition::*;
use crate::transducer::element::Element;
use crate::transducer::family::TrainPair;
pub use crate::transducer::sentence::tests::fixtures::painted;

/// A pair of painted scenes as the one-pair family a voice reads it as:
/// each scene on its own grid, exactly as the eyes of the voice would (20),
/// the fixtures' unpainted sites written `0`.
pub fn family_of_pair(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> Vec<TrainPair> {
    use crate::transducer::family::sample_of_painted;
    // The fixtures stand on that 0 (track 26 S2: declared, never deduced
    // from one pair).
    crate::transducer::raster::stand_on(0);
    vec![(sample_of_painted(&painted(a), 0).expect("a scene"), sample_of_painted(&painted(b), 0).expect("a scene"))]
}

/// THE GROUND A SPEAKER STATES FOR A FAMILY (MU-SENSING §6.2), as the mind
/// states it (`Mind::speak_with`): the scope open on the speaker's thread,
/// else its own reading of the family; one symbol under one ground, none
/// otherwise.
pub fn speaker_ground(family: &crate::transducer::family::Family) -> Option<Vec<u8>> {
    crate::transducer::raster::stated(&crate::transducer::raster::on_ground_of(family, crate::transducer::raster::ground))
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
    let acts = rule.form.vertices.entries().any(|(_, v)| {
        matches!(v, Vertex::Word { .. } | Vertex::Glued { .. } | Vertex::Rebased { .. } | Vertex::Unfolded { .. })
    });
    if !acts {
        return None;
    }
    mint_sentence(&rule, &family, &NoCrystal, 0)
}

/// A VOICE SPEAKS A PAIR TO THE COURT through the one door an utterance has
/// (law 8, ruled 2026-09-10): the voice's sentence for the pair, bound to a
/// task of that family (the voice's own binding, as the task arm binds it),
/// heard by `perceive_sentence`. `Outcome::Pivot` where the voice has no
/// sentence to speak. No painted site of the pair reaches the court.
pub fn speak_pair(k: &mut Court, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: [u8; 32]) -> Outcome {
    let Some(e) = sentence_of(a, b) else {
        return Outcome::Pivot;
    };
    let family = family_of_pair(a, b);
    let task = crate::registry::task_address(&crate::transducer::family::Task { train: family.clone(), test: Vec::new() })
        .expect("a grid pair's chain is A5's, so the task has an identity");
    k.bind_task_family(&task, family);
    k.perceive_sentence(voice, task.as_bytes(), &e)
}

/// A sentence with its word vertices STRUCK OUT: a tape of the Frame and
/// its demands alone, which is a chain and not a sentence (50.6), refused
/// by the blind replay as `REFUSE_NOT_A_SENTENCE`. The tamper a forged word
/// carries where a chain-era fixture inflated a magnitude.
pub fn strip_words(spoken: &Element) -> Element {
    use crate::transducer::sentence::{as_element_form, Vertex};
    let mut f = crate::transducer::sentence::from_element_form(spoken)
        .expect("a sentence tape reads back");
    let words: Vec<_> = f
        .vertices
        .entries()
        .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
        .map(|(k, _)| k)
        .collect();
    assert!(f.retract_vertices(&words), "the Frame and its demands keep a depth");
    f.reindex();
    as_element_form(&f, spoken.magnitude()).expect("a tape")
}

/// Two voices speak a pair and `glue_local` glues it: a basis reached
/// through the fixture's two wire identifiers (§3.4's exception), so a test
/// built on this proves nothing about 47.3's witness and says so.
pub fn glue_pair(k: &mut Court, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) {
speak_pair(k, a, b, [1; 32]);
let _ = speak_pair(k, a, b, [2; 32]);
k.glue_local();
}

/// A court that docketed one word on its own book, bound to `own_vk`.
pub fn docketed(own_vk: [u8; 32]) -> (Court, crate::transducer::address::Address) {
    let mut k = Court::new();
    k.bind_identity(own_vk);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole: one body, one word (20 as ruled, L7)
    assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
    k.court_stroke();
    let addr = k.provisional.scoped().next().expect("docketed").0;
    (k, crate::transducer::address::Address::from(addr))
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
    let sig = key.sign(&prefix.bytes().expect("the prefix frames"));
    crate::court::admission::verify_signature(prefix, &key.verifying_bytes(), &sig)
        .expect("a key verifies its own signature over its own prefix")
}

/// A PEER'S LINE ADMITTED on `k` as its receipts would arrive: each one's
/// 47.1 prefix signed by `key` and admitted under origin `([7; 32], 2)`,
/// in line order. Answers what each admission came to.
pub fn admit_line(
    k: &mut Court,
    key: &crate::court::identity::HyphaKey,
    line: &[crate::chronometry::KStepReceipt],
) -> Vec<crate::court::admission::Admitted> {
    line.iter()
        .map(|r| {
            let pre = crate::court::admission::SignedPrefix {
                genesis: [7; 32],
                epoch: 2,
                prev_receipt_hash: r.prev_hash(),
                at: r.stamp.place(),
                height: r.stamp.h,
                chart: vec![],
                payload: r.stamp.key(),
            };
            k.admit_receipt(([7; 32], 2), verified(&pre, key), &reconstructed(r))
        })
        .collect()
}

/// A receipt as admission takes it: reconstructed from its own
/// coordinates (MU-HOSTING §5a.10). A fixture's receipts are sealed, so
/// each re-derives.
pub fn reconstructed(
    r: &crate::chronometry::KStepReceipt,
) -> crate::court::admission::Reconstructed<crate::chronometry::KStepReceipt> {
    crate::court::admission::Reconstructed::receipt(r).expect("a sealed receipt re-derives")
}

/// ONE RECEIPT, read by the link sense (MU-HOSTING §5a.10): each
/// re-derives its own hash, and `a` stands at the stamp of `b` with a chain
/// continuing across the two. A receipt has no `==`.
pub fn one_receipt(a: &crate::chronometry::KStepReceipt, b: &crate::chronometry::KStepReceipt) -> bool {
    use crate::chronometry::Parent;
    a.rederives() && b.rederives() && crate::court::admission::Predecessor::cites(&Parent::of(a), &Parent::of(b))
}

/// ONE RUN OF RECEIPTS: the same count, each pair `one_receipt`.
pub fn one_run(a: &[crate::chronometry::KStepReceipt], b: &[crate::chronometry::KStepReceipt]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| one_receipt(x, y))
}

/// ONE ROOT, by `Root::restates`, either side absent alike.
pub fn one_root(a: Option<crate::chronometry::Root>, b: Option<crate::chronometry::Root>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a.restates(&b),
        _ => false,
    }
}

/// ONE ERA: one root (`Root::restates`), its signer, witness and both
/// signatures, its identities and range, and one run of receipts.
pub fn one_era(a: &crate::chronometry::Era, b: &crate::chronometry::Era) -> bool {
    a.root.restates(&b.root)
        && (a.signer, a.witness, a.sig_home, a.sig_witness) == (b.signer, b.witness, b.sig_home, b.sig_witness)
        && (a.identities, a.first, a.count, a.at) == (b.identities, b.first, b.count, b.at)
        && one_run(&a.receipts, &b.receipts)
}

/// ONE ARCHIVE: the same count, each pair `one_era`.
pub fn one_archive(a: &[crate::chronometry::Era], b: &[crate::chronometry::Era]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| one_era(x, y))
}

/// ONE FOLD PROPOSAL: one place after, one root, one set of epoch rows.
pub fn one_proposal(a: &crate::server::cognition::Proposal, b: &crate::server::cognition::Proposal) -> bool {
    a.after == b.after && a.root.restates(&b.root) && a.epochs == b.epochs
}

/// A fetched page as the back-fill reconstructs it: the run up to the
/// first receipt whose chain hash does not re-derive.
pub fn page(
    rs: &[crate::chronometry::KStepReceipt],
) -> Vec<crate::court::admission::Reconstructed<crate::chronometry::KStepReceipt>> {
    crate::court::admission::Reconstructed::receipts(rs).0
}

pub fn digits(rows: &[&str]) -> crate::transducer::spelled::SectionSample {
rows.iter()
    .map(|r| r.bytes().map(|b| b - b'0').collect())
    .collect()
}

pub fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 20)
}

/// 78.153 — A FETCHED WORD GLUED AS THE EXCHANGE GLUES IT: each fetch on one
/// stroke of the court's own metabolism, its size the host's (`spend_unit`,
/// read off the host's free memory); a replay that stroke could not finish
/// waits as dialect and the next fetch resumes it where it stopped — so a
/// fetch that rules nothing must have moved the replay, or the glue stalls
/// and the claim fails. Answers the verdict the replay reached: what a claim
/// about a glue's verdict asserts on, on any host.
pub fn fetched(k: &mut Court, spoken: &Element, peer: [u8; 32], own_vk: [u8; 32], cover: &mut crate::network::nerve::HomeTrie) -> Glue {
    loop {
        let before = k.court_reconstruction_progress(spoken);
        let mut stroke = k.stroke();
        if let Some(verdict) = k.glue_fetched_ruled(spoken.clone(), peer, own_vk, cover, &mut stroke) {
            return verdict;
        }
        assert_ne!(k.court_reconstruction_progress(spoken), before, "an unread glue resumes where it stopped, or it stalls");
    }
}

/// The origin the window fixtures' keepers sign at.
pub fn keeper_origin() -> crate::chronometry::tuniversal::Origin {
    crate::chronometry::tuniversal::Origin { genesis: [3; 32], epoch: 0 }
}

/// Two readings of the keeper `sk` at `origin`: the first citing `court` at
/// `carried` (its tick `tick`), the second its successor, citing the same.
pub fn keeper_line(sk: &ed25519_dalek::SigningKey, court: [u8; 32], carried: u64, tick: u64) -> Vec<crate::chronometry::tuniversal::TickReading> {
    use crate::chronometry::tuniversal::{Cited, Tick, TickReading};
    let origin = keeper_origin();
    let cites = vec![Cited { keeper: court, reading: [9; 32], h: carried }];
    let first = TickReading::sign(sk, &origin, Tick { h: tick, lanes: crate::economy::Mu::zero(), k: 0 }, [0; 32], cites.clone()).expect("signed");
    let at = first.hash(&origin).expect("hashed");
    let back = TickReading::sign(sk, &origin, Tick { h: tick + 1, lanes: crate::economy::Mu::zero(), k: 1 }, at, cites).expect("signed");
    vec![first, back]
}

/// The reach of `court` over keepers' lines, each `(keeper, readings)`.
pub fn reach_over(
    court: [u8; 32],
    lines: &[([u8; 32], Vec<crate::chronometry::tuniversal::TickReading>)],
) -> Result<crate::chronometry::tuniversal::Reach, crate::chronometry::tuniversal::WindowRefusal> {
    use crate::chronometry::tuniversal::{KeeperLine, Reach};
    Reach::of(keeper_origin(), court, lines.iter().map(|(keeper, l)| KeeperLine { keeper: *keeper, readings: l.iter().collect() }))
}

/// 78.125 (law version 5) — EVERY WINDOW `k` OPENED AT OR BELOW `through`
/// CLOSED, as the court loop closes it: two other keepers each cite the court
/// at `through` and read again after, and the reach they witness is handed to
/// the court, which seals the close. The receipt sealed, where one was.
pub fn close_through(k: &mut Court, through: u64) -> Option<crate::chronometry::KStepReceipt> {
    let keepers = [ed25519_dalek::SigningKey::from_bytes(&[61; 32]), ed25519_dalek::SigningKey::from_bytes(&[62; 32])];
    let lines: Vec<([u8; 32], Vec<_>)> = keepers.iter().map(|sk| (sk.verifying_key().to_bytes(), keeper_line(sk, k.identity, through, 1))).collect();
    let reach = reach_over(k.identity, &lines);
    k.close_windows(&reach)
}
