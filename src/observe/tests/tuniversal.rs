use super::super::*;
use super::common::*;

/// 78.152 — A COURT'S TUNIVERSAL STANDING TRAVELS EXACTLY: three keepers
/// exchange readings, the observer reads one court's keepers off them, and
/// the Court reading carrying that standing — and a hand-built residual
/// with signed lanes at the lane carrier's ends — reads back as itself.
#[test]
fn tstanding_round_trips_and_refuses_other_version() {
    use crate::chronometry::tuniversal::{Edge, Keepers, Origin, Residual, TPoint, Tick};
    use ed25519_dalek::SigningKey;
    let origin = Origin { genesis: [0x6a; 32], epoch: 146 };
    let keys = [1u8, 2, 3].map(|n| SigningKey::from_bytes(&[n; 32]));
    let vks = keys.clone().map(|k| k.verifying_key().to_bytes());
    let mut courts = vks.map(|v| Keepers::new(origin, v));
    let tick = |h, lanes, k| Tick { h, lanes, k };
    let hear = |courts: &mut [Keepers; 3], to: usize, from: usize| {
        let ring = courts[from].own_ring();
        courts[to].admit(vks[from], ring).expect("the ring admits");
    };
    assert_eq!(courts[0].read_own(&keys[0], tick(10, [4, 0, 0, 0], 1)), Some(true));
    hear(&mut courts, 1, 0);
    assert_eq!(courts[1].read_own(&keys[1], tick(12, [0, 5, 0, 0], 3)), Some(true));
    hear(&mut courts, 2, 0);
    hear(&mut courts, 2, 1);
    assert_eq!(courts[2].read_own(&keys[2], tick(14, [0, 0, 6, 0], 2)), Some(true));
    // c's view: a final at 10 on b's citation, b at 12 on c's, c cited by
    // nobody; the door's point the greatest final.
    let seen = keepers_seen(&courts[2]);
    let a10 = TPoint { origin, h: 10, lanes: [4, 0, 0, 0], k: 1 };
    let b12 = TPoint { origin, h: 12, lanes: [0, 5, 0, 0], k: 3 };
    let of = |v: [u8; 32]| seen.iter().find(|s| s.key == v).cloned().expect("the keeper is seen");
    assert_eq!(seen.len(), 3);
    assert_eq!(seen.iter().filter(|s| s.own).map(|s| s.key).collect::<Vec<_>>(), [vks[2]]);
    assert_eq!(of(vks[0]), KeeperSeen { key: vks[0], own: false, newest: Some(tick(10, [4, 0, 0, 0], 1)), final_of: Some(a10) });
    assert_eq!(of(vks[1]), KeeperSeen { key: vks[1], own: false, newest: Some(tick(12, [0, 5, 0, 0], 3)), final_of: Some(b12) });
    assert_eq!(of(vks[2]), KeeperSeen { key: vks[2], own: true, newest: Some(tick(14, [0, 0, 6, 0], 2)), final_of: None });
    assert_eq!(courts[2].final_point(), Some(b12));
    let read = TStanding { final_point: courts[2].final_point(), advances: 2, keepers: seen, residual: courts[2].residual() };
    // A residual measured on all three edges, signed, at the carrier's ends.
    let built = TStanding {
        final_point: Some(TPoint { origin, h: u64::MAX, lanes: [u64::MAX, 0, 1, 2], k: 7 }),
        advances: u64::MAX,
        keepers: vec![KeeperSeen { key: party(4), own: true, newest: None, final_of: None }],
        residual: Residual {
            edges: vec![
                Edge { from: party(4), to: party(5), h: 20, delta: [-3, 5, 0, i128::MIN] },
                Edge { from: party(5), to: party(6), h: 21, delta: [i128::MAX, -1, 2, 0] },
                Edge { from: party(4), to: party(6), h: 20, delta: [1, 1, -1, 0] },
            ],
            holonomy: Some([-4, 3, 3, -9]),
        },
    };
    for standing in [read, built, TStanding::default()] {
        let court = Reading::Court(Box::new(Court { k: 5, tuniversal: standing.clone(), ..Default::default() }));
        let mut wire = Vec::new();
        court.put(&mut wire).expect("the court writes");
        let mut at = 0;
        assert_eq!(Reading::take(&wire, &mut at), Some(court.clone()));
        assert_eq!(at, wire.len(), "read whole");
        // Another version of the standing is refused, never misread.
        let mut alone = Vec::new();
        standing.put(&mut alone).expect("the standing writes");
        let mut at = 0;
        assert_eq!(TStanding::take(&alone, &mut at), Some(standing));
        alone[0] = TSTANDING_VERSION + 1;
        let mut at = 0;
        assert_eq!(TStanding::take(&alone, &mut at), None);
    }
}
