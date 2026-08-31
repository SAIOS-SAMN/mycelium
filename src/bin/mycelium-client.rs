// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-client — a voice on the wire. Sends one demand (scene A → scene B)
//! as a lane-tagged frame and prints the server's verdict. Usage:
//!   mycelium-client [addr] [voice_byte]
//! The demo demand is a rigid translation of an L-tromino; run it from two
//! distinct voice bytes to watch the germ install by two-voice consilience.
use mycelium::cognition::encode_demand;
use mycelium::network::wire::{Frame, Stratum};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:7500".to_string());
    let voice = std::env::args().nth(2).and_then(|s| s.parse::<u8>().ok()).unwrap_or(1);

    let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)];
    let b = [(0, 3, 1u8), (0, 4, 1), (1, 3, 1)]; // +0,+3 translate
    let frame = Frame { lane: Stratum::Spatial, body: encode_demand(&a, &b, &[voice; 32]) };

    let mut s = TcpStream::connect(&addr).expect("connect");
    s.write_all(&frame.encode().expect("encode")).expect("send");
    let mut head = [0u8; 5];
    s.read_exact(&mut head).expect("reply header");
    let mut body = vec![0u8; head[1] as usize];
    s.read_exact(&mut body).expect("reply body");
    let verdict = match body.first() {
        Some(1) => "recognised (recall)",
        Some(2) => "germ pending (one voice)",
        Some(3) => "GERM INSTALLED (two-voice consilience)",
        Some(4) => "pivot (irreducible residual)",
        Some(5) => "fragmented (per-body)",
        _ => "unknown",
    };
    // The hypha reports its per-lane syntropic/entropic metabolism (order flux);
    // a continuous client paces its demand cadence by it (syntropic lanes pull
    // more, entropic lanes back off).
    let flux: Vec<i64> = body.get(1..5).map(|b| b.iter().map(|&x| x as i8 as i64).collect()).unwrap_or_default();
    println!("voice {voice} → {verdict}; hypha metabolism(order flux) {flux:?}");
}
