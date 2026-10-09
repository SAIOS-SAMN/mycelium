// 78.125 — ACT_CHALLENGE: a sealed fold challenged within its window — a
// demand, a replay, a default, a correction (`Kernel::challenge`) — answered
// by the receipt or the refusal by name; or, sealing nothing, a fold's
// finality and a domain's open arbitrages read.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::challenge::{
    decode_challenge_request, encode_challenge_reply, encode_finality_reading, encode_line_head_reply, encode_obstructions_reply,
    ChallengeRequest, FinalityReading, LineHead,
};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(request) = decode_challenge_request(&frame.body) else { return };
    // The reads stay on the kernel (78.153b): finality, obstructions, tolls
    // and line heads are not on the shelf yet, and the domain acts are 78.154's.
    let body = match crate::held::held!(door.kernel) {
        Ok(mut h) => {
            let body = h.run(|k, _| match request {
                ChallengeRequest::Signed(c) => Some(encode_challenge_reply(&k.challenge(&c))),
                ChallengeRequest::Finality(fold) => Some(encode_finality_reading(&FinalityReading {
                    finality: k.finality(&fold),
                    bonded: k.fold_bonded(&fold),
                    toll_held: k.fold_holds_toll(&fold),
                })),
                ChallengeRequest::Obstructions(domain) => Some(encode_obstructions_reply(&k.open_obstructions(&domain))),
                ChallengeRequest::LineHead(tier) => {
                    let (head, at) = k.line_head(&tier);
                    // 78.133 — the μ held toward it in toll escrow; a sum past
                    // its carrier is answered with nothing, never an invented
                    // value.
                    let held = k.toll_held_for(&tier)?;
                    Some(encode_line_head_reply(&LineHead { head, at, held }))
                }
            });
            let Some(body) = body else { return };
            body
        }
        Err(_) => return,
    };
    // 78.137 — acknowledged only once durable.
    if !crate::persist::durable(&door.kernel, &door.state_path) {
        return;
    }
    answer(stream, Frame { lane: frame.lane, body });
}
