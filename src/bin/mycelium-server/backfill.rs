// 47.2 step 3 — THE BACK-FILL'S WIRE: where a page of a signer's receipts
// is asked, and how. The chase itself is `mycelium::server::backfill`,
// one loop for the server and the tests (ACT1-hold-b).
//
//! A gap is not a refusal: the frame that reported it is admitted again
//! once the chain reaches it, and nothing back-filled is cited as a
//! parent. The pages of one gap are paced by the exchange's own stroke —
//! one spend per page — so a peer whose K outruns one page per ask still
//! closes (5.2g, S13.10a, R4).

use crate::exchange::Mesh;
use crate::transport::tell_peer;
use mycelium::anastome::dialogue;
use mycelium::chronometry::{KStepReceipt, Place};

/// THE PAGE SOURCE OVER THE WIRE: a page of `who`'s receipts strictly
/// after `after` up to `upto`, asked of the signer that owns the chain.
///
/// ASK THE SIGNER THAT OWNS THE CHAIN, not the one that mentioned it. Every
/// frontier entry went to `dest` — the peer whose digest reported the gap —
/// so when a receipt of a cited a receipt of b, c asked A for B'S chain, and
/// a serves another signer's receipts only as far as it happens to have
/// admitted them itself. Measured on the fleet at epoch 11: `backfill ce02…
/// 76..76 admitted 0 cites 244b… K 75`, repeating, the chase never closing
/// because the question was put to a node that could not answer it.
///
/// The mesh already knows where each signer lives (`addrs`, written by
/// `admit_peer_digest`), so the fetch is routed there. `dest` remains the
/// fallback for a signer this node has no address for — a proxy hop is
/// worse than the direct ask and better than none. `None` where the peer
/// did not answer, deferred, or sent no page.
pub(crate) fn fetch<'a>(
    mesh: &'a Mesh,
    dest: &'a str,
) -> impl FnMut(&[u8; 32], Option<Place>, Place) -> Option<Vec<KStepReceipt>> + 'a {
    move |who: &[u8; 32], after: Option<Place>, upto: Place| {
        let at = match mesh.addrs.lock() {
            Ok(a) => a.get(who).cloned(),
            Err(_) => None,
        };
        let to_ask: &str = match &at {
            Some(a) => a.as_str(),
            None => dest,
        };
        let req = dialogue::encode_receipts_request(&mesh.vk, who, after, upto);
        let (reply, _) = tell_peer(to_ask, req)?;
        if crate::transport::is_deferred(&reply) {
            return None;
        }
        dialogue::decode_receipts_reply(&reply)
    }
}
