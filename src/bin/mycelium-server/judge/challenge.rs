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
    let body = match door.kernel.lock() {
        Ok(mut k) => match request {
            ChallengeRequest::Signed(c) => encode_challenge_reply(&k.challenge(&c)),
            ChallengeRequest::Finality(fold) => encode_finality_reading(&FinalityReading {
                finality: k.finality(&fold),
                bonded: k.fold_bonded(&fold),
                toll_held: k.fold_holds_toll(&fold),
            }),
            ChallengeRequest::Obstructions(domain) => encode_obstructions_reply(&k.open_obstructions(&domain)),
            ChallengeRequest::LineHead(tier) => {
                let (head, at) = k.line_head(&tier);
                // 78.133 — the μ held toward it in toll escrow; a sum past its
                // carrier is answered with nothing, never an invented value.
                let Some(held) = k.toll_held_for(&tier) else { return };
                encode_line_head_reply(&LineHead { head, at, held })
            }
        },
        Err(_) => return,
    };
    // 78.137 — acknowledged only once durable.
    if !crate::persist::durable(&door.kernel, &door.state_path) {
        return;
    }
    answer(stream, Frame { lane: frame.lane, body });
}
