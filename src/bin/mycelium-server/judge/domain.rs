// 78.120 — ACT_DOMAIN: a domain founded on this court (`Kernel::admit_domain`),
// answered by the genesis receipt's stamp and chain hash, or the refusal by
// name.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::domain::{decode_domain, encode_domain_reply};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(domain) = decode_domain(&frame.body) else { return };
    let founded = match door.kernel.lock() {
        Ok(mut k) => k.admit_domain(&domain),
        Err(_) => return,
    };
    answer(stream, Frame { lane: frame.lane, body: encode_domain_reply(&founded) });
}
