// A9, MU-DERIVATIONS D3.3 (ruled 2026-10-31) — π* binds the WHOLE common set:
// every term common to the training evidence, held as a superposition and
// judged by the court's reconstruction over the withheld pairs.

use crate::court::verdict::derive_pi;
use crate::transducer::anchor::Selection;

/// A 4×4 scene of four 2×2 blocks, hues `a, b` on the top row of blocks and
/// `c, d` below.
fn quilt(a: u8, b: u8, c: u8, d: u8) -> Vec<crate::transducer::spelled::PaintedSite> {
    (0..4)
        .flat_map(|r| {
            (0..4).map(move |col| {
                let hue = match (r / 2, col / 2) {
                    (0, 0) => a,
                    (0, _) => b,
                    (_, 0) => c,
                    _ => d,
                };
                crate::transducer::spelled::PaintedSite::on_plane([r, col], hue)
            })
        })
        .collect()
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 22)
}

/// THE SUPERPOSITION IS JUDGED BY RECONSTRUCTION ON THE WITHHELD PAIR: of the
/// terms common to the training pairs, those whose evaluation on the
/// withheld scene carries its footprint all stand — the whole support among
/// them — and a term that does not (the training corner's own hue, absent
/// from the withheld corner) is let go; no term is chosen by its key.
#[test]
fn derive_pi_keeps_every_term_reconstructing_withheld() {
    let origin = crate::transducer::spelled::SectionSite::on_plane([0, 0]);
    let fp = vec![origin];
    let train = vec![(fp.clone(), quilt(1, 2, 3, 4)), (fp.clone(), quilt(1, 5, 6, 7))];
    let common = derive_pi(&train, &[], &mut stroke()).expect("read");
    assert!(common.contains(&Selection::Hue(1)) && common.contains(&Selection::Hue(255)), "{common:?}");
    for (fp, scene) in &train {
        let set = crate::transducer::anchor::derive_carrying_all(fp, scene, &mut stroke()).expect("read");
        assert!(common.iter().all(|t| set.contains(t.key().as_bytes())), "common to every training set");
    }
    let withheld = vec![(fp.clone(), quilt(2, 1, 3, 4))];
    let judged = derive_pi(&train, &withheld, &mut stroke()).expect("read");
    assert!(judged.contains(&Selection::Hue(255)), "the whole support reconstructs the withheld corner");
    assert!(!judged.contains(&Selection::Hue(1)), "the training corner's hue does not");
    assert!(judged.len() > 1, "a superposition, every standing term kept: {judged:?}");
    for t in &judged {
        assert!(common.contains(t), "judged within the common set");
        assert!(t.eval(&withheld[0].1).contains(&origin), "{t:?} reconstructs the withheld footprint");
    }
}

fn line(row: &str) -> crate::transducer::spelled::SectionSample {
    crate::transducer::spelled::SectionSample::from(vec![row.bytes().map(|b| b - b'0').collect::<Vec<u8>>()])
}

/// The word: a body of 1 recoloured 3 in place, read off one pair. Its
/// footprint is the body's own painted sites, which selection terms over the
/// input scene can carry. (A motion's footprint also reaches the ground
/// sites it lands on, which no term over the input's painted sites carries,
/// so a motion binds ⊤.)
fn recoloured_word() -> crate::transducer::sentence::Form {
    crate::transducer::family::cipher_form(&line("1100200"), &line("3300200"))
}

/// #158 — π* IS BOUND FROM THE WORD'S OWN FOOTPRINTS: on a family and a
/// withheld pair whose change the word's speech covers, the A9 judgement
/// binds exactly what `derive_pi` binds over the word's footprints there
/// (each deduced by speaking the word), and no pair is left unbound.
#[test]
fn bind_pi_binds_common_set_over_deduced_footprints() {
    use crate::transducer::family::footprint::{footprint_of, Footprint};
    use crate::transducer::family::NoCrystal;
    let form = recoloured_word();
    let train = vec![(line("1100200"), line("3300200"))];
    let withheld = vec![(line("0110020"), line("0330020"))];
    let bound = crate::transducer::raster::on_ground(crate::transducer::family::transduce::Ground::One(0), || {
        let b = crate::court::verdict::bind_pi(&form, &train, &withheld, &NoCrystal, &mut stroke()).expect("bound");
        let evidence = |fam: &[(crate::transducer::spelled::SectionSample, crate::transducer::spelled::SectionSample)]| -> Vec<crate::transducer::anchor::Evidence> {
            fam.iter()
                .enumerate()
                .map(|(i, (x, _))| match footprint_of(&form, x, Some(i), &NoCrystal, &mut stroke()) {
                    Footprint::Read(fp) => (fp, crate::transducer::raster::painted_sites(x)),
                    other => panic!("the word speaks pair {i}: {other:?}"),
                })
                .collect()
        };
        let direct = derive_pi(&evidence(&train), &evidence(&withheld), &mut stroke()).expect("read");
        (b, direct)
    });
    let (b, direct) = bound;
    assert!(b.unbound.is_empty(), "the word covers the withheld change: {:?}", b.unbound);
    assert_eq!(b.terms, direct, "the judgement binds derive_pi's common set over the deduced footprints");
    assert!(!b.terms.is_empty(), "a superposition is bound: {:?}", b.terms);
}

/// #158 — A WORD WHOSE FOOTPRINT MISSES A WITHHELD PAIR'S CHANGE IS NOT
/// BOUND ON THAT PAIR: the pair that paints a site the word leaves alone is
/// named unbound with that site, and it judges nothing: the family's common
/// set stands, yet no withheld pair binds it, so the word stays ⊤. The same
/// family with a withheld pair the word covers binds terms.
#[test]
fn word_missing_a_withheld_change_is_not_bound_on_that_pair() {
    use crate::transducer::family::NoCrystal;
    let form = recoloured_word();
    let train = vec![(line("11002000"), line("33002000"))];
    let covered = (line("01102000"), line("03302000"));
    let missed = (line("11002000"), line("33002007"));
    let ground = || crate::transducer::family::transduce::Ground::One(0);
    let bind = |withheld: &[(crate::transducer::spelled::SectionSample, crate::transducer::spelled::SectionSample)]| {
        crate::transducer::raster::on_ground(ground(), || crate::court::verdict::bind_pi(&form, &train, withheld, &NoCrystal, &mut stroke()))
            .expect("bound")
    };
    let only = bind(std::slice::from_ref(&missed));
    assert_eq!(only.unbound.len(), 1, "{:?}", only.unbound);
    assert_eq!(only.unbound[0].pair, 0, "the pair whose change the word misses");
    assert_eq!(only.unbound[0].missed.iter().map(|s| s.coordinates()).collect::<Vec<_>>(), vec![[0, 7]], "the site it misses");
    assert!(only.terms.is_empty(), "no withheld pair binds the word: it stays ⊤");
    let held = bind(std::slice::from_ref(&covered));
    assert!(held.unbound.is_empty(), "the covered pair binds the word");
    assert!(!held.terms.is_empty(), "and the common set is bound on it: {:?}", held.terms);
}

/// ONE EVIDENCE OR NONE BINDS NOTHING BEYOND ITS OWN SET, AND A SPENT TOKEN
/// REFUSES THE WHOLE DERIVATION, never a part of it.
#[test]
fn derive_pi_refuses_spent_token_whole() {
    let fp = vec![crate::transducer::spelled::SectionSite::on_plane([0, 0])];
    let train = vec![(fp.clone(), quilt(1, 2, 3, 4)), (fp, quilt(1, 3, 2, 4))];
    assert_eq!(derive_pi(&[], &[], &mut stroke()), Ok(Vec::new()));
    let reply = derive_pi(&train, &[], &mut crate::physics::Physics::default().stroke_n(5));
    assert!(reply.is_err(), "{reply:?}");
}
