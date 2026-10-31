use super::super::*;
use ed25519_dalek::SigningKey;

pub(super) fn key(n: u8) -> SigningKey {
    SigningKey::from_bytes(&[n; 32])
}

pub(super) fn verifying(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

pub(super) const ORIGIN: Origin = Origin { genesis: [0x6a; 32], epoch: 146 };

pub(super) fn tick(h: u64, lanes: [u64; 4], k: u64) -> Tick {
    Tick { h, lanes, k }
}

/// Three keepers, one court each, all in this process.
pub(super) struct Mesh {
    pub(super) keys: [SigningKey; 3],
    pub(super) courts: [Keepers; 3],
}

impl Mesh {
    pub(super) fn new() -> Mesh {
        let keys = [key(1), key(2), key(3)];
        let courts = [Keepers::new(ORIGIN, verifying(&keys[0])), Keepers::new(ORIGIN, verifying(&keys[1])), Keepers::new(ORIGIN, verifying(&keys[2]))];
        Mesh { keys, courts }
    }

    pub(super) fn read(&mut self, i: usize, t: Tick) -> bool {
        self.courts[i].read_own(&self.keys[i], t).expect("a reading writes")
    }

    /// Court `to` admits `from`'s ring, as a digest carries it.
    pub(super) fn hear(&mut self, to: usize, from: usize) -> Result<Admitted, KeeperRefusal> {
        let ring = self.courts[from].own_ring();
        let keeper = verifying(&self.keys[from]);
        self.courts[to].admit(keeper, ring)
    }
}
