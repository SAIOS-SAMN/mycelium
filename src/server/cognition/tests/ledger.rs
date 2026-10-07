// 78.123 B — THE LEDGER TREE AS A COMPLEX: a parent fold nets children sealed
// on the court's line — aggregation by continuity, conservation at each child
// vertex, the rates cocycle read on the domain's gauge.

use crate::anastome::dialogue::uplink::{ChildNet, ChildRefusal, Discontinuous, Net, TierFold, Uplinked};
use crate::chronometry::{Cite, Stamp};
use crate::server::cognition::Kernel;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

fn who(b: u8) -> [u8; 32] {
    [b; 32]
}

/// `v` on the G lane.
fn g(v: i128) -> [i128; 4] {
    [0, v, 0, 0]
}

/// The tiers: `A` folds first; `P` and `Q` net it.
fn tiers() -> (SigningKey, SigningKey, SigningKey) {
    (key(21), key(22), key(23))
}

/// A court on which one domain names the three tiers.
fn founded() -> Kernel {
    let mut k = Kernel::new();
    found(&mut k);
    k
}

/// The one domain founded on `k`.
fn found(k: &mut Kernel) {
    use crate::anastome::dialogue::domain::{DomainGenesis, Founded};
    let (a, p, q) = tiers();
    let mut ts = vec![vk(&a), vk(&p), vk(&q)];
    ts.sort_unstable();
    let domain = key(4);
    let genesis = DomainGenesis { domain: vk(&domain), tiers: ts };
    assert!(matches!(k.admit_domain(&genesis.sign(&domain)), Founded::Sealed { .. }));
}

fn fold(tier: &SigningKey, prefix: u8, rows: Vec<Net>, declared: Vec<Net>, children: Vec<ChildNet>) -> TierFold {
    TierFold { tier: vk(tier), prefix: [prefix; 32], rows, declared, children }
}

/// `A`'s fold: `w1` +10, `w2` −4 on G, and the 6 it made declared by `w9`.
fn a_fold() -> TierFold {
    fold(&tiers().0, 1, vec![(who(1), g(10)), (who(2), g(-4))], vec![(who(9), g(6))], Vec::new())
}

/// The fold admitted, its receipt's stamp.
fn sealed(k: &mut Kernel, f: TierFold, by: &SigningKey) -> Stamp {
    match k.admit_uplink(&f.sign(by)) {
        Uplinked::Admitted { at, .. } | Uplinked::Chained { at, .. } => at,
        other => panic!("admitted: {other:?}"),
    }
}

/// A part of `of` at `at`, at the G rate `n/d`.
fn part(of: &TierFold, at: Stamp, rows: Vec<Net>, declared: Vec<Net>, n: i128, d: i128) -> ChildNet {
    ChildNet {
        at,
        rows: of.rows.clone(),
        declared: of.declared.clone(),
        included_rows: rows,
        included_declared: declared,
        rate: [(1, 1), (n, d), (1, 1), (1, 1)],
    }
}

/// A CHILD NETTED WHOLE, TRANSLATED, IS SEALED BY ITS CHILDREN'S COMMITMENT
/// AND RESTORES: `P` nets all of `A` at 2 on G — its 6 becomes 12, balanced
/// by `P`'s rows — the receipt cites the children's commitment, nothing is
/// open, and the consumed child, restored, cannot be netted again.
#[test]
fn a_child_netted_whole_is_sealed_and_its_consumption_restores() {
    let (a, p, q) = tiers();
    let mut k = founded();
    let child = a_fold();
    let at = sealed(&mut k, child.clone(), &a);
    let whole = part(&child, at, child.rows.clone(), child.declared.clone(), 2, 1);
    let parent = fold(&p, 2, vec![(who(1), g(12))], Vec::new(), vec![whole.clone()]);
    let commitment = parent.children_hash().expect("it nets a child");
    let rows = parent.rows_hash();
    let reply = k.admit_uplink(&parent.sign(&p));
    let Uplinked::Chained { provisional: false, obstructions: 0, open_remainders: 0, .. } = reply else { panic!("{reply:?}") };
    let head = k.chain.head().expect("sealed");
    assert_eq!(head.cites, vec![Cite::Chained { tier: vk(&p), prefix: [2; 32], rows, children: commitment }]);
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "the ledger tree restores with the line");
    let again = fold(&q, 3, vec![(who(1), g(1))], Vec::new(), vec![part(&child, at, vec![(who(1), g(1))], Vec::new(), 1, 1)]);
    assert_eq!(
        booted.admit_uplink(&again.sign(&q)),
        Uplinked::ChildRefused { index: 0, why: ChildRefusal::OverIncluded { lane: 1 } },
        "consumed, and held so across the restart"
    );
}

/// CONSERVATION AT THE CHILD: two parents partition it — the first leaves an
/// open remainder, the second consumes it; more than remains, or a part of
/// the wrong sign, is a double count refused by the lane it passes.
#[test]
fn parents_partition_a_child_and_a_double_count_is_refused() {
    let (a, p, q) = tiers();
    let mut k = founded();
    let child = a_fold();
    let at = sealed(&mut k, child.clone(), &a);
    let half = || part(&child, at, vec![(who(1), g(5)), (who(2), g(-2))], vec![(who(9), g(3))], 1, 1);
    let first = fold(&p, 2, vec![(who(1), g(3))], Vec::new(), vec![half()]);
    assert!(matches!(k.admit_uplink(&first.sign(&p)), Uplinked::Chained { open_remainders: 1, .. }), "value in flight");
    let wrong_sign = fold(&q, 3, vec![(who(1), g(2))], Vec::new(), vec![part(&child, at, vec![(who(2), g(2))], Vec::new(), 1, 1)]);
    assert_eq!(k.admit_uplink(&wrong_sign.sign(&q)), Uplinked::ChildRefused { index: 0, why: ChildRefusal::OverIncluded { lane: 1 } });
    let second = fold(&q, 3, vec![(who(1), g(3))], Vec::new(), vec![half()]);
    assert!(matches!(k.admit_uplink(&second.sign(&q)), Uplinked::Chained { open_remainders: 0, .. }), "consumed");
    let third = fold(&p, 4, vec![(who(1), g(1))], Vec::new(), vec![part(&child, at, vec![(who(1), g(1))], Vec::new(), 1, 1)]);
    assert_eq!(k.admit_uplink(&third.sign(&p)), Uplinked::ChildRefused { index: 0, why: ChildRefusal::OverIncluded { lane: 1 } });
}

/// A CHILD IS PROVED BEFORE IT IS NETTED: one whose rows do not match its
/// receipt's commitment, one on no receipt of this line, one folded in
/// another domain — each refused by name, nothing sealed.
#[test]
fn a_child_not_proved_on_the_line_is_refused_by_name() {
    use crate::anastome::dialogue::domain::{DomainGenesis, Founded};
    let (a, p, _) = tiers();
    let mut k = founded();
    let child = a_fold();
    let at = sealed(&mut k, child.clone(), &a);
    let mut altered = part(&child, at, child.rows.clone(), child.declared.clone(), 1, 1);
    altered.rows[0].1 = g(11);
    let parent = |c: ChildNet| fold(&p, 2, vec![(who(1), g(6))], Vec::new(), vec![c]);
    assert_eq!(k.admit_uplink(&parent(altered).sign(&p)), Uplinked::ChildRefused { index: 0, why: ChildRefusal::Mismatch });
    let nowhere = part(&child, Stamp { h: 99, ..at }, child.rows.clone(), child.declared.clone(), 1, 1);
    assert_eq!(k.admit_uplink(&parent(nowhere).sign(&p)), Uplinked::ChildRefused { index: 0, why: ChildRefusal::NotOnLine });
    let (other, x) = (key(5), key(30));
    let genesis = DomainGenesis { domain: vk(&other), tiers: vec![vk(&x)] };
    assert!(matches!(k.admit_domain(&genesis.sign(&other)), Founded::Sealed { .. }));
    let foreign = fold(&x, 1, child.rows.clone(), child.declared.clone(), Vec::new());
    let there = sealed(&mut k, foreign.clone(), &x);
    let netted = part(&foreign, there, foreign.rows.clone(), foreign.declared.clone(), 1, 1);
    assert_eq!(k.admit_uplink(&parent(netted).sign(&p)), Uplinked::ChildRefused { index: 0, why: ChildRefusal::ForeignDomain });
}

/// THE RATES ARE STRICTLY POSITIVE AND ONLY G TRANSLATES (lexicon v2): a
/// zero rate, and a rate other than one on S, are refused by the lane.
#[test]
fn a_rate_is_positive_and_only_value_translates() {
    let (a, p, _) = tiers();
    let mut k = founded();
    let child = a_fold();
    let at = sealed(&mut k, child.clone(), &a);
    let mut zero = part(&child, at, child.rows.clone(), child.declared.clone(), 1, 1);
    zero.rate[1] = (0, 1);
    let parent = |c: ChildNet| fold(&p, 2, vec![(who(1), g(6))], Vec::new(), vec![c]);
    assert_eq!(k.admit_uplink(&parent(zero).sign(&p)), Uplinked::Discontinuous(Discontinuous::Rate { lane: 1 }));
    let mut matter = part(&child, at, child.rows.clone(), child.declared.clone(), 1, 1);
    matter.rate[0] = (2, 1);
    assert_eq!(k.admit_uplink(&parent(matter).sign(&p)), Uplinked::Discontinuous(Discontinuous::Rate { lane: 0 }));
}

/// THE RATES COCYCLE: `A → P` at 2, `P → Q` at 1, and `A → Q` closing the
/// cycle. At 2 the cycle's holonomy is one and the seal is final; at 1 it is
/// exactly ½ on G — arbitrage, docketed, and the seal PROVISIONAL.
#[test]
fn a_rates_cycle_closes_at_one_or_seals_provisional_on_its_exact_gain() {
    for (rate, q_rows, provisional) in [(2, 12, false), (1, 9, true)] {
        let (a, p, q) = tiers();
        let mut k = founded();
        let child = a_fold();
        let at = sealed(&mut k, child.clone(), &a);
        let half = |n| part(&child, at, vec![(who(1), g(5)), (who(2), g(-2))], vec![(who(9), g(3))], n, 1);
        let p_fold = fold(&p, 2, vec![(who(1), g(6))], Vec::new(), vec![half(2)]);
        let p_at = sealed(&mut k, p_fold.clone(), &p);
        let p_whole = part(&p_fold, p_at, p_fold.rows.clone(), p_fold.declared.clone(), 1, 1);
        let q_fold = fold(&q, 3, vec![(who(1), g(q_rows))], Vec::new(), vec![p_whole, half(rate)]);
        let reply = k.admit_uplink(&q_fold.clone().sign(&q));
        let Uplinked::Chained { provisional: sealed_provisional, obstructions, .. } = reply else { panic!("{reply:?}") };
        assert_eq!((sealed_provisional, obstructions), (provisional, u32::from(provisional)), "rate {rate}");
        // As served: the open arbitrages of the domain.
        let open = k.open_obstructions(&vk(&key(4)));
        match provisional {
            false => assert!(open.is_empty()),
            true => {
                let [o] = open.as_slice() else { panic!("one obstruction") };
                let fold_of = |t: &TierFold| crate::server::cognition::ledger::vertex(&t.tier, &t.prefix);
                assert_eq!((&o.child, &o.parent), (&fold_of(&child), &fold_of(&q_fold)), "localized to the edge that closed it");
                assert_eq!(k.chain.head().map(|h| h.stamp), Some(o.fold), "named by the fold that closed it");
                let half = num_rational::BigRational::new(1.into(), 2.into());
                assert_eq!(o.holonomy.lanes()[1], half, "the exact gain on G");
            }
        }
    }
}

/// B′ — A RATE THAT MOVES BETWEEN EPOCHS IS NO ARBITRAGE: the vertices are
/// folds, so `A`'s fold of one epoch netted by `P` at 2 and its fold of the
/// next netted at 3 are two edges between four folds — no cycle, nothing
/// docketed, both final.
#[test]
fn a_rate_that_moves_between_epochs_is_no_arbitrage() {
    let (a, p, _) = tiers();
    let mut k = founded();
    for (epoch, rate) in [(1u8, 2i128), (5, 3)] {
        let child = TierFold { prefix: [epoch; 32], ..a_fold() };
        let at = sealed(&mut k, child.clone(), &a);
        let whole = part(&child, at, child.rows.clone(), child.declared.clone(), rate, 1);
        let parent = fold(&p, epoch + 1, vec![(who(1), g(6 * rate))], Vec::new(), vec![whole]);
        let reply = k.admit_uplink(&parent.sign(&p));
        assert!(matches!(reply, Uplinked::Chained { provisional: false, obstructions: 0, .. }), "rate {rate}: {reply:?}");
    }
    assert!(k.open_obstructions(&vk(&key(4))).is_empty(), "nothing docketed");
}

/// A witness's signed position over a root hash (as the era tests make one).
fn co_sign(name: &str, root_hash: [u8; 32]) -> crate::anastome::dialogue::ReceiptSection {
    use crate::chronometry::{ActKind, ReceiptChain};
    let key = super::common::court_key(name);
    let mut c = ReceiptChain::with_vk(key.vk_bytes());
    let head = c.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    let prefix = crate::court::admission::SignedPrefix {
        genesis: [7; 32],
        epoch: 2,
        prev_receipt_hash: head.prev_hash(),
        at: head.stamp.place(),
        height: head.stamp.h,
        chart: vec![],
        payload: root_hash.to_vec(),
    };
    let sig = key.sign(&prefix.bytes().expect("the prefix frames"));
    crate::anastome::dialogue::ReceiptSection { prefix, sig, receipt: head }
}

/// B′ — A COLD CHILD IS PROVED BY ITS SEALED COORDINATES: `A`'s fold sealed,
/// the court's line folded past it (its receipt off the hot line), and `P`
/// still nets it — from the coordinates the court held at its admission,
/// which leave once it is netted whole, surviving a restart meanwhile.
#[test]
fn a_child_folded_off_the_hot_line_is_proved_by_its_sealed_coordinates() {
    let (a, p, _) = tiers();
    let mut k = Kernel::new();
    k.bind_identity([0x5c; 32]);
    found(&mut k);
    let child = a_fold();
    let at = sealed(&mut k, child.clone(), &a);
    k.orbit();
    let proposal = match k.fold_proposal() {
        Some(pending) => pending.clone(),
        None => k.propose_fold().expect("a closed orbit proposes"),
    };
    k.commit_fold([3u8; 64], &co_sign("w", proposal.root.hash)).expect("the fold sealed");
    assert!(k.chain.node(&at).is_none(), "the child's receipt is off the hot line");
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert!(booted.ledgers.folds.contains(&at.key()), "held by its coordinates across the restart");
    let whole = part(&child, at, child.rows.clone(), child.declared.clone(), 1, 1);
    let parent = fold(&p, 2, vec![(who(1), g(6))], Vec::new(), vec![whole]);
    let reply = booted.admit_uplink(&parent.sign(&p));
    assert!(matches!(reply, Uplinked::Chained { open_remainders: 0, .. }), "{reply:?}");
    assert!(!booted.ledgers.folds.contains(&at.key()), "netted whole, it leaves the held folds");
}

/// 78.125 — A B′ LEAF READS: its gauge steps (as b3a69d03 wrote them — a
/// realm netted at 5/9 on G, as :7520 holds) come back as edges sealed by no
/// recorded fold, and the potential they re-derive is the one held; a leaf
/// carrying a B′ obstruction is refused, never dropped.
#[test]
fn a_bprime_leaf_reads_its_gauge_steps_as_edges() {
    use crate::server::cognition::ledger::{vertex, Ledgers};
    use crate::transducer::address::{put_key_bytes, put_key_count};
    use crate::transducer::cech::coefficient::{Coefficient, Rates};
    let (domain, realm, progenitor) = ([4u8; 32], vertex(&[21; 32], &[1; 32]), vertex(&[22; 32], &[2; 32]));
    let rate = Rates::of([(1, 1), (5, 9), (1, 1), (1, 1)]).expect("positive");
    let leaf = |obstructions: usize| {
        let mut v = Vec::new();
        put_key_count(&mut v, 0);
        put_key_count(&mut v, 0);
        put_key_count(&mut v, 1);
        v.extend_from_slice(&domain);
        put_key_count(&mut v, 1);
        put_key_bytes(&mut v, &realm);
        put_key_bytes(&mut v, &progenitor);
        rate.put_canonical(&mut v);
        put_key_count(&mut v, obstructions);
        v
    };
    let b = leaf(0);
    let mut p = 0;
    let read = Ledgers::take(&b, &mut p).expect("a B′ leaf reads");
    assert_eq!(p, b.len());
    let edges = read.edges.get(&domain).expect("its steps, as edges");
    assert!(matches!(edges.as_slice(), [e] if e.by.is_empty() && e.rate == rate));
    let gauge = read.gauges.get(&domain).expect("re-derived");
    assert_eq!(gauge.find(&realm), Some((progenitor.clone(), rate.clone())), "the potential held");
    assert!(Ledgers::take(&leaf(1), &mut 0).is_none(), "a B′ obstruction is refused, never dropped");
}
