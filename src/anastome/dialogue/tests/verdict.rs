// 47.2, 47.4 — THE RULED ANSWER on the wire: what a receiver answers to an
// equivocation proof parses back to the arm it reached.

use crate::anastome::dialogue::*;

/// 42, 47.2: A RULING'S ANSWER IS FOUR ARMS, and the ruling court must be
/// able to tell them apart. It was one byte, `applied as u8`, and every
/// outcome answered `0`: another mesh, a bad signature, a chain gap, a
/// stale frame, an equivocation, and a ruling that found nothing to move.
/// A gap means "send me the back-fill" and an equivocation is 47.4's burn
/// proof — the mesh's only bond-slashing evidence — and both read as "your
/// ruling did not land".
#[test]
fn a_ruling_is_answered_with_the_arm_it_reached() {
    for r in [Ruled::Applied, Ruled::Inert] {
        let back = decode_ruled(&encode_ruled(&r).expect("frames")).expect("parses whole");
        assert_eq!(
            std::mem::discriminant(&back),
            std::mem::discriminant(&r),
            "applied and inert are two answers, not one byte"
        );
    }

    let why = crate::court::verdict::REFUSE_EQUIVOCATION;
    let back = decode_ruled(&encode_ruled(&Ruled::Refused(why.as_bytes().to_vec())).expect("frames"))
        .expect("a refusal parses whole");
    match back {
        Ruled::Refused(got) => assert_eq!(
            String::from_utf8(got).expect("a 34.1 constant is text"),
            why,
            "the 34.1 constant travels, so a burn proof is not lost at the door"
        ),
        _ => panic!("the arm it was encoded as"),
    }

    let (from, to) = (Some(crate::chronometry::Place { k: 7, i: 2 }), crate::chronometry::Place { k: 19, i: 0 });
    let back = decode_ruled(&encode_ruled(&Ruled::Gap { from, to }).expect("frames")).expect("a gap parses whole");
    match back {
        Ruled::Gap { from: f, to: t } => {
            assert_eq!((f, t), (from, to), "a gap names the span to back-fill, which a zero byte could not")
        }
        _ => panic!("the arm it was encoded as"),
    }
    let open = encode_ruled(&Ruled::Gap { from: None, to }).expect("frames");
    assert!(
        matches!(decode_ruled(&open), Some(Ruled::Gap { from: None, to: t }) if t == to),
        "and one from the line's start"
    );
    assert!(decode_ruled(&open[..open.len() - 1]).is_none(), "a short gap refuses");

    assert!(decode_ruled(&[9u8]).is_none(), "an undefined arm refuses the frame");
    assert!(decode_ruled(&[]).is_none());
    assert!(decode_ruled(&[3u8, 0, 0]).is_none(), "a truncated gap is not a gap");
}
