// MU-CLIENT §4.2, §7.1 R1 — a long law's admission lands on the thinking
// mind's field, or its refusal is named and the admission handed back whole.

use crate::transducer::mind::{admit, scratch_here, with_lanes, AdmissionRefused, MindField};

/// A scratch field whose law moved `moved` on ledger 0, lane 0, and nothing
/// else: its admission carries exactly that lane.
fn admission_moving(moved: u64) -> crate::transducer::mind::Admission {
    let (mut scratch, from) = scratch_here();
    scratch.think(|| {
        let _moved = with_lanes(|l| l[0][0] += moved);
    });
    match scratch.into_admission(&from) {
        Some(a) => a,
        None => panic!("a scratch field yields its admission"),
    }
}

/// The admission is a reading or a refusal by name, never a dropped
/// boolean: outside a mind's tick it is `NoMindThinking`, handed back whole
/// and landing when the mind thinks; a lane that would pass its carrier is
/// `LanePastItsCarrier`, naming the ledger, lane, held count and addition,
/// and nothing of it lands.
#[test]
fn mind_admit_names_its_refusal() {
    let mut budget = crate::physics::Physics::default().stroke();

    // No mind thinking on this thread: refused by name, handed back.
    let back = match admit(admission_moving(5), &mut budget) {
        Err(AdmissionRefused::NoMindThinking(back)) => back,
        other => panic!("outside a tick the admission is refused by name: {other:?}"),
    };
    assert!(AdmissionRefused::NoMindThinking(Box::new(admission_moving(1))).to_string().contains("no mind is thinking"));

    // The admission handed back lands when the mind thinks.
    let mut mind = MindField::default();
    let landed = match mind.think(|| admit(*back, &mut budget)) {
        Ok(landed) => landed,
        Err(r) => panic!("the handed-back admission lands: {r}"),
    };
    assert!(landed.to_string().starts_with("admission landed"));
    assert_eq!((landed.flux.len(), landed.taught, landed.parked), (0, 0, 0), "no flux, word or cover was recorded");
    let held = mind.think(|| with_lanes(|l| l[0][0]).ok());
    assert_eq!(held, Some(5), "the lane the law moved landed on the mind's ledger");

    // A lane past its carrier: refused by name, and nothing lands.
    mind.think(|| {
        let _at_edge = with_lanes(|l| l[0][0] = u64::MAX - 1);
    });
    // The scratch is taken outside the tick: a scratch thinks in the
    // thread's slots, and never inside another mind's tick.
    let past = admission_moving(5);
    let refused = mind.think(|| admit(past, &mut budget));
    match &refused {
        Err(AdmissionRefused::LanePastItsCarrier { ledger, lane, held, add }) => {
            assert_eq!((*ledger, *lane, *held, *add), (0, 0, u64::MAX - 1, 5), "the refusal names where it passed");
        }
        other => panic!("a lane past its carrier is refused by name: {other:?}"),
    }
    let held = mind.think(|| with_lanes(|l| l[0][0]).ok());
    assert_eq!(held, Some(u64::MAX - 1), "nothing of a refused admission landed");
    let r = match refused {
        Err(r) => r,
        Ok(l) => panic!("refused, not {l}"),
    };
    assert!(r.to_string().contains("passes its carrier"), "{r}");
}
