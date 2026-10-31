use crate::court::identity::HyphaKey;
use crate::network::mesh::*;

/// A signed mesh record opens under its node's key with its bind, peers,
/// bonds, genesis and epoch intact.
#[test]
fn mesh_record_round_trips_and_verifies() {
    let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-test.key");
    let rec = MeshRecord {
        kind: HYPHA,
        bind: Some("127.0.0.1:7500".into()),
        peers: vec!["127.0.0.1:7501".into(), "127.0.0.1:7502".into()],
        bootstrap: String::new(),
        hypha: vec![([1; 32], [8, 8, 1, 8])],
        voice: vec![([2; 32], [8, 8, 1, 8])],
        genesis: Some([7; 32]),
        epoch: 2,
    };
    let bytes = rec.signed_bytes(&key).expect("the record fits its lengths");
    let back = MeshRecord::open(&bytes, &key.verifying_bytes()).expect("verify");
    assert_eq!(back.bind, rec.bind);
    assert_eq!(back.peers, rec.peers);
    assert_eq!(back.hypha[0].1, [8, 8, 1, 8]);
    assert_eq!(back.genesis, rec.genesis);
    assert_eq!(back.epoch, rec.epoch);
}

/// 45.5: a genesis of all zeros is not a genesis, it is the ABSENCE of
/// one. The field held `[0u8; 32]` and every reader compared against
/// that sentinel, so a node with no origin and a node whose origin
/// happened to be zero were the same value. MSH2 carries a PRESENCE
/// BYTE, so the tape says which it is rather than a reader inferring
/// it, and 47.2's origin check can never meet the sentinel.
#[test]
fn genesis_none_reads_back_absent_not_zero() {
    let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-nogen.key");
    let rec = MeshRecord {
        kind: HYPHA,
        bind: Some("127.0.0.1:7500".into()),
        peers: vec![],
        bootstrap: String::new(),
        hypha: vec![],
        voice: vec![],
        genesis: None,
        epoch: 2,
    };
    let bytes = rec.signed_bytes(&key).expect("the record fits its lengths");
    let back = MeshRecord::open(&bytes, &key.verifying_bytes()).expect("verify");
    assert_eq!(back.genesis, None, "unwritten stays unwritten");

    let written = MeshRecord { genesis: Some([9; 32]), ..rec };
    let back = MeshRecord::open(&written.signed_bytes(&key).expect("fits"), &key.verifying_bytes()).expect("verify");
    assert_eq!(back.genesis, Some([9; 32]), "a written one survives");

    // The sentinel it replaces: a genesis that IS all zeros is a
    // genesis, and reads back as one. MSH1 could not say this.
    let zeros = MeshRecord { genesis: Some([0; 32]), ..written };
    let back = MeshRecord::open(&zeros.signed_bytes(&key).expect("fits"), &key.verifying_bytes()).expect("verify");
    assert_eq!(back.genesis, Some([0; 32]), "a zero origin is an origin");
}

/// A bind nothing has written is ABSENT, and an address that is the
/// empty string is a bind of no characters. Two different records.
#[test]
fn bind_none_reads_back_unbound_not_empty() {
    let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-nobind.key");
    let rec = MeshRecord {
        kind: HYPHA,
        bind: None,
        peers: vec![],
        bootstrap: String::new(),
        hypha: vec![],
        voice: vec![],
        genesis: None,
        epoch: 0,
    };
    let back = MeshRecord::open(&rec.signed_bytes(&key).expect("fits"), &key.verifying_bytes()).expect("verify");
    assert_eq!(back.bind, None, "unbound stays unbound");

    let empty = MeshRecord { bind: Some(String::new()), ..rec };
    let back = MeshRecord::open(&empty.signed_bytes(&key).expect("fits"), &key.verifying_bytes()).expect("verify");
    assert_eq!(back.bind, Some(String::new()), "an empty address is an address");
}

/// §10 — A RECUT KEEPS THE PROVISIONING. The bind, the peers and the
/// bootstrap are how a node is wired into the mesh, and they live in
/// the record a magic bump would make unreadable. MSH1 opens under its
/// own rules and the next save writes MSH2.
///
/// Measured before this stood: all eight homes of the mesh held MSH1,
/// `open_at` refused every one of them, and the nodes — including the
/// one-shot `--recut` runs — exited 2 at boot.
#[test]
fn open_reads_msh1_record_under_its_rules() {
    let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-v1.key");
    // The MSH1 layout, written out: magic, kind, bind as a plain
    // length-prefixed string, peers, bootstrap, bonds, 32 genesis
    // bytes, epoch.
    let mut v1 = Vec::from(*MAGIC_V1);
    v1.push(HYPHA);
    put_str(&mut v1, "127.0.0.1:7500").expect("fits");
    crate::transducer::address::put_u8_count(&mut v1,1).expect("fits");
    put_str(&mut v1, "127.0.0.1:7501").expect("fits");
    put_str(&mut v1, "").expect("fits");
    crate::transducer::address::put_u8_count(&mut v1,1).expect("fits");
    put_bond(&mut v1, &[5; 32], &[1, 2, 3, 4]);
    crate::transducer::address::put_u8_count(&mut v1,0).expect("fits");
    v1.extend_from_slice(&[9u8; 32]);
    v1.extend_from_slice(&7u32.to_le_bytes());
    let sig = key.sign(&v1);
    let mut signed = v1.clone();
    signed.extend_from_slice(&sig);

    let back = MeshRecord::open(&signed, &key.verifying_bytes()).expect("the old record opens");
    assert_eq!(back.bind, Some("127.0.0.1:7500".into()), "the bind survives");
    assert_eq!(back.peers, vec!["127.0.0.1:7501".to_string()], "and the peers");
    assert_eq!(back.hypha, vec![([5; 32], [1, 2, 3, 4])], "and the genesis bonds");
    assert_eq!(back.genesis, Some([9; 32]), "and the origin it stood under");
    assert_eq!(back.epoch, 7);
    // MSH1's own conflations, read as MSH1 wrote them: an empty bind is
    // "bound nowhere" and 32 zeros are "no genesis". That is what MSH2
    // exists to end, and it ends for records written from here on.
    let mut bare = Vec::from(*MAGIC_V1);
    bare.push(VOICE);
    put_str(&mut bare, "").expect("fits");
    crate::transducer::address::put_u8_count(&mut bare,0).expect("fits");
    put_str(&mut bare, "").expect("fits");
    crate::transducer::address::put_u8_count(&mut bare,0).expect("fits");
    crate::transducer::address::put_u8_count(&mut bare,0).expect("fits");
    bare.extend_from_slice(&[0u8; 32]);
    bare.extend_from_slice(&0u32.to_le_bytes());
    let (rec, _, layout) = MeshRecord::decode_layout(&bare).expect("the bare old record decodes");
    assert_eq!(layout, Layout::Legacy, "and the reading names the layout it read");
    assert_eq!(rec.bind, None, "an empty MSH1 bind is absence");
    assert_eq!(rec.genesis, None, "and 32 zeros are no genesis");

    // AND THE NEXT WRITE IS MSH2: the record re-signs under the new
    // magic and reads back with the presence bytes.
    let re = back.signed_bytes(&key).expect("fits");
    assert_eq!(re.get(0..4), Some(&MAGIC[..]), "saved under the current magic");
    let again = MeshRecord::open(&re, &key.verifying_bytes()).expect("and opens");
    assert_eq!(again.bind, back.bind);
    assert_eq!(again.genesis, back.genesis);
    assert_eq!(again.peers, back.peers);
}

/// X7 — THE OLD ARM HAS AN END. A boot that reads an MSH1 record writes
/// it back as MSH2 there and then, so the mesh retires the layout by
/// booting rather than by a flag, and the arm that reads MSH1 can be
/// deleted once every node has. Read here on the migration door
/// itself, since that is where the boot enters: the record on disk is
/// MSH1 before the call and MSH2 after it, with the wiring §10 says a
/// recut keeps — bind, peers, bootstrap, bonds, origin, epoch — intact
/// across the rewrite, and the second call writes nothing new.
#[test]
fn load_or_migrate_hypha_resaves_msh1_as_msh2_once() {
    let home = std::path::PathBuf::from("/tmp/mycelium-mesh-x7");
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).expect("a home to boot in");
    let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-x7.key");

    let mut v1 = Vec::from(*MAGIC_V1);
    v1.push(HYPHA);
    put_str(&mut v1, "127.0.0.1:7500").expect("fits");
    crate::transducer::address::put_u8_count(&mut v1, 1).expect("fits");
    put_str(&mut v1, "127.0.0.1:7501").expect("fits");
    put_str(&mut v1, "").expect("fits");
    crate::transducer::address::put_u8_count(&mut v1, 1).expect("fits");
    put_bond(&mut v1, &[5; 32], &[1, 2, 3, 4]);
    crate::transducer::address::put_u8_count(&mut v1, 0).expect("fits");
    v1.extend_from_slice(&[9u8; 32]);
    v1.extend_from_slice(&7u32.to_le_bytes());
    let mut signed = v1.clone();
    signed.extend_from_slice(&key.sign(&v1));
    std::fs::write(home.join("mesh"), &signed).expect("the provisioned record");
    assert_eq!(
        std::fs::read(home.join("mesh")).expect("on disk").get(0..4),
        Some(&MAGIC_V1[..]),
        "provisioned under the old magic"
    );

    let rec = load_or_migrate_hypha(&home, &key).expect("the record opens");
    assert_eq!(rec.bind, Some("127.0.0.1:7500".into()));
    assert_eq!(rec.peers, vec!["127.0.0.1:7501".to_string()]);
    assert_eq!(rec.hypha, vec![([5; 32], [1, 2, 3, 4])]);
    assert_eq!(rec.genesis, Some([9; 32]));
    assert_eq!(rec.epoch, 7);
    let now = std::fs::read(home.join("mesh")).expect("still on disk");
    assert_eq!(now.get(0..4), Some(&MAGIC[..]), "and the boot wrote it back as MSH2");
    assert_eq!(
        MeshRecord::open_layout(&now, &key.verifying_bytes()).map(|(_, l)| l),
        Some(Layout::Current),
        "read as the current layout from here on"
    );

    // ONCE: the second boot reads MSH2 and has nothing to write.
    let again = load_or_migrate_hypha(&home, &key).expect("the record opens");
    assert_eq!(std::fs::read(home.join("mesh")).expect("on disk"), now, "byte for byte");
    assert_eq!(again.bind, rec.bind);
    let _ = std::fs::remove_dir_all(&home);
}
