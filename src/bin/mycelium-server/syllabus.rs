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

/// ONE PAGE FOR ONE VOICE: the `depth` addresses standing after that
/// voice's cursor, and the cursor they start at. A voice with no cursor
/// starts at the beginning of the order.
///
/// The door read this by cloning `sy.order` — every address the court has
/// ever ruled — out of the lock and then walking a window of at most
/// `depth` entries from it. Checklist (j) refuses "any `clone()` of a
/// carrier into two places", and on a mesh of ten thousand polling peers
/// the cost was O(ruled words) per request per peer, copied while the
/// syllabus lock was held, to read at most eight. The window is a slice
/// read from the cursor: O(depth), and it never materialises the book.
pub(crate) fn syllabus_page(
    sy: &mut Syllabus,
    voice: &[u8; 32],
    depth: usize,
) -> (usize, Vec<Vec<u8>>) {
    let at = match sy.served.get(voice) {
        Some(&n) => n,
        None => {
            sy.served.insert(voice, 0);
            0
        }
    };
    let page = match sy.order.get(at..) {
        Some(rest) => rest.iter().take(depth).cloned().collect(),
        None => Vec::new(),
    };
    (at, page)
}
