// 78.93 B1 — ACT_WALLET: a party's account at this court, read off the
// treasury as the court stroke last published it (78.151), never the court.
// 78.154 (4) — or a STAKE act, a write, taken under the court.
//
//! A reading: nothing moves, no receipt is written. The account carries the
//! head the reading stands at — its height. An account that cannot be
//! measured is not answered — never answered as an empty one.

use crate::context::Context;
use crate::transport::answer;
use mycelium::anastome::dialogue::lease::{decode_close, decode_lease, encode_lease_reply};
use mycelium::anastome::dialogue::wallet::{decode_stake, decode_wallet, encode_stake_reply, encode_wallet_reply};
use mycelium::network::wire::Delimited;
use mycelium::network::channel::Channel;

pub(crate) fn judge(stream: &mut Channel, delimited: &Delimited, context: &Context) {
    // MU-ECONOMY §5.2 — a compute lease opened or closed: a write, taken
    // under the court and acknowledged only once durable, as a stake is.
    let lease = match (decode_lease(&delimited.body), decode_close(&delimited.body)) {
        (Some(open), _) => Some(crate::court_lock::held!(context.court).map(|mut h| encode_lease_reply(&h.run(|k, _| k.open_lease(&open))))),
        (None, Some(close)) => Some(crate::court_lock::held!(context.court).map(|mut h| encode_lease_reply(&h.run(|k, _| k.close_lease(&close))))),
        (None, None) => None,
    };
    if let Some(reply) = lease {
        let Ok(body) = reply else { return };
        if !crate::persist::durable(&context.court, &context.state_path) {
            return;
        }
        answer(stream, Delimited { body });
        return;
    }
    if let Some(s) = decode_stake(&delimited.body) {
        let body = match crate::court_lock::held!(context.court) {
            // Reads the staker's own claims (a fibre), constant per claim.
            Ok(mut h) => encode_stake_reply(&h.run(|k, _| k.stake(&s))),
            Err(_) => return,
        };
        // 78.137 — acknowledged only once durable.
        if !crate::persist::durable(&context.court, &context.state_path) {
            return;
        }
        answer(stream, Delimited { body });
        return;
    }
    let Some(who) = decode_wallet(&delimited.body) else { return };
    mycelium::observe::note_serve(mycelium::observe::ServeAxis::ReadsPublished, 1);
    let Some(a) = context.published.take().wallet(&who) else { return };
    answer(stream, Delimited { body: encode_wallet_reply(&a) });
}
