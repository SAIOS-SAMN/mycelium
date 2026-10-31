// 47.3 — the fold's witness learns a home it holds no address for from the
// configured doors (2026-10-22): a and c missed b's digest at boot, no
// exchange ran, and every fold of b's went unmeasured until a door's digest
// taught them b.

use crate::judge::fold::learn_doors;
use crate::transport::answer;
use crate::Mesh;
use ed25519_dalek::{Signer, SigningKey};
use mycelium::anastome::dialogue::{cover_signed_prefix, encode_cover_digest};
use mycelium::network::nerve::{Cover, Keyed, Router};
use mycelium::network::wire::{Frame, Stratum};
use std::net::TcpListener;
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

/// A door that answers one ask with its own signed digest, naming itself.
fn door_answering(sk: SigningKey) -> (String, std::thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let bind = addr.clone();
    let served = std::thread::spawn(move || {
        let (mut s, _) = listener.accept().expect("accept");
        let mut ask = [0u8; 4096];
        let _ = std::io::Read::read(&mut s, &mut ask);
        let (room, roots, ruled) = ([1u64, 0, 0, 0], [[0u8; 32]; 4], [0u8; 32]);
        let sig = sk.sign(&cover_signed_prefix(&roots, &ruled, &room)).to_bytes();
        let vk = sk.verifying_key().to_bytes();
        let body = encode_cover_digest(&room, &roots, &ruled, &vk, &bind, &[], &sig).expect("a digest writes");
        answer(&mut s, Frame { lane: Stratum::Spatial, body });
    });
    (addr, served)
}

/// THE HOME IS LEARNED: a configured door the mesh holds no address for is
/// asked for its digest, and its key is held at its door after.
#[test]
fn learn_doors_learns_unknown_home_from_configured_doors() {
    let (a, b) = (SigningKey::from_bytes(&[1; 32]), SigningKey::from_bytes(&[2; 32]));
    let vb = b.verifying_key().to_bytes();
    let m = mesh(&a);
    let (door_b, served) = door_answering(b);
    assert!(m.addrs.lock().expect("addrs").get(&vb).is_none(), "b's door unknown");
    let join = vec![m.bind.clone(), door_b.clone()];
    learn_doors(&m, &join, &Mutex::new(Router::new()));
    served.join().expect("b answered");
    assert_eq!(m.addrs.lock().expect("addrs").get(&vb), Some(&door_b), "b is held at its door");
}

/// A DOOR ALREADY HELD IS NEVER ASKED: the learning reaches only the doors
/// the mesh lacks, so a witness that knows every door adds no traffic.
#[test]
fn learn_doors_skips_door_already_held() {
    let (a, b) = (SigningKey::from_bytes(&[3; 32]), SigningKey::from_bytes(&[4; 32]));
    let m = mesh(&a);
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let door_b = listener.local_addr().expect("addr").to_string();
    m.addrs.lock().expect("addrs").insert(b.verifying_key().to_bytes(), door_b.clone());
    learn_doors(&m, &[door_b], &Mutex::new(Router::new()));
    listener.set_nonblocking(true).expect("nonblocking");
    assert!(
        matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock),
        "no ask reached a door already held"
    );
}
