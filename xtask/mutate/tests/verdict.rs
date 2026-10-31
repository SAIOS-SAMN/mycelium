use super::super::verdict::{summary, MutantVerdict};

/// The audit passes only where every mutant was killed; a survivor, a
/// stale anchor or an unbuilt mutant each leave it red.
#[test]
fn summary_passes_only_when_every_mutant_killed() {
    let killed = MutantVerdict::Killed(vec!["t".to_string()]);
    assert!(summary(&[killed.clone(), killed.clone()]).1);
    for gap in [MutantVerdict::Survived, MutantVerdict::Stale("moved".into()), MutantVerdict::Unbuilt("e".into())] {
        assert!(!summary(&[killed.clone(), gap]).1);
    }
}
