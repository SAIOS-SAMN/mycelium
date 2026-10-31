// MU-CLIENT §5.6 (W, C, S, E1) — THE COURT MEASURES WHAT IS SPOKEN UNDER A
// SCENE, by the spec's own conditions: a word on every site its speech
// changed and nothing where it said something false; a conjugate by what its
// conjugator added, lane by lane; a sentence's gluing only where its rings
// compose.

use crate::arc::Grid;
use crate::cognition::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Word;
use crate::transducer::element::{from_connection, translator_versor};
use crate::transducer::family::local::{form_local, measure_sites};
use crate::transducer::family::{extract_rule, mint_sentence, site_key, NoCrystal};

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

fn sites(at: &[(i64, i64)]) -> Keyed<()> {
    let mut k = Keyed::new();
    for &(r, c) in at {
        k.insert(site_key(r, c), ());
    }
    k
}

/// The slide by two columns, spelled as a connection.
fn slide() -> crate::transducer::element::Element {
    from_connection(&Word::identity(), &translator_versor(0, 2), &[], &Vec::new(), [1, 0, 0, 0]).expect("a slide spells")
}

/// W — THE COURT'S ONE READER PAYS A WORD ON THE SITES ITS SPEECH CHANGED, AND
/// NOTHING ON A PAIR IT SPOKE FALSELY ON. On a family whose first pair the
/// slide says whole and whose second it says falsely (a body that stands), the
/// court's `discharge_on` is the first pair's demand over the sites the slide
/// changed there — painted and vacated — and nothing of the second; on no
/// scene it measures nothing.
#[test]
fn discharge_on_pays_changed_sites_not_false_pair() {
    let court = Court::new();
    let family = vec![
        (grid(&["0000", "1100", "0000"]), grid(&["0000", "0011", "0000"])),
        (grid(&["1100", "0000", "0200"]), grid(&["0011", "0000", "0200"])),
    ];
    let w = slide();
    let addr = w.address();
    let read = court.discharge_on(addr.as_bytes(), &w, Some(&family));
    let changed = sites(&[(1, 0), (1, 1), (1, 2), (1, 3)]);
    assert_eq!(read, measure_sites(&family, &[changed, Keyed::new()]), "the first pair's changed sites, nothing of the false one");
    assert!(read.iter().any(|&m| m > 0), "and the word is paid there");
    assert_eq!(court.discharge_on(addr.as_bytes(), &w, None), [0; 4], "on no scene, nothing");
}

/// C — A CONJUGATE THAT ADDS NOTHING IS PAID NOTHING. The court holds a rule W
/// that acts on the family (its local discharge non-zero); W conjugated by
/// the identity rotor acts exactly as W does, so its transport — its local
/// discharge less W's, lane by lane — is zero on every lane, never a copy of
/// W's own pay. A word that is no conjugate of a held word carries none.
#[test]
fn transport_on_pays_identity_conjugate_nothing() {
    let mut court = Court::new();
    let family = vec![(grid(&["0000", "1100", "0000"]), grid(&["0000", "0011", "0000"]))];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the slide extracts");
    let held = mint_sentence(&rule, &family, &NoCrystal, 0).expect("the rule mints");
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    // Held and RULED, as an install holds it (`home_install`): a conjugate
    // cites the court's ruled words, and only a ruled word resolves.
    court.basis_push(held.clone(), [1; 32], &mut budget);
    court.rule_own(held.address().as_bytes(), &mut budget);
    let form = crate::transducer::sentence::from_element_form(&held).expect("its form");
    assert!(form_local(&form, &family, &court, &Word::identity()).pay.iter().any(|&m| m > 0), "W acts on the family");
    let (vertex, rotor, chart) = form
        .vertices
        .entries()
        .find_map(|(_, v)| match v {
            crate::transducer::sentence::Vertex::Word { address, derivation, acts, .. } => {
                let (_, a) = acts.scoped().next()?;
                Some((address.clone(), derivation.rotor(), a.chart.clone()))
            }
            _ => None,
        })
        .expect("W speaks by a word vertex");
    let derivation = crate::transducer::alphabet::Derivation::factored(vec![(held.address(), vertex, 1)], Word::identity(), rotor)
        .conjugated(crate::transducer::alphabet::Rotor::identity());
    let conj = crate::transducer::sentence::composed_form(derivation, &crate::transducer::element::identity_versor(), &chart).expect("the conjugate composes");
    assert!(court.conjugate_of(&conj).is_some(), "a conjugate of the held W");
    assert_eq!(court.transport_on(&conj, &family), [0; 4], "the identity conjugator added nothing on any lane");
    assert!(court.conjugate_of(&slide()).is_none(), "a slide is no conjugate");
}

/// S, E1 (as ruled 2026-10-15) — A SENTENCE IS ALIGNED BY ITS CLOSURE, NEVER
/// BY ITS RINGS RETURNING. The family's sentence (a bar slid and recoloured)
/// carries one ring, and its holonomy is the slide — it never returns
/// (`Unbound`): demanding it return would be squaring (78.168 §2), and it is
/// not docked for it. Spoken under its scene, with no word of it supplied, its
/// gluing is its own whole section, S, G and H read on its sites, 𝒯 the ground
/// its composition's motions covered; with the words it teaches supplied, the
/// gluing is what it closes beyond them, never more on any lane; bound to no
/// family it glues nothing.
#[test]
fn gluing_of_reads_closure_beyond_words() {
    let mut court = Court::new();
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0022", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0022"])),
    ];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a rule extracts");
    let spoken = mint_sentence(&rule, &family, &NoCrystal, 0).expect("a sentence");
    let form = crate::transducer::sentence::from_element_form(&spoken).expect("its form");
    let cycles = crate::transducer::sentence::Form::cycles_of(&form.faces, &form.vertices);
    let rings: Vec<crate::transducer::sentence::Orbit> = cycles.kept.iter().chain(&cycles.vacua).cloned().collect();
    let composed = crate::transducer::sentence::compose::sentences_of(&form, &rings, &mut crate::physics::Physics::default().stroke_n(1 << 20));
    assert!(
        composed.iter().any(|(_, r)| r.as_ref().is_ok_and(|c| !c.closure.closes())),
        "a ring whose holonomy never returns: the slide it carries"
    );
    let addr = spoken.address();
    assert_eq!(court.gluing_of(addr.as_bytes(), &spoken, &[]), [0; 4], "bound to no family: nothing");
    let task = crate::registry::citation(&crate::registry::task_id(&crate::arc::Task { train: family.clone(), test: Vec::new() }).expect("a task identity"));
    court.bind_task_family(task, family.clone());
    let codes = court.perceive_speech([1; 32], &task, std::slice::from_ref(&spoken));
    assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD], "spoken under its scene, it stands");
    let section = form_local(&form, &family, &court, &Word::identity()).pay;
    assert!(section.iter().any(|&m| m > 0), "it closes the family on its section");
    let whole = court.gluing_of(addr.as_bytes(), &spoken, &[]);
    let t = crate::transducer::sentence::presentation::LANE_T;
    for l in (0..4).filter(|&l| l != t) {
        assert_eq!(whole[l], section[l], "lane {l}: its whole section where no word is supplied, the unreturning ring docking none of it");
    }
    let ground = crate::transducer::family::form_deflation_read(&form, &family, &court, &Word::identity()).pay[t];
    assert_eq!(whole[t], ground, "𝒯: the ground its composition's motions covered, whole");
    let words = crate::cognition::book::speech::words_of_sentence(&spoken, &mut crate::physics::Physics::default().stroke_n(1 << 20));
    let beyond = court.gluing_of(addr.as_bytes(), &spoken, &words);
    for l in (0..4).filter(|&l| l != t) {
        assert!(beyond[l] <= whole[l], "lane {l}: its words only take sites away: {beyond:?} against {whole:?}");
    }
}
