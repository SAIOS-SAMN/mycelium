// 47.1 — the receipt section rides behind a FRAMED cover digest, so its
// start is derived and never guessed at.

use crate::anastome::dialogue::*;

/// The receipt section must be found INSIDE a real digest frame, at the
/// offset one past the digest's signature, with the stratum-digest reply
/// in front of it exactly as `judge` assembles it. The prefix codec
/// round-tripping on its own proves nothing about that offset, and an
/// offset that is wrong by one byte is silent.
///
/// It also pins what the reader does with the frame's own ambiguity: a
/// cover digest with no stratum-digest reply in front of it parses TWO
/// ways that both consume the buffer exactly -- its chart count and
/// invariant bytes read as a digests reply -- so the reader refuses it
/// rather than inventing the charts the old guess produced. Every
/// sender in the tree prepends the reply, so the refused shape is one
/// nothing sends; the format's inability to say which shape it is, is
/// filed with 5.2c, since only a tag on the frame can settle it.
#[test]
fn the_receipt_section_is_found_after_a_full_digest_frame() {
    use crate::transducer::stalk::Invariant;
    let charts = vec![(0u8, Invariant(vec![1, 2, 3])), (2u8, Invariant(vec![9]))];
    let room = [11u64, 12, 13, 14];
    let cover_roots = [[1u8; 32], [2; 32], [3; 32], [4; 32]];
    let ruled_root = [5u8; 32];
    let vk = [6u8; 32];
    let sig = [7u8; 64];

    let mut chain = crate::chronometry::ReceiptChain::with_vk(vk);
    chain.append(b"an address", [0; 32], Vec::new()).expect("appends");
    let head = chain.receipts()[0].clone();
    let prefix = crate::court::admission::SignedPrefix {
        genesis: [8; 32],
        epoch: 2,
        prev_receipt_hash: head.prev_receipt_hash,
        k: head.step_index,
        height: head.height,
        chart: vec![1, 2, 3, 9],
        payload: ruled_root.to_vec(),
    };
    let rsig = [9u8; 64];
    let digests = vec![crate::transducer::sheaf::StratumDigest {
        inv: Invariant(vec![4, 5]),
        count: 3,
        root: vec![7u8; 12],
    }];

    // The shape `judge` sends: the stratum-digest reply, the cover
    // digest, and the receipt section on the end.
    let mut body = encode_digests_reply(&digests);
    body.extend(encode_cover_digest(
        &charts, &room, &cover_roots, &ruled_root, &vk, "127.0.0.1:7500", &sig,
    ));
    let bare = body.len();
    body.extend(encode_receipt_section(&prefix, &rsig, &head).expect("a prefix that fits frames"));

    let d = decode_cover_digest(&body).expect("digest decodes");
    assert_eq!(
        (d.charts, d.room, d.cover_roots, d.ruled_root, d.vk, d.sig),
        (charts.clone(), room, cover_roots, ruled_root, vk, sig)
    );
    assert_eq!(d.bind, "127.0.0.1:7500");
    assert_eq!(
        cover_digest_end(&body),
        Some(bare),
        "the section begins one past the digest's signature"
    );
    let sec = decode_receipt_section(&body, bare).expect("section");
    assert_eq!((sec.prefix, sec.sig, sec.receipt), (prefix.clone(), rsig, head.clone()));

    // A peer that sends no section still decodes, and reads as absent.
    let short = &body[..bare];
    assert!(decode_cover_digest(short).is_some(), "the digest alone still decodes");
    assert_eq!(cover_digest_end(short), Some(short.len()));
    assert!(
        decode_receipt_section(short, short.len()).is_none(),
        "no section is absent, never a partial admission"
    );

    // A truncated section is refused whole, never admitted in part.
    let cut = &body[..body.len() - 1];
    assert!(
        decode_receipt_section(cut, bare).is_none(),
        "a section that does not parse whole is refused"
    );

    // The unframed shape is genuinely ambiguous and refuses.
    let mut lone = encode_cover_digest(
        &charts, &room, &cover_roots, &ruled_root, &vk, "127.0.0.1:7500", &sig,
    );
    lone.extend(encode_receipt_section(&prefix, &rsig, &head).expect("frames"));
    assert!(
        decode_cover_digest(&lone).is_none(),
        "two readings consume the buffer: the reader refuses rather than guessing"
    );
}

/// 47.1, I33 — A PREFIX THAT DOES NOT FIT THE WIRE IS NOT FRAMED. The
/// section's length is two bytes; a prefix carrying a hypha's whole cover
/// (10,855 axes on the fleet of 2026-09-08) was written with a wrapped
/// length, and every peer refused every digest whole while the hypha
/// believed it spoke. `fits` says so before anything is signed, and
/// `encode_receipt_section` refuses rather than wraps; a prefix of one
/// chart frames and reads back.
#[test]
fn a_prefix_that_does_not_fit_the_wire_is_refused_not_wrapped() {
    let mut chain = crate::chronometry::ReceiptChain::with_vk([6u8; 32]);
    chain.append(b"an address", [0; 32], Vec::new()).expect("appends");
    let head = chain.receipts()[0].clone();
    let prefix = |chart: Vec<u64>| crate::court::admission::SignedPrefix {
        genesis: [8; 32],
        epoch: 2,
        prev_receipt_hash: head.prev_receipt_hash,
        k: head.step_index,
        height: head.height,
        chart,
        payload: b"a root".to_vec(),
    };
    let cover = prefix(vec![7u64; 10_855]);
    assert!(!cover.fits(), "a whole cover does not fit two bytes of length");
    assert!(
        encode_receipt_section(&cover, &[9u8; 64], &head).is_none(),
        "and is not framed with a wrapped length"
    );
    let one = prefix(vec![1, 2, 3, 9]);
    assert!(one.fits(), "one chart fits");
    let framed = encode_receipt_section(&one, &[9u8; 64], &head).expect("frames");
    let back = decode_receipt_section(&framed, 0).expect("and reads back whole");
    assert_eq!(back.prefix, one);
    assert_eq!(receipt_section_end(&framed, 0), Some(framed.len()));
}
