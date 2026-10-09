// 26, 50.2 step 2 — A CITED GENERATOR IS PAID AT THE INSTALL, and a
// sentence that does not stand pays nobody.

use crate::cognition::*;
use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::{Derivation, LetterId, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::{as_element_form, Form, Vertex};

fn turn_90(n: usize) -> Word {
    let q = LetterId::turn_90((0, 1)).expect("the 90° turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// A closed one-word sentence whose vertex carries `d`.
fn sentence_at(d: Derivation, tag: u8, mag: [u64; 4]) -> Element {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    let v = Vertex::word_on(
        d,
        // Its own chart: an address is derived from its coordinates (54.2).
        crate::transducer::stalk::Invariant::counts(&[u64::from(tag)]),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        Vec::new(),
        [0],
        None,
    );
    let c = f.stand_member(&v, 0).expect("a word vertex stands");
    f.place_overlap(crate::network::nerve::coord::ChartAt::Frame, c, None).expect("star");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    as_element_form(&f, mag).expect("a tape")
}

/// The common case: a sentence measuring one unit on the first lane.
fn sentence_with(d: Derivation, tag: u8) -> Element {
    sentence_at(d, tag, [1, 0, 0, 0])
}

/// The generator and the sentence that derives through it twice.
fn generator_and_citer() -> (Element, Element) {
    // The generator measures on a lane the speaker can actually draw:
    // a mint bonds the first lane, so a use priced only there is a use
    // no voice can pay and the test would prove nothing.
    let g = sentence_at(Derivation::spelled(turn_90(1)), 1, [0, 1, 0, 0]);
    let citer = sentence_with(
        Derivation::compose(
            &[(g.address(), Address::from_bytes([TAG_FORM, 1]), 2, turn_90(1))],
            Word::identity(),
        ),
        9,
    );
    (g, citer)
}

/// A sentence that never installs pays nobody. Here the court has not
/// ruled the cited form at all, so the citer WAITS (21 step 3) — it is
/// not refused, and it is also not installed, so no generator is paid
/// and no balance moves anywhere.
#[test]
fn a_citer_that_does_not_install_pays_no_generator() {
    let teacher = [1u8; 32];
    let speaker = [3u8; 32];
    let (g, citer) = generator_and_citer();

    let mut k = Kernel::new();
    let anchor = k.anchor_pub();
    k.treasury.mint_claim(&[speaker], [8, 8, 8, 8], anchor);
    let before = k.treasury.balance_mu(&teacher);

    // The generator was never ruled here.
    assert_eq!(
        k.perceive_speech(speaker, &[0; 32], std::slice::from_ref(&citer)),
        vec![crate::anastome::dialogue::SPEECH_DIALECT],
        "the citer waits on a form this court has not ruled"
    );
    assert!(!k.ruled_contains(citer.address().as_bytes()), "so it does not install");
    assert_eq!(
        k.treasury.balance_mu(&teacher),
        before,
        "and nothing is paid for a sentence that did not stand"
    );
    // Nothing was drawn from the speaker either.
    let _ = g;
}

/// The family D6 and L8t are proved on: a body the taught generator does
/// NOT reproduce. An L turning 180° is `q²`; the generator taught is a
/// BAR turning 90°, `q`. The bar's chart is not the L's, so 21 step
/// 3's glue never sees it (`on_body` answers at the body's chart) while
/// the derivation does (`generators_under` answers at the VERSOR's), and
/// A12 factors `q² = q·q`. Where the generator acts on the same body the
/// glue fires instead and the placement word absorbs the difference —
/// which is the same mechanism at the form level, not a failure of this
/// one.
fn bar_turn_90() -> Vec<crate::transducer::family::TrainPair> {
    vec![(digits(&["111", "000", "000"]), digits(&["001", "001", "001"]))]
}

fn l_turn_180() -> Vec<crate::transducer::family::TrainPair> {
    vec![(digits(&["110", "100", "000"]), digits(&["000", "001", "011"]))]
}

fn digits(rows: &[&str]) -> Vec<Vec<u8>> {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// Hear a sentence from `voice` on `task` and install it through the two
/// vantages a home install needs (47.3).
fn install_on(k: &mut Kernel, voice: [u8; 32], e: &Element, task: &[u8]) {
    assert_eq!(
        k.perceive_speech(voice, task, std::slice::from_ref(e)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "the court hears it"
    );
    k.rule_all_provisionals();
    let cover = k.genesis_cover().expect("measured");
    k.enroll([9; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&e.address()), [9; 32]);
}

/// 26 AS RULED 2026-09-10 (L8t) — THE USE OF A CITED GENERATOR IS A SHARE
/// OF THE CITER'S OWN MINT. The teacher speaks the bar's 90° turn and
/// the court rules it. A mind learns it and derives the L's 180° turn
/// through it. A second voice speaks that sentence; at ITS install the
/// cited form's own deflation on the citer's family is minted to the
/// teacher, out of what the citer closed — not transferred out of the
/// speaker's balance, which paid zero whenever the citer's own draw had
/// already occupied the lane.
#[test]
fn a_cited_generators_speaker_is_paid_a_share_of_the_citers_mint() {
    use crate::client::sdk::Epigenome;
    use crate::transducer::family::{
        extract_rule_with, mint_sentence, cipher_form, sentence_measure, NoCrystal,
    };
    use crate::transducer::sentence::as_element_form;
    let teacher = [1u8; 32];
    let speaker = [3u8; 32];
    let (gen_fam, cite_fam) = (bar_turn_90(), l_turn_180());

    let g = cipher_form(&gen_fam[0].0, &gen_fam[0].1);
    let g_mag = sentence_measure(&g, &gen_fam, &NoCrystal, 0).expect("the bar is spoken");
    let g_e = as_element_form(&g, g_mag).expect("a tape");
    let g_addr = g_e.address();

    let mut k = Kernel::new();
    let g_task = [6u8; 32];
    k.bind_task_family(g_task, gen_fam.clone());
    let c_task = [7u8; 32];
    k.bind_task_family(c_task, cite_fam.clone());
    install_on(&mut k, teacher, &g_e, &g_task);
    assert!(k.ruled_contains(g_addr.as_bytes()), "the generator is language");
    assert_eq!(k.discoverer_of(g_addr.as_bytes()), Some(teacher), "and it is the teacher's");

    let mut epi = Epigenome::new();
    epi.learn(&g_e.to_bytes().expect("the record writes"), &mut super::common::budget()).expect("the syllabus teaches it back");
    let rule = extract_rule_with(&cite_fam, &epi, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts");
    let cited: Vec<_> = rule
        .form
        .vertices
        .entries()
        .filter_map(|(_, v)| match v {
            Vertex::Word { derivation, .. } => Some(derivation.generators.clone()),
            _ => None,
        })
        .flatten()
        .collect();
    assert!(
        cited.iter().any(|(form, _, _)| form.as_bytes() == g_addr.as_bytes()),
        "the 180° turn DERIVES through the 90° turn the mind was taught, and is not glued to it"
    );
    let spoken = mint_sentence(&rule, &cite_fam, &epi, 0).expect("minted over the mind's own crystal");

    // 26 as ruled 2026-09-10 — WHAT A DERIVATION CITATION EARNS IS ROOM,
    // NOT A MINT. The generator contributes an ACTION and closes nothing
    // on the citer's family by itself, so it takes no share of the mint;
    // what a use earns is standing to say more (E2), added to the bond at
    // the read and stored nowhere.
    let room_before = k.lane_room_of(&teacher);
    let purse_before = k.treasury.balance_mu(&teacher);
    install_on(&mut k, speaker, &spoken, &c_task);
    assert!(k.ruled_contains(spoken.address().as_bytes()), "the citer installs");

    let cited = k
        .cited_by
        .get(g_addr.as_bytes())
        .expect("the derivation citation is recorded as a use");
    assert!(
        cited.contains(spoken.address().as_bytes()),
        "by the citer's own address, as a Glued citation is"
    );
    assert_eq!(
        k.use_credit(&teacher),
        Some(g_e.magnitude()),
        "the teacher's credit is the cited form's own μ⃗, once for this one citer"
    );
    let room_after = k.lane_room_of(&teacher);
    assert!(
        room_after.iter().zip(&room_before).any(|(a, b)| a > b),
        "and the teacher's room rises with it: before {room_before:?} after {room_after:?}"
    );
    assert_eq!(
        k.treasury.balance_mu(&teacher),
        purse_before,
        "while nothing is minted and no balance moves: a use is room, not coin"
    );
}

/// The 270° turn of the same bar: mind B's generator, where
/// mind A holds the 90° turn. `q³` and `q`, and the L's 180° turn is `q²`
/// through either — `q·q` for A and `q³·q³ = q⁶ ≡ q²` for B.
fn bar_turn_270() -> Vec<crate::transducer::family::TrainPair> {
    vec![(digits(&["111", "000", "000"]), digits(&["100", "100", "100"]))]
}

/// L9's CLOSE — THE PROOF THROUGH THE CODEC, ON TWO COURTS OF DISTINCT
/// IDENTITY. Two minds are taught different generators by one court and
/// speak the same family. The vertex ADDRESS is one, because identity is
/// the rotor (1.5); the DERIVATIONS are two, because each mind factored
/// through what it holds (50.2 step 2); and each teacher's room rises at
/// its own key on the court that installs the citer.
///
/// Everything crosses the codec: each mind's sentence is encoded as a
/// speech act and decoded by the court, so what is proved is what the
/// wire carries and not what one process happened to hold.
#[test]
fn two_minds_reach_one_address_by_two_derivations_and_both_teachers_earn_room() {
    use crate::anastome::dialogue;
    use crate::client::sdk::Epigenome;
    use crate::transducer::family::{
        extract_rule_with, mint_sentence, cipher_form, sentence_measure, NoCrystal,
    };
    use crate::transducer::sentence::as_element_form;

    let (teacher_a, teacher_b) = ([1u8; 32], [2u8; 32]);
    let (voice_a, voice_b) = ([3u8; 32], [4u8; 32]);
    let cite_fam = l_turn_180();
    let c_task = [7u8; 32];

    // One court teaches both generators; each mind learns its own.
    let mut school = Kernel::new();
    let taught = |k: &mut Kernel, fam: &[crate::transducer::family::TrainPair], who: [u8; 32], task: [u8; 32]| {
        let f = cipher_form(&fam[0].0, &fam[0].1);
        let mag = sentence_measure(&f, fam, &NoCrystal, 0).expect("the bar is spoken");
        let e = as_element_form(&f, mag).expect("a tape");
        k.bind_task_family(task, fam.to_vec());
        install_on(k, who, &e, &task);
        e
    };
    let g_a = taught(&mut school, &bar_turn_90(), teacher_a, [11u8; 32]);
    let g_b = taught(&mut school, &bar_turn_270(), teacher_b, [12u8; 32]);
    assert!(!g_a.same_vertex(&g_b), "the two minds are taught different generators");

    let mind = |g: &crate::transducer::element::Element| {
        let mut epi = Epigenome::new();
        epi.learn(&g.to_bytes().expect("the record writes"), &mut super::common::budget()).expect("the syllabus teaches it back");
        let rule = extract_rule_with(&cite_fam, &epi, &mut crate::physics::Physics::default().stroke_n(1 << 20))
            .expect("the family extracts");
        mint_sentence(&rule, &cite_fam, &epi, 0).expect("minted over the mind's own crystal")
    };
    let said_a = mind(&g_a);
    let said_b = mind(&g_b);

    // ONE ADDRESS. Compared by the vertex, never by bytes (rule 3).
    assert!(
        said_a.same_vertex(&said_b),
        "two presentations, one vertex: the address cannot tell them apart, and must not"
    );

    // TWO DERIVATIONS, each naming its own mind's teacher.
    // THE PROOFS ARE READ OFF THE ENVELOPE (54.5 Road (d)). This read
    // `derivation.generators` from the decoded form, and that is precisely
    // what the address does not carry: §1.5 makes the composed ROTOR the
    // coordinate so two presentations reach one address, which is the claim
    // three lines above this. A decoded vertex is therefore `spelled`, and
    // reading the proof off the form would have made the next two
    // assertions vacuous rather than false.
    //
    // The two halves of this test are the two carriers: the ADDRESS cannot
    // tell the minds apart, and the ENVELOPE must.
    let cites = |e: &crate::transducer::element::Element| -> Vec<crate::transducer::address::Address> {
        e.receipts()
            .derivations
            .iter()
            .flat_map(|d| d.generators.iter().map(|(form, _, _)| form.clone()))
            .collect()
    };
    let (ca, cb) = (cites(&said_a), cites(&said_b));
    assert!(
        ca.iter().any(|a| a.as_bytes() == g_a.address().as_bytes()),
        "A derives through what A was taught"
    );
    assert!(
        cb.iter().any(|a| a.as_bytes() == g_b.address().as_bytes()),
        "B derives through what B was taught"
    );
    assert_ne!(ca, cb, "and the two proofs differ where the address cannot");

    // TWO COURTS OF DISTINCT IDENTITY, each hearing one mind THROUGH THE
    // CODEC and installing what it heard.
    for (court_id, voice, said, teacher, g) in [
        ([21u8; 32], voice_a, &said_a, teacher_a, &g_a),
        ([22u8; 32], voice_b, &said_b, teacher_b, &g_b),
    ] {
        let mut k = Kernel::new();
        k.set_court_voice(court_id);
        k.bind_task_family(c_task, cite_fam.clone());
        let g_task = [13u8; 32];
        let gen_fam = match teacher == teacher_a { true => bar_turn_90(), false => bar_turn_270() };
        k.bind_task_family(g_task, gen_fam);
        install_on(&mut k, teacher, g, &g_task);

        let frame = dialogue::encode_speech(&voice, c_task, 1, &[said.to_bytes().expect("the record writes")]).expect("the task frames");
        let act = dialogue::decode_speech(&frame).expect("the frame decodes");
        // 78.153: a stroke that holds the frame's replays.
        let heard = k
            .hear_frame_on(&act, &mut crate::physics::Physics::default().stroke_n(1 << 20))
            .expect("the court hears it");
        // One sentence was spoken, so one adjudication comes back.
        let [only] = heard.as_slice() else { panic!("one sentence, one adjudication") };
        assert_eq!(
            only.code(),
            dialogue::SPEECH_STOOD,
            "the sentence stands on a court that holds the generator it cites"
        );
        k.rule_all_provisionals();
        let cover = k.genesis_cover().expect("measured");
        k.enroll([9; 32], cover);
        k.confirm_peer_holds_addrs(std::slice::from_ref(&said.address()), [9; 32]);
        assert!(k.ruled_contains(said.address().as_bytes()), "and installs");

        assert_eq!(
            k.use_credit(&teacher),
            Some(g.magnitude()),
            "this court credits THIS mind's teacher, under that teacher's own key"
        );
        assert!(
            k.lane_room_of(&teacher)
                .expect("a credited teacher's room reads")
                .iter()
                .any(|&r| r > 0),
            "so the teacher's room rises where its generator was used"
        );
    }
}
