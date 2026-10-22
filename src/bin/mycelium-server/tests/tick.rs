// 78.152b — the keeper's clock is the mesh's, never a court's load: a
// reading is signed in the court loop with no digest asked, and a peer's ring
// pushed as ACT_TICK is heard at the door with no kernel.

use crate::tshelf::{hear_tick, sign_tick, TickRefused};
use crate::Mesh;
use ed25519_dalek::SigningKey;
use mycelium::anastome::dialogue::encode_tick;
use mycelium::chronometry::tuniversal::{Admitted, Keepers, Origin, TPoint, Tick};
use mycelium::network::nerve::{Cover, Keyed};
use std::sync::{Arc, Mutex};

const GENESIS: [u8; 32] = [0x6b; 32];
const EPOCH: u32 = 152;

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
        origin: Arc::new(Mutex::new(Some((GENESIS, EPOCH)))),
    }
}

/// ONE TEST, for the keepers are this process's own (`tshelf`'s statics):
/// the court loop's signing needs no digest, and the door hears a known
/// peer's pushed ring, refuses a stranger's, and takes no kernel doing so.
#[test]
fn a_reading_is_signed_with_no_digest_and_a_pushed_ring_is_heard_off_the_kernel() {
    let (a, b) = (SigningKey::from_bytes(&[1; 32]), SigningKey::from_bytes(&[2; 32]));
    let (va, vb) = (a.verifying_key().to_bytes(), b.verifying_key().to_bytes());
    let m = mesh(&a);
    let origin = Origin { genesis: GENESIS, epoch: EPOCH };
    // Read on this thread: the signing and the door run here, and the tests
    // beside this one take kernels of their own on theirs.
    let holds = crate::held::holds_here();

    // THE COURT LOOP SIGNS: a tick read, a reading due, no digest built.
    assert_eq!(sign_tick(&m, None), None, "no tick read, nothing signed");
    let t10 = Tick { h: 10, lanes: [4, 0, 0, 0], k: 1 };
    let ring = sign_tick(&m, Some(t10)).expect("a reading is due");
    let [first] = ring.as_slice() else { panic!("one reading: {}", ring.len()) };
    assert_eq!((first.keeper, first.tick), (va, t10));
    assert!(first.verifies(&origin), "signed by this court at its origin");
    assert_eq!(sign_tick(&m, Some(t10)), None, "the same tick is not due again");
    assert_eq!(sign_tick(&m, Some(Tick { h: 11, ..t10 })), None, "nor a tick whose measure stood");

    // A PEER'S RING, PUSHED: b heard a's reading and cites it.
    let mut kb = Keepers::new(origin, vb);
    assert_eq!(kb.admit(va, ring), Ok(Admitted::Anchored));
    assert_eq!(kb.read_own(&b, Tick { h: 12, lanes: [0, 5, 0, 0], k: 3 }), Some(true));
    let body = encode_tick(&vb, &kb.own_ring()).expect("a tick writes");

    assert_eq!(hear_tick(&m, &body), Err(TickRefused::Stranger), "a keeper no signed digest named");
    m.addrs.lock().expect("addrs").insert(vb, "127.0.0.1:2".to_string());
    assert_eq!(hear_tick(&m, &body), Ok(Admitted::Anchored), "a known peer court's ring is heard");
    assert_eq!(hear_tick(&m, &body), Ok(Admitted::Stale), "and heard once");
    assert_eq!(
        crate::tshelf::final_point(),
        Some(TPoint { origin, h: 10, lanes: [4, 0, 0, 0], k: 1 }),
        "a's 10 is final: a signed it, b cites it"
    );

    // This court's own ring, and a frame that does not read, are refused.
    let own = encode_tick(&va, &[]).expect("writes");
    assert_eq!(hear_tick(&m, &own), Err(TickRefused::Stranger));
    let mut long = body.clone();
    long.push(0);
    assert_eq!(hear_tick(&m, &long), Err(TickRefused::Unreadable));

    assert_eq!(crate::held::holds_here(), holds, "no kernel was taken: not by the signing, not by the door");
}
