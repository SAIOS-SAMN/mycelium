use super::super::*;
use super::common::*;

/// 78.152 (PERSISTED) — A KEEPER RESTARTED FROM ITS RECORD never signs a
/// height at or below its last, and its next reading extends its line at a
/// peer (`Extended`, never `Anchored`). The record reads back by value; a
/// record altered, cut short, or of another origin is no floor.
#[test]
fn resume_never_signs_at_or_below_last_height() {
    let mut m = Mesh::new();
    let a = verifying(&m.keys[0]);
    let mut kept = None;
    assert_eq!(m.courts[0].read_own_kept(&m.keys[0], tick(10, [4, 0, 0, 0], 1), |r| {
        kept = Some(r.clone());
        true
    }), Some(true));
    assert_eq!(m.courts[0].read_own_kept(&m.keys[0], tick(11, [5, 0, 0, 0], 2), |r| {
        kept = Some(r.clone());
        true
    }), Some(true));
    assert_eq!(m.hear(1, 0), Ok(Admitted::Anchored));
    let rec = kept.expect("the last reading was kept");
    assert_eq!((rec.origin, rec.reading.tick.h, rec.reading.keeper), (ORIGIN, 11, a));
    // A reading the record did not keep is neither held nor published.
    assert_eq!(m.courts[0].read_own_kept(&m.keys[0], tick(12, [6, 0, 0, 0], 3), |_| false), Some(false));
    assert_eq!(m.courts[0].newest(&a).map(|r| r.tick.h), Some(11));

    // The record's bytes read back to the same values.
    let bytes = rec.encode().expect("a record writes");
    let back = LastSigned::decode(&bytes).expect("reads back");
    assert_eq!(back, rec);
    assert_eq!(back.hash, rec.reading.hash(&ORIGIN).expect("hashes"));
    let mut cut = bytes.clone();
    cut.pop();
    assert_eq!(LastSigned::decode(&cut), None, "a record cut short is none");
    let mut long = bytes.clone();
    long.push(0);
    assert_eq!(LastSigned::decode(&long), None, "a byte past the record is none");
    let mut altered = bytes.clone();
    if let Some(b) = altered.get_mut(106) {
        *b ^= 1;
    }
    assert_eq!(LastSigned::decode(&altered), None, "an altered tick does not verify");
    let mut version = bytes.clone();
    if let Some(b) = version.get_mut(4) {
        *b = 2;
    }
    assert_eq!(LastSigned::decode(&version), None, "another version is none");
    // On disk, as the home keeps it: written durably, read back by value.
    let dir = std::env::temp_dir().join(format!("mycelium-keeper-record-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a scratch home");
    let path = dir.join("keeper");
    assert_eq!(record::read(&path), Recorded::None, "a keeper that never signed here");
    assert!(record::write(&path, &rec));
    assert_eq!(record::read(&path), Recorded::Last(Box::new(rec.clone())));
    std::fs::write(&path, &cut).expect("writes");
    assert!(matches!(record::read(&path), Recorded::Unreadable(_)), "a record that does not read is refused, never none");
    let _ = std::fs::remove_dir_all(&dir);

    // THE RESTART: a fresh line, resumed from the record.
    let mut restarted = Keepers::new(ORIGIN, a);
    let elsewhere = Keepers::new(Origin { genesis: [1; 32], epoch: 146 }, a);
    assert!(!elsewhere.clone().resume(&back), "a record of another origin is no floor there");
    assert!(!Keepers::new(ORIGIN, verifying(&m.keys[1])).resume(&back), "nor another keeper's");
    assert!(restarted.resume(&back));
    // At or below the last signed height, nothing is signed — with the
    // measure moved or not.
    for h in [9, 10, 11] {
        assert_eq!(restarted.read_own(&m.keys[0], tick(h, [99, 0, 0, 0], 99)), Some(false), "h {h} is at or below 11");
    }
    // Above it, the reading names the record's reading as its parent.
    assert_eq!(restarted.read_own(&m.keys[0], tick(13, [7, 0, 0, 0], 4)), Some(true));
    let ring = restarted.own_ring();
    let newest = ring.iter().next_back().expect("a reading");
    assert_eq!((newest.tick.h, newest.parent), (13, back.hash));
    // A peer holding the line extends it; it does not re-anchor.
    assert_eq!(m.courts[1].admit(a, ring.clone()), Ok(Admitted::Extended(1)));
    assert_eq!(m.courts[1].newest(&a).map(|r| r.tick.h), Some(13));
    // A peer that missed the last reading before the restart takes it from
    // the ring and extends too: the ring carries it.
    let mut late = Keepers::new(ORIGIN, verifying(&m.keys[2]));
    let before: Vec<TickReading> = m.courts[0].own_ring().into_iter().take(1).collect();
    assert_eq!(late.admit(a, before), Ok(Admitted::Anchored));
    assert_eq!(late.admit(a, ring), Ok(Admitted::Extended(2)));
}
