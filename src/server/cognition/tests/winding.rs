// 78.68 — THE WINDING CARRIER: a closed ring traversed again writes nothing
// until it returns, and each age is one `Return` act.

use crate::chronometry::ActKind;
use crate::cognition::court::winding::{Draw, Winding};
use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::stalk::Invariant;

fn returns(k: &Kernel) -> usize {
    k.chain().nodes().filter(|r| r.kind == ActKind::Return).count()
}

/// A RING OF PERIOD THREE: its first close enters the record; the next two
/// closes are carried — the winding moves and nothing is sealed — and the
/// third completes the age with one `Return`. Three ages, three returns,
/// never one act per pass. A ring closed to the identity (period one)
/// returns at every traversal after its first close, and a ring of infinite
/// order (period zero) never carries.
#[test]
fn a_traversed_ring_writes_one_return_per_age() {
    let mut k = Kernel::new();
    k.bind_identity([0x5c; 32]);
    let chart = Invariant::counts(&[1, 2]);
    let ring = b"ring";
    k.close_ring(&chart, ring, 3, &[], None);
    assert_eq!(k.winding_tally().first, 1, "the first close enters the record");
    for traversal in 1..=8usize {
        let before = k.chain().len();
        k.close_ring(&chart, ring, 3, &[], None);
        // The first close was the ring's first pass; a return falls where
        // the passes total a multiple of ℓ.
        let total = traversal + 1;
        assert_eq!(returns(&k), total / 3, "one Return per completed age");
        if total % 3 != 0 {
            assert_eq!(k.chain().len(), before, "a carried traversal seals nothing");
        }
    }
    let t = k.winding_tally();
    assert_eq!((t.returns, t.carried), (3, 5), "three ages; the other five passes carried");

    let identity = b"identity";
    k.close_ring(&chart, identity, 1, &[], None);
    let r0 = returns(&k);
    k.close_ring(&chart, identity, 1, &[], None);
    k.close_ring(&chart, identity, 1, &[], None);
    assert_eq!(returns(&k), r0 + 2, "period one: every traversal is an age");

    let open = b"rosette";
    let before = k.winding_tally().unbound;
    k.close_ring(&chart, open, 0, &[], None);
    k.close_ring(&chart, open, 0, &[], None);
    assert_eq!(k.winding_tally().unbound, before + 2, "infinite order: settled where it closes, never carried");
}

/// THE CARRIER READ BACK BY ITS OWN COORDINATES: period, winding, rewrite,
/// accrued draws and torsion — the accrued pay is a claim, kept whole.
#[test]
fn a_winding_carrier_reads_back_whole() {
    let w = Winding {
        ell: 4,
        m: 3,
        rewrite: Some(Address::from(vec![7, 8, 9])),
        draws: vec![Draw { listing: Address::from(vec![1, 2]), parties: vec![[3; 32], [4; 32]], share: [5, 0, 6, 1] }],
        torse: vec![([9; 32], [1, 2, 3, 4])],
    };
    let mut v = Vec::new();
    w.put(&mut v);
    let mut p = 0usize;
    assert_eq!(Winding::take(&v, &mut p), Some(w), "the carrier reads back");
    assert_eq!(p, v.len(), "whole");
}

/// 78.69 — THE PRESSURE A WITNESS RAISED IS RELIEVED EXACTLY WHEN THE WORD
/// LEAVES THE BOOK: `steps` times its magnitude raised, held against its
/// address, and released whole by `pending_take` — never raised `steps`
/// times and relieved once, the ratchet that climbed 𝒯 on the courts.
#[test]
fn a_witnessed_charge_is_relieved_whole_when_the_word_leaves() {
    let mut k = Kernel::new();
    let word = super::common::sentence_of(&[(0, 0, 1), (0, 1, 1)], &[(0, 0, 2), (0, 1, 2)]).expect("a sentence");
    let addr = word.address();
    k.pending.insert(&addr, word);
    let before = k.physics.pressure;
    k.charge_witnessed(&addr, &[1, 0, 3, 0], 4);
    assert_eq!(k.physics.pressure, [before[0] + 4, before[1], before[2] + 12, before[3]], "four steps raised");
    let _ = k.pending_take(addr.as_bytes()).expect("the word was pending");
    assert!(k.witnessed.get(addr.as_bytes()).is_none(), "the charge left with the word");
    assert_eq!(k.physics.pressure[2], before[2], "𝒯 back where it stood: the raised twelve relieved whole");
}
