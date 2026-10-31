// 78.145 — ACT_SEAT's wire: every sub-kind reads back whole, a byte left over
// is no frame, a BigQ past 64 bits crosses exactly, and the fixed acts
// decode into the exact values meant with the acts' signatures verifying —
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
    Post::Call(Call { domain: DOMAIN, sub_origin: None, seat: SEAT, mind: m, nonce: [3; 32], fn_id: 3, args: call_args() }).sign(&mind()).expect("writes")
}

/// The golden PERCEPT: the house, at local time Q 12/1 and seq 1, counsels
/// the mind to call fn 3 with the same arguments, worth Q 1/2, cost Q 1/3.
fn golden_percept() -> SignedPost {
    let m = mind().verifying_key().to_bytes();
    let counsel = Record::Counsel { fn_id: 3, args: call_args(), worth: Arg::Q { num: 1, den: 2 }, cost: Arg::Q { num: 1, den: 3 } };
    let p = Percept { domain: DOMAIN, sub_origin: None, seat: SEAT, mind: m, local_time: Arg::Q { num: 12, den: 1 }, seq: 1, records: vec![counsel], point: Some(golden_point()) };
    Post::Percept(p).sign(&house()).expect("writes")
}

/// 78.152 — the final point the golden percept is stamped at.
fn golden_point() -> crate::chronometry::tuniversal::TPoint {
    use crate::chronometry::tuniversal::{Origin, TPoint};
    TPoint { origin: Origin { genesis: [6; 32], epoch: 146 }, h: 40_961, lanes: [5, 0, 2, 1], k: 812 }
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
fn fixed_posts_decode_to_values_and_verify() {
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
    assert_eq!(p.point, Some(golden_point()), "the house's final point, exactly, signed with the percept");
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
fn seat_posts_round_trip_whole() {
    let m = mind().verifying_key().to_bytes();
    let seats = Seats {
        domain: DOMAIN,
        sub_origin: None,
        epoch: 4,
        seats: vec![
            Seat { seat: SEAT, role: Role::Steward, on: [6; 32], beat: Arg::Q { num: 1, den: 60 }, fns: vec![SeatFn { id: 3, tags: vec![0, 5, 1] }] },
            Seat { seat: 8, role: Role::Office(9), on: [6; 32], beat: big(), fns: Vec::new() },
            Seat { seat: 1, role: Role::Ancients, on: [1; 32], beat: Arg::Q { num: 1, den: 1 }, fns: Vec::new() },
        ],
    };
    round_trip(&Post::Seats(seats).sign(&house()).expect("writes"));
    round_trip(&Post::Take(Take { domain: DOMAIN, sub_origin: None, seat: SEAT, mind: m, nonce: [4; 32] }).sign(&mind()).expect("writes"));
    round_trip(&Post::Taken(Taken { domain: DOMAIN, sub_origin: None, seat: SEAT, mind: m, ok: false, reason: 513 }).sign(&house()).expect("writes"));
    round_trip(&golden_percept());
    let plain = Percept {
        domain: DOMAIN,
        sub_origin: None,
        seat: SEAT,
        mind: m,
        local_time: big(),
        seq: 9,
        records: vec![Record::Plain { kind: 2, args: vec![Arg::Count(40), Arg::Flag(true), Arg::Mv(crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(BigInt::from(3u8))))] }],
        point: None,
    };
    round_trip(&Post::Percept(plain).sign(&house()).expect("writes"));
    round_trip(&golden_call());
    let call_ref = golden_call().reference().expect("writes");
    round_trip(&Post::Outcome(Outcome { domain: DOMAIN, sub_origin: None, seat: SEAT, mind: m, call: call_ref, applied: true, reason: 0, deed: [8; 32] }).sign(&house()).expect("writes"));
    // Reads.
    let listing = encode_read_listing(&DOMAIN, None);
    assert_eq!(decode_seat_request(&listing), Some(Ok(SeatRequest::ReadListing(DOMAIN, None))));
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
fn arg_big_q_crosses_exactly_only_reduced() {
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
fn arg_bytes_crosses_whole() {
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
fn bytes_percept_decodes_to_values_and_verifies() {
    let m = mind().verifying_key().to_bytes();
    let record = Record::Plain { kind: 4, args: vec![Arg::Bytes(vec![0xA5; 5])] };
    let p = Percept { domain: DOMAIN, sub_origin: None, seat: SEAT, mind: m, local_time: Arg::Q { num: 12, den: 1 }, seq: 2, records: vec![record.clone()], point: None };
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
fn post_and_mail_replies_round_trip() {
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
    assert_eq!(decode_listing_reply(&[], None), Some(None));
}

const SUB: [u8; 32] = [0x5c; 32];

/// SEAT-WIRE v1.4 CONFORMANCE BY DECODED VALUES AND SIGNATURE (Concordance
/// 1.4): every post scoped to the sub-origin `[0x5c;32]` — seeds [1;32] the
/// house and [2;32] the mind, domain [9;32], seat 7 — is written under its
/// sub-kind with the SCOPED bit (0x80), decodes into exactly its values with
/// the sub-origin, and verifies under its signer; the same post unscoped is
/// v1.3's, unchanged; the scope is signed (the bit flipped does not verify);
/// a scoped listing read names its sub-origin and a listing of another scope
/// is not read as it.
#[test]
fn scoped_posts_decode_to_values_and_verify() {
    let m = mind().verifying_key().to_bytes();
    let h = house().verifying_key().to_bytes();
    let posts = [
        (Post::Seats(Seats { domain: DOMAIN, sub_origin: Some(SUB), epoch: 4, seats: vec![Seat { seat: SEAT, role: Role::Steward, on: [6; 32], beat: Arg::Q { num: 1, den: 60 }, fns: vec![SeatFn { id: 3, tags: vec![0, 5] }] }] }), house(), h),
        (Post::Take(Take { domain: DOMAIN, sub_origin: Some(SUB), seat: SEAT, mind: m, nonce: [4; 32] }), mind(), m),
        (Post::Taken(Taken { domain: DOMAIN, sub_origin: Some(SUB), seat: SEAT, mind: m, ok: true, reason: 0 }), house(), h),
        (Post::Percept(Percept { domain: DOMAIN, sub_origin: Some(SUB), seat: SEAT, mind: m, local_time: Arg::Q { num: 7, den: 3 }, seq: 2, records: Vec::new(), point: Some(golden_point()) }), house(), h),
        (Post::Call(Call { domain: DOMAIN, sub_origin: Some(SUB), seat: SEAT, mind: m, nonce: [3; 32], fn_id: 3, args: call_args() }), mind(), m),
        (Post::Outcome(Outcome { domain: DOMAIN, sub_origin: Some(SUB), seat: SEAT, mind: m, call: [7; 32], applied: false, reason: REASON_NOT_APPLIED, deed: [0; 32] }), house(), h),
    ];
    for (i, (post, key, signer)) in posts.into_iter().enumerate() {
        let s = post.clone().sign(&key).expect("writes");
        let frame = encode_post(&s).expect("writes");
        assert_eq!(frame.get(1).copied(), u8::try_from(i).ok().map(|k| k | SCOPED), "sub-kind {i} with the SCOPED bit");
        let back = decoded(&frame);
        assert_eq!(back.post, post, "decodes to exactly its values");
        assert_eq!(back.post.sub_origin(), Some(SUB));
        assert!(back.signed_by(&signer), "verifies under its signer");
        round_trip(&s);
        // The scope is signed: the same fields under the unscoped sub-kind
        // are a different post (they do not even read whole).
        let mut flipped = frame.clone();
        if let Some(b) = flipped.get_mut(1) {
            *b &= !SCOPED;
        }
        let refused = match decode_seat_request(&flipped) {
            Some(Ok(SeatRequest::Post(p))) => !p.signed_by(&signer),
            _ => true,
        };
        assert!(refused, "sub-kind {i}: the scope is bound to the signature");
    }
    // Unscoped, a post is v1.3's: no SCOPED bit, no sub-origin bytes.
    let plain = golden_call();
    let frame = encode_post(&plain).expect("writes");
    assert_eq!(frame.get(1).copied(), Some(SUB_CALL));
    assert_eq!(decoded(&frame).post.sub_origin(), None);
    // A scoped listing read, and its reply read only at its scope.
    let ask = encode_read_listing(&DOMAIN, Some(&SUB));
    assert_eq!(ask.get(1).copied(), Some(SUB_READ_LISTING | SCOPED));
    assert_eq!(decode_seat_request(&ask), Some(Ok(SeatRequest::ReadListing(DOMAIN, Some(SUB)))));
    let listing = Post::Seats(Seats { domain: DOMAIN, sub_origin: Some(SUB), epoch: 1, seats: Vec::new() }).sign(&house()).expect("writes");
    let body = listing.body().expect("writes");
    assert_eq!(decode_listing_reply(&body, Some(&SUB)), Some(Some(listing)));
    assert_eq!(decode_listing_reply(&body, Some(&[1; 32])), None, "another scope's listing is not read as this one");
}

/// SEAT-WIRE v1.4 — READ-MAIL AT THE RELAY'S STANDING (sub-kind 8): the read
/// `[reader][after][epoch][nonce][sig]` reads back whole and verifies under
/// its sub-kind; its reply `[1][epoch][seq][count]…` decodes to exactly the
/// relay's epoch, last sequence and pieces; the refusal is `[0][reason]`;
/// sub-kind 7 is answered as before.
#[test]
fn mail_reply_standing_carries_relay_epoch_and_seq() {
    let r = ReadMail::sign_standing(&mind(), 17, 0xfeed_beef, [5; 32]);
    assert!(r.verified());
    let frame = encode_read_mail(&r);
    assert_eq!(frame.get(1).copied(), Some(SUB_READ_MAIL_STANDING));
    assert_eq!(frame.len(), 2 + 32 + 8 + 8 + 32 + 64);
    assert_eq!(decode_seat_request(&frame), Some(Ok(SeatRequest::ReadMail(r.clone()))));
    assert_eq!(r.standing, Some(0xfeed_beef));
    let mut short = frame.clone();
    short.pop();
    assert_eq!(decode_seat_request(&short), Some(Err(Refusal::Malformed)));
    // The relay honours `after` only under its own epoch and below its seq.
    assert_eq!((r.after_on(0xfeed_beef, 40), r.after_on(1, 40), r.after_on(0xfeed_beef, 16)), (17, 0, 0));
    let call = golden_call();
    let read = MailRead { epoch: 0x0123_4567_89ab_cdef, seq: 41, mail: vec![Mail { seq: 40, sub: SUB_CALL, body: call.body().expect("writes") }] };
    let reply = encode_mail_reply_standing(Ok(&read)).expect("writes");
    assert_eq!(reply.get(..1), Some(&[1u8][..]), "served: the reply's lead byte");
    let back = decode_mail_reply_standing(&reply).expect("reads").expect("served");
    assert_eq!(back, read, "the relay's epoch, last sequence and pieces, exactly");
    assert_eq!(back.mail.iter().map(Mail::post).collect::<Vec<_>>(), vec![Some(call)]);
    let empty = MailRead { epoch: 9, seq: 0, mail: Vec::new() };
    assert_eq!(decode_mail_reply_standing(&encode_mail_reply_standing(Ok(&empty)).expect("writes")), Some(Ok(empty)));
    for why in [Refusal::Unsigned, Refusal::Malformed] {
        assert_eq!(decode_mail_reply_standing(&encode_mail_reply_standing(Err(why)).expect("writes")), Some(Err(why)));
    }
    let mut long = reply.clone();
    long.push(0);
    assert_eq!(decode_mail_reply_standing(&long), None, "a byte past the reply is none");
    // Sub-kind 7 stands as v1.3's.
    let old = ReadMail::sign(&mind(), 17, [5; 32]);
    assert_eq!((encode_read_mail(&old).get(1).copied(), old.standing), (Some(SUB_READ_MAIL), None));
}
