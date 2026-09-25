// 78.37(3), FX1 — THE FLUX, READ: a form spoken through a filament and sealed
// credits it the act's frame once per act, per lane; a filament no circuit
// traverses reads zero; a lapse by extent is read under flux.

use super::super::*;

/// Thoughts standing on the given charts, each pair on one chart joined by
/// an edge, as `link_new` joins them.
fn thoughts(on: &[(u8, &[u8])], escrow: [u64; 4]) -> (Contemplation, Vec<Vec<u8>>) {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cech::Section;
    let mut c = Contemplation::new();
    let mut vs = Vec::new();
    for &(i, chart) in on {
        let v = vec![i; 4];
        let mut cover = Keyed::new();
        cover.insert(chart, ());
        let b = Bubble { escrow, cover, ..Bubble::default() };
        c.admit_incidence(&std::sync::Arc::from(v.as_slice()), &b);
        c.bubbles.insert(&v, b);
        c.complex.set_section(&v, Section::at(v.clone().into(), Rotor::identity()), [1, 0, 0, 0]);
        vs.push(v);
    }
    for (i, a) in on.iter().enumerate() {
        for b in &on[i + 1..] {
            if a.1 == b.1 {
                c.complex.set_edge(
                    vec![a.0; 4],
                    vec![b.0; 4],
                    Rotor::identity(),
                    crate::transducer::element::identity_versor(),
                    [1, 0, 0, 0],
                );
            }
        }
    }
    (c, vs)
}

fn edge(a: &[u8], b: &[u8]) -> Vec<u8> {
    crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts(vec![a, b])).as_bytes().to_vec()
}

/// Two thoughts sharing chart `c`, two more sharing chart `d`. A form whose
/// circuit visits `c` is spoken and two acts seal it: the edge on `c` is
/// credited each act's frame once, per lane; the edge on `d` reads zero. An
/// act on a word spoken from no held thought is unfiled; a form with no
/// circuit is acyclic.
#[test]
fn a_filament_a_sealed_circuit_traverses_is_credited_once_per_act() {
    let (mut c, vs) = thoughts(&[(1, b"c"), (2, b"c"), (3, b"d"), (4, b"d")], [0; 4]);
    let circuit = |charts: &[&[u8]], words: Vec<Vec<u8>>, unnamed| flux::Circuits {
        charts: if charts.is_empty() { Vec::new() } else { vec![charts.iter().map(|c| c.to_vec()).collect()] },
        words,
        unnamed,
    };
    c.file_spoken(b"spoken", circuit(&[b"c", b"c"], vec![b"w".to_vec()], 1));
    let words = c.credit_flux(b"spoken", [1, 2, 0, 3]);
    assert_eq!(words, vec![b"w".to_vec()], "the word vertices the circuit ran through");
    let _ = c.credit_flux(b"spoken", [0, 1, 0, 0]);
    let on_c = edge(&vs[0], &vs[1]);
    assert_eq!(c.flux.get(&on_c), Some(&[1, 3, 0, 3]), "once per act, per lane, a chart visited twice counted once");
    assert!(c.flux.get(&edge(&vs[2], &vs[3])).is_none(), "no circuit traversed the edge on d");
    let _ = c.credit_flux(b"never spoken", [1, 1, 1, 1]);
    c.file_spoken(b"acyclic", circuit(&[], Vec::new(), 0));
    let _ = c.credit_flux(b"acyclic", [1, 1, 1, 1]);
    c.file_spoken(b"unsealed", circuit(&[b"d"], Vec::new(), 0));
    c.flux_close();
    let f = c.meet_graph().flux;
    assert_eq!((f.orbit.traversed, f.orbit.zero, f.orbit.held), (1, 1, 0), "{f:?}");
    assert_eq!((f.credited, f.acyclic, f.unfiled, f.unsealed, f.unnamed), (2, 1, 1, 1, 1), "{f:?}");
    assert_eq!((f.closes, f.gone, f.unfilamented), (1, 0, 0));
    assert!(c.flux.is_empty() && c.spoken.is_empty(), "the orbit's flux and spoken forms leave at its close");
    c.flux_close();
    assert_eq!(c.meet_graph().flux.orbit.zero, 2, "the next orbit reads both zero");
}

/// THE EXTENT'S LAPSES, READ UNDER FLUX over each thought's OWN paid extent:
/// a thought whose edge carried flux since it was paid would be renewed;
/// one whose every edge read zero since — flux before its payment does not
/// count — would lapse by ZeroFlux; an isolated thought is unfilamented, its
/// flux undefined; a thought never paid to a height does not read.
#[test]
fn the_extent_lapsed_is_read_under_flux() {
    let (mut c, vs) = thoughts(&[(1, b"c"), (2, b"c"), (3, b"d"), (4, b"d"), (5, b"e")], [1, 0, 0, 0]);
    assert_eq!(c.under_flux(&vs[0], &Keyed::new()), UnderFlux::Unread, "never paid to a height");
    let flow = |c: &mut Contemplation, chart: &[u8]| {
        c.file_spoken(b"spoken", flux::Circuits { charts: vec![vec![chart.to_vec()]], words: Vec::new(), unnamed: 0 });
        let _ = c.credit_flux(b"spoken", [0, 0, 1, 0]);
    };
    c.set_now(10);
    flow(&mut c, b"d");
    c.flux_close();
    c.set_now(20);
    flow(&mut c, b"c");
    c.set_now(30);
    for v in [&vs[0], &vs[2], &vs[4]] {
        let b = c.bubbles.get_mut(v).expect("held");
        (b.paid_from, b.until) = (Some(15), Some(25));
    }
    let _ = c.lapse_on_clock();
    let f = c.meet_graph().flux;
    let n = |u: UnderFlux| f.extent_n[u.index()];
    assert_eq!(
        (n(UnderFlux::Renewed), n(UnderFlux::ZeroFlux), n(UnderFlux::Unfilamented), n(UnderFlux::HeldByRing)),
        (1, 1, 1, 0),
        "{f:?}"
    );
    assert_eq!(f.extent[UnderFlux::ZeroFlux.index()], [1, 0, 0, 0], "its escrow, per lane");
}
