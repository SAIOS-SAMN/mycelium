// 16.10, 16.11 — the cover homes a word by incidence and refuses when no
// node dominates; the router finds a node by identity.

use crate::network::nerve::*;

use crate::transducer::stalk::Invariant;

fn inv(axes: &[u64]) -> Invariant {
    Invariant(axes.to_vec())
}

#[test]
fn cover_lcp_is_the_plane_and_a_miss_at_the_first_axis_is_novel() {
    let mut c = Cover::new();
    let a = [1u8; 32];
    let b = [2u8; 32];
    c.admit(0, &inv(&[1, 2, 3]), a);
    c.admit(0, &inv(&[1, 2, 9]), b);
    let plane = c.fiber(0, &inv(&[1, 2]));
    assert!(plane.iter().any(|x| x.as_slice() == a.as_slice()) && plane.iter().any(|x| x.as_slice() == b.as_slice()), "prefix 1,2 is the plane both cover");
    assert_eq!(c.fiber(0, &inv(&[1, 2, 3])), vec![a.to_vec()], "exact chart is one hypha");
    assert_eq!(c.multiplicity(0, &inv(&[1, 2])), 2, "multiplicity is the vertex count");
    assert!(c.fiber(0, &inv(&[7, 8])).is_empty(), "a first-axis miss is novel");
    let refined = inv(&[1, 2, 3, 7]);
    c.rehome(0, &inv(&[1, 2, 3]), &refined, a);
    assert_eq!(c.fiber(0, &refined), vec![a.to_vec()], "the hypha re-homes under the refined prefix");
    assert_eq!(c.multiplicity(0, &inv(&[1, 2])), 2, "the plane still has both covering vertices");
}

#[test]
fn home_is_identity_order_on_the_fiber_and_room_when_novel() {
    let mut c = Cover::new();
    let mut rooms = Keyed::new();
    let mut addrs = Keyed::new();
    let lo = [1u8; 32];
    let hi = [9u8; 32];
    c.admit(0, &inv(&[4]), lo);
    c.admit(0, &inv(&[4]), hi);
    addrs.insert(lo, "127.0.0.1:7500".into());
    addrs.insert(hi, "127.0.0.1:7501".into());
    rooms.insert(lo, [8, 8, 8, 8]);
    rooms.insert(hi, [8, 8, 8, 8]);
    let (id, addr) = home(&c, &inv(&[4]), &[1, 0, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, lo, "equal rooms: identity is the last tiebreak");
    assert_eq!(addr, "127.0.0.1:7500");
    // Unequal rooms on the occupied lane: more remaining room wins.
    rooms.insert(lo, [2, 8, 8, 8]);
    rooms.insert(hi, [8, 8, 8, 8]);
    let (id, _) = home(&c, &inv(&[4]), &[1, 0, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, hi, "more remaining room on the occupied lane homes there");
    // Incomparable rooms on two occupied lanes: no lane is ranked above
    // another, so identity is the tiebreak (lexicographic room would pick hi).
    rooms.insert(lo, [2, 8, 8, 8]);
    rooms.insert(hi, [8, 2, 8, 8]);
    let (id, _) = home(&c, &inv(&[4]), &[1, 1, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, lo, "incomparable rooms fall to identity, never a lane order");
    rooms.insert(lo, [8, 8, 8, 8]);
    rooms.insert(hi, [8, 8, 8, 8]);
    // Novel chart: both have room; identity still wins.
    let (id, _) = home(&c, &inv(&[99]), &[1, 0, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, lo);
    // Room: hi has room on the residual lane, lo does not.
    rooms.insert(lo, [8, 0, 8, 8]);
    rooms.insert(hi, [0, 3, 0, 0]);
    let (id, _) = home(&c, &inv(&[99]), &[0, 1, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, hi, "novel home is the hypha with room on every moved lane");
    // Both cover the chart; lo has no room on the occupied lane, hi does.
    rooms.insert(lo, [8, 0, 8, 8]);
    rooms.insert(hi, [0, 3, 0, 0]);
    let (id, _) = home(&c, &inv(&[4]), &[0, 1, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, hi, "fiber ∩ room: covering without room loses to covering with room");
}

#[test]
fn retract_hypha_does_not_leave_stale_charts() {
    let mut c = Cover::new();
    let a = [1u8; 32];
    c.admit(0, &inv(&[1]), a);
    c.retract_hypha(&a);
    assert!(c.fiber(0, &inv(&[1])).is_empty());
}

/// Three covers over one corpus: each listing homes at one hypha, and
/// the home counts sum to the corpus.
#[test]
fn three_covers_home_counts_sum_to_the_corpus() {
    let mut c = Cover::new();
    let mut rooms = Keyed::new();
    let mut addrs = Keyed::new();
    let ha = [1u8; 32];
    let hb = [2u8; 32];
    let hc = [3u8; 32];
    c.admit(0, &inv(&[10]), ha);
    c.admit(0, &inv(&[20]), hb);
    c.admit(0, &inv(&[30]), hc);
    addrs.insert(ha, "127.0.0.1:7500".into());
    addrs.insert(hb, "127.0.0.1:7501".into());
    addrs.insert(hc, "127.0.0.1:7502".into());
    rooms.insert(ha, [8, 8, 8, 8]);
    rooms.insert(hb, [8, 8, 8, 8]);
    rooms.insert(hc, [8, 8, 8, 8]);
    let corpus: Vec<Invariant> = (0..9).map(|i| inv(&[10 + 10 * (i % 3)])).collect();
    let mut counts = [0usize; 3];
    for chart in &corpus {
        let (id, _) = home(&c, chart, &[1, 0, 0, 0], &rooms, &addrs, None);
        match id {
            x if x == ha => counts[0] += 1,
            x if x == hb => counts[1] += 1,
            x if x == hc => counts[2] += 1,
            _ => panic!("a corpus listing must home on a covering hypha, got {id:?}"),
        }
    }
    assert_eq!(counts[0] + counts[1] + counts[2], corpus.len(), "home counts sum to the corpus");
    assert!(counts.iter().all(|&n| n > 0), "each cover homes some of the corpus");
}


fn xor(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    std::array::from_fn(|i| a[i] ^ b[i])
}

/// The trie's closest matches a brute-force XOR argmin — but by descent, not
/// a scan. (Correctness anchor; the trie is O(bits), the brute-force O(N).)
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "law 2, rule 4 — the fixture needs ids spread over the key space and a brute-force argmin to check the descent against; neither is cognition"
)]
fn trie_closest_matches_brute_force_without_scanning() {
    let ids: Vec<[u8; 32]> = (0u8..40).map(|k| *blake3::hash(&[k]).as_bytes()).collect();
    let mut r = Router::new();
    for (i, id) in ids.iter().enumerate() {
        r.insert(*id, format!("peer{i}"));
    }
    for k in [7u8, 19, 33, 200] {
        let key = *blake3::hash(&[k, 0xAA]).as_bytes();
        let brute = ids.iter().min_by_key(|id| xor(id, &key)).copied().unwrap();
        assert_eq!(r.closest(&key).unwrap().0.as_slice(), brute.as_slice(), "trie descent finds the XOR-closest");
    }
}

/// 47.3, 42.6: A FORM IS OFFERED TO THE HYPHAE COVERING ITS STRATUM, never
/// to every peer. The witness "holds ruled sections over the chart, since
/// replay needs them, so it is at nerve distance one from the home" (47.3),
/// and 42.6 bounds the mesh cover by "the number of covering hyphae" — a
/// destination set indexed by the size of the mesh is the one shape that
/// section forbids, and it is what the exchange did.
#[test]
fn a_form_is_offered_to_the_hyphae_covering_its_chart_and_not_to_every_peer() {
    let home = [1u8; 32];
    let near = [2u8; 32];
    let far = [3u8; 32];
    let elsewhere = [4u8; 32];
    let chart = inv(&[1, 2, 3]);
    let mut c = Cover::new();
    // Three hyphae on the form's lane: the home, one on its exact chart,
    // one on the plane above it. A fourth covers a different chart, and a
    // fifth covers this chart on a lane the form does not occupy.
    c.admit(0, &chart, home);
    c.admit(0, &chart, near);
    c.admit(0, &inv(&[1, 2]), far);
    c.admit(0, &inv(&[7, 7]), elsewhere);
    c.admit(2, &chart, [5u8; 32]);

    // A form occupying lane 0 only.
    let w = c.witnesses(&chart, &[3, 0, 0, 0], &home);
    let mut got: Vec<Vec<u8>> = w.fiber(ROOT).map(|(k, _)| k).collect();
    got.sort();
    assert_eq!(
        got,
        vec![near.to_vec()],
        "the hyphae on the form's own chart, and this home is not its own witness"
    );
    assert!(
        !got.iter().any(|v| v.as_slice() == elsewhere.as_slice()),
        "a hypha covering another chart is not at nerve distance one"
    );
    assert!(
        !got.iter().any(|v| v.as_slice() == [5u8; 32].as_slice()),
        "nor is one covering this chart on a lane the form does not occupy"
    );

    // A form occupying the plane above resolves to the plane's cover.
    let up = c.witnesses(&inv(&[1, 2]), &[3, 0, 0, 0], &home);
    let mut got: Vec<Vec<u8>> = up.fiber(ROOT).map(|(k, _)| k).collect();
    got.sort();
    assert_eq!(got, vec![near.to_vec(), far.to_vec()], "the plane both cover");

    // A chart no hypha covers has no eligible witness. The form waits;
    // that is 47.3's answer, and it is not the same as reaching nobody.
    assert!(
        c.witnesses(&inv(&[9, 9]), &[3, 0, 0, 0], &home).is_empty(),
        "a form no hypha covers has no witness and is not broadcast"
    );
    // A form measuring nothing falls back to lane 0 — `occupied_lanes`'s
    // own rule for an empty reading — so it is witnessed on that lane and
    // nowhere else, never on all four.
    let none = c.witnesses(&chart, &[0; 4], &home);
    let got: Vec<Vec<u8>> = none.fiber(ROOT).map(|(k, _)| k).collect();
    assert_eq!(got, vec![near.to_vec()], "lane 0's cover, by that fallback");
}
