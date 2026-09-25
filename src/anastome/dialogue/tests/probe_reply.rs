// PR1, 26, 34.3 — THE PROBE REPLY IS A READING: the settled μ⃗ per lane,
// the closure with its versor, the remaining μ⃗, and no outcome code.

use crate::anastome::dialogue::*;

fn q(n: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(num_bigint::BigInt::from(n))
}

/// A residual's versor: the tether between two unit spheres.
fn versor(a: i64, b: i64) -> crate::transducer::cga::Mv {
    let sa = crate::transducer::cga::sphere_at(&[q(a), q(0), q(0)], &q(1));
    let sb = crate::transducer::cga::sphere_at(&[q(b), q(0), q(0)], &q(1));
    crate::transducer::blade::tether_of(&sa, &sb)
}

/// Every closure state rides the wire whole — a residual with its versor
/// as its ray and one whose versor did not read, by name — and a reply with
/// bytes past its reading is refused.
#[test]
fn the_probe_reply_carries_its_reading_and_no_code() {
    let readings = [
        ProbeReading { settled: [1, 0, 2, 0], closure: ProbeClosure::Closed, remaining: [0; 4] },
        ProbeReading { settled: [0, 3, 0, 0], closure: ProbeClosure::Residual(Some(versor(0, 3))), remaining: [0, 0, 1, 4] },
        ProbeReading { settled: [0; 4], closure: ProbeClosure::Residual(None), remaining: [2, 0, 0, 0] },
        ProbeReading { settled: [0; 4], closure: ProbeClosure::Nothing, remaining: [5, 5, 5, 5] },
        ProbeReading::gone(),
    ];
    for r in &readings {
        let wire = encode_probe_reply(r);
        assert_eq!(decode_probe_reply(&wire).as_ref(), Some(r), "{r:?} reads back whole");
        let mut long = wire.clone();
        long.push(0);
        assert!(decode_probe_reply(&long).is_none(), "a byte past the reading is refused");
    }
    // λV is one residual: the wire writes the ray.
    let two = q(2);
    let scaled = ProbeReading { closure: ProbeClosure::Residual(Some(versor(0, 3).scale(&two))), ..readings[1].clone() };
    assert_eq!(decode_probe_reply(&encode_probe_reply(&scaled)).as_ref(), Some(&readings[1]));
}

/// THE VOICE'S TALLY keys a residual's versor by its CLASS: two probes whose
/// residuals stand off by one versor (and by λ of it) are one class counted
/// twice; another versor of another class is another entry; a residual whose
/// versor did not read is counted apart. Lanes sum per lane.
#[test]
fn the_probe_tally_keys_residual_versors_by_their_class() {
    let mut t = crate::observe::ProbeTally::default();
    let residual = |v: Option<crate::transducer::cga::Mv>| ProbeReading {
        settled: [1, 0, 0, 2],
        closure: ProbeClosure::Residual(v),
        remaining: [0, 1, 0, 0],
    };
    t.note(&residual(Some(versor(0, 3))));
    t.note(&residual(Some(versor(0, 3).scale(&q(5)))));
    t.note(&residual(Some(versor(0, 1))));
    t.note(&residual(None));
    t.note(&ProbeReading { settled: [4, 0, 0, 0], closure: ProbeClosure::Closed, remaining: [0; 4] });
    assert_eq!((t.closed, t.residual, t.residual_unread), (1, 4, 1));
    assert_eq!(t.settled, [8, 0, 0, 8], "summed per lane");
    assert_eq!(t.remaining, [0, 4, 0, 0]);
    let counts: Vec<u64> = t.classes.iter().map(|(_, n)| *n).collect();
    assert_eq!(counts.iter().sum::<u64>() + t.unclassed, 3, "three residual versors read");
    assert!(counts.contains(&2), "one class met twice: {:?}", t.classes);
}
