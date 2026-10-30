// 78.146 amended, MU-CLIENT §7.1 — A READ HELD TO THE COURT'S NEXT EVENT (ruled
// 2026-10-17): a seat reader's mail read with nothing filed past
// its cursor is answered not at once but at the court's next event — a post
// filed on the relay, or the keeper signing its next TUniversal reading — so
// a resting mind blocks on its own landing and the court is its clock. Observed
// on the development deployment, 2026-10-18 04:30: each resting mind read an empty box ~120
// times a second, every read under the kernel's lock, court a at 98%.
//
//! ONE HELD READ PER READER, by its key: a reader that asks again while held
//! has its earlier read answered at once, so the hold is bounded by the
//! readers, never by asks. The held reads are answered on their own thread,
//! through the relay's ordinary read (`judge::seat::judge`), never on a judge
//! worker, so the judging pool is never held by a wait. No clock is read:
//! the court's own events wake them.

use crate::door::Door;
use mycelium::anastome::dialogue::seat::{decode_seat_request, SeatRequest};
use mycelium::network::nerve::Keyed;
use mycelium::network::wire::Frame;
use std::net::TcpStream;
use std::sync::{Condvar, LazyLock, Mutex};

/// THE HELD READS AND THE COURT'S EVENTS: the reads waiting, by reader, and
/// how many events the court has had.
struct Held {
    reads: Keyed<(TcpStream, Frame)>,
    events: u64,
}

static HELD: LazyLock<(Mutex<Held>, Condvar)> = LazyLock::new(|| (Mutex::new(Held { reads: Keyed::new(), events: 0 }), Condvar::new()));

/// THE COURT HAD AN EVENT — a post filed on the relay, a TUniversal reading
/// signed: every held read is answered.
pub(crate) fn stir() {
    let (lock, woken) = &*HELD;
    if let Ok(mut h) = lock.lock() {
        h.events = h.events.wrapping_add(1);
        woken.notify_all();
    }
}

/// A SEAT READ, OFFERED TO BE HELD: `None` where it was taken (held, or —
/// a read of the same reader already held — answered the earlier one in its
/// place); the stream and frame back where it is no mail read, or something
/// was filed past its cursor, to be answered now as before.
pub(crate) fn offer(stream: TcpStream, frame: Frame, door: &Door) -> Option<(TcpStream, Frame)> {
    let Some(Ok(SeatRequest::ReadMail(r))) = decode_seat_request(&frame.body) else { return Some((stream, frame)) };
    if !r.verified() || door.seat_relay.filed_past(&r) {
        return Some((stream, frame));
    }
    let (lock, _) = &*HELD;
    let earlier = match lock.lock() {
        Ok(mut h) => {
            let earlier = h.reads.remove(&r.reader);
            h.reads.insert(r.reader, (stream, frame));
            earlier
        }
        Err(_) => return None,
    };
    if let Some((mut s, f)) = earlier {
        crate::judge::seat::judge(&mut s, &f, door);
    }
    None
}

/// THE THREAD THAT ANSWERS THE HELD READS at each event of the court.
pub(crate) fn spawn(door: Door) {
    std::thread::spawn(move || {
        let (lock, woken) = &*HELD;
        let mut seen = 0u64;
        loop {
            let reads = {
                let Ok(mut h) = lock.lock() else { return };
                while h.events == seen {
                    h = match woken.wait(h) {
                        Ok(g) => g,
                        Err(_) => return,
                    };
                }
                seen = h.events;
                let readers: Vec<Vec<u8>> = h.reads.scoped().map(|(k, _)| k).collect();
                readers.iter().filter_map(|k| h.reads.remove(k)).collect::<Vec<_>>()
            };
            for (mut stream, frame) in reads {
                crate::judge::seat::judge(&mut stream, &frame, &door);
            }
        }
    });
}
