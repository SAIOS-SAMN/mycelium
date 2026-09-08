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
fn a_peer_that_never_answers_is_refused_within_its_transport_budget() {
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
/// measure is the polls this kernel made between the write and the first
/// byte back, the poll that read it included: at least one stroke for any
/// reply, never zero and never a floor. The deadline the caller passes is
/// this host's own reserve, a measurement, so a responder that has merely
/// not been scheduled is not refused.
#[test]
fn a_peer_that_answers_is_read_whole_and_its_round_trip_is_counted_in_strokes() {
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

    let out = tell_peer_within(&addr, vec![0x10, 1, 2], mycelium::physics::reserve_max());
    let (reply, rtt) = out.expect("a peer that answers is not silence");
    assert_eq!(reply, echo, "the reply is read whole");
    assert!(rtt >= 1, "a round trip takes at least the one poll that read it: {rtt}");
    assert_eq!(responder.join().expect("join"), vec![0x10, 1, 2], "and the act arrived whole");
}
