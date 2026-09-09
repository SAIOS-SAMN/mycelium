// A13, A21 — the keyed relation: what a fiber, a link and a meet read, and
// that each is a walk of structure rather than a search.


use crate::network::nerve::{Keyed, ROOT};

/// Keys bind, rebind, read, and unbind exactly; a branch left with one
/// child collapses; the walk lists everything. Prefix reads a stratum.
#[test]
fn the_keyed_relation_is_exact() {
    let mut k: Keyed<u32> = Keyed::new();
    let a = b"a".as_slice();
    let b = b"b".as_slice();
    let c = b"c".as_slice();
    assert_eq!(k.insert(a, 1), None);
    assert_eq!(k.insert(b, 2), None);
    assert_eq!(k.insert(c, 3), None);
    assert_eq!(k.get(a), Some(&1));
    assert_eq!(k.get(b), Some(&2));
    assert_eq!(k.get(c), Some(&3));
    assert_eq!(k.insert(b, 20), Some(2));
    assert_eq!(k.get(b), Some(&20));
    assert_eq!(k.len(), 3);
    assert_eq!(k.fiber(ROOT).count(), 3);
    assert_eq!(k.remove(b), Some(20));
    assert_eq!(k.get(b), None);
    assert_eq!(k.get(a), Some(&1));
    assert_eq!(k.get(c), Some(&3));
    assert_eq!(k.remove(a), Some(1));
    assert_eq!(k.remove(a), None);
    assert_eq!(k.len(), 1);
    assert_eq!(k.fiber(ROOT).count(), 1);
    assert_eq!(k.get(c), Some(&3));
}

#[test]
fn a_prefix_of_a_key_is_its_stratum() {
    let mut k: Keyed<u8> = Keyed::new();
    k.insert([1u8, 10, 0], 1);
    k.insert([1u8, 10, 1], 2);
    k.insert([1u8, 11, 0], 3);
    k.insert([2u8, 0, 0], 4);
    let coarse: Vec<_> = k.fiber(&[1]).map(|(key, v)| (key.to_vec(), *v)).collect();
    assert_eq!(coarse.len(), 3, "prefix [1] is the chart's coarse stratum");
    assert!(coarse.iter().all(|(key, _)| key.starts_with(&[1])));
    let mid: Vec<_> = k.fiber(&[1, 10]).map(|(key, _)| key.to_vec()).collect();
    assert_eq!(mid.len(), 2);
    assert!(mid.iter().all(|key| key.starts_with(&[1, 10])));
    assert_eq!(k.fiber(&[1, 10, 0]).count(), 1);
    assert_eq!(k.get(&[2, 0, 0]), Some(&4));
}

#[test]
fn a_key_and_its_proper_prefix_coexist() {
    let mut k: Keyed<u8> = Keyed::new();
    k.insert([1u8, 0], 1);
    k.insert([1u8], 2);
    assert_eq!(k.get(&[1]), Some(&2), "the prefix is bound");
    assert_eq!(k.get(&[1, 0]), Some(&1), "the extension whose next bit is 0 is bound");
    assert_eq!(k.len(), 2);
    let mut k2: Keyed<u8> = Keyed::new();
    k2.insert([1u8], 2);
    k2.insert([1u8, 0], 1);
    assert_eq!(k2.get(&[1]), Some(&2));
    assert_eq!(k2.get(&[1, 0]), Some(&1));
    assert_eq!(k2.remove(&[1]), Some(2));
    assert_eq!(k2.get(&[1]), None);
    assert_eq!(k2.get(&[1, 0]), Some(&1));
    assert_eq!(k2.len(), 1);
}

/// 45.7, S9.14 — THE CURSOR IS AN ORBIT, NOT A WALK: a carrier larger than
/// one budget is walked whole across strokes, the walk continues past the
/// last key from the first and reports the circuit, a key admitted below
/// the resume point is reached on the next circuit, and a budget beyond
/// the carrier yields every key once and ends where it began.
#[test]
fn the_cursor_is_an_orbit() {
    let mut k: Keyed<u8> = Keyed::new();
    for b in [2u8, 4, 6, 8, 10] {
        k.insert([b], b);
    }
    let mut phys = crate::physics::Physics::default();
    // Stroke 1 from the first key: three of five, no circuit yet.
    let mut budget = phys.stroke_n(3);
    let mut cur = k.cursor(ROOT, &mut budget);
    let seen: Vec<u8> = (&mut cur).map(|(_, v)| *v).collect();
    assert_eq!(seen, [2, 4, 6]);
    assert!(!cur.circuit(), "the walk has not reached the end");
    let resume = cur.last_key().map(<[u8]>::to_vec).expect("a key was yielded");
    assert_eq!(resume, [6]);
    // Stroke 2 resumes: the tail, then past the end from the first key.
    let mut budget = phys.stroke_n(3);
    let mut cur = k.cursor(&resume, &mut budget);
    let seen: Vec<u8> = (&mut cur).map(|(_, v)| *v).collect();
    assert_eq!(seen, [8, 10, 2], "the walk continues past the last key from the first");
    assert!(cur.circuit(), "one circuit of the carrier completed");
    let resume = cur.last_key().map(<[u8]>::to_vec).expect("a key was yielded");
    // A key admitted below the resume point is reached on the next circuit.
    k.insert([1u8], 1);
    let mut budget = phys.stroke_n(5);
    let mut cur = k.cursor(&resume, &mut budget);
    let seen: Vec<u8> = (&mut cur).map(|(_, v)| *v).collect();
    assert_eq!(seen, [4, 6, 8, 10, 1], "the orbit reaches the key admitted below the cursor");
    assert!(cur.circuit());
    // A budget beyond the carrier: every key once, ending where it began.
    let mut budget = phys.stroke_n(64);
    let mut cur = k.cursor(&[6], &mut budget);
    let seen: Vec<u8> = (&mut cur).map(|(_, v)| *v).collect();
    assert_eq!(seen, [8, 10, 1, 2, 4, 6], "every key once, ending where it began");
    assert!(cur.circuit());
    assert_eq!(budget.remaining(), 58, "the walk spends one read per key and no more");
    // From the first key with the same budget: one circuit, the last key last.
    let mut budget = phys.stroke_n(64);
    let mut cur = k.cursor(ROOT, &mut budget);
    assert_eq!((&mut cur).count(), 6);
    assert!(cur.circuit(), "a walk from the first key past the last is a circuit");
    assert_eq!(cur.last_key(), Some([10u8].as_slice()));
    // An empty carrier has nothing to orbit.
    let empty: Keyed<u8> = Keyed::new();
    let mut budget = phys.stroke_n(4);
    let mut cur = empty.cursor(ROOT, &mut budget);
    assert!(cur.next().is_none());
    assert!(!cur.circuit(), "no circuit on an empty carrier");
}

#[test]
fn meet_is_intersection_and_costs_shared_prefixes() {
    use crate::network::nerve::work;
    let key = |n: u8| vec![n];
    let mut a: Keyed<u8> = Keyed::new();
    let mut b: Keyed<u8> = Keyed::new();
    a.insert(key(1), 1);
    a.insert(key(2), 2);
    a.insert(key(3), 3);
    b.insert(key(2), 20);
    b.insert(key(3), 30);
    b.insert(key(4), 40);
    let m = a.meet(&b);
    assert!(m.contains(&key(2)) && m.contains(&key(3)));
    assert!(!m.contains(&key(1)) && !m.contains(&key(4)));
    // The cost claim, stated: with the shared structure fixed, growing
    // the larger trie's DISJOINT bulk costs the meet nothing, because
    // the prune stops at the branch where that bulk diverges. The
    // counter is reset AFTER construction; resetting before it measured
    // n inserts plus the meet, so the ratio was the build's own linear
    // growth and the move under test was invisible. A bound of 8x at
    // 4N passed for any implementation that did not actually explode.
    let run = |n: usize| {
        let mut big: Keyed<u8> = Keyed::new();
        let mut small: Keyed<u8> = Keyed::new();
        small.insert(key(1), 1);
        small.insert(key(2), 2);
        for i in 0..n {
            big.insert([10u8, i as u8], 1);
        }
        big.insert(key(1), 1);
        work::reset();
        let m = big.meet(&small);
        (work::total(), m.len())
    };
    let (w, held) = run(8);
    let (w4, held4) = run(32);
    assert_eq!(held, 1, "the one shared key is the meet");
    assert_eq!(held4, 1, "and stays the only one as the bulk grows");
    assert!(w > 0, "the meet is measured, not skipped");
    assert_eq!(w, w4, "4N of disjoint bulk costs the meet nothing: {w} then {w4}");
}
