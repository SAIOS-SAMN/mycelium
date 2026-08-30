// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-server — the new topological node. Binds a TCP listener, reads
//! lane-tagged wire frames, and drives each demand through the cognition kernel
//! (transducer → solve → conception → two-voice admission). Novel germs install
//! to the basis only on the second independent voice. Plaintext TCP for now;
//! Noise is a transport wrapper to add over the same `wire` framing.
use mycelium::cognition::{decode_demand, outcome_code, Kernel, Outcome};
use mycelium::network::wire::Frame;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

fn read_frame(s: &mut TcpStream) -> Option<Frame> {
    let mut head = [0u8; 5];
    s.read_exact(&mut head).ok()?;
    let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
    if len > mycelium::network::wire::BODY_BUDGET {
        return None;
    }
    let mut buf = vec![0u8; 5 + len];
    buf[..5].copy_from_slice(&head);
    s.read_exact(&mut buf[5..]).ok()?;
    Frame::decode(&buf).map(|(f, _)| f)
}

fn main() {
    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:7500".to_string());
    let kernel = Arc::new(Mutex::new(Kernel::new()));
    let listener = TcpListener::bind(&addr).expect("bind");
    eprintln!("mycelium-server: listening on {addr} (topological node, Δ-core)");
    for stream in listener.incoming() {
        let Ok(mut s) = stream else { continue };
        let Some(frame) = read_frame(&mut s) else { continue };
        let Some((voice, a, b)) = decode_demand(&frame.body) else { continue };
        let out = kernel.lock().unwrap().ingest(&a, &b, voice);
        let basis = kernel.lock().unwrap().basis_len();
        let label = match &out {
            Outcome::Recognised(i) => format!("recognised (recall) #{i}"),
            Outcome::GermPending => "germ pending (one voice)".to_string(),
            Outcome::Installed(i) => format!("GERM INSTALLED #{i} (two-voice consilience)"),
            Outcome::Fragmented(parts) => format!("fragmented into {} sub-demands", parts.len()),
            Outcome::Pivot => "pivot (irreducible residual — new fiber owed)".to_string(),
        };
        eprintln!("  demand voice {:02x} → {label}; basis={basis}", voice[0]);
        let reply = Frame { lane: frame.lane, body: vec![outcome_code(&out)] };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
    }
}
