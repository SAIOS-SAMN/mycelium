// law 9 — THE COURT'S TRANSPORT APERTURE: how a hypha reaches a peer, and
// what bounds the reach.
//
//! Every wait here is denominated in STROKES and never in seconds:
//! `read_budgeted` spends a stroke budget while a peer is silent, and the
//! rtt an aperture holds is a stroke count.

use crate::Mesh;
use mycelium::network::wire::Frame;
use std::io::{Read, Write};
use std::net::TcpStream;

pub(crate) fn arm_deadline(s: &TcpStream) {
    let _ = s.set_read_timeout(None);
    let _ = s.set_write_timeout(None);
}

/// Write one frame back, or REFUSE the act. `encode` answers `None` when a
/// body is over the wire budget, and every one of these sites wrote
/// `.unwrap_or_default()` -- an EMPTY buffer, which is not a frame. The
/// caller then waited on five bytes that were never coming, with no read
/// bound, which is how one oversized reply stopped an exchange thread and
/// every act it had left to perform. An act that cannot be framed is
/// refused, said once, and the connection closes; the caller reads that as
/// silence, which it already counts (rule 11).
pub(crate) fn answer(s: &mut TcpStream, frame: Frame) {
    match frame.encode() {
        Some(bytes) => {
            let _ = s.write_all(&bytes);
        }
        None => eprintln!(
            "  court reply refused: body {} over the wire budget",
            frame.body.len()
        ),
    }
}

pub(crate) fn read_frame(s: &mut TcpStream) -> Option<Frame> {
    let mut head = [0u8; 5];
    s.read_exact(&mut head).ok()?;
    let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
    if len > mycelium::network::wire::BODY_BUDGET {
        return None;
    }
    let mut buf = vec![0u8; 5 + len];
    buf[..5].copy_from_slice(&head);
    s.read_exact(&mut buf[5..]).ok()?;
    Frame::decode(&buf).map(|(f, _)| f)
}

/// THE DEFERRAL FRAME, read on a hypha-to-hypha reply (23.8, I36): a full
/// reflex pool answers every act with the pending outcome and four zero
/// lanes. On a peer's reply that is impedance — the peer refused nothing
/// and witnessed nothing — and never a vantage: decoded as an offer reply
/// its code was arm 2, `Refused` by key `0000…`, and one hypha logged
/// 4,762 such refusals in ten minutes. A predicate at the wire boundary
/// (§2.3); every reader of a peer's reply asks it before decoding.
pub(crate) fn is_deferred(reply: &[u8]) -> bool {
    mycelium::anastome::dialogue::is_deferral(
        reply,
        mycelium::cognition::outcome_code(&mycelium::cognition::Outcome::GermPending),
    )
}

/// Speak one act to a peer on the nerve, bounded by the transport deadline;
/// the reply, if any, is not needed. Silence is the peer's impedance.
pub(crate) fn tell_peer(addr: &str, body: Vec<u8>) -> Option<(Vec<u8>, u64)> {
    tell_peer_within(addr, body, 5)
}

/// SPEAK TO A PEER AT ITS OWN MEASURED PATIENCE (Phase 1 item 3). The mesh
/// measured every peer's round trip and backed its deadline off on every
/// timeout (`note_rtt_ok`, `note_rtt_timeout`, into `mesh.rtt`) — and no
/// ask ever read it: every `tell_peer` waited the literal 5, forty polls, so
/// a peer answering from behind one court stroke was counted silent, and
/// every back-fill page on the fleet went unanswered (the catch-up reading,
/// 2026-09-30). This waits the deadline the peer's own wire measured, the
/// literal only where none is measured yet, and records what it saw.
pub(crate) fn tell_peer_measured(mesh: &Mesh, addr: &str, body: Vec<u8>) -> Option<(Vec<u8>, u64)> {
    // An unmeasured wire is asked as `tell_peer` asks every wire; the first
    // answer measures it.
    let reply = match mesh.rtt.lock().ok().and_then(|m| m.get(addr.as_bytes()).copied()) {
        Some(hint) => tell_peer_within(addr, body, hint),
        None => tell_peer(addr, body),
    };
    match reply {
        Some((reply, dt)) => {
            note_rtt_ok(mesh, addr.as_bytes(), dt);
            Some((reply, dt))
        }
        None => {
            note_rtt_timeout(mesh, addr.as_bytes());
            None
        }
    }
}

/// `tell_peer` with the transport measure the caller holds: `rtt_hint` is
/// the peer's measured round trip in strokes, and the wait is an aperture
/// of it (`transport_deadline`). A caller with no measurement passes the
/// same floor the rtt window uses.
pub(crate) fn tell_peer_within(addr: &str, body: Vec<u8>, rtt_hint: u64) -> Option<(Vec<u8>, u64)> {
    let mut s = std::net::TcpStream::connect(addr).ok()?;
    let _ = s.set_nodelay(true);
    // A PEER THAT SAYS NOTHING IS SILENT, NOT AUTHORITATIVE. `read_exact`
    // with no bound makes a peer that writes nothing an unbounded wait, and
    // one such peer stops this thread and every act it was going to perform
    // -- which is what a frame that failed to encode on the far side
    // produced: an empty write, and a caller waiting on five bytes that
    // never came. Silence is a refusal and is counted as one.
    //
    // The bound is STROKES, not seconds. My first cut set two 20-second
    // socket timeouts, which is a wall clock in a binary (law 9) and the
    // very sites B6 exists to remove. A socket cannot be handed a stroke,
    // so the READ is budgeted instead: non-blocking, one spend per attempt,
    // refused when the transport's own deadline is spent.
    let frame = Frame { lane: mycelium::network::wire::Stratum::Spatial, body };
    s.write_all(&frame.encode()?).ok()?;
    s.set_nonblocking(true).ok()?;
    // A deadline the ledger cannot count does not end the read; an
    // unmeasured one has no deadline to spend either.
    #[expect(clippy::cast_possible_truncation, reason = "site 4 — transport_deadline's strokes (rtt × aperture_cycle), a u64 → usize narrowing on a 32-bit target; its opposite operation is asked at site 4")]
    let stall = match mycelium::physics::transport_deadline(rtt_hint) {
        mycelium::physics::Lambda::Strokes(n) => n as usize,
        mycelium::physics::Lambda::BeyondTheLedger
        | mycelium::physics::Lambda::Unmeasured => usize::MAX,
    };
    let mut head = [0u8; 5];
    // THE ROUND TRIP IS MEASURED HERE (45.4, W9b): the strokes this kernel
    // spent between the write and the first byte back, one stroke per poll
    // of the socket, the poll that found the byte included. It returned a
    // literal 1, so every rtt window held ones, `rtt_floor` was 1 on every
    // wire and `transport_deadline(1)` was eight yields -- microseconds,
    // refusing a peer that had merely not been scheduled. The body's read
    // is the frame's length, not the transport, and is not in the measure.
    let rtt = read_budgeted(&mut s, &mut head, stall)?;
    let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
    if len > mycelium::network::wire::BODY_BUDGET {
        return None;
    }
    let mut reply = vec![0u8; len];
    read_budgeted(&mut s, &mut reply, stall)?;
    Some((reply, rtt))
}

/// Fill `buf` from a non-blocking stream, paying one stroke per attempt
/// that finds nothing, and answer with the strokes it took for the first
/// byte to arrive: the polls made up to and including the one that read
/// it, so a reply present on the first poll took one stroke, the
/// instrument's own resolution, never a floor. `None` when the peer
/// closed, when the read errors, or when the budget is spent -- all three
/// are the same answer to the caller: this peer is SILENT, which the
/// exchange already counts as a refusal. It is never a partial buffer read
/// as a short frame (rule 11). An empty buffer is filled by no poll and
/// measures nothing, which reads as zero strokes.
pub(crate) fn read_budgeted(s: &mut TcpStream, buf: &mut [u8], stall: usize) -> Option<u64> {
    let fresh = || mycelium::physics::Physics::default().stroke_n(stall);
    let mut budget = fresh();
    let mut got = 0usize;
    let mut polls = 0u64;
    let mut first_byte_at = 0u64;
    while got < buf.len() {
        polls += 1;
        match std::io::Read::read(s, &mut buf[got..]) {
            Ok(0) => return None,
            Ok(n) => {
                if got == 0 {
                    first_byte_at = polls;
                }
                got += n;
                // PROGRESS REFILLS THE BUDGET. Spent per poll it counted
                // scheduler passes, not transport: `transport_deadline(8)`
                // is 64, and 64 yields pass in microseconds, so a peer that
                // was merely slow to be scheduled was refused as silent.
                // Measured on my own test, which passed alone and failed
                // beside its neighbours. What the budget bounds is a STALL
                // -- consecutive polls that move no bytes -- so a peer
                // trickling a large frame is never refused for being slow,
                // and one that has said nothing at all still is.
                budget = fresh();
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                if !budget.spend() {
                    return None;
                }
                // A POLL COSTS THE PROCESSOR (the stroke-duration ruling).
                // `yield_now` returns at once when no other thread on this
                // core is ready, so the whole poll budget passed in
                // microseconds and a peer whose listener had merely not
                // been scheduled was counted SILENT — which is what every
                // court read of every peer on the epoch-10 fleet, while
                // each of them pinned a core in its own idle loop.
                std::thread::sleep(mycelium::physics::STROKE_FLOOR);
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(_) => return None,
        }
    }
    Some(first_byte_at)
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
        (Some(_), None) => false,
    }
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
    let n = mycelium::physics::aperture_cycle().max(1) as usize;
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
        mycelium::physics::Lambda::Strokes(n) => n,
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
    let ratio = mycelium::physics::aperture_cycle().max(2);
    let Some(floor) = rtt_floor(mesh, id) else {
        return;
    };
    let cap = match mycelium::physics::persist_dwell(floor) {
        mycelium::physics::Lambda::Strokes(n) => n,
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
