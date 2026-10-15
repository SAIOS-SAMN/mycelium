// 42 — THE COURT: a verdict installs, excises or refuses, and every organ
// reaches it by replaying the word rather than by trusting a signature.
//
// The pair fixture is an L recoloured WHOLE (one body, one word). Under 70.4
// a body is hue-blind and its hues are its gauge, so an L with one site
// recoloured is one body whose gauge no single hue map carries — a pair the
// voice has no sentence for (50.5), so nothing reaches a court.

use super::common::*;
use crate::cognition::*;
use crate::transducer::element::{self, Element};

/// THE BLIND REPLAY refutes every tamper axis and upholds honest speech —
/// the contradiction docket's whole force is this determinism.
#[test]
fn blind_replay_refutes_tampered_words() {
use crate::transducer::germinate::germinate;
use crate::verdict::blind_replay;
let w = element::from_canonical(
    &germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap(),
);
assert!(blind_replay(&w, &mut super::common::budget()).is_none(), "an honest derivation replays clean");
// Tamper 1 CANNOT BE BUILT any more: A5's chain is the field's type, so
// an element cannot hold one that is off its min corner. What a shifted
// chain is, is the same substance said at another place — A5's quotient,
// not false speech — and it comes back as the same word.
let shifted: Vec<(i64, i64, u8, u8)> =
    w.sub().iter().map(|(r, c, b, a)| (r + 3, *c, *b, *a)).collect();
let off = element::from_residual(&shifted).expect("a shift is still one transition per site");
assert_eq!(off.address(), w.address(), "A5 quotients the translation: one substance");
assert!(blind_replay(&off, &mut super::common::budget()).is_none(), "so it replays clean — it is the same word");
// Tamper 2: inflate the claimed magnitude (yield fraud).
let mut fat = w.clone();
fat.mag = [999, 0, 0, 0];
assert!(blind_replay(&fat, &mut super::common::budget()).is_some(), "inflated magnitude does not re-derive");
// Tamper 3: an empty chain claiming work.
let mut empty = w.clone();
empty.sub = element::CanonicalChain::default();
assert!(blind_replay(&empty, &mut super::common::budget()).is_some(), "no evidence, no word");
}

/// 47.3, 47.5, Stage 12 S12.4d — AN OFFER STANDS PENDING UNTIL THE WITNESS
/// ANSWERS. Before it is offered a word has no standing at a witness;
/// offered, it stands pending at the K it left; a witness that sent its
/// vantage has answered and is not asked again; a word that leaves the
/// docket takes its standings with it.
#[test]
fn an_offer_stands_pending_until_the_witness_answers() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8), (0, 1, 2)];
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
// The admitting act emitted its offer: one germ was spoken, so one offer.
let offers = k.take_offers();
let [(addr, _)] = offers.as_slice() else { panic!("the germ is offered at first vantage") };
assert!(k.take_offers().is_empty(), "taken once: the transition's offers are its own");
let witness = [7u8; 32];
assert_eq!(k.offer_standing(addr.as_bytes(), &witness), None, "not yet offered to this witness");
let now = k.k_step();
k.note_offered(addr.as_bytes(), witness, now);
assert_eq!(
    k.offer_standing(addr.as_bytes(), &witness),
    Some(OfferStanding::Pending { k: now }),
    "offered: pending at the K it left"
);
k.note_witness_answered(addr.as_bytes(), witness);
assert_eq!(
    k.offer_standing(addr.as_bytes(), &witness),
    Some(OfferStanding::Answered),
    "the witness answered and is not asked again"
);
k.relocate_standing(addr.as_bytes());
assert_eq!(
    k.offer_standing(addr.as_bytes(), &witness),
    None,
    "a word off the docket takes its standings with it"
);
}

/// STEP 5 — PROVISIONAL UNTIL THE COURT SPEAKS: a two-voice install escrows
/// its consilience award (unfinalized speech earns nothing); the docket
/// exports the spoken word; the upheld verdict releases the award; a
/// slashing verdict excises the word and debits the voices into the pool.
/// The second voice speaks the same percept: a percept lists one sentence
/// (20 as ruled, 50.1), and a second vantage on it is a use of that
/// address, never a second listing.
#[test]
fn installs_are_provisional_until_verdict() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
// The install PAID NOTHING — the award sits in escrow.
assert_eq!(k.treasury().balance_mu(&[1; 32]), [0; 4], "unfinalized speech earns nothing");
assert_eq!(k.treasury().balance_mu(&[2; 32]), [0; 4]);
let dockets = k.dockets();
assert_eq!(dockets.len(), 1, "the provisional word is on the docket");
let row = &dockets[0];
assert_eq!(row.voices.len(), 2, "both attesting voices ride the docket");
assert!(row.escrowed.iter().any(|&m| m > 0), "the escrowed award is real");
assert_eq!(row.spoken.address(), row.addr.clone(), "the docket carries the word as spoken");
let addr = &row.addr;
// UPHELD: finality releases the award — the voices are paid at last.
assert!(k.apply_verdict(&addr.clone(), crate::anastome::dialogue::RULING_UPHELD));
assert!(k.treasury().balance_mu(&[1; 32]).iter().any(|&m| m > 0), "finality pays voice A");
assert!(k.treasury().balance_mu(&[2; 32]).iter().any(|&m| m > 0), "finality pays voice B");
assert!(k.dockets().is_empty(), "the docket cleared");
assert!(!k.apply_verdict(&addr.clone(), crate::anastome::dialogue::RULING_UPHELD), "a verdict lands once");
}

/// A slashing verdict (failed blind replay — false speech) excises the word
/// from the basis and debits the voices' earned balances into the pool.
#[test]
fn a_slashing_verdict_excises_and_debits() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
let addr = k.dockets()[0].addr.clone();
assert!(k.apply_verdict(&addr, crate::anastome::dialogue::RULING_SLASHED));
assert_eq!(k.treasury().balance_mu(&[1; 32]), [0; 4], "nothing was ever paid");
assert!(k.dockets().is_empty());
}

/// Court organ: docketing a sentence does not grow the basis. The stroke
/// puts a section on the ruled cover. A peer's fetch glues by replay;
/// a forged word is slashed with a reason. Signatures are not consulted.
/// The forged word is the spoken sentence with its word vertices struck
/// out: a tape of the Frame and a demand, refused as a chain (50.6).
#[test]
fn court_organ_glue_on_replay_not_on_signatures() {
let mut home = Kernel::new();
let mut peer = Kernel::new();
let mut third = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
// The bar recoloured, a site painted beside it (50.5). H1b, 70.4 —
// separated: painted at `(0, 2)` the site is one piece with the bar once
// the cover is hue-blind, and no versor carries that piece.
let b = vec![(0, 0, 4), (0, 1, 4), (0, 3, 3)];
assert_eq!(speak_pair(&mut home, &a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(speak_pair(&mut home, &a, &b, [2; 32]), Outcome::GermPending);
assert_eq!(home.basis_len(), 0);
home.court_stroke();
let spoken = home.provisional.scoped().next().unwrap().1.clone();
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(fetched(&mut peer, &spoken, [1; 32], [2; 32], &mut cover), Glue::Admitted));
assert!(peer.basis_len() >= 1, "the word is language on the second organ");
assert!(matches!(fetched(&mut third, &spoken, [2; 32], [3; 32], &mut cover), Glue::Admitted));
let forged = strip_words(&spoken);
assert!(
    matches!(fetched(&mut third, &forged, [2; 32], [3; 32], &mut cover), Glue::Slashed(_)),
    "never admit on a word that fails replay"
);
}

/// Adopt writes a field entry and a deflation so the stroke can rule
/// a client-minted connection. Ruled listings do not starve later ones.
#[test]
fn adopt_reaches_the_docket_and_stroke_skips_ruled() {
use crate::transducer::element::from_connection;
fn conn(shift: i64) -> Element {
    from_connection(
        &crate::transducer::alphabet::Word::identity(),
        &crate::transducer::element::translator_versor(shift, 0),
        &[],
        &Vec::new(),
        [1, 0, 0, 0],
    )
    .expect("spells")
}
let mut k = Kernel::new();
let cap = crate::physics::Physics::default().stroke().remaining();
let mut addrs = Vec::new();
for i in 0..(cap + 1) {
    let w = conn(i as i64 + 1);
    let addr = w.address();
    let mut adopt = k.stroke();
    assert!(matches!(k.install_adopted(&w, [1; 32], &mut adopt), Adoption::Installed));
    assert!(k.field().get(&addr).is_some(), "adopt writes a field entry");
    addrs.push(addr);
}
let ruled_n = |k: &Kernel, addrs: &[crate::transducer::address::Address]| {
    addrs
        .iter()
        .filter(|a| {
            k.word_at(a).is_some_and(|w| k.ruled_have(&crate::transducer::stalk::invariant_of(w)).iter().any(|h| h.as_slice() == a.as_bytes()))
        })
        .count()
};
k.court_stroke();
assert_eq!(ruled_n(&k, &addrs), cap, "one stroke rules a spend of unruled listings, not already-ruled ones");
k.court_stroke();
assert_eq!(ruled_n(&k, &addrs), cap + 1, "later listings are not starved by already-ruled ones");
}

/// R6, 47.3 — A SIMPLEX'S VOLUME IS THE OPEN LISTINGS ON ITS CHART: the
/// measure of the remainders the chart's open, non-flat fields hold. It
/// is one quantity for the chart, whatever vantages the simplex has; a
/// flat field and a field on another chart contribute nothing. Read
/// through a speaker's index it is zero for every hypha vertex, so no
/// section stands a vertex and nothing fills.
#[test]
fn a_simplex_volume_is_the_open_listings_on_its_chart() {
use crate::transducer::address::Address;
use crate::transducer::stalk::{Closure, Invariant};
let mut k = Kernel::new();
let inv = Invariant::counts(&[7]);
let listed = Address::from(&b"listed"[..]);
let flat = Address::from(&b"flat"[..]);
let elsewhere = Address::from(&b"elsewhere"[..]);
let rem = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 0)];
let mag = crate::transducer::element::measure_lanes(&rem);
assert!(mag.iter().any(|&x| x > 0));
k.write_field(listed.as_bytes(), inv.clone(), Closure::Cavity, rem.clone(), &[0; 4], &mag);
k.write_field(flat.as_bytes(), inv.clone(), Closure::Flat, Vec::new(), &[0; 4], &[0; 4]);
k.write_field(elsewhere.as_bytes(), Invariant::counts(&[8]), Closure::Cavity, rem.clone(), &[0; 4], &mag);
assert_eq!(k.volume_of_simplex(&inv), mag, "the open listing on the chart is the simplex's volume");
assert_eq!(k.listings_on(&inv).len(), 1, "the flat field and the other chart's field are not on it");
assert_eq!(k.volume_of_simplex(&Invariant::counts(&[9])), [0; 4], "a chart with no open listing has no volume");
}

/// 45.7, S9.14, S13.3 — THE DOCKET IS A CIRCUIT: a docket of more listings
/// than one budget is ruled whole across strokes, the stroke whose read
/// passes the end of the key order reports the circuit and its period,
/// and a listing admitted below the resume point is ruled on the next
/// circuit — where a cursor that resumed after the last key it served and
/// stopped at the end of the order never reached it.
#[test]
fn the_docket_is_a_circuit_and_reaches_a_listing_admitted_below_the_cursor() {
use crate::transducer::element::from_connection;
fn conn(shift: i64) -> Element {
    from_connection(
        &crate::transducer::alphabet::Word::identity(),
        &crate::transducer::element::translator_versor(shift, 0),
        &[],
        &Vec::new(),
        [1, 0, 0, 0],
    )
    .expect("spells")
}
let mut k = Kernel::new();
let cap = crate::physics::Physics::default().stroke().remaining();
for i in 0..(cap + 1) {
    let mut adopt = k.stroke();
    assert!(matches!(k.install_adopted(&conn(2 * (i as i64 + 1)), [1; 32], &mut adopt), Adoption::Installed));
}
k.court_stroke();
assert_eq!(k.docket_circuits().circuits(), 0, "one stroke over cap + 1 listings is not a circuit");
// The stroke above yielded keys, so the docket's cursor stands.
let resume = k.open_cursor.clone().expect("one stroke over the docket set the cursor");
// A listing whose address sorts below the resume point, found among the
// odd shifts — never constructed by hand.
let below = (0..cap as i64 + 2)
    .map(|i| conn(2 * i + 1))
    .find(|w| w.address().as_bytes() < resume.as_slice())
    .expect("an odd shift sorts below the cursor");
let addr = below.address();
let mut adopt = k.stroke();
assert!(matches!(k.install_adopted(&below, [1; 32], &mut adopt), Adoption::Installed));
assert!(!k.ruled_contains(addr.as_bytes()), "admitted, not yet ruled");
// The circuit reaches it before the read returns to where it began: the
// tail, then past the end from the first key.
let mut strokes = 0;
while !k.ruled_contains(addr.as_bytes()) {
    strokes += 1;
    assert!(strokes <= 3, "the listing below the cursor is ruled within one circuit");
    k.court_stroke();
}
let docket = k.docket_circuits();
assert!(docket.circuits() >= 1, "the circuit passed the end of the docket: {docket:?}");
assert!(docket.period() >= 1, "the period is measured in strokes: {docket:?}");
}

/// The home installs when a peer's ruled digest holds the stratum.
#[test]
fn home_installs_when_peer_ruled_root_gains_the_address() {
let mut home = Kernel::new();
let mut peer = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
assert_eq!(speak_pair(&mut home, &a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(speak_pair(&mut home, &a, &b, [2; 32]), Outcome::GermPending);
assert_eq!(home.basis_len(), 0);
home.rule_all_provisionals();
let spoken = home.provisional.scoped().next().unwrap().1.clone();
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(fetched(&mut peer, &spoken, [1; 32], [2; 32], &mut cover), Glue::Admitted));
assert!(peer.basis_len() >= 1);
let cover = home.genesis_cover().expect("measured");
home.enroll([2; 32], cover); // 47.3: a witness with room
home.confirm_peer_holds(&peer.ruled_digests(), [2; 32]);
assert!(home.basis_len() >= 1, "the home's own word is language once a peer holds the stratum");
assert!(home.provisional.is_empty(), "the provisional leaves the home");
}

/// A peer have-list that names a ruled provisional is the second organ:
/// the home installs without waiting for a digest-root match.
#[test]
fn home_installs_on_a_have_list_naming_a_ruled_provisional() {
let mut home = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
assert_eq!(speak_pair(&mut home, &a, &b, [1; 32]), Outcome::GermPending);
let addr = home.provisional.scoped().next().unwrap().0.clone();
// 78.153: every stroke the docket's replay takes.
home.court_settle();
assert!(home.provisional.contains(&addr), "still provisional until the second organ");
// 47.3: a witness is a second VANTAGE with room, not merely a second
// key. An unbonded peer has none and does not converge the object.
home.confirm_peer_holds_addrs(&[Address::from(addr.as_slice())], [2; 32]);
assert!(home.basis_len() == 0, "a witness without room is not a vantage");
let cover = home.genesis_cover().expect("measured");
home.enroll([2; 32], cover);
home.confirm_peer_holds_addrs(&[Address::from(addr.as_slice())], [2; 32]);
assert!(home.basis_len() >= 1, "the witness re-derives the address and it installs");
assert!(home.provisional.is_empty(), "the docket clears on install");
}

/// 23.1 step 7 on the court's own stroke (I3): a docketed form is replayed
/// against the family it is bound to, so a form that leaves a remainder on
/// a pair of that family is slashed with `REFUSE_NOT_CLOSED` where the
/// stroke rules, and the same form bound to the family it closes is ruled.
/// Before this the stroke replayed with no family, so closure was checked
/// only at speech and never again.
#[test]
fn the_court_stroke_refuses_a_form_that_leaves_a_remainder_on_its_family() {
use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};
let grid = |rows: &[&str]| -> crate::arc::Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
};
let (x, y) = (grid(&["1"]), grid(&["2"]));
let one_pair = vec![(x.clone(), y.clone())];
let rule = extract_rule(&one_pair, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a recolour is a rule");
let word = mint_sentence(&rule, &one_pair, &NoCrystal, 0).expect("it closes its own pair");
let addr = word.address();
// H1, 70.4 — separated: `"23"` is ONE two-hue body once hue is a fiber.
let wider = vec![(x, y), (grid(&["100"]), grid(&["203"]))];

let mut refusing = Kernel::new();
let task = [7u8; 32];
refusing.bind_task_family(task, wider);
refusing.docket_connection(&word, [1; 32]);
refusing.speech_task.insert(&addr, Address::from_bytes(task));
assert!(refusing.provisional.contains(&addr), "docketed");
let at = refusing.admitted_at(&addr).expect("the docket admitted it at an act");
// 78.153: every stroke the replay takes; a spent stroke slashes nothing.
refusing.court_settle();
assert_eq!(refusing.court_slashed, 1, "the stroke refuses the form on the family it does not close");
assert!(!refusing.provisional.contains(&addr), "and it leaves the docket");
assert!(!refusing.ruled_contains(&addr), "never ruled");
// EC1: A SLASH IS ALWAYS AN ACT — the stroke's own ruling seals
// `Ruling(Slashed)` citing the word by the stamp that admitted it.
assert!(
    refusing.chain().own().any(|r| r.kind
        == crate::chronometry::ActKind::Ruling(crate::chronometry::Ruled::Slashed)
        && r.cites.iter().any(|c| c.at() == Some(at))),
    "the stroke's slash is a Ruling(Slashed) node citing the word"
);

let mut ruling = Kernel::new();
ruling.bind_task_family(task, one_pair);
ruling.docket_connection(&word, [1; 32]);
ruling.speech_task.insert(&addr, Address::from_bytes(task));
ruling.court_settle();
assert_eq!(ruling.court_slashed, 0, "the form closes every pair of the family it is bound to");
assert!(ruling.ruled_contains(&addr), "and the stroke rules it");
}

/// A SENTENCE SPOKEN FOR ITSELF IS BOUND TO NO TASK. `Entity::speak` names
/// an empty task for a word spoken on its own; the gate compared it with
/// `[0; 32]` alone, so the listing was bound to the empty task and every
/// reader of the binding asked for the family the empty task has none of.
#[test]
fn a_sentence_spoken_with_no_task_binds_no_task() {
    let e = sentence_of(&[(0, 0, 1)], &[(0, 0, 2)]).expect("a recolour is a sentence");
    let addr = e.address();
    let mut k = Kernel::new();
    let out = k.perceive_sentence([1; 32], &[], &e);
    assert!(!matches!(out, Outcome::Refused(_) | Outcome::Pivot), "heard, not refused: {out:?}");
    assert!(k.speech_task.get(&addr).is_none(), "no task was named, so none is bound");
    assert!(k.listing_task.get(&addr).is_none());

    let mut bound = Kernel::new();
    let task = [7u8; 32];
    let out = bound.perceive_sentence([1; 32], &task, &e);
    assert!(!matches!(out, Outcome::Refused(_) | Outcome::Pivot), "heard, not refused: {out:?}");
    assert_eq!(bound.speech_task.get(&addr).map(|t| t.as_bytes().to_vec()), Some(task.to_vec()));
}

/// 50.5 — A SENTENCE THAT DECLARES WHAT IT LEFT UNSPOKEN IS HEARD, and its
/// demand is listed for the market. The rule is read off one pair and
/// spoken over a family whose second pair asks for a site the rule's word
/// never reaches: the mint declares that site as a `Source` demand, the
/// court replays the sentence on the bound family with its own
/// `undeclared_remainder` and finds nothing undeclared, and the demand is
/// listed under the sentence rather than the sentence refused for saying
/// less than the whole.
#[test]
fn a_sentence_declaring_its_unspoken_remainder_is_heard_and_its_demand_listed() {
    use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};
    let grid = |rows: &[&str]| -> crate::arc::Grid {
        rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
    };
    let learned = vec![(grid(&["1"]), grid(&["2"]))];
    let rule = extract_rule(&learned, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("a recolour extracts");
    // H1, 70.4 — separated, as above.
    let family = vec![(grid(&["1"]), grid(&["2"])), (grid(&["100"]), grid(&["203"]))];
    let s = mint_sentence(&rule, &family, &NoCrystal, 0).expect("spoken with its demand");
    let mut k = Kernel::new();
    let task = [7u8; 32];
    k.bind_task_family(task, family);
    let out = k.perceive_sentence([1; 32], &task, &s);
    assert!(!matches!(out, Outcome::Refused(_) | Outcome::Pivot), "heard, not refused: {out:?}");
    assert!(k.court_demands_listed >= 1, "the demand it declared is listed for the market");
}

/// 47.3 — A BONDED HYPHA IS A SECOND VANTAGE, AND AN UNBONDED ONE IS NOT.
/// The home dockets a word at its own vantage; a peer's replay of it is
/// `AwaitingWitness` while the peer holds no bond here, and `Witnessed`
/// once the genesis bonds it at the cover (MU-PRODUCT §4). On 0fd41ca no
/// court bonded any other hypha, so every answered offer waited forever.
#[test]
fn a_bonded_hypha_is_a_second_vantage_and_an_unbonded_one_is_not() {
    use crate::court::convergence::Converged;
    let mut k = Kernel::new();
    let peer = [2u8; 32];
    let word = crate::transducer::element::from_residual(&vec![(0i64, 0i64, 1u8, 2u8)])
        .expect("one link is A5's chain");
    let addr = word.address();
    k.docket_connection(&word, [9; 32]);
    let (kk, h) = (k.k_step(), k.height());
    assert_eq!(
        k.converge_witness(&addr, peer, word.clone(), kk, h),
        Converged::AwaitingWitness,
        "an unbonded hypha is no vantage"
    );
    k.enroll(peer, [12, 3, 0, 20]);
    assert!(
        matches!(k.converge_witness(&addr, peer, word, kk, h), Converged::Witnessed { .. }),
        "bonded at the cover, the same replay is the second vantage"
    );
}

/// 47.3, 13.9: A WITNESS GLUES AN OFFER AS THE HOME, NOT AS ITSELF. The
/// Čech simplex of a converging pair has two vertices, one per vantage. Gluing
/// with `peer == self_id` writes the offered word into the section this
/// court's own reading is read back from, so the court compares the form
/// with itself and agrees every time — an agreement manufactured by writing
/// the answer into the place the question is asked.
#[test]
fn a_witness_glues_an_offer_at_the_homes_vantage() {
    let home_vk = [1u8; 32];
    let witness_vk = [2u8; 32];
    let mut home = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
    assert_eq!(speak_pair(&mut home, &a, &b, home_vk), Outcome::GermPending);
    assert_eq!(speak_pair(&mut home, &a, &b, [3; 32]), Outcome::GermPending);
    home.court_stroke();
    let spoken =
        home.provisional.scoped().next().unwrap().1.clone();
    let inv = crate::transducer::stalk::invariant_of(&spoken);
    let offered = spoken.address();

    let mut witness = Kernel::new();
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(fetched(&mut witness, &spoken, home_vk, witness_vk, &mut cover), Glue::Admitted));
    let cx = witness.complexes().get(&inv.key()).expect("the class has a complex");
    assert_eq!(
        cx.sections.get(home_vk.as_slice()).map(|s| s.addr.clone()),
        Some(offered.clone()),
        "the offered form is the HOME's section"
    );
    assert!(
        cx.sections.get(witness_vk.as_slice()).is_none(),
        "and is not this court's own reading, which it has not made"
    );

    // The old read, kept here so the difference is a measurement and not a
    // claim: gluing as oneself files the home's word under this court's own
    // vantage, and the simplex has one vertex where 13.9 needs two.
    let mut confused = Kernel::new();
    let mut cover = crate::network::nerve::Cover::new();
    let _ = fetched(&mut confused, &spoken, witness_vk, witness_vk, &mut cover);
    let cx = confused.complexes().get(&inv.key()).expect("the class has a complex");
    assert_eq!(
        cx.sections.get(witness_vk.as_slice()).map(|s| s.addr.clone()),
        Some(offered),
        "one vantage: the court holds the offer as its own re-derivation"
    );
    assert!(cx.sections.get(home_vk.as_slice()).is_none(), "and the home has no section at all");
}

/// 47.3, 1.7: A WITNESS NEVER ANSWERS WITH THE BYTES IT WAS HANDED. The
/// second vantage a home installs on must be a form THIS court reached on
/// its own tapes. `glue_fetched`'s `Admitted` arm files the spoken tape at
/// its own address, so reading that address back returns the offer verbatim
/// — and a home comparing that by `same_vertex` compares the form with
/// itself and converges every time. Chapter 1.7 names the failure: "it can
/// only confirm that a voice sent bytes it had seen before, which is byte
/// exactness, the one failure this architecture cannot absorb."
#[test]
fn a_witness_that_rederived_nothing_answers_with_nothing() {
    let home_vk = [1u8; 32];
    let witness_vk = [2u8; 32];
    let mut home = Kernel::new();
    home.bind_identity(home_vk);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
    assert_eq!(speak_pair(&mut home, &a, &b, home_vk), Outcome::GermPending);
    assert_eq!(speak_pair(&mut home, &a, &b, [3; 32]), Outcome::GermPending);
    home.court_stroke();
    let spoken = home.provisional.scoped().next().unwrap().1.clone();
    let inv = crate::transducer::stalk::invariant_of(&spoken);
    let offered = spoken.address();

    // A witness that has done no work of its own on this class.
    let mut witness = Kernel::new();
    witness.bind_identity(witness_vk);
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(
        fetched(&mut witness, &spoken, home_vk, witness_vk, &mut cover),
        Glue::Admitted
    ));

    // The offered address now RESOLVES on the witness — the glue filed the
    // home's own tape there — which is exactly why it must not be the answer.
    let filed = witness.word_at(&offered).expect("the glue filed the offer at its address");
    assert!(
        filed.same_vertex(&spoken),
        "and what it filed is the home's own word, byte for byte the offer"
    );

    // The second vantage is this court's OWN section, and a court that has
    // re-derived nothing on the class holds none. This is the answer that
    // travels: unmeasured, which is not agreement.
    assert!(
        witness.own_section(&inv).is_none(),
        "a court that has re-derived nothing on the class holds no section of its own"
    );
    assert!(
        witness.complexes().get(inv.key().as_bytes()).is_some_and(|cx| cx
            .sections
            .get(home_vk.as_slice())
            .is_some()),
        "the offer was filed at the HOME's vantage, which is where it belongs"
    );
}

/// 13.9, 13.10: A COURT THAT RULES ITS OWN WORD HOLDS A SECTION AT ITS OWN
/// VANTAGE — and that is the first vantage every later comparison needs.
/// `Complex::set_section` had one production caller, always at the incoming
/// peer's key, so `self_rep` in `glue_fetched` was always None: 13.9's
/// difference on vertices, its automorphism check, its connecting image and
/// 13.10's whole ruling table were unreachable, and every glue returned
/// `Admitted` for want of anything to compare against.
///
/// A word a PEER sent is not this court's own reading, however it is ruled;
/// sectioning that at this vantage would file the peer's tape as this
/// court's re-derivation, which is 1.7's byte exactness one level down.
#[test]
fn a_court_sections_what_it_ruled_itself_and_not_what_a_peer_sent() {
    let me = [0x0au8; 32];
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];

    // ITS OWN: perceived here, replayed here, ruled here on the stroke.
    let mut own = Kernel::new();
    own.bind_identity(me);
    assert_eq!(speak_pair(&mut own, &a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(speak_pair(&mut own, &a, &b, [2; 32]), Outcome::GermPending);
    let spoken = own.provisional.scoped().next().unwrap().1.clone();
    let inv = crate::transducer::stalk::invariant_of(&spoken);
    assert!(
        own.own_section(&inv).is_none(),
        "a provisional is not yet a section: the court has not ruled it"
    );
    // 78.153: every stroke the docket's replay takes.
    own.court_settle();
    let mine = own
        .own_section(&inv)
        .expect("the stroke ruled it, so this court holds it at its own vantage");
    assert!(
        mine.same_vertex(&spoken),
        "and the section is the word it reached, by vertex (M1), never by bytes"
    );

    // A PEER'S: the same word, arriving by glue. It is ruled into the
    // cover, and it is filed at the PEER's vantage, never at this one.
    let mut heard = Kernel::new();
    heard.bind_identity(me);
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(
        fetched(&mut heard, &spoken, [0x0b; 32], me, &mut cover),
        Glue::Admitted
    ));
    assert!(
        heard.ruled_have(&inv).iter().any(|h| h.as_slice() == spoken.address().as_bytes()),
        "the glued word did join the ruled cover"
    );
    assert!(
        heard.own_section(&inv).is_none(),
        "but a word a peer sent is not this court's own re-derivation"
    );
}
