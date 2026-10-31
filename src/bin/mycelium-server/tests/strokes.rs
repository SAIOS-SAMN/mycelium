// 45.2, law 9 — the court has no clock but the ledger, and the transport
// is bounded in strokes.

use crate::transport::tell_peer_within;


/// A PEER THAT ACCEPTS AND SAYS NOTHING IS REFUSED, in strokes. This
/// waited forever: `read_exact` on a blocking socket with no bound, and
/// one such peer stopped the exchange thread and every act it had left.
/// The bound is a budget, not a duration — a socket cannot be handed a
/// stroke, so the read pays one per attempt that finds nothing and
/// refuses when the transport's deadline is spent (law 9).
#[test]
fn tell_peer_within_refuses_silent_peer_within_budget() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    // Accept and hold the connection open, writing nothing at all.
    let held = std::thread::spawn(move || listener.accept().map(|(s, _)| s));

    let out = tell_peer_within(&addr, vec![0x10], 1);
    assert!(out.is_none(), "silence is a refusal, not an answer");
    drop(held.join());
}

/// A PEER THAT ANSWERS IS READ WHOLE, AND ITS ROUND TRIP IS COUNTED IN
/// STROKES (45.4, W9b). The transport returned a literal 1 for every reply,
/// so no rtt window ever held a measurement and Λ could not move. The
/// measure is the polls this court made between the write and the first
/// byte back, the poll that read it included: at least one stroke for any
/// reply, never zero and never a floor. The deadline the caller passes is
/// this host's own reserve, a measurement, so a responder that has merely
/// not been scheduled is not refused.
#[test]
fn tell_peer_within_reads_reply_and_counts_strokes() {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let echo = vec![0x21u8, 7, 8, 9];
    let reply_body = echo.clone();
    let responder = std::thread::spawn(move || {
        let (mut s, _) = listener.accept().expect("accept");
        let mut head = [0u8; 5];
        s.read_exact(&mut head).expect("head");
        let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
        let mut body = vec![0u8; len];
        s.read_exact(&mut body).expect("body");
        let frame = mycelium::network::wire::Frame {
            lane: mycelium::network::wire::Stratum::Spatial,
            body: reply_body,
        };
        s.write_all(&frame.encode().expect("encode")).expect("write");
        body
    });

    // The deadline is this test's patience for the responder thread to be
    // scheduled, not the measure: a poll is a yield, and on a host running a
    // whole deployment at full CPU one reserve of yields passed before the
    // responder ran once (read on 2026-09-08 with eight processes live). The
    // patience is the host's own reserve squared — a measurement, never a
    // literal — and the rtt read back is whatever it took.
    let reserve = mycelium::physics::reserve_max();
    let patience = reserve
        .checked_mul(reserve)
        .expect("the host's reserve squared fits the clock this test runs on");
    let out = tell_peer_within(&addr, vec![0x10, 1, 2], patience);
    let (reply, rtt) = out.expect("a peer that answers is not silence");
    assert_eq!(reply, echo, "the reply is read whole");
    assert!(rtt >= 1, "a round trip takes at least the one poll that read it: {rtt}");
    assert_eq!(responder.join().expect("join"), vec![0x10, 1, 2], "and the act arrived whole");
}
