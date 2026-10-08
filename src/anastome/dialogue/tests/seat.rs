// 78.145 — ACT_SEAT's wire: every sub-kind reads back whole, a byte left over
// is no frame, a BigQ past 64 bits crosses exactly, and the GOLDEN VECTORS
// other kernels match bytes against.

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

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

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

// Computed independently of this crate (PyNaCl ed25519 and the blake3
// reference) from the layout in 78.145, and matched here byte for byte.
const GOLDEN_CALL: &str = "53040909090909090909090909090909090909090909090909090909090909090909070000008139770ea87d175f56a35466c34c7ecccb8d8a91b4ee37a25df60f5b8fc9b394030303030303030303030303030303030303030303030303030303030303030303000300030000000000000002000000000000000509000000000000000000400100030105050505050505050505050505050505050505050505050505050505050505055e38bf9a10d0ff42b5248d2279f80984d513a18beb9da5925182e1c22f5728e48774e744decd809eb8b2c8e4568683f89441ff95b93ca300b77a6456cd27860f";
const GOLDEN_CALL_REF: &str = "31cb3c49a809b03a09c5b4a57005dcf5399675654167a07819cc4fcd76ff9041";
const GOLDEN_PERCEPT: &str = "53030909090909090909090909090909090909090909090909090909090909090909070000008139770ea87d175f56a35466c34c7ecccb8d8a91b4ee37a25df60f5b8fc9b394000c00000000000000010000000000000001000000000000000100ffff03000300030000000000000002000000000000000509000000000000000000400100030105050505050505050505050505050505050505050505050505050505050505050001000000000000000200000000000000000100000000000000030000000000000038fb1c38b9c4d70d0daccf1fc970bc239d412f8b9e0fba59bc5cdb00e10c123c22e1501bb429cc5f0d2b704547697f99201bf32e2258e999f28824b7640dd903";

/// THE GOLDEN VECTORS: fixed seeds ([1;32] the house, [2;32] the mind), the
/// domain [9;32], seat 7 — the frames' bytes and the call's reference, so a
/// kernel in another language matches bytes against them.
#[test]
fn the_golden_vectors_stand() {
    let call = golden_call();
    let call_frame = encode_post(&call).expect("writes");
    let call_ref = call.reference().expect("writes");
    let percept_frame = encode_post(&golden_percept()).expect("writes");
    eprintln!("house vk   {}", hex(&house().verifying_key().to_bytes()));
    eprintln!("mind vk    {}", hex(&mind().verifying_key().to_bytes()));
    eprintln!("CALL       {}", hex(&call_frame));
    eprintln!("CALL ref   {}", hex(&call_ref));
    eprintln!("PERCEPT    {}", hex(&percept_frame));
    assert_eq!(hex(&call_frame), GOLDEN_CALL);
    assert_eq!(hex(&call_ref), GOLDEN_CALL_REF);
    assert_eq!(hex(&percept_frame), GOLDEN_PERCEPT);
    // The reference is the citation hash of exactly the signed bytes:
    // `b"mycelium seat v1" ‖ [sub] ‖ [fields]`, the act tag unsigned.
    let mut signed = SIGNED.to_vec();
    signed.extend_from_slice(&call_frame[1..call_frame.len() - 64]);
    assert_eq!(call_ref, crate::chronometry::payload_hash(&signed));
    assert!(call.signed_by(&mind().verifying_key().to_bytes()));
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
