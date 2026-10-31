// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 P5 — THE CONFORMANCE READ IN PROCESS: a client modelled
// as the profiles it declares and the set of acts it speaks; each act it
// speaks round-trips through both ends of its codec by decoded exact values,
// and a client that omits a MUST act of a declared profile fails by that
// act's name.

use mycelium_sdk::conformance::{check_codecs, ActRequirement, Finding, Need, Profile, DEFERRAL, STATEMENT};
use mycelium_sdk::deferral::{deferral_of, encode_deferral, read_deferral, Deferral, PENDING_CODE};
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
    let deferred = Finding::Deferred { act: "task", deficit: [1, 0, 2, 0] };
    assert_eq!((passed.act(), passed.passed()), ("task", true));
    assert_eq!((unread.act(), unread.passed()), ("solve", false));
    assert_eq!((deferred.act(), deferred.passed()), ("task", false));
}

/// Every profile is held to the deferral: it is checked, MUST, and passes
/// — no MUST reply decoder reads a deferral's bytes as a reply.
#[test]
fn check_codecs_holds_every_profile_to_deferral() {
    for profiles in [&[Profile::Mind][..], &[Profile::Kernel], &Profile::ALL] {
        let report = check_codecs(profiles, |_| true);
        assert_eq!(report.need(DEFERRAL), Some(Need::Must));
        assert!(report.findings.iter().any(|f| f.act() == DEFERRAL && f.passed()), "{report}");
    }
}

/// A deferral reads back with the exact deficit the door stated; an answer
/// that carries a height, and a frame cut short, are not deferrals; and a
/// deferral reads as no task envelope and no verdict.
#[test]
fn read_deferral_round_trips_deficit() {
    for deficit in [[0; 4], [1, 0, 2, 3], [u64::MAX, 0, 0, 1]] {
        let bytes = encode_deferral(PENDING_CODE, &deficit);
        assert_eq!(read_deferral(&bytes), Some(Deferral { deficit }));
        assert!(read_deferral(&bytes[..bytes.len() - 1]).is_none());
        assert!(mycelium_sdk::task::decode_task_envelope(&bytes).is_none());
        assert!(mycelium_sdk::task::decode_solve_verdict(&bytes).is_none());
    }
    // The height is the eight bytes after the code: a pending outcome at
    // height 3 is an answer.
    let mut answer = encode_deferral(PENDING_CODE, &[1, 0, 0, 0]);
    answer[1] = 3;
    assert!(read_deferral(&answer).is_none(), "a pending outcome with a height is an answer");
}

/// A session's refusal carries the deferral where `speak` met one, and no
/// other refusal reads as one.
#[test]
fn deferral_of_reads_session_refusal() {
    let d = Deferral { deficit: [0, 4, 0, 1] };
    assert_eq!(deferral_of(&std::io::Error::new(std::io::ErrorKind::WouldBlock, d)), Some(d));
    assert_eq!(deferral_of(&std::io::Error::new(std::io::ErrorKind::WouldBlock, "full")), None);
    assert_eq!(deferral_of(&std::io::Error::new(std::io::ErrorKind::TimedOut, d)), None);
}
