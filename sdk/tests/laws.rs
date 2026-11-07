// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-EQUATIONS, SHARED LAWS — every row of the shared-law table the SDK
// publishes resolves: each L0 law at its coordinate and version, the
// register served whole, and each forms shell by its id.

use mycelium_sdk::laws::{decode_register_reply, law_at, FORM_LAW_IDS, LAWS, LAW_VERSION};

/// Each law reads back from its coordinate at the register's version.
#[test]
fn laws_resolve_at_coordinates() {
    for law in LAWS {
        assert_eq!(law_at(&law.address()), Some(law));
        assert!(matches!(law.address().as_bytes(), [_, index, version] if *index == law.index() && *version == LAW_VERSION));
    }
}

/// The register a hypha serves carries every law of the register, in order.
#[test]
fn register_reply_carries_every_law() {
    let served = decode_register_reply(&mycelium::physics::laws::codec::encode_register_reply()).expect("the register reads at its version");
    assert_eq!(served.len(), LAWS.len());
    for (s, law) in served.iter().zip(LAWS) {
        assert_eq!((s.law, s.statement.as_str()), (law, law.statement()));
    }
}

/// The forms rows are the `forms` crate's shells, by id, each at the
/// version MU-EQUATIONS §11 tables: `FORMS-FRAME` 1, `FORMS-PASSAGE` 4,
/// `FORMS-SHARES` 1, `FORMS-COORDINATE` 4, `FORMS-COUPLING` 1,
/// `FORMS-TURNING` 1.
#[test]
fn form_law_ids_equal_forms_shells() {
    assert_eq!(
        FORM_LAW_IDS,
        [
            (mycelium_sdk::forms::reference::LAW_ID, 1),
            (mycelium_sdk::forms::passage::LAW_ID, 4),
            (mycelium_sdk::forms::shares::LAW_ID, 1),
            (mycelium_sdk::forms::coordinate::LAW_ID, 4),
            (mycelium_sdk::forms::coupling::LAW_ID, 1),
            (mycelium_sdk::forms::turning::LAW_ID, 1)
        ]
    );
}
