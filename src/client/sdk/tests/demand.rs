// 17.6, 16.12 — THE DEMAND: what a mind lacks when what it holds does
// not span an incoming word, and the element it says instead of a count.

use crate::client::sdk::*;

/// M6 STEP 5, A12 STEP 5: A MIND EMITS THE ELEMENT IT LACKS. When what it
/// holds does not span an incoming connection, it says the canonical
/// remainder — the exact boundary demand, unique for that lattice, with no
/// choice in it — and keeps the part that did factor. `Admitted` there,
/// "new geometry", would be said of a word the mind can name the missing
/// part of; and on the chain arm a count of residual links is what no hypha
/// can docket and no mind can reduce.
#[test]
fn a_mind_speaks_the_element_it_lacks_and_never_a_count() {
    use crate::transducer::alphabet::{LetterId, Word};
    use crate::transducer::element::{from_connection, identity_versor};

    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    let p5 = LetterId::pythagorean_5((0, 1)).expect("a prime on the same plane");
    let held = Word::from_letters(&[(q, false)]);
    let spoken = Word::from_letters(&[(q, false), (p5, false)]);

    let mut epi = Epigenome::new();
    let one = from_connection(&held, &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0])
        .expect("a connection on the plane");
    epi.learn(&one.to_bytes());
    assert_eq!(epi.genome_len(), 1, "the mind holds the quarter turn");

    // The quarter turn IS held; the prime is not. The demand must ask for
    // the prime and nothing else.
    let ask = from_connection(&spoken, &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0])
        .expect("a connection on the plane");
    let Comprehension::Novel { demand, understood } = epi.comprehend(&ask) else {
        panic!("a word outside the span is a demand, got {:?}", epi.comprehend(&ask));
    };
    let asked = demand
        .letters()
        .expect("the demand is a connection")
        .spelling
        .word
        .form()
        .expect("on the plane");
    assert!(
        asked.rot.iter().any(|&x| x != 0),
        "it asks for something: the prime is genuinely missing"
    );
    assert!(
        (1..=6).all(|u| asked.rot[u] == 0),
        "and never for the quarter turn it already holds: {:?}",
        asked.rot
    );
    assert!(
        !understood.is_empty(),
        "the part that did factor is kept, not thrown away with the rest"
    );

    // THE DEMAND IS THE SAME DEMAND ON EVERY MIND THAT HOLDS THE SAME
    // WORDS. The HNF residue is canonical, so two minds speak one element.
    let mut twin = Epigenome::new();
    twin.learn(&one.to_bytes());
    let Comprehension::Novel { demand: twins, .. } = twin.comprehend(&ask) else {
        panic!("the twin reaches the same arm");
    };
    assert!(
        demand.same_vertex(&twins),
        "one lattice, one residue, one demand — by vertex, not by bytes"
    );

    // AND A DEMAND CAN BE SPOKEN. A count could not: this is a connection
    // with no chain, which a hypha dockets as any word (M6 step 5).
    assert!(demand.is_connection(), "the demand is an element on the wire");
    assert!(demand.sub().is_empty(), "spoken as a connection with no chain");
    let mut pos = 0usize;
    let back = crate::transducer::element::Element::from_bytes(&demand.to_bytes(), &mut pos)
        .expect("it round-trips as any word does");
    assert!(back.same_vertex(&demand), "and re-derives to the same vertex");
}

/// 42.6, A21 §2: THE BASIS IS THE STRATUM, not the exact chart. A
/// connection's chart is its rotor's trace and determinant (M4 step 2), so
/// two different rotations never share one, and a basis read at the exact
/// chart is empty or a single word: nothing to factor through, and the
/// mind said `Admitted` to a word it held the generators of one axis away.
/// The stratum walks out by `peel` to the first fibre that holds anything
/// and no further.
#[test]
fn the_factoring_basis_is_read_on_the_stratum_not_the_exact_chart() {
    use crate::transducer::alphabet::{LetterId, Word};
    use crate::transducer::element::{from_connection, identity_versor};

    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    let p5 = LetterId::pythagorean_5((0, 1)).expect("a prime on the same plane");
    let mut epi = Epigenome::new();
    let one = from_connection(&Word::from_letters(&[(q, false)]), &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0])
        .expect("a connection on the plane");
    epi.learn(&one.to_bytes());
    let ask = from_connection(&Word::from_letters(&[(q, false), (p5, false)]), &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0])
        .expect("a connection on the plane");

    let exact = crate::transducer::stalk::invariant_of(&ask);
    assert!(
        epi.working_atlas().fiber(&exact).is_empty(),
        "the exact chart of another rotation holds nothing to factor through"
    );
    let (at, generators) = epi.working_atlas().stratum(&exact);
    assert_eq!(generators.len(), 1, "the stratum holds the one word the mind has");
    assert_eq!(
        generators[0].0.as_slice(),
        one.address().as_bytes(),
        "and it is the quarter turn"
    );
    assert!(at.0.len() < exact.0.len(), "read one or more axes coarser than the exact chart");
    assert!(
        matches!(epi.comprehend(&ask), Comprehension::Novel { .. }),
        "so the word is factored and its remainder is the demand, never Admitted"
    );
}
