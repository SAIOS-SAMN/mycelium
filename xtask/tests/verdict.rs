use super::super::verdict;

/// I4 — A TREE RED ON THE BOUND STILL REPORTS CLIPPY, and each reading
/// is refused on its own.
#[test]
fn verdict_reports_clippy_under_red_bound() {
    let both = verdict(Err("bound: over".into()), Err("clippy: 67".into()));
    assert_eq!(both, Err("bound: over\nclippy: 67".into()), "both readings, the first not hiding the second");
    assert_eq!(verdict(Err("bound: over".into()), Ok(())), Err("bound: over".into()));
    assert_eq!(verdict(Ok(()), Err("clippy: 1".into())), Err("clippy: 1".into()));
    assert_eq!(verdict(Ok(()), Ok(())), Ok(()), "the tree passes only where both do");
}
