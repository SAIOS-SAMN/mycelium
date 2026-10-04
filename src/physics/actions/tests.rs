// 78.94 — THE ONE TABLE OF ACTIONS: every action at one coordinate, its
// kind, whether a court seals it, and the laws it answers to.

use super::*;
use crate::anastome::dialogue as d;

/// EVERY ACTION HOLDS ONE COORDINATE AND ONE ONLY: the table names each
/// action once, no two share a byte, every byte reads back as its action, a
/// party's actions stand below the court's range and a court's own at or
/// above it, and a byte no action holds names nothing.
#[test]
fn every_action_holds_one_coordinate() {
    for (i, a) in ACTIONS.iter().enumerate() {
        assert_eq!(Action::of(a.byte()), Some(*a), "{} reads back", String::from_utf8_lossy(a.name()));
        for b in &ACTIONS[i + 1..] {
            assert_ne!(a.byte(), b.byte(), "one coordinate each");
            assert_ne!(a, b, "the table names each action once");
        }
        assert_eq!(a.on_wire(), a.kind() != Kind::Court && !matches!(a, Action::Section | Action::Stood | Action::Uttered | Action::Learned));
    }
    for unassigned in [0x07u8, 0x0A, 0x0F, 0x3D, 0x53, 0xFF] {
        assert_eq!(Action::of(unassigned), None);
    }
    assert_eq!(Action::of_tag(Action::Orbit.byte()), None, "a court act is never asked for");
}

/// WHAT A COURT SEALS is exactly its own acts, the syllabus page and the
/// fold — the kinds a receipt carries; an observation never seals, answers
/// to no law, and cannot be a node; a transition answers to every law.
#[test]
fn an_observation_seals_nothing_and_a_transition_answers_to_every_law() {
    let sealed: Vec<Action> = ACTIONS.iter().copied().filter(|a| a.sealed()).collect();
    assert_eq!(sealed.len(), 17, "{sealed:?}");
    for a in ACTIONS {
        match a.kind() {
            Kind::Observation | Kind::Retired => {
                assert!(a.laws().is_empty());
                assert!(!a.sealed(), "{a:?}");
                assert_eq!(Action::from_byte(a.byte()), None);
            }
            Kind::Transition | Kind::Exchange | Kind::Court => {
                assert_eq!(a.laws(), &crate::physics::laws::LAWS);
            }
        }
    }
    let mut chain = crate::chronometry::ReceiptChain::with_vk([3; 32]);
    let op = [0u8; 32];
    assert!(chain.append(Action::Why, [0; 4], Vec::new(), op, Vec::new()).is_none(), "an observation writes no node");
    assert!(chain.append(Action::Speech, [0; 4], Vec::new(), op, Vec::new()).is_none(), "a request is not its sealed outcome");
    assert!(chain.append(Action::Attest, [0; 4], Vec::new(), op, Vec::new()).is_some(), "a court's act is sealed");
}

/// THE WIRE'S SPELLINGS ARE THE TABLE'S COORDINATES: each `ACT_*` names the
/// action the table holds at that byte, of the kind the table says.
#[test]
fn the_wire_spells_the_table() {
    let spelled = [
        (d::ACT_SPEECH, Action::Speech, Kind::Transition),
        (d::ACT_PROBE, Action::Probe, Kind::Transition),
        (d::ACT_SYLLABUS, Action::Syllabus, Kind::Transition),
        (d::ACT_FOLD, Action::Fold, Kind::Exchange),
        (d::ACT_EQUIVOCATION, Action::Equivocation, Kind::Exchange),
        (d::ACT_QUOTE, Action::Quote, Kind::Observation),
        (d::ACT_LAWS, Action::Laws, Kind::Observation),
        (d::ACT_WHY, Action::Why, Kind::Observation),
        (d::ACT_WALLET, Action::Wallet, Kind::Observation),
        (d::ACT_PERCEPT, Action::Percept, Kind::Retired),
        (d::ACT_TEACH, Action::Teach, Kind::Retired),
        (d::ACT_DOCKET, Action::Docket, Kind::Retired),
    ];
    for (tag, action, kind) in spelled {
        assert_eq!(Action::of_tag(tag), Some(action));
        assert_eq!(action.kind(), kind, "{action:?}");
    }
    // A sealed act's name is unchanged by the table, so every operator
    // digest over it is unchanged.
    assert_eq!(Action::Install.name(), b"install");
    assert_eq!(Action::Ruling(Ruled::Slashed).name(), b"ruling");
    assert!(Action::Install.changes_algorithm() && !Action::Settled.changes_algorithm());
}
