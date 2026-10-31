// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 P5 — THE CONFORMANCE READ IN PROCESS: a client modelled
// as the profiles it declares and the set of acts it speaks; each act it
// speaks round-trips through both ends of its codec by decoded exact values,
// and a client that omits a MUST act of a declared profile fails by that
// act's name.

use mycelium_sdk::conformance::{check_codecs, ActRequirement, Finding, Need, Profile, STATEMENT};
use mycelium_sdk::wire::ACT_TABLE;

/// A client speaking exactly the named acts.
fn speaking(names: &'static [&'static str]) -> impl Fn(&ActRequirement) -> bool {
    move |a| names.contains(&a.name)
}

/// The acts `sdk/examples/client.rs` speaks.
const CLIENT_EXAMPLE: &[&str] = &["laws register", "task", "solve"];

/// The acts the reference mind (`mycelium-client`) speaks.
const REFERENCE_MIND: &[&str] =
    &["laws register", "task", "task by citation", "solve", "quote", "probe", "peers", "syllabus", "speech", "domain reserve", "seat mail"];

/// The acts `sdk/examples/house.rs` speaks through the kernel profile.
const HOUSE_EXAMPLE: &[&str] = &[
    "laws register",
    "laws statement",
    "laws concordance",
    "laws signature",
    "laws clock",
    "laws verify",
    "domain founding",
    "domain manifest",
    "domain universe",
];

/// Every act the statement names, in either profile, is an act the SDK may
/// send, named as the act table names it.
#[test]
fn statement_acts_stand_in_act_table() {
    for a in STATEMENT.union(&Profile::ALL) {
        let table = ACT_TABLE.iter().find(|(code, _)| *code == a.act);
        let Some((_, name)) = table else { panic!("{} names act {} outside the act table", a.name, a.act) };
        assert!(a.name.starts_with(name), "{} is not named for the act table's {name}", a.name);
    }
}

/// A client speaking every act of each profile, and of both, passes every
/// act's check — each act, MUST or MAY, has one — and conforms.
#[test]
fn check_codecs_round_trips_every_act() {
    for profiles in [&[Profile::Mind][..], &[Profile::Kernel], &Profile::ALL] {
        let report = check_codecs(profiles, |_| true);
        assert_eq!(report.failures().count(), 0, "{report}");
        assert!(report.conforms(), "{report}");
        for a in STATEMENT.union(profiles) {
            assert!(report.findings.iter().any(|f| f.act() == a.name && f.passed()), "{} unchecked\n{report}", a.name);
        }
    }
}

/// The client example and the reference mind each conform as minds.
#[test]
fn check_codecs_admits_client_example_and_reference_mind() {
    for client in [CLIENT_EXAMPLE, REFERENCE_MIND] {
        let report = check_codecs(&[Profile::Mind], speaking(client));
        assert!(report.conforms(), "{client:?}\n{report}");
    }
}

/// A client that omits a MUST act of its profile fails by that act's name,
/// and by none other.
#[test]
fn check_codecs_refuses_client_omitting_must_act() {
    for profile in Profile::ALL {
        for omitted in STATEMENT.of_need(profile, Need::Must) {
            let report = check_codecs(&[profile], |a| a.name != omitted.name);
            assert!(!report.conforms());
            let failed: Vec<&str> = report.failures().map(|(act, _)| act).collect();
            assert_eq!(failed, [omitted.name]);
        }
    }
}

/// A kernel speaking only its uplink conforms, and is never checked, failed
/// or held to task or solve.
#[test]
fn conformance_kernel_profile_omits_task_and_solve() {
    let report = check_codecs(&[Profile::Kernel], speaking(HOUSE_EXAMPLE));
    assert!(report.conforms(), "{report}");
    for name in ["task", "solve"] {
        assert!(report.findings.iter().all(|f| f.act() != name), "{name} checked:\n{report}");
        assert_eq!(report.need(name), Some(Need::May));
    }
    let mind = check_codecs(&[Profile::Mind], speaking(HOUSE_EXAMPLE));
    let failed: Vec<&str> = mind.failures().map(|(act, _)| act).collect();
    assert_eq!(failed, ["task", "solve"], "the same client, declared a mind, fails by task and solve");
}

/// A kernel that does not found its domain fails by that act's name alone.
#[test]
fn conformance_kernel_missing_founding_fails_by_name() {
    let report = check_codecs(&[Profile::Kernel], |a| HOUSE_EXAMPLE.contains(&a.name) && a.name != "domain founding");
    assert!(!report.conforms());
    let failed: Vec<&str> = report.failures().map(|(act, _)| act).collect();
    assert_eq!(failed, ["domain founding"]);
}

/// A client declaring both profiles is held to the MUST acts of both.
#[test]
fn check_codecs_holds_both_profiles_to_union() {
    let both = check_codecs(&Profile::ALL, speaking(HOUSE_EXAMPLE));
    let failed: Vec<&str> = both.failures().map(|(act, _)| act).collect();
    assert_eq!(failed, ["task", "solve"]);
    let mut all = HOUSE_EXAMPLE.to_vec();
    all.extend_from_slice(CLIENT_EXAMPLE);
    let report = check_codecs(&Profile::ALL, |a| all.contains(&a.name));
    assert!(report.conforms(), "{report}");
}

/// A report that declares no profile conforms to nothing.
#[test]
fn report_without_profile_does_not_conform() {
    let report = check_codecs(&[], |_| true);
    assert!(report.findings.is_empty());
    assert!(!report.conforms());
}

/// A finding names its act, and only a pass passes.
#[test]
fn finding_names_act() {
    let passed = Finding::Passed { act: "task", read: String::new() };
    let unread = Finding::Unread { act: "solve", reason: String::new() };
    assert_eq!((passed.act(), passed.passed()), ("task", true));
    assert_eq!((unread.act(), unread.passed()), ("solve", false));
}
