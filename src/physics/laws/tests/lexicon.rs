/// 78.122 — THE LEXICON IS SERVED AS IT IS WRITTEN: the register reads back
/// whole at its version, every term once, no name in two vocabularies — and
/// every served term and the version stand in `MU-LEXICON.md`, so the table
/// and its exposition cannot drift.
#[test]
fn lexicon_reply_serves_terms_as_written() {
    use super::super::codec::{decode_laws_request, decode_lexicon_reply, encode_lexicon_reply, encode_lexicon_request, LawsRequest};
    use super::super::lexicon::{LEXICON_VERSION, TERMS};
    assert_eq!(decode_laws_request(&encode_lexicon_request()), Some(LawsRequest::Lexicon));
    let (version, served) = decode_lexicon_reply(&encode_lexicon_reply()).expect("the lexicon reads back");
    assert_eq!(version, LEXICON_VERSION);
    assert_eq!(served.len(), TERMS.len());
    for (i, s) in served.iter().enumerate() {
        assert!(served[i + 1..].iter().all(|o| o.name != s.name), "{} in one vocabulary only", s.name);
    }
    let doc = include_str!("../../../../docs/MU-LEXICON.md");
    assert!(doc.contains(&format!("**Status:** canonical, version {LEXICON_VERSION} (")), "the doc names the served version");
    for term in TERMS {
        assert!(doc.contains(term.name), "{} stands in MU-LEXICON.md", term.name);
    }
}
