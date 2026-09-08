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


/// A PAGE IS A WINDOW THE STROKE AFFORDS, not a copy of the book. The door
/// cloned the whole ruled order out of the lock on every request —
/// checklist (j), and O(ruled words) per request per peer on a mesh of ten
/// thousand pollers. The page is read from the voice's own cursor, it
/// spends a `Budget` rather than a depth summed out of the four reserve
/// lanes and clamped at eight (law 5, rule 5), it advances only by what was
/// served, and each voice has its own.
#[test]
fn a_syllabus_page_is_the_window_after_a_voice_own_cursor() {
    fn afford(n: usize) -> mycelium::physics::Budget {
        mycelium::physics::Physics::default().stroke_n(n)
    }
    let mut sy = Syllabus { order: Vec::new(), at: Keyed::new(), served: Keyed::new() };
    let book: Vec<Vec<u8>> = (0u8..10).map(|i| vec![i]).collect();
    syllabus_admit(&mut sy, book.clone());
    assert_eq!(sy.order.len(), 10, "ten ruled words stand in the order");

    let a = [0xaau8; 32];
    let (at, page) = crate::syllabus::syllabus_page(&mut sy, &a, &mut afford(3));
    assert_eq!(at, 0, "a voice with no cursor starts at the beginning");
    assert_eq!(page, book[0..3].to_vec(), "and reads what the stroke affords");

    // The cursor advances by what was served, and the next page follows on.
    sy.served.insert(a, at + page.len());
    let (at, page) = crate::syllabus::syllabus_page(&mut sy, &a, &mut afford(3));
    assert_eq!((at, page), (3, book[3..6].to_vec()), "the next window follows");

    // A second voice is served from its own cursor, not the first's.
    let b = [0xbbu8; 32];
    let (at, page) = crate::syllabus::syllabus_page(&mut sy, &b, &mut afford(2));
    assert_eq!((at, page), (0, book[0..2].to_vec()), "each voice has its own cursor");

    // A cursor at the end reads an empty page rather than walking anything.
    sy.served.insert(a, 10);
    let (at, page) = crate::syllabus::syllabus_page(&mut sy, &a, &mut afford(8));
    assert_eq!(at, 10);
    assert!(page.is_empty(), "a voice served to the end is served nothing");
    // And a cursor past the end is an empty page, never a panic.
    sy.served.insert(a, 99);
    assert!(crate::syllabus::syllabus_page(&mut sy, &a, &mut afford(8)).1.is_empty());
    // A page never runs past the order however much budget it is given.
    let c = [0xccu8; 32];
    assert_eq!(crate::syllabus::syllabus_page(&mut sy, &c, &mut afford(100)).1.len(), 10);
    // And a spent stroke serves nothing at all rather than a default page.
    let d = [0xddu8; 32];
    assert!(crate::syllabus::syllabus_page(&mut sy, &d, &mut afford(0)).1.is_empty());
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
