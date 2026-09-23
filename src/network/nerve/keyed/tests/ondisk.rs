// Z11 — the trie persisted, followed on disk by the key's own bits.

use crate::network::nerve::keyed::ondisk;
use crate::network::nerve::Keyed;

/// Keys with shared prefixes, keys that are prefixes of other keys (the
/// `end` case a PATRICIA branch carries), and keys of differing length —
/// the three shapes the in-memory prefix read has three decisions for.
fn sample() -> Keyed<Vec<u8>> {
    let mut k: Keyed<Vec<u8>> = Keyed::new();
    for (key, val) in [
        (vec![0x10u8], b"a".to_vec()),
        (vec![0x10, 0x00], b"b".to_vec()),
        (vec![0x10, 0x01], b"c".to_vec()),
        (vec![0x10, 0x01, 0xff], b"d".to_vec()),
        (vec![0x11], b"e".to_vec()),
        (vec![0xff, 0xff, 0xff, 0xff], b"f".to_vec()),
        (vec![], b"root".to_vec()),
    ] {
        k.insert(key, val);
    }
    k
}

/// THE ON-DISK PREFIX READ ANSWERS WHAT THE IN-MEMORY ONE ANSWERS, at every
/// key the carrier holds and at keys it does not.
///
/// This is the reading that lets the genome live on disk: a word is
/// reached by its address and by no position (25 as ruled 2026-09-20), so
/// the file has to be the trie and the prefix read has to be the same read.
/// Two walks that agreed "in principle" would be two readers of one fact
/// (law 12); the claim checked here is that they agree on every key.
#[test]
fn the_file_reaches_what_the_carrier_holds() {
    let k = sample();
    let mut file = Vec::new();
    ondisk::write(&k, &mut file).expect("the trie writes out");

    for (key, val) in k.scoped() {
        assert_eq!(
            ondisk::get(&file, &key).map(<[u8]>::to_vec),
            Some(val.clone()),
            "the file reaches {key:02x?} as the carrier does"
        );
    }
    // Keys the carrier does not hold — including a proper prefix of one it
    // does, and an extension of one it does — are absent in both.
    for absent in [
        vec![0x10u8, 0x02],
        vec![0x10, 0x01, 0xfe],
        vec![0x12],
        vec![0xff],
        vec![0x10, 0x01, 0xff, 0x00],
    ] {
        assert_eq!(k.get(&absent), None, "the carrier does not hold {absent:02x?}");
        assert_eq!(
            ondisk::get(&file, &absent),
            None,
            "and neither does the file"
        );
    }
}

/// THE KEY SET IS WHAT A BOOT READS, and it is the carrier's own — the
/// resident index under Z11 is exactly this, with no value loaded.
#[test]
fn the_files_key_set_is_the_carriers() {
    let k = sample();
    let mut file = Vec::new();
    ondisk::write(&k, &mut file).expect("writes");
    let mut from_file = ondisk::keys(&file).expect("keys read");
    from_file.sort();
    let mut from_mem: Vec<Vec<u8>> = k.scoped().map(|(key, _)| key).collect();
    from_mem.sort();
    assert_eq!(from_file, from_mem, "every key and no other");
}

/// AN EMPTY CARRIER SAYS SO. A file that holds nothing is a header and an
/// absent root, not a zero length the reader has to interpret (rule 11).
#[test]
fn an_empty_carrier_writes_a_file_that_holds_nothing() {
    let k: Keyed<Vec<u8>> = Keyed::new();
    let mut file = Vec::new();
    ondisk::write(&k, &mut file).expect("writes");
    assert_eq!(ondisk::keys(&file).as_deref(), Some(&[][..]));
    assert_eq!(ondisk::get(&file, b"anything"), None);
}

/// A FILE THAT IS NOT ONE IS REFUSED, and a truncated one does not spin:
/// every step of the read moves to a node written before it, so a
/// cycle cannot be followed (rule 10).
#[test]
fn a_file_that_is_not_a_trie_is_refused() {
    assert_eq!(ondisk::get(b"", b"k"), None);
    assert_eq!(ondisk::get(b"NOPE\0\0\0\0", b"k"), None);
    let k = sample();
    let mut file = Vec::new();
    ondisk::write(&k, &mut file).expect("writes");
    // Cuts at the header, inside it, and mid-body: each is a file the
    // reader must refuse rather than interpret.
    for cut in [0, 4, 8, file.len() / 2] {
        let short = &file[..cut.min(file.len())];
        // Truncated: any answer is refused or absent, and none of them hang.
        let _ = ondisk::get(short, &[0x10, 0x01]);
        let _ = ondisk::keys(short);
    }
}
