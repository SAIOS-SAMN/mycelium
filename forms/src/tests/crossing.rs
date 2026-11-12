// LAW-PASSAGE v3 (proposal 78.172 §5; netstratum docs/117 §12): the
// passage is the pole swap, the reflection in the throat's unit `e₊`. Its
// joint-read fixtures, written on plane 1's units, the law's claims over
// every plane the algebra holds, and the passage's refusals.

use crate::algebra::{base_axis, Clifford, SignatureRefusal, EM};
use crate::passage::{pass_share, Frame, PassageRefusal, Place, Pole, Poles};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

/// Plane `k`'s positive unit `e₊ₖ`, squaring to +1: base axis `2k − 1`.
fn positive(k: usize) -> Option<usize> {
    base_axis(2 * k - 1)
}

/// Plane `k`'s negative unit `e₋ₖ`, squaring to −1: base axis `2k`.
fn negative(k: usize) -> Option<usize> {
    base_axis(2 * k)
}

/// Every plane the algebra's masks hold, in order.
fn planes() -> impl Iterator<Item = usize> {
    (1..).map_while(|k| negative(k).map(|_| k))
}

/// The signature of the planes `1..=m`: each plane's negative unit
/// negative, beside the conformal pair's `e₋`.
fn signature(m: usize) -> usize {
    (1..=m).filter_map(negative).fold(EM, |s, g| s | g)
}

/// A generator of a plane the fixture names.
fn unit(g: Option<usize>) -> usize {
    g.expect("a plane the algebra holds")
}

/// `c` times the product of the generators `gens`, in the order written,
/// under the signature of the planes `1..=m`.
fn word(m: usize, c: BigRational, gens: &[usize]) -> Clifford {
    let sig = signature(m);
    let one = Clifford::blade(sig, 0, c).expect("the scalar");
    gens.iter().fold(one, |w, g| {
        let generator = Clifford::blade(sig, *g, q(1, 1)).expect("a generator the algebra holds");
        w.product(&generator).expect("one signature")
    })
}

/// The sum of `words`, each under one signature.
fn sum(words: Vec<Clifford>) -> Clifford {
    let mut it = words.into_iter();
    let first = it.next().expect("a term");
    it.fold(first, |s, w| s.sum(&w).expect("one signature"))
}

/// Plane `k`'s poles, under the planes `1..=m`.
fn poles(m: usize, k: usize) -> Poles {
    Poles::of(&word(m, q(1, 1), &[unit(positive(k))]), &word(m, q(1, 1), &[unit(negative(k))])).expect("two units of opposite sign")
}

/// The frame state at hold `h` and reach `r` on plane `k`.
fn frame(p: &Poles, h: BigRational, r: BigRational) -> Clifford {
    p.state(&Frame { hold: h, reach: r }).expect("a frame state")
}

/// The same value: one signature, the same terms.
fn same(a: &Clifford, b: &Clifford) -> bool {
    a.negatives() == b.negatives() && a.terms().eq(b.terms())
}

/// The degrees the value holds terms on, ascending, each once.
fn degrees(v: &Clifford) -> Vec<u32> {
    let mut d: Vec<u32> = v.terms().map(|(m, _)| m.count_ones()).collect();
    d.sort_unstable();
    d.dedup();
    d
}

/// THE RIDING STATE of fixture 4: `q = 3 + 2·e₊₂e₊₃ + e₊₁e₋₂`, even, over
/// three planes.
fn riding() -> Clifford {
    sum(vec![
        word(3, q(3, 1), &[]),
        word(3, q(2, 1), &[unit(positive(2)), unit(positive(3))]),
        word(3, q(1, 1), &[unit(positive(1)), unit(negative(2))]),
    ])
}

/// THE POLES ARE THE PLANE'S NULL DIRECTIONS, the frame state on them is
/// the whole on `e₊` and the span on `e₋`: `H·n_Z + R·n_Q =
/// (H + R)·e₊ + (H − R)·e₋`, and the state reads back as its hold and
/// reach.
#[test]
fn state_equals_whole_on_positive_and_span_on_negative() {
    let p = poles(1, 1);
    let x = frame(&p, q(4, 5), q(1, 5));
    let written = sum(vec![word(1, q(1, 1), &[unit(positive(1))]), word(1, q(3, 5), &[unit(negative(1))])]);
    assert!(same(&x, &written), "{x:?}");
    assert_eq!(p.read(&x), Ok(Frame { hold: q(4, 5), reach: q(1, 5) }));
    let closed = frame(&p, q(1, 1), q(0, 1));
    let open = frame(&p, q(0, 1), q(1, 1));
    assert!(closed.product(&closed).expect("one signature").is_zero(), "n_Z is null");
    assert!(open.product(&open).expect("one signature").is_zero(), "n_Q is null");
}

/// FIXTURE 1: span `3/5` (`H = 4/5`, `R = 1/5`) crossing from `T = 0`
/// gives `S = −3/5` and `T = −6/5`; crossing back gives `S = 3/5` and
/// `T = 0`, and the frame state it started from.
#[test]
fn cross_writes_span_change_on_flux_and_return_cancels() {
    let p = poles(1, 1);
    let x = frame(&p, q(4, 5), q(1, 5));
    let there = p.cross(&x, &q(0, 1)).expect("a frame state");
    assert_eq!((there.span.clone(), there.flux.clone()), (q(-3, 5), q(-6, 5)));
    let back = p.cross(&there.frame, &there.flux).expect("a frame state");
    assert_eq!((back.span, back.flux), (q(3, 5), q(0, 1)));
    assert!(same(&back.frame, &x), "the pass and its return are the identity");
}

/// FIXTURE 2: at the throat (`H = R`, span 0) a crossing changes nothing:
/// the frame state, `S = 0`, and nothing written on `T`.
#[test]
fn cross_at_throat_changes_nothing() {
    let p = poles(1, 1);
    let x = frame(&p, q(1, 2), q(1, 2));
    assert_eq!(p.read(&x).map(|f| f.place()), Ok(Place::Throat));
    let crossed = p.cross(&x, &q(7, 3)).expect("a frame state");
    assert!(same(&crossed.frame, &x));
    assert_eq!((crossed.span, crossed.flux), (q(0, 1), q(7, 3)));
}

/// FIXTURE 3: a pole, `H > 0` and `R = 0` (null), swaps to the other pole,
/// `H·n_Q`; and the passage is one-way there: every further crossing lands
/// at a pole, never in the interior.
#[test]
fn swap_carries_pole_to_other_pole_and_never_into_interior() {
    let p = poles(1, 1);
    let x = frame(&p, q(4, 5), q(0, 1));
    assert_eq!(p.read(&x).map(|f| f.place()), Ok(Place::Pole(Pole::Closed)));
    let swapped = p.swap(&x).expect("a frame state");
    assert!(same(&swapped, &frame(&p, q(0, 1), q(4, 5))), "H·n_Z ↦ H·n_Q");
    let mut at = x;
    for _ in 0..4 {
        at = p.swap(&at).expect("a frame state");
        assert!(matches!(p.read(&at).map(|f| f.place()), Ok(Place::Pole(_))), "no return to the interior");
    }
}

/// FIXTURE 4: the riding state's parity. An even `q` gives `e₊·q` odd, and
/// `e₊·e₊·q = q`.
#[test]
fn ride_flips_parity_and_twice_is_identity() {
    let p = poles(3, 1);
    let s = riding();
    assert_eq!(degrees(&s), [0, 2], "the state is even");
    let once = p.ride(&s).expect("a state of one parity");
    assert_eq!(degrees(&once), [1, 3], "e₊·q is odd");
    let e = word(3, q(1, 1), &[unit(positive(1))]);
    assert!(same(&once, &e.product(&s).expect("one signature")), "one-sided, e₊·q");
    assert!(same(&p.ride(&once).expect("a state of one parity"), &s), "e₊·e₊·q = q");
}

/// FIXTURE 5: the sandwich applied twice is the identity on a general frame
/// state, in every plane the algebra holds, the span reversed once.
#[test]
fn swap_twice_is_identity_on_every_frame() {
    let m = planes().count();
    for k in planes() {
        let p = poles(m, k);
        for (h, r) in [(q(7, 3), q(-2, 5)), (q(-5, 2), q(1, 9)), (q(0, 1), q(3, 1)), (q(11, 4), q(11, 4))] {
            let x = frame(&p, h.clone(), r.clone());
            let once = p.swap(&x).expect("a frame state");
            assert_eq!(p.read(&once), Ok(Frame { hold: r.clone(), reach: h.clone() }), "plane {k}: the roles exchange");
            assert!(same(&p.swap(&once).expect("a frame state"), &x), "plane {k}");
        }
    }
}

/// THE ROLES EXCHANGE AND THE FACE INVERTS (netstratum docs/117 §12.6):
/// `(H, R) = (4/5, 1/5)` crossing stands at reach `4/5` about hold `1/5`;
/// the whole `H + R = 1` is kept, and `δ` goes `4/5 ↦ 1/5`.
#[test]
fn cross_exchanges_hold_and_reach_and_inverts_face() {
    let p = poles(1, 1);
    let before = Frame { hold: q(4, 5), reach: q(1, 5) };
    let crossed = p.cross(&p.state(&before).expect("a frame state"), &q(0, 1)).expect("a frame state");
    let after = p.read(&crossed.frame).expect("a frame state");
    assert_eq!(after, Frame { hold: q(1, 5), reach: q(4, 5) });
    assert_eq!((before.whole(), after.whole()), (q(1, 1), q(1, 1)));
    assert_eq!((before.share(), after.share()), (Ok(q(4, 5)), Ok(q(1, 5))));
    assert_eq!(pass_share(&q(4, 5)), after.share(), "the face read from the other pole");
}

/// NO REACH MOVES: the swap keeps `|H − R|` and reverses its sign, and the
/// flux of a pass and its immediate return cancel, over exact frames.
#[test]
fn cross_keeps_span_size_and_cancels_flux_on_return() {
    let p = poles(1, 1);
    for (h, r, t) in [(q(3, 2), q(1, 7), q(0, 1)), (q(-1, 3), q(5, 6), q(9, 4)), (q(2, 1), q(2, 1), q(-1, 2))] {
        let x = frame(&p, h.clone(), r.clone());
        let there = p.cross(&x, &t).expect("a frame state");
        assert_eq!(there.span, &r - &h, "H − R ↦ R − H");
        let back = p.cross(&there.frame, &there.flux).expect("a frame state");
        assert_eq!(back.flux, t, "the return cancels on T");
    }
}

/// A passage state off the plane is refused `NotOnPolePlane` with the blades it
/// holds there: another plane's unit, a scalar, a bivector of the plane,
/// and the zero, which holds none.
#[test]
fn read_refuses_state_off_plane() {
    let p = poles(2, 1);
    let x = frame(&p, q(1, 1), q(2, 1));
    let other = unit(positive(2));
    let off = x.sum(&word(2, q(1, 1), &[other])).expect("one signature");
    assert_eq!(p.swap(&off).err(), Some(PassageRefusal::NotOnPolePlane { blades: vec![other] }));
    let scalar = word(2, q(3, 1), &[]);
    assert_eq!(p.cross(&scalar, &q(0, 1)).err(), Some(PassageRefusal::NotOnPolePlane { blades: vec![0] }));
    let plane = unit(positive(1)) | unit(negative(1));
    let bivector = word(2, q(1, 1), &[unit(positive(1)), unit(negative(1))]);
    assert_eq!(p.read(&bivector).err(), Some(PassageRefusal::NotOnPolePlane { blades: vec![plane] }));
    let zero = x.difference(&x).expect("one signature");
    assert_eq!(p.read(&zero).err(), Some(PassageRefusal::NotOnPolePlane { blades: vec![] }));
}

/// A riding state holding both parities is refused `MixedParity` with the
/// degrees it holds.
#[test]
fn ride_refuses_mixed_parity() {
    let p = poles(3, 1);
    let mixed = riding().sum(&word(3, q(1, 1), &[unit(positive(3))])).expect("one signature");
    assert_eq!(p.ride(&mixed).err(), Some(PassageRefusal::MixedParity { degrees: vec![0, 1, 2] }));
}

/// A pole that is not one generator at coefficient one is refused
/// `NotUnit` with its terms; units of the wrong sign `WrongSign` with the
/// generator; and a state of another signature `Mismatched`.
#[test]
fn poles_refuse_non_unit_wrong_sign_and_mismatch() {
    let (e_plus, e_minus) = (unit(positive(1)), unit(negative(1)));
    let plus = word(1, q(1, 1), &[e_plus]);
    let minus = word(1, q(1, 1), &[e_minus]);
    let twice = word(1, q(2, 1), &[e_plus]);
    assert_eq!(Poles::of(&twice, &minus).err(), Some(PassageRefusal::NotUnit { terms: vec![(e_plus, q(2, 1))] }));
    let pair = word(1, q(1, 1), &[e_plus, e_minus]);
    assert_eq!(Poles::of(&plus, &pair).err(), Some(PassageRefusal::NotUnit { terms: vec![(e_plus | e_minus, q(1, 1))] }));
    assert_eq!(Poles::of(&minus, &minus).err(), Some(PassageRefusal::WrongSign { generator: e_minus }));
    assert_eq!(Poles::of(&plus, &plus).err(), Some(PassageRefusal::WrongSign { generator: e_plus }));
    let flat = Clifford::blade(EM, e_plus, q(1, 1)).expect("a blade the algebra holds");
    let x = Clifford::blade(EM, e_minus, q(1, 1)).expect("a blade the algebra holds");
    assert_eq!(Poles::of(&flat, &x).err(), Some(PassageRefusal::WrongSign { generator: e_minus }));
    assert_eq!(poles(1, 1).read(&x).err(), Some(PassageRefusal::Mismatched(SignatureRefusal::Mismatched { generators: e_minus })));
}

/// A frame state of no whole holds no share, refused `NoWhole` with its
/// span; one whose hold lies outside the whole, `NotShare` with the value.
#[test]
fn share_refuses_no_whole_and_outside_unit_interval() {
    assert_eq!(Frame { hold: q(1, 2), reach: q(-1, 2) }.share(), Err(PassageRefusal::NoWhole { span: q(1, 1) }));
    assert_eq!(Frame { hold: q(3, 2), reach: q(-1, 2) }.share(), Err(PassageRefusal::NotShare(Box::new(q(3, 2)))));
    assert_eq!(Frame { hold: q(0, 1), reach: q(0, 1) }.place(), Place::Throat, "the zero frame reads at the throat");
    assert_eq!(Frame { hold: q(0, 1), reach: q(2, 1) }.place(), Place::Pole(Pole::Open));
    assert_eq!(Frame { hold: q(2, 1), reach: q(1, 1) }.place(), Place::Interior);
}
