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
    assert_eq!(back.nodes().cloned().collect::<Vec<_>>(), c.nodes().cloned().collect::<Vec<_>>(), "the same nodes");
    assert_eq!((back.head_stamp(), back.root()), (c.head_stamp(), c.root()), "on the same head and root");
}

/// WHAT WAS FILED SINCE THE LAST WRITE IS APPENDED, a fold is its root and
/// a shed names what left — or the whole is written, where the file would
/// otherwise carry as many that left as stand; the file read back at every
/// step is the chain.
#[test]
fn the_line_appends_in_its_own_order_and_reads_back_as_the_chain() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    let mut file = Vec::new();
    step(&mut c, Vec::new());
    let first = c.line_write().expect("writes").expect("the first write");
    assert!(first.whole, "nothing was written: the first write is the whole");
    land(&mut file, first);
    same(&file, &c);
    assert_eq!(c.line_write(), Ok(None), "nothing moved, nothing written");

    // A peer's line, admitted.
    let mut e = ReceiptChain::with_vk([4; 32]);
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
    let cites = vec![Cite::Root { at: Stamp::new(root.height, e.vk(), root.at), hash: root.hash }];
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
fn the_line_is_written_whole_once_what_left_reaches_what_stands() {
    let mut c = ReceiptChain::with_vk([1; 32]);
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

/// A FRAME THAT RUNS PAST THE END DID NOT LAND: the line stands on the
/// frames before it, says so, and its next write is the whole.
#[test]
fn a_torn_last_frame_leaves_the_line_on_the_frames_before_it() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    let mut file = Vec::new();
    step(&mut c, Vec::new());
    land(&mut file, c.line_write().expect("writes").expect("the first write"));
    let before = c.clone();
    step(&mut c, Vec::new());
    let next = c.line_write().expect("writes").expect("the next frame");
    file.extend_from_slice(&next.bytes[..next.bytes.len() - 1]);
    let (mut back, torn) = ReceiptChain::from_line(&file).expect("the line reads");
    assert!(torn, "the torn frame is named");
    assert_eq!(back.nodes().cloned().collect::<Vec<_>>(), before.nodes().cloned().collect::<Vec<_>>(), "on the frames before it");
    assert!(back.line_write().expect("writes").expect("a write").whole, "the next write is the whole");
}
