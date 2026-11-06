// MU-PRECURSORS §3.1 — THE REFERENCE MIND INSIDE NOISE: its sessions to the
// hyphae and its chatter with other minds ride Noise XX on its home's static
// key, each peer pinned on first contact; a changed key, or a pinned peer
// answering in plaintext, refused by name with no fallback.

use crate::client::sdk::chatter::{self, Chatter, Said, OPENING_REFUSED};
use crate::client::sdk::{Mind, Voice};
use crate::network::channel::{self, Channel, Kind, Pace, Side};
use crate::network::noise::{NoiseHome, OPEN};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::{channel as mpsc, Receiver};
use std::sync::{Arc, Mutex};

fn home() -> (tempfile::TempDir, NoiseHome) {
    let dir = tempfile::tempdir().expect("a home");
    let h = NoiseHome::open(dir.path()).expect("the home opens");
    (dir, h)
}

fn key(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

/// What a tap read of one connection: the caller's bytes and the door's.
#[derive(Clone, Default)]
struct Crossed {
    up: Vec<u8>,
    down: Vec<u8>,
}

type Seen = Arc<Mutex<Vec<Crossed>>>;

/// One direction of a tapped connection, every byte recorded before it is
/// carried on.
fn carry(mut from: TcpStream, mut to: TcpStream, seen: Seen, at: usize, up: bool) {
    let mut b = vec![0u8; 4096];
    loop {
        match from.read(&mut b) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                if let Ok(mut s) = seen.lock() {
                    let r = &mut s[at];
                    let side = if up { &mut r.up } else { &mut r.down };
                    side.extend_from_slice(&b[..n]);
                }
                if to.write_all(&b[..n]).is_err() {
                    break;
                }
            }
        }
    }
    let _ = to.shutdown(Shutdown::Write);
}

/// A TAP: a host in the middle. Every connection made to its address is
/// carried to `to`, and both directions recorded per connection: the wire
/// as an unkeyed party reads it.
fn tap(to: String) -> (String, Seen) {
    let l = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = l.local_addr().expect("addr").to_string();
    let seen: Seen = Arc::new(Mutex::new(Vec::new()));
    let held = Arc::clone(&seen);
    std::thread::spawn(move || {
        for c in l.incoming() {
            let Ok(caller) = c else { continue };
            let Ok(door) = TcpStream::connect(&to) else { continue };
            let at = match held.lock() {
                Ok(mut s) => {
                    s.push(Crossed::default());
                    s.len() - 1
                }
                Err(_) => continue,
            };
            let (Ok(caller_r), Ok(door_r)) = (caller.try_clone(), door.try_clone()) else { continue };
            let (a, b) = (Arc::clone(&held), Arc::clone(&held));
            std::thread::spawn(move || carry(caller_r, door, a, at, true));
            std::thread::spawn(move || carry(door_r, caller, b, at, false));
        }
    });
    (addr, seen)
}

fn seen(s: &Seen) -> Vec<Crossed> {
    s.lock().expect("the tap").clone()
}

/// Whether `plain` stands anywhere in `wire`.
fn carries(wire: &[u8], plain: &[u8]) -> bool {
    !plain.is_empty() && wire.windows(plain.len()).any(|w| w == plain)
}

/// A HYPHA'S DOOR holding `home`: each connection opened as the channel
/// opens it, its first frame read and handed back with the channel's kind;
/// the connection is then let go.
fn door(home: NoiseHome) -> (String, Receiver<(Kind, Vec<u8>)>) {
    let l = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = l.local_addr().expect("addr").to_string();
    let (to, heard) = mpsc();
    std::thread::spawn(move || {
        for c in l.incoming() {
            let Ok(tcp) = c else { continue };
            let Ok(ch) = channel::accept(tcp, Some(&home), Pace::Blocking) else { continue };
            let read = match ch {
                Channel::Noise(mut n) => n.read_delimited().map(|(f, _)| (Kind::Noise, f.body)).ok(),
                Channel::Plain(_) => Some((Kind::Plain, Vec::new())),
            };
            if let Some(r) = read {
                let _ = to.send(r);
            }
        }
    });
    (addr, heard)
}

fn hex(b: &[u8; 32]) -> String {
    crate::court::identity::hex32(b)
}

/// THE MIND SPEAKS INSIDE NOISE. A mind opened with its home
/// (`Mind::open_with_noise`) dials its hypha with the opening and the XX
/// handshake: a host in the middle reads the opening and then ciphertext,
/// never the act, while the keyed door reconstructs the act whole (the
/// peers request under the mind's voice) and the hypha's key stands pinned
/// under the address dialled. The chatter the mind opens listens inside
/// Noise on the same key: a peer mind's question crosses as ciphertext, is
/// answered, and the mind's key stands pinned under its voice; a frame sent
/// to a sealed post crosses as ciphertext and is delivered whole, its
/// signature verifying.
#[test]
fn mind_speaks_inside_noise() {
    let ((_dm, mine), (_dh, hypha), (_dp, peer)) = (home(), home(), home());
    let (at, heard) = door(hypha.clone());
    let (wire, tapped) = tap(at);
    let voice = Voice::from_key(key(9));
    let id = voice.id();
    let mut mind = Mind::open_with_noise(&wire, voice, mine.clone());
    let (kind, act) = heard.recv().expect("the door read an act");
    assert_eq!(kind, Kind::Noise);
    assert_eq!(crate::anastome::dialogue::decode_peers_request(&act), Some(id), "the keyed door reconstructs the act whole");
    for c in seen(&tapped) {
        assert!(c.up.iter().zip(OPEN.iter()).all(|(a, b)| a == b), "every session opens with the channel's opening");
        assert!(!carries(&c.up, &act) && !carries(&c.up, &id), "the act never crosses in plaintext");
    }
    assert_eq!(mine.pinned(&wire).expect("pins read"), Some(hypha.public()), "the hypha pinned under the address dialled");

    // The mind's chatter, on the same key.
    let rendezvous = tempfile::tempdir().expect("a rendezvous");
    let post = mind.open_chatter(rendezvous.path()).expect("the post opens");
    let (wire, tapped) = tap(post);
    let prefix = [0x5au8; 64];
    let question = crate::observe::encode_query(&prefix).expect("a query");
    let reply = chatter::ask_over(&wire, &hex(&id), &question, Some(&peer)).expect("the mind answered");
    assert!(crate::observe::decode_reply(&reply).is_some(), "the keyed asker reconstructs the answer");
    let c = seen(&tapped).pop().expect("one connection");
    assert!(c.up.starts_with(&OPEN) && c.down.starts_with(&OPEN));
    assert!(!carries(&c.up, &prefix), "the question never crosses in plaintext");
    assert!(!carries(&c.down, &reply), "the answer never crosses in plaintext");
    assert_eq!(peer.pinned(&hex(&id)).expect("pins read"), Some(mine.public()), "the mind pinned under its voice");
    assert!(channel::book().iter().any(|m| m.peer == hex(&id) && m.kind == Kind::Noise && m.side == Side::Dialled));

    // What a mind's carrier sends (`send_over`), delivered whole by a sealed post.
    let (to, delivered) = mpsc::<Vec<u8>>();
    let sealed = chatter::Post::listen_over("127.0.0.1:0", Some(mine.clone()), move |f| {
        let _ = to.send(f);
    }, |_: &[u8]| None)
    .expect("listen");
    let (wire, tapped) = tap(sealed.endpoint().to_string());
    let k = key(3);
    let said = Chatter { speaker: k.verifying_key().to_bytes(), point: None, said: Said::Say { said: vec![9; 100], words: vec![], evidence: None } };
    let frame = said.sign(&k).expect("signed");
    chatter::send_over(&wire, &wire, &frame, Some(&peer)).expect("sent");
    let got = delivered.recv().expect("delivered");
    assert_eq!(Chatter::verify(&got), Some(said), "the keyed post reconstructs the signed frame");
    assert!(!carries(&seen(&tapped).pop().expect("one connection").up, &frame), "the frame never crosses in plaintext");
}

/// A CHANGED OR UNSEALED PEER IS REFUSED BY NAME, with no fallback: a mind
/// pinned to one key and a post at that name presenting another is
/// `PeerKeyChanged` and nothing is delivered, sealed or plain; a pinned mind
/// whose post answers in plaintext is `PinnedAnsweredPlain` and is never
/// sent to in plaintext; a mind's session to a hypha pinned to another key
/// is refused the same way. Neither pin is moved.
#[test]
fn mind_refuses_a_changed_or_unsealed_peer() {
    let ((_da, a), (_db, b), (_dc, c)) = (home(), home(), home());
    let (to, delivered) = mpsc::<Vec<u8>>();
    let impostor = chatter::Post::listen_over("127.0.0.1:0", Some(c.clone()), move |f| {
        let _ = to.send(f);
    }, |_: &[u8]| None)
    .expect("listen");
    let name = hex(&[7; 32]);
    a.pin(&name, &b.public()).expect("pinned to b");
    let refused = chatter::send_over(impostor.endpoint(), &name, b"never", Some(&a)).expect_err("a changed key is refused");
    assert_eq!(refused.kind(), std::io::ErrorKind::PermissionDenied, "a refusal, never a mind gone");
    assert!(refused.to_string().contains("changed static key"), "named: {refused}");
    let plain = chatter::Post::listen("127.0.0.1:0", |_| {}, |_: &[u8]| None).expect("listen");
    let refused = chatter::send_over(plain.endpoint(), &name, b"never", Some(&a)).expect_err("a pinned peer answering in plaintext is refused");
    assert!(refused.to_string().contains("answered in plaintext"), "named: {refused}");
    assert!(!channel::book().iter().any(|m| m.peer == name && m.kind == Kind::Plain), "never dialled in plaintext");
    assert_eq!(a.pinned(&name).expect("pins read"), Some(b.public()), "never re-pinned");
    drop(impostor);
    assert!(delivered.try_recv().is_err(), "nothing reached the impostor's post");

    // The mind's session: its hypha pinned to b, a door there presenting c.
    let (at, _heard) = door(c.clone());
    a.pin(&at, &b.public()).expect("pinned to b");
    let session = crate::client::sdk::Session::open(&at, Voice::from_key(key(4))).with_noise(a.clone());
    let refused = session.speak(vec![0x10]).expect_err("the session is refused");
    assert!(refused.to_string().contains("changed static key"), "named: {refused}");
    assert_eq!(a.pinned(&at).expect("pins read"), Some(b.public()), "never re-pinned");
}

/// THE POST TELLS THE OPENING FROM A PLAINTEXT FRAME by all five bytes: a
/// plaintext frame whose length begins with the opening's first byte (255
/// bytes) is read whole at a sealed post, its caller named plaintext; a
/// post with no home refuses the opening in plaintext, and a mind with a
/// home names that post plaintext, never pins it, and delivers in
/// plaintext.
#[test]
fn post_tells_the_opening_from_a_plaintext_frame() {
    let ((_da, a), (_db, b)) = (home(), home());
    let (to, delivered) = mpsc::<Vec<u8>>();
    let sealed = chatter::Post::listen_over("127.0.0.1:0", Some(b.clone()), move |f| {
        let _ = to.send(f);
    }, |_: &[u8]| None)
    .expect("listen");
    let frame = vec![0x2au8; 255];
    assert_eq!(255usize.to_le_bytes()[0], OPEN[0], "the length's first byte is the opening's");
    chatter::send(sealed.endpoint(), &frame).expect("sent in plaintext");
    assert_eq!(delivered.recv().expect("delivered"), frame, "read whole, the bytes read to tell it apart included");

    let (to, delivered) = mpsc::<Vec<u8>>();
    let plain = chatter::Post::listen("127.0.0.1:0", move |f| {
        let _ = to.send(f);
    }, |_: &[u8]| None)
    .expect("listen");
    let mut s = TcpStream::connect(plain.endpoint()).expect("connect");
    s.write_all(&OPEN).expect("the opening");
    let mut answered = [0u8; 5];
    s.read_exact(&mut answered).expect("an answer");
    assert_eq!(answered, OPENING_REFUSED, "the opening refused in plaintext");
    let name = hex(&[8; 32]);
    chatter::send_over(plain.endpoint(), &name, &frame, Some(&a)).expect("delivered in plaintext");
    assert_eq!(delivered.recv().expect("delivered"), frame);
    assert_eq!(a.pinned(&name).expect("pins read"), None, "a plaintext peer is never pinned");
    assert!(channel::book().iter().any(|m| m.peer == name && m.kind == Kind::Plain && m.side == Side::Dialled), "named plaintext");
}
