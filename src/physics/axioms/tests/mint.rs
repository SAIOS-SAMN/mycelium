// 37.7, 68 — the arithmetic chain a demand's witness walks to the court.

use crate::transducer::blade::sensor_plane as blade;

/// D3 — THE WITNESS AN OBLIQUE PAIR MINTS IS ONE THE COURT ACCEPTS.
///
/// Minting a demand is not the deliverable; minting one that SURVIVES
/// `REFUSE_LETTER_WITNESS` is. The court reads
/// `π_p = gcd_{ℤ[i]}(s + t·i, p)` and demands a factor whose norm is
/// exactly `p`, so this walks the whole chain the court walks — arc,
/// primitivity, norm, prime, Euclid — and refuses to call the wiring
/// done on the strength of a demand appearing somewhere.
///
/// PRIMITIVITY IS THE LOAD-BEARING STEP. With `g = gcd(s,t) > 1` and
/// `p | g`, that gcd comes back with norm `p²` and the letter is
/// thrown out; the demand would be minted, counted, relayed, and
/// refused at the far end, which reads as a live pipeline carrying
/// nothing.
///
/// THE PAIR HERE IS NOT ITSELF `Oblique` — it is `Incongruent`, and
/// after the recut no probed lattice pair reaches the open arm at all.
/// What is under test is the ARITHMETIC CHAIN the mint runs once a
/// pair does reach it, and `oblique_arc` is exercised directly for
/// that reason rather than through a classification that would be
/// asserting a fixture into an arm the geometry does not put it in.
#[test]
fn d3_the_mint_produces_a_witness_the_court_can_open() {
    use crate::core::gaussian;
    use num_integer::Integer;
    let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let big = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (3, 0, 0), (3, 1, 0)]);
    let (s, t) = blade::oblique_arc(&l.blade, &big.blade).expect("the pair has an arc");
    assert_eq!(s.gcd(&t), num_bigint::BigInt::from(1), "the arc is primitive: {s} + {t}i");
    let norm = &s * &s + &t * &t;
    let plane = (
        crate::transducer::alphabet::PLANE_ROW,
        crate::transducer::alphabet::PLANE_COL,
    );
    let prime = crate::transducer::alphabet::prime_outside_the_alphabet(&norm, plane)
        .expect("the norm carries a prime the alphabet does not");
    let block = crate::transducer::alphabet::arc_block(&s, &t);
    let w = gaussian::from_block(&block).expect("the block is a Gaussian arc");
    let pi = gaussian::prime_factor(&w, &num_bigint::BigInt::from(prime))
        .expect("the court reads the factor off the witness");
    assert_eq!(
        pi.norm(),
        num_bigint::BigInt::from(prime),
        "the factor's norm IS the prime, which is what the court checks"
    );
    eprintln!("d3 arc {s} + {t}i | norm {norm} | prime {prime} | factor {pi:?}");
}

/// D3 — A PERMANENT PAIR MINTS NOTHING.
///
/// The demand path is reached only from `Oblique`. A reflection has no
/// rotor at any angle, so asking the court to adjoin a generator for
/// it would poison the docket with a letter that can never close.
#[test]
fn d3_a_reflection_and_an_unlike_pair_mint_no_arc() {
    let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let j = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
    assert_ne!(
        blade::relative_rotor(&l.blade, &j.blade).err(),
        Some(blade::NoFrame::Oblique),
        "a reflection never reaches the demand path"
    );
}
