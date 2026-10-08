// 78.145 — ACT_SEAT's wire: every sub-kind reads back whole, a byte left over
// is no frame, a BigQ past 64 bits crosses exactly, and the fixed acts
// decode into the exact values meant with their signatures verifying —
// conformance by decoded values and signature, never by bytes (Concordance
// 1.4, law 2).

use crate::anastome::dialogue::seat::*;
use crate::anastome::dialogue::ACT_SEAT;
use ed25519_dalek::SigningKey;
use num_bigint::{BigInt, BigUint};

fn house() -> SigningKey {
    SigningKey::from_bytes(&[1; 32])
}
fn mind() -> SigningKey {
    SigningKey::from_bytes(&[2; 32])
}
const DOMAIN: [u8; 32] = [9; 32];
const SEAT: u32 = 7;

/// 2^70 / 3.
fn big() -> Arg {
    Arg::big_q(BigInt::from(1u8) << 70usize, BigUint::from(3u8)).expect("a positive denominator")
}

fn call_args() -> Vec<Arg> {
    vec![Arg::Q { num: 3, den: 2 }, big(), Arg::Key([5; 32])]
}

/// The golden CALL: the mind at seat 7 calls fn 3 with `[Q 3/2, BigQ 2^70/3,
/// Key [5;32]]`, call nonce `[3;32]`.
fn golden_call() -> SignedPost {
    let m = mind().verifying_key().to_bytes();
    Post::Call(Call { domain: DOMAIN, seat: SEAT, mind: m, nonce: [3; 32], fn_id: 3, args: call_args() }).sign(&mind()).expect("writes")
}

/// The golden PERCEPT: the house, at local time Q 12/1 and seq 1, counsels
/// the mind to call fn 3 with the same arguments, worth Q 1/2, cost Q 1/3.
fn golden_percept() -> SignedPost {
    let m = mind().verifying_key().to_bytes();
    let counsel = Record::Counsel { fn_id: 3, args: call_args(), worth: Arg::Q { num: 1, den: 2 }, cost: Arg::Q { num: 1, den: 3 } };
    let p = Percept { domain: DOMAIN, seat: SEAT, mind: m, local_time: Arg::Q { num: 12, den: 1 }, seq: 1, records: vec![counsel] };
    Post::Percept(p).sign(&house()).expect("writes")
}

/// The frame read back as the post it carries, or a failure naming why.
fn decoded(frame: &[u8]) -> SignedPost {
    match decode_seat_request(frame) {
        Some(Ok(SeatRequest::Post(s))) => s,
        other => panic!("the frame does not decode to a post: {other:?}"),
    }
}

/// CONFORMANCE BY DECODED VALUES AND SIGNATURE (Concordance 1.4, law 2): the
/// fixed acts — seeds [1;32] the house and [2;32] the mind, domain [9;32],
/// seat 7 — decode into exactly the typed values meant (the rationals equal
/// exactly, the keys equal, the records and counsel as structures), and each
/// signature verifies under its signer's key. No kernel is judged by
/// reproducing bytes.
#[test]
fn the_fixed_acts_decode_to_their_values_and_verify() {
    let m = mind().verifying_key().to_bytes();
    let h = house().verifying_key().to_bytes();
    let call = decoded(&encode_post(&golden_call()).expect("writes"));
    let Post::Call(c) = &call.post else { panic!("a call: {:?}", call.post) };
    assert_eq!((c.domain, c.seat, c.mind, c.nonce, c.fn_id), (DOMAIN, SEAT, m, [3; 32], 3));
    assert_eq!(c.args.len(), 3);
    assert_eq!(c.args[0], Arg::Q { num: 3, den: 2 }, "Q 3/2, exactly");
    let Arg::BigQ { num, den } = &c.args[1] else { panic!("a BigQ: {:?}", c.args[1]) };
    assert_eq!((num.clone(), den.clone()), (BigInt::from(1u8) << 70usize, BigUint::from(3u8)), "2^70/3, exactly");
    assert_eq!(c.args[2], Arg::Key([5; 32]));
    assert!(call.signed_by(&m), "the call verifies under the mind's key");
    assert!(!call.signed_by(&h), "and under no other");
    // The reference cites exactly the signed bytes: `b"mycelium seat v1" ‖
    // [sub] ‖ [fields]`, the act tag unsigned.
    assert_eq!(call.reference(), call.post.signed_bytes().map(|b| crate::chronometry::payload_hash(&b)));
    let percept = decoded(&encode_post(&golden_percept()).expect("writes"));
    let Post::Percept(p) = &percept.post else { panic!("a percept: {:?}", percept.post) };
    assert_eq!((p.domain, p.seat, p.mind, p.seq), (DOMAIN, SEAT, m, 1));
    assert_eq!(p.local_time, Arg::Q { num: 12, den: 1 });
    assert_eq!(
        p.records,
        vec![Record::Counsel { fn_id: 3, args: call_args(), worth: Arg::Q { num: 1, den: 2 }, cost: Arg::Q { num: 1, den: 3 } }],
        "one counsel, as a structure"
    );
    assert!(percept.signed_by(&h), "the percept verifies under the house's key");
}

fn round_trip(s: &SignedPost) {
    let frame = encode_post(s).expect("writes");
    assert_eq!(frame[0], ACT_SEAT);
    assert_eq!(decode_seat_request(&frame), Some(Ok(SeatRequest::Post(s.clone()))));
    let mut longer = frame.clone();
    longer.push(0);
    assert_eq!(decode_seat_request(&longer), Some(Err(Refusal::Malformed)), "a byte left over is no frame");
    assert_eq!(decode_seat_request(&frame[..frame.len() - 1]), Some(Err(Refusal::Malformed)), "nor a byte short");
    // The body the relay holds reads back as the post.
    assert_eq!(SignedPost::of_body(s.post.sub(), &s.body().expect("writes")).as_ref(), Some(s));
}

/// EVERY SUB-KIND READS BACK WHOLE, and only whole.
#[test]
fn every_seat_post_reads_back_whole() {
    let m = mind().verifying_key().to_bytes();
    let seats = Seats {
        domain: DOMAIN,
        epoch: 4,
        seats: vec![
            Seat { seat: SEAT, role: Role::Steward, on: [6; 32], beat: Arg::Q { num: 1, den: 60 }, fns: vec![SeatFn { id: 3, tags: vec![0, 5, 1] }] },
            Seat { seat: 8, role: Role::Office(9), on: [6; 32], beat: big(), fns: Vec::new() },
            Seat { seat: 1, role: Role::Ancients, on: [1; 32], beat: Arg::Q { num: 1, den: 1 }, fns: Vec::new() },
        ],
    };
    round_trip(&Post::Seats(seats).sign(&house()).expect("writes"));
    round_trip(&Post::Take(Take { domain: DOMAIN, seat: SEAT, mind: m, nonce: [4; 32] }).sign(&mind()).expect("writes"));
    round_trip(&Post::Taken(Taken { domain: DOMAIN, seat: SEAT, mind: m, ok: false, reason: 513 }).sign(&house()).expect("writes"));
    round_trip(&golden_percept());
    let plain = Percept {
        domain: DOMAIN,
        seat: SEAT,
        mind: m,
        local_time: big(),
        seq: 9,
        records: vec![Record::Plain { kind: 2, args: vec![Arg::Count(40), Arg::Flag(true), Arg::Mv(crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(BigInt::from(3u8))))] }],
    };
    round_trip(&Post::Percept(plain).sign(&house()).expect("writes"));
    round_trip(&golden_call());
    let call_ref = golden_call().reference().expect("writes");
    round_trip(&Post::Outcome(Outcome { domain: DOMAIN, seat: SEAT, mind: m, call: call_ref, applied: true, reason: 0, deed: [8; 32] }).sign(&house()).expect("writes"));
    // Reads.
    let listing = encode_read_listing(&DOMAIN);
    assert_eq!(decode_seat_request(&listing), Some(Ok(SeatRequest::ReadListing(DOMAIN))));
    let r = ReadMail::sign(&mind(), 17, [5; 32]);
    assert!(r.verified());
    assert_eq!(decode_seat_request(&encode_read_mail(&r)), Some(Ok(SeatRequest::ReadMail(r.clone()))));
    let mut longer = encode_read_mail(&r);
    longer.push(1);
    assert_eq!(decode_seat_request(&longer), Some(Err(Refusal::Malformed)));
    assert_eq!(decode_seat_request(&[0x3C, 0]), None, "another act is not a seat frame");
}

/// A BIGQ PAST 64 BITS crosses exactly; an unreduced, zero-denominator or
/// non-minimal one is refused; a flag is 0 or 1.
#[test]
fn a_big_rational_crosses_exactly_and_only_reduced() {
    let mut out = Vec::new();
    big().put(&mut out).expect("writes");
    let mut p = 0;
    assert_eq!(Arg::take(&out, &mut p), Some(big()));
    assert_eq!(p, out.len());
    let huge = Arg::big_q(-(BigInt::from(7u8) << 200usize), BigUint::from(5u8)).expect("reduced");
    let mut out = Vec::new();
    huge.put(&mut out).expect("writes");
    let mut p = 0;
    assert_eq!(Arg::take(&out, &mut p), Some(huge));
    // 4/2 written by hand: unreduced, refused.
    let unreduced = [5u8, 1, 0, 4, 1, 0, 2];
    assert_eq!(Arg::take(&unreduced, &mut 0), None);
    let zero_den = [5u8, 1, 0, 4, 1, 0, 0];
    assert_eq!(Arg::take(&zero_den, &mut 0), None);
    // 1/3 with a padded numerator: not minimal, refused.
    let padded = [5u8, 2, 0, 1, 0, 1, 0, 3];
    assert_eq!(Arg::take(&padded, &mut 0), None);
    assert_eq!(Arg::take(&[0u8, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], &mut 0), None, "a Q over zero");
    assert_eq!(Arg::take(&[3u8, 2], &mut 0), None, "a flag is 0 or 1");
    assert!(Arg::big_q(BigInt::from(1u8), BigUint::from(0u8)).is_none());
}

/// 78.146 — ARG TAG 6, BYTES: `[6][len u16][bytes]`, read back whole and only
/// whole; a length past the run is refused; the empty run crosses.
#[test]
fn a_bytes_argument_crosses_whole() {
    for run in [Vec::new(), vec![0xA5; 5], vec![0x5A; 65_535]] {
        let a = Arg::Bytes(run.clone());
        let mut out = Vec::new();
        a.put(&mut out).expect("writes");
        let mut p = 0;
        assert_eq!(Arg::take(&out, &mut p), Some(a));
        assert_eq!(p, out.len());
        assert_eq!(Arg::take(&out[..out.len() - 1], &mut 0), None, "a byte short is no argument");
    }
    assert!(Arg::Bytes(vec![0; 65_536]).put(&mut Vec::new()).is_none(), "past the u16 wall it is not written");
    assert_eq!(Arg::take(&[6u8, 3, 0, 1, 2], &mut 0), None, "a length past the run");
}

/// 78.146 — A BYTES PERCEPT, BY ITS DECODED VALUES: the house at local time
/// Q 12/1, seq 2, one record of kind 4 carrying `Bytes [0xA5; 5]` (a task's
/// id) decodes into exactly that record, and its signature verifies under the
/// house's key (Concordance 1.4).
#[test]
fn a_bytes_percept_decodes_to_its_values_and_verifies() {
    let m = mind().verifying_key().to_bytes();
    let record = Record::Plain { kind: 4, args: vec![Arg::Bytes(vec![0xA5; 5])] };
    let p = Percept { domain: DOMAIN, seat: SEAT, mind: m, local_time: Arg::Q { num: 12, den: 1 }, seq: 2, records: vec![record.clone()] };
    let s = Post::Percept(p).sign(&house()).expect("writes");
    let back = decoded(&encode_post(&s).expect("writes"));
    let Post::Percept(read) = &back.post else { panic!("a percept: {:?}", back.post) };
    assert_eq!((read.seq, read.local_time.clone(), read.records.clone()), (2, Arg::Q { num: 12, den: 1 }, vec![record]));
    assert!(back.signed_by(&house().verifying_key().to_bytes()));
    round_trip(&s);
}

/// THE REPLIES: a post's `[1][seq]` or `[0][reason]`; the mail's count and
/// pieces, its two-byte refusal told from an empty box; the listing or none.
#[test]
fn the_replies_read_back() {
    assert_eq!(decode_post_reply(&encode_post_reply(Ok(42))), Some(Ok(42)));
    for why in [Refusal::Unsigned, Refusal::NotTheHouse, Refusal::Malformed, Refusal::UnknownDomain] {
        assert_eq!(decode_post_reply(&encode_post_reply(Err(why))), Some(Err(why)));
        assert_eq!(decode_mail_reply(&encode_mail_reply(Err(why)).expect("writes")), Some(Err(why)));
    }
    let empty = encode_mail_reply(Ok(&[])).expect("writes");
    assert_eq!(empty, vec![0, 0]);
    assert_eq!(decode_mail_reply(&empty), Some(Ok(Vec::new())));
    let call = golden_call();
    let mail = vec![Mail { seq: 3, sub: SUB_CALL, body: call.body().expect("writes") }];
    let reply = encode_mail_reply(Ok(&mail)).expect("writes");
    let read = decode_mail_reply(&reply).expect("reads").expect("mail");
    assert_eq!(read, mail);
    assert_eq!(read[0].post(), Some(call));
    assert_eq!(decode_listing_reply(&[]), Some(None));
}
