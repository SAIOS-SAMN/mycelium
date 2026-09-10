// 50.2 step 2 — ONE ADDRESS, TWO PATHS: two minds whose presentations
// differ reach one vertex through different generators.

use crate::client::sdk::*;
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element::{from_connection, identity_versor};

fn plane_word(n: usize) -> Word {
    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// A generator this mind EARNED: a connection it spoke, a court ruled,
/// and the syllabus taught back (§3.4, directive 5 — never a carrier
/// filled by initialisation). The court's ruling is what makes it
/// language; `learn` is the mind receiving it.
fn taught(epi: &mut Epigenome, k: &mut crate::cognition::Kernel, w: Word) -> Element {
    let e = from_connection(&w, &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0])
        .expect("a connection on the plane");
    assert_eq!(
        k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&e)),
        vec![crate::anastome::dialogue::SPEECH_STOOD],
        "the court hears the sibling"
    );
    k.rule_all_provisionals();
    let cover = k.genesis_cover().expect("measured");
    k.enroll([2; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&e.address()), [2; 32]);
    assert!(k.ruled_contains(&e.address()), "the court ruled it");
    epi.learn(&e.to_bytes()).expect("the syllabus teaches it back");
    e
}

/// 50.2 step 2, 13.10, 23.9. The half turn is reached from the quarter
/// turn by one mind and from the three-quarter turn by the other. The
/// VERTEX is the same — identity is the rotor, projectively (1.5) — so
/// both minds speak one address; the PATHS differ, and that difference is
/// the whole of what tells two languages apart on the wire.
#[test]
fn two_presentations_reach_one_vertex_by_two_paths() {
    let half = plane_word(2);
    let mut court = crate::cognition::Kernel::new();

    let mut a = Epigenome::new();
    let ga = taught(&mut a, &mut court, plane_word(1));
    let mut b = Epigenome::new();
    let gb = taught(&mut b, &mut court, plane_word(3));

    // The two minds hold different generators, compared by the algebra
    // and never by address (rule 3, directive 1).
    assert!(
        !ga.same_vertex(&gb),
        "the two minds earned different generators"
    );

    let vertex = crate::transducer::address::Address::from_bytes([
        crate::transducer::address::TAG_FORM,
        1,
    ]);
    let pa = a.path_of(&vertex, &half).expect("A reaches the half turn");
    let pb = b.path_of(&vertex, &half).expect("B reaches the half turn");

    assert!(pa.whole, "A's generators compose to the vertex");
    assert!(pb.whole, "B's generators compose to the vertex");
    assert_eq!(
        pa.generators.first().map(|(x, _)| x.clone()),
        Some(ga.address()),
        "A cites the quarter turn it was taught"
    );
    assert_eq!(
        pb.generators.first().map(|(x, _)| x.clone()),
        Some(gb.address()),
        "B cites the three-quarter turn it was taught"
    );
    assert_ne!(
        pa.generators, pb.generators,
        "one vertex, two paths: the proofs differ where the address cannot"
    );
}

/// R2 (directive 5): every member of the working atlas has a receipt on
/// the mind's own chain naming it. Asserted as content, never on trust.
#[test]
fn every_generator_the_mind_holds_was_taught_to_it() {
    let mut court = crate::cognition::Kernel::new();
    let mut epi = Epigenome::new();
    let g = taught(&mut epi, &mut court, plane_word(1));
    let chart = crate::transducer::stalk::invariant_of(&g);
    let held = epi.working_atlas().fiber(&chart);
    assert!(
        held.iter().any(|(addr, st)| {
            *st == Standing::Ruled
                && epi.working_word(addr).is_some_and(|w| w.same_vertex(&g))
        }),
        "the generator stands Ruled because a court ruled it, and it is the vertex the court ruled"
    );
}

/// A mind that holds nothing has no path: there is no presentation to
/// factor through, and A22's spelling from the identity stands alone.
/// This is the empty voice of A's ledger criterion — it cites nothing and
/// so pays nobody.
#[test]
fn a_mind_that_holds_nothing_has_no_path() {
    let epi = Epigenome::new();
    let vertex = crate::transducer::address::Address::from_bytes([
        crate::transducer::address::TAG_FORM,
        1,
    ]);
    assert!(
        epi.path_of(&vertex, &plane_word(2)).is_none(),
        "an empty presentation reaches nothing"
    );
}
