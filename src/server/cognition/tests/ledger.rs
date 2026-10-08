// 78.123 B — THE LEDGER TREE AS A COMPLEX: a parent fold nets children on the
// court — aggregation by continuity, conservation at each child vertex, the
// rates cocycle read on the domain's gauge. 78.138 — the lines `A`, `P`, `Q`
// never touch the court: each of their folds is CARRIED INLINE in a fold of
// the domain's reserve `R`, which includes none of it, so it stands to be
// netted where it stands inside that receipt.

use crate::anastome::dialogue::uplink::{Child, ChildNet, ChildRefusal, Discontinuous, FoldAt, Net, TierFold, Uplinked};
use crate::chronometry::Stamp;
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

/// The lines: `A` folds first; `P` and `Q` net it.
fn tiers() -> (SigningKey, SigningKey, SigningKey) {
    (key(21), key(22), key(23))
}

/// The domain's reserve — the only tier that uplinks.
fn reserve() -> SigningKey {
    key(24)
}

/// A court on which one domain names the three lines and its reserve.
fn founded() -> Kernel {
    let mut k = Kernel::new();
    found(&mut k);
    k
}

/// The one domain founded on `k`, its reserve designated, each line bonded.
fn found(k: &mut Kernel) {
    use crate::anastome::dialogue::domain::{Bond, DomainGenesis, Founded, Reservation};
    let (a, p, q) = tiers();
    let r = reserve();
    let mut ts = vec![vk(&a), vk(&p), vk(&q), vk(&r)];
    ts.sort_unstable();
    let domain = key(4);
    let genesis = DomainGenesis { domain: vk(&domain), tiers: ts };
    assert!(matches!(k.admit_domain(&genesis.sign(&domain)), Founded::Sealed { .. }));
    let reservation = Reservation { domain: vk(&domain), tier: vk(&r), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&reservation.sign(&domain)), Founded::Reserved { .. }));
    let anchor = k.anchor();
    k.treasury.genesis(vk(&r), [3, 0, 0, 0], anchor);
    for (i, line) in [(0u8, &a), (1, &p), (2, &q)] {
        let bond = Bond { domain: vk(&domain), reserve: vk(&r), tier: vk(line), mu: Some([1, 0, 0, 0]), nonce: [i; 32] };
        assert!(matches!(k.bond(&bond.sign(&r)), Founded::Bonded { .. }));
    }
}

fn fold(tier: &SigningKey, prefix: u8, rows: Vec<Net>, declared: Vec<Net>, children: Vec<ChildNet>) -> TierFold {
    TierFold { tier: vk(tier), prefix: [prefix; 32], rows, declared, children, handoffs: Vec::new() }
}

/// `A`'s fold: `w1` +10, `w2` −4 on G, and the 6 it made declared by `w9`.
fn a_fold() -> TierFold {
    fold(&tiers().0, 1, vec![(who(1), g(10)), (who(2), g(-4))], vec![(who(9), g(6))], Vec::new())
}

/// THE LINE'S FOLD CARRIED in a fresh fold of the reserve, which includes
/// none of it: the reserve fold's answer.
fn carry(k: &mut Kernel, f: TierFold, by: &SigningKey) -> Uplinked {
    let carried = ChildNet {
        rows: f.rows.clone(),
        declared: f.declared.clone(),
        child: Child::Carried(Box::new(f.sign(by))),
        included_rows: Vec::new(),
        included_declared: Vec::new(),
        rate: [(1, 1); 4],
    };
    let mut prefix = [7u8; 32];
    prefix[..8].copy_from_slice(&k.chain.head_height().to_be_bytes());
    let carrier = TierFold { tier: vk(&reserve()), prefix, rows: Vec::new(), declared: Vec::new(), children: vec![carried], handoffs: Vec::new() };
    k.admit_uplink(&carrier.sign(&reserve()))
}

/// The line's fold carried, and where it stands: `[0]` inside the reserve's
/// receipt.
fn sealed(k: &mut Kernel, f: TierFold, by: &SigningKey) -> FoldAt {
    match carry(k, f, by) {
        Uplinked::Chained { at, .. } => FoldAt { stamp: at, path: vec![0] },
        other => panic!("carried: {other:?}"),
    }
}

/// A part of `of` at `at`, at the G rate `n/d`.
fn part(of: &TierFold, at: &FoldAt, rows: Vec<Net>, declared: Vec<Net>, n: i128, d: i128) -> ChildNet {
    ChildNet {
        child: Child::Sealed(at.clone()),
        rows: of.rows.clone(),
        declared: of.declared.clone(),
        included_rows: rows,
        included_declared: declared,
        rate: [(1, 1), (n, d), (1, 1), (1, 1)],
    }
}

/// A CHILD NETTED WHOLE, TRANSLATED, IS HELD BY ITS CHILDREN'S COMMITMENT AND
/// RESTORES: `P` nets all of `A` at 2 on G — its 6 becomes 12, balanced by
/// `P`'s rows — `P`'s record holds the children's commitment, nothing is
/// open but `P` itself (carried whole, to be netted in its turn), and the
/// consumed child, restored, cannot be netted again.
#[test]
fn a_child_netted_whole_is_sealed_and_its_consumption_restores() {
    let (a, p, q) = tiers();
    let mut k = founded();
    let child = a_fold();
    let at = sealed(&mut k, child.clone(), &a);
    let whole = part(&child, &at, child.rows.clone(), child.declared.clone(), 2, 1);
    let parent = fold(&p, 2, vec![(who(1), g(12))], Vec::new(), vec![whole.clone()]);
    let commitment = parent.children_hash().expect("it nets a child");
    let reply = carry(&mut k, parent, &p);
    let Uplinked::Chained { at: r_at, provisional: false, obstructions: 0, open_remainders: 1, .. } = reply else { panic!("{reply:?}") };
    let p_at = FoldAt { stamp: r_at, path: vec![0] };
    assert_eq!(k.ledgers.records.get(&p_at.key()).and_then(|r| r.children), Some(commitment), "P held by its children's commitment");
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "the ledger tree restores with the line");
    let again = fold(&q, 3, vec![(who(1), g(1))], Vec::new(), vec![part(&child, &at, vec![(who(1), g(1))], Vec::new(), 1, 1)]);
    assert_eq!(
        carry(&mut booted, again, &q),
        Uplinked::ChildRefused { index: 0, why: ChildRefusal::OverIncluded { lane: 1 } },
        "consumed, and held so across the restart"
    );
}

/// CONSERVATION AT THE CHILD: two parents partition it — the first leaves an
/// open remainder, the second consumes it; more than remains, or a part of
/// the wrong sign, is a double count refused by the lane it passes. (Each
/// carried parent is itself open, to be netted in its turn.)
#[test]
fn parents_partition_a_child_and_a_double_count_is_refused() {
    let (a, p, q) = tiers();
    let mut k = founded();
    let child = a_fold();
    let at = sealed(&mut k, child.clone(), &a);
    let half = || part(&child, &at, vec![(who(1), g(5)), (who(2), g(-2))], vec![(who(9), g(3))], 1, 1);
    let first = fold(&p, 2, vec![(who(1), g(3))], Vec::new(), vec![half()]);
    assert!(matches!(carry(&mut k, first, &p), Uplinked::Chained { open_remainders: 2, .. }), "value in flight, and P");
    let wrong_sign = fold(&q, 3, vec![(who(1), g(2))], Vec::new(), vec![part(&child, &at, vec![(who(2), g(2))], Vec::new(), 1, 1)]);
    assert_eq!(carry(&mut k, wrong_sign, &q), Uplinked::ChildRefused { index: 0, why: ChildRefusal::OverIncluded { lane: 1 } });
    let second = fold(&q, 3, vec![(who(1), g(3))], Vec::new(), vec![half()]);
    assert!(matches!(carry(&mut k, second, &q), Uplinked::Chained { open_remainders: 1, .. }), "A consumed; Q itself open");
    let third = fold(&p, 4, vec![(who(1), g(1))], Vec::new(), vec![part(&child, &at, vec![(who(1), g(1))], Vec::new(), 1, 1)]);
    assert_eq!(carry(&mut k, third, &p), Uplinked::ChildRefused { index: 0, why: ChildRefusal::OverIncluded { lane: 1 } });
}

/// A CHILD IS PROVED BEFORE IT IS NETTED: one whose rows do not match its
/// commitment, one standing nowhere on this court, one folded in another
/// domain — each refused by name, nothing sealed.
#[test]
fn a_child_not_proved_on_the_line_is_refused_by_name() {
    use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
    let (a, p, _) = tiers();
    let mut k = founded();
    let child = a_fold();
    let at = sealed(&mut k, child.clone(), &a);
    let mut altered = part(&child, &at, child.rows.clone(), child.declared.clone(), 1, 1);
    altered.rows[0].1 = g(11);
    let parent = |c: ChildNet| fold(&p, 2, vec![(who(1), g(6))], Vec::new(), vec![c]);
    assert_eq!(carry(&mut k, parent(altered), &p), Uplinked::ChildRefused { index: 0, why: ChildRefusal::Mismatch });
    let nowhere = FoldAt { stamp: Stamp { h: 99, ..at.stamp }, path: vec![0] };
    let lost = part(&child, &nowhere, child.rows.clone(), child.declared.clone(), 1, 1);
    assert_eq!(carry(&mut k, parent(lost), &p), Uplinked::ChildRefused { index: 0, why: ChildRefusal::NotOnLine });
    let (other, x) = (key(5), key(30));
    let genesis = DomainGenesis { domain: vk(&other), tiers: vec![vk(&x)] };
    assert!(matches!(k.admit_domain(&genesis.sign(&other)), Founded::Sealed { .. }));
    let reservation = Reservation { domain: vk(&other), tier: vk(&x), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&reservation.sign(&other)), Founded::Reserved { .. }));
    let foreign = fold(&x, 1, child.rows.clone(), child.declared.clone(), Vec::new());
    let Uplinked::Admitted { at: there, .. } = k.admit_uplink(&foreign.clone().sign(&x)) else { panic!("its own reserve's") };
    let netted = part(&foreign, &FoldAt::sealed(there), foreign.rows.clone(), foreign.declared.clone(), 1, 1);
    assert_eq!(carry(&mut k, parent(netted), &p), Uplinked::ChildRefused { index: 0, why: ChildRefusal::ForeignDomain });
}

/// THE RATES ARE STRICTLY POSITIVE AND ONLY G TRANSLATES (lexicon v2): a
/// zero rate, and a rate other than one on S, are refused by the lane.
#[test]
fn a_rate_is_positive_and_only_value_translates() {
    let (a, p, _) = tiers();
    let mut k = founded();
    let child = a_fold();
    let at = sealed(&mut k, child.clone(), &a);
    let mut zero = part(&child, &at, child.rows.clone(), child.declared.clone(), 1, 1);
    zero.rate[1] = (0, 1);
    let parent = |c: ChildNet| fold(&p, 2, vec![(who(1), g(6))], Vec::new(), vec![c]);
    assert_eq!(carry(&mut k, parent(zero), &p), Uplinked::Discontinuous(Discontinuous::Rate { lane: 1 }));
    let mut matter = part(&child, &at, child.rows.clone(), child.declared.clone(), 1, 1);
    matter.rate[0] = (2, 1);
    assert_eq!(carry(&mut k, parent(matter), &p), Uplinked::Discontinuous(Discontinuous::Rate { lane: 0 }));
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
        let half = |n| part(&child, &at, vec![(who(1), g(5)), (who(2), g(-2))], vec![(who(9), g(3))], n, 1);
        let p_fold = fold(&p, 2, vec![(who(1), g(6))], Vec::new(), vec![half(2)]);
        let p_at = sealed(&mut k, p_fold.clone(), &p);
        let p_whole = part(&p_fold, &p_at, p_fold.rows.clone(), p_fold.declared.clone(), 1, 1);
        let q_fold = fold(&q, 3, vec![(who(1), g(q_rows))], Vec::new(), vec![p_whole, half(rate)]);
        let reply = carry(&mut k, q_fold.clone(), &q);
        let Uplinked::Chained { at: r_at, provisional: sealed_provisional, obstructions, .. } = reply else { panic!("{reply:?}") };
        assert_eq!((sealed_provisional, obstructions), (provisional, u32::from(provisional)), "rate {rate}");
        // As served: the open arbitrages of the domain.
        let open = k.open_obstructions(&vk(&key(4)));
        match provisional {
            false => assert!(open.is_empty()),
            true => {
                let [o] = open.as_slice() else { panic!("one obstruction") };
                let fold_of = |t: &TierFold| crate::server::cognition::ledger::vertex(&t.tier, &t.prefix);
                assert_eq!((&o.child, &o.parent), (&fold_of(&child), &fold_of(&q_fold)), "localized to the edge that closed it");
                assert_eq!(o.fold, FoldAt { stamp: r_at, path: vec![0] }, "named by the fold that closed it, where it stands");
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
        let whole = part(&child, &at, child.rows.clone(), child.declared.clone(), rate, 1);
        let parent = fold(&p, epoch + 1, vec![(who(1), g(6 * rate))], Vec::new(), vec![whole]);
        let reply = carry(&mut k, parent, &p);
        assert!(matches!(reply, Uplinked::Chained { provisional: false, obstructions: 0, .. }), "rate {rate}: {reply:?}");
    }
    assert!(k.open_obstructions(&vk(&key(4))).is_empty(), "nothing docketed");
}

/// 78.138 — A LINE CARRYING A LINE, NEVER FLATTENED: `P`'s fold carries
/// `A`'s inside it, netting it whole at 2, and the reserve carries `P`: one
/// receipt, `A` standing at `[0, 0]` and `P` at `[0]`, each recorded and each
/// a fold of its own line. An unbonded line, or one whose signature is not
/// its own, is refused at the top child's index.
#[test]
fn a_line_carries_a_line_inside_one_receipt() {
    let (a, p, q) = tiers();
    let mut k = founded();
    let child = a_fold();
    let inner = ChildNet {
        rows: child.rows.clone(),
        declared: child.declared.clone(),
        child: Child::Carried(Box::new(child.clone().sign(&a))),
        included_rows: child.rows.clone(),
        included_declared: child.declared.clone(),
        rate: [(1, 1), (2, 1), (1, 1), (1, 1)],
    };
    let parent = fold(&p, 2, vec![(who(1), g(12))], Vec::new(), vec![inner]);
    let Uplinked::Chained { at, open_remainders: 1, .. } = carry(&mut k, parent, &p) else { panic!("one receipt") };
    let (a_at, p_at) = (FoldAt { stamp: at, path: vec![0, 0] }, FoldAt { stamp: at, path: vec![0] });
    assert_eq!(k.ledgers.records.get(&a_at.key()).map(|r| r.tier), Some(vk(&a)), "A inside P inside the reserve's receipt");
    assert_eq!(k.ledgers.records.get(&p_at.key()).map(|r| r.tier), Some(vk(&p)));
    assert_eq!(k.line_head(&vk(&a)), (child.prefix, Some(a_at)), "A's line advanced, never touching the court");
    let empty_span = fold(&p, 2, Vec::new(), Vec::new(), Vec::new());
    assert_eq!(carry(&mut k, empty_span, &p), Uplinked::ChildRefused { index: 0, why: ChildRefusal::Repeat }, "a span over its own head");
    let unbonded = key(31);
    let forged = fold(&q, 9, Vec::new(), Vec::new(), Vec::new()).sign(&unbonded);
    let carried = ChildNet { rows: Vec::new(), declared: Vec::new(), child: Child::Carried(Box::new(forged)), included_rows: Vec::new(), included_declared: Vec::new(), rate: [(1, 1); 4] };
    let carrier = TierFold { tier: vk(&reserve()), prefix: [8; 32], rows: Vec::new(), declared: Vec::new(), children: vec![carried], handoffs: Vec::new() };
    assert_eq!(k.admit_uplink(&carrier.sign(&reserve())), Uplinked::ChildRefused { index: 0, why: ChildRefusal::Unsigned });
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

/// B′ — A COLD CHILD IS PROVED BY ITS SEALED COORDINATES: `A`'s fold carried,
/// the court's line folded past its receipt (off the hot line), and `P` still
/// nets it — from the coordinates the court held at its admission, which
/// leave once it is netted whole, surviving a restart meanwhile.
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
    assert!(k.chain.node(&at.stamp).is_none(), "the carrying receipt is off the hot line");
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert!(booted.ledgers.folds.contains(&at.key()), "held by its coordinates across the restart");
    let whole = part(&child, &at, child.rows.clone(), child.declared.clone(), 1, 1);
    let parent = fold(&p, 2, vec![(who(1), g(6))], Vec::new(), vec![whole]);
    let reply = carry(&mut booted, parent, &p);
    assert!(matches!(reply, Uplinked::Chained { open_remainders: 1, .. }), "only P itself open: {reply:?}");
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
