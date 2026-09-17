// 50.4, A22 — the ordered factorization: a rotor spelled one commuting
// plane pair at a time, and the typed demand a block leaves.

use crate::transducer::alphabet::*;
use crate::transducer::sentence::Demand;
use num_bigint::BigInt;

fn word(tape: &[(LetterId, bool)]) -> Word {
    Word::from_letters(tape)
}

/// 37.7 P2's precondition, asserted rather than assumed: the witness is
/// the Gaussian arc `s + t·i` written out as a block, and the prime the
/// demand names divides its norm — which is what makes
/// `gcd_{ℤ[i]}(s + t·i, p)` a factor and not a unit. A witness that does
/// not carry the prime is the `REFUSE_LETTER_WITNESS` case.
fn witness_carries(w: &[[Q; 2]; 2], prime: u64) -> bool {
    let n = &w[0][0] * &w[0][0] + &w[1][0] * &w[1][0];
    n.is_integer() && (n.to_integer() % BigInt::from(prime)) == BigInt::from(0)
}

/// A rotor that is a product of letters on three planes, one per commuting
/// block, factors to a word with the same rotor, and its tape holds the
/// blocks in the order of 50.4: the sensor pair first.
#[test]
fn factor_rotor_peels_commuting_blocks_in_order() {
    let q01 = LetterId::quarter((0, 1)).expect("quarter (0,1)");
    let p23 = LetterId::pythagorean_5((2, 3)).expect("(3,4,5) on (2,3)");
    let p13 = LetterId::pythagorean_5((1, 3)).expect("(3,4,5) on (1,3)");
    let r = word(&[(q01, false), (p23, false), (p13, true)]).rotor();
    let w = factor_rotor(&r).expect("a versor").expect("spells");
    assert!(w.rotor().projective_eq(&r), "the word re-derives the rotor");
    let planes: Vec<(u8, u8)> = w.letters().iter().filter_map(|&(id, _)| plane_of(id)).collect();
    let first_13 = planes.iter().position(|p| *p == (1, 3)).expect("the (1,3) letter is on the tape");
    let last_01 = planes.iter().rposition(|p| *p == (0, 1)).expect("the (0,1) letter is on the tape");
    let last_23 = planes.iter().rposition(|p| *p == (2, 3)).expect("the (2,3) letter is on the tape");
    assert!(last_01 < first_13 && last_23 < first_13, "the first commuting pair is peeled before the second: {planes:?}");
    assert_eq!(
        factor_rotor(&Rotor::identity()).expect("a versor").expect("spells"),
        Word::identity(),
        "the identity spells as the empty word"
    );
}

/// A2's run reduction reads each run on its own plane: a run on `(2,3)`
/// survives the normal form with its rotor, where read through the sensor's
/// plane form it was the identity there and was erased.
#[test]
fn a_run_on_another_plane_survives_the_normal_form() {
    let p23 = LetterId::pythagorean_5((2, 3)).expect("(3,4,5) on (2,3)");
    let q23 = LetterId::quarter((2, 3)).expect("quarter (2,3)");
    let w = word(&[(p23, false), (q23, false), (q23, false), (p23, false)]);
    let n = w.normal_form();
    assert!(!n.is_empty(), "the run is not erased");
    assert!(n.rotor().projective_eq(&w.rotor()), "the normal form keeps the rotor");
    assert!(n.letters().len() <= w.letters().len(), "and is no longer than the tape");
}

/// A rotation whose Gaussian factor is the prime 17 — the (8,15,17) rotor,
/// outside the alphabet on every plane — leaves `RotorPrime(plane, 17)`,
/// and the plane it stood on is named.
#[test]
fn a_prime_outside_the_alphabet_is_a_rotor_prime_demand() {
    let q = |n: i64, d: i64| Q::new(BigInt::from(n), BigInt::from(d));
    // Φ = 8/17 + 15/17 e01: the rotor of the (8,15,17) rotation on the sensor plane.
    let r = Rotor::scalar(q(8, 17)).add(&Rotor::blade(0b0011, q(15, 17)));
    assert!(r.weight().is_some(), "a rotor");
    match factor_rotor(&r).expect("a versor") {
        Err(Demand::RotorPrime { plane, prime, witness }) => {
            assert_eq!(plane, (0, 1));
            assert_eq!(prime, 17);
            assert!(
                witness_carries(&witness, prime),
                "37.7 P2: the prime is read off the witness, so the witness carries it"
            );
        }
        other => panic!("expected a rotor prime demand, got {other:?}"),
    }
    let r13 = Rotor::scalar(q(8, 17)).add(&Rotor::blade(0b1010, q(15, 17)));
    match factor_rotor(&r13).expect("a versor") {
        Err(Demand::RotorPrime { plane, prime, witness }) => {
            assert_eq!(plane, (1, 3), "the demand names the plane the block stood on");
            assert_eq!(prime, 17);
            assert!(witness_carries(&witness, prime), "and the witness carries it");
        }
        other => panic!("expected a rotor prime demand on (1,3), got {other:?}"),
    }
}

/// The order of 50.4 is a presentation, not a restriction: the quarter on
/// (0,2) after the quarter on (0,1) is the same rotor as the quarter on
/// (0,1) after the quarter on (1,2), a relation of the group, and the peel
/// finds it in order. A rotor whose Tait–Bryan half-angle is irrational —
/// `1 + e01 + e02`, a versor of weight 3 — has no rational plane rotors in
/// any order and stands as a `Standing` demand, never a chain.
#[test]
fn a_block_the_order_does_not_decompose_is_a_standing_demand() {
    let q01 = LetterId::quarter((0, 1)).expect("quarter (0,1)");
    let q02 = LetterId::quarter((0, 2)).expect("quarter (0,2)");
    let r = word(&[(q02, false), (q01, false)]).rotor();
    let w = factor_rotor(&r).expect("a versor").expect("a relation of the group");
    assert!(w.rotor().projective_eq(&r), "the reordered word re-derives the rotor");
    let planes: Vec<(u8, u8)> = w.letters().iter().filter_map(|&(id, _)| plane_of(id)).collect();
    assert!(planes.contains(&(1, 2)) && !planes.contains(&(0, 2)), "spelled in order: (0,1) then (1,2): {planes:?}");

    let q = |n: i64, d: i64| Q::new(BigInt::from(n), BigInt::from(d));
    let r = Rotor::scalar(q(1, 1)).add(&Rotor::blade(0b0011, q(1, 1))).add(&Rotor::blade(0b0101, q(1, 1)));
    assert!(r.weight().is_some(), "a versor");
    match factor_rotor(&r).expect("a versor") {
        Err(Demand::Standing { block, rotor }) => {
            use num_traits::Zero;
            assert!(block.iter().flatten().any(|x| !x.is_zero()), "the standing block is the plane's own reading");
            assert!(rotor.projective_eq(&r), "and the demand carries the rotor it stood on as its witness");
        }
        other => panic!("expected a standing demand, got {other:?}"),
    }
    // The (5,12,13) letter has no place on (2,3): a rotation by it there is
    // a prime outside that plane's alphabet.
    let r23 = Rotor::scalar(q(5, 13)).add(&Rotor::blade(0b1100, q(12, 13)));
    match factor_rotor(&r23).expect("a versor") {
        Err(Demand::RotorPrime { plane, prime, witness }) => {
            assert_eq!((plane, prime), ((2, 3), 13));
            assert!(witness_carries(&witness, prime), "and the witness carries it");
        }
        other => panic!("expected the prime 13 on (2,3), got {other:?}"),
    }
}

/// L13, MEASURED. The ordered peel of 50.4 is a greedy division by the
/// alphabet's shells in a fixed order. The question the row asks is
/// whether it MISSES anything the seventeen can reach: an arc whose norm
/// factors entirely into the alphabet's own primes (5 and 13) but which
/// the ordered peel leaves standing.
///
/// Enumerated over the ALPHABET'S OWN LETTERS, which is the only
/// enumeration that answers the question. A letter's rotor is the
/// Pythagorean pair itself — the (3,4,5) letter is `3 + 4·e01`, not
/// `2 + e01` — so as a Gaussian it is `(2+i)²`, and the rotors the
/// seventeen reach on one plane are the products of `3+4i`, `5+12i`,
/// `1+i` and their conjugates. A `π` of norm 5 taken as a rotor is the
/// HALF of the (3,4,5) rotation and the alphabet genuinely does not
/// contain it; standing on that measures the enumeration, not the peel.
///
/// Every product of those generators up to total degree 4, peeled on the
/// sensor plane. The bound is the enumeration's own; this is a test
/// fixture, not a search in the engine (rule 10).
#[test]
fn l13_the_ordered_peel_against_the_alphabets_own_primes() {
    use crate::core::gaussian::Gauss;
    use crate::transducer::alphabet::{rotor_block_on, spell_peel_on, PLANE_COL, PLANE_ROW};
    use crate::transducer::peel::PeelNest;
    let plane = (PLANE_ROW, PLANE_COL);
    // The letters as Gaussians: what the alphabet actually holds here.
    let gens = [
        Gauss::from_i64(3, 4),
        Gauss::from_i64(3, -4),
        Gauss::from_i64(5, 12),
        Gauss::from_i64(5, -12),
        Gauss::from_i64(1, 1),
        Gauss::from_i64(1, -1),
    ];
    let qr = |n: &BigInt| Q::from_integer(n.clone());

    let mut tried = 0usize;
    let mut stood = Vec::new();
    let mut words: Vec<Gauss> = vec![Gauss::from_i64(1, 0)];
    for _ in 0..4usize {
        let mut next = Vec::new();
        for w in &words {
            for g in &gens {
                let p = w.mul(g);
                let rotor = Rotor::scalar(qr(&p.a)).add(&Rotor::blade(0b0011, qr(&p.b)));
                let Some(block) = rotor_block_on(&rotor, plane) else {
                    continue;
                };
                tried += 1;
                match spell_peel_on(plane, &block) {
                    Some((PeelNest::Closed { .. }, _)) => {}
                    _ => stood.push((p.a.clone(), p.b.clone())),
                }
                next.push(p);
            }
        }
        words = next;
    }
    assert!(tried > 0, "the enumeration ran");
    // THE ANSWER IS RECORDED HERE, whichever way it falls: an empty list
    // means the ordered peel reaches every arc the alphabet's primes
    // build, and L13's limit is unreachable within the seventeen; a
    // non-empty one names the arcs it misses.
    eprintln!("L13: {tried} arcs of norm 5^a·13^b tried, {} stood: {stood:?}", stood.len());
    assert!(
        stood.is_empty(),
        "the ordered peel misses {} of {tried} arcs the alphabet's own primes build: {stood:?}",
        stood.len()
    );
}
