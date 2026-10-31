// 45.8, 78.24 — the packed act: what the act carries and references, read
// back by re-deriving the rest.

use crate::chronometry::pack::{put_var, take_var, Packer, Record, Unpacker};
use crate::chronometry::*;

/// One act with nothing named.
fn step(c: &mut ReceiptChain, parents: Vec<Parent>) -> KStepReceipt {
    c.append(ActKind::List, [3, 0, 1 << 40, 7], Vec::new(), [9; 32], parents).expect("appends")
}

fn cite(r: &KStepReceipt) -> Parent {
    Parent { at: r.stamp, hash: r.chain_hash }
}

/// The nodes a stream carries, read back.
fn read(b: &[u8]) -> Option<Vec<KStepReceipt>> {
    let mut u = Unpacker::new();
    let mut p = 0usize;
    let mut out = Vec::new();
    while p < b.len() {
        if let Record::Node(r) = u.take(b, &mut p)? {
            out.push(*r);
        }
    }
    Some(out)
}

/// A COUNT IN SEVEN-BIT GROUPS reads back exactly, at every width, and a
/// group past a `u64` refuses.
#[test]
fn take_var_round_trips_and_refuses_past_u64() {
    for v in [0, 1, 127, 128, 300, 1 << 40, u64::MAX] {
        let mut b = Vec::new();
        put_var(&mut b, v);
        let mut p = 0usize;
        assert_eq!((take_var(&b, &mut p), p), (Some(v), b.len()));
    }
    let past = [0xffu8; 10];
    assert_eq!(take_var(&past, &mut 0), None, "ten groups of seven ones is past a u64");
}

/// TWO LINES CITING EACH OTHER, PACKED, READ BACK AS THEY WERE: every
/// stamp, height, predecessor and hash re-derived, none carried.
#[test]
fn packer_round_trips_two_cross_citing_lines() {
    let mut a = ReceiptChain::with_signer([1; 32]);
    let mut b = ReceiptChain::with_signer([2; 32]);
    let a0 = step(&mut a, Vec::new());
    let b0 = step(&mut b, Vec::new());
    a.admit(b0.clone());
    b.admit(a0.clone());
    let a1 = step(&mut a, vec![cite(&b0)]);
    let b1 = step(&mut b, vec![cite(&a0)]);
    let (_, frame) = a.open_orbit().expect("open");
    let a2 = a.append(ActKind::Orbit, frame, Vec::new(), [8; 32], Vec::new()).expect("the close");
    let a3 = step(&mut a, Vec::new());
    let all = vec![a0, b0, a1, b1, a2, a3];
    let mut packer = Packer::new();
    let mut stream = Vec::new();
    for r in &all {
        packer.node(&mut stream, r).expect("packs");
    }
    packer.anchors(&mut stream);
    assert_eq!(read(&stream), Some(all), "the same receipts, re-derived");
}

/// A STREAM THAT STARTS MID-LINE names its first predecessor whole, and
/// every node after it re-derives from the one before.
#[test]
fn packer_names_first_predecessor_of_mid_line_run() {
    let mut a = ReceiptChain::with_signer([1; 32]);
    let line: Vec<KStepReceipt> = (0..5).map(|_| step(&mut a, Vec::new())).collect();
    let mut packer = Packer::new();
    let mut stream = Vec::new();
    for r in &line[2..] {
        packer.node(&mut stream, r).expect("packs");
    }
    packer.anchors(&mut stream);
    assert_eq!(read(&stream).as_deref(), Some(&line[2..]));
}

/// A NODE WHOSE PREDECESSOR LEFT the ledger still reads as the receipt it
/// was: the predecessor it named by place is named by citation once the
/// place is empty.
#[test]
fn node_reads_as_sealed_once_predecessor_left() {
    let mut e = ReceiptChain::with_signer([4; 32]);
    let e0 = step(&mut e, Vec::new());
    let shut = {
        let (_, frame) = e.open_orbit().expect("open");
        e.append(ActKind::Orbit, frame, Vec::new(), [9; 32], Vec::new()).expect("the close")
    };
    let (_, root) = e.fold_before(shut.stamp.place()).expect("folds");
    let cites = vec![Cite::Root { at: Stamp::new(root.height, e.signer(), root.at), hash: root.hash }];
    let fold = e.append(ActKind::Fold, [0; 4], cites, [9; 32], Vec::new()).expect("the fold node");
    let mut c = ReceiptChain::with_signer([1; 32]);
    step(&mut c, Vec::new());
    for r in [&e0, &shut, &fold] {
        c.admit(r.clone());
    }
    assert_eq!(c.shed_folded(|_| false), 2);
    assert_eq!(c.node(&fold.stamp), Some(fold), "the fold node, its predecessor gone, reads as it was sealed");
}
