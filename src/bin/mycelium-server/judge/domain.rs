// 78.120 — ACT_DOMAIN: a domain founded on this court (`Kernel::admit_domain`),
// answered by the genesis receipt's stamp and chain hash, or the refusal by
// name; 78.127 — or its reserve tier designated (`Kernel::designate_reserve`).

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::domain::{decode_domain_request, encode_domain_reply, DomainRequest};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(request) = decode_domain_request(&frame.body) else { return };
    let body = match door.kernel.lock() {
        Ok(mut k) => match request {
            DomainRequest::Genesis(domain) => encode_domain_reply(&k.admit_domain(&domain)),
            DomainRequest::Reserve(r) => encode_domain_reply(&k.designate_reserve(&r)),
            DomainRequest::ReadReserve(domain) => mycelium::anastome::dialogue::domain::encode_reserve_reading(&k.reserve_reading(&domain)),
            DomainRequest::Bond(b) => encode_domain_reply(&k.bond(&b)),
            DomainRequest::ReadBond { tier, .. } => mycelium::anastome::dialogue::domain::encode_bond_reading(&k.bond_of(&tier)),
            DomainRequest::Origin(o) => encode_domain_reply(&k.record_origin(&o)),
            DomainRequest::ReadOrigin(domain) => mycelium::anastome::dialogue::domain::encode_origin_reading(k.origin_of(&domain).as_ref()),
        },
        Err(_) => return,
    };
    answer(stream, Frame { lane: frame.lane, body });
}
