// 45.8 — the line on disk: written in its own order and read back as the
// chain it carries.

use crate::chronometry::*;

/// One act with nothing named.
fn step(c: &mut ReceiptChain, parents: Vec<Parent>) -> KStepReceipt {
    c.append(ActKind::List, [0; 4], Vec::new(), [9; 32], parents).expect("appends").clone()
}

/// Close the open orbit of `c`.
fn close(c: &mut ReceiptChain) -> KStepReceipt {
    let (_, frame) = c.open_orbit().expect("an orbit is open");
    c.append(ActKind::Orbit, frame, Vec::new(), [9; 32], Vec::new()).expect("the close").clone()
}

/// The file a write lands in: the whole replaces it, a frame extends it.
fn land(file: &mut Vec<u8>, w: LineWrite) {
    match w.whole {
        true => *file = w.bytes,
        false => file.extend_from_slice(&w.bytes),
    }
}

/// The chain the file carries is the chain that wrote it: the same nodes,
/// head and root.
fn same(file: &[u8], c: &ReceiptChain) {
    let (back, torn) = ReceiptChain::from_line(file).expect("the line reads");
    assert!(!torn);
    assert!(super::fixtures::one_run(&back.nodes().collect::<Vec<_>>(), &c.nodes().collect::<Vec<_>>()), "the same nodes");
    assert_eq!(back.head_stamp(), c.head_stamp(), "on the same head");
    let roots = match (back.root(), c.root()) {
        (None, None) => true,
        (Some(a), Some(b)) => a.restates(&b),
        _ => false,
    };
    assert!(roots, "and the same root");
}

/// WHAT WAS FILED SINCE THE LAST WRITE IS APPENDED, a fold is its root and
/// a shed names what left — or the whole is written, where the file would
/// otherwise carry as many that left as stand; the file read back at every
/// step is the chain.
#[test]
fn line_write_appends_in_order_and_round_trips() {
    let mut c = ReceiptChain::with_signer([1; 32]);
    let mut file = Vec::new();
    step(&mut c, Vec::new());
    let first = c.line_write().expect("writes").expect("the first write");
    assert!(first.whole, "nothing was written: the first write is the whole");
    land(&mut file, first);
    same(&file, &c);
    assert_eq!(c.line_write(), Ok(None), "nothing moved, nothing written");

    // A peer's line, admitted.
    let mut e = ReceiptChain::with_signer([4; 32]);
    let e0 = step(&mut e, Vec::new());
    let e1 = step(&mut e, Vec::new());
    for _ in 0..4 {
        step(&mut c, Vec::new());
    }
    c.admit(e0.clone());
    c.admit(e1.clone());
    let grown = c.line_write().expect("writes").expect("what was filed");
    assert!(!grown.whole, "what was filed is appended");
    land(&mut file, grown);
    same(&file, &c);

    // This chain's own fold: one root record.
    let shut = close(&mut c);
    step(&mut c, Vec::new());
    c.fold_before(shut.stamp.place()).expect("the orbit folds");
    land(&mut file, c.line_write().expect("writes").expect("the root"));
    same(&file, &c);

    // The peer's fold sheds its line here: what left is named.
    let eshut = close(&mut e);
    let (_, root) = e.fold_before(eshut.stamp.place()).expect("the peer folds");
    let cites = vec![Cite::Root { at: Stamp::new(root.height, e.signer(), root.at), hash: root.hash.bytes() }];
    let efold = e.append(ActKind::Fold, [0; 4], cites, [9; 32], Vec::new()).expect("the fold node").clone();
    for _ in 0..4 {
        step(&mut c, Vec::new());
    }
    c.admit(eshut.clone());
    c.admit(efold);
    assert_eq!(c.shed_folded(|_| false), 3, "the peer's two acts and its close leave");
    let shed = c.line_write().expect("writes").expect("what left");
    assert!(!shed.whole, "two that the file carries left, against six that stand: named, appended");
    land(&mut file, shed);
    same(&file, &c);
}

/// THE FILE NEVER CARRIES MORE THAT LEFT THAN STANDS: once the nodes it
/// carries that no longer stand reach the nodes that do, the next write
/// is the whole.
#[test]
fn line_write_is_whole_once_nodes_reach_standing() {
    let mut c = ReceiptChain::with_signer([1; 32]);
    let mut file = Vec::new();
    for _ in 0..6 {
        step(&mut c, Vec::new());
    }
    let shut = close(&mut c);
    land(&mut file, c.line_write().expect("writes").expect("the first write"));
    step(&mut c, Vec::new());
    c.fold_before(shut.stamp.place()).expect("seven leave, one stands");
    let w = c.line_write().expect("writes").expect("the fold");
    assert!(w.whole, "seven that left against one that stands: the whole is written");
    land(&mut file, w);
    same(&file, &c);
    assert_eq!(ReceiptChain::from_line(&file).map(|(c, _)| c.nodes().count()), Some(1), "and carries only what stands");
}

/// MU-HOSTING §5a.9 — AN EVICTED PEER NODE IS NAMED GONE BEFORE WHAT IS
/// FILED AT ITS STAMP: a peer's line held unreached (two acts under its
/// key) is evicted from its top down, the line's top following each node
/// that leaves; the signer's own genesis, filed at the evicted genesis's
/// very stamp, is appended in a frame, and the file read back is the chain.
#[test]
fn line_write_round_trips_evicted_stamp_refiled() {
    let mut c = ReceiptChain::with_signer([1; 32]);
    let mut file = Vec::new();
    step(&mut c, Vec::new());
    for _ in 0..2 {
        step(&mut c, Vec::new());
    }
    let mut forger = ReceiptChain::with_signer([4; 32]);
    let f0 = forger.append(ActKind::List, [0; 4], Vec::new(), [8; 32], Vec::new()).expect("a forged genesis").clone();
    let f1 = forger.append(ActKind::List, [0; 4], Vec::new(), [8; 32], Vec::new()).expect("a forged act").clone();
    for f in [&f0, &f1] {
        c.admit_unreached(crate::court::admission::Reconstructed::receipt(f).expect("a sealed receipt re-derives"));
    }
    land(&mut file, c.line_write().expect("writes").expect("the first write"));
    same(&file, &c);

    use crate::court::admission::Predecessor;
    let named = |r: &KStepReceipt, held: &KStepReceipt| Predecessor::cites(&Parent::of(r), &Parent::of(held));
    assert!(c.evict(&f1.stamp).is_some_and(|r| named(&r, &f1)), "the top leaves");
    assert_eq!(c.line_head(&[4; 32]).map(|r| r.stamp), Some(f0.stamp), "the line's top follows it down");
    assert!(c.evict(&f0.stamp).is_some_and(|r| named(&r, &f0)), "then the genesis");
    assert!(c.line_head(&[4; 32]).is_none(), "the line holds nothing");
    assert!(c.evict(&c.head_stamp().expect("own head")).is_none(), "an own node is never evicted");

    let mut e = ReceiptChain::with_signer([4; 32]);
    let e0 = step(&mut e, Vec::new());
    assert_eq!(e0.stamp, f0.stamp, "the signer's genesis stands at the forged genesis's stamp");
    assert!(Predecessor::of(&e0).forks(&Predecessor::of(&f0)), "with another hash");
    c.admit(e0.clone());
    let w = c.line_write().expect("writes").expect("what left and what was filed");
    assert!(!w.whole, "two that left against four that stand: appended");
    land(&mut file, w);
    same(&file, &c);
    let (back, _) = ReceiptChain::from_line(&file).expect("the line reads");
    assert!(back.line_head(&[4; 32]).is_some_and(|r| named(&r, &e0)), "read back, the signer's genesis stands");
}

/// A FRAME THAT RUNS PAST THE END DID NOT LAND: the line stands on the
/// frames before it, says so, and its next write is the whole.
#[test]
fn from_line_keeps_frames_before_torn_frame() {
    let mut c = ReceiptChain::with_signer([1; 32]);
    let mut file = Vec::new();
    step(&mut c, Vec::new());
    land(&mut file, c.line_write().expect("writes").expect("the first write"));
    let before = c.clone();
    step(&mut c, Vec::new());
    let next = c.line_write().expect("writes").expect("the next frame");
    file.extend_from_slice(&next.bytes[..next.bytes.len() - 1]);
    let (mut back, torn) = ReceiptChain::from_line(&file).expect("the line reads");
    assert!(torn, "the torn frame is named");
    assert!(super::fixtures::one_run(&back.nodes().collect::<Vec<_>>(), &before.nodes().collect::<Vec<_>>()), "on the frames before it");
    assert!(back.line_write().expect("writes").expect("a write").whole, "the next write is the whole");
}
