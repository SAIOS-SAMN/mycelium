// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ENCRYPTED CHANNEL: Noise_XX over a byte stream (a TCP connection on
//! the mesh), carrying the wire's frames (`wire::Frame`) inside Noise
//! transport messages.
//!
//! No certificate authority stands behind it. Each host keeps one static
//! Noise key (`noise.key` in its home) and pins each peer's static key on
//! first contact (`noise.pins`, keyed by the peer's address): a pinned peer
//! that presents a different key is refused by name (`PeerKeyChanged`).
//!
//! The channel only encrypts and authenticates the BYTES between two hosts.
//! Identity stays in the Ed25519-signed acts the frames carry; the Noise
//! static key is a Curve25519 key of the channel and is never an identity,
//! never derived from one and never read as one.
//!
//! On the stream every Noise message is `[len:2 BE][message]`, a message at
//! most `MESSAGE_MAX` bytes. A frame's encoding is cut into plaintext chunks
//! of at most `CHUNK` bytes, one transport message each; the reader joins
//! them back. A frame over the wire's budget is refused exactly as the
//! plaintext wire refuses it: before a byte is sent, and on read before its
//! body is allocated.
//!
//! The stream under the channel is BLOCKING: a `WouldBlock` in the middle of
//! a message would lose the bytes already read of it.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use super::wire::{read_lane, split_point, Frame, BODY_BUDGET};
use crate::chronometry::tuniversal::TPoint;

/// The one handshake pattern and cipher suite of the channel.
pub const PATTERN: &str = "Noise_XX_25519_ChaChaPoly_BLAKE2s";
/// Noise's ceiling on one message, tag included.
pub const MESSAGE_MAX: usize = 65535;
/// The AEAD tag every transport message carries.
pub const TAG_LEN: usize = 16;
/// The plaintext one transport message carries at most.
pub const CHUNK: usize = MESSAGE_MAX - TAG_LEN;
/// The host's static Noise private key, under its home directory.
pub const KEY_FILE: &str = "noise.key";
/// The host's pins: one `peer<TAB>hex(static key)` line per peer address.
pub const PIN_FILE: &str = "noise.pins";
/// A Curve25519 key's length.
pub const KEY_LEN: usize = 32;

/// Why the channel refused, by name.
#[derive(Debug, thiserror::Error)]
pub enum NoiseRefusal {
    /// The stream under the channel failed.
    #[error("noise channel i/o: {0}")]
    Io(std::io::Error),
    /// The peer closed the stream at or inside a message.
    #[error("noise channel closed by the peer")]
    Closed,
    /// The XX handshake did not complete: a message failed to build or to
    /// authenticate, or the pattern itself would not resolve.
    #[error("noise handshake failed")]
    HandshakeFailed,
    /// The handshake ended without the peer's static key of `KEY_LEN` bytes.
    #[error("noise peer presented no static key")]
    NoPeerStatic,
    /// A pinned peer presented a static key other than the one pinned.
    #[error("noise peer {peer} presented a changed static key")]
    PeerKeyChanged { peer: String, pinned: [u8; KEY_LEN], presented: [u8; KEY_LEN] },
    /// A peer address the pin store cannot hold as one line.
    #[error("noise peer address {0:?} cannot be pinned")]
    PeerLabelMalformed(String),
    /// A Noise message past `MESSAGE_MAX`.
    #[error("noise message of {len} bytes over the {MESSAGE_MAX}-byte ceiling")]
    MessageTooLarge { len: usize },
    /// A frame whose body is past the wire's budget.
    #[error("frame of {len} bytes over the wire budget")]
    FrameOverBudget { len: usize },
    /// A decrypted frame whose head names no lane, or a pointed body shorter
    /// than its point.
    #[error("noise channel carried a malformed frame")]
    FrameMalformed,
    /// A transport message that failed to authenticate: tampered, replayed,
    /// reordered or cut.
    #[error("noise transport message failed to authenticate")]
    Decrypt,
    /// A transport message that would not seal.
    #[error("noise transport message failed to seal")]
    EncryptFailed,
    /// A key file that holds anything other than one `KEY_LEN`-byte key.
    #[error("noise key file {0} is not a {KEY_LEN}-byte key")]
    KeyFileMalformed(PathBuf),
    /// A pin line for the peer whose key does not read as `KEY_LEN` hex bytes.
    #[error("noise pin store {0} holds a malformed line")]
    PinStoreMalformed(PathBuf),
}

impl From<std::io::Error> for NoiseRefusal {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::UnexpectedEof => NoiseRefusal::Closed,
            _ => NoiseRefusal::Io(e),
        }
    }
}

fn builder() -> Result<snow::Builder<'static>, NoiseRefusal> {
    let params = PATTERN.parse().map_err(|_| NoiseRefusal::HandshakeFailed)?;
    Ok(snow::Builder::new(params))
}

/// The Curve25519 public key of a private key, by the resolver the
/// handshake itself uses: the key a peer sees is the key this reports.
fn public_of(private: &[u8; KEY_LEN]) -> Result<[u8; KEY_LEN], NoiseRefusal> {
    use snow::resolvers::CryptoResolver;
    let mut dh = snow::resolvers::DefaultResolver
        .resolve_dh(&snow::params::DHChoice::Curve25519)
        .ok_or(NoiseRefusal::HandshakeFailed)?;
    dh.set(private);
    <[u8; KEY_LEN]>::try_from(dh.pubkey()).map_err(|_| NoiseRefusal::HandshakeFailed)
}

/// A HOST'S NOISE HOME: its static key and its pins, under one directory.
#[derive(Clone)]
pub struct NoiseHome {
    dir: PathBuf,
    private: [u8; KEY_LEN],
    public: [u8; KEY_LEN],
}

impl std::fmt::Debug for NoiseHome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoiseHome")
            .field("dir", &self.dir)
            .field("public", &crate::court::identity::hex32(&self.public))
            .finish_non_exhaustive()
    }
}

impl NoiseHome {
    /// The home at `dir`: its key read, or minted once from OS entropy where
    /// none exists (written `0600`, never overwritten). A key file of any
    /// other length is refused and left as it is: a key minted over it would
    /// be refused by every peer that pinned the old one.
    pub fn open(dir: &Path) -> Result<Self, NoiseRefusal> {
        let path = dir.join(KEY_FILE);
        let private = match read_key(&path)? {
            Some(k) => k,
            None => mint_key(&path)?,
        };
        let public = public_of(&private)?;
        Ok(NoiseHome { dir: dir.to_path_buf(), private, public })
    }

    /// This host's static public key: what every peer pins for it.
    pub fn public(&self) -> [u8; KEY_LEN] {
        self.public
    }

    pub fn key_path(&self) -> PathBuf {
        self.dir.join(KEY_FILE)
    }

    pub fn pin_path(&self) -> PathBuf {
        self.dir.join(PIN_FILE)
    }

    /// The key pinned for `peer`, or none where the peer was never met. The
    /// first line naming the peer is its pin.
    pub fn pinned(&self, peer: &str) -> Result<Option<[u8; KEY_LEN]>, NoiseRefusal> {
        let path = self.pin_path();
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(NoiseRefusal::Io(e)),
        };
        for line in text.lines() {
            let Some((host, key)) = line.split_once('\t') else { continue };
            if host == peer {
                return crate::court::identity::parse_hex32(key)
                    .map(Some)
                    .ok_or(NoiseRefusal::PinStoreMalformed(path));
            }
        }
        Ok(None)
    }

    /// TRUST ON FIRST USE: `presented` against the pin for `peer`. A pinned
    /// peer passes only with the pinned key; an unpinned one is pinned to the
    /// key it presented, and read back, so two first contacts racing on one
    /// peer agree on the line that won.
    pub fn pin(&self, peer: &str, presented: &[u8; KEY_LEN]) -> Result<(), NoiseRefusal> {
        if peer.is_empty() || peer.contains(['\t', '\n', '\r']) {
            return Err(NoiseRefusal::PeerLabelMalformed(peer.to_string()));
        }
        let held = match self.pinned(peer)? {
            Some(k) => k,
            None => {
                use std::os::unix::fs::OpenOptionsExt;
                let line = format!("{peer}\t{}\n", crate::court::identity::hex32(presented));
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .mode(0o600)
                    .open(self.pin_path())?
                    .write_all(line.as_bytes())?;
                self.pinned(peer)?.ok_or(NoiseRefusal::PinStoreMalformed(self.pin_path()))?
            }
        };
        match held == *presented {
            true => Ok(()),
            false => Err(NoiseRefusal::PeerKeyChanged { peer: peer.to_string(), pinned: held, presented: *presented }),
        }
    }
}

fn read_key(path: &Path) -> Result<Option<[u8; KEY_LEN]>, NoiseRefusal> {
    match std::fs::read(path) {
        Ok(bytes) => <[u8; KEY_LEN]>::try_from(bytes.as_slice())
            .map(Some)
            .map_err(|_| NoiseRefusal::KeyFileMalformed(path.to_path_buf())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(NoiseRefusal::Io(e)),
    }
}

/// A fresh static key, written once. A create that loses to another
/// process's reads the key that won.
fn mint_key(path: &Path) -> Result<[u8; KEY_LEN], NoiseRefusal> {
    use std::os::unix::fs::OpenOptionsExt;
    let kp = builder()?.generate_keypair().map_err(|_| NoiseRefusal::HandshakeFailed)?;
    let private = <[u8; KEY_LEN]>::try_from(kp.private.as_slice()).map_err(|_| NoiseRefusal::HandshakeFailed)?;
    match std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(path) {
        Ok(mut f) => {
            f.write_all(&private)?;
            f.sync_all()?;
            Ok(private)
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            read_key(path)?.ok_or(NoiseRefusal::KeyFileMalformed(path.to_path_buf()))
        }
        Err(e) => Err(NoiseRefusal::Io(e)),
    }
}

/// One Noise message onto the stream, `[len:2 BE][message]`, in one write.
fn put_message<S: Write>(s: &mut S, msg: &[u8]) -> Result<(), NoiseRefusal> {
    let len = u16::try_from(msg.len()).map_err(|_| NoiseRefusal::MessageTooLarge { len: msg.len() })?;
    let mut out = Vec::with_capacity(2 + msg.len());
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(msg);
    s.write_all(&out)?;
    Ok(())
}

/// One Noise message off the stream; its length prefix bounds it at
/// `MESSAGE_MAX`.
fn take_message<S: Read>(s: &mut S) -> Result<Vec<u8>, NoiseRefusal> {
    let mut head = [0u8; 2];
    s.read_exact(&mut head)?;
    let mut body = vec![0u8; usize::from(u16::from_be_bytes(head))];
    s.read_exact(&mut body)?;
    Ok(body)
}

fn remote_static_of(hs: &snow::HandshakeState) -> Result<[u8; KEY_LEN], NoiseRefusal> {
    hs.get_remote_static()
        .and_then(|k| <[u8; KEY_LEN]>::try_from(k).ok())
        .ok_or(NoiseRefusal::NoPeerStatic)
}

/// THE INITIATOR'S HALF of XX (`-> e`, `<- e ee s es`, `-> s se`) over
/// `stream`, pinning the responder's static key under `peer`, the address
/// dialled. A changed key is refused before this host's static key is sent.
pub fn initiate<S: Read + Write>(mut stream: S, home: &NoiseHome, peer: &str) -> Result<NoiseStream<S>, NoiseRefusal> {
    let mut hs = builder()?
        .local_private_key(&home.private)
        .and_then(|b| b.build_initiator())
        .map_err(|_| NoiseRefusal::HandshakeFailed)?;
    let mut buf = vec![0u8; MESSAGE_MAX];
    let n = hs.write_message(&[], &mut buf).map_err(|_| NoiseRefusal::HandshakeFailed)?;
    put_message(&mut stream, &buf[..n])?;
    let m = take_message(&mut stream)?;
    hs.read_message(&m, &mut buf).map_err(|_| NoiseRefusal::HandshakeFailed)?;
    let remote_static = remote_static_of(&hs)?;
    home.pin(peer, &remote_static)?;
    let n = hs.write_message(&[], &mut buf).map_err(|_| NoiseRefusal::HandshakeFailed)?;
    put_message(&mut stream, &buf[..n])?;
    let ts = hs.into_transport_mode().map_err(|_| NoiseRefusal::HandshakeFailed)?;
    Ok(NoiseStream::new(stream, ts, remote_static))
}

/// THE RESPONDER'S HALF of XX (`<- e`, `-> e ee s es`, `<- s se`) over
/// `stream`. With `peer`, the initiator's static key is pinned under it; an
/// inbound caller dials from a port of its own choosing and is pinned only
/// under a name the caller gives it.
pub fn respond<S: Read + Write>(mut stream: S, home: &NoiseHome, peer: Option<&str>) -> Result<NoiseStream<S>, NoiseRefusal> {
    let mut hs = builder()?
        .local_private_key(&home.private)
        .and_then(|b| b.build_responder())
        .map_err(|_| NoiseRefusal::HandshakeFailed)?;
    let mut buf = vec![0u8; MESSAGE_MAX];
    let m = take_message(&mut stream)?;
    hs.read_message(&m, &mut buf).map_err(|_| NoiseRefusal::HandshakeFailed)?;
    let n = hs.write_message(&[], &mut buf).map_err(|_| NoiseRefusal::HandshakeFailed)?;
    put_message(&mut stream, &buf[..n])?;
    let m = take_message(&mut stream)?;
    hs.read_message(&m, &mut buf).map_err(|_| NoiseRefusal::HandshakeFailed)?;
    let remote_static = remote_static_of(&hs)?;
    if let Some(peer) = peer {
        home.pin(peer, &remote_static)?;
    }
    let ts = hs.into_transport_mode().map_err(|_| NoiseRefusal::HandshakeFailed)?;
    Ok(NoiseStream::new(stream, ts, remote_static))
}

/// An established channel: the stream, the transport cipher states, and the
/// plaintext of the last message read that has not yet been taken.
pub struct NoiseStream<S> {
    inner: S,
    ts: snow::TransportState,
    rbuf: Vec<u8>,
    rpos: usize,
    remote_static: [u8; KEY_LEN],
}

impl<S> NoiseStream<S> {
    fn new(inner: S, ts: snow::TransportState, remote_static: [u8; KEY_LEN]) -> Self {
        NoiseStream { inner, ts, rbuf: Vec::new(), rpos: 0, remote_static }
    }

    /// The peer's static Noise key the handshake authenticated.
    pub fn remote_static(&self) -> [u8; KEY_LEN] {
        self.remote_static
    }

    pub fn get_ref(&self) -> &S {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut S {
        &mut self.inner
    }
}

impl<S: Read + Write> NoiseStream<S> {
    /// `plain` sealed in transport messages of at most `CHUNK` plaintext
    /// bytes each, in order.
    pub fn send(&mut self, plain: &[u8]) -> Result<(), NoiseRefusal> {
        let mut cipher = vec![0u8; MESSAGE_MAX];
        for chunk in plain.chunks(CHUNK) {
            let n = self.ts.write_message(chunk, &mut cipher).map_err(|_| NoiseRefusal::EncryptFailed)?;
            put_message(&mut self.inner, &cipher[..n])?;
        }
        self.inner.flush()?;
        Ok(())
    }

    /// The next transport message opened into the read buffer. A message
    /// that carries no plaintext is read past.
    fn fill(&mut self) -> Result<(), NoiseRefusal> {
        while self.rpos == self.rbuf.len() {
            let msg = take_message(&mut self.inner)?;
            let mut plain = vec![0u8; msg.len()];
            let n = self.ts.read_message(&msg, &mut plain).map_err(|_| NoiseRefusal::Decrypt)?;
            plain.truncate(n);
            self.rbuf = plain;
            self.rpos = 0;
        }
        Ok(())
    }

    fn take_plain(&mut self, out: &mut [u8]) -> Result<(), NoiseRefusal> {
        let mut got = 0;
        while got < out.len() {
            self.fill()?;
            let n = (out.len() - got).min(self.rbuf.len() - self.rpos);
            out[got..got + n].copy_from_slice(&self.rbuf[self.rpos..self.rpos + n]);
            self.rpos += n;
            got += n;
        }
        Ok(())
    }

    /// One wire frame, with the point it carries where one is given, sealed
    /// onto the channel. A body over the wire's budget is refused before a
    /// byte is sent.
    pub fn write_frame(&mut self, frame: &Frame, point: Option<&TPoint>) -> Result<(), NoiseRefusal> {
        let bytes = frame.encode_at(point).ok_or(NoiseRefusal::FrameOverBudget { len: frame.body.len() })?;
        self.send(&bytes)
    }

    /// One wire frame off the channel, with the point it carries. A head
    /// naming a body past the wire's budget is refused before the body is
    /// allocated.
    pub fn read_frame(&mut self) -> Result<(Frame, Option<TPoint>), NoiseRefusal> {
        let mut head = [0u8; 5];
        self.take_plain(&mut head)?;
        let (lane, pointed) = read_lane(head[0]).ok_or(NoiseRefusal::FrameMalformed)?;
        let len = usize::try_from(u32::from_le_bytes([head[1], head[2], head[3], head[4]]))
            .map_err(|_| NoiseRefusal::FrameMalformed)?;
        if len > BODY_BUDGET + TPoint::LEN {
            return Err(NoiseRefusal::FrameOverBudget { len });
        }
        let mut body = vec![0u8; len];
        self.take_plain(&mut body)?;
        let (body, point) = split_point(body, pointed).ok_or(NoiseRefusal::FrameMalformed)?;
        Ok((Frame { lane, body }, point))
    }
}

/// The channel as a byte stream, so a reader of plaintext frames reads it
/// unchanged. A close at a message boundary reads as the end of the stream.
impl<S: Read + Write> Read for NoiseStream<S> {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        if out.is_empty() {
            return Ok(0);
        }
        match self.fill() {
            Ok(()) => {}
            Err(NoiseRefusal::Closed) => return Ok(0),
            Err(NoiseRefusal::Io(e)) => return Err(e),
            Err(r) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, r)),
        }
        let n = out.len().min(self.rbuf.len() - self.rpos);
        out[..n].copy_from_slice(&self.rbuf[self.rpos..self.rpos + n]);
        self.rpos += n;
        Ok(n)
    }
}

impl<S: Read + Write> Write for NoiseStream<S> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.send(buf) {
            Ok(()) => Ok(buf.len()),
            Err(NoiseRefusal::Io(e)) => Err(e),
            Err(r) => Err(std::io::Error::other(r)),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::wire::Stratum;
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
    /// other's public key, and a pointed frame and its reply read back
    /// exactly, lane, body and point.
    #[test]
    fn initiate_completes_handshake_and_frames_round_trip() {
        let ((_da, a), (_db, b)) = (home(), home());
        let (ia, rb) = handshake(&a, &b, "127.0.0.1:7500", |s| s);
        let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
        assert_eq!(ia.remote_static(), b.public());
        assert_eq!(rb.remote_static(), a.public());
        assert_ne!(a.public(), b.public());

        use crate::chronometry::tuniversal::Origin;
        let point = TPoint { origin: Origin { genesis: [7; 32], epoch: 146 }, h: 40_961, lanes: [3, 0, 9, 1], k: 812 };
        let ask = Frame { lane: Stratum::Gauge, body: vec![0x53, 1, 2, 3] };
        ia.write_frame(&ask, Some(&point)).unwrap();
        assert_eq!(rb.read_frame().unwrap(), (ask, Some(point)));
        let reply = Frame { lane: Stratum::Harmonic, body: vec![9; 40] };
        rb.write_frame(&reply, None).unwrap();
        assert_eq!(ia.read_frame().unwrap(), (reply, None));
    }

    /// A frame past one Noise message rides as ⌈bytes / CHUNK⌉ messages,
    /// none over the ceiling, and joins back to the same frame.
    #[test]
    fn write_frame_chunks_past_one_message_and_rejoins() {
        let ((_da, a), (_db, b)) = (home(), home());
        let (ia, rb) = handshake(&a, &b, "peer", Tap::new);
        let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
        rb.get_mut().seen.clear();
        let body: Vec<u8> = (0..3 * CHUNK + 7).map(|i| u8::try_from(i % 251).unwrap()).collect();
        let frame = Frame { lane: Stratum::Temporal, body };
        let sender = std::thread::spawn(move || {
            ia.write_frame(&frame, None).unwrap();
            frame
        });
        let (got, point) = rb.read_frame().unwrap();
        let frame = sender.join().unwrap();
        assert_eq!((got, point), (frame.clone(), None));
        let lens = message_lengths(&rb.get_ref().seen);
        let plain = 5 + frame.body.len();
        assert_eq!(lens.len(), plain.div_ceil(CHUNK));
        assert!(lens.iter().all(|&l| l <= MESSAGE_MAX));
        assert_eq!(lens.iter().map(|l| l - TAG_LEN).sum::<usize>(), plain);
    }

    /// A body over the wire's budget is refused before a byte is sent: the
    /// next frame is the next thing the peer reads.
    #[test]
    fn write_frame_refuses_over_budget_before_sending() {
        let ((_da, a), (_db, b)) = (home(), home());
        let (ia, rb) = handshake(&a, &b, "peer", |s| s);
        let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
        let over = Frame { lane: Stratum::Spatial, body: vec![0; BODY_BUDGET + 1] };
        assert!(matches!(ia.write_frame(&over, None), Err(NoiseRefusal::FrameOverBudget { len }) if len == BODY_BUDGET + 1));
        let next = Frame { lane: Stratum::Spatial, body: vec![4, 2] };
        ia.write_frame(&next, None).unwrap();
        assert_eq!(rb.read_frame().unwrap(), (next, None));
    }

    /// A head that names a body past the budget is refused by its length,
    /// the body never allocated.
    #[test]
    fn read_frame_refuses_head_past_budget() {
        let ((_da, a), (_db, b)) = (home(), home());
        let (ia, rb) = handshake(&a, &b, "peer", |s| s);
        let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
        let len = BODY_BUDGET + TPoint::LEN + 1;
        let mut head = vec![Stratum::Spatial.as_u8()];
        head.extend_from_slice(&u32::try_from(len).unwrap().to_le_bytes());
        ia.write_all(&head).unwrap();
        assert!(matches!(rb.read_frame(), Err(NoiseRefusal::FrameOverBudget { len: l }) if l == len));
    }

    /// One flipped bit of a transport message's ciphertext fails its tag.
    #[test]
    fn read_frame_refuses_tampered_ciphertext() {
        let ((_da, a), (_db, b)) = (home(), home());
        let (ia, rb) = handshake(&a, &b, "peer", Tap::new);
        let (mut ia, mut rb) = (ia.unwrap(), rb.unwrap());
        // Past the two-byte length, the first ciphertext byte.
        rb.get_mut().flip_in = Some(2);
        ia.write_frame(&Frame { lane: Stratum::Gauge, body: vec![1, 2, 3] }, None).unwrap();
        assert!(matches!(rb.read_frame(), Err(NoiseRefusal::Decrypt)));
    }

    /// A peer pinned on first contact passes again with its key; the same
    /// address presenting another key is refused by name, before the
    /// initiator's static key is sent, and the pin survives a reload.
    #[test]
    fn pin_refuses_changed_peer_key_by_name() {
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
        assert!(matches!(a.pin("a\tb", &c.public()), Err(NoiseRefusal::PeerLabelMalformed(_))));
    }

    /// The static key is minted once, `0600`, and reloads to the same key; a
    /// key file of another length is refused and left untouched.
    #[test]
    fn noise_home_key_persists_across_open() {
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
}
