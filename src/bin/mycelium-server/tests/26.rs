// 26 — the wire: an act is a frame or it is not written, and a ruled word
// enters the syllabus once, in order.

use crate::syllabus::{syllabus_admit, Syllabus};
use crate::transport::answer;
use mycelium::network::nerve::Keyed;
use mycelium::network::wire::Frame;
use std::net::TcpStream;

/// AN ENCODE FAILURE REFUSES THE FRAME. Twenty-one sites wrote
/// `.encode().unwrap_or_default()`, which sends an EMPTY buffer when a
/// body is over the wire budget. That is not a frame, and the caller
/// waited on five bytes that were never coming: one oversized reply
/// stopped an exchange thread and every act it had left. Nothing is
/// written, and nothing is invented in its place.
#[test]
fn a_frame_over_the_wire_budget_is_refused_and_nothing_is_written() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let reader = std::thread::spawn(move || {
        let (mut s, _) = listener.accept().expect("accept");
        let mut buf = Vec::new();
        std::io::Read::read_to_end(&mut s, &mut buf).expect("read");
        buf
    });

    let mut s = TcpStream::connect(&addr).expect("connect");
    let over = mycelium::network::wire::BODY_BUDGET + 1;
    answer(
        &mut s,
        Frame { lane: mycelium::network::wire::Stratum::Spatial, body: vec![0u8; over] },
    );
    drop(s);
    assert!(reader.join().expect("join").is_empty(), "an unframeable act writes nothing");
}

/// And a frame within the budget is written whole, so the refusal above
/// is not simply a write path that never works.
#[test]
fn a_frame_within_the_budget_is_written_whole() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr").to_string();
    let reader = std::thread::spawn(move || {
        let (mut s, _) = listener.accept().expect("accept");
        let mut buf = Vec::new();
        std::io::Read::read_to_end(&mut s, &mut buf).expect("read");
        buf
    });

    let mut s = TcpStream::connect(&addr).expect("connect");
    let body = vec![7u8, 8, 9];
    answer(
        &mut s,
        Frame { lane: mycelium::network::wire::Stratum::Spatial, body: body.clone() },
    );
    drop(s);
    let got = reader.join().expect("join");
    assert_eq!(&got[5..], &body[..], "the body arrives");
    assert_eq!(
        u32::from_le_bytes([got[1], got[2], got[3], got[4]]) as usize,
        body.len(),
        "with its length"
    );
}


#[test]
fn court_seed_enters_syllabus_order_once() {
    let mut sy = Syllabus {
        order: Vec::new(),
        at: Keyed::new(),
        served: Keyed::new(),
    };
    let a = vec![1u8];
    let b = vec![2u8];
    syllabus_admit(&mut sy, [a.clone(), b.clone()]);
    syllabus_admit(&mut sy, [a.clone()]);
    assert_eq!(sy.order, vec![a.clone(), b.clone()], "a ruled address is taught once, in order");
    assert!(sy.at.contains(&a) && sy.at.contains(&b));
}
