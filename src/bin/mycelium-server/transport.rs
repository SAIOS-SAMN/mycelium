// law 9 — THE COURT'S TRANSPORT APERTURE: how a hypha reaches a peer, and
// what bounds the reach.
//
//! Every wait here is denominated in STROKES and never in seconds:
//! `read_budgeted` spends a stroke budget while a peer is silent, and the
//! rtt an aperture holds is a stroke count.

use crate::Mesh;
use mycelium::network::nerve::Router;
use mycelium::network::wire::Frame;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;

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

/// Speak one act to a peer on the nerve, bounded by the transport deadline;
/// the reply, if any, is not needed. Silence is the peer's impedance.
pub(crate) fn tell_peer(addr: &str, body: Vec<u8>) -> Option<(Vec<u8>, u64)> {
    tell_peer_within(addr, body, 5)
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
    let stall = mycelium::physics::transport_deadline(rtt_hint) as usize;
    let mut head = [0u8; 5];
    read_budgeted(&mut s, &mut head, stall)?;
    let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
    if len > mycelium::network::wire::BODY_BUDGET {
        return None;
    }
    let mut reply = vec![0u8; len];
    read_budgeted(&mut s, &mut reply, stall)?;
    // THE RTT IS NOT MEASURED. Every caller feeds this 1 into
    // `note_rtt_ok`, so every rtt window holds 1s, `rtt_floor` is 1, and
    // `transport_deadline(1)` is 8 -- eight yields, which is microseconds
    // and refuses a peer that merely has not been scheduled yet. 5.1 claims
    // "rtt in strokes"; this constant is why the claim has nothing behind
    // it, and why the stall budget above has no measurement to be
    // denominated in. Filed as W9b; not invented here (rule 11).
    Some((reply, 1))
}

/// Fill `buf` from a non-blocking stream, paying one stroke per attempt
/// that finds nothing. `None` when the peer closed, when the read errors,
/// or when the budget is spent -- all three are the same answer to the
/// caller: this peer is SILENT, which the exchange already counts as a
/// refusal. It is never a partial buffer read as a short frame (rule 11).
pub(crate) fn read_budgeted(s: &mut TcpStream, buf: &mut [u8], stall: usize) -> Option<()> {
    let fresh = || mycelium::physics::Physics::default().stroke_n(stall);
    let mut budget = fresh();
    let mut got = 0usize;
    while got < buf.len() {
        match std::io::Read::read(s, &mut buf[got..]) {
            Ok(0) => return None,
            Ok(n) => {
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
                std::thread::yield_now();
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(_) => return None,
        }
    }
    Some(())
}

/// The floor of the last aperture of measured round trips to a peer.
pub(crate) fn rtt_floor(mesh: &Mesh, id: &[u8]) -> Option<u64> {
    mesh.rtt_window
        .lock()
        .ok()
        .and_then(|w| w.get(id).and_then(|ring| ring.iter().copied().min()))
}

/// 47.5's Λ for one listing, in admitted receipts:
/// `from_transport(rtt_floor, aperture_cycle())` to the hypha the
/// listing's address is routed to. `None` where this node has measured no
/// round trip to that hypha, or routes the address nowhere — an unmeasured
/// transport has no deadline (`transport_deadline`), and a citation does
/// not lapse against a number nobody read.
///
/// The chapter's Λ is per CHART and this is per listing address; a
/// listing's charts can be homed at different hyphae, and a chart's home
/// is the Cover's answer (16.11) over the invariant and magnitude the
/// kernel holds. Recorded in `lapse_docket`, not guessed at here.
pub(crate) fn withholding_lambda(mesh: &Mesh, router: &Mutex<Router>, addr: &[u8]) -> Option<u64> {
    let home = {
        let r = router.lock().ok()?;
        r.closest(addr).map(|(id, _)| id)?
    };
    let floor = rtt_floor(mesh, &home)?;
    Some(mycelium::physics::from_transport(
        floor,
        mycelium::physics::aperture_cycle() as u64,
    ))
}

/// A measured round trip in strokes enters the peer's window.
pub(crate) fn note_rtt_ok(mesh: &Mesh, id: impl AsRef<[u8]>, sample: u64) {
    let id = id.as_ref();
    let n = mycelium::physics::aperture_cycle().max(1) as usize;
    if let Ok(mut w) = mesh.rtt_window.lock() {
        let mut ring = w.get(id).cloned().unwrap_or_default();
        ring.push(sample);
        if ring.len() > n {
            ring.remove(0);
        }
        w.insert(id, ring);
    }
    let cap = rtt_floor(mesh, id)
        .map(mycelium::physics::persist_dwell)
        .unwrap_or_else(|| mycelium::physics::persist_dwell(sample));
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
    let cap = mycelium::physics::persist_dwell(floor);
    if let Ok(mut m) = mesh.rtt.lock() {
        let Some(cur) = m.get(id).copied() else {
            return;
        };
        m.insert(id, cur.saturating_mul(ratio as u64).min(cap));
    }
}
