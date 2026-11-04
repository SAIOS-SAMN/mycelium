// MU-DERIVATIONS D4.1, D2.7 — a peer's patience is a reading of the
// transport: its own measured round trip under its voice key, the mesh's
// measured floor where it has none, one aperture where nothing is measured,
// and a one-shot first ask read to the stream's own end.

use crate::transport::{note_rtt_ok, patience, tell_peer_within, voice_at, AskWindow, Stall};
use crate::Mesh;
use ed25519_dalek::SigningKey;
use mycelium::network::nerve::{HomeTrie, Keyed};
use mycelium::physics::{aperture_cycle, transport_deadline};
use std::sync::{Arc, Mutex};

fn mesh() -> Mesh {
    let sk = SigningKey::from_bytes(&[3; 32]);
    Mesh {
        cover: Arc::new(Mutex::new(HomeTrie::new())),
        rooms: Arc::new(Mutex::new(Keyed::new())),
        addrs: Arc::new(Mutex::new(Keyed::new())),
        vk: sk.verifying_key().to_bytes(),
        sk,
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

fn aperture() -> usize {
    aperture_cycle() as usize
}

/// A stall of exactly `n` strokes.
fn strokes(n: usize) -> Stall {
    Stall::Strokes(std::num::NonZeroUsize::new(n).expect("non-zero"))
}

/// AN UNMEASURED PEER WAITS THE MESH'S MEASURED FLOOR: windows with floors
/// 3 and 7 on two peers' keys; a third peer, never measured, is asked at an
/// aperture of 3 — the floor the rtt window holds, never a literal.
#[test]
fn tell_peer_waits_mesh_floor_for_unmeasured_peer() {
    let m = mesh();
    for s in [5, 3, 4] {
        note_rtt_ok(&m, [1u8; 32], s);
    }
    note_rtt_ok(&m, [2u8; 32], 7);
    assert_eq!(patience(&m, Some(&[9u8; 32])), strokes(3 * aperture()));
    assert_eq!(patience(&m, None), strokes(3 * aperture()));
}

/// AN EMPTY MESH WAITS ONE APERTURE: `Lambda::Unmeasured` is read live, its
/// stall `Unmeasured`, and an ask resolves it to the aperture itself, `Λ(1)`.
#[test]
fn tell_peer_empty_mesh_waits_one_aperture() {
    let m = mesh();
    assert_eq!(crate::transport::prior(&m), mycelium::physics::Lambda::Unmeasured);
    assert_eq!(patience(&m, None), Stall::Unmeasured);
    assert_eq!(patience(&m, None).resolved(), strokes(aperture()));
    assert_eq!(Stall::of(transport_deadline(1)), strokes(aperture()), "the same as one stroke's aperture");
}

/// ONE KEY PER PEER, THE VOICE: a round trip noted under a peer's voice is
/// read for its endpoint through the address book; the endpoint itself keys
/// nothing.
#[test]
fn rtt_window_keeps_one_key_per_peer() {
    let m = mesh();
    let voice = [4u8; 32];
    if let Ok(mut a) = m.addrs.lock() {
        a.insert(voice, "127.0.0.1:9".to_string());
    }
    note_rtt_ok(&m, voice, 6);
    assert_eq!(voice_at(&m, "127.0.0.1:9"), Some(voice));
    let v = voice_at(&m, "127.0.0.1:9");
    assert_eq!(patience(&m, v.as_ref().map(|v| v.as_slice())), Stall::of(transport_deadline(6)));
    assert!(m.rtt_window.lock().is_ok_and(|w| w.get("127.0.0.1:9".as_bytes()).is_none()), "no window under the endpoint");
}

/// A FIRST ASK HAS NO NUMERIC DEADLINE: a window that measured nothing reads
/// to the stream's end.
#[test]
fn first_ask_has_no_numeric_deadline() {
    assert_eq!(AskWindow::default().stall(), Stall::Beyond);
}

/// THE PAGE LOOP ASKS AT ITS MEASURED FLOOR: after asks measured 9, 3 and 7,
/// the next page waits an aperture of 3.
#[test]
fn page_loop_asks_at_measured_floor() {
    let mut w = AskWindow::default();
    for s in [9, 3, 7] {
        w.note(s);
    }
    assert_eq!(w.stall(), strokes(3 * aperture()));
}

/// D2.6 — A READ ON NO BOUND AND NO MEASUREMENT IS REFUSED: `Unmeasured`
/// begins no read; an ask resolves it to one aperture (a first ask).
#[test]
fn read_budgeted_refuses_unmeasured_stall() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let mut s = std::net::TcpStream::connect(addr).expect("connect");
    let mut buf = [0u8; 1];
    assert_eq!(crate::transport::read_budgeted(&mut s, &mut buf, Stall::Unmeasured), None);
    assert_eq!(Stall::Unmeasured.resolved(), Stall::of(transport_deadline(1)), "a first ask waits one aperture");
}

/// D2.6 — A MEASURED ROUND TRIP IS NEVER ZERO: a reply present on the first
/// poll measures the poll that read it.
#[test]
fn rtt_is_nonzero_by_measurement() {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let responder = std::thread::spawn(move || {
        let (mut s, _) = listener.accept().expect("accept");
        let mut head = [0u8; 6];
        s.read_exact(&mut head).expect("the act");
        let frame = mycelium::network::wire::WireFrame { lane: mycelium::network::wire::Lane::Spatial, body: vec![7] };
        s.write_all(&frame.encode().expect("encode")).expect("write");
    });
    let (_, rtt) = tell_peer_within(&addr, vec![0x10], Stall::Beyond).expect("answered");
    assert!(rtt.get() >= 1);
    responder.join().expect("join");
}

/// A CLI'S FIRST ASK READS TO THE STREAM'S END: a peer that takes the act
/// and closes without a word ends the read at once — silent, by the stream's
/// own end, with no poll counted.
#[test]
fn cli_first_ask_reads_to_stream_end() {
    use std::io::Read;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let closer = std::thread::spawn(move || {
        let (mut s, _) = listener.accept().expect("accept");
        let mut head = [0u8; 6];
        let _ = s.read_exact(&mut head);
    });
    assert!(tell_peer_within(&addr, vec![0x10], Stall::Beyond).is_none(), "the stream ended: silent");
    closer.join().expect("join");
}
