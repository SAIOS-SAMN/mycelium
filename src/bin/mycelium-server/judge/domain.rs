// 78.120 — ACT_DOMAIN: a domain founded on this court (`Court::admit_domain`),
// answered by the genesis receipt's stamp and chain hash, or the refusal by
// name; 78.127 — or its reserve tier designated (`Court::designate_reserve`).
// 78.154 — or a universe's LEDGER entry, its MAP, a TRAVERSE, and the
// reads of each: each act handed the door's quorum-final point (78.152 TU-7), read off
// the shelf outside the court; READ-LEDGER served off the published reading
// (78.151), never the court.

use crate::context::Context;
use crate::transport::answer;
use mycelium::anastome::dialogue::atlas::{encode_ledger_page, encode_ledger_reply, encode_map_reading, encode_map_reply};
use mycelium::anastome::dialogue::crossing::encode_traverse_reply;
use mycelium::anastome::dialogue::domain::{decode_domain_request, encode_domain_reply, DomainRequest};
use mycelium::network::wire::Delimited;
use mycelium::network::channel::Channel;

/// 78.154 — READ-LEDGER off the published reading: `true` where the frame
/// was one and was answered here.
pub(crate) fn serve_ledger(stream: &mut Channel, frame: &Delimited, context: &Context) -> bool {
    let Some(DomainRequest::ReadLedger { domain, after }) = decode_domain_request(&frame.body) else { return false };
    mycelium::observe::note_serve(mycelium::observe::ServeAxis::ReadsPublished, 1);
    let page = context.published.take().ledger_page(&domain, after);
    answer(stream, Delimited { lane: frame.lane, body: encode_ledger_page(&page) });
    true
}

pub(crate) fn judge(stream: &mut Channel, frame: &Delimited, context: &Context) {
    let Some(request) = decode_domain_request(&frame.body) else { return };
    if serve_ledger(stream, frame, context) {
        return;
    }
    // The final point is read off the shelf before the court is taken.
    let fin = crate::tuniversal::final_point();
    if let DomainRequest::Retrieve(s) = &request {
        retrieve(stream, frame, context, s, fin);
        return;
    }
    let body = match crate::court_lock::held!(context.court) {
        // 78.153b — on the hold's stroke: a LEDGER entry's ladder pruning
        // spends it; MAP and TRAVERSE read what the frame carries; READ-MAP
        // walks one universe's records (a fibre), owed to the shelf.
        Ok(mut h) => h.run(|k, reads| match request {
            DomainRequest::Genesis(domain) => encode_domain_reply(&k.hear_domain(&domain)),
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
            // Answered above, beside the court's signed position.
            DomainRequest::Retrieve(_) => Vec::new(),
        }),
        Err(_) => return,
    };
    // 78.137 — acknowledged only once durable.
    if !crate::persist::durable(&context.court, &context.state_path) {
        return;
    }
    answer(stream, Delimited { lane: frame.lane, body });
}

/// #159 — A COORDINATE RETRIEVAL (`ACT_DOMAIN` sub-kind 19), answered beside
/// the court's own 47.1 section over its head at the answer, signed over the
/// request (`retrieve::answered_payload`): the retriever verifies the
/// section and that the receipts stand under its head. A served answer this
/// court cannot sign is answered with nothing, never unsigned; it is
/// acknowledged only once the claim it drew is durable (78.137).
fn retrieve(stream: &mut Channel, frame: &Delimited, context: &Context, s: &mycelium::anastome::dialogue::retrieve::SignedRetrieve, fin: Option<mycelium::chronometry::tuniversal::TPoint>) {
    use mycelium::anastome::dialogue::retrieve::{answered_payload, encode_retrieve_reply, Retrieved};
    let read = match crate::court_lock::held!(context.court) {
        Ok(mut h) => h.run(|k, _| k.retrieve_signed(s, fin)),
        Err(_) => return,
    };
    let section = match &read {
        Retrieved::Served { .. } => {
            let payload = answered_payload(&s.retrieve);
            let Some(section) = crate::admission::sign_position(&context.court, &context.mesh, &mycelium::transducer::stalk::Invariant::default(), &payload) else {
                eprintln!("  retrieve unanswered: this court holds no position to sign");
                return;
            };
            Some(section)
        }
        Retrieved::Refused(_) => None,
    };
    if !crate::persist::durable(&context.court, &context.state_path) {
        return;
    }
    let Some(body) = encode_retrieve_reply(&read, section.as_ref()) else {
        eprintln!("  retrieve unanswered: a receipt or the section cannot be written");
        return;
    };
    answer(stream, Delimited { lane: frame.lane, body });
}
