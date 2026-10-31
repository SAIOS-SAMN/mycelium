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
fn answer_refuses_frame_over_budget_writing_nothing() {
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
fn answer_writes_frame_within_budget() {
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


/// A PAGE'S WINDOW IS THE ORDER AFTER A VOICE'S OWN CURSOR, not a copy of
/// the book (checklist (j)), as many as the wire's count carries; what of it
/// is served is the court's reserve and the frame's to say
/// (`Court::serve_syllabus`). Each voice has its own cursor.
#[test]
fn syllabus_window_follows_each_voice_cursor() {
    let mut sy = Syllabus { order: Vec::new(), at: Keyed::new(), served: Keyed::new() };
    let book: Vec<Vec<u8>> = (0u16..300).map(|i| i.to_be_bytes().to_vec()).collect();
    syllabus_admit(&mut sy, book.clone());
    assert_eq!(sy.order.len(), 300, "three hundred ruled words stand in the order");

    let a = [0xaau8; 32];
    let (at, window) = crate::syllabus::syllabus_window(&mut sy, &a);
    assert_eq!(at, 0, "a voice with no cursor starts at the beginning");
    assert_eq!(window, book[0..255].to_vec(), "and its window is what one page's count carries");

    // The cursor advances by what the page consumed, and the window follows.
    sy.served.insert(a, 10);
    let (at, window) = crate::syllabus::syllabus_window(&mut sy, &a);
    assert_eq!((at, window), (10, book[10..265].to_vec()), "the next window follows");

    // A second voice is read from its own cursor, not the first's.
    let b = [0xbbu8; 32];
    assert_eq!(crate::syllabus::syllabus_window(&mut sy, &b).0, 0, "each voice has its own cursor");

    // A cursor at or past the end reads an empty window, never a panic.
    sy.served.insert(a, 300);
    assert!(crate::syllabus::syllabus_window(&mut sy, &a).1.is_empty());
    sy.served.insert(a, 999);
    assert!(crate::syllabus::syllabus_window(&mut sy, &a).1.is_empty());
}

/// A court's seed enters the syllabus order once: a ruled address is taught once, in order.
#[test]
fn syllabus_admit_enters_each_address_once_in_order() {
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
