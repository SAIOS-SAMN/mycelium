// 78.125 — 2(c) THE CHALLENGE WINDOW: a fold's prefix replayed — upheld or
// falsified — an availability demand and its default, an arbitrage booked by
// its tier, and finality read recursively over every fold a fold nets.

use crate::anastome::dialogue::challenge::{line_head, Challenge, ChallengeRefusal, Challenged, Entry, Finality};
use crate::anastome::dialogue::uplink::{ChildNet, Net, TierFold, Uplinked};
use crate::chronometry::{ActKind, Cite, Stamp};
use crate::physics::laws::CHALLENGE_WINDOW;
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

fn g(v: i128) -> [i128; 4] {
    [0, v, 0, 0]
}

/// The tiers `A`, `P`, `Q`, and a challenger who is none of them.
fn tiers() -> (SigningKey, SigningKey, SigningKey) {
    (key(21), key(22), key(23))
}

fn challenger() -> SigningKey {
    key(40)
}

fn domain() -> [u8; 32] {
    vk(&key(4))
}

fn founded() -> Kernel {
    use crate::anastome::dialogue::domain::{DomainGenesis, Founded};
    let (a, p, q) = tiers();
    let mut ts = vec![vk(&a), vk(&p), vk(&q)];
    ts.sort_unstable();
    let mut k = Kernel::new();
    let genesis = DomainGenesis { domain: domain(), tiers: ts };
    assert!(matches!(k.admit_domain(&genesis.sign(&key(4))), Founded::Sealed { .. }));
    k
}

/// A fold whose prefix is the head its tier line's `entries` reach from the
/// zero head (the tier's first fold).
fn fold(tier: &SigningKey, entries: &[Entry], rows: Vec<Net>, declared: Vec<Net>, children: Vec<ChildNet>) -> TierFold {
    TierFold { tier: vk(tier), prefix: line_head([0; 32], entries), rows, declared, children, handoffs: Vec::new() }
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

fn sealed(k: &mut Kernel, f: TierFold, by: &SigningKey) -> Stamp {
    match k.admit_uplink(&f.sign(by)) {
        Uplinked::Admitted { at, .. } | Uplinked::Chained { at, .. } => at,
        other => panic!("admitted: {other:?}"),
    }
}

fn whole(of: &TierFold, at: Stamp, n: i128) -> ChildNet {
    ChildNet {
        at,
        rows: of.rows.clone(),
        declared: of.declared.clone(),
        included_rows: of.rows.clone(),
        included_declared: of.declared.clone(),
        rate: [(1, 1), (n, 1), (1, 1), (1, 1)],
    }
}

/// The evidence for `f` at `at`: its own rows, declared, children and line.
fn evidence(f: &TierFold, at: Stamp, entries: Vec<Entry>) -> Challenge {
    Challenge::Evidence { fold: at, rows: f.rows.clone(), declared: f.declared.clone(), children: f.children.clone(), entries }
}

/// The court's line carried past `h` by acts of its own.
fn pass(k: &mut Kernel, h: u64) {
    while k.chain.head_height() < h {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
    }
}

/// A REPLAY UPHOLDS AN HONEST FOLD AND THE SEAL SAYS SO: the prefix chains to
/// the cited head and re-derives the rows; the receipt carries the prefix's
/// content hash; the fold reads provisional for its window, then final and
/// UPHELD — not merely unchallenged. A line that does not chain, or rows not
/// the fold's, are refused, nothing found.
#[test]
fn a_replay_upholds_an_honest_fold_and_finality_says_upheld() {
    let (a, ..) = tiers();
    let mut k = founded();
    let f = a_fold();
    let at = sealed(&mut k, f.clone(), &a);
    let wrong_line = vec![Entry { rows: vec![(who(1), g(1))], declared: Vec::new() }];
    assert_eq!(k.challenge(&evidence(&f, at, wrong_line).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotThePrefix));
    let mut not_it = f.clone();
    not_it.rows[0].1 = g(11);
    assert_eq!(k.challenge(&evidence(&not_it, at, a_entries()).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotTheFold));
    let Challenged::Upheld { prefix, .. } = k.challenge(&evidence(&f, at, a_entries()).sign(&challenger())) else { panic!("upheld") };
    let head = k.chain.head().expect("sealed");
    assert_eq!(head.cites, vec![Cite::Challenge { fold: at, kind: 1, outcome: 1, evidence: prefix }], "the upheld prefix on the receipt");
    assert_eq!(k.finality(&at), Finality::Provisional { until: at.h + CHALLENGE_WINDOW });
    pass(&mut k, at.h + CHALLENGE_WINDOW);
    assert_eq!(k.finality(&at), Finality::Final { upheld: true });
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "the windows restore");
    assert_eq!(booted.finality(&at), Finality::Final { upheld: true }, "and read the same");
}

/// A REPLAY FALSIFIES A FOLD ITS LINE DOES NOT SUM TO: the tier's line made
/// 6 and declared it, but its fold reports `w1` at 9 and `w2` at −3 — the
/// same continuity, other rows. Falsified, reversed, final never.
#[test]
fn a_replay_falsifies_a_fold_its_own_line_does_not_sum_to() {
    let (a, ..) = tiers();
    let mut k = founded();
    let e = a_entries();
    let lie = fold(&a, &e, vec![(who(1), g(9)), (who(2), g(-3))], e[0].declared.clone(), Vec::new());
    assert_eq!(lie.continuity(), Ok(()), "continuity alone does not catch it");
    let at = sealed(&mut k, lie.clone(), &a);
    let reply = k.challenge(&evidence(&lie, at, e).sign(&challenger()));
    assert!(matches!(reply, Challenged::Falsified { unwound: 0, .. }), "{reply:?}");
    assert_eq!(k.finality(&at), Finality::Falsified);
    pass(&mut k, at.h + CHALLENGE_WINDOW);
    assert_eq!(k.finality(&at), Finality::Falsified, "never final");
}

/// AVAILABILITY: a demand gives the tier `W` heights from the demand; a second
/// demand and an early default are refused; unanswered, the default
/// falsifies. Answered in time, the default is refused — it was published.
#[test]
fn a_demanded_prefix_unpublished_by_its_deadline_falsifies_by_default() {
    let (a, ..) = tiers();
    let mut k = founded();
    let f = a_fold();
    let at = sealed(&mut k, f.clone(), &a);
    let Challenged::Demanded { at: asked, deadline, .. } = k.challenge(&Challenge::Demand { fold: at }.sign(&challenger())) else {
        panic!("demanded")
    };
    assert_eq!(deadline, asked.h + CHALLENGE_WINDOW);
    // A resend by the same filer is answered with the original; another
    // filer's demand is refused — one demand per fold.
    let Challenged::Demanded { at: again, .. } = k.challenge(&Challenge::Demand { fold: at }.sign(&challenger())) else { panic!("resent") };
    assert_eq!(again, asked, "the original demand, nothing sealed twice");
    assert_eq!(k.challenge(&Challenge::Demand { fold: at }.sign(&key(41))), Challenged::Refused(ChallengeRefusal::Demanded));
    assert_eq!(k.challenge(&Challenge::Default { fold: at }.sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotDue));
    pass(&mut k, deadline);
    assert_eq!(k.finality(&at), Finality::Blocked, "the default is due and unclaimed: never final");
    assert!(matches!(k.challenge(&Challenge::Default { fold: at }.sign(&challenger())), Challenged::Falsified { .. }));
    assert_eq!(k.finality(&at), Finality::Falsified);

    // Answered in time: the tier publishes, and no default stands.
    let mut k = founded();
    let at = sealed(&mut k, f.clone(), &a);
    let Challenged::Demanded { deadline, .. } = k.challenge(&Challenge::Demand { fold: at }.sign(&challenger())) else { panic!() };
    assert!(matches!(k.challenge(&evidence(&f, at, a_entries()).sign(&a)), Challenged::Upheld { .. }), "the tier publishes");
    pass(&mut k, deadline);
    assert_eq!(k.challenge(&Challenge::Default { fold: at }.sign(&challenger())), Challenged::Refused(ChallengeRefusal::Published));
    assert_eq!(k.finality(&at), Finality::Final { upheld: true });
}

/// `P` nets all of `A` at 2 on G, and its own line holds the rest: its rows
/// less `2 · A` per identity — `w1` 12 − 20, `w2` 0 + 8.
fn p_over(at: Stamp) -> (TierFold, Vec<Entry>) {
    let p_line = vec![Entry { rows: vec![(who(1), g(-8)), (who(2), g(8))], declared: Vec::new() }];
    let f = fold(&tiers().1, &p_line, vec![(who(1), g(12))], Vec::new(), vec![whole(&a_fold(), at, 2)]);
    (f, p_line)
}

/// FINALITY IS RECURSIVE, AND A REVERSED CHILD UNWINDS ITS ANCESTORS: `P`
/// replays upheld on its OWN line (never re-listing `A`'s transfers) yet
/// reads blocked while `A` is demanded and unanswered; `A` defaulted, `P`
/// is unwound — it must refile.
#[test]
fn finality_waits_on_every_fold_netted_and_a_reversed_child_unwinds_its_ancestors() {
    let (a, p, _) = tiers();
    let mut k = founded();
    let a_at = sealed(&mut k, a_fold(), &a);
    let (pf, p_line) = p_over(a_at);
    let p_at = sealed(&mut k, pf.clone(), &p);
    assert!(matches!(k.challenge(&evidence(&pf, p_at, p_line).sign(&challenger())), Challenged::Upheld { .. }), "own entries only");
    let Challenged::Demanded { deadline, .. } = k.challenge(&Challenge::Demand { fold: a_at }.sign(&challenger())) else { panic!() };
    pass(&mut k, p_at.h + CHALLENGE_WINDOW);
    assert_eq!(k.finality(&p_at), Finality::Blocked, "its own window passed, but a fold it nets is not final");
    pass(&mut k, deadline);
    let reply = k.challenge(&Challenge::Default { fold: a_at }.sign(&challenger()));
    assert!(matches!(reply, Challenged::Falsified { unwound: 1, .. }), "{reply:?}");
    assert_eq!((k.finality(&a_at), k.finality(&p_at)), (Finality::Falsified, Finality::Unwound));
}

/// A FALSIFIED PARENT RETURNS WHAT IT NETTED: `P` lies on its own line; the
/// replay falsifies it, `A`'s parts come back, and `Q` refiles all of `A`.
#[test]
fn a_falsified_parent_returns_its_netting_and_the_child_is_netted_again() {
    let (a, p, q) = tiers();
    let mut k = founded();
    let a_at = sealed(&mut k, a_fold(), &a);
    // P's line holds `w1` −7, `w2` +7; netting A at 2 its rows must then be
    // `w1` 13, `w2` −1 — it reports `w1` 12 and nothing for `w2`.
    let lying_line = vec![Entry { rows: vec![(who(1), g(-7)), (who(2), g(7))], declared: Vec::new() }];
    let lie = fold(&p, &lying_line, vec![(who(1), g(12))], Vec::new(), vec![whole(&a_fold(), a_at, 2)]);
    assert_eq!(lie.continuity(), Ok(()), "continuity alone does not catch it");
    let lie_at = sealed(&mut k, lie.clone(), &p);
    let refile = |k: &mut Kernel| {
        let qf = fold(&q, &[], vec![(who(1), g(10)), (who(2), g(-4))], Vec::new(), vec![whole(&a_fold(), a_at, 1)]);
        k.admit_uplink(&qf.sign(&q))
    };
    assert!(matches!(refile(&mut k), Uplinked::ChildRefused { .. }), "consumed by P");
    let reply = k.challenge(&evidence(&lie, lie_at, lying_line).sign(&challenger()));
    assert!(matches!(reply, Challenged::Falsified { unwound: 0, .. }), "{reply:?}");
    assert!(matches!(refile(&mut k), Uplinked::Chained { open_remainders: 0, .. }), "A's parts returned: refiled whole");
}

/// AN ARBITRAGE IS BOOKED BY ITS TIER: the cycle `A → P` at 2, `P → Q` at 1,
/// `A → Q` at 1 obstructs at `Q`'s fold; another key cannot book it, a child
/// not on the cycle is no obstruction; `Q`'s tier names the holder and the
/// gain, the obstruction is discharged, and `Q` reads final after its window.
#[test]
fn an_arbitrage_is_booked_by_the_closing_tier_and_its_fold_reads_final() {
    let (_, _, q) = tiers();
    let book = |at: Stamp, child: Vec<u8>| Challenge::Correction { fold: at, child, holder: who(7), amount: 3 };
    // Left open past the window: blocked, and too late to book.
    let mut k = founded();
    let (q_at, child) = arbitrage(&mut k);
    assert_eq!(k.challenge(&book(q_at, child.clone()).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotTheTier));
    assert_eq!(k.challenge(&book(q_at, vec![9; 64]).sign(&q)), Challenged::Refused(ChallengeRefusal::NoObstruction));
    pass(&mut k, q_at.h + CHALLENGE_WINDOW);
    assert_eq!(k.finality(&q_at), Finality::Blocked, "an open arbitrage at its fold");
    assert_eq!(k.challenge(&book(q_at, child).sign(&q)), Challenged::Refused(ChallengeRefusal::Closed), "booked within the window");
    // Booked within the window: discharged, and final.
    let mut k = founded();
    let (q_at, child) = arbitrage(&mut k);
    assert!(matches!(k.challenge(&book(q_at, child).sign(&q)), Challenged::Discharged { .. }));
    assert!(k.open_obstructions(&domain()).is_empty(), "discharged");
    pass(&mut k, q_at.h + CHALLENGE_WINDOW);
    assert_eq!(k.finality(&q_at), Finality::Final { upheld: false }, "unchallenged, every fold it nets final");
}

/// The cycle `A → P` at 2, `P → Q` at 1, `A → Q` at 1, sealed on `k`: `Q`'s
/// fold, and the child vertex of the edge that closed it.
fn arbitrage(k: &mut Kernel) -> (Stamp, Vec<u8>) {
    let (a, p, q) = tiers();
    let af = a_fold();
    let a_at = sealed(k, af.clone(), &a);
    let half = |n| ChildNet {
        at: a_at,
        rows: af.rows.clone(),
        declared: af.declared.clone(),
        included_rows: vec![(who(1), g(5)), (who(2), g(-2))],
        included_declared: vec![(who(9), g(3))],
        rate: [(1, 1), (n, 1), (1, 1), (1, 1)],
    };
    let p_fold =
        TierFold { tier: vk(&p), prefix: [2; 32], rows: vec![(who(1), g(6))], declared: Vec::new(), children: vec![half(2)], handoffs: Vec::new() };
    let p_at = sealed(k, p_fold.clone(), &p);
    let q_fold = TierFold {
        tier: vk(&q),
        prefix: [3; 32],
        rows: vec![(who(1), g(9))],
        declared: Vec::new(),
        children: vec![whole(&p_fold, p_at, 1), half(1)],
        handoffs: Vec::new(),
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
fn a_publication_streams_in_parts_and_is_judged_when_its_head_reaches_the_prefix() {
    let (a, ..) = tiers();
    let line: Vec<Entry> = vec![
        Entry { rows: vec![(who(1), g(4))], declared: vec![(who(9), g(4))] },
        Entry { rows: vec![(who(1), g(6)), (who(2), g(-4))], declared: Vec::new() },
        Entry { rows: Vec::new(), declared: vec![(who(9), g(2))] },
    ];
    let f = fold(&a, &line, vec![(who(1), g(10)), (who(2), g(-4))], vec![(who(9), g(6))], Vec::new());
    let open = |f: &TierFold, at: Stamp| Challenge::Open { fold: at, rows: f.rows.clone(), declared: f.declared.clone(), children: Vec::new() };
    let part = |at: Stamp, entries: &[Entry]| Challenge::Part { fold: at, entries: entries.to_vec() };
    let mut k = founded();
    let at = sealed(&mut k, f.clone(), &a);
    assert_eq!(k.challenge(&part(at, &line[..1]).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotOpened));
    assert_eq!(k.challenge(&open(&f, at).sign(&challenger())), Challenged::Streaming { head: [0; 32] });
    let sealed_before = k.chain.head_height();
    let Challenged::Streaming { head } = k.challenge(&part(at, &line[..1]).sign(&challenger())) else { panic!("streaming") };
    assert_eq!(head, line_head([0; 32], &line[..1]));
    assert_eq!(k.chain.head_height(), sealed_before, "nothing sealed mid-stream");
    assert_eq!(k.challenge(&part(at, &line[1..]).sign(&a)), Challenged::Refused(ChallengeRefusal::NotOpened), "another filer's stream");
    let reply = k.challenge(&part(at, &line[1..]).sign(&challenger()));
    assert!(matches!(reply, Challenged::Upheld { prefix, .. } if prefix == f.prefix), "{reply:?}");
    // The same line under a fold that reports other rows: falsified.
    let mut k = founded();
    let lie = fold(&a, &line, vec![(who(1), g(9)), (who(2), g(-3))], vec![(who(9), g(6))], Vec::new());
    let at = sealed(&mut k, lie.clone(), &a);
    assert!(matches!(k.challenge(&open(&lie, at).sign(&challenger())), Challenged::Streaming { .. }));
    assert!(matches!(k.challenge(&part(at, &line).sign(&challenger())), Challenged::Falsified { .. }));
}

/// A SPAN PAST ONE FRAME IS PUBLISHED IN PARTS: 4096 entries of three
/// identities each — past the wire's one budget as a single act, refused by
/// name before a byte is sent — stream through the court in parts that each
/// fit it, and the replay upholds the fold.
#[test]
fn a_span_past_one_frame_is_published_in_parts_each_within_the_budget() {
    use crate::anastome::dialogue::challenge::encode_challenge;
    use crate::network::wire::{Frame, Stratum, BODY_BUDGET};
    let (a, ..) = tiers();
    let line: Vec<Entry> =
        (0..4096).map(|_| Entry { rows: vec![(who(1), g(2)), (who(2), g(-1)), (who(3), g(-1))], declared: Vec::new() }).collect();
    let f = fold(&a, &line, vec![(who(1), g(8192)), (who(2), g(-4096)), (who(3), g(-4096))], Vec::new(), Vec::new());
    let mut k = founded();
    let at = sealed(&mut k, f.clone(), &a);
    let whole = evidence(&f, at, line.clone()).sign(&challenger());
    let framed = |c: &crate::anastome::dialogue::challenge::SignedChallenge| Frame { lane: Stratum::Spatial, body: encode_challenge(c) }.encode();
    assert!(framed(&whole).is_none(), "one act past the budget is refused before it is sent");
    let open = Challenge::Open { fold: at, rows: f.rows.clone(), declared: Vec::new(), children: Vec::new() }.sign(&challenger());
    assert!(matches!(k.challenge(&open), Challenged::Streaming { .. }));
    let mut last = None;
    for chunk in line.chunks(1024) {
        let part = Challenge::Part { fold: at, entries: chunk.to_vec() }.sign(&challenger());
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
/// it. `P` pruned in turn, `A` is named by nothing and nettable no more, and
/// is let go; `P`, its value still in flight, keeps its stub. A streamed
/// replay whose window closed is let go too.
#[test]
fn the_windows_settle_and_a_pruned_child_still_reads_final_for_its_parent() {
    let (a, p, _) = tiers();
    let mut k = founded();
    let a_at = sealed(&mut k, a_fold(), &a);
    pass(&mut k, a_at.h + CHALLENGE_WINDOW / 2);
    let (pf, _) = p_over(a_at);
    let p_at = sealed(&mut k, pf.clone(), &p);
    let open = Challenge::Open { fold: p_at, rows: pf.rows.clone(), declared: Vec::new(), children: pf.children.clone() };
    assert!(matches!(k.challenge(&open.sign(&challenger())), Challenged::Streaming { .. }));
    // A's window closes; P's stands.
    pass(&mut k, a_at.h + CHALLENGE_WINDOW);
    let _ = k.orbit();
    assert!(!k.ledgers.records.contains(&a_at.key()), "A's record pruned");
    assert!(k.ledgers.finals.contains(&a_at.key()), "to a stub its unfinished parent names");
    assert_eq!(k.finality(&a_at), Finality::Final { upheld: false });
    assert_eq!(k.replays.len(), 1, "P's stream stands within its window");
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.finality(&a_at), Finality::Final { upheld: false }, "the stub read final across the restart");
    // P's window closes: final through its pruned child.
    pass(&mut booted, p_at.h + CHALLENGE_WINDOW);
    assert_eq!(booted.finality(&p_at), Finality::Final { upheld: false }, "the parent reads final through the stub");
    let _ = booted.orbit();
    assert!(booted.ledgers.records.is_empty(), "every record final and pruned");
    // A, consumed and named by nothing, keeps its stub while the hot line
    // holds its receipt — else it would read pre-line — and reads final.
    assert!(booted.ledgers.finals.contains(&a_at.key()), "A's stub stays while the hot line holds its receipt");
    assert_eq!(booted.finality(&a_at), Finality::Final { upheld: false });
    assert!(booted.ledgers.finals.contains(&p_at.key()), "P, its value in flight, keeps its stub");
    assert_eq!(booted.finality(&p_at), Finality::Final { upheld: false });
    // The stream on the original court: its window closed, it is let go.
    pass(&mut k, p_at.h + CHALLENGE_WINDOW);
    let _ = k.orbit();
    assert!(k.replays.is_empty(), "a stream whose window closed is let go");
}

/// A SELF-DEMAND WITHDRAWN REOPENS THE WINDOW, ONCE (the coordinator's
/// ruling): the tier demanded its own fold and let the deadline pass — it
/// reads blocked; withdrawn by the tier, naming its demand's receipt, the
/// evidence window reopens `W` from the withdrawal, the fold reads
/// provisional, and its evidence is upheld. Another key cannot withdraw it,
/// another stamp is not the demand, a second withdrawal is refused; an
/// external challenger's demand is never withdrawn, and a default claimed
/// closes the fold.
#[test]
fn a_self_demand_withdrawn_reopens_the_window_once() {
    let (a, ..) = tiers();
    let mut k = founded();
    let f = a_fold();
    let at = sealed(&mut k, f.clone(), &a);
    let Challenged::Demanded { at: demand, deadline, .. } = k.challenge(&Challenge::Demand { fold: at }.sign(&a)) else { panic!("demanded") };
    pass(&mut k, deadline);
    assert_eq!(k.finality(&at), Finality::Blocked, "the self-demand's deadline passed unanswered");
    let withdraw = |d: Stamp| Challenge::Withdraw { fold: at, demand: d };
    assert_eq!(k.challenge(&withdraw(demand).sign(&challenger())), Challenged::Refused(ChallengeRefusal::NotTheTier));
    assert_eq!(k.challenge(&withdraw(at).sign(&a)), Challenged::Refused(ChallengeRefusal::NotTheDemand));
    let Challenged::Reopened { until, at: w, .. } = k.challenge(&withdraw(demand).sign(&a)) else { panic!("reopened") };
    assert_eq!(until, w.h + CHALLENGE_WINDOW);
    assert_eq!(k.finality(&at), Finality::Provisional { until }, "provisional through the reopened window");
    assert_eq!(k.challenge(&withdraw(demand).sign(&a)), Challenged::Refused(ChallengeRefusal::Withdrawn), "a second chance is one");
    assert!(matches!(k.challenge(&evidence(&f, at, a_entries()).sign(&a)), Challenged::Upheld { .. }), "the evidence it owed");
    // Once per fold.
    let mut k = founded();
    let at = sealed(&mut k, f.clone(), &a);
    let Challenged::Demanded { at: demand, .. } = k.challenge(&Challenge::Demand { fold: at }.sign(&a)) else { panic!() };
    assert!(matches!(k.challenge(&Challenge::Withdraw { fold: at, demand }.sign(&a)), Challenged::Reopened { .. }));
    let Challenged::Demanded { at: again, .. } = k.challenge(&Challenge::Demand { fold: at }.sign(&a)) else { panic!("a demand again") };
    assert_eq!(k.challenge(&Challenge::Withdraw { fold: at, demand: again }.sign(&a)), Challenged::Refused(ChallengeRefusal::Withdrawn));
    // An external challenger's demand is never withdrawn; a default closes it.
    let mut k = founded();
    let at = sealed(&mut k, f, &a);
    let Challenged::Demanded { at: theirs, deadline, .. } = k.challenge(&Challenge::Demand { fold: at }.sign(&challenger())) else { panic!() };
    assert_eq!(k.challenge(&Challenge::Withdraw { fold: at, demand: theirs }.sign(&a)), Challenged::Refused(ChallengeRefusal::NotTheDemand));
    pass(&mut k, deadline);
    assert!(matches!(k.challenge(&Challenge::Default { fold: at }.sign(&challenger())), Challenged::Falsified { .. }));
    assert_eq!(k.challenge(&Challenge::Withdraw { fold: at, demand: theirs }.sign(&a)), Challenged::Refused(ChallengeRefusal::Closed));
}

/// A TIER'S LINE HEAD IS READ BACK: the zero head before it folds; its last
/// recorded fold's prefix and that fold's stamp after — the same after a
/// restart, where the stamp is re-derived from the records.
#[test]
fn a_tiers_line_head_is_read_back_with_the_fold_that_set_it() {
    let (a, ..) = tiers();
    let mut k = founded();
    assert_eq!(k.line_head(&vk(&a)), ([0; 32], None), "no fold: the zero head");
    let f = a_fold();
    let at = sealed(&mut k, f.clone(), &a);
    assert_eq!(k.line_head(&vk(&a)), (f.prefix, Some(at)));
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.line_head(&vk(&a)), (f.prefix, Some(at)), "re-derived from the records");
}

/// A FOLD SEALED BEFORE THE LINE LAW IS GRANDFATHERED (the coordinator's
/// ruling): no record, no line to replay — final, unchallenged, pre-line; a
/// demand is refused, never defaulted; and a parent netting it reads final
/// once its own window passes. A stamp naming no fold is unrecorded.
#[test]
fn a_fold_sealed_before_the_line_law_is_grandfathered() {
    let (a, p, _) = tiers();
    let mut k = founded();
    let a_at = sealed(&mut k, a_fold(), &a);
    // As sealed before 2(c): admitted, held by its coordinates, never recorded.
    k.ledgers.records.remove(&a_at.key());
    assert_eq!(k.finality(&a_at), Finality::PreLine);
    assert_eq!(k.challenge(&Challenge::Demand { fold: a_at }.sign(&challenger())), Challenged::Refused(ChallengeRefusal::NoRecord));
    let (pf, _) = p_over(a_at);
    let p_at = sealed(&mut k, pf, &p);
    pass(&mut k, p_at.h + CHALLENGE_WINDOW);
    assert_eq!(k.finality(&p_at), Finality::Final { upheld: false }, "a grandfathered child is final");
    assert_eq!(k.finality(&Stamp { h: 7, ..a_at }), Finality::Unrecorded);
}
