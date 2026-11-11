// LAW-COORDINATE v3 (proposal 78.173 §5, fixtures 14 and 15): the passage
// read on the coordinate — the re-origin across a throat and the face
// exchange within one enclosure, by `forms::passage`'s pole swap — and the
// share and the flow invariant under dilation (xylarium's joint fixture).

use super::bodies::*;
use crate::algebra::{base_axis, Clifford, EM};
use crate::coordinate::*;
use crate::passage::{PassageRefusal, PassageState, Poles};

/// Plane 1's poles: `e₊₁` (base axis 1, squaring to +1) and `e₋₁` (base
/// axis 2, squaring to −1), beside the conformal pair's `e₋`.
fn poles() -> Poles {
    let (p, n) = (base_axis(1).expect("e₊₁"), base_axis(2).expect("e₋₁"));
    let sig = EM | n;
    let unit = |g: usize| Clifford::blade(sig, g, q(1, 1)).expect("a generator the algebra holds");
    Poles::of(&unit(p), &unit(n)).expect("two units of opposite sign")
}

/// The passage's keys: S's `e_S1` holds the span, T's `e_T1` the flux.
fn through() -> Through {
    Through { s: on('S', 1), t: on('T', 1) }
}

/// The enclosure past the throat: the root enclosed once, cut at bell 1.
fn past() -> Origin {
    Origin { enclosures: vec![Local { h: 1, lanes: on_s(q(0, 1)), k: vec![0] }], ..root() }
}

/// FIXTURE 14, THE RE-ORIGIN: a body at hold `4/5` and reach `1/5` (span
/// `3/5` on S, T = 0) passes the throat into the enclosure past it. It
/// stands there with `S = −3/5` (`|S|` kept) and `T = 0 − 2·(3/5) = −6/5`,
/// H and `K` untouched; the frame reads reach and hold exchanged (hold
/// `1/5`, reach `4/5`), the whole `H + R = 1` kept, and the face
/// `δ = 4/5 ↦ 1/5`.
#[test]
fn re_origin_reverses_span_and_writes_flux() {
    let body_ = body(&root(), 1, lanes(q(3, 5), q(0, 1), q(0, 1), q(45, 1)), vec![0]);
    let frame = PassageState { hold: q(4, 5), reach: q(1, 5) };
    let passed = body_.pass(&poles(), &frame, &through(), &past()).expect("a lawful passage");
    assert_eq!(passed.body, body(&past(), 1, lanes(q(-3, 5), q(0, 1), q(-6, 5), q(45, 1)), vec![0]));
    assert_eq!(passed.frame, PassageState { hold: q(1, 5), reach: q(4, 5) });
    assert_eq!(passed.frame.whole(), frame.whole());
    assert_eq!((frame.share(), passed.share.clone()), (Ok(q(4, 5)), q(1, 5)));
    assert_eq!(passed.frame.share(), Ok(passed.share));
}

/// FIXTURE 14, WITHIN ONE ENCLOSURE THE PASSAGE IS THE FACE EXCHANGE ALONE:
/// passed into its own origin, the body is unchanged, and only the frame
/// reads reach and hold exchanged, `δ ↦ 1 − δ`.
#[test]
fn passage_within_enclosure_exchanges_face_alone() {
    let body_ = body(&root(), 1, lanes(q(3, 5), q(0, 1), q(0, 1), q(45, 1)), vec![0]);
    let frame = PassageState { hold: q(4, 5), reach: q(1, 5) };
    let passed = body_.pass(&poles(), &frame, &through(), &root()).expect("a lawful passage");
    assert_eq!(passed.body, body_);
    assert_eq!((passed.frame, passed.share), (PassageState { hold: q(1, 5), reach: q(4, 5) }, q(1, 5)));
}

/// FIXTURE 14, A SPAN NOT THE BODY'S S IS REFUSED: the frame state's span
/// `3/5` against `S = 1/2` is refused `SpanDiffers`, naming both; a frame
/// of no whole is refused by the passage's own name.
#[test]
fn re_origin_refuses_other_span() {
    let body_ = body(&root(), 1, on_s(q(1, 2)), vec![0]);
    let frame = PassageState { hold: q(4, 5), reach: q(1, 5) };
    assert_eq!(
        body_.pass(&poles(), &frame, &through(), &past()),
        Err(MotionRefusal::SpanDiffers { held: Box::new(q(1, 2)), read: Box::new(q(3, 5)) })
    );
    let none = PassageState { hold: q(1, 2), reach: q(-1, 2) };
    assert_eq!(body_.pass(&poles(), &none, &through(), &past()), Err(MotionRefusal::Passage(Box::new(PassageRefusal::NoWhole { span: q(1, 1) }))));
}

/// FIXTURE 15, DILATION INVARIANCE: for each `(H, R)` of `(1, 3)`,
/// `(2/7, 5/3)` and `(9/4, 1/60)` and each `λ` of `7/3`, `1/11` and
/// `1000003/2`, the share `δ(λH, λR) = δ(H, R)` (`1/4`, `6/41`, `135/136`),
/// and the flow between the two dilated faces, `δ ⊕ (1 − δ)`, equals the
/// undilated one (`16/19`, `1681/1891`, `18496/18631`).
#[test]
fn share_and_flow_are_invariant_under_dilation() {
    let cases = [(q(1, 1), q(3, 1), q(1, 4), q(16, 19)), (q(2, 7), q(5, 3), q(6, 41), q(1681, 1891)), (q(9, 4), q(1, 60), q(135, 136), q(18496, 18631))];
    for (h, r, delta, flowed) in cases {
        let face = |h: &crate::Q, r: &crate::Q| PassageState { hold: h.clone(), reach: r.clone() }.share().expect("a share");
        assert_eq!(face(&h, &r), delta);
        assert_eq!(flow(&face(&h, &r), &face(&r, &h)), Ok(flowed.clone()));
        for lambda in [q(7, 3), q(1, 11), q(1_000_003, 2)] {
            let (lh, lr) = (&h * &lambda, &r * &lambda);
            assert_eq!(face(&lh, &lr), delta, "λ = {lambda}");
            assert_eq!(flow(&face(&lh, &lr), &face(&lr, &lh)), Ok(flowed.clone()), "λ = {lambda}");
        }
    }
    assert_eq!(flow(&q(1, 1), &q(1, 2)), Err(MotionRefusal::NotPush(Box::new(q(1, 1)))), "a push of 1 is no push");
}
