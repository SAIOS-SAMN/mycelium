// 47.1 — the receipt section rides behind a FRAMED cover digest, so its
// start is derived and never guessed at.

use crate::anastome::dialogue::*;

/// The receipt section must be found INSIDE a real digest frame, at the
/// offset one past the digest's signature, with the stratum-digest reply
/// in front of it exactly as `judge` assembles it. The prefix codec
/// round-tripping on its own proves nothing about that offset, and an
/// offset that is wrong by one byte is silent.
///
/// It also pins the frame's ambiguity. While the digest carried the
/// charts, a bare one parsed TWO ways that both consumed the buffer --
/// its chart count and invariant bytes read as a digests reply -- and the
/// reader refused it rather than inventing charts. The digest is now the
/// ROOTS (A4): every field ahead of the bind is fixed-width, so the bare
/// shape has one reading and decodes. 5.2c's tag is still owed for the
/// general case; this is the shape narrowing, not the tag arriving.
#[test]
fn the_receipt_section_is_found_after_a_full_digest_frame() {
    use crate::transducer::stalk::Invariant;
    let room = [11u64, 12, 13, 14];
    let cover_roots = [[1u8; 32], [2; 32], [3; 32], [4; 32]];
    let ruled_root = [5u8; 32];
    let vk = [6u8; 32];
    let sig = [7u8; 64];

    let mut chain = crate::chronometry::ReceiptChain::with_vk(vk);
    chain.append(crate::chronometry::ActKind::List, [0; 4],b"an address", [0; 32], Vec::new()).expect("appends");
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
        inv: Invariant::counts(&[4, 5]),
        count: 3,
        root: vec![7u8; 12],
    }];

    // The shape `judge` sends: the stratum-digest reply, the cover
    // digest, and the receipt section on the end.
    let mut body = encode_digests_reply(&digests);
    body.extend(
        encode_cover_digest(&room, &cover_roots, &ruled_root, &vk, "127.0.0.1:7500", &sig)
            .expect("the digest fits its lengths"),
    );
    let bare = body.len();
    body.extend(encode_receipt_section(&prefix, &rsig, &head).expect("a prefix that fits frames"));

    let d = decode_cover_digest(&body).expect("digest decodes");
    assert_eq!(
        (d.room, d.cover_roots, d.ruled_root, d.vk, d.sig),
        (room, cover_roots, ruled_root, vk, sig)
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

    // The unframed shape: one reading now, since the fields ahead of the
    // bind are fixed-width and no chart count stands at the head.
    let mut lone = encode_cover_digest(&room, &cover_roots, &ruled_root, &vk, "127.0.0.1:7500", &sig)
        .expect("the digest fits its lengths");
    lone.extend(encode_receipt_section(&prefix, &rsig, &head).expect("frames"));
    let alone = decode_cover_digest(&lone).expect("the roots-only digest has one reading");
    assert_eq!((alone.room, alone.cover_roots, alone.ruled_root), (room, cover_roots, ruled_root));
}

/// A4, owed since Stage 8c — THE COVER IS A PAGE. The digest carries the
/// four roots; a peer whose roots moved asks for the charts under them
/// from the last cursor it holds, and the server continues past that
/// cursor. Neither side keeps a position: the coordinate on the wire IS
/// the last chart the asker read (rule 15, S13.3).
///
/// Pinned here: the ask and the page ride the same act and are told apart
/// by shape; the page is served in cursor order whatever order the charts
/// arrive in; a budget that admits two rows says `more`, and resuming from
/// the page's last cursor serves the rest exactly once.
#[test]
fn a_cover_is_read_by_page_from_the_last_cursor_held() {
    use crate::transducer::stalk::Invariant;
    let vk = [6u8; 32];
    // Out of cursor order on purpose: lane 2 before lane 0, and the wide
    // chart before the narrow one inside lane 0.
    let charts = vec![
        (2u8, Invariant::counts(&[9])),
        (0u8, Invariant::counts(&[2, 7, 7])),
        (0u8, Invariant::counts(&[2, 3])),
    ];
    let ask = encode_cover_request(&vk, b"");
    assert_eq!(decode_cover_request(&ask), Some((vk, Vec::new())), "an ask round-trips");
    assert!(decode_cover_page(&ask).is_none(), "an ask is not a page");

    // Two rows of the three: the act byte, the vk, the bind's length and
    // bytes, the count, then per row the lane byte, the key's length byte
    // and the chart's own key, then the `more` byte. A row is measured from
    // `cover_cursor` — the cursor IS the lane and the key — rather than from
    // a width formula, which went stale the moment a coordinate stopped
    // being eight bytes (A21 as ruled 2026-09-18).
    let row = |lane: u8, inv: &Invariant| cover_cursor(lane, inv).len() + 1;
    let two = 1
        + 32
        + 1
        + "127.0.0.1:7500".len()
        + 4
        + row(0, &Invariant::counts(&[2, 3]))
        + row(0, &Invariant::counts(&[2, 7, 7]))
        + 1;
    let page = encode_cover_page(&vk, "127.0.0.1:7500", &charts, b"", two).expect("a page");
    assert!(decode_cover_request(&page).is_none(), "a page is not an ask");
    let (first, more) = decode_cover_page(&page).expect("a page decodes");
    assert_eq!(first.vk, vk);
    assert_eq!(first.bind, "127.0.0.1:7500");
    assert_eq!(
        first.charts,
        vec![(0u8, Invariant::counts(&[2, 3])), (0u8, Invariant::counts(&[2, 7, 7]))],
        "cursor order, not the caller's order"
    );
    assert!(more, "the third chart did not fit and the page says so");

    // Resume from the last cursor read: the rest, once, and no more.
    let (end, _) = first.charts.split_last().expect("a row");
    let last = cover_cursor(end.0, &end.1);
    let rest = encode_cover_page(&vk, "127.0.0.1:7500", &charts, &last, 1 << 16).expect("a page");
    let (tail, more) = decode_cover_page(&rest).expect("a page decodes");
    assert_eq!(tail.charts, vec![(2u8, Invariant::counts(&[9]))], "past the cursor, nothing repeated");
    assert!(!more, "the cover is served");

    // A chart wider than the axis count is passed over, not refused with
    // the page: a refusal would meet it at the head of every later ask.
    let wide = vec![(0u8, Invariant::counts(&[1])), (1u8, Invariant::counts(&[4u64; 256]))];
    let (only, more) = encode_cover_page(&vk, "a", &wide, b"", 1 << 16)
        .as_deref()
        .and_then(decode_cover_page)
        .expect("a page");
    assert_eq!(only.charts, vec![(0u8, Invariant::counts(&[1]))]);
    assert!(!more, "and the page is complete without it");
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
    chain.append(crate::chronometry::ActKind::List, [0; 4],b"an address", [0; 32], Vec::new()).expect("appends");
    let head = chain.receipts()[0].clone();
    let prefix = |chart: Vec<u8>| crate::court::admission::SignedPrefix {
        genesis: [8; 32],
        epoch: 2,
        prev_receipt_hash: head.prev_receipt_hash,
        k: head.step_index,
        height: head.height,
        chart,
        payload: b"a root".to_vec(),
    };
    let cover = prefix(vec![7u8; 86_840]);
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
