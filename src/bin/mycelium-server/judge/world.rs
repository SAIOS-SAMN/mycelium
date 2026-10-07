// 78.120 — ACT_WORLD: a world founded on this court (`Kernel::admit_world`),
// answered by the genesis receipt's stamp and chain hash, or the refusal by
// name.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::world::{decode_world, encode_world_reply};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(world) = decode_world(&frame.body) else { return };
    let founded = match door.kernel.lock() {
        Ok(mut k) => k.admit_world(&world),
        Err(_) => return,
    };
    answer(stream, Frame { lane: frame.lane, body: encode_world_reply(&founded) });
}
