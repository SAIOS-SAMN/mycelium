// THE SHARED-LAW TABLE'S FORMS ROWS: every id the crate publishes is a
// shell's own, once, at the shell's own version.

use crate::LAW_IDS;

/// Each entry resolves to a hosted shell, and each shell stands once.
#[test]
fn law_ids_resolve_to_shells() {
    let shells = [(crate::frame::LAW_ID, crate::frame::LAW_VERSION), (crate::shares::LAW_ID, crate::shares::LAW_VERSION)];
    assert_eq!(LAW_IDS, shells.as_slice());
    for (i, (id, version)) in LAW_IDS.iter().enumerate() {
        assert!(id.starts_with("FORMS-"), "{id} is no forms law id");
        assert!(*version >= 1, "{id} stands at no version");
        assert!(LAW_IDS.iter().skip(i + 1).all(|(other, _)| other != id), "{id} named twice");
    }
}
