// 78.153c — THE COURT'S STANDING IS SERVED ON EVERY HYPHA: published at the
// first court stroke after boot, asked or not, so the first question a
// restarted hypha hears finds its acts (b and c answered `--reading acts`
// with nothing after a ship, 2026-10-09).

use crate::Mesh;
use ed25519_dalek::SigningKey;
use mycelium::cognition::Kernel;
use mycelium::network::nerve::{Cover, Keyed};
use mycelium::observe::{self, Judged, Reading, Source};
use std::sync::{Arc, Mutex};

fn mesh(sk: &SigningKey) -> Mesh {
    Mesh {
        cover: Arc::new(Mutex::new(Cover::new())),
        rooms: Arc::new(Mutex::new(Keyed::new())),
        addrs: Arc::new(Mutex::new(Keyed::new())),
        vk: sk.verifying_key().to_bytes(),
        sk: sk.clone(),
        bind: "127.0.0.1:1".into(),
        rtt: Arc::new(Mutex::new(Keyed::new())),
        rtt_window: Arc::new(Mutex::new(Keyed::new())),
        last_roots: Arc::new(Mutex::new(Keyed::new())),
        asked_at: Arc::new(Mutex::new(Keyed::new())),
        circuits: Arc::new(Mutex::new(Keyed::new())),
        deficit: Arc::new(Mutex::new([0; 4])),
        fold_unmeasured_at: Arc::new(Mutex::new(Keyed::new())),
        origin: Arc::new(Mutex::new(None)),
    }
}

/// TWO COURTS, NEITHER ASKED: each first stroke publishes its own acts under
/// its own key — the second court's as surely as the first's — with the
/// verdicts it counted (a no-work and a door refusal) on their slots.
#[test]
fn every_court_serves_its_acts_from_its_first_stroke_unasked() {
    for seed in [0x41u8, 0x42] {
        let sk = SigningKey::from_bytes(&[seed; 32]);
        let m = mesh(&sk);
        let mut k = Kernel::new();
        k.note_judged(Judged::NoWork);
        k.note_judged(Judged::Unsigned);
        assert!(crate::judge::observe::answer_standing(&k, &m, m.vk, true), "the first stroke publishes");
        let acts = observe::readings(&Source::Court(m.vk).prefix())
            .into_iter()
            .find_map(|(_, r)| match r {
                Reading::Acts(a) => Some(a),
                _ => None,
            })
            .expect("this court's acts are served");
        let mut want = [0u64; observe::JUDGED.len()];
        want[Judged::NoWork.slot()] = 1;
        want[Judged::Unsigned.slot()] = 1;
        assert_eq!(acts.judged, want, "court {seed:02x}: its own verdicts, on their slots");
    }
}
