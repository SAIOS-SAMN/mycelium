use super::super::audit::repo_root;
use super::super::catalogue::CATALOGUE;

/// Every catalogued mutant changes its span and names a file and a suite.
#[test]
fn catalogue_changes_span_and_names_suite_per_mutant() {
    for m in CATALOGUE {
        assert_ne!(m.before, m.after, "{}", m.law);
        assert!(!m.before.is_empty() && !m.file.is_empty() && !m.filter.is_empty(), "{}", m.law);
    }
}

/// Every `before` span occurs exactly once in its file in this tree.
#[test]
fn catalogue_keeps_each_anchor_once_in_file() {
    let repo = repo_root().expect("the xtask manifest sits in the repository");
    for m in CATALOGUE {
        let text = std::fs::read_to_string(repo.join(m.file)).unwrap_or_default();
        assert_eq!(text.matches(m.before).count(), 1, "{} [{}]", m.law, m.file);
    }
}
