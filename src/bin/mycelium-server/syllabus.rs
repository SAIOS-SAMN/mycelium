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
