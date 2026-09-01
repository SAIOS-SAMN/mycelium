// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! mycelium-server — a HYPHA in the Čech-nerve mesh (never a star). Each hypha
//! runs its OWN Kernel (one eye per node), carries an id + economic tier, and
//! registers its peers into its nerve so the mesh is a non-ring cover of hyphas.
//! Clients dial a hypha (server/client split preserved); peers fuse bases via
//! sun_recon (P6). Usage:
//!   mycelium-server [bind] [--id N] [--tier founder|validator|participant] [--peer ADDR]...
use mycelium::anastome::dialogue;
use mycelium::cognition::{decode_percept, germ_bytes, outcome_code, Kernel, Outcome};
use mycelium::economy::Tier;
use mycelium::network::nerve::{Fingerprint, Router};
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

/// The RENDEZVOUS for a content key: the XOR-closest peer (found by trie descent,
/// O(bits), never a peer scan), IF closer than this node — else `None` (self IS
/// the rendezvous). One destination, content-addressed; no fanout.
fn rendezvous(key: &[u8; 32], self_id: &[u8; 32], router: &Router) -> Option<String> {
    let dist = |idb: &[u8; 32]| -> [u8; 32] { std::array::from_fn(|i| idb[i] ^ key[i]) };
    match router.closest(key) {
        Some((pid, addr)) if dist(&pid) < dist(self_id) => Some(addr),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let addr = args.get(1).filter(|a| !a.starts_with("--")).cloned().unwrap_or_else(|| "127.0.0.1:7500".to_string());
    let tier = Tier::from_str(flag(&args, "--tier").unwrap_or("participant"));
    let id = flag(&args, "--id").map(id_of).unwrap_or_else(|| id_of(&addr));
    let peers = flags(&args, "--peer");

    // P7 — durable state: the germinated basis + earned ledger persist across
    // restart. Default path is per-node; override with --state PATH.
    let id_tag = flag(&args, "--id").map(String::from).unwrap_or_else(|| addr.replace(':', "_"));
    let state_path = flag(&args, "--state").map(String::from).unwrap_or_else(|| format!("/opt/mycelium-v0.1.0/state/hypha-{id_tag}.ledger"));
    if let Some(dir) = std::path::Path::new(&state_path).parent() {
        let _ = std::fs::create_dir_all(dir);
    }

    let kernel = Arc::new(Mutex::new(Kernel::new()));
    // Resume the learned language from disk (a corrupt ledger boots clean, no crash).
    if let Ok(bytes) = std::fs::read(&state_path) {
        let mut k = kernel.lock().unwrap();
        if k.restore(&bytes) {
            eprintln!("restored basis={} from {state_path}", k.basis_len());
        }
    }
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
    // The XOR routing trie — O(bits) rendezvous lookup, built once (rebuilt on
    // discovery churn later). No per-demand peer scan.
    let mut router = Router::new();
    for pr in &peers { router.insert(id_of(pr), pr.clone()); }

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
        // Body: [act][..] — the ANASTOME dialogue (anastome::dialogue). Percept and
        // routed-gossip percepts keep their historical tags; quote and demand are
        // the spoken economy; teach rides the reply; settle is appended on install.
        let Some((&kind, rest)) = frame.body.split_first() else { continue };

        // ── quote: "what is the highest-yield work?" ─────────────────────────
        if kind == dialogue::ACT_QUOTE {
            let Some(qvoice) = dialogue::decode_quote_request(&frame.body) else { continue };
            let (surface, priced) = kernel.lock().unwrap().quote(4);
            let items: Vec<dialogue::WorkItem> =
                priced.iter().map(|(e, mu)| (e.address(), *mu, e.sub().to_vec())).collect();
            eprintln!(
                "  quote voice {:02x} → surface {surface:?}; {} clusters offered",
                qvoice[0],
                items.len()
            );
            let reply = Frame { lane: frame.lane, body: dialogue::encode_quote_reply(&surface, &items) };
            let _ = s.write_all(&reply.encode().unwrap_or_default());
            continue;
        }

        // ── percept / gossip / demand → the judge ────────────────────────────
        let pbytes = if kind == dialogue::ACT_GOSSIP && rest.len() >= 32 { &rest[32..] } else { rest };
        let (out, basis, flux, reserve, taught, settle, voice) = {
            let mut k = kernel.lock().unwrap();
            let (out, voice) = if kind == dialogue::ACT_DEMAND {
                // A spoken, unsolved coboundary challenge — a peer's independent
                // derivation attesting an open residual.
                let Some((dvoice, chain)) = dialogue::decode_demand(&frame.body) else { continue };
                (k.attest(&chain, dvoice), dvoice)
            } else {
                // The client PERCEIVED (segmented bodies); the JUDGE adjudicates.
                let Some((pvoice, a_bodies, b_bodies)) = decode_percept(pbytes) else { continue };
                (k.adjudicate(&a_bodies, &b_bodies, pvoice), pvoice)
            };
            // TEACH: on install, hand the canonical ANASTOME form back; SETTLE: the
            // consilience receipt (parties + the μ⃗ award priced at the surface).
            let (taught, settle) = match &out {
                Outcome::Installed(i) => {
                    let g = &k.basis()[*i];
                    let addr = g.address();
                    let mu = mycelium::economy::price(
                        &g.magnitude(),
                        &mycelium::economy::yield_surface(k.physics()),
                    );
                    (germ_bytes(g), Some((addr, k.voices_of(&addr), mu)))
                }
                _ => (Vec::new(), None),
            };
            (out, k.basis_len(), k.physics().metabolism_rate(), k.physics().reserve, taught, settle, voice)
        };
        let label = match &out {
            Outcome::Decomposed(word) => format!("valid: decomposed → sentence {word:?} ({} generators)", word.len()),
            Outcome::GermPending => "valid: germ pending (one voice)".to_string(),
            Outcome::Installed(i) => format!("valid: GERM INSTALLED #{i}"),
            Outcome::Fragmented(parts) => format!("valid: fragmented ({} bodies)", parts.len()),
            Outcome::Pivot => "invalid: pivot (degenerate — nothing to reduce)".to_string(),
        };
        let src = match kind {
            k if k == dialogue::ACT_GOSSIP => "gossip",
            k if k == dialogue::ACT_DEMAND => "demand",
            _ => "client",
        };
        eprintln!(
            "  {src} percept voice {:02x} → {label}; basis={basis}; metabolism {flux:?}; reserve {reserve:?}",
            voice[0]
        );
        // Reply: outcome | metabolism/pressure(4) | taught (len16 | bytes) | settle?
        let mut body = vec![outcome_code(&out)];
        for f in flux {
            body.push(f.min(255) as u8); // per-lane pressure, saturated to a byte
        }
        body.extend_from_slice(&(taught.len() as u16).to_le_bytes());
        body.extend_from_slice(&taught);
        if let Some((germ, voices, mu)) = &settle {
            body.extend_from_slice(&dialogue::encode_settle(germ, voices, mu));
        }
        let reply = Frame { lane: frame.lane, body };
        let _ = s.write_all(&reply.encode().unwrap_or_default());

        // P7 — a new germ changed the durable language: persist it (atomic write —
        // to .tmp then rename — so a crash mid-write never corrupts the ledger).
        if matches!(out, Outcome::Installed(_)) {
            let snap = kernel.lock().unwrap().snapshot();
            let tmp = format!("{state_path}.tmp");
            if std::fs::write(&tmp, &snap).is_ok() {
                let _ = std::fs::rename(&tmp, &state_path);
            }
        }

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
            let key = *blake3::hash(&pbytes[32..]).as_bytes(); // demand address, voice-free
            if let Some(dest) = rendezvous(&key, &id, &router) {
                let mut g = vec![0x01u8];
                g.extend_from_slice(&key);
                g.extend_from_slice(pbytes);
                let gframe = Frame { lane: frame.lane, body: g }.encode().unwrap_or_default();
                if let Ok(mut ps) = TcpStream::connect(&dest) {
                    let _ = ps.write_all(&gframe);
                }
            }
        }
    }
}
