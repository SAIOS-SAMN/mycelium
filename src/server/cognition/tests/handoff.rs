// 78.126 — 2(b) MATTER HANDOFFS: matter and a word crossing from domain X to
// domain Y on one court — offered, accepted whole, refused, expired — each
// side consumed by exactly one fold, a double-spend refused, a reversal
// un-consuming, and the handoff let go once final folds consumed both sides.

use crate::anastome::dialogue::challenge::{line_head, Challenge, Challenged, Entry};
use crate::anastome::dialogue::handoff::{Handed, Handoff, HandoffRefusal, Mouth, Offer, Side, State, Unbalanced};
use crate::anastome::dialogue::uplink::{Discontinuous, Net, SideRefusal, TierFold, Uplinked};
use crate::chronometry::{ActKind, Stamp};
use crate::physics::laws::CHALLENGE_WINDOW;
use crate::server::cognition::Kernel;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

fn who(b: u8) -> [u8; 32] {
    [b; 32]
}

/// X's tier (and a second of X's), and Y's.
fn x() -> SigningKey {
    key(21)
}
fn x2() -> SigningKey {
    key(22)
}
fn y() -> SigningKey {
    key(31)
}

/// Two domains on one court: X names `x` and `x2`, Y names `y`; their
/// reserves `x` and `y` (the ancients) have opened `the_mouth()` between them
/// — 78.128: nothing crosses between domains but through an open mouth.
fn founded() -> Kernel {
    use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
    let mut k = Kernel::new();
    let mut xs = vec![vk(&x()), vk(&x2())];
    xs.sort_unstable();
    for (d, tiers, reserve) in [(key(4), xs, x()), (key(5), vec![vk(&y())], y())] {
        let g = DomainGenesis { domain: vk(&d), tiers };
        assert!(matches!(k.admit_domain(&g.sign(&d)), Founded::Sealed { .. }));
        let r = Reservation { domain: vk(&d), tier: vk(&reserve), nonce: [0; 32] };
        assert!(matches!(k.designate_reserve(&r.sign(&d)), Founded::Reserved { now: true, .. }));
    }
    // The throat's bank (78.135), funded first.
    let anchor = k.anchor();
    k.treasury.genesis(vk(&x()), [1, 0, 0, 0], anchor);
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(the_mouth())).sign(&x())), Handed::Sealed { .. }));
    assert!(matches!(k.handoff(&Handoff::Accept { id: the_mouth().id() }.sign(&y())), Handed::Sealed { state: State::Accepted, .. }));
    k
}

/// X's black-hole mouth and Y's white-hole mouth, kept by X, toll-free.
fn the_mouth() -> Mouth {
    let one = crate::transducer::element::identity_versor();
    Mouth { from_tier: vk(&x()), to_tier: vk(&y()), keeper: vk(&x()), from_frame: one.clone(), to_frame: one, toll: [0; 4], nonce: [0; 32], tariff: None, tube: Some(crate::anastome::dialogue::handoff::Tube { mass: (1, 1), from_r: (1, 1), to_r: (1, 1), supersedes: None }) }
}

/// The offer through the mouth.
fn through(o: Offer) -> Handoff {
    Handoff::Traverse { offer: Box::new(o), mouth: the_mouth().id() }
}

/// Five of matter and one word, from X's `w1` to Y's `w8`.
fn cargo() -> Offer {
    Offer {
        from_tier: vk(&x()),
        to_tier: vk(&y()),
        out_nets: vec![(who(1), [-5, 0, 0, -1])],
        in_nets: vec![(who(8), [5, 0, 0, 1])],
        charter: [0; 32],
        nonce: [1; 32],
    }
}

fn offered(k: &mut Kernel, o: Offer) -> [u8; 32] {
    match k.handoff(&through(o).sign(&x())) {
        Handed::Sealed { id, state: State::Offered { .. }, .. } => id,
        other => panic!("offered: {other:?}"),
    }
}

fn fold(tier: &SigningKey, prefix: u8, rows: Vec<Net>, handoffs: Vec<([u8; 32], Side)>) -> TierFold {
    TierFold { tier: vk(tier), prefix: [prefix; 32], rows, declared: Vec::new(), children: Vec::new(), handoffs }
}

fn pass(k: &mut Kernel, h: u64) {
    while k.chain.head_height() < h {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
    }
}

/// AN OFFER CONSERVES ACROSS THE BOUNDARY OR IS NO HANDOFF: G or T moving,
/// out and in not summing to zero, a side running backward, an unordered
/// list, another tier filing, both tiers of one domain — each refused.
#[test]
fn an_offer_conserves_across_the_boundary_on_matter_and_words_only() {
    let mut k = founded();
    let refused = |k: &mut Kernel, o: Offer, by: &SigningKey| match k.handoff(&through(o).sign(by)) {
        Handed::Refused(r) => r,
        other => panic!("refused: {other:?}"),
    };
    let value = Offer { out_nets: vec![(who(1), [0, -5, 0, 0])], in_nets: vec![(who(8), [0, 5, 0, 0])], ..cargo() };
    assert_eq!(refused(&mut k, value, &x()), HandoffRefusal::Unbalanced(Unbalanced::Lane { lane: 1 }), "value crosses by rates and μ");
    let made = Offer { in_nets: vec![(who(8), [6, 0, 0, 1])], ..cargo() };
    assert_eq!(refused(&mut k, made, &x()), HandoffRefusal::Unbalanced(Unbalanced::Unconserved { lane: 0 }));
    let backward = Offer { out_nets: vec![(who(1), [5, 0, 0, -1])], in_nets: vec![(who(8), [-5, 0, 0, 1])], ..cargo() };
    assert_eq!(refused(&mut k, backward, &x()), HandoffRefusal::Unbalanced(Unbalanced::Backward { lane: 0 }));
    let unordered = Offer { out_nets: vec![(who(2), [-1, 0, 0, 0]), (who(1), [-4, 0, 0, -1])], ..cargo() };
    assert_eq!(refused(&mut k, unordered, &x()), HandoffRefusal::Unbalanced(Unbalanced::Unordered));
    assert_eq!(refused(&mut k, cargo(), &y()), HandoffRefusal::NotTheTier, "the sender files the offer");
    let at_home = Offer { to_tier: vk(&x2()), ..cargo() };
    assert_eq!(refused(&mut k, at_home.clone(), &x()), HandoffRefusal::NotTheMouth, "within a domain only along its own road (78.136)");
    assert!(
        matches!(k.handoff(&Handoff::Offer(Box::new(at_home)).sign(&x())), Handed::Refused(HandoffRefusal::Unbound)),
        "a plain offer within one domain is no handoff"
    );
    assert!(
        matches!(k.handoff(&Handoff::Offer(Box::new(cargo())).sign(&x())), Handed::Refused(HandoffRefusal::NoMouth)),
        "domains share no space: nothing crosses but through a mouth"
    );
    let first = k.handoff(&through(cargo()).sign(&x()));
    let Handed::Sealed { id, .. } = first else { panic!("offered") };
    let height = k.chain.head_height();
    assert_eq!(k.handoff(&through(cargo()).sign(&x())), first, "a resent offer: its original receipt");
    assert_eq!(k.chain.head_height(), height, "nothing sealed twice");
    assert_eq!(k.handoff_read(&id), Some((State::Offered { until: k.chain.head_height() + CHALLENGE_WINDOW }, false, false)));
}

/// ACCEPTED WHOLE, EACH SIDE CONSUMED ONCE: X's fold consumes OUT — matter
/// and the word leaving balance its rows; without the side the same rows are
/// value from nowhere. Y cannot consume IN before accepting; accepted, Y's
/// fold consumes IN; a second consumption is a double-spend, refused. The
/// standing restores.
#[test]
fn a_handoff_accepted_is_consumed_once_on_each_side() {
    let mut k = founded();
    let id = offered(&mut k, cargo());
    let leaving = vec![(who(1), [-5, 0, 0, -1])];
    assert!(matches!(k.admit_uplink(&fold(&x(), 1, leaving.clone(), Vec::new()).sign(&x())), Uplinked::Discontinuous(Discontinuous::Unbalanced { .. })));
    assert!(matches!(k.admit_uplink(&fold(&x(), 1, leaving.clone(), vec![(id, Side::Out)]).sign(&x())), Uplinked::Admitted { .. }));
    let arriving = vec![(who(8), [5, 0, 0, 1])];
    assert_eq!(
        k.admit_uplink(&fold(&y(), 1, arriving.clone(), vec![(id, Side::In)]).sign(&y())),
        Uplinked::HandoffRefused { index: 0, why: SideRefusal::NotYet }
    );
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&y())), Handed::Sealed { state: State::Accepted, .. }));
    assert!(matches!(k.handoff(&Handoff::Refuse { id }.sign(&y())), Handed::Refused(HandoffRefusal::Answered)));
    assert!(matches!(k.admit_uplink(&fold(&y(), 1, arriving.clone(), vec![(id, Side::In)]).sign(&y())), Uplinked::Admitted { .. }));
    assert_eq!(
        k.admit_uplink(&fold(&y(), 2, arriving, vec![(id, Side::In)]).sign(&y())),
        Uplinked::HandoffRefused { index: 0, why: SideRefusal::Consumed },
        "a double-spend"
    );
    assert_eq!(
        k.admit_uplink(&fold(&x(), 2, leaving, vec![(id, Side::Out)]).sign(&x())),
        Uplinked::HandoffRefused { index: 0, why: SideRefusal::Consumed }
    );
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.handoff_read(&id), Some((State::Accepted, true, true)));
}

/// REFUSED OR EXPIRED, IT RETURNS TO THE SENDER: refused by Y, X's fold
/// consumes both sides — what left, come back — and Y's IN is not Y's.
/// Unanswered, expiry is the sender's to claim, only past the window, and
/// the receiver answers only within it.
#[test]
fn a_handoff_refused_or_expired_returns_to_the_sender() {
    let mut k = founded();
    let id = offered(&mut k, cargo());
    assert!(matches!(k.handoff(&Handoff::Refuse { id }.sign(&y())), Handed::Sealed { state: State::Returned, .. }));
    assert_eq!(
        k.admit_uplink(&fold(&y(), 1, vec![(who(8), [5, 0, 0, 1])], vec![(id, Side::In)]).sign(&y())),
        Uplinked::HandoffRefused { index: 0, why: SideRefusal::NotYours }
    );
    let both = fold(&x(), 1, Vec::new(), vec![(id, Side::Out), (id, Side::In)]);
    assert!(matches!(k.admit_uplink(&both.sign(&x())), Uplinked::Admitted { .. }), "out and back: nothing moved");

    let mut k = founded();
    let id = offered(&mut k, cargo());
    assert!(matches!(k.handoff(&Handoff::Expire { id }.sign(&x())), Handed::Refused(HandoffRefusal::Window)), "not yet");
    let now = k.chain.head_height();
    pass(&mut k, now + CHALLENGE_WINDOW);
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&y())), Handed::Refused(HandoffRefusal::Window)), "too late");
    assert!(matches!(k.handoff(&Handoff::Expire { id }.sign(&y())), Handed::Refused(HandoffRefusal::NotTheTier)));
    assert!(matches!(k.handoff(&Handoff::Expire { id }.sign(&x())), Handed::Sealed { state: State::Returned, .. }));
}

/// A REVERSED FOLD UN-CONSUMES ITS SIDE, AND FINAL FOLDS LET THE HANDOFF GO:
/// X's fold consuming OUT lies on its line — falsified, the side is open
/// again and X refiles it honestly (its line declaring the crossing, which
/// a replay reads); once both consuming folds read final and are pruned,
/// the handoff is let go at the orbit's close.
#[test]
fn a_reversed_fold_unconsumes_its_side_and_final_folds_let_the_handoff_go() {
    let mut k = founded();
    let id = offered(&mut k, cargo());
    let leaving = vec![(who(1), [-5, 0, 0, -1])];
    let lying_line = vec![Entry { rows: vec![(who(1), [-4, 0, 0, -1]), (who(2), [-1, 0, 0, 0])], declared: leaving.clone() }];
    let lie = TierFold { prefix: line_head([0; 32], &lying_line), ..fold(&x(), 0, leaving.clone(), vec![(id, Side::Out)]) };
    let Uplinked::Admitted { at: lie_at, .. } = k.admit_uplink(&lie.clone().sign(&x())) else { panic!("admitted") };
    let evidence = |f: &TierFold, at: Stamp, entries: Vec<Entry>| Challenge::Evidence {
        fold: at,
        rows: f.rows.clone(),
        declared: f.declared.clone(),
        children: Vec::new(),
        entries,
    };
    assert!(matches!(k.challenge(&evidence(&lie, lie_at, lying_line).sign(&key(40))), Challenged::Falsified { .. }));
    assert_eq!(k.handoff_read(&id), Some((State::Offered { until: lie_at.h - 1 + CHALLENGE_WINDOW }, false, false)), "OUT open again");
    // Refiled honestly: its line runs on from the falsified fold's head.
    let honest_line = vec![Entry { rows: leaving.clone(), declared: leaving.clone() }];
    let honest = TierFold { prefix: line_head(lie.prefix, &honest_line), ..fold(&x(), 0, leaving, vec![(id, Side::Out)]) };
    let Uplinked::Admitted { at: x_at, .. } = k.admit_uplink(&honest.clone().sign(&x())) else { panic!("refiled") };
    assert!(matches!(k.challenge(&evidence(&honest, x_at, honest_line).sign(&key(40))), Challenged::Upheld { .. }), "the crossing replays");
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&y())), Handed::Sealed { .. }));
    let Uplinked::Admitted { at: y_at, .. } = k.admit_uplink(&fold(&y(), 1, vec![(who(8), [5, 0, 0, 1])], vec![(id, Side::In)]).sign(&y())) else {
        panic!("consumed")
    };
    pass(&mut k, y_at.h + CHALLENGE_WINDOW);
    let _ = k.orbit();
    assert_eq!(k.handoff_read(&id), None, "both sides consumed by final folds: let go");
    assert!(k.handoffs.consumed_by.is_empty());
}
