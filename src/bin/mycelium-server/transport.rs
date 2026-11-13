// law 9 — THE COURT'S TRANSPORT APERTURE: how a hypha reaches a peer, and
// what bounds the reach.
//
//! Every wait here is denominated in STROKES and never in seconds: a polled
//! read (`channel::Pace::Polled`) spends a stroke budget while a peer is
//! silent, and the rtt an aperture holds is a stroke count.
//!
//! THE CHANNEL (MU-PRECURSORS §3.1). The door accepts each connection on the
//! channel its caller speaks (`accept`), and every ask to a peer is dialled
//! on Noise where the peer speaks it (`tell_peer_over`); the host's static
//! key and its pins are its Noise home (`seal`). Envelopes inside a Noise
//! channel ride `NoiseStream::{write_delimited, read_delimited}`; a plaintext peer's
//! ride the stream as they always did.

use crate::Mesh;
use mycelium::network::channel::{self, Channel, Pace};
use mycelium::network::noise::NoiseHome;
use mycelium::network::wire::Delimited;
use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) fn arm_deadline(s: &TcpStream) {
    let _ = s.set_read_timeout(None);
    let _ = s.set_write_timeout(None);
}

/// THIS HOST'S NOISE HOME, opened once at boot: its static key and its pins.
static HOME: std::sync::OnceLock<NoiseHome> = std::sync::OnceLock::new();

/// The door and every ask from here on speak Noise with `home`. A second
/// home is not taken: the first stands for the process.
pub(crate) fn seal(home: NoiseHome) {
    let _ = HOME.set(home);
}

/// The process's Noise home, where one was sealed.
pub(crate) fn home() -> Option<&'static NoiseHome> {
    HOME.get()
}

/// A CONNECTION AT THE DOOR, on the channel its caller speaks: the opening
/// answered with the handshake, a plaintext envelope left to be read. A
/// handshake that fails is refused by name and the connection let go.
pub(crate) fn accept(tcp: TcpStream, pace: Pace) -> Option<Channel> {
    accept_with(tcp, home(), pace)
}

pub(crate) fn accept_with(tcp: TcpStream, home: Option<&NoiseHome>, pace: Pace) -> Option<Channel> {
    let at = tcp.peer_addr().ok();
    match channel::accept(tcp, home, pace) {
        Ok(c) => Some(c),
        Err(why) => {
            match at {
                Some(at) => eprintln!("  channel from {at} refused: {why}"),
                None => eprintln!("  channel refused: {why}"),
            }
            None
        }
    }
}

/// THE LISTENER'S OWN ANSWER, where every reader is busy (23.8): the
/// caller's channel opened at the mesh's measured prior, so a caller that
/// stalls its opening holds the listener no longer than a silent peer holds
/// an ask, and `delimited` (the deferral) written on it unread — inside Noise
/// for a Noise caller, as before for a plaintext one.
pub(crate) fn defer_unread(tcp: TcpStream, mesh: &Mesh, delimited: Delimited) {
    defer_unread_with(tcp, mesh, delimited, home());
}

pub(crate) fn defer_unread_with(tcp: TcpStream, mesh: &Mesh, delimited: Delimited, home: Option<&NoiseHome>) {
    let Some(pace) = pace_of(Stall::of(prior(mesh)).resolved()) else { return };
    let Some(mut s) = accept_with(tcp, home, pace) else { return };
    answer(&mut s, delimited);
}

/// The pace a stall reads at; none for `Unmeasured`, on which no read is
/// begun (D2.6; `tell_peer_within` resolves it as a first ask).
pub(crate) fn pace_of(stall: Stall) -> Option<Pace> {
    match stall {
        Stall::Cycles(n) => Some(Pace::Polled(Some(n))),
        Stall::Beyond => Some(Pace::Polled(None)),
        Stall::Unmeasured => None,
    }
}

/// Write one envelope back, or REFUSE the act. `encode` answers `None` when a
/// body is over the wire budget, and every one of these sites wrote
/// `.unwrap_or_default()` -- an EMPTY buffer, which is not an envelope. The
/// caller then waited on five bytes that were never coming, with no read
/// bound, which is how one oversized reply stopped an exchange thread and
/// every act it had left to perform. An act that cannot be delimited is
/// refused, said once, and the connection closes; the caller reads that as
/// silence, which it already counts (rule 11).
///
/// `delimited`'s body is the act; the expression the door writes is formed by
/// the door's clock (`tuniversal::express`), which stands the door's final
/// point beside the act. The transport moves its bytes and reads none.
pub(crate) fn answer(s: &mut Channel, delimited: Delimited) {
    let _ = answer_written(s, delimited);
}

/// `answer`, saying whether the reply was WRITTEN: false where the envelope
/// would not encode or the asker had gone (a reply that missed the asker's
/// wait). The catch-up door counts it (`Court::serve`); a write that fails
/// is never silent there (2026-10-08). A LOWER BOUND: a reply written just
/// after the asker closed can still land in the socket's buffer and read as
/// written, so `unwritten` counts the misses the wire refused, not all.
pub(crate) fn answer_written(s: &mut Channel, delimited: Delimited) -> bool {
    let refused = || eprintln!("  court reply refused: body {} over the wire budget", delimited.body.len());
    let Some(body) = crate::tuniversal::express(&delimited.body) else {
        refused();
        return false;
    };
    let delimited = Delimited { body };
    match s {
        Channel::Noise(n) => match n.write_delimited(&delimited) {
            Ok(()) => true,
            Err(mycelium::network::noise::NoiseRefusal::OverBudget { .. }) => {
                refused();
                false
            }
            Err(_) => false,
        },
        Channel::Plain(p) => match delimited.encode() {
            Some(bytes) => p.write_all(&bytes).is_ok(),
            None => {
                refused();
                false
            }
        },
    }
}

/// ONE ACT READ AT THE DOOR: the envelope's body read whole, then handed to
/// the door's clock as an expression (`tuniversal::hear`), which hears the
/// speaker's point and answers the act. Bytes that are no expression close
/// the connection, as a truncated envelope does.
pub(crate) fn read_delimited(s: &mut Channel) -> Option<Delimited> {
    let delimited = match s {
        Channel::Plain(_) => read_plain_delimited(s)?,
        Channel::Noise(n) => match n.read_delimited() {
            Ok(delimited) => delimited,
            // Refused by name inside the channel, as on plain TCP below.
            Err(mycelium::network::noise::NoiseRefusal::OverBudget { .. }) => {
                refuse_over_budget(s);
                return None;
            }
            Err(_) => return None,
        },
    };
    match crate::tuniversal::hear(delimited.body) {
        Ok(body) => Some(Delimited { body }),
        // #235 — AN EXPRESSION AT ANOTHER WIRE VERSION IS REFUSED BY NAME,
        // before its act is read: answered refused at this door's version,
        // which the speaker's reader names, and the connection closed.
        Err(mycelium::anastome::dialogue::Unheard::Version { spoken, served }) => {
            refuse_version(s, spoken, served);
            None
        }
        Err(mycelium::anastome::dialogue::Unheard::Malformed) => None,
    }
}

/// Refusals by wire version, counted, said at powers of two.
static VERSION_REFUSED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// THE ACT AT A WIRE VERSION THIS DOOR DOES NOT SPEAK (#235), answered
/// refused at the door's own version before the connection closes.
fn refuse_version(s: &mut Channel, spoken: u16, served: u16) {
    let n = VERSION_REFUSED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    if n.is_power_of_two() {
        eprintln!("  door refused wire {spoken:#06x}: this door speaks {served:#06x} ({n} refused)");
    }
    let refused = mycelium::cognition::Outcome::Refused(mycelium::verdict::REFUSE_MALFORMED);
    let body = mycelium::anastome::dialogue::encode_outcome(mycelium::cognition::outcome_code(&refused), 0, &[0; 4]);
    answer(s, Delimited { body });
}

/// THE ACT WHOSE HEAD NAMES A BODY PAST THE WIRE'S BUDGET, answered refused
/// before the connection closes (78.125).
fn refuse_over_budget(s: &mut Channel) {
    let refused = mycelium::cognition::Outcome::Refused(mycelium::verdict::REFUSE_MALFORMED);
    let body = mycelium::anastome::dialogue::encode_outcome(mycelium::cognition::outcome_code(&refused), 0, &[0; 4]);
    answer(s, Delimited { body });
}

fn read_plain_delimited(s: &mut Channel) -> Option<Delimited> {
    let mut head = [0u8; mycelium::network::wire::HEAD];
    s.read_exact(&mut head).ok()?;
    let Some(len) = mycelium::network::wire::delimited_len(head) else {
        // REFUSED BY NAME, NOT DROPPED (78.125): the length names a body past
        // the wire's one budget. Its body is never read — a length is no
        // licence to allocate — but the act is answered refused before the
        // connection closes.
        refuse_over_budget(s);
        return None;
    };
    let mut body = vec![0u8; len];
    s.read_exact(&mut body).ok()?;
    Some(Delimited { body })
}

/// THE DEFERRAL, read on a hypha-to-hypha reply (23.8, I36): a full
/// reflex pool answers every act with the pending outcome and the door's
/// four deficit lanes. On a peer's reply that is impedance — the peer refused nothing
/// and witnessed nothing — and never a vantage: decoded as an offer reply
/// its code was arm 2, `Refused` by key `0000…`, and one hypha logged
/// 4,762 such refusals in ten minutes. A predicate at the wire boundary
/// (§2.3); every reader of a peer's reply asks it before decoding.
pub(crate) fn is_deferred(reply: &[u8]) -> bool {
    mycelium::anastome::dialogue::is_deferral(reply, mycelium::anastome::dialogue::PENDING_CODE)
}

/// HOW LONG AN ASK STALLS ON A PEER THAT SAYS NOTHING, a reading of the
/// transport (MU-DERIVATIONS D2.6, D2.7, D4.1), never a chosen count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stall {
    /// At most this many consecutive polls that move no byte.
    Cycles(std::num::NonZeroUsize),
    /// Until the stream's own end (end of file or reset): a one-shot ask
    /// whose wait no measurement bounds, or a deadline past the ledger's
    /// count. No poll counter.
    Beyond,
    /// Nothing measured: no wire of the mesh, no peer's own. A read refuses
    /// it (`read_budgeted`); an ask resolves it as a first ask is resolved
    /// (`Stall::resolved`).
    Unmeasured,
}

impl Stall {
    /// The stall an aperture `Λ` reads: its strokes; past the ledger, the
    /// stream's own end; unmeasured, `Unmeasured` (a zero is no measurement).
    pub(crate) fn of(l: mycelium::physics::Lambda) -> Stall {
        match l {
            mycelium::physics::Lambda::Cycles(n) => match usize::try_from(n).map(std::num::NonZeroUsize::new) {
                Ok(Some(n)) => Stall::Cycles(n),
                Ok(None) => Stall::Unmeasured,
                Err(_) => Stall::Beyond,
            },
            mycelium::physics::Lambda::BeyondTheLedger => Stall::Beyond,
            mycelium::physics::Lambda::Unmeasured => Stall::Unmeasured,
        }
    }

    /// AN ASK WITH NOTHING MEASURED IS A FIRST ASK (D2.6, D2.7, D4.1): a
    /// court's waits one aperture of strokes, `Λ(1)` — the instrument's own
    /// resolution times the aperture, the one count the tree holds for a
    /// cycle with nothing measured (the unmeasured arm of `exchange_due`) —
    /// so one silent peer never stops a court thread.
    pub(crate) fn resolved(self) -> Stall {
        match self {
            Stall::Unmeasured => match std::num::NonZeroUsize::new(mycelium::physics::aperture_cycle() as usize) {
                Some(a) => Stall::Cycles(a),
                None => Stall::Unmeasured,
            },
            s => s,
        }
    }
}

/// THE MESH'S MEASURED FLOOR: the least round trip any peer's window holds,
/// in strokes — `rtt_floor` taken across the window's keys. `None` where no
/// wire of the mesh is measured yet (at boot, or a lone court).
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the floor of the mesh's windows is their extent: the least measured round trip, a reading, never a choice"
)]
pub(crate) fn mesh_floor(mesh: &Mesh) -> Option<u64> {
    mesh.rtt_window.lock().ok()?.scoped().filter_map(|(_, ring)| ring.iter().copied().min()).min()
}

/// THE PRIOR OF A PEER THIS MESH HAS NOT MEASURED (D4.1): an aperture of the
/// mesh's measured floor, so an unmeasured peer is asked at the floor the rtt
/// window holds; `Lambda::Unmeasured` where nothing is measured.
pub(crate) fn prior(mesh: &Mesh) -> mycelium::physics::Lambda {
    match mesh_floor(mesh) {
        Some(floor) => mycelium::physics::transport_deadline(floor),
        None => mycelium::physics::Lambda::Unmeasured,
    }
}

/// A PEER'S PATIENCE: an aperture of its own measured round trip, read under
/// its voice key; the mesh's prior where it has none.
pub(crate) fn patience(mesh: &Mesh, voice: Option<&[u8]>) -> Stall {
    let hint = voice.and_then(|v| mesh.rtt.lock().ok().and_then(|m| m.get(v).copied()));
    match hint {
        Some(rtt) => Stall::of(mycelium::physics::transport_deadline(rtt)),
        None => Stall::of(prior(mesh)),
    }
}

/// A ONE-SHOT ASKER'S OWN WINDOW (D2.7): the last aperture of round trips it
/// measured to one endpoint. Its first ask has nothing measured, so it reads
/// to the stream's end (`Stall::Beyond`: the operator holds the process, and
/// it ends with the peer); every later ask waits an aperture of the window's
/// floor, exactly as `rtt_floor` reads the mesh's.
#[derive(Debug, Default)]
pub(crate) struct AskWindow {
    rtts: Vec<u64>,
}

impl AskWindow {
    /// The stall the next ask is made at.
    #[expect(clippy::disallowed_methods, reason = "rule 4 — the floor of the window is that window's extent, a reading")]
    pub(crate) fn stall(&self) -> Stall {
        match self.rtts.iter().copied().min() {
            Some(floor) => Stall::of(mycelium::physics::transport_deadline(floor)),
            None => Stall::Beyond,
        }
    }

    /// A measured round trip enters the window, the oldest leaving past an
    /// aperture.
    pub(crate) fn note(&mut self, rtt: u64) {
        self.rtts.push(rtt);
        if self.rtts.len() > mycelium::physics::aperture_cycle() as usize {
            self.rtts.remove(0);
        }
    }

    /// One ask at the window's stall, its round trip noted.
    pub(crate) fn ask(&mut self, endpoint: &str, body: Vec<u8>) -> Option<Vec<u8>> {
        let (reply, rtt) = tell_peer_within(endpoint, body, self.stall())?;
        self.note(rtt.get());
        Some(reply)
    }
}

/// THE VOICE KEY OF THE PEER AT `endpoint`, as the mesh's address book holds
/// it: ONE KEY PER PEER (D4.1) — the endpoint is a locator, the voice the
/// coordinate its measurements are kept under. `None` for an endpoint whose
/// voice the mesh has not yet learnt (a seed asked before its digest).
pub(crate) fn voice_at(mesh: &Mesh, endpoint: &str) -> Option<[u8; 32]> {
    let book = mesh.addrs.lock().ok()?;
    book.scoped().find(|(_, e)| e.as_str() == endpoint).and_then(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok())
}

/// SPEAK TO A PEER AT ITS OWN MEASURED PATIENCE (Phase 1 item 3; D4.1): the
/// deadline its own wire measured, read under its voice key, or the mesh's
/// measured prior where it has none; and what it saw is recorded under that
/// key. An endpoint whose voice is not yet known is asked at the prior and
/// records nothing: its measurement has no coordinate to stand at.
pub(crate) fn tell_peer_measured(mesh: &Mesh, endpoint: &str, body: Vec<u8>) -> Option<(Vec<u8>, std::num::NonZeroU64)> {
    let voice = voice_at(mesh, endpoint);
    let reply = tell_peer_within(endpoint, body, patience(mesh, voice.as_ref().map(|v| v.as_slice())));
    if let Some(v) = voice {
        match &reply {
            Some((_, dt)) => note_rtt_ok(mesh, v, dt.get()),
            None => note_rtt_timeout(mesh, v),
        }
    }
    reply
}

/// Speak one act to a peer, its silence bounded by `stall` (an aperture of
/// the peer's measured round trip, `patience`; or the stream's own end for a
/// one-shot ask; `Unmeasured` resolved as a first ask). The reply and its
/// round trip in strokes — never zero: the poll that read the first byte is
/// counted (D2.6) — or `None`: the peer is silent.
pub(crate) fn tell_peer_within(endpoint: &str, body: Vec<u8>, stall: Stall) -> Option<(Vec<u8>, std::num::NonZeroU64)> {
    tell_peer_over(endpoint, body, stall, home())
}

/// `tell_peer_within` on the channel `home` opens: Noise where the peer
/// speaks it, plaintext where it does not (named), and a pinned peer that
/// presents another key, or answers in plaintext, refused by name and never
/// asked in plaintext. The handshake waits at the same stall as the reply.
pub(crate) fn tell_peer_over(endpoint: &str, body: Vec<u8>, stall: Stall, home: Option<&NoiseHome>) -> Option<(Vec<u8>, std::num::NonZeroU64)> {
    let pace = pace_of(stall.resolved())?;
    let mut s = match channel::dial(endpoint, home, pace) {
        Ok(s) => s,
        Err(why @ (mycelium::network::noise::NoiseRefusal::PeerKeyChanged { .. } | mycelium::network::noise::NoiseRefusal::PinnedAnsweredPlain { .. })) => {
            eprintln!("  channel to {endpoint} refused: {why}");
            return None;
        }
        Err(_) => return None,
    };
    // A PEER THAT SAYS NOTHING IS SILENT, NOT AUTHORITATIVE. `read_exact`
    // with no bound makes a peer that writes nothing an unbounded wait, and
    // one such peer stops this thread and every act it was going to perform
    // -- which is what an envelope that failed to encode on the far side
    // produced: an empty write, and a caller waiting on five bytes that
    // never came. Silence is a refusal and is counted as one.
    //
    // The bound is STROKES, not seconds. The first cut set two 20-second
    // socket timeouts, which is a wall clock in a binary (law 9) and the
    // very sites B6 exists to remove. A socket cannot be handed a stroke,
    // so the READ is polled instead (`Pace::Polled`): non-blocking, one
    // spend per poll that moves no byte, refused when the transport's own
    // deadline is spent. PROGRESS REFILLS THE BUDGET: what it bounds is a
    // STALL, so a peer trickling a large envelope is never refused for being
    // slow, and one that has said nothing at all still is. A poll costs the
    // processor its scheduler floor (the stroke-duration ruling), so a peer
    // whose listener has merely not been scheduled is not counted silent.
    // A court's act to a peer is written as the door's expression: the act
    // and the door's final point beside it (`tuniversal::express`).
    let delimited = Delimited { body: crate::tuniversal::express(&body)? };
    // THE ROUND TRIP IS MEASURED HERE (45.4, W9b): the strokes this court
    // spent between the write and the first byte back, one stroke per poll
    // of the socket, the poll that found the byte included. It returned a
    // literal 1, so every rtt window held ones, `rtt_floor` was 1 on every
    // wire and `transport_deadline(1)` was eight yields -- microseconds,
    // refusing a peer that had merely not been scheduled. The handshake
    // before the write is not in the measure.
    s.paced_mut().measure();
    let reply = match &mut s {
        Channel::Noise(n) => {
            n.write_delimited(&delimited).ok()?;
            n.read_delimited().ok()?.body
        }
        Channel::Plain(p) => {
            p.write_all(&delimited.encode()?).ok()?;
            let mut head = [0u8; mycelium::network::wire::HEAD];
            p.read_exact(&mut head).ok()?;
            let len = mycelium::network::wire::delimited_len(head)?;
            let mut reply = vec![0u8; len];
            p.read_exact(&mut reply).ok()?;
            reply
        }
    };
    // Never zero: the poll that read the first byte is counted (D2.6).
    let rtt = std::num::NonZeroU64::new(s.paced().first_byte_at()?)?;
    // The peer door's final point stands in its expression: a claim of that
    // door, heard here (`tuniversal::hear`) — this court's final point is its
    // own keepers' quorum.
    let heard = crate::tuniversal::hear(reply).map_err(|why| {
        if let mycelium::anastome::dialogue::Unheard::Version { spoken, served } = why {
            eprintln!("  peer reply refused: it speaks wire {spoken:#06x}, this door {served:#06x}");
        }
    });
    Some((heard.ok()?, rtt))
}

/// WHETHER A PEER IS ASKED AGAIN (45.7, S13.1), a predicate at the wire
/// boundary (§2.3): never asked, or this court's own roots moved, or the
/// peer's Λ — measured in strokes, `None` while unmeasured — has elapsed
/// since the last ask on this court's own K. An unmeasured peer is not
/// polled: it is asked again when the roots move.
pub(crate) fn exchange_due(last_asked: Option<u64>, k_now: u64, lambda: Option<u64>, own_roots_moved: bool) -> bool {
    match (last_asked, lambda) {
        (None, _) => true,
        _ if own_roots_moved => true,
        // The last ask was at a K this court has passed, so the gap is
        // exact; one recorded in the future is not yet due.
        (Some(last), Some(l)) => k_now.checked_sub(last).is_some_and(|gap| gap >= l),
        // AN UNMEASURED PEER IS ASKED AGAIN ONCE PER APERTURE of this
        // court's own K. It waited for this court's roots to move, which
        // is rare: after the restarts of 2026-09-30 hypha a asked a booting
        // b once, heard nothing, and asked it 3 times in twenty minutes
        // against c's 120 — so a never learned b's address, 2 508 of b's
        // folds went unmeasured at a, and its back-fill of b's line
        // proxied through c. The aperture is the calibrated one, not a
        // count chosen here.
        (Some(last), None) => {
            k_now.checked_sub(last).is_some_and(|gap| gap >= u64::from(mycelium::physics::aperture_cycle()))
        }
    }
}

/// WHETHER A QUIET PEER IS ASKED AGAIN (MU-DERIVATIONS D2.5; 78.43, time is
/// distance along an orbit), a predicate at the wire boundary (§2.3):
/// never asked, or the TUniversal final point has advanced since the last
/// ask, which follows the peer's last answer. `asked_at` is the count of the
/// final point's advances read at that ask, `advances` the count now. An
/// idle network stays quiet, and a moving one is heard at the rate it
/// moves; no constant and no backoff enter. A silent peer is not asked
/// again while the network is still, either.
pub(crate) fn quiet_due(asked_at: Option<u64>, advances: u64) -> bool {
    asked_at.is_none_or(|at| advances > at)
}

/// The floor of the last aperture of measured round trips to a peer.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the floor of the window is that window's extent; a peer with no measured round trip has no floor and refuses"
)]
pub(crate) fn rtt_floor(mesh: &Mesh, id: &[u8]) -> Option<u64> {
    mesh.rtt_window
        .lock()
        .ok()
        .and_then(|w| w.get(id).and_then(|ring| ring.iter().copied().min()))
}

/// A measured round trip in strokes enters the peer's window.
pub(crate) fn note_rtt_ok(mesh: &Mesh, id: impl AsRef<[u8]>, sample: u64) {
    let id = id.as_ref();
    let n = mycelium::physics::aperture_cycle() as usize;
    if let Ok(mut w) = mesh.rtt_window.lock() {
        // A peer with no window yet has measured no round trips.
        let mut ring = match w.get(id) {
            Some(r) => r.clone(),
            None => Vec::new(),
        };
        ring.push(sample);
        if ring.len() > n {
            ring.remove(0);
        }
        w.insert(id, ring);
    }
    // The measured floor where this peer has one, this sample's own dwell
    // where it does not; a dwell beyond the ledger caps nothing.
    let dwell = match rtt_floor(mesh, id) {
        Some(floor) => mycelium::physics::persist_dwell(floor),
        None => mycelium::physics::persist_dwell(sample),
    };
    let cap = match dwell {
        mycelium::physics::Lambda::Cycles(n) => n,
        mycelium::physics::Lambda::BeyondTheLedger
        | mycelium::physics::Lambda::Unmeasured => u64::MAX,
    };
    if let Ok(mut m) = mesh.rtt.lock() {
        m.insert(id, sample.min(cap));
    }
}

/// A timeout raises the peer's RTT by the aperture ratio, capped at an
/// aperture of the measured floor: a dead peer's deadline is a ratio of
/// what its wire measured, never a wall clock and never unbounded.
pub(crate) fn note_rtt_timeout(mesh: &Mesh, id: impl AsRef<[u8]>) {
    let id = id.as_ref();
    let ratio = mycelium::physics::aperture_cycle();
    let Some(floor) = rtt_floor(mesh, id) else {
        return;
    };
    let cap = match mycelium::physics::persist_dwell(floor) {
        mycelium::physics::Lambda::Cycles(n) => n,
        mycelium::physics::Lambda::BeyondTheLedger
        | mycelium::physics::Lambda::Unmeasured => u64::MAX,
    };
    if let Ok(mut m) = mesh.rtt.lock() {
        let Some(cur) = m.get(id).copied() else {
            return;
        };
        // A backed-off deadline that does not fit is not a deadline this
        // peer was measured at; the standing reading holds (rule 11).
        if let Some(backed) = cur.checked_mul(ratio as u64) {
            m.insert(id, backed.min(cap));
        }
    }
}
