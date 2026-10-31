use super::super::*;

pub(super) fn party(b: u8) -> [u8; 32] {
    [b; 32]
}

/// A court's readings held under its key, written as a query reply and read
/// back: every one as itself.
pub(super) fn reads_back(all: &[Reading]) {
    let entries: Vec<(Vec<u8>, Reading)> = all.iter().map(|r| (Source::Court(party(7)).key(r.tag()), r.clone())).collect();
    let reply = encode_reply(&entries, usize::MAX).expect("the reply writes");
    let total = u32::try_from(entries.len()).expect("a few readings");
    assert_eq!(decode_reply(&reply), Some((total, entries)));
}
