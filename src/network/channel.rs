// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §3.1, MU-ENGINEERING §9 — THE CHANNEL A CONNECTION RIDES:
// Noise XX where both ends speak it, plaintext where one does not, chosen
// per peer and named either way.
//
//! A dialler holding a Noise home (`NoiseHome`) opens every connection with
//! the channel's opening (`noise::OPEN`) and the XX handshake, its static key
//! the host's long-term Noise key, the peer's pinned under the address
//! dialled. A door holding a home reads the first four bytes of each
//! connection: the opening is answered with the handshake, a length is a
//! plaintext envelope read as it always was.
//!
//! OPT-IN PER PEER (MU-PRECURSORS §3.1). A peer that does not speak Noise
//! stays plaintext and is named: in the log the first time it is met, and in
//! the book the observer reads (`book`, the `channels` reading). A dialler
//! whose peer answers the opening in plaintext dials it again in plaintext,
//! unless the peer is pinned: a pinned peer is held to its seal, refused by
//! name (`PinnedAnsweredPlain`), and a pinned peer presenting another static
//! key is refused by name (`PeerKeyChanged`), never with a fallback.
//!
//! The channel moves bytes. It knows byte counts, peer addresses, peer keys
//! and the pace of a read, and never what the bytes mean.

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::num::NonZeroUsize;

use super::noise::{self, NoiseHome, NoiseRefusal, NoiseStream, KEY_LEN, OPEN};

/// HOW A CHANNEL'S READS WAIT, denominated in strokes (law 9), never in
/// seconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pace {
    /// A blocking read, as the socket gives it; no poll is counted.
    Blocking,
    /// A polled read, one stroke per poll that moves no byte. `Some(n)`
    /// refuses after `n` such polls in a row (a stall; progress refills the
    /// count), `None` reads to the stream's own end. The polls are counted,
    /// so a round trip is measured in them (`Paced::first_byte_at`).
    Polled(Option<NonZeroUsize>),
}

/// A TCP STREAM READ AT A PACE. Writes block; a polled read waits inside its
/// own `read`, so to the Noise channel above it the stream is blocking.
#[derive(Debug)]
pub struct Paced {
    tcp: TcpStream,
    pace: Pace,
    polls: u64,
    first: Option<u64>,
    /// Bytes read off the stream to tell the channel apart (`take_head`),
    /// served again before the stream's own.
    held: Vec<u8>,
}

impl Paced {
    pub fn new(tcp: TcpStream, pace: Pace) -> Self {
        Paced { tcp, pace, polls: 0, first: None, held: Vec::new() }
    }

    pub fn tcp(&self) -> &TcpStream {
        &self.tcp
    }

    /// The pace the next read waits at.
    pub fn set_pace(&mut self, pace: Pace) {
        self.pace = pace;
    }

    /// A fresh measure: the polls counted so far are let go.
    pub fn measure(&mut self) {
        self.polls = 0;
        self.first = None;
    }

    /// The polls made since `measure`, up to and including the one that read
    /// the first byte; none where no byte was read. One poll is the
    /// instrument's own resolution, never a floor.
    pub fn first_byte_at(&self) -> Option<u64> {
        self.first
    }

    /// One operation on the stream polled non-blocking at the stall `stall`.
    fn polled<T>(&mut self, stall: Option<NonZeroUsize>, mut op: impl FnMut(&mut TcpStream) -> io::Result<T>) -> io::Result<T> {
        self.tcp.set_nonblocking(true)?;
        let mut budget = stall.map(|n| crate::physics::Physics::default().stroke_n(n.get()));
        let out = loop {
            // Past 2⁶⁴ polls the measure has no carrier: refused, never capped.
            let Some(polls) = self.polls.checked_add(1) else {
                break Err(io::Error::other("the poll count passed its carrier"));
            };
            self.polls = polls;
            match op(&mut self.tcp) {
                Ok(v) => break Ok(v),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    if budget.as_mut().is_some_and(|b| !b.spend()) {
                        break Err(io::Error::new(io::ErrorKind::TimedOut, "the peer stalled past its pace"));
                    }
                    // A poll costs the processor its scheduler floor (the
                    // stroke-duration ruling), never a measured wait.
                    std::thread::sleep(crate::physics::stroke_floor());
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => break Err(e),
            }
        };
        self.tcp.set_nonblocking(false)?;
        out
    }

    /// THE FIRST `N` BYTES THE PEER SENT, read at the stream's pace and held:
    /// every read after serves them again before the stream's own, so a
    /// plaintext envelope they open is read whole. A peer that closes before
    /// `N` bytes is refused (`UnexpectedEof`).
    pub fn take_head<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut head = [0u8; N];
        self.read_exact(&mut head)?;
        self.held.splice(0..0, head);
        Ok(head)
    }
}

impl Read for Paced {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.is_empty() {
            return Ok(0);
        }
        if !self.held.is_empty() {
            let n = out.len().min(self.held.len());
            out[..n].copy_from_slice(&self.held[..n]);
            self.held.drain(..n);
            return Ok(n);
        }
        let n = match self.pace {
            Pace::Blocking => self.tcp.read(out)?,
            Pace::Polled(stall) => self.polled(stall, |t| t.read(out))?,
        };
        if n > 0 && self.first.is_none() {
            self.first = Some(self.polls);
        }
        Ok(n)
    }
}

impl Write for Paced {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.tcp.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tcp.flush()
    }
}

/// WHAT A CHANNEL IS SEALED BY.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The XX handshake completed: every byte rides Noise transport messages.
    Noise,
    /// The peer does not speak Noise, or this end holds no home: plain TCP.
    Plain,
}

/// WHICH END OF THE CONNECTION THIS HOST IS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    /// This host dialled the peer, named by the address dialled.
    Dialled,
    /// The peer dialled this host's door, named by its static key (Noise) or
    /// its address without the port it chose (plaintext).
    Accepted,
}

/// AN OPEN CONNECTION, on the channel its peer speaks.
pub enum Channel {
    Plain(Paced),
    Noise(Box<NoiseStream<Paced>>),
}

impl std::fmt::Debug for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Channel").field("kind", &self.kind()).field("tcp", self.paced().tcp()).finish()
    }
}

impl Channel {
    /// A plaintext channel over a stream already open, read blocking.
    pub fn plain(tcp: TcpStream) -> Self {
        Channel::Plain(Paced::new(tcp, Pace::Blocking))
    }

    pub fn kind(&self) -> Kind {
        match self {
            Channel::Plain(_) => Kind::Plain,
            Channel::Noise(_) => Kind::Noise,
        }
    }

    pub fn paced(&self) -> &Paced {
        match self {
            Channel::Plain(p) => p,
            Channel::Noise(n) => n.get_ref(),
        }
    }

    pub fn paced_mut(&mut self) -> &mut Paced {
        match self {
            Channel::Plain(p) => p,
            Channel::Noise(n) => n.get_mut(),
        }
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.paced().tcp().peer_addr()
    }

    /// The peer's static Noise key, where the channel is sealed.
    pub fn remote_static(&self) -> Option<[u8; KEY_LEN]> {
        match self {
            Channel::Plain(_) => None,
            Channel::Noise(n) => Some(n.remote_static()),
        }
    }
}

impl Read for Channel {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        match self {
            Channel::Plain(p) => p.read(out),
            Channel::Noise(n) => n.read(out),
        }
    }
}

impl Write for Channel {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Channel::Plain(p) => p.write(buf),
            Channel::Noise(n) => n.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Channel::Plain(p) => p.flush(),
            Channel::Noise(n) => n.flush(),
        }
    }
}

fn connect(endpoint: &str, pace: Pace) -> Result<Paced, NoiseRefusal> {
    let tcp = TcpStream::connect(endpoint)?;
    let _ = tcp.set_nodelay(true);
    Ok(Paced::new(tcp, pace))
}

/// DIAL `endpoint`. With a home, the channel is Noise where the peer speaks
/// it; a peer that answers the opening in plaintext is dialled again in
/// plaintext and named, unless it is pinned, which is refused by name. A
/// pinned peer presenting another key is refused by name. Without a home the
/// channel is plaintext, named as well.
pub fn dial(endpoint: &str, home: Option<&NoiseHome>, pace: Pace) -> Result<Channel, NoiseRefusal> {
    dial_as(endpoint, endpoint, home, pace)
}

/// DIAL `endpoint` AS THE PEER NAMED `peer`: `dial`, the peer pinned, met and
/// named under `peer` where the address is not its name. A mind's post
/// listens at a port chosen afresh at each opening, so a mind is pinned
/// under the voice it files its address by (`sdk::chatter::send_over`),
/// never under a port another mind may take next.
pub fn dial_as(endpoint: &str, peer: &str, home: Option<&NoiseHome>, pace: Pace) -> Result<Channel, NoiseRefusal> {
    let Some(home) = home else {
        met(peer, Kind::Plain, Side::Dialled);
        return Ok(Channel::Plain(connect(endpoint, pace)?));
    };
    // A peer met in plaintext by this process and never pinned is dialled
    // in plaintext without the opening asked again.
    if home.pinned(peer)?.is_none() && answered_plain(peer) {
        met(peer, Kind::Plain, Side::Dialled);
        return Ok(Channel::Plain(connect(endpoint, pace)?));
    }
    match noise::initiate(connect(endpoint, pace)?, home, peer) {
        Ok(s) => {
            met(peer, Kind::Noise, Side::Dialled);
            Ok(Channel::Noise(Box::new(s)))
        }
        Err(NoiseRefusal::NotSpoken { .. }) => match home.pinned(peer)? {
            Some(pinned) => Err(NoiseRefusal::PinnedAnsweredPlain { peer: peer.to_string(), pinned }),
            None => {
                met(peer, Kind::Plain, Side::Dialled);
                Ok(Channel::Plain(connect(endpoint, pace)?))
            }
        },
        Err(e) => Err(e),
    }
}

/// ACCEPT a connection at a door. With a home, a connection that opens with
/// the channel's opening is answered with the XX handshake (the caller is
/// pinned under no name: it dials from a port of its own choosing, and its
/// identity is the signature on each act); any other head is a plaintext
/// envelope, read whole from its first byte and named.
///
/// The connection's first four bytes tell the two apart: a plaintext
/// envelope opens with its length, and the opening's four, read as a length,
/// pass `wire::DELIMITED_MAX`, so no plaintext envelope opens with them. The
/// four are held and read again by whichever channel follows.
pub fn accept(tcp: TcpStream, home: Option<&NoiseHome>, pace: Pace) -> Result<Channel, NoiseRefusal> {
    let mut paced = Paced::new(tcp, pace);
    let head = paced.take_head::<{ super::wire::HEAD }>()?;
    match (home, head[..] == OPEN[..super::wire::HEAD]) {
        (Some(home), true) => {
            let s = noise::respond(paced, home, None)?;
            met(&crate::court::identity::hex32(&s.remote_static()), Kind::Noise, Side::Accepted);
            Ok(Channel::Noise(Box::new(s)))
        }
        _ => {
            let at = match paced.tcp().peer_addr() {
                Ok(a) => a.ip().to_string(),
                Err(_) => String::from("an unread address"),
            };
            met(&at, Kind::Plain, Side::Accepted);
            Ok(Channel::Plain(paced))
        }
    }
}

/// ONE PEER AS THIS HOST MET IT: its name, the channel, the side, and how
/// many connections.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Met {
    pub peer: String,
    pub kind: Kind,
    pub side: Side,
    pub count: u64,
}

/// THE BOOK OF PEERS MET, by name, channel and side: the observer's reading
/// of the channels (`observe::Channels`).
static BOOK: std::sync::Mutex<Vec<Met>> = std::sync::Mutex::new(Vec::new());

/// Whether this process dialled `endpoint` and was answered in plaintext.
fn answered_plain(endpoint: &str) -> bool {
    match BOOK.lock() {
        Ok(b) => b.iter().any(|m| m.peer == endpoint && m.kind == Kind::Plain && m.side == Side::Dialled),
        Err(_) => false,
    }
}

/// A connection met: counted under its peer, and the first of each peer,
/// channel and side said in the log. An entry is kept while the book's bytes
/// fit the sealed process memory (MU-DERIVATIONS D2.5 (a)); one past it is
/// refused by name and the book left as it stood. A door of its own that
/// tells the two channels apart by its own reading of the head (a mind's
/// post) names what it met here too.
pub(crate) fn met(peer: &str, kind: Kind, side: Side) {
    let Ok(mut book) = BOOK.lock() else { return };
    if let Some(m) = book.iter_mut().find(|m| m.peer == peer && m.kind == kind && m.side == side) {
        // Past 2⁶⁴ connections the count stands; it is never capped at a
        // value it did not read.
        if let Some(n) = m.count.checked_add(1) {
            m.count = n;
        }
        return;
    }
    let held = book
        .iter()
        .map(|m| m.peer.len())
        .chain([peer.len()])
        .try_fold(0u64, |acc, len| acc.checked_add(u64::try_from(len + std::mem::size_of::<Met>()).ok()?));
    if let Some(c) = crate::physics::sealed() {
        if !c.memory().holds(held) {
            eprintln!("  channel book refused {peer}: past the process's memory");
            return;
        }
    }
    let said = match (kind, side) {
        (Kind::Noise, Side::Dialled) => format!("  channel: dialled {peer} inside Noise"),
        (Kind::Noise, Side::Accepted) => format!("  channel: accepted static key {peer} inside Noise"),
        (Kind::Plain, Side::Dialled) => format!("  channel: dialled {peer} in PLAINTEXT: it does not speak Noise, or this host holds no Noise home"),
        (Kind::Plain, Side::Accepted) => format!("  channel: accepted {peer} in PLAINTEXT: it does not speak Noise"),
    };
    eprintln!("{said}");
    book.push(Met { peer: peer.to_string(), kind, side, count: 1 });
}

/// The peers this host has met, in the order first met.
pub fn book() -> Vec<Met> {
    match BOOK.lock() {
        Ok(b) => b.clone(),
        Err(_) => Vec::new(),
    }
}
