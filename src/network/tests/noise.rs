use crate::chronometry::tuniversal::TPoint;
use crate::network::noise::*;
use crate::network::wire::{Delimited, DELIMITED_MAX, HEAD};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// A stream that records what it reads and can flip one byte of it: the
/// wire between two hosts, as a host in the middle sees it.
struct Tap<S> {
    inner: S,
    seen: Vec<u8>,
    flip_in: Option<usize>,
}

impl<S> Tap<S> {
    fn new(inner: S) -> Self {
        Tap { inner, seen: Vec::new(), flip_in: None }
    }
}

impl<S: Read> Read for Tap<S> {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(out)?;
        for b in out[..n].iter_mut() {
            match self.flip_in {
                Some(0) => {
                    *b ^= 0x01;
                    self.flip_in = None;
                }
                Some(k) => self.flip_in = Some(k - 1),
                None => {}
            }
        }
        self.seen.extend_from_slice(&out[..n]);
        Ok(n)
    }
}

impl<S: Write> Write for Tap<S> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

fn loopback() -> (TcpStream, TcpStream) {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let a = TcpStream::connect(l.local_addr().unwrap()).unwrap();
    let (b, _) = l.accept().unwrap();
    (a, b)
}

fn home() -> (tempfile::TempDir, NoiseHome) {
    let dir = tempfile::tempdir().unwrap();
    let h = NoiseHome::open(dir.path()).unwrap();
    (dir, h)
}

/// A handshake between `a` (initiating, pinning under `peer`) and `b`
/// (responding over `wrap` of its stream), both halves returned.
fn handshake<W, F>(a: &NoiseHome, b: &NoiseHome, peer: &str, wrap: F) -> (Result<NoiseStream<TcpStream>, NoiseRefusal>, Result<NoiseStream<W>, NoiseRefusal>)
where
    W: Read + Write + Send + 'static,
    F: FnOnce(TcpStream) -> W + Send + 'static,
{
    let (sa, sb) = loopback();
    let b = b.clone();
    let responder = std::thread::spawn(move || respond(wrap(sb), &b, None));
    let ia = initiate(sa, a, peer);
    let rb = responder.join().unwrap();
    (ia, rb)
}

/// The parsed `[len:2][message]` lengths of bytes off the stream.
fn message_lengths(mut seen: &[u8]) -> Vec<usize> {
    let mut out = Vec::new();
    while seen.len() >= 2 {
        let len = usize::from(u16::from_be_bytes([seen[0], seen[1]]));
        out.push(len);
        seen = &seen[2 + len..];
    }
    assert!(seen.is_empty(), "the stream is whole messages");
    out
}

/// XX completes between two homes over loopback TCP: each side holds the
/// other's public key, and an envelope carrying an expression with its point,
/// and its reply, read back exactly.
#[test]
fn initiate_completes_handshake_and_round_trips_envelopes() {
    let ((_da, a), (_db, b)) = (home(), home());
    let (ia, rb) = handshake(&a, &b, "127.0.0.1:7500", |s| s);
    let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
    assert_eq!(ia.remote_static(), b.public());
    assert_eq!(rb.remote_static(), a.public());
    assert_ne!(a.public(), b.public());

    use crate::chronometry::tuniversal::Origin;
    let point = TPoint { origin: Origin { genesis: [7; 32], epoch: 146 }, h: 40_961, lanes: [3, 0, 9, 1].into(), k: 812 };
    let said = crate::anastome::dialogue::Expression::of(vec![0x53, 1, 2, 3], Some(point));
    let ask = Delimited { body: said.write().unwrap() };
    ia.write_delimited(&ask).unwrap();
    let heard = rb.read_delimited().unwrap();
    assert_eq!(heard, ask);
    assert_eq!(crate::anastome::dialogue::Expression::read(heard.body), Some(said), "the point rides inside the expression");
    let reply = Delimited { body: vec![9; 40] };
    rb.write_delimited(&reply).unwrap();
    assert_eq!(ia.read_delimited().unwrap(), reply);
}

/// An envelope past one Noise message rides as ⌈bytes / CHUNK⌉ messages,
/// none over the ceiling, and joins back to the same envelope.
#[test]
fn write_delimited_chunks_past_one_message_and_rejoins() {
    let ((_da, a), (_db, b)) = (home(), home());
    let (ia, rb) = handshake(&a, &b, "peer", Tap::new);
    let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
    rb.get_mut().seen.clear();
    let body: Vec<u8> = (0..3 * CHUNK + 7).map(|i| u8::try_from(i % 251).unwrap()).collect();
    let delimited = Delimited { body };
    let sender = std::thread::spawn(move || {
        ia.write_delimited(&delimited).unwrap();
        delimited
    });
    let got = rb.read_delimited().unwrap();
    let delimited = sender.join().unwrap();
    assert_eq!(got, delimited.clone());
    let lens = message_lengths(&rb.get_ref().seen);
    let plain = HEAD + delimited.body.len();
    assert_eq!(lens.len(), plain.div_ceil(CHUNK));
    assert!(lens.iter().all(|&l| l <= MESSAGE_MAX));
    assert_eq!(lens.iter().map(|l| l - TAG_LEN).sum::<usize>(), plain);
}

/// A body over the wire's budget is refused before a byte is sent: the
/// next envelope is the next thing the peer reads.
#[test]
fn write_delimited_refuses_over_budget_before_sending() {
    let ((_da, a), (_db, b)) = (home(), home());
    let (ia, rb) = handshake(&a, &b, "peer", |s| s);
    let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
    let over = Delimited { body: vec![0; DELIMITED_MAX + 1] };
    assert!(matches!(ia.write_delimited(&over), Err(NoiseRefusal::OverBudget { len }) if len == DELIMITED_MAX + 1));
    let next = Delimited { body: vec![4, 2] };
    ia.write_delimited(&next).unwrap();
    assert_eq!(rb.read_delimited().unwrap(), next);
}

/// A head that names a body past the budget is refused by its length,
/// the body never allocated.
#[test]
fn read_delimited_refuses_head_past_budget() {
    let ((_da, a), (_db, b)) = (home(), home());
    let (ia, rb) = handshake(&a, &b, "peer", |s| s);
    let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
    let len = DELIMITED_MAX + 1;
    ia.write_all(&u32::try_from(len).unwrap().to_le_bytes()).unwrap();
    assert!(matches!(rb.read_delimited(), Err(NoiseRefusal::OverBudget { len: l }) if l == len));
}

/// One flipped bit of a transport message's ciphertext fails its tag.
#[test]
fn read_delimited_refuses_tampered_ciphertext() {
    let ((_da, a), (_db, b)) = (home(), home());
    let (ia, rb) = handshake(&a, &b, "peer", Tap::new);
    let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
    // Past the two-byte length, the first ciphertext byte.
    rb.get_mut().flip_in = Some(2);
    ia.write_delimited(&Delimited { body: vec![1, 2, 3] }).unwrap();
    assert!(matches!(rb.read_delimited(), Err(NoiseRefusal::Decrypt)));
}

/// A peer pinned on first contact passes again with its key; the same
/// address presenting another key is refused by name, before the
/// initiator's static key is sent, and the pin survives a reload.
#[test]
fn pin_refuses_changed_peer_key() {
    let ((da, a), (_db, b), (_dc, c)) = (home(), home(), home());
    let (ia, rb) = handshake(&a, &b, "10.0.0.2:7500", |s| s);
    assert!(ia.is_ok() && rb.is_ok());
    assert_eq!(a.pinned("10.0.0.2:7500").unwrap(), Some(b.public()));
    let (ia, rb) = handshake(&a, &b, "10.0.0.2:7500", |s| s);
    assert!(ia.is_ok() && rb.is_ok(), "the pinned key passes again");

    let (ia, rc) = handshake(&a, &c, "10.0.0.2:7500", |s| s);
    match ia {
        Err(NoiseRefusal::PeerKeyChanged { peer, pinned, presented }) => {
            assert_eq!(peer, "10.0.0.2:7500");
            assert_eq!(pinned, b.public());
            assert_eq!(presented, c.public());
        }
        other => panic!("expected PeerKeyChanged, read {other:?}", other = other.map(|s| s.remote_static())),
    }
    assert!(rc.is_err(), "the impostor never reaches transport");
    assert_eq!(NoiseHome::open(da.path()).unwrap().pinned("10.0.0.2:7500").unwrap(), Some(b.public()));
    assert!(matches!(a.pin("a\tb", &c.public()), Err(NoiseRefusal::PeerLabel(_))));
}

/// THE OPENING READS AS NO PLAINTEXT ENVELOPE: its head, read as a plaintext
/// length, passes the wire's `DELIMITED_MAX`, so a door that speaks only
/// plaintext refuses it by length and reads no body; a responder opened
/// with anything else answers `NotSpoken`.
#[test]
fn opening_reads_as_no_plaintext_envelope() {
    let len = usize::try_from(u32::from_le_bytes([OPEN[0], OPEN[1], OPEN[2], OPEN[3]])).unwrap();
    assert!(len > DELIMITED_MAX);
    assert!(len > crate::network::wire::BODY_BUDGET + TPoint::TRAILER_MAX);
    assert!(Delimited::decode(&OPEN).is_none());

    let (_db, b) = home();
    let (mut sa, sb) = loopback();
    let responder = std::thread::spawn(move || respond(sb, &b, None).map(|s| s.remote_static()));
    let delimited = Delimited { body: vec![1, 2, 3, 4, 5] };
    sa.write_all(&delimited.encode().unwrap()).unwrap();
    assert!(matches!(responder.join().unwrap(), Err(NoiseRefusal::NotSpoken { answered }) if answered[..HEAD] == 5u32.to_le_bytes()));
}

/// The static key is minted once, `0600`, and reloads to the same key; a
/// key file of another length is refused and left untouched.
#[test]
fn noise_home_keeps_key_across_open() {
    use std::os::unix::fs::PermissionsExt;
    let (dir, a) = home();
    let again = NoiseHome::open(dir.path()).unwrap();
    assert_eq!(a.public(), again.public());
    let meta = std::fs::metadata(a.key_path()).unwrap();
    assert_eq!(meta.len(), u64::try_from(KEY_LEN).unwrap());
    assert_eq!(meta.permissions().mode() & 0o777, 0o600);

    let bad = tempfile::tempdir().unwrap();
    std::fs::write(bad.path().join(KEY_FILE), [1u8; KEY_LEN - 1]).unwrap();
    assert!(matches!(NoiseHome::open(bad.path()), Err(NoiseRefusal::KeyFileMalformed(_))));
    assert_eq!(std::fs::read(bad.path().join(KEY_FILE)).unwrap(), vec![1u8; KEY_LEN - 1]);
}

/// D1.3 O2 (78.182 item 19) — PLAINTEXT STAYS ON THE HOST: a loopback peer,
/// or a peer at the socket's own address, stands on this host; any other
/// address is across the host's boundary, where only Noise is spoken.
#[test]
fn plaintext_peer_stands_on_this_host_only() {
    use crate::network::channel::on_this_host;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let own = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 7));
    assert!(on_this_host(IpAddr::V4(Ipv4Addr::LOCALHOST), own), "loopback, v4");
    assert!(on_this_host(IpAddr::V6(Ipv6Addr::LOCALHOST), own), "loopback, v6");
    assert!(on_this_host(own, own), "this host reached by its own address");
    assert!(!on_this_host(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 8)), own), "a neighbour is across the boundary");
    assert!(!on_this_host(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 5)), own), "a remote host is across the boundary");
}

/// A plaintext dial and accept over loopback stand: the one-host path the
/// rule keeps (D1.3 O2).
#[test]
fn plaintext_over_loopback_is_spoken() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let at = listener.local_addr().expect("addr").to_string();
    let door = std::thread::spawn(move || {
        let (tcp, _) = listener.accept().expect("accept");
        let mut paced = crate::network::channel::Paced::new(tcp, crate::network::channel::Pace::Blocking);
        let _ = paced.take_head::<{ HEAD }>();
        drop(paced);
    });
    let dialled = crate::network::channel::dial(&at, None, crate::network::channel::Pace::Blocking);
    assert!(matches!(dialled, Ok(crate::network::channel::Channel::Plain(_))), "a loopback peer is dialled in plaintext");
    drop(dialled);
    door.join().expect("the door ends");
}
