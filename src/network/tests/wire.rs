use crate::network::wire::*;

/// An envelope decodes to itself: a length and a body, no lane byte.
#[test]
fn envelope_round_trips_as_length_and_body() {
    let f = Delimited { body: vec![1, 2, 3, 9] };
    let bytes = f.encode().unwrap();
    assert_eq!(&bytes[..HEAD], &4u32.to_le_bytes(), "the envelope opens with the body's length");
    let (g, n) = Delimited::decode(&bytes).unwrap();
    assert_eq!(f, g);
    assert_eq!(n, HEAD + 4);
}

/// A body past the envelope's bound is refused by the encoder, never
/// truncated, and a head naming one is refused before the body is read.
#[test]
fn encode_refuses_body_past_delimited_max() {
    let f = Delimited { body: vec![0u8; DELIMITED_MAX + 1] };
    assert!(f.encode().is_none(), "over MTU — refused, never truncated");
    let past = u32::try_from(DELIMITED_MAX + 1).unwrap().to_le_bytes();
    assert_eq!(delimited_len(past), None);
}

/// 78.152 TU-7, v0.5.0 — THE POINT STANDS IN THE EXPRESSION, NOT ON THE
/// ENVELOPE: an act's expression carries its speaker's last final point
/// behind the act, its lanes holding shares of a unit, read back exactly by
/// its count; the envelope carrying it is a length and the expression's bytes.
#[test]
fn expression_carries_last_final_point_inside_the_body() {
    use crate::anastome::dialogue::Expression;
    use crate::chronometry::tuniversal::{Origin, TPoint};
    let q = |n: i64, d: i64| crate::core::Q::new(n.into(), d.into());
    let lanes = crate::economy::Mu::of([q(3, 1), q(0, 1), q(9, 900), q(1, 2)]).expect("non-negative");
    let point = TPoint { origin: Origin { genesis: [7; 32], epoch: 146 }, h: 40_961, lanes, k: 812 };
    let e = Expression::of(vec![0x53, 1, 2], Some(point.clone()));
    let body = e.write().unwrap();
    assert_eq!(body[0], 0x53, "the act's tag stays at offset zero");
    let (delimited, n) = Delimited::decode(&Delimited { body: body.clone() }.encode().unwrap()).unwrap();
    assert_eq!(n, HEAD + 3 + point.to_bytes().len() + 2 + 2);
    assert_eq!(Expression::read(delimited.body), Some(e));
    let none = Expression::of(vec![0x53, 1, 2], None);
    let [lo, hi] = crate::anastome::dialogue::WIRE_VERSION.to_le_bytes();
    assert_eq!(none.write().unwrap(), vec![0x53, 1, 2, 0, 0, lo, hi], "a speaker with no point read writes the count zero, then its wire version");
    assert_eq!(Expression::read(none.write().unwrap()), Some(none), "an expression with no point reads with none");
}

/// An expression whose count is missing, or counts no point read whole, is
/// refused whole: no writing is legacy.
#[test]
fn expression_without_its_count_is_refused() {
    use crate::anastome::dialogue::{Expression, Unheard, WIRE_VERSION};
    let [lo, hi] = WIRE_VERSION.to_le_bytes();
    assert_eq!(Expression::heard(vec![0x53]), Err(Unheard::Malformed), "one byte holds no version");
    assert_eq!(Expression::heard(vec![0x53, lo, hi]), Err(Unheard::Malformed), "a version holds no count");
    assert_eq!(Expression::heard(vec![0x53, 1, 2, 9, 0, lo, hi]), Err(Unheard::Malformed), "a count of nine passes the bytes");
    assert_eq!(Expression::heard(vec![0x53, 1, 2, 3, 0, lo, hi]), Err(Unheard::Malformed), "three bytes read as no point");
}

/// #235 — A READER HEARS ITS OWN MAJOR AT ANY MINOR, AND REFUSES ANOTHER
/// MAJOR BY NAME before the point or the act: an expression of an older or a
/// newer major is `Unheard::Version`, naming both versions; a minor step,
/// additive by the wire version rule (MU-ENGINEERING §4), is heard. The
/// version is the expression's last two bytes, so it is read whatever the
/// act and the point hold.
#[test]
fn expression_at_another_wire_version_is_refused_by_name() {
    use crate::anastome::dialogue::{speaks, Expression, Unheard, WIRE_VERSION};
    let at = |v: u16| {
        let mut b = Expression::of(vec![0x53, 1, 2], None).write().unwrap();
        let n = b.len();
        b[n - 2..].copy_from_slice(&v.to_le_bytes());
        b
    };
    let major = WIRE_VERSION >> 8;
    for spoken in [WIRE_VERSION, WIRE_VERSION + 1, major << 8 | 0xff] {
        assert_eq!(Expression::heard(at(spoken)).map(|e| e.act), Ok(vec![0x53, 1, 2]), "{spoken:#06x}, one major, is heard");
    }
    for spoken in [(major - 1) << 8, (major + 1) << 8, 0x0200, 0xffff] {
        assert_eq!(Expression::heard(at(spoken)), Err(Unheard::Version { spoken, served: WIRE_VERSION }), "{spoken:#06x} is refused by name");
    }
    assert!(speaks(major << 8 | 7, major << 8) && !speaks((major + 1) << 8, major << 8 | 9), "the major alone decides");
}
