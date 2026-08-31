// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-client — a VOICE that PERCEIVES. The client senses the scene
//! (segments bodies) and reports the PERCEPT to its hypha; the hypha's judge
//! adjudicates valid/invalid and TEACHES back the canonical ANASTOME element,
//! which the client learns (so no server↔client dialect drifts). Usage:
//!   mycelium-client [addr] [voice_byte]
use mycelium::cognition::{encode_percept, perceive};
use mycelium::network::wire::{Frame, Stratum};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:7500".to_string());
    let voice = std::env::args().nth(2).and_then(|s| s.parse::<u8>().ok()).unwrap_or(1);

    // PERCEPTION happens here, in the client — the sensorium, not the server.
    // Scene mode (3rd arg): default = a rigid translate (a recall specimen);
    // "deform" = an L→bar shape change no motor closes — a RESIDUAL the judge must
    // germinate as a coordinate-exact coboundary germ (the reframe, on the wire).
    let mode = std::env::args().nth(3).unwrap_or_default();
    let (a, b): (&[(i64, i64, u8)], &[(i64, i64, u8)]) = if mode == "deform" {
        (&[(0, 0, 1), (0, 1, 1), (1, 0, 1)], &[(0, 0, 1), (0, 1, 1), (0, 2, 1)])
    } else {
        (&[(0, 0, 1), (0, 1, 1), (1, 0, 1)], &[(0, 3, 1), (0, 4, 1), (1, 3, 1)])
    };
    let (a_bodies, b_bodies) = perceive(a, b);
    // kind byte 0x00 = an ORIGIN percept from a client (a peer gossip is 0x01).
    let mut pbody = vec![0x00u8];
    pbody.extend_from_slice(&encode_percept(&a_bodies, &b_bodies, &[voice; 32]));
    let frame = Frame { lane: Stratum::Spatial, body: pbody };

    let mut s = TcpStream::connect(&addr).expect("connect");
    s.write_all(&frame.encode().expect("encode")).expect("send");

    let mut head = [0u8; 5];
    s.read_exact(&mut head).expect("reply header");
    let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
    let mut body = vec![0u8; len];
    s.read_exact(&mut body).expect("reply body");

    let verdict = match body.first() {
        Some(1) => "valid: recall",
        Some(2) => "valid: germ pending",
        Some(3) => "valid: GERM INSTALLED",
        Some(4) => "invalid: pivot",
        Some(5) => "valid: fragmented",
        _ => "unknown",
    };
    let flux: Vec<i64> = body.get(1..5).map(|b| b.iter().map(|&x| x as i8 as i64).collect()).unwrap_or_default();
    // The taught ANASTOME element (the language the hypha hands back).
    let taught_len = body.get(5..7).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).unwrap_or(0);
    let taught = body.get(7..7 + taught_len).map(|t| {
        t.iter().take(6).map(|x| format!("{x:02x}")).collect::<String>()
    });
    println!(
        "voice {voice} → {verdict}; metabolism {flux:?}; taught ANASTOME {}",
        taught.map(|t| format!("{t}… ({taught_len}B)")).unwrap_or_else(|| "—".into())
    );
}
