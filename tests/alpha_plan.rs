// AI-GENERATED CONTENT — see DISCLAIMER.md.
//! Alpha-plan proofs: vertices, GEN2, faces by constancy, speech, dialect, mint.

use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use mycelium::sdk::Epigenome;
use mycelium::transducer::alphabet::{relators, RelatorKind, Word, LetterId};
use mycelium::transducer::element::{from_connection, affine_of, identity_versor, Element};
use mycelium::transducer::family::{extract_rule, speak_rule, mint_sentence};

use mycelium::arc::Grid;

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b.saturating_sub(b'0')).collect()).collect()
}

#[test]
fn normal_form_two_tapes_one_key() {
    let q = LetterId::quarter((0, 1)).unwrap();
    let a = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
    let b = Word::identity();
    assert_eq!(a.key(), b.key());
    let geo = a.geodesic();
    assert_eq!(a.matrix(), geo.matrix());
    assert!(relators().iter().any(|r| r.kind == RelatorKind::QuarterOrderFour));
}

#[test]
fn gen2_round_trip_and_size() {
    let affine = affine_of(&identity_versor()).unwrap();
    let w = from_connection(&affine, &[(1, 2)], &vec![(0, 0, 1u8, 2)], [1, 1, 0, 0]).unwrap();
    let b = w.to_bytes();
    assert_eq!(b[0], Element::GEN2_MAGIC);
    let mut p = 0;
    let back = Element::from_bytes(&b, &mut p).unwrap();
    assert_eq!(back.address(), w.address());
    assert!(b.len() < 200, "a word is under 200 bytes, got {}", b.len());
}

#[test]
fn faces_by_constancy_three_squares() {
    let train_in = grid(&["11011", "11011", "00000", "11000", "11000"]);
    let train_out: Grid = train_in
        .iter()
        .map(|row| row.iter().map(|&h| if h == 1 { 2 } else { h }).collect())
        .collect();
    let family = vec![(train_in.clone(), train_out.clone())];
    let rule = extract_rule(&family).expect("three squares extract");
    let bodies = rule
        .nerve
        .germs
        .iter()
        .filter(|g| g.kind != mycelium::transducer::peel::ShellKind::Frame)
        .count();
    assert_eq!(bodies, 3, "three bodies remain distinct points, got {} germs {} sections", rule.nerve.germ_count(), rule.sections);
    let id = identity_versor();
    assert!(
        rule.nerve.faces.iter().filter(|f| {
            let gi = &rule.nerve.germs[f.i];
            let gj = &rule.nerve.germs[f.j];
            gi.kind != mycelium::transducer::peel::ShellKind::Frame
                && gj.kind != mycelium::transducer::peel::ShellKind::Frame
        }).all(|f| !mycelium::transducer::element::same_versor(&f.versor, &id)),
        "body-body faces carry a non-identity relative versor"
    );
    assert!(rule.nerve.is_sentence(), "three squares form a sentence");
    let spoken = speak_rule(&rule, &train_in).expect("spoken");
    assert!(spoken.remainder.is_empty(), "all three squares are reached: {:?}", spoken.remainder);
    assert_eq!(spoken.grid(), train_out);
    let _ = mint_sentence(&rule, &family);
}

#[test]
fn speak_rule_recolours_or_conflicts() {
    let train_in = grid(&["1100", "0000"]);
    let train_out = grid(&["2200", "0000"]);
    let family = vec![(train_in.clone(), train_out.clone())];
    let rule = extract_rule(&family).expect("one square extracts");
    let spoken = speak_rule(&rule, &train_in).expect("spoken");
    assert!(spoken.remainder.is_empty(), "the square is reached: {:?}", spoken.remainder);
    assert_eq!(spoken.grid(), train_out);
}

#[test]
fn speech_act_one_frame() {
    let affine = affine_of(&identity_versor()).unwrap();
    let w = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [1, 1, 0, 0]).unwrap();
    let body = dialogue::encode_speech(&[1u8; 32], &[2u8; 32], 7, &[w.to_bytes()]);
    let (voice, task, tu, ss) = dialogue::decode_speech(&body).expect("decode");
    assert_eq!(voice[0], 1);
    assert_eq!(task[0], 2);
    assert_eq!(tu, 7);
    assert_eq!(ss.len(), 1);
    let reply = dialogue::encode_speech_reply(&[dialogue::SPEECH_STOOD]);
    assert_eq!(dialogue::decode_speech_reply(&reply), Some(vec![dialogue::SPEECH_STOOD]));
}

#[test]
fn perceive_speech_refuses_with_a_reason() {
    let mut k = Kernel::new();
    let mut bad = mycelium::transducer::element::from_residual(&vec![(0, 0, 1u8, 2)]);
    bad.set_magnitude([99, 99, 99, 99]);
    let codes = k.perceive_speech([9; 32], [0; 32], std::slice::from_ref(&bad));
    assert_eq!(
        codes,
        vec![dialogue::SPEECH_REFUSED],
        "an unframed chain is refused, not stood"
    );
    let affine = affine_of(&identity_versor()).unwrap();
    let ok = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [1, 1, 0, 0]).unwrap();
    let codes = k.perceive_speech([9; 32], [0; 32], std::slice::from_ref(&ok));
    assert_eq!(codes, vec![dialogue::SPEECH_STOOD], "a replay-clean word stands");
}

#[test]
fn a_single_recolour_is_a_one_morphism_sentence() {
    let train_in = grid(&["1100", "0000"]);
    let train_out = grid(&["2200", "0000"]);
    let family = vec![(train_in, train_out)];
    let rule = extract_rule(&family).expect("one body extracts");
    assert!(rule.nerve.is_sentence(), "a single non-trivial morphism is a sentence");
    assert!(rule.nerve.is_closed());
}

#[test]
fn dialect_enters_on_stood_and_sheds() {
    let mut epi = Epigenome::new();
    let affine = affine_of(&identity_versor()).unwrap();
    let w = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).unwrap();
    let addr = w.address();
    epi.stand_dialect(w, 1);
    assert_eq!(epi.dialect_len(), 1);
    let n = epi.shed(1);
    assert_eq!(n, 0, "fresh dialect is not shed");
    let aperture = mycelium::physics::aperture_cycle() as u64;
    epi.touch(&addr, 1 + aperture);
    let n = epi.shed(1 + aperture);
    assert_eq!(n, 0, "a working-atlas use resets the TUniversal aperture");
    let n = epi.shed(1 + aperture + aperture + 1);
    assert_eq!(n, 1);
    assert_eq!(epi.dialect_len(), 0);
}

#[test]
fn hypha_snapshot_is_myc2() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8), (0, 1, 2)];
    let _ = k.ingest(&a, &b, [1; 32]);
    let snap = k.snapshot();
    assert_eq!(&snap[..4], b"MYC2");
    let mut k2 = Kernel::new();
    assert!(k2.restore(&snap));
    assert_eq!(k2.occupancy(), [0, 0, 0, 0], "cold listings exert zero occupancy");
}

#[test]
fn dialect_is_a_word_in_the_epigenome() {
    let mut epi = Epigenome::new();
    assert!(epi.express(&[(0, 0, 1u8, 2)]));
    assert_eq!(epi.dialect_len(), 1, "an expressed word stands as dialect");
    assert_eq!(epi.genome_len(), 0, "the genome stays frozen");
}

#[test]
fn genome_unchanged_by_dialect() {
    let mut epi = Epigenome::new();
    let n0 = epi.genome_len();
    let frozen = epi.crystallize();
    assert_eq!(&frozen[..4], b"GEN2");
    let affine = affine_of(&identity_versor()).unwrap();
    let w = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).unwrap();
    epi.stand_dialect(w, 3);
    assert_eq!(epi.genome_len(), n0, "dialect is not the genome");
}

#[test]
fn cohesion_is_per_lane() {
    let c = mycelium::transducer::bubble::Contemplation::new();
    let mass = c.cohesion();
    assert_eq!(mass.len(), 0);
}

#[test]
fn mint_once_on_home_install() {
    let mut k = Kernel::new();
    let affine = affine_of(&identity_versor()).unwrap();
    let w = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [2, 1, 0, 0]).unwrap();
    let codes = k.perceive_speech([9; 32], [0; 32], std::slice::from_ref(&w));
    assert_eq!(codes.first().copied(), Some(dialogue::SPEECH_STOOD));
}
