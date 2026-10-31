// 26 — THE SYLLABUS: every court-ratified word in ruling order, with a
// cursor per enrolled voice so the stream is served exactly once on contact.

use mycelium::network::nerve::Keyed;

/// Shared door state: the listener only accepts and try_sends; every act
/// that needs the kernel runs on a reflex worker, snapshot / off-lock /
/// re-lock, never on the accept thread.
pub(crate) struct Syllabus {
    pub(crate) order: Vec<Vec<u8>>,
    pub(crate) at: Keyed<()>,
    pub(crate) served: Keyed<usize>,
}

/// Push newly ruled addresses into the served stream. Court_stroke admits
/// onto the ruled cover; this is the teaching organ that drip feeds them.
pub(crate) fn syllabus_admit(sy: &mut Syllabus, addrs: impl IntoIterator<Item = impl AsRef<[u8]>>) {
    for addr in addrs {
        let addr = addr.as_ref();
        if !sy.at.contains(addr) {
            sy.at.insert(addr, ());
            sy.order.push(addr.to_vec());
        }
    }
}

/// ONE WINDOW FOR ONE VOICE: the addresses standing after that voice's
/// cursor, as many as one page's wire count carries, and the cursor they
/// start at. A voice with no cursor starts at the beginning of the order.
/// What of the window is SERVED is the court's to say — the page ends at
/// the frame's bytes or where its reserve cannot fund the next word
/// (`Court::serve_syllabus`) — never a count of reads.
///
/// The window is a slice read from the cursor: O(page), never O(cursor)
/// and never O(order) (checklist (j): no clone of the order).
pub(crate) fn syllabus_window(sy: &mut Syllabus, voice: &[u8; 32]) -> (usize, Vec<Vec<u8>>) {
    let at = match sy.served.get(voice) {
        Some(&n) => n,
        None => {
            sy.served.insert(voice, 0);
            0
        }
    };
    let window = match sy.order.get(at..) {
        Some(rest) => rest.iter().take(usize::from(u8::MAX)).cloned().collect(),
        None => Vec::new(),
    };
    (at, window)
}
