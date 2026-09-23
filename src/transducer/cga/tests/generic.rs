// BQ3, 71.2(3) — THE GENERIC CARRIER AGAINST THE ORACLE: every reading the
// carrier itself answers, re-taken on `Cl(n+1, 1)` at n = 3 and compared
// with the golden file the fixed carrier wrote; and the algebra at n = 4.

use super::oracle::inputs;
use crate::transducer::cga::generic::{self, blades, bivector_planes, pseudo, MvN};
use crate::transducer::cga::{Mv, BLADES, EM, EP};
use num_rational::BigRational;
use num_traits::{One, Zero};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn hex(m: &MvN) -> String {
    let mut b = Vec::new();
    m.put_canonical(&mut b);
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn opt(m: Option<MvN>) -> String {
    match m {
        Some(m) => hex(&m),
        None => "refused".into(),
    }
}

/// The fixed carrier's value as the generic carrier's, at n = 3: the same
/// coefficient in the same slot.
fn generic_of(m: &Mv) -> MvN {
    let cs: Vec<BigRational> = (0..BLADES).map(|mask| m.coeff(mask).clone()).collect();
    MvN::from_coefficients(3, &cs).expect("thirty-two coefficients are the n = 3 algebra's")
}

/// The ops the carrier answers by itself, in the oracle's own line order.
const CARRIER_OPS: [&str; 20] = [
    "canonical", "reverse", "grade0", "grade1", "grade2", "grade3", "grade4", "grade5", "norm2",
    "inverse", "geo", "add", "sub", "sandwich", "wedge1", "quotient", "ratio", "projective_eq",
    "class", "holonomy_key",
];

fn carrier_readings(inputs: &[(String, MvN)]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, (name, m)) in inputs.iter().enumerate() {
        let (next_name, next) = &inputs[(i + 1) % inputs.len()];
        let pair = format!("{name}*{next_name}");
        out.push(format!("canonical {name}: {}", hex(m)));
        out.push(format!("reverse {name}: {}", hex(&m.reverse())));
        for g in 0..=5u32 {
            out.push(format!("grade{g} {name}: {}", hex(&m.grade(g))));
        }
        out.push(format!("norm2 {name}: {}", m.norm2()));
        out.push(format!("inverse {name}: {}", opt(m.versor_inverse())));
        out.push(format!("geo {pair}: {}", hex(&m.geo(next))));
        out.push(format!("add {pair}: {}", hex(&m.add(next))));
        out.push(format!("sub {pair}: {}", hex(&m.sub(next))));
        out.push(format!("sandwich {pair}: {}", hex(&m.sandwich(next))));
        out.push(format!("wedge1 {pair}: {}", hex(&m.grade(1).wedge_vector(1, &next.grade(1)))));
        out.push(format!("quotient {pair}: {}", opt(MvN::quotient(m, next))));
        out.push(format!(
            "ratio {pair}: {}",
            match generic::projective_ratio(m, next) {
                Some(l) => l.to_string(),
                None => "refused".into(),
            }
        ));
        out.push(format!("projective_eq {pair}: {}", m.projective_eq(next)));
        out.push(format!(
            "class {name}: {}",
            match m.class_invariants() {
                Some((i1, i2, kind)) => format!("{i1} {i2} {kind:?}"),
                None => "refused".into(),
            }
        ));
        out.push(format!(
            "holonomy_key {name}: {}",
            match m.class_key() {
                Some(k) => k.as_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>(),
                None => "refused".into(),
            }
        ));
        out.push(format!(
            "key_roundtrip {name}: {}",
            match m.key().and_then(|k| MvN::from_key(k.as_bytes(), 3)) {
                Some(back) => hex(&back),
                None => "refused".into(),
            }
        ));
    }
    out
}

/// THE GENERIC CARRIER REPRODUCES THE ORACLE AT n = 3: every carrier-level
/// line of the golden file, re-taken on `Cl(n+1, 1)`, byte for byte.
#[test]
fn the_generic_carrier_reproduces_the_oracle_at_n3() {
    let golden = std::fs::read_to_string("src/transducer/cga/tests/oracle.golden")
        .expect("the oracle has been written");
    let op_of = |line: &str| line.split_whitespace().next().map(str::to_string);
    let want: Vec<&str> = golden
        .lines()
        .filter(|l| {
            op_of(l).is_some_and(|op| CARRIER_OPS.contains(&op.as_str()) || op == "key_roundtrip")
        })
        .collect();
    let ins: Vec<(String, MvN)> = inputs().iter().map(|(n, m)| (n.clone(), generic_of(m))).collect();
    let got = carrier_readings(&ins);
    let first = want.iter().zip(&got).position(|(w, g)| *w != g.as_str());
    assert!(
        first.is_none() && want.len() == got.len(),
        "the generic carrier moved at line {:?} of {} (got {}): {:?} against {:?}",
        first,
        want.len(),
        got.len(),
        first.map(|i| want[i]),
        first.map(|i| &got[i])
    );
    eprintln!("  generic: {} carrier readings reproduce the oracle at n = 3", got.len());
}

/// THE ALGEBRA AT n = 4: `2⁶` blades, fifteen planes, the signature's
/// metric, the new axis at its own bit, and the n = 3 algebra inside it.
#[test]
fn the_algebra_at_n4_has_its_own_count_and_holds_n3_inside_it() {
    let n = 4u8;
    assert_eq!(blades(n), 64, "Cl(5,1) has 2^6 blades");
    assert_eq!(bivector_planes(n).len(), 15, "C(5,2) + 5 = 15 planes");
    assert_eq!(bivector_planes(3).len(), 10, "and the n = 3 row of the family is ten");
    let e4 = generic::base_axis(4);
    assert_eq!(e4, 1 << 5, "the fourth base axis takes bit 5, so e+ and e- never move");
    let one = |mask| MvN::blade(n, mask, BigRational::one()).expect("a generator of Cl(5,1)");
    assert_eq!(one(e4).norm2(), q(1, 1), "a base axis squares to +1");
    assert_eq!(one(EP).norm2(), q(1, 1), "e+ squares to +1");
    assert_eq!(one(EM).norm2(), q(-1, 1), "e- squares to -1");
    assert!(MvN::blade(3, e4, BigRational::one()).is_none(), "and n = 3 has no fourth axis");
    // The pseudoscalar is grade n + 2 and every generator anticommutes past
    // it an odd or even number of times as the dimension says.
    let i4 = one(pseudo(n));
    assert_eq!((pseudo(n) as u32).count_ones(), 6, "grade n + 2");
    assert!(!i4.geo(&i4).is_zero(), "the pseudoscalar is invertible");
    // THE INCLUSION: an n = 3 value is the same value at n = 4.
    let a = MvN::blade(3, 0b00011, q(2, 1)).expect("e1e2 at n = 3");
    let b = one(e4 | EM);
    let ab = a.geo(&b);
    assert_eq!(ab.n(), 4, "a product lives in the larger algebra");
    assert_eq!(ab.coeff(0b00011 | e4 | EM), q(2, 1), "e1e2 · e4e- is the grade-4 blade, exactly");
    // A plane through the fourth axis turns and one through e- boosts.
    let turn = one(generic::base_axis(1) | e4);
    let boost = one(e4 | EM);
    assert!(turn.geo(&turn).scalar_part() < BigRational::zero(), "e1e4 squares negative: harmonic");
    assert!(boost.geo(&boost).scalar_part() > BigRational::zero(), "e4e- squares positive: luminal");
}
