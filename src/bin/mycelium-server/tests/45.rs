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
