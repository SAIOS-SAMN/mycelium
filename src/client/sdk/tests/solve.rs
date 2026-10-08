// 26, the actions' physics — A VOICE SENDS ONLY A SOLVE IT CAN STAND BEHIND:
// the rule replayed on its own train exactly as the court will replay it on
// the withheld pairs; no rule, a rule its train refutes, or a rule the court
// already refused is kept home, each counted by name.

use crate::client::sdk::*;

fn grid(rows: &[&str]) -> Vec<Vec<u8>> {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// The domino painted 2 and carried two columns right, on two rows.
fn family() -> Vec<(crate::arc::Grid, crate::arc::Grid)> {
    vec![
        (grid(&["1100", "0000"]), grid(&["0022", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0022"])),
    ]
}

#[test]
fn a_voice_keeps_home_a_solve_its_own_train_refutes() {
    let train = family();
    let rule = crate::transducer::family::extract_rule(&train, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts");
    let spoken = crate::transducer::family::mint_sentence(&rule, &train, &crate::transducer::family::NoCrystal, 0)
        .expect("and mints");
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("solver"));
    let mut budget = entity.stroke_n(1 << 20);
    let outs = vec![grid(&["0000", "0000"])];
    // NO RULE: nothing is sent.
    // Wire v1.2: a task is named by its 32-byte citation.
    let task = [7u8; 32];
    assert_eq!(entity.solve_task(&task, &train, &outs, None, &mut budget), None);
    assert_eq!(entity.solves.unminted, 1, "a raster is no proof of a motion");
    // A TRAIN THE RULE DOES NOT SPEAK: the pairs left unchanged. The voice's
    // own replay refuses it, as the court's would; nothing is sent.
    let other = vec![
        (grid(&["1100", "0000"]), grid(&["1100", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "1100"])),
    ];
    assert_eq!(entity.solve_task(&task, &other, &outs, Some(&spoken), &mut budget), None);
    assert_eq!(entity.solves.refuted, 1, "refuted at home");
    // ITS OWN TRAIN: the replay is clean, so the solve goes to the wire —
    // which answers nothing here, and nothing is counted as refuted.
    let _ = entity.solve_task(&task, &train, &outs, Some(&spoken), &mut budget);
    assert_eq!(entity.solves.refuted, 1, "the rule its own train speaks is not refuted");
    // THE SENSES READ IT, typed.
    let senses = entity.senses(&crate::observe::Learning::default());
    assert_eq!(senses.read("solving", "unminted"), Some(1));
    assert_eq!(senses.read("solving", "refuted at home"), Some(1));
}
