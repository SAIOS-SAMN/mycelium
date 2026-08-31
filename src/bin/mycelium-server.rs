// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-server — a HYPHA in the Čech-nerve mesh (never a star). Each hypha
//! runs its OWN Kernel (one eye per node), carries an id + economic tier, and
//! registers its peers into its nerve so the mesh is a non-ring cover of hyphas.
//! Clients dial a hypha (server/client split preserved); peers fuse bases via
//! sun_recon (P6). Usage:
//!   mycelium-server [bind] [--id N] [--tier founder|validator|participant] [--peer ADDR]...
use mycelium::cognition::{decode_percept, germ_bytes, outcome_code, Kernel, Outcome};
use mycelium::economy::Tier;
use mycelium::network::nerve::Fingerprint;
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

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].as_str())
}
fn flags(args: &[String], name: &str) -> Vec<String> {
    args.windows(2).filter(|w| w[0] == name).map(|w| w[1].clone()).collect()
}

/// A stable node id from a string (address) — a wire identity, not cognition.
fn id_of(s: &str) -> [u8; 32] {
    *blake3::hash(s.as_bytes()).as_bytes()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let addr = args.get(1).filter(|a| !a.starts_with("--")).cloned().unwrap_or_else(|| "127.0.0.1:7500".to_string());
    let tier = Tier::from_str(flag(&args, "--tier").unwrap_or("participant"));
    let id = flag(&args, "--id").map(id_of).unwrap_or_else(|| id_of(&addr));
    let peers = flags(&args, "--peer");

    let kernel = Arc::new(Mutex::new(Kernel::new()));
    // Register peers into the nerve — the non-ring Čech cover of hyphas. (Real
    // per-lane fingerprints arrive with sun_recon/P6; here presence + a seeded
    // bloom so adjacency is genuine, not a static ring.)
    {
        let mut k = kernel.lock().unwrap();
        for p in &peers {
            let pid = id_of(p);
            let mut bloom = [0u8; 32];
            bloom[(pid[0] as usize) % 32] |= 1 << (pid[1] % 8);
            k.nerve.observe(pid, Fingerprint { beta1: [0; 4], bloom });
        }
    }
    let listener = TcpListener::bind(&addr).expect("bind");
    eprintln!(
        "mycelium-hypha {:02x} tier {:?} bound {addr}; nerve peers {} {:?}",
        id[0],
        tier,
        peers.len(),
        peers
    );

    for stream in listener.incoming() {
        let Ok(mut s) = stream else { continue };
        let Some(frame) = read_frame(&mut s) else { continue };
        // The client PERCEIVED (segmented bodies); the JUDGE only adjudicates.
        let Some((voice, a_bodies, b_bodies)) = decode_percept(&frame.body) else { continue };
        let (out, basis, flux, taught) = {
            let mut k = kernel.lock().unwrap();
            let out = k.adjudicate(&a_bodies, &b_bodies, voice);
            // TEACH: on a valid element, hand its canonical ANASTOME form back.
            let taught = match &out {
                Outcome::Recognised(i) | Outcome::Installed(i) => germ_bytes(&k.basis()[*i]),
                _ => Vec::new(),
            };
            (out, k.basis_len(), k.physics().order_flux(), taught)
        };
        let label = match &out {
            Outcome::Recognised(i) => format!("valid: recall #{i}"),
            Outcome::GermPending => "valid: germ pending (one voice)".to_string(),
            Outcome::Installed(i) => format!("valid: GERM INSTALLED #{i}"),
            Outcome::Fragmented(parts) => format!("valid: fragmented ({} bodies)", parts.len()),
            Outcome::Pivot => "invalid: pivot (residual — new fiber owed)".to_string(),
        };
        eprintln!("  percept voice {:02x} → {label}; basis={basis}; metabolism {flux:?}", voice[0]);
        // Reply: outcome | metabolism(4) | taught ANASTOME element (len16 | bytes).
        let mut body = vec![outcome_code(&out)];
        for f in flux {
            body.push(f.clamp(-128, 127) as i8 as u8);
        }
        body.extend_from_slice(&(taught.len() as u16).to_le_bytes());
        body.extend_from_slice(&taught);
        let reply = Frame { lane: frame.lane, body };
        let _ = s.write_all(&reply.encode().unwrap_or_default());
    }
}
