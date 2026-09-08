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

/// ONE PAGE FOR ONE VOICE: the addresses standing after that voice's
/// cursor that this stroke's `Budget` affords, and the cursor they start
/// at. A voice with no cursor starts at the beginning of the order.
///
/// The door read this by cloning `sy.order` — every address the court has
/// ever ruled — out of the lock and then walking a window from it.
/// Checklist (j) refuses "any `clone()` of a carrier into two places", and
/// on a mesh of ten thousand polling peers the cost was O(ruled words) per
/// request per peer, copied while the syllabus lock was held. The window
/// is a slice read from the cursor: O(page), never O(cursor) and never
/// O(order).
///
/// THE PAGE IS WHAT A STROKE AFFORDS. Its size was
/// `(1 + (free * 7) / (4 * reserve_max)).min(8)` with
/// `free = reserve.iter().sum()` — law 5's "μ⃗ is never summed … or
/// reduced to a scalar" in one expression, and then rule 5's own named
/// example, `depth.min(8)`, on the result. What the court can afford to do
/// in one stroke is a `Budget`, which is the doctrine's measure of exactly
/// that (rule 10), so the page spends one and takes a word per spend.
pub(crate) fn syllabus_page(
    sy: &mut Syllabus,
    voice: &[u8; 32],
    budget: &mut mycelium::physics::Budget,
) -> (usize, Vec<Vec<u8>>) {
    let at = match sy.served.get(voice) {
        Some(&n) => n,
        None => {
            sy.served.insert(voice, 0);
            0
        }
    };
    let mut page = Vec::new();
    if let Some(rest) = sy.order.get(at..) {
        for addr in rest {
            if !budget.spend() {
                break;
            }
            page.push(addr.clone());
        }
    }
    (at, page)
}
