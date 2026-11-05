#![cfg(not(debug_assertions))]
#![allow(clippy::expect_used, clippy::panic, reason = "§2.4 binds cognition; this crate is a test")]
// AI-GENERATED CONTENT — see DISCLAIMER.md
//
//! 78.137 — THE DURABLE-ADMISSION DRILL: a court act answered is on the
//! disk. A hypha on a fresh temporary home is put court acts one round at a
//! time and killed with SIGKILL after each answered round (no persist on the
//! way down), then started again; after the last round every act is read
//! back, and none acknowledged may be lost.
//!
//! RELEASE ONLY: the drill runs the release server binary through one boot
//! per round.
//!
//! NEGATIVE CONTROL: built with `--features drill-undurable`, the server
//! answers a court act before its pre-reply sync, and this test must fail
//! (`cargo test --release --features drill-undurable --test durable`), the
//! proof that the green drill can fail.

use mycelium::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use mycelium::sdk::{Session, Voice};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// The rounds of the drill, two court acts each.
const ROUNDS: u64 = 20;

/// Domain `i`'s key and its one tier's, distinct for every `i`.
fn keys(i: u64) -> (ed25519_dalek::SigningKey, ed25519_dalek::SigningKey) {
    let mut d = [7u8; 32];
    d[..8].copy_from_slice(&i.to_le_bytes());
    let mut t = d;
    t[31] = 8;
    (ed25519_dalek::SigningKey::from_bytes(&d), ed25519_dalek::SigningKey::from_bytes(&t))
}

/// A hypha serving the drill's home. Dropping it kills the process with
/// SIGKILL, so a failed assertion leaves no server behind.
struct Hypha {
    child: Child,
}

impl Hypha {
    /// The server started on `home`, returned once it has bound its
    /// listener. Its stderr is drained on a thread of its own, so the pipe
    /// never fills while it serves.
    fn start(home: &Path, args: &[&str]) -> Hypha {
        let mut child = Command::new(env!("CARGO_BIN_EXE_mycelium-server"))
            .arg("--home")
            .arg(home)
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the server binary starts");
        let stderr = child.stderr.take();
        // Held from here on, so every path below kills and waits on it.
        let hypha = Hypha { child };
        let stderr = stderr.expect("the server's stderr is piped");
        let (lines, heard) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                // Past the bound line nobody listens; the pipe is drained.
                let _ = lines.send(line);
            }
        });
        let mut log = Vec::new();
        // The channel closes when the server's stderr does: a server that
        // exits before it binds ends the loop.
        for line in heard.iter() {
            let bound = line.contains("mycelium-hypha bound ");
            log.push(line);
            if bound {
                return hypha;
            }
        }
        panic!("the hypha exited before it bound:\n{}", log.join("\n"));
    }
}

impl Drop for Hypha {
    fn drop(&mut self) {
        // SIGKILL: no handler, no persist; only what was durable survives.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Domain `i` founded and its reserve designated: two court acts, each
/// answered only once durable.
fn act(endpoint: &str, i: u64) -> Result<(), String> {
    let s = Session::open(endpoint, Voice::from_voice([9; 32]));
    let (domain, tier) = keys(i);
    let (dv, tv) = (domain.verifying_key().to_bytes(), tier.verifying_key().to_bytes());
    let genesis = DomainGenesis { domain: dv, tiers: vec![tv] }.sign(&domain);
    match s.found_domain(&genesis) {
        Ok(Founded::Sealed { .. } | Founded::AlreadyFounded) => {}
        other => return Err(format!("act {i}: genesis {other:?}")),
    }
    let reservation = Reservation { domain: dv, tier: tv, nonce: [0; 32] }.sign(&domain);
    match s.reserve(&reservation) {
        Ok(Founded::Reserved { .. }) => Ok(()),
        other => Err(format!("act {i}: reserve {other:?}")),
    }
}

/// The domains of `0..n` not read back with their reserve.
fn lost(endpoint: &str, n: u64) -> Vec<u64> {
    let s = Session::open(endpoint, Voice::from_voice([9; 32]));
    (0..n)
        .filter(|&i| {
            let (domain, tier) = keys(i);
            let held = s.reserve_reading(domain.verifying_key().to_bytes()).map(|r| r.current);
            !matches!(held, Ok(Some(t)) if t == tier.verifying_key().to_bytes())
        })
        .collect()
}

/// 78.137 — EVERY ANSWERED COURT ACT SURVIVES SIGKILL. Each round founds a
/// domain and designates its reserve, then the hypha is killed and started
/// again on the same home; after the last round every domain is read back
/// with its reserve, and none is lost.
#[test]
fn court_act_survives_kill_after_answer() {
    let home = tempfile::tempdir().expect("a home to write into");
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .and_then(|l| l.local_addr())
        .expect("a free port on the loopback")
        .port();
    let endpoint = format!("127.0.0.1:{port}");
    std::fs::write(home.path().join("bind"), format!("{endpoint}\n")).expect("the bind written");
    let mut hypha = Hypha::start(home.path(), &["--mint-genesis"]);
    for i in 0..ROUNDS {
        let answered = act(&endpoint, i);
        assert_eq!(answered, Ok(()), "round {i} is answered");
        drop(hypha);
        hypha = Hypha::start(home.path(), &[]);
    }
    let lost = lost(&endpoint, ROUNDS);
    drop(hypha);
    assert!(lost.is_empty(), "{} of {ROUNDS} answered rounds lost: domains {lost:?}", lost.len());
}
