// 78.120 — ACT_DOMAIN: a domain founded on this court (`Kernel::admit_domain`),
// answered by the genesis receipt's stamp and chain hash, or the refusal by
// name; 78.127 — or its reserve tier designated (`Kernel::designate_reserve`).
// 78.154 — or a universe's LEDGER entry, its MAP, a TRAVERSE, and the
// reads of each: each act handed the door's quorum-final point (78.152 TU-7), read off
// the shelf outside the kernel; READ-LEDGER served off the published reading
// (78.151), never the kernel.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::atlas::{encode_ledger_page, encode_ledger_reply, encode_map_reading, encode_map_reply};
use mycelium::anastome::dialogue::crossing::encode_traverse_reply;
use mycelium::anastome::dialogue::domain::{decode_domain_request, encode_domain_reply, DomainRequest};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

/// 78.154 — READ-LEDGER off the published reading: `true` where the frame
/// was one and was answered here.
pub(crate) fn serve_ledger(stream: &mut TcpStream, frame: &Frame, door: &Door) -> bool {
    let Some(DomainRequest::ReadLedger { domain, after }) = decode_domain_request(&frame.body) else { return false };
    mycelium::observe::note_serve(mycelium::observe::ServeAxis::ReadsPublished, 1);
    let page = door.published.take().ledger_page(&domain, after);
    answer(stream, Frame { lane: frame.lane, body: encode_ledger_page(&page) });
    true
}

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(request) = decode_domain_request(&frame.body) else { return };
    if serve_ledger(stream, frame, door) {
        return;
    }
    // The final point is read off the shelf before the kernel is taken.
    let fin = crate::tshelf::final_point();
    let body = match crate::held::held!(door.kernel) {
        // 78.153b — on the hold's stroke: a LEDGER entry's ladder pruning
        // spends it; MAP and TRAVERSE read what the frame carries; READ-MAP
        // walks one universe's records (a fiber), owed to the shelf.
        Ok(mut h) => h.run(|k, reads| match request {
            DomainRequest::Genesis(domain) => encode_domain_reply(&k.admit_domain(&domain)),
            DomainRequest::Reserve(r) => encode_domain_reply(&k.designate_reserve(&r)),
            DomainRequest::ReadReserve(domain) => mycelium::anastome::dialogue::domain::encode_reserve_reading(&k.reserve_reading(&domain)),
            DomainRequest::Bond(b) => encode_domain_reply(&k.bond(&b)),
            DomainRequest::ReadBond { tier, .. } => mycelium::anastome::dialogue::domain::encode_bond_reading(&k.bond_of(&tier)),
            DomainRequest::Origin(o) => encode_domain_reply(&k.record_origin(&o)),
            DomainRequest::ReadOrigin(domain) => mycelium::anastome::dialogue::domain::encode_origin_reading(k.origin_of(&domain).as_ref()),
            DomainRequest::State(s) => encode_domain_reply(&k.state(&s)),
            DomainRequest::ReadUniverse(domain) => mycelium::anastome::dialogue::universe::encode_universe_reading(&k.universe_reading(&domain)),
            DomainRequest::Ledger(s) => encode_ledger_reply(&k.ledger_on(&s, fin, reads)),
            DomainRequest::ReadLedger { domain, after } => encode_ledger_page(&k.ledger_page(&domain, after)),
            DomainRequest::Map(s) => encode_map_reply(&k.map(&s, fin)),
            DomainRequest::ReadMap(domain) => encode_map_reading(&k.map_reading(&domain)),
            DomainRequest::Traverse(s) => encode_traverse_reply(&k.traverse(&s, fin)),
            // 78.155b — a manifest granted or refused, and the endowment read.
            DomainRequest::Manifest(s) => mycelium::anastome::dialogue::endowment::encode_grant_reply(&k.manifest(&s)),
            // A reading that passes its carrier is answered with nothing that
            // decodes, never a capped total.
            DomainRequest::ReadEndowment(domain) => {
                k.endowment_reading(&domain).and_then(|r| mycelium::anastome::dialogue::endowment::encode_endowment_reading(&r)).into_iter().flatten().collect()
            }
        }),
        Err(_) => return,
    };
    // 78.137 — acknowledged only once durable.
    if !crate::persist::durable(&door.kernel, &door.state_path) {
        return;
    }
    answer(stream, Frame { lane: frame.lane, body });
}
