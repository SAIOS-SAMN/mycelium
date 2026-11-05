// MU-DERIVATIONS D4.2 — the evidence pull's reply is filled to the frame by
// each item's own width, never a count chosen beside it, and an item no
// frame carries is refused by name, the page ending before it.

use crate::anastome::dialogue::*;
use crate::transducer::anchor::EvidenceOf;

/// An item of a one-byte address, `nf` footprint sites and `ns` scene sites:
/// `σ = 2 + 1 + 4 + 16·nf + 4 + 17·ns`.
fn item(nf: i64, ns: i64) -> EvidenceOf<Vec<u8>> {
    use crate::transducer::spelled::{PaintedSite, SectionSite};
    EvidenceOf {
        addr: vec![7],
        footprint: (0..nf).map(|i| SectionSite::on_plane([i, 0])).collect(),
        scene: (0..ns).map(|i| PaintedSite::on_plane([i, 0], 1)).collect(),
    }
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 10)
}

/// THE PAGE IS THE LONGEST PREFIX THAT FITS: three items of one width; a
/// budget one byte short of the header and three fills two, the whole three
/// read to the store's end, and the reply round-trips.
#[test]
fn evidence_reply_fills_frame_budget() {
    let store = vec![item(2, 3), item(2, 3), item(2, 3)];
    let w = evidence_width(&store[0]).expect("framed");
    assert_eq!(w, 2 + 1 + 4 + 16 * 2 + 4 + 17 * 3, "the width read off the format");
    let short = EVIDENCE_REPLY_HEADER + 3 * w - 1;
    assert_eq!(evidence_page(&store, short, &mut stroke()), (2, EvidencePageEnd::Full));
    assert_eq!(evidence_page(&store, short + 1, &mut stroke()), (3, EvidencePageEnd::Store));
    let body = encode_evidence_reply(&store[..2]).expect("framed");
    assert_eq!(body.len(), EVIDENCE_REPLY_HEADER + 2 * w, "the page's bytes are the header and the widths");
    assert_eq!(decode_evidence_reply(&body).map(|v| v.len()), Some(2));
}

/// AN ITEM NO FRAME CARRIES IS REFUSED BY NAME: the page ends before it,
/// naming its width, and the items before it ride.
#[test]
fn evidence_reply_refuses_unframable_item() {
    let store = vec![item(1, 1), item(64, 64), item(1, 1)];
    let budget = EVIDENCE_REPLY_HEADER + evidence_width(&store[0]).expect("framed") * 2;
    let wide = evidence_width(&store[1]);
    assert_eq!(evidence_page(&store, budget, &mut stroke()), (1, EvidencePageEnd::Unframable { width: wide }));
    assert_eq!(evidence_page(&store, budget, &mut crate::physics::Physics::default().stroke_n(0)), (0, EvidencePageEnd::Spent));
}
