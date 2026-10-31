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
fn decode_receipt_section_reads_past_cover_digest() {
    use crate::transducer::stalk::Invariant;
    let room = [11u64, 12, 13, 14];
    let cover_roots = [[1u8; 32], [2; 32], [3; 32], [4; 32]];
    let ruled_root = [5u8; 32];
    let sig = [7u8; 64];
    // 78.152 — the keeper ring the digest carries: two readings of the
    // digest's own key, parent-linked.
    let keeper = ed25519_dalek::SigningKey::from_bytes(&[6; 32]);
    let vk = keeper.verifying_key().to_bytes();
    let origin = crate::chronometry::tuniversal::Origin { genesis: [8; 32], epoch: 2 };
    let mut keepers = crate::chronometry::tuniversal::Keepers::new(origin, vk);
    for (h, s) in [(4u64, 1u64), (5, 3)] {
        keepers.read_own(&keeper, crate::chronometry::tuniversal::Tick { h, lanes: [s, 0, 0, 0], k: h }).expect("writes");
    }
    let ring = keepers.own_ring();
    assert_eq!(ring.len(), 2);

    let mut chain = crate::chronometry::ReceiptChain::with_signer(vk);
    let head = chain.append(crate::chronometry::ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    let prefix = crate::court::admission::SignedPrefix {
        genesis: [8; 32],
        epoch: 2,
        prev_receipt_hash: head.prev_hash(),
        at: head.stamp.place(),
        height: head.stamp.h,
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
    let mut body = encode_digests_reply(&digests).expect("the digests fit the reply's fields");
    body.extend(
        encode_cover_digest(&room, &cover_roots, &ruled_root, &vk, "127.0.0.1:7500", &ring, &sig)
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
    assert_eq!(d.ring, ring, "the keeper ring, reading for reading");
    let mut other = crate::chronometry::tuniversal::Keepers::new(origin, [1; 32]);
    assert_eq!(other.admit(d.vk, d.ring), Ok(crate::chronometry::tuniversal::Admitted::Anchored), "and it verifies where it lands");
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
    let mut lone = encode_cover_digest(&room, &cover_roots, &ruled_root, &vk, "127.0.0.1:7500", &[], &sig)
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
fn encode_cover_page_resumes_from_last_cursor() {
    use crate::transducer::stalk::Invariant;
    let vk = [6u8; 32];
    // Out of cursor order on purpose: lane 2 before lane 0, and the wide
    // chart before the narrow one inside lane 0.
    let charts = vec![
        (2u8, Invariant::counts(&[9])),
        (0u8, Invariant::counts(&[2, 7, 7])),
        (0u8, Invariant::counts(&[2, 3])),
    ];
    let ask = encode_cover_request(&vk, b"").expect("an empty cursor fits");
    assert_eq!(decode_cover_request(&ask), Some((vk, Vec::new())), "an ask round-trips");
    assert!(decode_cover_page(&ask).is_none(), "an ask is not a page");

    // Two rows of the three: the act byte, the vk, the bind's length and
    // bytes, the count, then per row the lane byte, the key's TWO length
    // bytes and the chart's own key, then the `more` byte. A row is measured from
    // `cover_cursor` — the cursor IS the lane and the key — rather than from
    // a width formula, which went stale the moment a coordinate stopped
    // being eight bytes (A21 as ruled 2026-09-18).
    let row = |lane: u8, inv: &Invariant| cover_cursor(lane, inv).len() + 2;
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

    // A chart wider than the row's LENGTH FIELD is passed over, not
    // refused with the page: a refusal would meet it at the head of every
    // later ask. The field is two bytes (A21), so the width that is passed
    // over is past 65535 and not past 255 — a 256-coordinate chart, which
    // the one-byte field dropped, is served.
    let served = vec![(0u8, Invariant::counts(&[1])), (1u8, Invariant::counts(&[4u64; 256]))];
    let (both, more) = encode_cover_page(&vk, "a", &served, b"", 1 << 16)
        .as_deref()
        .and_then(decode_cover_page)
        .expect("a page");
    assert_eq!(both.charts.len(), 2, "a 256-coordinate chart is served, not dropped");
    assert!(!more, "and the page is complete");
    // A Count axis is ten bytes (kind, a one-byte length, eight): 7 000 of
    // them are past two bytes of length.
    let over = vec![(0u8, Invariant::counts(&[1])), (1u8, Invariant::counts(&[4u64; 7000]))];
    let (only, more) = encode_cover_page(&vk, "a", &over, b"", 1 << 20)
        .as_deref()
        .and_then(decode_cover_page)
        .expect("a page");
    assert_eq!(only.charts, vec![(0u8, Invariant::counts(&[1]))], "past the field, passed over");
    assert!(!more, "and the page is complete without it");
}

/// 47.1, I33 — A PREFIX THAT DOES NOT FIT THE WIRE IS NOT FRAMED. The
/// section's length is two bytes; a prefix carrying a hypha's whole cover
/// (10,855 axes on the development deployment, 2026-09-08) was written with a wrapped
/// length, and every peer refused every digest whole while the hypha
/// believed it spoke. `fits` says so before anything is signed, and
/// `encode_receipt_section` refuses rather than wraps; a prefix of one
/// chart frames and reads back.
#[test]
fn encode_receipt_section_refuses_prefix_past_wire() {
    let mut chain = crate::chronometry::ReceiptChain::with_signer([6u8; 32]);
    let head = chain.append(crate::chronometry::ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    let prefix = |chart: Vec<u8>| crate::court::admission::SignedPrefix {
        genesis: [8; 32],
        epoch: 2,
        prev_receipt_hash: head.prev_hash(),
        at: head.stamp.place(),
        height: head.stamp.h,
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

/// A21 — A CHART IS AS WIDE AS THE BODY'S OWN MODULI, and the cover serves
/// it. The row's length field was ONE byte, so a chart key past 255 was
/// passed over with no refusal anywhere: `encode_cover_page` skipped it and
/// the asker resumed past it, so the body simply never appeared on any
/// peer's cover. Measured under `ChartAxis` since each length is its own
/// coordinate (STD1 3c-2): a `Count` is 10 bytes and a single-limb `Ratio`
/// 21, so a body's chart passes 255 at eleven ratios.
#[test]
fn encode_cover_page_serves_chart_past_one_byte() {
    use crate::transducer::stalk::{ChartAxis, Invariant};
    use num_bigint::BigInt;
    use num_rational::BigRational;
    let vk = [4u8; 32];
    // Four counts and eleven ratios: past 255 bytes, which one length byte
    // cannot name.
    let mut axes: Vec<ChartAxis> = [1u64, 0, 0, 4].into_iter().map(ChartAxis::Count).collect();
    for n in 1..=11i64 {
        axes.push(ChartAxis::Ratio(BigRational::new(BigInt::from(n), BigInt::from(7))));
    }
    let wide = Invariant(axes);
    assert!(
        wide.key().as_bytes().len() > u8::MAX as usize,
        "the fixture is past one byte of length: {} bytes",
        wide.key().as_bytes().len()
    );
    let charts = vec![(0u8, wide.clone())];
    let page = encode_cover_page(&vk, "127.0.0.1:7500", &charts, b"", 1 << 16).expect("a page");
    let (read, more) = decode_cover_page(&page).expect("a page decodes");
    assert_eq!(read.charts, charts, "the wide chart is served whole and reads back exactly");
    assert!(!more, "and the cover is served");
}
