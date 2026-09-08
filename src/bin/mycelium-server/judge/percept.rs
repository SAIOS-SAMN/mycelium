// ACT_PERCEPT, ACT_DEMAND and ACT_GOSSIP — the language moves: a percept
// the client already reduced, a demand replayed against the book, and the
// rendezvous a still-unwitnessed percept is routed to.
//
//! The client PERCEIVED and REDUCED; the judge REPLAYS the proof. A demand
//! carries the speaker's own reduction and is replayed the same way. What
//! the outcome teaches and settles is read from the docket's standing of
//! the cited word, never from a sentence's length.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;
use crate::door::{demand_cover_home, rendezvous};
use crate::persist::persist_if_due;
use mycelium::cognition::{decode_percept, outcome_code, Outcome};


/// Replay a percept, a demand, or a gossiped percept, and answer with the
/// outcome, the book height, the metabolism and what it taught (21, 26).
pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door, kind: u8, arrived_routed: bool) {
    let kernel = &door.kernel;
    let id = &door.id;
    let mesh = &door.mesh;
    let Some((_, rest)) = frame.body.split_first() else { return };

let pbytes = if kind == dialogue::ACT_GOSSIP {
    let mut p = 0usize;
    let _ = mycelium::transducer::address::take_addr(rest, &mut p);
    rest.get(p..).unwrap_or(&[])
} else {
    rest
};
if kind == dialogue::ACT_DEMAND && !arrived_routed {
    if let Some(dialogue::DemandAct { chain, .. }) = dialogue::decode_demand(&frame.body) {
        let (home_vk, home_addr) = demand_cover_home(mesh, kernel, &chain);
        if home_vk != mesh.vk && home_addr != mesh.bind && !home_addr.is_empty() {
            let _ = door.nerve_tx.try_send((home_addr, dialogue::encode_routed_demand(id, &frame.body)));
        }
    }
}
let (out, voice) = if kind == dialogue::ACT_DEMAND {
    // A spoken coboundary with the speaker's own reduction — replayed.
    let Some(dialogue::DemandAct { voice: dvoice, chain, cites, k: k_voice }) =
        dialogue::decode_demand_at(&frame.body)
    else {
        return;
    };
    (mycelium::synthesis::attest(kernel, &chain, dvoice, &cites, k_voice), dvoice)
} else {
    // The client PERCEIVED and REDUCED; the JUDGE replays the proof.
    let Some(mycelium::cognition::Percept { voice: pvoice, a: a_bodies, b: b_bodies, frame, proof }) =
        decode_percept(pbytes)
    else {
        return;
    };
    (
        mycelium::synthesis::adjudicate(kernel, &a_bodies, &b_bodies, pvoice, &frame, &proof),
        pvoice,
    )
};
// TEACH by the docket's standing of the cited word (anchored → taught),
// never by sentence length. SETTLE: the consilience receipt.
let (basis, flux, reserve, taught, settle, height) = {
    let k = kernel.lock().unwrap();
    let (taught, settle) = match &out {
        Outcome::Installed(i) => match k.basis().get(*i) {
            Some(g) => {
                let addr = g.address();
                let mu = mycelium::economy::price(&g.magnitude(), &k.physics().pressure);
                (g.to_bytes(), Some((addr.clone(), k.voices_of(addr.as_bytes()), mu)))
            }
            None => (Vec::new(), None),
        },
        Outcome::Decomposed(word) => (k.taught_anchored(word), None),
        _ => (Vec::new(), None),
    };
    (
        (k.basis_len(), k.sections_total()),
        k.physics().metabolism_rate(),
        k.physics().reserve,
        taught,
        settle,
        k.book_height(),
    )
};
let label = match &out {
    Outcome::Decomposed(word) => format!("valid: decomposed → sentence {word:?} ({} generators)", word.len()),
    Outcome::GermPending => "valid: docketed, awaiting witness".to_string(),
    Outcome::Installed(i) => format!("valid: GERM INSTALLED #{i}"),
    Outcome::Sectioned(i) => format!("valid: section of #{i}"),
    Outcome::Fragmented(parts) => format!("valid: fragmented ({} bodies)", parts.len()),
    Outcome::Pivot => "invalid: pivot (degenerate — nothing to reduce)".to_string(),
};
let src = match kind {
    k if k == dialogue::ACT_GOSSIP => "gossip",
    k if k == dialogue::ACT_DEMAND => "demand",
    _ => "client",
};
eprintln!(
    "  {src} percept voice {:02x} → {label}; basis={} vol={}; metabolism {flux:?}; reserve {reserve:?}",
    voice[0], basis.0, basis.1
);
persist_if_due(kernel, &door.state_path);
// Reply: outcome | height u64 | pressure 4×u64 | taught (len16 | bytes) | settle?
let mut body = dialogue::encode_outcome_reply(outcome_code(&out), height, &flux, &taught);
if let Some((germ, voices, mu)) = &settle {
    body.extend_from_slice(&dialogue::encode_settle(germ, voices, mu));
}
let reply = Frame { lane: frame.lane, body };
answer(stream, reply);

// P6/R5 · sun_recon: a CLIENT percept still awaiting consilience routes ONE
// receipt to its RENDEZVOUS — the node XOR-closest to the demand's
// content-address (voice EXCLUDED so all voices of a germ converge there).
// The rendezvous accumulates the distinct voice + verifies by recomputation;
// quorum → install + teach outward. No fanout; a down rendezvous fails
// silently (resilience). Gossip (kind 1) is never re-routed here.
if kind == dialogue::ACT_PERCEPT
    && pbytes.len() > 32
    && matches!(out, Outcome::GermPending | Outcome::Fragmented(_))
{
    let Some(mycelium::cognition::Percept { a: a_bodies, b: b_bodies, frame: extent, .. }) = decode_percept(pbytes)
    else {
        return;
    };
    let fa: Vec<(i64, i64, u8)> = a_bodies.into_iter().flatten().collect();
    let fb: Vec<(i64, i64, u8)> = b_bodies.into_iter().flatten().collect();
    let chain = mycelium::transducer::germinate::germinate(&fa, &fb).unwrap_or_default();
    let key = mycelium::transducer::element::listing_key(&chain, &extent);
    let dest = door.router.lock().ok().and_then(|r| rendezvous(key.as_bytes(), id, &r));
    if let Some(dest) = dest {
        let mut g = vec![0x01u8];
        let _ = mycelium::transducer::address::put_addr(&mut g, &key);
        g.extend_from_slice(pbytes);
        if let Ok(mut ps) = TcpStream::connect(&dest) {
            answer(&mut ps, Frame { lane: frame.lane, body: g });
        }
    }
}
}
