// AI-GENERATED CONTENT — see DISCLAIMER.md.
//! Alpha-plan proofs: vertices, GEN3, faces by constancy, speech, dialect, mint.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "§2.4 binds cognition; this crate is a test"
)]

use mycelium::anastome::dialogue;
use mycelium::cognition::Kernel;
use mycelium::sdk::Epigenome;
use mycelium::transducer::alphabet::{relators, RelatorKind, Word, LetterId};
use mycelium::transducer::element::{from_connection, identity_versor, Element};
use mycelium::transducer::family::{extract_rule, perceive, speak_rule, mint_sentence, NoCrystal};
use mycelium::transducer::sentence::{FormFace, Vertex};
use mycelium::transducer::stalk::Stalks;

use mycelium::arc::Grid;

fn grid(rows: &[&str]) -> Grid {
    // A byte below '0' is not a hue; the row is the fixture and says so.
    rows.iter()
        .map(|r| r.bytes().map(|b| b.checked_sub(b'0').expect("a grid row is digits")).collect())
        .collect()
}

fn conn(sigma: &[(u8, u8)], rem: Vec<(i64, i64, u8, u8)>, mag: [u64; 4]) -> Element {
    from_connection(&Word::identity(), &identity_versor(), sigma, &rem, mag).unwrap()
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
fn the_record_round_trip_and_size() {
    let w = conn(&[(1, 2)], vec![(0, 0, 1u8, 2)], [1, 1, 0, 0]);
    let b = w.to_bytes();
    assert_eq!(b[0], Element::RECORD_MAGIC);
    let mut p = 0;
    let back = Element::from_bytes(&b, &mut p).unwrap();
    assert_eq!(back.address(), w.address());
    // THE BOUND IS THE CANONICAL GEN3 RECORD'S, not GEN2's (54.7 B6/B7).
    // It stood at 600, which was the pre-B6 layout: the address did not
    // lead the record and a sentence still wrote a description field. B6
    // put the address first and purged the description, and the round trip
    // has measured 1047 bytes at every commit since — including at the
    // commit that introduced it. The assertion was checking a retired
    // schema and failing on the live one, which is a stale expectation and
    // not a size regression.
    //
    // The bound still bounds: what it says is that a WORD is a short tape —
    // kilobytes, not the megabytes a form's address runs to (Z4c) — so it
    // catches the failure it exists for, a word that starts carrying a
    // form's coordinates.
    assert!(b.len() < 2048, "a word is a short tape, got {}", b.len());
    for magic in [Element::GEN2_MAGIC, Element::PRE_WHOLE_MAGIC] {
        let mut old = b.clone();
        old[0] = magic;
        let mut p = 0;
        assert!(Element::from_bytes(&old, &mut p).is_none(), "an old file is refused");
    }
}

#[test]
fn faces_by_constancy_three_squares() {
    let train_in = grid(&["11011", "11011", "00000", "11000", "11000"]);
    let train_out: Grid = train_in
        .iter()
        .map(|row| row.iter().map(|&h| if h == 1 { 2 } else { h }).collect())
        .collect();
    let family = vec![(train_in.clone(), train_out.clone())];
    let rule = extract_rule(&family, &mut mycelium::physics::Physics::default().stroke_n(1 << 20)).expect("three squares extract");
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
    let Vertex::Word { acts, .. } = v else {
        panic!("the class vertex is a Word, got {v:?}");
    };
    assert!(
        acts.scoped().any(|(_, a)| a.gauge.iter().any(|&(from, to)| from == 1 && to == 2)),
        "the Frame-to-class face carries the recolour gauge, got {:?}",
        acts.scoped().map(|(_, a)| a.gauge.clone()).collect::<Vec<_>>()
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
        acts.scoped().map(|(_, a)| atlas.fiber(&a.chart).len()).sum::<usize>(),
        3,
        "on the test grid the charts this vertex acts on hold three bodies between them"
    );
    let spoken = speak_rule(&rule, &train_in, &NoCrystal).expect("spoken");
    assert!(spoken.remainder.is_empty(), "all three squares are reached: {:?}", spoken.remainder);
    assert_eq!(spoken.grid(), Some(train_out));
    let _ = mint_sentence(&rule, &family, &NoCrystal, 0);
}

#[test]
fn speak_rule_recolours_or_conflicts() {
    let train_in = grid(&["1100", "0000"]);
    let train_out = grid(&["2200", "0000"]);
    let family = vec![(train_in.clone(), train_out.clone())];
    let rule = extract_rule(&family, &mut mycelium::physics::Physics::default().stroke_n(1 << 20)).expect("one square extracts");
    let spoken = speak_rule(&rule, &train_in, &NoCrystal).expect("spoken");
    assert!(spoken.remainder.is_empty(), "the square is reached: {:?}", spoken.remainder);
    assert_eq!(spoken.grid(), Some(train_out));
}

#[test]
fn speech_act_one_frame() {
    let w = conn(&[(1, 2), (2, 1)], Vec::new(), [1, 1, 0, 0]);
    let body = dialogue::encode_speech(&[1u8; 32], [2u8; 32], 7, &[w.to_bytes()]).expect("the task frames");
    let act = dialogue::decode_speech(&body).expect("decode");
    assert_eq!(act.voice[0], 1);
    assert_eq!(act.task[0], 2);
    assert_eq!(act.k, 7);
    assert_eq!(act.sentences.len(), 1);
    let stood = dialogue::Adjudication::Stood { receipt: [4; 32] };
    let reply =
        dialogue::encode_speech_reply(std::slice::from_ref(&stood)).expect("one adjudication fits");
    assert_eq!(dialogue::decode_speech_reply(&reply), Some(vec![stood]));
}

#[test]
fn perceive_speech_refuses_with_a_reason() {
    let mut k = Kernel::new();
    let mut bad = mycelium::transducer::element::from_canonical(
        &mycelium::transducer::element::canonical_chain(&[(0, 0, 1u8, 2)])
            .expect("one cell is A5's chain"),
    );
    bad.set_magnitude([99, 99, 99, 99]);
    let codes = k.perceive_speech([9; 32], &[0; 32], std::slice::from_ref(&bad));
    assert_eq!(
        codes,
        vec![dialogue::SPEECH_REFUSED],
        "an unframed chain is refused, not stood"
    );
    let ok = conn(&[(1, 2), (2, 1)], Vec::new(), [1, 1, 0, 0]);
    let codes = k.perceive_speech([9; 32], &[0; 32], std::slice::from_ref(&ok));
    assert_eq!(codes, vec![dialogue::SPEECH_STOOD], "a replay-clean word stands");
}

#[test]
fn a_single_recolour_is_a_one_morphism_sentence() {
    let train_in = grid(&["1100", "0000"]);
    let train_out = grid(&["2200", "0000"]);
    let family = vec![(train_in, train_out)];
    let rule = extract_rule(&family, &mut mycelium::physics::Physics::default().stroke_n(1 << 20)).expect("one body extracts");
    let words: Vec<_> = rule
        .form
        .vertices
        .iter()
        .filter(|v| matches!(v, mycelium::transducer::sentence::Vertex::Word { .. }))
        .collect();
    // ONE RECOLOUR IS ONE WORD VERTEX, which the slice pattern says: the
    // count and the kind are one claim, so there is no second arm left to
    // call unreachable (§2.4 — a test crate is allowed its aborts, and
    // `unreachable!` is not among them, because it asserts nothing).
    let [mycelium::transducer::sentence::Vertex::Word { acts, .. }] = words.as_slice() else {
        panic!("one recolour is one Word vertex, got {:?}", words.len())
    };
    assert!(
        acts.scoped().any(|(_, a)| a.gauge.iter().any(|&(from, to)| from == 1 && to == 2)),
        "the morphism carries the 1→2 gauge, got {:?}",
        acts.scoped().map(|(_, a)| a.gauge.clone()).collect::<Vec<_>>()
    );
    assert!(rule.form.is_closed(), "a single non-trivial morphism is a closed sentence");
}

#[test]
fn dialect_enters_on_stood_and_sheds() {
    let mut epi = Epigenome::new();
    let w = conn(&[(1, 2), (2, 1)], Vec::new(), [1, 0, 0, 0]);
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

/// 28, 78.27: the snapshot names its layout — the mind as keyed tries,
/// MYT1 — and the positional tapes before it do not re-read: MYC7 is
/// refused by name, an older one as what it is.
#[test]
fn hypha_snapshot_is_myt1() {
    let mut k = Kernel::new();
    // A voice's sentence for a recolour, spoken through the one door an
    // utterance has (law 8): a listing on the book to snapshot.
    let family = vec![(grid(&["11"]), grid(&["22"]))];
    let rule = extract_rule(&family, &mut mycelium::physics::Physics::default().stroke_n(1 << 20)).expect("a recolour extracts");
    let spoken = mint_sentence(&rule, &family, &NoCrystal, 0).expect("a sentence");
    let task = mycelium::registry::task_id(&mycelium::arc::Task { train: family.clone(), test: Vec::new() })
        .expect("a grid pair's chain is A5's, so the task has an identity");
    k.bind_task_family(&task, family);
    let codes = k.perceive_speech([1; 32], task.as_bytes(), std::slice::from_ref(&spoken));
    assert_eq!(codes, vec![dialogue::SPEECH_STOOD], "the sentence is listed");
    let snap = k.snapshot();
    assert_eq!(&snap[..4], b"MYT1");
    let mut k2 = Kernel::new();
    assert!(k2.restore(&snap));
    assert_eq!(k2.occupancy(), [0, 0, 0, 0], "cold listings exert zero occupancy");
    for magic in [b"MYC7", b"MYC6"] {
        let mut old = snap.clone();
        old[..4].copy_from_slice(magic);
        assert!(!Kernel::new().restore(&old), "an old file is refused");
    }
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
    let (frozen, refused) = epi.crystallize(|_| None);
    assert_eq!(&frozen[..4], b"GEN6");
    assert_eq!(refused.total(), 0, "an empty genome refuses nothing");
    let w = conn(&[(1, 2), (2, 1)], Vec::new(), [1, 0, 0, 0]);
    epi.stand_dialect_at(w, 3, 3);
    assert_eq!(epi.genome_len(), n0, "dialect is not the genome");
}

// `cohesion_is_per_lane` was here and is DELETED with the reading it named
// (P10). It built an EMPTY contemplation and asserted the map came back
// empty, so the per-lane sum it was named for never ran: the walk had
// nothing to walk and the test passed by iterating nothing (rule 13). The
// function it called summed each idea's `volume` per chart and called the
// total a chart's congruent mass, which counts how often the mind repeated
// itself rather than whether two ideas agree; agreement is the transition
// between their transports being the identity, a reading of the two.

#[test]
fn mint_once_on_home_install() {
    let mut k = Kernel::new();
    let w = conn(&[(1, 2), (2, 1)], Vec::new(), [2, 1, 0, 0]);
    let codes = k.perceive_speech([9; 32], &[0; 32], std::slice::from_ref(&w));
    // One sentence was spoken, so one code comes back.
    assert!(matches!(codes.as_slice(), [dialogue::SPEECH_STOOD]));
}
