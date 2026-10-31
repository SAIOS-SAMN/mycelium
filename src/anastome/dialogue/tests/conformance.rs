// MU-PRECURSORS §4 P5 — THE CLIENT CONFORMANCE STATEMENT, by decoded exact
// values (Concordance 1.4): the statement a hypha serves reads back as the
// statement this tree holds, a damaged one reads as none, and the request
// for it is the register's sixth mode.

use crate::anastome::dialogue::conformance::*;
use crate::physics::laws::codec::{decode_laws_request, encode_statement_request, LawsRequest};

/// The served statement decodes to the statement held, field by field.
#[test]
fn statement_round_trips() {
    let read = decode_statement_reply(&encode_statement_reply());
    assert_eq!(read, Some(StatementRead::of(&STATEMENT)));
}

/// A statement short of a byte, or with one left over, is no statement.
#[test]
fn statement_refuses_truncated_and_trailing_bytes() {
    let bytes = encode_statement_reply();
    assert_eq!(decode_statement_reply(&bytes[..bytes.len() - 1]), None);
    let mut longer = bytes.clone();
    longer.push(0);
    assert_eq!(decode_statement_reply(&longer), None);
}

/// A need byte no need names refuses the whole statement.
#[test]
fn statement_refuses_unknown_need() {
    assert_eq!(Need::of(2), None);
    for need in [Need::Must, Need::May] {
        assert_eq!(Need::of(need.byte()), Some(need));
    }
}

/// Every act is named once, and the client loop's acts are the MUST acts.
#[test]
fn statement_names_each_act_once() {
    for (i, a) in STATEMENT.acts.iter().enumerate() {
        assert!(STATEMENT.acts.iter().skip(i + 1).all(|b| b.name != a.name), "{} named twice", a.name);
        assert_eq!(STATEMENT.act(a.name), Some(a));
    }
    let must: Vec<&str> = STATEMENT.of_need(Need::Must).map(|a| a.name).collect();
    assert_eq!(must, ["task", "solve", "laws register"]);
}

/// The statement is asked as `[ACT_LAWS][6]`, read whole.
#[test]
fn statement_request_round_trips() {
    assert_eq!(decode_laws_request(&encode_statement_request()), Some(LawsRequest::Statement));
    let mut longer = encode_statement_request();
    longer.push(0);
    assert_eq!(decode_laws_request(&longer), None);
}
