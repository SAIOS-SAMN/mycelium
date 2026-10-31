// AI-GENERATED CONTENT — see DISCLAIMER.md.
//! Alpha-plan proofs: vertices, GEN3, faces by constancy, speech, dialect, mint.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "§2.4 binds cognition; this crate is a test"
)]

use mycelium::anastome::dialogue;
use mycelium::cognition::Court;
use mycelium::transducer::alphabet::{relators, RelatorKind, Word, LetterId};
use mycelium::transducer::element::{from_connection, identity_versor, Element};
use mycelium::transducer::family::{extract_rule, perceive, speak_rule, mint_sentence, NoCrystal};
use mycelium::transducer::sentence::Vertex;
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

/// A connection that MOVES (one column) with its gauge: a word spoken with
/// no task. Motionless, a σ alone is a colour word, which stands only by the
/// evidence of the family it is spoken with (78.53).
fn moved(sigma: &[(u8, u8)], mag: [u64; 4]) -> Element {
    from_connection(&Word::identity(), &mycelium::transducer::element::translator_versor(1, 0), sigma, &Vec::new(), mag)
        .unwrap()
}

/// Two tapes of one normal form have one key: four 90°-turn letters and the identity word share a key and a matrix.
#[test]
fn word_key_is_one_for_two_spellings() {
    let q = LetterId::turn_90((0, 1)).unwrap();
    let a = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
    let b = Word::identity();
    assert_eq!(a.key(), b.key());
    let geo = a.geodesic();
    assert_eq!(a.matrix(), geo.matrix());
    assert!(relators().iter().any(|r| r.kind == RelatorKind::Turn90OrderFour));
}

/// The element record round-trips within its size bound, and a record under an old magic is refused.
#[test]
fn element_record_round_trips_and_refuses_old_magic() {
    let w = conn(&[(1, 2)], vec![(0, 0, 1u8, 2)], [1, 1, 0, 0]);
    let b = w.to_bytes().expect("the record writes");
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

/// Faces by constancy: three squares carried by one recolour are one vertex joined to the frame, and the rule speaks them all.
#[test]
fn extract_rule_joins_three_squares_in_one_vertex() {
    let train_in = grid(&["11011", "11011", "00000", "11000", "11000"]);
    let train_out: Grid = train_in
        .iter()
        .map(|row| row.iter().map(|&h| if h == 1 { 2 } else { h }).collect())
        .collect();
    let family = vec![(train_in.clone(), train_out.clone())];
    let rule = extract_rule(&family, &mut mycelium::physics::Physics::default().stroke_n(1 << 20)).expect("three squares extract");
    let non_frame: Vec<(mycelium::network::nerve::coord::VertexAt, &Vertex)> = rule
        .form
        .vertices
        .entries()
        .filter(|(_, v)| !matches!(v, Vertex::Frame))
        .collect();
    assert_eq!(
        non_frame.len(),
        1,
        "three squares carried by one recolour are one vertex, got {}",
        rule.form.vertices.len()
    );
    let (vi, v) = non_frame[0].clone();
    let Vertex::Word { acts, .. } = v else {
        panic!("the class vertex is a Word, got {v:?}");
    };
    assert!(
        acts.scoped().any(|(_, a)| a.gauge.iter().any(|&(from, to)| from == 1 && to == 2)),
        "the Frame-to-class face carries the recolour gauge, got {:?}",
        acts.scoped().map(|(_, a)| a.gauge.clone()).collect::<Vec<_>>()
    );
    assert!(
        // A star overlap: the Frame's chart to a member of the class (AT1).
        rule.form.faces.entries().any(|(o, _)| {
            use mycelium::network::nerve::coord::VertexAt;
            (o.from_vertex() == VertexAt::Frame && o.to_vertex() == vi)
                || (o.to_vertex() == VertexAt::Frame && o.from_vertex() == vi)
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

/// A spoken rule recolours the square it was extracted on, leaving no remainder.
#[test]
fn speak_rule_recolours_single_square() {
    let train_in = grid(&["1100", "0000"]);
    let train_out = grid(&["2200", "0000"]);
    let family = vec![(train_in.clone(), train_out.clone())];
    let rule = extract_rule(&family, &mut mycelium::physics::Physics::default().stroke_n(1 << 20)).expect("one square extracts");
    let spoken = speak_rule(&rule, &train_in, &NoCrystal).expect("spoken");
    assert!(spoken.remainder.is_empty(), "the square is reached: {:?}", spoken.remainder);
    assert_eq!(spoken.grid(), Some(train_out));
}

/// A speech act is one frame that decodes back, and its reply carries each adjudication with where its word stands.
#[test]
fn encode_speech_round_trips_in_one_frame() {
    let w = conn(&[(1, 2), (2, 1)], Vec::new(), [1, 1, 0, 0]);
    let body = dialogue::encode_speech(&[1u8; 32], [2u8; 32], 7, &[w.to_bytes().expect("the record writes")]).expect("the task frames");
    let act = dialogue::decode_speech(&body).expect("decode");
    assert_eq!(act.voice[0], 1);
    assert_eq!(act.task[0], 2);
    assert_eq!(act.k, 7);
    assert_eq!(act.sentences.len(), 1);
    // 78.93 B0: a verdict carries where its word stands, or says none does.
    let at = mycelium::chronometry::Stamp::new(9, [3; 32], mycelium::chronometry::Place { k: 2, i: 1 });
    let stood = dialogue::Adjudication::Stood { receipt: [4; 32], at: Some(at) };
    let ratified = dialogue::Adjudication::Ratified { receipt: [5; 32], at: None };
    let both = [stood, ratified];
    let reply = dialogue::encode_speech_reply(&both).expect("two adjudications fit");
    assert_eq!(dialogue::decode_speech_reply(&reply), Some(both.to_vec()));
}

/// Perceived speech is refused with a reason when the chain is unframed, and a replay-clean word stands.
#[test]
fn perceive_speech_refuses_unframed_chain() {
    let mut k = Court::new();
    let mut bad = mycelium::transducer::element::from_canonical(
        &mycelium::transducer::element::canonical_chain(&[(0, 0, 1u8, 2)])
            .expect("one cell is A5's chain"),
    );
    assert!(bad.set_magnitude([99, 99, 99, 99]), "a fresh word holds any lane");
    let codes = k.perceive_speech([9; 32], &[0; 32], std::slice::from_ref(&bad));
    assert_eq!(
        codes,
        vec![dialogue::SPEECH_REFUSED],
        "an unframed chain is refused, not stood"
    );
    let ok = moved(&[(1, 2), (2, 1)], [1, 1, 0, 0]);
    let codes = k.perceive_speech([9; 32], &[0; 32], std::slice::from_ref(&ok));
    assert_eq!(codes, vec![dialogue::SPEECH_STOOD], "a replay-clean word stands");
}

/// A single recolour is a one-morphism sentence: one word vertex carrying the gauge, and the form closed.
#[test]
fn extract_rule_closes_single_recolour_as_one_word() {
    let train_in = grid(&["1100", "0000"]);
    let train_out = grid(&["2200", "0000"]);
    let family = vec![(train_in, train_out)];
    let rule = extract_rule(&family, &mut mycelium::physics::Physics::default().stroke_n(1 << 20)).expect("one body extracts");
    let words: Vec<_> = rule
        .form
        .vertices
        .scoped()
        .map(|(_, v)| v)
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

/// 28, 78.27, 45.8: the snapshot names its layout — the mind as keyed
/// tries, its line and eras on disk beside it, MYT2 — and the tapes before
/// it do not re-read: MYT1 and MYC7 are refused by name, an older one as
/// what it is.
#[test]
fn snapshot_writes_myt2_and_refuses_older_layouts() {
    let mut k = Court::new();
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
    assert_eq!(&snap[..4], b"MYT2");
    let mut k2 = Court::new();
    assert!(k2.restore(&snap));
    assert_eq!(k2.occupancy(), [0, 0, 0, 0], "cold listings exert zero occupancy");
    for magic in [b"MYT1", b"MYC7", b"MYC6"] {
        let mut old = snap.clone();
        old[..4].copy_from_slice(magic);
        assert!(!Court::new().restore(&old), "an old file is refused");
    }
}


// `cohesion_is_per_lane` was here and is DELETED with the reading it named
// (P10). It built an EMPTY contemplation and asserted the map came back
// empty, so the per-lane sum it was named for never ran: the walk had
// nothing to walk and the test passed by iterating nothing (rule 13). The
// function it called summed each idea's `volume` per chart and called the
// total a chart's congruent mass, which counts how often the mind repeated
// itself rather than whether two ideas agree; agreement is the transition
// between the two ideas' transports being the identity, a reading of the two.

/// A sentence spoken once at its home is stood once, with one code back.
#[test]
fn perceive_speech_answers_one_code_per_sentence() {
    let mut k = Court::new();
    let w = moved(&[(1, 2), (2, 1)], [2, 1, 0, 0]);
    let codes = k.perceive_speech([9; 32], &[0; 32], std::slice::from_ref(&w));
    // One sentence was spoken, so one code comes back.
    assert!(matches!(codes.as_slice(), [dialogue::SPEECH_STOOD]));
}
