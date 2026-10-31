use crate::genesis::*;
use mycelium::cognition::Court;

/// Every hypha of the genesis is bonded on this court at the cover,
/// beside the voices and the court itself; a bond already held stands.
#[test]
fn bond_genesis_bonds_every_hypha_and_voice_once() {
    let mut k = Court::new();
    let (court, peer, other, voice) = ([1u8; 32], [2u8; 32], [3u8; 32], [9u8; 32]);
    let cover = [12u64, 3, 0, 20];
    let genesis = Genesis { hypha: vec![(court, cover), (peer, cover), (other, cover)], voice: vec![(voice, [8, 8, 1, 8])] };
    bond_genesis(&mut k, court, cover, &genesis, "t");
    for who in [court, peer, other, voice] {
        assert_eq!(k.treasury().bond_of(&who), Some(cover), "{who:?} is bonded at the cover");
        assert_eq!(k.treasury().balance_mu(&who), cover, "{who:?} is funded the cover to spend");
    }
    assert!(k.treasury().audit().is_ok(), "and the allocation conserves");
    // Bonded again with another cover: what is held stands (§6).
    bond_genesis(&mut k, court, [1, 1, 1, 1], &genesis, "t");
    assert_eq!(k.treasury().bond_of(&peer), Some(cover));
    assert_eq!(k.treasury().balance_mu(&peer), cover, "and funded once, never again");
}
