// 78.93 B1 — ACT_WALLET: a party's account at this court, read off the
// treasury as the court stroke last published it (78.151), never the kernel.
//
//! A reading: nothing moves, no receipt is written. The account carries the
//! head the reading stands at — its height. An account that cannot be
//! measured is not answered — never answered as an empty one.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::wallet::{decode_wallet, encode_wallet_reply};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(who) = decode_wallet(&frame.body) else { return };
    mycelium::observe::note_serve(mycelium::observe::ServeAxis::ReadsPublished, 1);
    let Some(a) = door.published.take().wallet(&who) else { return };
    answer(stream, Frame { lane: frame.lane, body: encode_wallet_reply(&a) });
}
