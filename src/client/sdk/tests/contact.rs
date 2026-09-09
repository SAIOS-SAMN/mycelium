// 39, A10 — the client's contact with the mesh: the impedance it measures,
// the aperture that derives from it, and the route it takes.

use crate::client::sdk::*;

/// Transport impedance is perceived, not thrown: failed exchanges raise
/// Z monotonically, the aperture derives Impeded then Occluded at the
/// ratio-law boundary, and one successful exchange restores Open.
#[test]
fn impedance_is_measured_and_the_aperture_derives() {
// A session pointed at a port nothing listens on: every exchange
// fails, so resistance accumulates deterministically.
let s = Session::open("127.0.0.1:1", Kernel::named("impedance-test"));
assert_eq!(s.transport(), TransportAperture::Open, "no speech yet — presumed clear");
let cycle = crate::physics::aperture_cycle() as u64;
for i in 1..=cycle {
    let _ = s.speak(vec![0xFF]);
    assert_eq!(s.impedance(), i, "each failure is one unit of measured Z");
}
assert!(
    matches!(s.transport(), TransportAperture::Occluded { z } if z == cycle),
    "a full stroke-cycle of failures reads as occlusion"
);
// Dwell is bounded by one aperture; Z is a routing coordinate.
assert_eq!(s.impedance(), cycle);

// ONE OCCLUDED WIRE IS NOT AN OCCLUDED MIND. `impedance` was a `max`
// over every wire the session had touched (rule 4), so a peer that had
// gone quiet made the mind read itself as occluded on the wires that
// were answering, and `transport` -- which paces the whole stroke --
// read it that way too. Z is per wire, and the wires are not one wire.
assert_eq!(s.z_at("127.0.0.1:2"), 0, "a wire never spoken on carries no Z");
assert_eq!(
    s.impedance(),
    s.z_at("127.0.0.1:1"),
    "the session's own impedance is the wire it is bound to"
);
assert!(!s.occluded_at("127.0.0.1:2"), "and another wire is not occluded by this one");
}

/// A successful exchange with any hypha decays Z on every path, so a
/// parked voice unsticks when the next hypha answers.
#[test]
fn impedance_decays_on_any_successful_exchange() {
use crate::network::wire::{Frame, Stratum};
use std::io::{Read, Write};
use std::net::TcpListener;
let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
let live = listener.local_addr().expect("addr").to_string();
let h = std::thread::spawn(move || {
    let Ok((mut s, _)) = listener.accept() else { return };
    let mut head = [0u8; 5];
    if s.read_exact(&mut head).is_err() {
        return;
    }
    let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
    let mut body = vec![0u8; len.min(1 << 16)];
    let _ = s.read_exact(&mut body);
    let reply = Frame { lane: Stratum::Spatial, body: vec![0] }.encode().unwrap_or_default();
    let _ = s.write_all(&reply);
});
let mut s = Session::open("127.0.0.1:1", Kernel::named("z-decay"));
let cycle = crate::physics::aperture_cycle() as u64;
for _ in 0..cycle {
    let _ = s.speak(vec![0xFF]);
}
assert!(
    matches!(s.transport(), TransportAperture::Occluded { .. }),
    "the seed is occluded"
);
s.add_peer(&live);
let _ = s.speak_nerve(vec![0xFF]);
assert!(
    s.impedance() < cycle,
    "a success on the next hypha decays Z, got {}",
    s.impedance()
);
let _ = h.join();
}

/// An empty cover routes by XOR-closest chart key over known hyphae,
/// not the bootstrap seed.
#[test]
fn empty_cover_routes_by_chart_key_over_known_hyphae() {
let mut s = Session::open("127.0.0.1:7500", Kernel::named("fan"));
s.know_hypha("127.0.0.1:7501", Kernel::named("p1").voice());
s.know_hypha("127.0.0.1:7502", Kernel::named("p2").voice());
let seed = "127.0.0.1:7500";
let mut saw_other = false;
for i in 0..64u64 {
    let inv = crate::transducer::stalk::Invariant(vec![i, i.wrapping_mul(3), 2]);
    let dest = s.route_chart(&inv, &[1, 0, 0, 0]);
    let closest = s.route(inv.key().as_bytes()).expect("known hyphae");
    assert_eq!(dest, closest, "empty cover is router.closest(chart key)");
    if dest != seed {
        saw_other = true;
    }
}
assert!(saw_other, "chart keys fan out across the known hyphae, not only the seed");
}

/// A kernel's identity is stable and name-derived; distinct names → distinct
/// voices (so two named kernels can reach consilience, one cannot with itself).
#[test]
fn kernel_identity_is_stable_and_distinct() {
assert_eq!(Kernel::named("sod").voice(), Kernel::named("sod").voice(), "stable seat");
assert_ne!(Kernel::named("sod").voice(), Kernel::named("net").voice(), "distinct kernels");
}

/// A reply is read through the codec's one reader of the outcome layout
/// (S13.4): the verdict with what it taught, the height, the lanes; a frame
/// that does not parse whole is no reply, never a reply of zeros.
#[test]
fn verdicts_decode_from_the_wire_reply() {
let body = dialogue::encode_outcome_reply(3, 5, &[1, 0, 2, 0], &[0xAB, 0xCD]);
let reply = decode_reply(&body).expect("the codec reads its own layout");
match reply.verdict {
    Verdict::Installed { taught } => assert_eq!(taught, vec![0xAB, 0xCD]),
    v => panic!("expected Installed, got {v:?}"),
}
assert_eq!((reply.height, reply.pressure), (5, [1, 0, 2, 0]));
let empty = dialogue::encode_outcome_reply(1, 0, &[0; 4], &[]);
assert_eq!(decode_reply(&empty).expect("parses").verdict, Verdict::Decomposed { taught: vec![] });
assert_eq!(
    decode_reply(&dialogue::encode_outcome_reply(2, 0, &[0; 4], &[])).expect("parses").verdict,
    Verdict::GermPending
);
assert!(decode_reply(&body[..body.len() - 1]).is_none(), "a short frame is no reply");
}
