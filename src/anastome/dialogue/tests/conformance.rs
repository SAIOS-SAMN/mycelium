// MU-PRECURSORS §4 P5 — THE CLIENT CONFORMANCE STATEMENT, by decoded exact
// values (Concordance 1.4): the statement a hypha serves reads back as the
// statement this tree holds, both profiles; a damaged one reads as none; the
// single-profile form of v0.3.0-rc.1 reads as the mind profile; and the
// request for it is the register's sixth mode.

use crate::anastome::dialogue::conformance::*;
use crate::physics::laws::codec::{decode_laws_request, encode_statement_request, LawsRequest};

/// The served statement decodes to the statement held, field by field.
#[test]
fn statement_round_trips() {
    let read = decode_statement_reply(&encode_statement_reply());
    assert_eq!(read, Some(StatementRead::of(&STATEMENT)));
}

/// Both profiles travel, each with its own acts in its own order.
#[test]
fn statement_round_trips_with_both_profiles() {
    let Some(read) = decode_statement_reply(&encode_statement_reply()) else { panic!("the statement did not decode") };
    let profiles: Vec<Profile> = read.profiles.iter().map(|p| p.profile).collect();
    assert_eq!(profiles, Profile::ALL);
    for profile in Profile::ALL {
        let names: Vec<&str> = read.acts(profile).iter().map(|a| a.name.as_str()).collect();
        let held: Vec<&str> = STATEMENT.acts(profile).iter().map(|a| a.name).collect();
        assert_eq!(names, held, "{}", profile.name());
    }
}

/// The single-profile form v0.3.0-rc.1 served reads as the mind profile
/// alone, its acts exact.
#[test]
fn statement_reads_single_profile_form() {
    let mut bytes = STATEMENT.wire_version.to_le_bytes().to_vec();
    bytes.push(STATEMENT.law_version);
    bytes.extend_from_slice(&STATEMENT.concordance_hash);
    put_acts(&mut bytes, STATEMENT.acts(Profile::Mind));
    let Some(read) = decode_statement_reply(&bytes) else { panic!("the single-profile form did not decode") };
    let held = StatementRead::of(&STATEMENT);
    assert_eq!(read.profiles, held.profiles[..1]);
    assert_eq!(read.acts(Profile::Kernel), []);
}

/// A statement short of a byte, with one left over, or with a form mark no
/// form names, is no statement.
#[test]
fn statement_refuses_truncated_trailing_and_unknown_form() {
    let bytes = encode_statement_reply();
    assert_eq!(decode_statement_reply(&bytes[..bytes.len() - 1]), None);
    let mut longer = bytes.clone();
    longer.push(0);
    assert_eq!(decode_statement_reply(&longer), None);
    let mut other = bytes.clone();
    other[2 + 1 + 32 + 1] = FORM_PROFILES + 1;
    assert_eq!(decode_statement_reply(&other), None);
}

/// A need or profile byte no need or profile names refuses the whole
/// statement.
#[test]
fn statement_refuses_unknown_need_and_profile() {
    assert_eq!(Need::of(2), None);
    for need in [Need::Must, Need::May] {
        assert_eq!(Need::of(need.byte()), Some(need));
    }
    assert_eq!(Profile::of(2), None);
    for profile in Profile::ALL {
        assert_eq!(Profile::of(profile.byte()), Some(profile));
    }
}

/// Every act is named once in its profile; the mind's MUST acts are its
/// client loop, and the kernel's are its uplink, task and solve only MAY.
#[test]
fn statement_names_each_act_once() {
    for profile in Profile::ALL {
        let acts = STATEMENT.acts(profile);
        for (i, a) in acts.iter().enumerate() {
            assert!(acts.iter().skip(i + 1).all(|b| b.name != a.name), "{} named twice", a.name);
            assert_eq!(STATEMENT.act(profile, a.name), Some(a));
        }
    }
    let must: Vec<&str> = STATEMENT.of_need(Profile::Mind, Need::Must).map(|a| a.name).collect();
    assert_eq!(must, ["task", "solve", "laws register"]);
    let must: Vec<&str> = STATEMENT.of_need(Profile::Kernel, Need::Must).map(|a| a.name).collect();
    let uplink = [
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
    assert_eq!(must, uplink);
    for name in ["task", "solve"] {
        assert_eq!(STATEMENT.act(Profile::Kernel, name).map(|a| a.need), Some(Need::May));
    }
}

/// A client declaring both profiles speaks the union: MUST where either
/// makes it MUST, each act once.
#[test]
fn statement_union_takes_strongest_need() {
    let both = STATEMENT.union(&Profile::ALL);
    for (i, a) in both.iter().enumerate() {
        assert!(both.iter().skip(i + 1).all(|b| b.name != a.name), "{} named twice", a.name);
        let must = Profile::ALL.iter().any(|p| STATEMENT.act(*p, a.name).is_some_and(|x| x.need == Need::Must));
        assert_eq!(a.need == Need::Must, must, "{}", a.name);
    }
    let task = both.iter().find(|a| a.name == "task").map(|a| a.need);
    assert_eq!(task, Some(Need::Must));
}

/// The statement is asked as `[ACT_LAWS][6]`, read whole.
#[test]
fn statement_request_round_trips() {
    assert_eq!(decode_laws_request(&encode_statement_request()), Some(LawsRequest::Statement));
    let mut longer = encode_statement_request();
    longer.push(0);
    assert_eq!(decode_laws_request(&longer), None);
}
