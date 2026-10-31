// 16.10, 16.11 — the cover homes a word by incidence and refuses when no
// node dominates; the router finds a node by identity.

use crate::network::nerve::*;

use crate::transducer::stalk::Invariant;

fn inv(axes: &[u64]) -> Invariant {
    Invariant::counts(axes)
}

/// The cover's longest common prefix is the plane both hyphae cover, and a
/// miss at the first axis is novel.
#[test]
fn fiber_reads_plane_and_first_axis_miss_as_novel() {
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

/// The hypha XOR-nearest the hash of `chart`'s key, of `a` and `b`: the
/// tiebreak `home` reads, computed apart.
#[expect(clippy::disallowed_methods, reason = "law 2 — the tiebreak's spreading digest, computed apart to check `home` against; no identity")]
fn nearer(chart: &Invariant, a: [u8; 32], b: [u8; 32]) -> [u8; 32] {
    let k =*blake3::hash(chart.key().as_bytes()).as_bytes();
    let (da, db): ([u8; 32], [u8; 32]) = (std::array::from_fn(|i| a[i] ^ k[i]), std::array::from_fn(|i| b[i] ^ k[i]));
    match da < db {
        true => a,
        false => b,
    }
}

/// Home is chosen by room, then by the chart's nearness on the fiber, and by
/// room when the chart is novel.
#[test]
fn home_ranks_room_then_chart_nearness() {
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
    let (id, _) = home(&c, &inv(&[4]), &[1, 0, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, nearer(&inv(&[4]), lo, hi), "equal rooms: the hypha nearest the chart");
    // Unequal rooms on the occupied lane: more remaining room wins.
    rooms.insert(lo, [2, 8, 8, 8]);
    rooms.insert(hi, [8, 8, 8, 8]);
    let (id, _) = home(&c, &inv(&[4]), &[1, 0, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, hi, "more remaining room on the occupied lane homes there");
    rooms.insert(lo, [8, 2, 8, 8]);
    rooms.insert(hi, [2, 8, 8, 8]);
    let (id, _) = home(&c, &inv(&[4]), &[1, 0, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, lo, "and from the other side");
    // Incomparable rooms on two occupied lanes: no lane is ranked above
    // another, so the chart's nearness is the tiebreak, never a lane order.
    rooms.insert(lo, [2, 8, 8, 8]);
    rooms.insert(hi, [8, 2, 8, 8]);
    let (id, _) = home(&c, &inv(&[4]), &[1, 1, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, nearer(&inv(&[4]), lo, hi), "incomparable rooms fall to the chart's nearness");
    rooms.insert(lo, [8, 8, 8, 8]);
    rooms.insert(hi, [8, 8, 8, 8]);
    // Novel chart: both have room; the chart's nearness decides.
    let (id, _) = home(&c, &inv(&[99]), &[1, 0, 0, 0], &rooms, &addrs, None);
    assert_eq!(id, nearer(&inv(&[99]), lo, hi));
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

/// COURTS BONDED ALIKE SHARE THE CHARTS (2026-10-14): three hyphae with equal
/// rooms covering every chart each home some of a corpus — never one hypha
/// all of it, which is what identity order made of every tie — and a chart
/// homes at the same hypha however often it is asked.
#[test]
fn home_shares_charts_among_equal_rooms() {
    let mut c = Cover::new();
    let mut rooms = Keyed::new();
    let mut addrs = Keyed::new();
    let ids = [[0x24u8; 32], [0xceu8; 32], [0x6cu8; 32]];
    let corpus: Vec<Invariant> = (0..24u64).map(|i| inv(&[1, i])).collect();
    for (n, id) in ids.iter().enumerate() {
        for chart in &corpus {
            c.admit(0, chart, *id);
        }
        addrs.insert(*id, format!("127.0.0.1:750{n}"));
        rooms.insert(*id, [8, 8, 8, 8]);
    }
    let mut counts = [0usize; 3];
    for chart in &corpus {
        let (id, _) = home(&c, chart, &[1, 0, 0, 0], &rooms, &addrs, None);
        assert_eq!(home(&c, chart, &[1, 0, 0, 0], &rooms, &addrs, None).0, id, "one home per chart");
        let Some(n) = ids.iter().position(|x| *x == id) else { panic!("a covering hypha") };
        counts[n] += 1;
    }
    assert!(counts.iter().all(|&n| n > 0), "every court bonded alike homes some charts: {counts:?}");
}

/// Retracting a hypha leaves no stale chart on its fiber.
#[test]
fn retract_hypha_leaves_no_stale_charts() {
    let mut c = Cover::new();
    let a = [1u8; 32];
    c.admit(0, &inv(&[1]), a);
    c.retract_hypha(&a);
    assert!(c.fiber(0, &inv(&[1])).is_empty());
}

/// Three covers over one corpus: each listing homes at one hypha, and
/// the home counts sum to the corpus.
#[test]
fn home_counts_over_three_covers_sum_to_corpus() {
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

/// The trie's closest matches a brute-force XOR argmin — but by following the
/// key's bits, not a scan. (Correctness anchor; the trie is O(bits), the brute-force O(N).)
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "law 2, rule 4 — the fixture needs ids spread over the key space and a brute-force argmin to check the prefix read against; neither is cognition"
)]
fn closest_matches_brute_force_xor_argmin() {
    let ids: Vec<[u8; 32]> = (0u8..40).map(|k| *blake3::hash(&[k]).as_bytes()).collect();
    let mut r = Router::new();
    for (i, id) in ids.iter().enumerate() {
        r.insert(*id, format!("peer{i}"));
    }
    for k in [7u8, 19, 33, 200] {
        let key = *blake3::hash(&[k, 0xAA]).as_bytes();
        let brute = ids.iter().min_by_key(|id| xor(id, &key)).copied().unwrap();
        assert_eq!(r.closest(&key).unwrap().0.as_slice(), brute.as_slice(), "the trie's prefix read finds the XOR-closest");
    }
}

/// 47.3, 42.6: A FORM IS OFFERED TO THE HYPHAE COVERING ITS STRATUM, never
/// to every peer. The witness "holds ruled sections over the chart, since
/// replay needs them, so it is at nerve distance one from the home" (47.3),
/// and 42.6 bounds the mesh cover by "the number of covering hyphae" — a
/// destination set indexed by the size of the mesh is the one shape that
/// section forbids, and it is what the exchange did.
#[test]
fn witnesses_are_covering_hyphae_not_every_peer() {
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
    // `witnesses` returns the witness set for one chart: a fiber already.
    let mut got: Vec<Vec<u8>> = w.scoped().map(|(k, _)| k).collect();
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
    let mut got: Vec<Vec<u8>> = up.scoped().map(|(k, _)| k).collect();
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
    let got: Vec<Vec<u8>> = none.scoped().map(|(k, _)| k).collect();
    assert_eq!(got, vec![near.to_vec()], "lane 0's cover, by that fallback");
}

/// Lane cover room is exact arithmetic, with no invented floor.
#[test]
fn lane_cover_room_is_exact_without_invented_floor() {
    // spend == 0 returns None (cannot divide by zero)
    assert_eq!(lane_cover_room(10, 100, 0, 0), None);
    // reserve < spend gives quotient 0: bond * 0 - 0 = 0 (no invented .max(1) floor)
    assert_eq!(lane_cover_room(2, 5, 0, 10), Some(0));
    // occupancy exceeding bought cover returns None
    assert_eq!(lane_cover_room(2, 20, 5, 10), None);
    // exact positive room
    assert_eq!(lane_cover_room(2, 20, 3, 10), Some(1));
}

