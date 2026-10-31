use super::super::*;
use super::common::*;

/// A COURT'S ACTS TRAVEL WHOLE (2026-09-29): every counter in its table.
#[test]
fn acts_round_trips() {
    let acts = Acts {
        k: 9,
        reserve: [1, 2, 3, 4],
        drawn: [5, 6, 7, 8],
        restored: [9, 10, 11, 12],
        refused: [13, 0, 1, 0],
        syllabus: [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26],
        fetch: [21, 22, 39, 49, 40, 41, 42, 43, 44, 45, 46, 47, 48, 65, 66, 67, 68, 69, 70],
        judged: [23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 59, 60, 61, 62, 63, 64],
        paid: [33, 34, 35, 36],
        colours: 37,
        colours_derived: 38,
        deferred: vec![crate::observe::Deferred { head: [1, 2, 3, 4, 5, 6, 7, 8], kind: 2, mag: [39, 40, 41, 42], short: [0, 43, 0, 0], tries: 44, learner: Some([9; 8]) }],
        provisional: [50, 51, 52, 53, 54, 55, 56, 57, 58, 59],
        pays: [71, 72, 73, 74, 75, 76, 77, 78, 79],
        pays_lanes: crate::observe::PayLanes(std::array::from_fn(|i| 80 + i as u64)),
    };
    reads_back(&[Reading::Acts(Box::new(acts))]);
}

/// MU-CLIENT §5.6 — EVERY PAY A PRODUCER NAMES IS A KIND OF ITS TABLE: each
/// counted once with its lanes on its own slot.
#[test]
fn acts_tally_pay_counts_each_kind_on_slot() {
    let mut t = crate::observe::ActsTally::default();
    for (i, kind) in [
        "scenes on no family held",
        "word sections",
        "conjugate transports",
        "sentence gluings",
        "demands closed by another mind",
        "attestations joined to a claim",
        "mints counted toward a grant",
        "verifications",
        "lease payments",
    ]
    .iter()
    .enumerate()
    {
        t.pay(kind, [1, 2, 3, i as u64]);
        assert_eq!(t.pays[i], 1, "{kind} is a slot of its table");
        assert_eq!(&t.pays_lanes[i * 4..i * 4 + 4], &[1, 2, 3, i as u64], "{kind} carries its lanes");
    }
    t.pay("no such kind", [9; 4]);
    assert_eq!(t.pays.iter().sum::<u64>(), 9, "a kind the table does not hold counts nothing");
}

/// EVERY NAME A PRODUCER COUNTS UNDER IS ITS TABLE'S: `count` is silent on a
/// name the table does not hold, so each name the producers use is pinned
/// here, and a renamed slot fails this test rather than counting nothing.
#[test]
fn count_names_every_producer_in_table() {
    fn holds<const N: usize>(table: &[&str; N], names: &[&str]) {
        for n in names {
            let mut slots = [0u64; N];
            count(&mut slots, table, n);
            assert_eq!(slots.iter().sum::<u64>(), 1, "{n} is a slot of its table");
        }
    }
    holds(
        &SYLLABUS,
        &["pages", "words served", "words missed", "ended: order", "ended: frame", "ended: count", "deferred: reserve"],
    );
    holds(
        &FETCH,
        &[
            "served to courts",
            "served to voices",
            "deferred: reserve",
            "deferred: learner short",
            "words: refracted (held, Γ)",
            "words: conjugate, carriage apart",
            "words: unspellable conjugator",
            "words: rotor per vertex δγ",
            "words: chart only",
            "words: unanchored",
            "bytes served",
            "bytes refracted, whole",
            "bytes refracted, as (held, Γ)",
        ],
    );
    holds(
        &PROVISIONAL,
        &[
            "letters docketed",
            "letters heard again (converged as a party)",
            "offers sent to witnesses",
            "offers with no eligible witness",
            "offers a witness left unanswered",
            "witness reached: installed",
            "witness reached: inexact",
            "witness reached: awaiting a vantage",
            "witness refused",
            "witness unmeasured",
        ],
    );
    holds(
        &JUDGED,
        &[
            "first closures",
            "closures again",
            "refused P",
            "refused D",
            "refused C",
            "refused I",
            "refused N",
            "refused ?",
            "not a rule",
            "unverifiable",
            "no work",
            "unread",
            "unsigned",
            "signed by an inner seat (served)",
            "no such task",
            "no rule carried",
        ],
    );
}

/// 78.153c — EVERY VERDICT THE COURT OR ITS DOOR COUNTS HAS ITS OWN SLOT:
/// `Judged::slot` is a bijection onto `JUDGED`, in its order, and a verdict
/// counted lands exactly there — so no name is dropped as "no work" was.
#[test]
fn judged_slot_is_bijection_onto_judged_table() {
    let mut seen = [false; JUDGED.len()];
    for (i, j) in Judged::ALL.iter().enumerate() {
        assert_eq!(j.slot(), i, "{j:?} sits in the table's order");
        assert_eq!(j.name(), JUDGED[i], "{j:?} reads by the table's name");
        assert!(!seen[i], "{j:?} shares no slot");
        seen[i] = true;
        let mut tally = ActsTally::default();
        tally.judge(*j);
        let mut want = [0u64; JUDGED.len()];
        want[i] = 1;
        assert_eq!(tally.judged, want, "{j:?} counted on its slot alone");
    }
    assert!(seen.iter().all(|&s| s), "every slot has its verdict");
    assert_eq!(Judged::NoWork.name(), "no work");
    assert_eq!(Judged::Unread.name(), "unread");
}
