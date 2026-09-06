// AI-GENERATED CONTENT — see DISCLAIMER.md.
//! Alpha-plan proofs: vertices, GEN3, faces by constancy, speech, dialect, mint.

use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use mycelium::sdk::Epigenome;
use mycelium::transducer::alphabet::{relators, RelatorKind, Word, LetterId};
use mycelium::transducer::element::{from_connection, affine_of, identity_versor, Element};
use mycelium::transducer::family::{extract_rule, perceive, speak_rule, mint_sentence, NoCrystal};
use mycelium::transducer::sentence::{FormFace, Vertex};
use mycelium::transducer::stalk::Stalks;

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
fn gen3_round_trip_and_size() {
    let affine = affine_of(&identity_versor()).unwrap();
    let w = from_connection(&affine, &[(1, 2)], &vec![(0, 0, 1u8, 2)], [1, 1, 0, 0]).unwrap();
    let b = w.to_bytes();
    assert_eq!(b[0], Element::GEN3_MAGIC);
    let mut p = 0;
    let back = Element::from_bytes(&b, &mut p).unwrap();
    assert_eq!(back.address(), w.address());
    assert!(b.len() < 200, "a word is under 200 bytes, got {}", b.len());
    let mut old = b.clone();
    old[0] = Element::GEN2_MAGIC;
    let mut p = 0;
    assert!(Element::from_bytes(&old, &mut p).is_none(), "an old file is refused");
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
    let non_frame: Vec<(usize, &Vertex)> = rule
        .form
        .vertices
        .iter()
        .enumerate()
        .filter(|(_, v)| !matches!(v, Vertex::Frame))
        .collect();
    assert_eq!(
        non_frame.len(),
        1,
        "three squares carried by one recolour are one vertex, got {}",
        rule.form.vertices.len()
    );
    let (vi, v) = non_frame[0];
    let Vertex::Word { class, gauge, .. } = v else {
        panic!("the class vertex is a Word, got {v:?}");
    };
    assert!(
        gauge.iter().any(|&(from, to)| from == 1 && to == 2),
        "the Frame-to-class face carries the recolour gauge, got {gauge:?}"
    );
    assert!(
        rule.form.faces.iter().any(|FormFace { i, j, .. }| {
            (*i == 0 && *j == vi as u16) || (*j == 0 && *i == vi as u16)
        }),
        "a named face joins Frame to the class"
    );
    let bodies = perceive(&train_in);
    let mut atlas = Stalks::default();
    for (i, body) in bodies.iter().enumerate() {
        let mut addr = [0u8; 32];
        addr[..8].copy_from_slice(&(i as u64).to_le_bytes());
        atlas.admit(&body.moduli, addr);
    }
    assert_eq!(
        atlas.fiber(class).len(),
        3,
        "on the test grid the class fiber holds three bodies"
    );
    let spoken = speak_rule(&rule, &train_in, &NoCrystal).expect("spoken");
    assert!(spoken.remainder.is_empty(), "all three squares are reached: {:?}", spoken.remainder);
    assert_eq!(spoken.grid(), train_out);
    let _ = mint_sentence(&rule, &family, &NoCrystal);
}

#[test]
fn speak_rule_recolours_or_conflicts() {
    let train_in = grid(&["1100", "0000"]);
    let train_out = grid(&["2200", "0000"]);
    let family = vec![(train_in.clone(), train_out.clone())];
    let rule = extract_rule(&family).expect("one square extracts");
    let spoken = speak_rule(&rule, &train_in, &NoCrystal).expect("spoken");
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
    let codes = k.perceive_speech([9; 32], &[0; 32], std::slice::from_ref(&bad));
    assert_eq!(
        codes,
        vec![dialogue::SPEECH_REFUSED],
        "an unframed chain is refused, not stood"
    );
    let affine = affine_of(&identity_versor()).unwrap();
    let ok = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [1, 1, 0, 0]).unwrap();
    let codes = k.perceive_speech([9; 32], &[0; 32], std::slice::from_ref(&ok));
    assert_eq!(codes, vec![dialogue::SPEECH_STOOD], "a replay-clean word stands");
}

#[test]
fn a_single_recolour_is_a_one_morphism_sentence() {
    let train_in = grid(&["1100", "0000"]);
    let train_out = grid(&["2200", "0000"]);
    let family = vec![(train_in, train_out)];
    let rule = extract_rule(&family).expect("one body extracts");
    let words: Vec<_> = rule
        .form
        .vertices
        .iter()
        .filter(|v| matches!(v, mycelium::transducer::sentence::Vertex::Word { .. }))
        .collect();
    assert_eq!(words.len(), 1, "one recolour is one Word vertex");
    match words[0] {
        mycelium::transducer::sentence::Vertex::Word { gauge, .. } => {
            assert!(
                gauge.iter().any(|&(from, to)| from == 1 && to == 2),
                "the morphism carries the 1→2 gauge, got {gauge:?}"
            );
        }
        _ => unreachable!(),
    }
    assert!(rule.form.is_closed(), "a single non-trivial morphism is a closed sentence");
}

#[test]
fn dialect_enters_on_stood_and_sheds() {
    let mut epi = Epigenome::new();
    let affine = affine_of(&identity_versor()).unwrap();
    let w = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).unwrap();
    let addr = w.address();
    epi.stand_dialect_at(w, 1, 1);
    assert_eq!(epi.dialect_len(), 1);
    let aperture = mycelium::physics::aperture_cycle() as u64;
    // The aperture is measured from the strokes noted; until a whole
    // aperture of strokes stands, nothing sheds.
    let n = epi.shed(1);
    assert_eq!(n, 0, "an unmeasured aperture sheds nothing");
    for tu in 1..=aperture {
        epi.note_stroke(tu);
    }
    let n = epi.shed(aperture);
    assert_eq!(n, 0, "fresh dialect is not shed");
    epi.touch(&addr, 1 + aperture);
    let n = epi.shed(1 + aperture);
    assert_eq!(n, 0, "a working-atlas use resets the TUniversal aperture");
    for tu in (aperture + 1)..=(2 * aperture + 1) {
        epi.note_stroke(tu);
    }
    let n = epi.shed(1 + aperture + aperture + 1);
    assert_eq!(n, 1, "idle for more than the measured aperture sheds");
    assert_eq!(epi.dialect_len(), 0);
}

#[test]
fn hypha_snapshot_is_myc3() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8), (0, 1, 2)];
    let _ = k.ingest(&a, &b, [1; 32]);
    let snap = k.snapshot();
    assert_eq!(&snap[..4], b"MYC3");
    let mut k2 = Kernel::new();
    assert!(k2.restore(&snap));
    assert_eq!(k2.occupancy(), [0, 0, 0, 0], "cold listings exert zero occupancy");
    let mut old = snap.clone();
    old[..4].copy_from_slice(b"MYC2");
    assert!(!Kernel::new().restore(&old), "an old file is refused");
}

#[test]
fn dialect_is_a_word_in_the_epigenome() {
    let mut epi = Epigenome::new();
    assert!(epi.express(&[(0, 0, 1u8, 2)], 1, 1));
    assert_eq!(epi.dialect_len(), 1, "an expressed word stands as dialect");
    assert_eq!(epi.genome_len(), 0, "the genome stays frozen");
}

#[test]
fn genome_unchanged_by_dialect() {
    let mut epi = Epigenome::new();
    let n0 = epi.genome_len();
    let frozen = epi.crystallize();
    assert_eq!(&frozen[..4], b"GEN3");
    let affine = affine_of(&identity_versor()).unwrap();
    let w = from_connection(&affine, &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).unwrap();
    epi.stand_dialect_at(w, 3, 3);
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
    let codes = k.perceive_speech([9; 32], &[0; 32], std::slice::from_ref(&w));
    assert_eq!(codes.first().copied(), Some(dialogue::SPEECH_STOOD));
}
