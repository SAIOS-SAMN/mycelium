// 78.119 — ACT_UPLINK: a kernel's settlement tier sends up its fold, and this
// court — the verifier outside the tier's closure — reconstructs it and
// admits it onto its line (`Court::hear_uplink`, `Court::admit_uplink`),
// answered by the receipt's stamp and chain hash, or
// the refusal by name.

use crate::context::Context;
use crate::transport::answer;
use mycelium::anastome::dialogue::uplink::{decode_uplink, encode_uplink_reply};
use mycelium::network::wire::Delimited;
use mycelium::network::channel::Channel;

pub(crate) fn judge(stream: &mut Channel, frame: &Delimited, context: &Context) {
    let Some(fold) = decode_uplink(&frame.body) else { return };
    let verdict = match crate::court_lock::held!(context.court) {
        Ok(mut h) => h.run(|k, _| k.hear_uplink(&fold)),
        Err(_) => return,
    };
    // 78.137 — acknowledged only once durable.
    if !crate::persist::durable(&context.court, &context.state_path) {
        return;
    }
    answer(stream, Delimited { lane: frame.lane, body: encode_uplink_reply(&verdict) });
}
