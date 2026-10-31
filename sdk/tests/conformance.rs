// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 P5 — THE CONFORMANCE READ IN PROCESS: a client modelled
// as the set of acts it speaks; each act it speaks round-trips through both
// ends of its codec by decoded exact values, and a client that omits a MUST
// act fails by that act's name.

use mycelium_sdk::conformance::{check_codecs, ActRequirement, Finding, Need, STATEMENT};
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

/// Every act the statement names is an act the SDK may send, named as the
/// act table names it.
#[test]
fn statement_acts_stand_in_act_table() {
    for a in STATEMENT.acts {
        let table = ACT_TABLE.iter().find(|(code, _)| *code == a.act);
        let Some((_, name)) = table else { panic!("{} names act {} outside the act table", a.name, a.act) };
        assert!(a.name.starts_with(name), "{} is not named for the act table's {name}", a.name);
    }
}

/// A client speaking every act passes each check it has, and conforms.
#[test]
fn check_codecs_round_trips_every_act() {
    let report = check_codecs(|_| true);
    assert_eq!(report.failures().count(), 0, "{report}");
    assert!(report.conforms(), "{report}");
    for a in STATEMENT.of_need(Need::Must) {
        assert!(report.findings.iter().any(|f| f.act() == a.name && f.passed()), "{} unchecked", a.name);
    }
}

/// The client example and the reference mind each conform.
#[test]
fn check_codecs_admits_client_example_and_reference_mind() {
    for client in [CLIENT_EXAMPLE, REFERENCE_MIND] {
        let report = check_codecs(speaking(client));
        assert!(report.conforms(), "{client:?}\n{report}");
    }
}

/// A client that omits a MUST act fails by that act's name, and by none
/// other.
#[test]
fn check_codecs_refuses_client_omitting_must_act() {
    for omitted in STATEMENT.of_need(Need::Must) {
        let report = check_codecs(|a| a.name != omitted.name);
        assert!(!report.conforms());
        let failed: Vec<&str> = report.failures().map(|(act, _)| act).collect();
        assert_eq!(failed, [omitted.name]);
    }
}

/// A finding names its act, and only a pass passes.
#[test]
fn finding_names_act() {
    let passed = Finding::Passed { act: "task", read: String::new() };
    let unread = Finding::Unread { act: "solve", reason: String::new() };
    assert_eq!((passed.act(), passed.passed()), ("task", true));
    assert_eq!((unread.act(), unread.passed()), ("solve", false));
}
