// 78.119 — ACT_UPLINK's wire: a signed fold and the court's answers read back
// whole; a byte left over is no fold. 78.138 — a line's fold carried inline,
// nested, and a fold standing inside another named by its path.

use crate::anastome::dialogue::uplink::{
    decode_uplink, decode_uplink_reply, encode_uplink, encode_uplink_reply, Child, ChildNet, ChildRefusal, Discontinuous, FoldAt, TierFold,
    Uplinked,
};
use crate::chronometry::Stamp;

/// A signed fold and every answer of the court read back whole, and a byte
/// left over is no fold.
#[test]
fn uplink_fold_round_trips_with_answers() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let fold = TierFold {
        tier: key.verifying_key().to_bytes(),
        prefix: [2u8; 32],
        rows: vec![([1u8; 32], [3, -1, 0, 7]), ([4u8; 32], [-3, 1, 0, -7])],
        declared: vec![([8u8; 32], [0, 0, 0, 0])],
        children: Vec::new(),
        handoffs: Vec::new(),
        transitions: Vec::new(),
    };
    // 78.123 B — a fold with no children writes exactly as 78.119's: its
    // bytes end at the signature, and it signs no children's commitment.
    let single = 1 + 32 + 32 + fold.rows_bytes().len() + 64;
    assert_eq!(fold.children_hash(), None);
    let signed = fold.clone().sign(&key);
    let mut asked = encode_uplink(&signed);
    assert_eq!(asked.len(), single, "a single-level fold's bytes unchanged");
    assert_eq!(decode_uplink(&asked), Some(signed));
    asked.push(0);
    assert_eq!(decode_uplink(&asked), None, "a byte left over is no fold");
    let at = Stamp { h: 3, vk: [6u8; 32], k: 2, i: 1 };
    let part = |child: Child| ChildNet {
        child,
        rows: vec![([1u8; 32], [2, 0, 0, 0])],
        declared: vec![([1u8; 32], [2, 0, 0, 0])],
        included_rows: vec![([1u8; 32], [1, 0, 0, 0])],
        included_declared: vec![([1u8; 32], [1, 0, 0, 0])],
        rate: [(1, 1), (3, 2), (1, 1), (1, 1)],
    };
    // A chained fold — its children after the signature — read back whole;
    // a child the court sealed itself writes untagged, as 78.123's did.
    let sealed = TierFold { children: vec![part(Child::Sealed(FoldAt::sealed(at)))], ..fold.clone() }.sign(&key);
    let asked = encode_uplink(&sealed);
    assert!(asked.len() > single, "its children ride after the signature");
    assert_eq!(decode_uplink(&asked), Some(sealed));
    // 78.138 — a line's fold CARRIED INLINE, itself carrying one, and a
    // fold standing inside another's receipt by its path: tagged, read back
    // whole, nested, never flattened.
    let line_key = ed25519_dalek::SigningKey::from_bytes(&[6u8; 32]);
    let grand = TierFold { tier: line_key.verifying_key().to_bytes(), prefix: [3u8; 32], ..fold.clone() }.sign(&line_key);
    let line = TierFold {
        tier: line_key.verifying_key().to_bytes(),
        prefix: [4u8; 32],
        children: vec![part(Child::Carried(Box::new(grand)))],
        ..fold.clone()
    }
    .sign(&line_key);
    let inside = FoldAt { stamp: at, path: vec![0, 1] };
    assert_eq!(FoldAt::from_key(&inside.key()), Some(inside.clone()), "a place inside keys and reads back");
    assert_eq!(FoldAt::sealed(at).key(), at.key(), "a fold the court sealed keys as it did");
    let carrying = TierFold { children: vec![part(Child::Carried(Box::new(line))), part(Child::Sealed(inside))], ..fold }.sign(&key);
    let asked = encode_uplink(&carrying);
    assert_eq!(decode_uplink(&asked), Some(carrying.clone()));
    let mut long = asked;
    long.push(0);
    assert_eq!(decode_uplink(&long), None, "a byte left over is no fold");
    for u in [
        Uplinked::Admitted { at, receipt: [9u8; 32] },
        Uplinked::Unsigned,
        Uplinked::Discontinuous(Discontinuous::Unordered),
        Uplinked::Discontinuous(Discontinuous::Overflow { lane: 2 }),
        Uplinked::Discontinuous(Discontinuous::Unbalanced { lane: 3 }),
        Uplinked::Discontinuous(Discontinuous::Rate { lane: 0 }),
        Uplinked::LineRefused,
        Uplinked::Unbound,
        Uplinked::Chained { at, receipt: [9u8; 32], provisional: true, obstructions: 2, open_remainders: 1 },
        Uplinked::ChildRefused { index: 3, why: ChildRefusal::NotOnLine },
        Uplinked::ChildRefused { index: 0, why: ChildRefusal::Mismatched },
        Uplinked::ChildRefused { index: 1, why: ChildRefusal::ForeignDomain },
        Uplinked::ChildRefused { index: 2, why: ChildRefusal::OverIncluded { lane: 1 } },
        Uplinked::ChildRefused { index: 0, why: ChildRefusal::Unsigned },
        Uplinked::ChildRefused { index: 0, why: ChildRefusal::Unbonded },
        Uplinked::ChildRefused { index: 0, why: ChildRefusal::Broken },
        Uplinked::ChildRefused { index: 0, why: ChildRefusal::Repeated },
        Uplinked::Repeated,
        Uplinked::NotReserve,
    ] {
        assert_eq!(decode_uplink_reply(&encode_uplink_reply(&u)), Some(u.clone()), "{u:?}");
    }
}

/// Track 24 B3 — A FOLD STATING TRANSITIONS, and a line it carries stating
/// its own, reads back whole with them; a fold stating none writes exactly
/// as before; and the refusal by the law's name and the unread answer read
/// back with where the transition stands and each breach or law unread.
#[test]
fn fold_with_transitions_round_trips_with_lawful_answers() {
    use crate::physics::laws::{Breach, Law, Transition, Unmeasured};
    use crate::transducer::cga::{rotator, Mv};
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let t = Transition { motion: rotator(&q(2), &q(1)), heights: (1, 2), bodies: Vec::new(), hues: Default::default(), fluxes: Vec::new() };
    let fold = TierFold {
        tier: key.verifying_key().to_bytes(),
        prefix: [2u8; 32],
        rows: Vec::new(),
        declared: Vec::new(),
        children: Vec::new(),
        handoffs: Vec::new(),
        transitions: Vec::new(),
    };
    let plain = fold.clone().sign(&key);
    let stating = TierFold { transitions: vec![t.clone(), Transition { motion: Mv::scalar(q(1)), ..t.clone() }], ..fold.clone() };
    assert_ne!(stating.signed_bytes(), fold.signed_bytes(), "the transitions are signed");
    let stating = stating.sign(&key);
    assert_eq!(decode_uplink(&encode_uplink(&stating)), Some(stating.clone()));
    assert_eq!(encode_uplink(&plain).len(), 1 + 32 + 32 + fold.rows_bytes().len() + 64, "a fold stating none writes as before");
    let part = ChildNet {
        child: Child::Carried(Box::new(stating.clone())),
        rows: Vec::new(),
        declared: Vec::new(),
        included_rows: Vec::new(),
        included_declared: Vec::new(),
        rate: [(1, 1); 4],
    };
    let carrying = TierFold { children: vec![part], transitions: vec![t], ..fold }.sign(&key);
    assert_eq!(decode_uplink(&encode_uplink(&carrying)), Some(carrying.clone()), "a carried line's transitions read back with it");
    let mut long = encode_uplink(&carrying);
    long.push(0);
    assert_eq!(decode_uplink(&long), None, "a byte left over is no fold");
    for u in [
        Uplinked::NotLawful { path: Vec::new(), index: 1, breaches: vec![(Law::Time, Breach::HeightNotAdvanced { before: 8, after: 8 })] },
        Uplinked::NotLawful { path: vec![0, 2], index: 0, breaches: vec![(Law::Conformal, Breach::NotAVersor), (Law::Continuity, Breach::MassNotConserved { made: q(-3) })] },
        Uplinked::Unread { path: vec![1], index: 4, unmeasured: vec![Unmeasured { law: Law::Conformal, needed_stroke: Some(14) }, Unmeasured { law: Law::Conformal, needed_stroke: None }] },
    ] {
        assert_eq!(decode_uplink_reply(&encode_uplink_reply(&u)), Some(u.clone()), "{u:?}");
    }
}
