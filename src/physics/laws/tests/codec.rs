use super::super::*;
use super::common::*;
use crate::transducer::cga::{point_at, sphere_at};

/// 78.64 H4 — THE CONTRACT'S BYTES ARE THE OBJECTS. Every law read back from
/// its coordinate; a transition the kernel sends and the verdict the court
/// answers decode to themselves exactly, so the court verifies what was sent
/// and the kernel reads what was ruled; and a verdict carrying every kind of
/// breach, each law beside it, survives its bytes whole. A truncated payload
/// decodes to nothing, never to a default.
#[test]
fn codec_round_trips_transition_and_verdict() {
    use super::super::codec::{law_at, put_transition, put_verdict, take_transition, take_verdict};
    for law in LAWS {
        assert_eq!(law_at(&law.address()), Some(law));
    }
    let t = Transition {
        motion: turn(),
        heights: (7, 8),
        bodies: carried(&turn()),
        hues: vec![(1, 2), (2, 3)],
        fluxes: vec![
            Flux { flow: Flow::Source, at: point_at(&[q(1), q(2), q(0)]), mass: q(3) },
            Flux { flow: Flow::Sink, at: sphere_at(&[q(0), q(0), q(0)], &q(1)), mass: Q::new(1.into(), 2.into()) },
        ],
    };
    let mut bytes = Vec::new();
    put_transition(&mut bytes, &t);
    let mut p = 0;
    assert_eq!(take_transition(&bytes, &mut p), Some(t.clone()));
    assert_eq!(p, bytes.len(), "every byte read, none left over");
    let v = verify(&t, &mut budget());
    let ruled = Verdict {
        breaches: vec![
            (Law::Conformal, Breach::NotAVersor),
            (Law::Conformal, Breach::Undecomposed(crate::transducer::cga::Undecomposed::Irrational(3))),
            (Law::Conformal, Breach::NotCarried { body: 1 }),
            (Law::Momentum, Breach::MomentumUnread { body: 0 }),
            (Law::Momentum, Breach::MomentumNotConserved { net: turn() }),
            (Law::Rigidity, Breach::NonRigid { i: 0, j: 1, delta: Q::new(5.into(), 4.into()) }),
            (Law::Rigidity, Breach::RigidityUnmeasured { i: 0, j: 2 }),
            (Law::Time, Breach::HeightNotAdvanced { before: 9, after: 9 }),
            (Law::Reservoir, Breach::NotUnitary),
            (Law::Continuity, Breach::MassNotConserved { made: Q::new((-3).into(), 2.into()) }),
            (Law::Continuity, Breach::FluxUnplaced { index: 1 }),
        ],
        unmeasured: vec![
            Unmeasured { law: Law::Conformal, needed_stroke: Some(14) },
            Unmeasured { law: Law::Rigidity, needed_stroke: None },
        ],
        ..v
    };
    let mut bytes = Vec::new();
    put_verdict(&mut bytes, &ruled);
    let mut p = 0;
    assert_eq!(take_verdict(&bytes, &mut p), Some(ruled));
    assert_eq!(p, bytes.len());
    let mut p = 0;
    assert_eq!(take_verdict(&bytes[..bytes.len() - 1], &mut p), None, "a truncated verdict is no verdict");
}

/// 78.62 steps 2 and 3 — THE ACT A KERNEL SPEAKS. The register request and
/// the verify request read back as what they ask; a verify request with a
/// byte left over is no request. The register's reply carries every law at
/// its coordinate with its statement, in the register's order, at the
/// register's version.
#[test]
fn decode_laws_request_reads_register_and_verify() {
    use super::super::codec::{decode_laws_request, decode_register_reply, encode_register_reply, encode_register_request, encode_verify_request, LawsRequest};
    assert_eq!(decode_laws_request(&encode_register_request()), Some(LawsRequest::Register));
    let t = Transition { motion: turn(), heights: (7, 8), bodies: carried(&turn()), hues: Vec::new(), fluxes: Vec::new() };
    let mut asked = encode_verify_request(&t);
    assert_eq!(decode_laws_request(&asked), Some(LawsRequest::Verify(t)));
    asked.push(0);
    assert_eq!(decode_laws_request(&asked), None, "a byte left over is no request");
    let served = decode_register_reply(&encode_register_reply()).expect("the register reads back");
    assert_eq!(served.iter().map(|s| s.law).collect::<Vec<_>>(), LAWS.to_vec());
    assert!(served.iter().all(|s| s.statement == s.law.statement()));
}

/// 78.118 — A BATCH: many transitions in one frame, read back whole and in
/// order; a byte left over is no request; the reply carries each
/// transition's own verdict, in the order sent — the lawful turn admitted,
/// the still height refused on time alone.
#[test]
fn encode_batch_reply_round_trips_verdicts_in_order() {
    use super::super::codec::{decode_batch_reply, decode_laws_request, encode_batch_reply, encode_batch_request, LawsRequest};
    let lawful = Transition { motion: turn(), heights: (7, 8), bodies: carried(&turn()), hues: Vec::new(), fluxes: Vec::new() };
    let still = Transition { heights: (8, 8), ..lawful.clone() };
    let batch = vec![lawful.clone(), still.clone(), lawful];
    let mut asked = encode_batch_request(&batch);
    assert_eq!(decode_laws_request(&asked), Some(LawsRequest::Batch(batch.clone())));
    asked.push(0);
    assert_eq!(decode_laws_request(&asked), None, "a byte left over is no request");
    let verdicts: Vec<Verdict> = batch.iter().map(verify_sized).collect();
    let back = decode_batch_reply(&encode_batch_reply(&verdicts)).expect("the verdicts read back");
    assert_eq!(back, verdicts);
    let laws: Vec<Vec<Law>> = back.iter().map(|v| v.breaches.iter().map(|(l, _)| *l).collect()).collect();
    assert_eq!(laws, vec![vec![], vec![Law::Time], vec![]], "each its own, in order");
}
