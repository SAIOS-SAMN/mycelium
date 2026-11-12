// MU-PRECURSORS §3.1, MU-ENGINEERING §9 — the channel a connection rides:
// Noise XX between a client and a hypha and between hyphae, each side's
// static key its host's long-term Noise key; opt-in per peer, a plaintext
// peer named and never hidden; a pinned peer held to its key and its seal,
// refused by name with no fallback; the deferral unchanged inside.

use crate::transport::{accept_with, answer, defer_unread_with, read_delimited, tell_peer_over, Stall};
use crate::Mesh;
use ed25519_dalek::SigningKey;
use mycelium::anastome::dialogue;
use mycelium::network::channel::{self, Kind, Pace, Side};
use mycelium::network::nerve::{HomeTrie, Keyed};
use mycelium::network::noise::{NoiseHome, NoiseRefusal, KEY_LEN};
use mycelium::network::wire::Delimited;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

/// What a door made of one connection: the channel, the caller's static
/// key, and the act it read; or why it read none.
type Heard = Result<(Kind, Option<[u8; KEY_LEN]>, Vec<u8>), String>;

fn home() -> (tempfile::TempDir, NoiseHome) {
    let dir = tempfile::tempdir().expect("a home");
    let h = NoiseHome::open(dir.path()).expect("the home opens");
    (dir, h)
}

/// A door at a fresh address holding `home` (none: a door that speaks only
/// plaintext): `connections` connections accepted as the hypha's readers
/// accept them, each act read and answered with `reply`.
fn door(home: Option<NoiseHome>, connections: usize, reply: Vec<u8>) -> (String, JoinHandle<Vec<Heard>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let served = std::thread::spawn(move || {
        let mut heard = Vec::new();
        for _ in 0..connections {
            let (tcp, _) = listener.accept().expect("accept");
            let Some(mut ch) = accept_with(tcp, home.as_ref(), Pace::Blocking) else {
                heard.push(Err(String::from("the channel was refused")));
                continue;
            };
            let (kind, caller) = (ch.kind(), ch.remote_static());
            match read_delimited(&mut ch) {
                Some(delimited) => {
                    answer(&mut ch, Delimited { body: reply.clone() });
                    heard.push(Ok((kind, caller, delimited.body)));
                }
                None => heard.push(Err(String::from("no envelope read"))),
            }
        }
        heard
    });
    (addr, served)
}

fn met(peer: &str, kind: Kind, side: Side) -> bool {
    channel::book().iter().any(|m| m.peer == peer && m.kind == kind && m.side == side)
}

/// A CLIENT AND A HYPHA SPEAK INSIDE NOISE: a session opted in with its
/// home dials the door, both static keys are the homes' long-term keys, the
/// hypha's is pinned under the address dialled, the act and its reply cross
/// whole, and the hypha's deferral reaches the session as a deferral.
#[test]
fn client_and_hypha_speak_inside_noise() {
    let ((_dc, client), (_dh, hypha)) = (home(), home());
    let deficit = [2u64, 0, 1, 0];
    let (addr, served) = door(Some(hypha.clone()), 1, vec![0x21, 7, 8]);
    let session = mycelium::sdk::Session::open(&addr, mycelium::sdk::Voice::from_voice([5; 32])).with_noise(client.clone());
    assert_eq!(session.speak(vec![0x10, 1, 2]).expect("answered"), vec![0x21, 7, 8]);
    let heard = served.join().expect("the door");
    assert_eq!(heard, vec![Ok((Kind::Noise, Some(client.public()), vec![0x10, 1, 2]))]);
    assert_eq!(client.pinned(&addr).expect("pins read"), Some(hypha.public()), "the hypha's key is pinned on first contact");
    assert!(met(&addr, Kind::Noise, Side::Dialled));
    assert!(met(&mycelium::court::identity::hex32(&client.public()), Kind::Noise, Side::Accepted));

    // 23.8 — the deferral inside the channel is the deferral it always was.
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let hypha_home = hypha.clone();
    let deferring = std::thread::spawn(move || {
        let (tcp, _) = listener.accept().expect("accept");
        let mut ch = accept_with(tcp, Some(&hypha_home), Pace::Blocking).expect("the channel");
        let kind = ch.kind();
        let _ = read_delimited(&mut ch);
        answer(&mut ch, Delimited { body: dialogue::encode_deferral(dialogue::PENDING_CODE, &deficit) });
        kind
    });
    let session = mycelium::sdk::Session::open(&addr, mycelium::sdk::Voice::from_voice([5; 32])).with_noise(client);
    let refusal = session.speak(vec![0x10]).expect_err("a deferral is a refusal to the caller");
    assert_eq!(mycelium::sdk::deferral_of(&refusal).map(|d| d.deficit), Some(deficit));
    assert_eq!(deferring.join().expect("the door"), Kind::Noise);
}

fn mesh() -> Mesh {
    let sk = SigningKey::from_bytes(&[4; 32]);
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

/// HYPHA PEERS SPEAK INSIDE NOISE: a court's ask to a peer is dialled with
/// its home, the handshake and the reply waited at the stall in strokes, the
/// round trip measured from the write and never zero; the peer's key is
/// pinned under its address. The listener's own deferral, written when every
/// reader is busy, rides inside the channel too.
#[test]
fn hypha_peers_speak_inside_noise() {
    let ((_da, a), (_db, b)) = (home(), home());
    let (addr, served) = door(Some(b.clone()), 1, vec![0x22, 9]);
    let (reply, rtt) = tell_peer_over(&addr, vec![0x10, 1, 2], Stall::Beyond, Some(&a)).expect("the peer answered");
    assert_eq!(reply, vec![0x22, 9]);
    assert!(rtt.get() >= 1, "a round trip takes at least the poll that read it");
    assert_eq!(served.join().expect("the door"), vec![Ok((Kind::Noise, Some(a.public()), vec![0x10, 1, 2]))]);
    assert_eq!(a.pinned(&addr).expect("pins read"), Some(b.public()));
    assert!(met(&addr, Kind::Noise, Side::Dialled));

    // Asked at a measured stall: the same channel, bounded in strokes.
    let (addr, served) = door(Some(b.clone()), 1, vec![0x23]);
    let stall = Stall::of(mycelium::physics::transport_deadline(1));
    assert_eq!(tell_peer_over(&addr, vec![0x10], stall, Some(&a)).map(|(r, _)| r), Some(vec![0x23]));
    assert!(matches!(served.join().expect("the door").as_slice(), [Ok((Kind::Noise, _, _))]));

    // 23.8 — the listener's deferral, its caller's channel opened first.
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let deficit = [0u64, 3, 0, 1];
    let m = mesh();
    let b_home = b.clone();
    let deferring = std::thread::spawn(move || {
        let (tcp, _) = listener.accept().expect("accept");
        let body = dialogue::encode_deferral(dialogue::PENDING_CODE, &deficit);
        defer_unread_with(tcp, &m, Delimited { body }, Some(&b_home));
    });
    let (reply, _) = tell_peer_over(&addr, vec![0x10], Stall::Beyond, Some(&a)).expect("the deferral");
    deferring.join().expect("the listener");
    assert!(crate::transport::is_deferred(&reply));
    assert_eq!(dialogue::deferral_lanes(&reply, dialogue::PENDING_CODE), Some(deficit));
    assert!(met(&addr, Kind::Noise, Side::Dialled));
}

/// A PINNED KEY THAT DOES NOT MATCH IS REFUSED BY NAME, with no fallback:
/// the address pinned to one key and a door there presenting another is
/// `PeerKeyChanged`, carrying both keys, and no act is spoken in plaintext
/// in its place; a pinned peer that answers in plaintext is
/// `PinnedAnsweredPlain`, and is not asked in plaintext either.
#[test]
fn pinned_key_mismatch_refused() {
    let ((_da, a), (_db, b), (_dc, c)) = (home(), home(), home());
    let (addr, served) = door(Some(c.clone()), 2, vec![0x24]);
    a.pin(&addr, &b.public()).expect("pinned to b");
    match channel::dial(&addr, Some(&a), Pace::Polled(None)) {
        Err(NoiseRefusal::PeerKeyChanged { peer, pinned, presented }) => {
            assert_eq!(peer, addr);
            assert_eq!(pinned, b.public());
            assert_eq!(presented, c.public());
        }
        other => panic!("expected PeerKeyChanged, read {:?}", other.map(|ch| ch.kind())),
    }
    assert_eq!(tell_peer_over(&addr, vec![0x10], Stall::Beyond, Some(&a)), None, "the court's ask is refused too");
    let heard = served.join().expect("the door");
    assert!(heard.iter().all(|h| h.is_err()), "the impostor door read no act, sealed or plain: {heard:?}");
    assert_eq!(a.pinned(&addr).expect("pins read"), Some(b.public()), "never re-pinned");

    // A pinned peer answering in plaintext: a door that speaks only plaintext.
    let (addr, served) = door(None, 1, vec![0x24]);
    a.pin(&addr, &b.public()).expect("pinned to b");
    match channel::dial(&addr, Some(&a), Pace::Polled(None)) {
        Err(NoiseRefusal::PinnedAnsweredPlain { peer, pinned }) => {
            assert_eq!(peer, addr);
            assert_eq!(pinned, b.public());
        }
        other => panic!("expected PinnedAnsweredPlain, read {:?}", other.map(|ch| ch.kind())),
    }
    assert_eq!(served.join().expect("the door"), vec![Err(String::from("no envelope read"))], "the opening alone, refused as a plaintext head");
    assert!(!met(&addr, Kind::Plain, Side::Dialled), "never dialled in plaintext");
}

/// A PLAINTEXT PEER IS NAMED, NOT HIDDEN, and still spoken to: a court
/// with a home asking a door that speaks only plaintext is answered in
/// plaintext on a second connection, the peer named in the book and never
/// pinned; a caller with no home is accepted in plaintext at a Noise door
/// and named there; the observer's channels reading names both and reads
/// back through its codec.
#[test]
fn plaintext_peer_named_not_hidden() {
    let ((_da, a), (_db, b)) = (home(), home());
    let (addr, served) = door(None, 2, vec![0x25]);
    let (reply, _) = tell_peer_over(&addr, vec![0x10, 4], Stall::Beyond, Some(&a)).expect("a plaintext peer is still spoken to");
    assert_eq!(reply, vec![0x25]);
    let heard = served.join().expect("the door");
    assert_eq!(heard.len(), 2);
    assert_eq!(heard[1], Ok((Kind::Plain, None, vec![0x10, 4])), "the act rode the second, plaintext connection");
    assert!(met(&addr, Kind::Plain, Side::Dialled));
    assert_eq!(a.pinned(&addr).expect("pins read"), None, "a plaintext peer is never pinned");

    let (addr, served) = door(Some(b), 1, vec![0x26]);
    assert_eq!(tell_peer_over(&addr, vec![0x10], Stall::Beyond, None).map(|(r, _)| r), Some(vec![0x26]));
    assert_eq!(served.join().expect("the door"), vec![Ok((Kind::Plain, None, vec![0x10]))]);
    assert!(met("127.0.0.1", Kind::Plain, Side::Accepted));

    let reading = mycelium::observe::Channels::now();
    let lines = reading.lines();
    assert!(lines.iter().any(|l| l.contains(&format!("PLAINTEXT dialled {addr}"))), "the plaintext peer is named: {lines:?}");
    assert!(lines.iter().any(|l| l.contains("PLAINTEXT accepted 127.0.0.1")), "{lines:?}");
    let mut out = Vec::new();
    mycelium::observe::Reading::Channels(Box::new(reading.clone())).put(&mut out).expect("the reading writes");
    let mut p = 0;
    let back = mycelium::observe::Reading::take(&out, &mut p).expect("and reads back");
    assert_eq!(back, mycelium::observe::Reading::Channels(Box::new(reading)));
    assert_eq!(p, out.len());
}

/// #235 — AN ACT AT ANOTHER WIRE VERSION IS REFUSED BY NAME AT THE DOOR,
/// before its act is read: the door reads no act, answers refused at its own
/// version, and closes; the speaker's reader hears that answer, which names
/// the door's version.
#[test]
fn door_refuses_an_act_at_another_wire_version() {
    use std::io::{Read, Write};
    let (addr, served) = door(None, 1, vec![0x01]);
    let mut act = mycelium::anastome::dialogue::Expression::of(vec![0x53, 1, 2], None).write().expect("an expression");
    let n = act.len();
    act[n - 2..].copy_from_slice(&0x0200u16.to_le_bytes());
    let mut tcp = std::net::TcpStream::connect(&addr).expect("dial");
    tcp.write_all(&Delimited { body: act }.encode().expect("an envelope")).expect("written");
    let mut reply = Vec::new();
    tcp.read_to_end(&mut reply).expect("the door answers and closes");
    let (envelope, _) = Delimited::decode(&reply).expect("one envelope");
    let heard = dialogue::Expression::heard(envelope.body).expect("the refusal is spoken at the door's version");
    let refused = mycelium::cognition::Outcome::Refused(mycelium::verdict::REFUSE_MALFORMED);
    assert_eq!(heard.act, dialogue::encode_outcome(mycelium::cognition::outcome_code(&refused), 0, &[0; 4]), "answered refused");
    assert_eq!(served.join().expect("the door"), vec![Err(String::from("no envelope read"))], "no act was read");
}
