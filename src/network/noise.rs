// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ENCRYPTED CHANNEL: Noise_XX over a byte stream (a TCP connection on
//! the mesh), carrying the wire's envelopes (`wire::Delimited`) inside Noise
//! transport messages.
//!
//! No certificate authority stands behind it. Each host keeps one static
//! Noise key (`noise.key` in its home) and pins each peer's static key on
//! first contact (`noise.pins`, keyed by the peer's address): a pinned peer
//! that presents a different key is refused by name (`PeerKeyChanged`).
//!
//! The channel only encrypts and authenticates the BYTES between two hosts.
//! Identity stays in the Ed25519-signed acts the envelopes carry; the Noise
//! static key is a Curve25519 key of the channel and is never an identity,
//! never derived from one and never read as one.
//!
//! On the stream every Noise message is `[len:2 BE][message]`, a message at
//! most `MESSAGE_MAX` bytes. An envelope's encoding is cut into plaintext chunks
//! of at most `CHUNK` bytes, one transport message each; the reader joins
//! them back. An envelope over the wire's budget is refused exactly as the
//! plaintext wire refuses it: before a byte is sent, and on read before its
//! body is allocated.
//!
//! The stream under the channel is BLOCKING: a `WouldBlock` in the middle of
//! a message would lose the bytes already read of it. A stream read at a
//! stroke pace (`channel::Paced`) waits inside its own `read`, so it is
//! blocking to the channel.
//!
//! THE OPENING. The initiator writes [`OPEN`] alone and begins the handshake
//! only once the responder has answered [`OPEN`]. No plaintext
//! envelope begins with `OPEN`: its first four bytes, read as a plaintext
//! length, pass the wire's `DELIMITED_MAX`. A door that speaks only
//! plaintext reads it as an envelope over budget and answers a plaintext
//! refusal, which the initiator reads as `NotSpoken`; a door that speaks
//! Noise tells the two apart by those four bytes (`channel::accept`).

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use super::wire::{delimited_len, Delimited, HEAD};

/// The one handshake pattern and cipher suite of the channel.
pub const PATTERN: &str = "Noise_XX_25519_ChaChaPoly_BLAKE2s";
/// Noise's ceiling on one message, tag included.
/// EXTERNAL: Noise Protocol Framework rev. 34 §7, message at most 65535 bytes.
pub const MESSAGE_MAX: usize = 65535;
/// The AEAD tag every transport message carries.
/// EXTERNAL: Noise Protocol Framework rev. 34 §7, ChaChaPoly tag 16 bytes.
pub const TAG_LEN: usize = 16;
/// The plaintext one transport message carries at most.
pub const CHUNK: usize = MESSAGE_MAX - TAG_LEN;
/// The host's static Noise private key, under its home directory.
pub const KEY_FILE: &str = "noise.key";
/// The host's pins: one `peer<TAB>hex(static key)` line per peer address.
pub const PIN_FILE: &str = "noise.pins";
/// A Curve25519 key's length.
/// EXTERNAL: Curve25519/X25519 key, 32 bytes (RFC 7748).
pub const KEY_LEN: usize = 32;
/// THE CHANNEL'S OPENING, written by each side before its first handshake
/// message: four bytes that read as a plaintext length past the wire's
/// `DELIMITED_MAX` (`0x58584eff`), then a fifth.
/// FORMAT: Noise channel opening bytes.
pub const OPEN: [u8; 5] = [0xff, b'N', b'X', b'X', 0x01];

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
    Handshake,
    /// The handshake ended without the peer's static key of `KEY_LEN` bytes.
    #[error("noise peer presented no static key")]
    NoPeerStatic,
    /// A pinned peer presented a static key other than the one pinned.
    #[error("noise peer {peer} presented a changed static key")]
    PeerKeyChanged { peer: String, pinned: [u8; KEY_LEN], presented: [u8; KEY_LEN] },
    /// The peer answered the opening with bytes other than [`OPEN`]: it does
    /// not speak Noise. `answered` holds what it wrote in the opening's place.
    #[error("noise peer does not speak Noise")]
    NotSpoken { answered: [u8; 5] },
    /// A pinned peer answered in plaintext: a channel once sealed is never
    /// opened again without its seal. Re-pinning is an operator act.
    #[error("noise peer {peer} is pinned and answered in plaintext")]
    PinnedAnsweredPlain { peer: String, pinned: [u8; KEY_LEN] },
    /// A peer address the pin store cannot hold as one line.
    #[error("noise peer address {0:?} cannot be pinned")]
    PeerLabel(String),
    /// A Noise message past `MESSAGE_MAX`.
    #[error("noise message of {len} bytes over the {MESSAGE_MAX}-byte ceiling")]
    MessageTooLarge { len: usize },
    /// An envelope past the wire's budget.
    #[error("envelope of {len} bytes over the wire budget")]
    OverBudget { len: usize },
    /// A decrypted envelope whose head does not read as a length.
    #[error("noise channel carried a malformed envelope")]
    Malformed,
    /// A transport message that failed to authenticate: tampered, replayed,
    /// reordered or cut.
    #[error("noise transport message failed to authenticate")]
    Decrypt,
    /// A transport message that would not seal.
    #[error("noise transport message failed to seal")]
    Encrypt,
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
    let params = PATTERN.parse().map_err(|_| NoiseRefusal::Handshake)?;
    Ok(snow::Builder::new(params))
}

/// The Curve25519 public key of a private key, by the resolver the
/// handshake itself uses: the key a peer sees is the key this reports.
fn public_of(private: &[u8; KEY_LEN]) -> Result<[u8; KEY_LEN], NoiseRefusal> {
    use snow::resolvers::CryptoResolver;
    let mut dh = snow::resolvers::DefaultResolver
        .resolve_dh(&snow::params::DHChoice::Curve25519)
        .ok_or(NoiseRefusal::Handshake)?;
    dh.set(private);
    <[u8; KEY_LEN]>::try_from(dh.pubkey()).map_err(|_| NoiseRefusal::Handshake)
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

    /// The home at `dir` where its key already stands, and none where no key
    /// was ever minted there: a reader that creates nothing (the console).
    pub fn existing(dir: &Path) -> Result<Option<Self>, NoiseRefusal> {
        let Some(private) = read_key(&dir.join(KEY_FILE))? else { return Ok(None) };
        let public = public_of(&private)?;
        Ok(Some(NoiseHome { dir: dir.to_path_buf(), private, public }))
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
            return Err(NoiseRefusal::PeerLabel(peer.to_string()));
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
    let kp = builder()?.generate_keypair().map_err(|_| NoiseRefusal::Handshake)?;
    let private = <[u8; KEY_LEN]>::try_from(kp.private.as_slice()).map_err(|_| NoiseRefusal::Handshake)?;
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

/// The opening, alone: a door that reads it as a plaintext head has read
/// every byte sent, so its refusal and its close reach the initiator whole.
fn put_open<S: Write>(s: &mut S) -> Result<(), NoiseRefusal> {
    s.write_all(&OPEN)?;
    s.flush()?;
    Ok(())
}

/// The peer's opening read: [`OPEN`], or `NotSpoken` with what stood in its
/// place.
fn take_opening<S: Read>(s: &mut S) -> Result<(), NoiseRefusal> {
    let mut answered = [0u8; 5];
    s.read_exact(&mut answered)?;
    match answered == OPEN {
        true => Ok(()),
        false => Err(NoiseRefusal::NotSpoken { answered }),
    }
}

/// THE INITIATOR'S HALF of XX (`-> e`, `<- e ee s es`, `-> s se`) over
/// `stream`, pinning the responder's static key under `peer`, the address
/// dialled. A changed key is refused before this host's static key is sent.
/// The handshake begins once the responder has answered the opening with its
/// own: a peer that answers anything else does not speak Noise
/// (`NotSpoken`), and no handshake byte was sent to it.
pub fn initiate<S: Read + Write>(mut stream: S, home: &NoiseHome, peer: &str) -> Result<NoiseStream<S>, NoiseRefusal> {
    let mut hs = builder()?
        .local_private_key(&home.private)
        .and_then(|b| b.build_initiator())
        .map_err(|_| NoiseRefusal::Handshake)?;
    put_open(&mut stream)?;
    take_opening(&mut stream)?;
    let mut buf = vec![0u8; MESSAGE_MAX];
    let n = hs.write_message(&[], &mut buf).map_err(|_| NoiseRefusal::Handshake)?;
    put_message(&mut stream, &buf[..n])?;
    let m = take_message(&mut stream)?;
    hs.read_message(&m, &mut buf).map_err(|_| NoiseRefusal::Handshake)?;
    let remote_static = remote_static_of(&hs)?;
    home.pin(peer, &remote_static)?;
    let n = hs.write_message(&[], &mut buf).map_err(|_| NoiseRefusal::Handshake)?;
    put_message(&mut stream, &buf[..n])?;
    let ts = hs.into_transport_mode().map_err(|_| NoiseRefusal::Handshake)?;
    Ok(NoiseStream::new(stream, ts, remote_static))
}

/// THE RESPONDER'S HALF of XX (`<- e`, `-> e ee s es`, `<- s se`) over
/// `stream`. With `peer`, the initiator's static key is pinned under it; an
/// inbound caller dials from a port of its own choosing and is pinned only
/// under a name the caller gives it. The stream opens with [`OPEN`]; one
/// that opens otherwise is `NotSpoken`.
pub fn respond<S: Read + Write>(mut stream: S, home: &NoiseHome, peer: Option<&str>) -> Result<NoiseStream<S>, NoiseRefusal> {
    let mut hs = builder()?
        .local_private_key(&home.private)
        .and_then(|b| b.build_responder())
        .map_err(|_| NoiseRefusal::Handshake)?;
    take_opening(&mut stream)?;
    put_open(&mut stream)?;
    let mut buf = vec![0u8; MESSAGE_MAX];
    let m = take_message(&mut stream)?;
    hs.read_message(&m, &mut buf).map_err(|_| NoiseRefusal::Handshake)?;
    let n = hs.write_message(&[], &mut buf).map_err(|_| NoiseRefusal::Handshake)?;
    put_message(&mut stream, &buf[..n])?;
    let m = take_message(&mut stream)?;
    hs.read_message(&m, &mut buf).map_err(|_| NoiseRefusal::Handshake)?;
    let remote_static = remote_static_of(&hs)?;
    if let Some(peer) = peer {
        home.pin(peer, &remote_static)?;
    }
    let ts = hs.into_transport_mode().map_err(|_| NoiseRefusal::Handshake)?;
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
            let n = self.ts.write_message(chunk, &mut cipher).map_err(|_| NoiseRefusal::Encrypt)?;
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

    /// One envelope sealed onto the channel. A body over the wire's
    /// budget is refused before a byte is sent.
    pub fn write_delimited(&mut self, delimited: &Delimited) -> Result<(), NoiseRefusal> {
        let bytes = delimited.encode().ok_or(NoiseRefusal::OverBudget { len: delimited.body.len() })?;
        self.send(&bytes)
    }

    /// One envelope off the channel. A head naming a body past the wire's
    /// budget is refused before the body is allocated.
    pub fn read_delimited(&mut self) -> Result<Delimited, NoiseRefusal> {
        let mut head = [0u8; HEAD];
        self.take_plain(&mut head)?;
        let Some(len) = delimited_len(head) else {
            let len = usize::try_from(u32::from_le_bytes(head)).map_err(|_| NoiseRefusal::Malformed)?;
            return Err(NoiseRefusal::OverBudget { len });
        };
        let mut body = vec![0u8; len];
        self.take_plain(&mut body)?;
        Ok(Delimited { body })
    }
}

/// The channel as a byte stream, so a reader of plaintext envelopes reads it
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
