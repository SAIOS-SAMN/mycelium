// 78.125 — 2(c) THE CHALLENGE WINDOW: a fold's prefix reconstructed — upheld
// or falsified — an availability demand and its default, an arbitrage booked
// by its tier, and finality read recursively over every fold a fold nets.
// 78.138 — the lines `A`, `P`, `Q` never touch the court: each fold of those lines
// is CARRIED INLINE in a fold of the domain's reserve `R`, and contested
// where it stands inside it.

use crate::anastome::dialogue::challenge::{line_head, Challenge, ChallengeRefusal, Challenged, Entry, Finality};
use crate::anastome::dialogue::uplink::{Child, ChildNet, FoldAt, Net, TierFold, Uplinked};
use super::fixtures::close_through;
use crate::chronometry::{Cite, Stamp};
use crate::server::cognition::Court;
use crate::transducer::cech::coefficient::Coefficient;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn verifying(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

fn who(b: u8) -> [u8; 32] {
    [b; 32]
}

fn g(v: i128) -> [i128; 4] {
    [0, v, 0, 0]
}

/// The lines `A`, `P`, `Q`, and a challenger who is none of them.
fn tiers() -> (SigningKey, SigningKey, SigningKey) {
    (key(21), key(22), key(23))
}

/// The domain's reserve — the only tier that uplinks.
fn reserve() -> SigningKey {
    key(24)
}

fn challenger() -> SigningKey {
    key(40)
}

fn domain() -> [u8; 32] {
    verifying(&key(4))
}

/// The domain founded with its three lines and its reserve, the reserve
/// designated, and each line BONDED by it — a line is carried only with a
/// bond of its own.
fn founded() -> Court {
    use crate::anastome::dialogue::domain::{Bond, DomainGenesis, Founded, Reservation};
    let (a, p, q) = tiers();
    let r = reserve();
    let mut ts = vec![verifying(&a), verifying(&p), verifying(&q), verifying(&r)];
    ts.sort_unstable();
    let mut k = Court::new();
    let genesis = DomainGenesis { domain: domain(), tiers: ts };
    assert!(matches!(k.hear_domain(&genesis.sign(&key(4))), Founded::Sealed { .. }));
    let reservation = Reservation { domain: domain(), tier: verifying(&r), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&reservation.sign(&key(4))), Founded::Reserved { .. }));
    super::boundary::signs(&mut k, &r);
    let anchor = k.anchor();
    k.treasury.genesis(verifying(&r), [12, 0, 0, 0], anchor);
    for (i, line) in [(0u8, &a), (1, &p), (2, &q)] {
        let bond = Bond { domain: domain(), reserve: verifying(&r), tier: verifying(line), mu: Some([4, 0, 0, 0].into()), nonce: [i; 32] };
        assert!(matches!(k.bond(&bond.sign(&r)), Founded::Bonded { .. }));
    }
    k
}

/// A fold whose prefix is the head its tier line's `entries` reach from the
/// zero head (the tier's first fold).
fn fold(tier: &SigningKey, entries: &[Entry], rows: Vec<Net>, declared: Vec<Net>, children: Vec<ChildNet>) -> TierFold {
    TierFold { tier: verifying(tier), prefix: line_head([0; 32], entries), rows, declared, children, handoffs: Vec::new(), transitions: Vec::new() }
}

/// `A`'s line: one entry — `w1` +10, `w2` −4 on G, the 6 made declared by `w9`.
fn a_entries() -> Vec<Entry> {
    vec![Entry { rows: vec![(who(1), g(10)), (who(2), g(-4))], declared: vec![(who(9), g(6))] }]
}

/// `A`'s fold, honest to its line.
fn a_fold() -> TierFold {
    let e = a_entries();
    fold(&tiers().0, &e, e[0].rows.clone(), e[0].declared.clone(), Vec::new())
}

/// A LINE'S FOLD ON THE COURT: carried inline in a fresh fold of the reserve
/// that includes none of it, so it stands whole to be netted — where it
/// stands, `[0]` inside the reserve's receipt.
fn sealed(k: &mut Court, f: TierFold, by: &SigningKey) -> FoldAt {
    let carried = ChildNet {
        rows: f.rows.clone(),
        declared: f.declared.clone(),
        child: Child::Carried(Box::new(f.sign(by))),
        included_rows: Vec::new(),
        included_declared: Vec::new(),
        rate: [(1, 1); 4],
    };
    let mut prefix = [5u8; 32];
    prefix[..8].copy_from_slice(&k.chain.head_height().to_be_bytes());
    let carrier = TierFold { tier: verifying(&reserve()), prefix, rows: Vec::new(), declared: Vec::new(), children: vec![carried], handoffs: Vec::new(), transitions: Vec::new() };
    match k.hear_uplink(&carrier.sign(&reserve())) {
        Uplinked::Chained { at, .. } => FoldAt { stamp: at, path: vec![0] },
        other => panic!("carried: {other:?}"),
    }
}

fn whole(of: &TierFold, at: &FoldAt, n: i128) -> ChildNet {
    ChildNet {
        child: Child::Sealed(at.clone()),
        rows: of.rows.clone(),
        declared: of.declared.clone(),
        included_rows: of.rows.clone(),
        included_declared: of.declared.clone(),
        rate: [(1, 1), (n, 1), (1, 1), (1, 1)],
    }
}

/// The evidence for `f` at `at`: its own rows, declared, children and line.
fn evidence(f: &TierFold, at: &FoldAt, entries: Vec<Entry>) -> Challenge {
    Challenge::Evidence { fold: at.clone(), rows: f.rows.clone(), declared: f.declared.clone(), children: f.children.clone(), entries }
}

/// ONLY THE RESERVE UPLINKS (78.138): a line's fold put straight to the
/// court is refused by name; carried in its reserve's, it stands.
#[test]
fn admit_uplink_refuses_line_not_carried_by_reserve() {
    let (a, ..) = tiers();
    let mut k = founded();
    let off_core = crate::court::admission::Boundary::NotThroughCore { act: crate::court::admission::UniverseAct::Fold, domain: domain() };
    assert_eq!(k.hear_uplink(&a_fold().sign(&a)), Uplinked::Boundary(off_core));
    let at = sealed(&mut k, a_fold(), &a);
    assert_eq!(at.path, vec![0]);
    assert!(k.ledgers.records.contains(&at.key()), "the line recorded inside its reserve's receipt");
}

/// A RECONSTRUCTION UPHOLDS AN HONEST FOLD AND THE SEAL SAYS SO: the prefix
/// chains to the cited head and re-derives the rows; the receipt carries the
/// prefix's content hash; the fold reads provisional for its window, then
/// final and UPHELD — not merely unchallenged. A line that does not chain, or
/// rows not the fold's, are refused, nothing found.
#[test]
fn challenge_upholds_honest_fold_as_final_upheld() {
    let (a, ..) = tiers();
    let mut k = founded();
    let f = a_fold();
    let at = sealed(&mut k, f.clone(), &a);
    let wrong_line = vec![Entry { rows: vec![(who(1), g(1))], declared: Vec::new() }];
    assert_eq!(k.challenge(&evidence(&f, &at, wrong_line).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotThePrefix));
    let mut not_it = f.clone();
    not_it.rows[0].1 = g(11);
    assert_eq!(k.challenge(&evidence(&not_it, &at, a_entries()).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotTheFold));
    let Challenged::Upheld { prefix, .. } = k.challenge(&evidence(&f, &at, a_entries()).sign(&challenger())) else { panic!("upheld") };
    let head = k.chain.head().expect("sealed");
    assert_eq!(head.cites, vec![Cite::Challenge { fold: at.clone(), kind: 1, outcome: 1, evidence: prefix }], "the upheld prefix on the receipt");
    assert_eq!(k.finality(&at), Finality::Provisional { opened: at.stamp.h, closed: None });
    close_through(&mut k, at.stamp.h).expect("the window closes, sealed");
    assert_eq!(k.finality(&at), Finality::Final { upheld: true });
    let mut booted = Court::new();
    assert!(booted.restore(&k.snapshot()), "the windows restore");
    assert_eq!(booted.finality(&at), Finality::Final { upheld: true }, "and read the same");
}

/// A RECONSTRUCTION FALSIFIES A FOLD ITS LINE DOES NOT SUM TO: the tier's
/// line made 6 and declared it, but its fold reports `w1` at 9 and `w2` at
/// −3 — the same continuity, other rows. Falsified, reversed, final never;
/// the reserve fold that carried it unwinds (it refiles without the line),
/// and the LINE's bond is slashed, never the reserve's — on the lane the
/// lie moved, against its exposure (D2.1): bonded 6 on G, exposed 15 (9 out
/// to `w1`, 6 declared), the cut is 6·15/21 = 30/7, so 12/7 stays bonded;
/// on S, where the fold moved nothing, nothing is cut.
#[test]
fn challenge_falsifies_fold_line_does_not_sum_to() {
    use crate::anastome::dialogue::domain::{Bond, Founded};
    let (a, ..) = tiers();
    let mut k = founded();
    let anchor = k.anchor();
    k.treasury.genesis(verifying(&reserve()), [0, 6, 0, 0], anchor);
    let on_g = Bond { domain: domain(), reserve: verifying(&reserve()), tier: verifying(&a), mu: Some([0, 6, 0, 0].into()), nonce: [9; 32] };
    assert!(matches!(k.bond(&on_g.sign(&reserve())), Founded::Bonded { .. }), "the line bonded on G too");
    let e = a_entries();
    let lie = fold(&a, &e, vec![(who(1), g(9)), (who(2), g(-3))], e[0].declared.clone(), Vec::new());
    assert_eq!(lie.continuity(), Ok(()), "continuity alone does not catch it");
    let at = sealed(&mut k, lie.clone(), &a);
    let reserve_bond = k.bond_of(&verifying(&reserve()));
    let reply = k.challenge(&evidence(&lie, &at, e).sign(&challenger()));
    assert!(matches!(reply, Challenged::Falsified { unwound: 1, .. }), "the carrying reserve fold unwound: {reply:?}");
    assert_eq!(k.finality(&at), Finality::Falsified);
    assert_eq!(k.finality(&FoldAt::sealed(at.stamp)), Finality::Unwound, "the reserve refiles without the line");
    let kept = crate::economy::Mu::counts([28, 12, 0, 0]).part(7).expect("sevenths");
    assert_eq!(k.bond_of(&verifying(&a)), kept, "the line's own bond slashed on G against its exposure, S untouched");
    let bounty = crate::economy::Mu::counts([0, 60, 0, 0]).part(119).expect("a part");
    assert_eq!(k.treasury.balance_mu(&verifying(&challenger())), bounty, "the bounty: the cut times the defect 2 against the exposure 15");
    assert_eq!(k.bond_of(&verifying(&reserve())), reserve_bond, "never the reserve's");
    let head = k.chain.head_height();
    assert!(close_through(&mut k, head).is_none(), "a falsified fold holds no window to close");
    assert_eq!(k.finality(&at), Finality::Falsified, "never final");
}

/// THE DEFECT IS ZERO EXACTLY WHERE THE FOLD RECONSTRUCTS (D2.1): the honest
/// fold against its line measures nothing on every lane; the lie's rows
/// (`w1` 9, `w2` −3 against the line's 10 and −4) measure 1 + 1 on G; a
/// declared net the line did not carry measures its whole value.
#[test]
fn defect_is_zero_iff_reconstructs() {
    use crate::server::cognition::challenge::defect;
    use crate::transducer::cech::coefficient::Lanes;
    let e = a_entries();
    let own_rows = Lanes::of(&e[0].rows).expect("sums");
    let own_declared = Lanes::of(&e[0].declared).expect("sums");
    let honest = defect(&e[0].rows, &e[0].declared, &[], &own_rows, &own_declared).expect("measured");
    assert!(honest.is_zero(), "the honest fold reconstructs");
    let lie = defect(&[(who(1), g(9)), (who(2), g(-3))], &e[0].declared, &[], &own_rows, &own_declared).expect("measured");
    assert_eq!(lie, [0, 2, 0, 0], "the lie on G, by its own residue");
    let more = vec![(who(9), g(6)), (who(8), g(3))];
    let undeclared = defect(&e[0].rows, &more, &[], &own_rows, &own_declared).expect("measured");
    assert_eq!(undeclared, [0, 3, 0, 0], "a declared net the line never carried");
}

/// AVAILABILITY: a demand gives the tier the window the demand opens, open
/// until every other keeper has carried it and returned; a second demand and
/// an early default are refused; unanswered, the default falsifies. Answered
/// in time, the default is refused — it was published.
#[test]
fn challenge_default_falsifies_unpublished_demand() {
    let (a, ..) = tiers();
    let mut k = founded();
    let f = a_fold();
    let at = sealed(&mut k, f.clone(), &a);
    let demand = || Challenge::Demand { fold: at.clone() };
    let Challenged::Demanded { at: asked, deadline, .. } = k.challenge(&demand().sign(&challenger())) else { panic!("demanded") };
    assert_eq!(deadline, None, "open until the keepers carry the demand and return");
    // A resend by the same filer is answered with the original; another
    // filer's demand is refused — one demand per fold.
    let Challenged::Demanded { at: again, .. } = k.challenge(&demand().sign(&challenger())) else { panic!("resent") };
    assert_eq!(again, asked, "the original demand, nothing sealed twice");
    assert_eq!(k.challenge(&demand().sign(&key(41))), Challenged::Refused(ChallengeRefusal::Demanded));
    assert_eq!(k.challenge(&Challenge::Default { fold: at.clone() }.sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotDue));
    close_through(&mut k, asked.h).expect("the demand's window closes");
    assert_eq!(k.finality(&at), Finality::Blocked, "the default is due and unclaimed: never final");
    assert!(matches!(k.challenge(&Challenge::Default { fold: at.clone() }.sign(&challenger())), Challenged::Falsified { .. }));
    assert_eq!(k.finality(&at), Finality::Falsified);

    // Answered in time: the tier publishes, and no default stands.
    let mut k = founded();
    let at = sealed(&mut k, f.clone(), &a);
    let Challenged::Demanded { at: asked, .. } = k.challenge(&Challenge::Demand { fold: at.clone() }.sign(&challenger())) else { panic!() };
    assert!(matches!(k.challenge(&evidence(&f, &at, a_entries()).sign(&a)), Challenged::Upheld { .. }), "the tier publishes");
    close_through(&mut k, asked.h).expect("the windows close");
    assert_eq!(k.challenge(&Challenge::Default { fold: at.clone() }.sign(&challenger())), Challenged::Refused(ChallengeRefusal::Published));
    assert_eq!(k.finality(&at), Finality::Final { upheld: true });
}

/// `P` nets all of `A` at 2 on G, and its own line holds the rest: its rows
/// less `2 · A` per identity — `w1` 12 − 20, `w2` 0 + 8.
fn p_over(at: &FoldAt) -> (TierFold, Vec<Entry>) {
    let p_line = vec![Entry { rows: vec![(who(1), g(-8)), (who(2), g(8))], declared: Vec::new() }];
    let f = fold(&tiers().1, &p_line, vec![(who(1), g(12))], Vec::new(), vec![whole(&a_fold(), at, 2)]);
    (f, p_line)
}

/// FINALITY IS RECURSIVE, AND A REVERSED CHILD UNWINDS ITS ANCESTORS: `P`
/// reconstructed upheld on its OWN line (never re-listing `A`'s transfers)
/// yet reads blocked while `A` is demanded and unanswered; `A` defaulted, `P`
/// is unwound — it must refile — and so are the reserve folds that carried
/// `A` and `P`.
#[test]
fn finality_waits_on_netted_folds_and_unwinds_ancestors() {
    let (a, p, _) = tiers();
    let mut k = founded();
    let a_at = sealed(&mut k, a_fold(), &a);
    let (pf, p_line) = p_over(&a_at);
    let p_at = sealed(&mut k, pf.clone(), &p);
    assert!(matches!(k.challenge(&evidence(&pf, &p_at, p_line).sign(&challenger())), Challenged::Upheld { .. }), "own entries only");
    let Challenged::Demanded { at: asked, .. } = k.challenge(&Challenge::Demand { fold: a_at.clone() }.sign(&challenger())) else { panic!() };
    close_through(&mut k, p_at.stamp.h).expect("P's window closes");
    assert_eq!(k.finality(&p_at), Finality::Blocked, "its own window passed, but a fold it nets is not final");
    close_through(&mut k, asked.h).expect("the demand's window closes");
    let reply = k.challenge(&Challenge::Default { fold: a_at.clone() }.sign(&challenger()));
    assert!(matches!(reply, Challenged::Falsified { unwound: 3, .. }), "P and the two carrying reserve folds: {reply:?}");
    assert_eq!((k.finality(&a_at), k.finality(&p_at)), (Finality::Falsified, Finality::Unwound));
}

/// A FALSIFIED PARENT RETURNS WHAT IT NETTED: `P` lies on its own line; the
/// reconstruction falsifies it, `A`'s parts come back, and `Q` refiles all
/// of `A`.
#[test]
fn challenge_returns_child_netting_of_falsified_parent() {
    let (a, p, q) = tiers();
    let mut k = founded();
    let a_at = sealed(&mut k, a_fold(), &a);
    // P's line holds `w1` −7, `w2` +7; netting A at 2 its rows must then be
    // `w1` 13, `w2` −1 — it reports `w1` 12 and nothing for `w2`.
    let lying_line = vec![Entry { rows: vec![(who(1), g(-7)), (who(2), g(7))], declared: Vec::new() }];
    let lie = fold(&p, &lying_line, vec![(who(1), g(12))], Vec::new(), vec![whole(&a_fold(), &a_at, 2)]);
    assert_eq!(lie.continuity(), Ok(()), "continuity alone does not catch it");
    let lie_at = sealed(&mut k, lie.clone(), &p);
    let refile = |k: &mut Court, nonce: u8| {
        let qf = TierFold { prefix: [nonce; 32], ..fold(&q, &[], vec![(who(1), g(10)), (who(2), g(-4))], Vec::new(), vec![whole(&a_fold(), &a_at, 1)]) };
        let carried = ChildNet {
            rows: qf.rows.clone(),
            declared: qf.declared.clone(),
            child: Child::Carried(Box::new(qf.sign(&q))),
            included_rows: Vec::new(),
            included_declared: Vec::new(),
            rate: [(1, 1); 4],
        };
        let mut prefix = [6u8; 32];
        prefix[0] = nonce;
        let carrier = TierFold { tier: verifying(&reserve()), prefix, rows: Vec::new(), declared: Vec::new(), children: vec![carried], handoffs: Vec::new(), transitions: Vec::new() };
        k.hear_uplink(&carrier.sign(&reserve()))
    };
    assert!(matches!(refile(&mut k, 1), Uplinked::ChildRefused { .. }), "consumed by P");
    let reply = k.challenge(&evidence(&lie, &lie_at, lying_line).sign(&challenger()));
    assert!(matches!(reply, Challenged::Falsified { unwound: 1, .. }), "P's carrying reserve fold: {reply:?}");
    assert!(matches!(refile(&mut k, 2), Uplinked::Chained { .. }), "A's parts returned: refiled whole");
    let refiled = k.ledgers.remaining.get(&a_at.key()).map(|(r, d)| r.is_identity() && d.is_identity());
    assert_eq!(refiled, Some(true), "all of A netted by Q");
}

/// AN ARBITRAGE IS BOOKED BY ITS TIER: the cycle `A → P` at 2, `P → Q` at 1,
/// `A → Q` at 1 obstructs at `Q`'s fold; another key cannot book it, a child
/// not on the cycle is no obstruction; `Q`'s tier names the holder and the
/// gain, the obstruction is discharged, and `Q` reads final after its window.
#[test]
fn challenge_books_correction_arbitrage_by_closing_tier() {
    let (_, _, q) = tiers();
    let book = |at: &FoldAt, child: Vec<u8>| Challenge::Correction { fold: at.clone(), child, holder: who(7), amount: 3 };
    // Left open past the window: blocked, and too late to book.
    let mut k = founded();
    let (q_at, child) = arbitrage(&mut k);
    assert_eq!(k.challenge(&book(&q_at, child.clone()).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotTheTier));
    assert_eq!(k.challenge(&book(&q_at, vec![9; 64]).sign(&q)), Challenged::Refused(ChallengeRefusal::NoObstruction));
    close_through(&mut k, q_at.stamp.h).expect("Q's window closes");
    assert_eq!(k.finality(&q_at), Finality::Blocked, "an open arbitrage at its fold");
    assert_eq!(k.challenge(&book(&q_at, child).sign(&q)), Challenged::Refused(ChallengeRefusal::Closed), "booked within the window");
    // Booked within the window: discharged, and final.
    let mut k = founded();
    let (q_at, child) = arbitrage(&mut k);
    assert!(matches!(k.challenge(&book(&q_at, child).sign(&q)), Challenged::Discharged { .. }));
    assert!(k.open_obstructions(&domain()).is_empty(), "discharged");
    close_through(&mut k, q_at.stamp.h).expect("Q's window closes");
    assert_eq!(k.finality(&q_at), Finality::Final { upheld: false }, "unchallenged, every fold it nets final");
}

/// The cycle `A → P` at 2, `P → Q` at 1, `A → Q` at 1, sealed on `k`: `Q`'s
/// fold, and the child vertex of the edge that closed it.
fn arbitrage(k: &mut Court) -> (FoldAt, Vec<u8>) {
    let (a, p, q) = tiers();
    let af = a_fold();
    let a_at = sealed(k, af.clone(), &a);
    let half = |n| ChildNet {
        child: Child::Sealed(a_at.clone()),
        rows: af.rows.clone(),
        declared: af.declared.clone(),
        included_rows: vec![(who(1), g(5)), (who(2), g(-2))],
        included_declared: vec![(who(9), g(3))],
        rate: [(1, 1), (n, 1), (1, 1), (1, 1)],
    };
    let p_fold =
        TierFold { tier: verifying(&p), prefix: [2; 32], rows: vec![(who(1), g(6))], declared: Vec::new(), children: vec![half(2)], handoffs: Vec::new(), transitions: Vec::new() };
    let p_at = sealed(k, p_fold.clone(), &p);
    let q_fold = TierFold {
        tier: verifying(&q),
        prefix: [3; 32],
        rows: vec![(who(1), g(9))],
        declared: Vec::new(),
        children: vec![whole(&p_fold, &p_at, 1), half(1)],
        handoffs: Vec::new(),
        transitions: Vec::new(),
    };
    let q_at = sealed(k, q_fold, &q);
    let open = k.open_obstructions(&domain());
    let [o] = open.as_slice() else { panic!("one open arbitrage") };
    assert_eq!(o.fold, q_at, "named by the fold that closed it");
    (q_at, o.child.clone())
}

/// A PUBLICATION STREAMS IN PARTS: opened by restating the fold, its line
/// carried part by part — nothing sealed until the head reaches the cited
/// prefix, then judged once. A part never opened, or opened by another
/// filer, is refused; a lying fold streamed is falsified as surely.
#[test]
fn challenge_part_judges_stream_at_cited_prefix() {
    let (a, ..) = tiers();
    let line: Vec<Entry> = vec![
        Entry { rows: vec![(who(1), g(4))], declared: vec![(who(9), g(4))] },
        Entry { rows: vec![(who(1), g(6)), (who(2), g(-4))], declared: Vec::new() },
        Entry { rows: Vec::new(), declared: vec![(who(9), g(2))] },
    ];
    let f = fold(&a, &line, vec![(who(1), g(10)), (who(2), g(-4))], vec![(who(9), g(6))], Vec::new());
    let open = |f: &TierFold, at: &FoldAt| Challenge::Open { fold: at.clone(), rows: f.rows.clone(), declared: f.declared.clone(), children: Vec::new() };
    let part = |at: &FoldAt, entries: &[Entry]| Challenge::Part { fold: at.clone(), entries: entries.to_vec() };
    let mut k = founded();
    let at = sealed(&mut k, f.clone(), &a);
    assert_eq!(k.challenge(&part(&at, &line[..1]).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotOpened));
    assert_eq!(k.challenge(&open(&f, &at).sign(&challenger())), Challenged::Streaming { head: [0; 32] });
    let sealed_before = k.chain.head_height();
    let Challenged::Streaming { head } = k.challenge(&part(&at, &line[..1]).sign(&challenger())) else { panic!("streaming") };
    assert_eq!(head, line_head([0; 32], &line[..1]));
    assert_eq!(k.chain.head_height(), sealed_before, "nothing sealed mid-stream");
    assert_eq!(k.challenge(&part(&at, &line[1..]).sign(&a)), Challenged::Refused(ChallengeRefusal::NotOpened), "another filer's stream");
    let reply = k.challenge(&part(&at, &line[1..]).sign(&challenger()));
    assert!(matches!(reply, Challenged::Upheld { prefix, .. } if prefix == f.prefix), "{reply:?}");
    // The same line under a fold that reports other rows: falsified.
    let mut k = founded();
    let lie = fold(&a, &line, vec![(who(1), g(9)), (who(2), g(-3))], vec![(who(9), g(6))], Vec::new());
    let at = sealed(&mut k, lie.clone(), &a);
    assert!(matches!(k.challenge(&open(&lie, &at).sign(&challenger())), Challenged::Streaming { .. }));
    assert!(matches!(k.challenge(&part(&at, &line).sign(&challenger())), Challenged::Falsified { .. }));
}

/// A SPAN PAST ONE FRAME IS PUBLISHED IN PARTS: 4096 entries of three
/// identities each — past the wire's one budget as a single act, refused by
/// name before a byte is sent — stream through the court in parts that each
/// fit it, and the reconstruction upholds the fold.
#[test]
fn challenge_part_streams_span_within_frame_budget() {
    use crate::anastome::dialogue::challenge::encode_challenge;
    use crate::network::wire::{Delimited, Lane, BODY_BUDGET};
    let (a, ..) = tiers();
    let line: Vec<Entry> =
        (0..4096).map(|_| Entry { rows: vec![(who(1), g(2)), (who(2), g(-1)), (who(3), g(-1))], declared: Vec::new() }).collect();
    let f = fold(&a, &line, vec![(who(1), g(8192)), (who(2), g(-4096)), (who(3), g(-4096))], Vec::new(), Vec::new());
    let mut k = founded();
    let at = sealed(&mut k, f.clone(), &a);
    let whole = evidence(&f, &at, line.clone()).sign(&challenger());
    let framed = |c: &crate::anastome::dialogue::challenge::SignedChallenge| Delimited { lane: Lane::Spatial, body: encode_challenge(c) }.encode();
    assert!(framed(&whole).is_none(), "one act past the budget is refused before it is sent");
    let open = Challenge::Open { fold: at.clone(), rows: f.rows.clone(), declared: Vec::new(), children: Vec::new() }.sign(&challenger());
    assert!(matches!(k.challenge(&open), Challenged::Streaming { .. }));
    let mut last = None;
    for chunk in line.chunks(1024) {
        let part = Challenge::Part { fold: at.clone(), entries: chunk.to_vec() }.sign(&challenger());
        let bytes = framed(&part).expect("each part fits the budget");
        assert!(bytes.len() <= BODY_BUDGET + 5);
        last = Some(k.challenge(&part));
    }
    assert!(matches!(last, Some(Challenged::Upheld { .. })), "{last:?}");
}

/// THE WINDOWS SETTLE AT AN ORBIT'S CLOSE, AND A PRUNED CHILD STILL READS
/// FINAL: `A` netted whole by `P`; `A`'s window closes first — final, its
/// record pruned to a stub its unfinished parent still names, read final
/// across a restart; then `P`'s window closes and `P` reads final through
/// it. A streamed reconstruction whose window closed is let go too.
#[test]
fn orbit_prunes_final_child_and_parent_reads_final() {
    let (a, p, _) = tiers();
    let mut k = founded();
    let a_at = sealed(&mut k, a_fold(), &a);
    let (pf, _) = p_over(&a_at);
    let p_at = sealed(&mut k, pf.clone(), &p);
    let open = Challenge::Open { fold: p_at.clone(), rows: pf.rows.clone(), declared: Vec::new(), children: pf.children.clone() };
    assert!(matches!(k.challenge(&open.sign(&challenger())), Challenged::Streaming { .. }));
    // A's window closes; P's stands.
    close_through(&mut k, a_at.stamp.h).expect("A's window closes");
    let _ = k.orbit();
    assert!(!k.ledgers.records.contains(&a_at.key()), "A's record pruned");
    assert!(k.ledgers.finals.contains(&a_at.key()), "to a stub its unfinished parent names");
    assert_eq!(k.finality(&a_at), Finality::Final { upheld: false });
    assert_eq!(k.reconstructions.len(), 1, "P's stream stands within its window");
    let mut booted = Court::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.finality(&a_at), Finality::Final { upheld: false }, "the stub read final across the restart");
    // P's window closes: final through its pruned child.
    close_through(&mut booted, p_at.stamp.h).expect("P's window closes");
    assert_eq!(booted.finality(&p_at), Finality::Final { upheld: false }, "the parent reads final through the stub");
    let _ = booted.orbit();
    assert!(booted.ledgers.records.is_empty(), "every record final and pruned");
    assert_eq!(booted.finality(&a_at), Finality::Final { upheld: false });
    assert_eq!(booted.finality(&p_at), Finality::Final { upheld: false });
    // The stream on the original court: its window closed, it is let go.
    close_through(&mut k, p_at.stamp.h).expect("P's window closes");
    let _ = k.orbit();
    assert!(k.reconstructions.is_empty(), "a stream whose window closed is let go");
}

/// A SELF-DEMAND WITHDRAWN REOPENS THE WINDOW, ONCE (as
/// ruled): the tier demanded its own fold and let the deadline pass — it
/// reads blocked; withdrawn by the tier, naming its demand's receipt, the
/// evidence window reopens at the withdrawal, the fold reads provisional
/// through it, and its evidence is upheld. Another key cannot withdraw it,
/// another stamp is not the demand, a second withdrawal is refused; an
/// external challenger's demand is never withdrawn, and a default claimed
/// closes the fold.
#[test]
fn challenge_withdraw_reopens_self_demand_once() {
    let (a, ..) = tiers();
    let mut k = founded();
    let f = a_fold();
    let at = sealed(&mut k, f.clone(), &a);
    let Challenged::Demanded { at: demand, .. } = k.challenge(&Challenge::Demand { fold: at.clone() }.sign(&a)) else { panic!("demanded") };
    close_through(&mut k, demand.h).expect("the demand's window closes");
    assert_eq!(k.finality(&at), Finality::Blocked, "the self-demand's deadline passed unanswered");
    let withdraw = |d: Stamp| Challenge::Withdraw { fold: at.clone(), demand: d };
    assert_eq!(k.challenge(&withdraw(demand).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotTheTier));
    assert_eq!(k.challenge(&withdraw(at.stamp).sign(&a)), Challenged::Refused(ChallengeRefusal::NotTheDemand));
    let Challenged::Reopened { closes, at: w, .. } = k.challenge(&withdraw(demand).sign(&a)) else { panic!("reopened") };
    assert_eq!(closes, None, "the reopened window stands open");
    assert_eq!(k.finality(&at), Finality::Provisional { opened: w.h, closed: None }, "provisional through the reopened window");
    assert_eq!(k.challenge(&withdraw(demand).sign(&a)), Challenged::Refused(ChallengeRefusal::Withdrawn), "a second chance is one");
    assert!(matches!(k.challenge(&evidence(&f, &at, a_entries()).sign(&a)), Challenged::Upheld { .. }), "the evidence it owed");
    // Once per fold.
    let mut k = founded();
    let at = sealed(&mut k, f.clone(), &a);
    let Challenged::Demanded { at: demand, .. } = k.challenge(&Challenge::Demand { fold: at.clone() }.sign(&a)) else { panic!() };
    assert!(matches!(k.challenge(&Challenge::Withdraw { fold: at.clone(), demand }.sign(&a)), Challenged::Reopened { .. }));
    let Challenged::Demanded { at: again, .. } = k.challenge(&Challenge::Demand { fold: at.clone() }.sign(&a)) else { panic!("a demand again") };
    assert_eq!(k.challenge(&Challenge::Withdraw { fold: at.clone(), demand: again }.sign(&a)), Challenged::Refused(ChallengeRefusal::Withdrawn));
    // An external challenger's demand is never withdrawn; a default closes it.
    let mut k = founded();
    let at = sealed(&mut k, f, &a);
    let Challenged::Demanded { at: external, .. } = k.challenge(&Challenge::Demand { fold: at.clone() }.sign(&challenger())) else { panic!() };
    assert_eq!(k.challenge(&Challenge::Withdraw { fold: at.clone(), demand: external }.sign(&a)), Challenged::Refused(ChallengeRefusal::NotTheDemand));
    close_through(&mut k, external.h).expect("the demand's window closes");
    assert!(matches!(k.challenge(&Challenge::Default { fold: at.clone() }.sign(&challenger())), Challenged::Falsified { .. }));
    assert_eq!(k.challenge(&Challenge::Withdraw { fold: at, demand: external }.sign(&a)), Challenged::Refused(ChallengeRefusal::Closed));
}

/// A TIER'S LINE HEAD IS READ BACK: the zero head before it folds; its last
/// recorded fold's prefix and where that fold stands after — inside its
/// reserve's receipt — the same after a restart, re-derived from the records.
#[test]
fn line_head_round_trips_with_setting_fold() {
    let (a, ..) = tiers();
    let mut k = founded();
    assert_eq!(k.line_head(&verifying(&a)), ([0; 32], None), "no fold: the zero head");
    let f = a_fold();
    let at = sealed(&mut k, f.clone(), &a);
    assert_eq!(k.line_head(&verifying(&a)), (f.prefix, Some(at.clone())));
    let mut booted = Court::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.line_head(&verifying(&a)), (f.prefix, Some(at)), "re-derived from the records");
}

/// A FOLD SEALED BEFORE THE LINE LAW IS GRANDFATHERED (as
/// ruled): no record, no line to reconstruct — final, unchallenged,
/// pre-line; a demand is refused, never defaulted; and a parent netting it
/// reads final once its own window passes. A place naming no fold is
/// unrecorded. (Only a fold the court sealed itself can be pre-line — a
/// reserve's: a carried line never was.)
#[test]
fn finality_grandfathers_pre_line_fold() {
    let (_, p, _) = tiers();
    let mut k = founded();
    let rf = TierFold { tier: verifying(&reserve()), ..a_fold() };
    let Uplinked::Admitted { at, .. } = k.hear_uplink(&rf.clone().sign(&reserve())) else { panic!("the reserve's own") };
    let r_at = FoldAt::sealed(at);
    // As sealed before 2(c): admitted, held by its coordinates, never recorded.
    k.ledgers.records.remove(&r_at.key());
    assert_eq!(k.finality(&r_at), Finality::PreLine);
    assert_eq!(k.challenge(&Challenge::Demand { fold: r_at.clone() }.sign(&challenger())), Challenged::Refused(ChallengeRefusal::NoRecord));
    let p_line = vec![Entry { rows: vec![(who(1), g(-8)), (who(2), g(8))], declared: Vec::new() }];
    let pf = fold(&p, &p_line, vec![(who(1), g(12))], Vec::new(), vec![whole(&rf, &r_at, 2)]);
    let p_at = sealed(&mut k, pf, &p);
    close_through(&mut k, p_at.stamp.h).expect("P's window closes");
    assert_eq!(k.finality(&p_at), Finality::Final { upheld: false }, "a grandfathered child is final");
    assert_eq!(k.finality(&FoldAt { stamp: Stamp { h: 7, ..at }, path: vec![0] }), Finality::Unrecorded);
}

/// Two other keepers of `court`: the keepers' keys, and each one's line citing the
/// court at `carried` (the first) and at `carried + gap` (the second).
fn keepers_citing(court: [u8; 32], carried: u64, gap: u64) -> Vec<([u8; 32], Vec<crate::chronometry::tuniversal::TickReading>)> {
    use super::fixtures::keeper_line;
    let (x, y) = (key(61), key(62));
    vec![(verifying(&x), keeper_line(&x, court, carried, 1)), (verifying(&y), keeper_line(&y, court, carried + gap, 5))]
}

use super::fixtures::reach_over as reach_of;

/// 78.125, LAW VERSION 5 — A WINDOW CLOSES AT REACH AND RETURN: every other
/// keeper cites the court at or above the opening and reads again; the reach
/// is the least height they carried it to (here the first keeper's, the
/// second carrying it further), and one sealed `Cite::Window` closes every
/// window opened at or below it at that receipt's height. A window opened
/// above the reach stays open, naming the reach.
#[test]
fn window_closes_at_reach_and_return() {
    let (a, p, _) = tiers();
    let mut k = founded();
    let at = sealed(&mut k, a_fold(), &a);
    let lines = keepers_citing(k.identity, at.stamp.h, 3);
    let reach = reach_of(k.identity, &lines);
    assert_eq!(reach.as_ref().map(|r| r.through), Ok(at.stamp.h), "the least height every other keeper carried");
    let (pf, _) = p_over(&at);
    let p_at = sealed(&mut k, pf, &p);
    let r = k.close_windows(&reach).expect("sealed");
    assert!(matches!(r.cites.as_slice(), [Cite::Window { through, witnesses }] if *through == at.stamp.h && witnesses.len() == 4));
    assert_eq!(k.window_close(at.stamp.h), Ok(r.stamp.h));
    assert_eq!(k.finality(&at), Finality::Final { upheld: false });
    assert_eq!(k.window_close(p_at.stamp.h), Err(crate::chronometry::tuniversal::WindowRefusal::Unreached { reach: at.stamp.h }));
    assert!(matches!(k.finality(&p_at), Finality::Provisional { closed: None, .. }));
}

/// A WINDOW STANDS OPEN WHILE ONE KEEPER HAS NOT CARRIED IT: the second
/// keeper read twice but never cited the court; the reach is refused naming
/// it, nothing is sealed, and the fold reads provisional with no close.
#[test]
fn window_stays_open_while_one_keeper_uncited() {
    use crate::chronometry::tuniversal::WindowRefusal;
    let (a, ..) = tiers();
    let mut k = founded();
    let at = sealed(&mut k, a_fold(), &a);
    let mut lines = keepers_citing(k.identity, at.stamp.h, 0);
    let silent = key(62);
    lines[1].1 = super::fixtures::keeper_line(&silent, [0x77; 32], at.stamp.h, 5);
    let reach = reach_of(k.identity, &lines);
    assert_eq!(reach, Err(WindowRefusal::Unwitnessed { keeper: verifying(&silent) }));
    assert!(k.close_windows(&reach).is_none(), "nothing sealed");
    assert_eq!(k.window_close(at.stamp.h), Err(WindowRefusal::Unwitnessed { keeper: verifying(&silent) }));
    assert_eq!(k.finality(&at), Finality::Provisional { opened: at.stamp.h, closed: None });
}

/// A LONE COURT CLOSES NOTHING (TU-3: one keeper alone finalises nothing):
/// with no keeper but the court, the reach is refused `NoOtherKeeper`, and a
/// court never handed a reach reads the same.
#[test]
fn window_never_closes_without_another_keeper() {
    use crate::chronometry::tuniversal::WindowRefusal;
    let (a, ..) = tiers();
    let mut k = founded();
    let at = sealed(&mut k, a_fold(), &a);
    assert_eq!(k.window_close(at.stamp.h), Err(WindowRefusal::NoOtherKeeper), "never handed a reach");
    let own = super::fixtures::keeper_line(&key(63), k.identity, at.stamp.h, 1);
    let reach = reach_of(k.identity, &[(k.identity, own)]);
    assert_eq!(reach, Err(WindowRefusal::NoOtherKeeper));
    assert!(k.close_windows(&reach).is_none());
    assert_eq!(k.window_close(at.stamp.h), Err(WindowRefusal::NoOtherKeeper));
    assert_eq!(k.finality(&at), Finality::Provisional { opened: at.stamp.h, closed: None });
}

/// A CLOSE RECONSTRUCTS FROM ITS SEALED CITATION: booted from the snapshot,
/// the court holds the `Cite::Window` receipt and reads every window's close
/// and every fold's finality exactly as before.
#[test]
fn window_close_reconstructs_from_sealed_citation() {
    let (a, ..) = tiers();
    let mut k = founded();
    let at = sealed(&mut k, a_fold(), &a);
    let r = close_through(&mut k, at.stamp.h).expect("sealed");
    let mut booted = Court::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.chain.node(&r.stamp).map(|n| n.cites.clone()), Some(r.cites.clone()), "the close on the line");
    assert_eq!(booted.window_close(at.stamp.h), k.window_close(at.stamp.h));
    assert_eq!(booted.finality(&at), Finality::Final { upheld: false });
}

/// A WINDOW OPENED UNDER LAW VERSION 4 CLOSES BY VERSION 4: a home read
/// without a window book holds version 5 from its head, and a fold sealed
/// below it closes at its seal plus 1024 heights, with no keeper's reach.
#[test]
fn window_opened_under_v4_closes_at_fixed_end() {
    use crate::physics::laws::CHALLENGE_WINDOW;
    let (a, ..) = tiers();
    let mut k = founded();
    let at = sealed(&mut k, a_fold(), &a);
    k.read_v4_windows();
    let end = at.stamp.h + CHALLENGE_WINDOW;
    assert_eq!(k.window_close(at.stamp.h), Ok(end));
    assert_eq!(k.finality(&at), Finality::Provisional { opened: at.stamp.h, closed: Some(end) });
    while k.chain.head_height() < end {
        k.append_step(crate::chronometry::ActKind::List, [1; 32], [2; 32]);
    }
    assert_eq!(k.finality(&at), Finality::Final { upheld: false });
}

/// A FORGED WITNESS IS REFUSED: a return reading whose signature does not
/// verify under its keeper closes nothing, refused naming the keeper.
#[test]
fn window_refuses_witness_with_forged_signature() {
    use crate::chronometry::tuniversal::WindowRefusal;
    let (a, ..) = tiers();
    let mut k = founded();
    let at = sealed(&mut k, a_fold(), &a);
    let mut reach = reach_of(k.identity, &keepers_citing(k.identity, at.stamp.h, 0)).expect("witnessed");
    reach.witnesses[0].back.sig[0] ^= 1;
    let keeper = reach.witnesses[0].back.keeper;
    assert!(k.close_windows(&Ok(reach)).is_none(), "nothing sealed");
    assert_eq!(k.window_close(at.stamp.h), Err(WindowRefusal::Unsigned { keeper }));
}

/// A RETURN NOT ABOVE THE REACH IS REFUSED: a witness whose return is its
/// reach reading itself proves no return, refused naming the keeper.
#[test]
fn window_refuses_return_not_above_reach() {
    use crate::chronometry::tuniversal::WindowRefusal;
    let (a, ..) = tiers();
    let mut k = founded();
    let at = sealed(&mut k, a_fold(), &a);
    let mut reach = reach_of(k.identity, &keepers_citing(k.identity, at.stamp.h, 0)).expect("witnessed");
    reach.witnesses[0].back = reach.witnesses[0].reach.clone();
    let keeper = reach.witnesses[0].reach.keeper;
    assert!(k.close_windows(&Ok(reach)).is_none(), "nothing sealed");
    assert_eq!(k.window_close(at.stamp.h), Err(WindowRefusal::Mismatched { keeper }));
}
