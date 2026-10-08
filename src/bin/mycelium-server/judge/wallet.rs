// 78.93 B1 — ACT_WALLET: a party's account at this court, read off the
// kernel's treasury at its head (`Kernel::wallet`).
//
//! A reading: nothing moves, no receipt is written. An account that cannot
//! be measured is not answered — never answered as an empty one.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::wallet::{decode_wallet, encode_wallet_reply};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(who) = decode_wallet(&frame.body) else { return };
    let account = match crate::held::held!(door.kernel) {
        Ok(k) => k.wallet(&who),
        Err(_) => return,
    };
    let Some(a) = account else { return };
    answer(stream, Frame { lane: frame.lane, body: encode_wallet_reply(&a) });
}
