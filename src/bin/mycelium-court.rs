// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! court — THE STAKED LINGUISTIC-FINALITY COURT (alpha Step 5). Clients sense;
//! hyphas prove; the court judges. It speaks the same ANASTOME wire as every
//! kernel — its authority is the deterministic replay, never the socket.
//!
//! Two dockets, per the ratified architecture:
//!
//!   CONTRADICTION (reactive) — blind replay of each provisional word: the
//!   spoken element must re-derive EXACTLY from its own centered chain
//!   (from_residual is the deterministic engine: same chain → same word, or
//!   the speech was false), the chain must be δ-closed under the substrate
//!   framing (centered, sorted, non-degenerate), and the install must strictly
//!   descend (0⃗ ≺_DM μ⃗). A failed replay is FALSE SPEECH → RULING_SLASHED:
//!   the word is excised and the voices' shares debit into the pool. Two valid
//!   derivations of the same address are DIALECTS — the transport is logged,
//!   never punished.
//!
//!   ADMISSION (proactive finality) — the candidate meets the FEDERATED basis
//!   (every word this court has already finalized, across all hyphas): a
//!   duplicate address with identical substance is cross-hypha consilience
//!   (upheld — the strongest confluence witness); the same base geometry under
//!   a DIFFERENT address violates the +axis law (it belongs as a section of
//!   the existing thought, not a sibling card) → RULING_DEMOTED — it stays
//!   local dialect, the award evaporates, nobody is slashed. A genuinely new
//!   geometry extends the tautology without conflict → RULING_UPHELD, and the
//!   word joins the federation.
//!
//! Every ruling is sealed onto the court's own chronometric receipt chain
//! (BLAKE3-linked K-steps) and the head hash rides the verdict act — the
//! signature binding each ruling to the court's tamper-evident history. The
//! court ENROLLS staked at its tier like any identity: judging is itself
//! priced work (the finality entries anchor to the hypha ledgers).
//!
//! Usage: court [--kernel NAME] --hypha ADDR [--hypha ADDR]...
use mycelium::anastome::dialogue;
use mycelium::chronometry::ReceiptChain;
use mycelium::physics::{reserve_max, Physics};
use mycelium::sdk::{Kernel, Session};
use mycelium::transducer::element::Element;
use mycelium::transducer::stalk::{self, Stalks};
use mycelium::verdict;
use std::time::Instant;

fn flags<'a>(args: &'a [String], name: &str) -> Vec<&'a str> {
    args.windows(2).filter(|w| w[0] == name).map(|w| w[1].as_str()).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let name = args
        .windows(2)
        .find(|w| w[0] == "--kernel")
        .map(|w| w[1].clone())
        .unwrap_or_else(|| "court".to_string());
    let hyphas: Vec<String> = flags(&args, "--hypha").into_iter().map(String::from).collect();
    if hyphas.is_empty() {
        eprintln!("court: no --hypha given");
        std::process::exit(1);
    }

    let kernel = Kernel::named(&name);
    let voice = kernel.voice();
    let sessions: Vec<Session> = hyphas.iter().map(|h| Session::open(h, Kernel::named(&name))).collect();
    // JUDICIAL CONTINUITY: the full court state (K-chain, federation, ruled
    // set, rebind marks, counters) persists via atomic write-rename and
    // rehydrates at boot — the judicial clock never resets, sealed words
    // are never re-adjudicated, and a corrupted journal refuses whole.
    let state_path = args
        .windows(2)
        .find(|w| w[0] == "--state")
        .map(|w| w[1].clone())
        .unwrap_or_else(|| "state/court.federation".to_string());
    let restored = mycelium::court::ledger::CourtState::load(&state_path);
    // --reaudit: re-derive the federation and ruled set under the current
    // congruence lens (the K-chain continues — the journal keeps both
    // judicial eras). Used when the lens itself is corrected, e.g. the
    // orbit-quotient upgrade that obsoleted coordinate-set comparison.
    let reaudit = args.iter().any(|a| a == "--reaudit");
    // The court's own tamper-evident history — every ruling is a K-step.
    let mut chain = ReceiptChain::default();
    // THE FEDERATION — every word this court has finalized, across all hyphas:
    // (address, the element as upheld). Vec-of-relations; the current tautology.
    let mut federation: Vec<([u8; 32], Element)> = Vec::new();
    // The federation's stalk space (index-aligned): congruence lookup
    // selects the fiber over a candidate's invariant; measured from the
    // held words at boot, admitted alongside every federation entry.
    let mut stalks = Stalks::default();
    let mut ruled: Vec<[u8; 32]> = Vec::new();
    // THE SELECTION FIBER'S REFINEMENT STATE — evidence accumulated per word
    // across the whole nerve, and the words already rebound.
    let mut evidence: Vec<([u8; 32], Vec<(Vec<(i64, i64)>, Vec<(i64, i64, u8)>)>)> = Vec::new();
    // (address, family size at the last binding) — rebinding is ITERATIVE:
    // a word re-rules whenever its evidence family has grown since it bound,
    // so loose early carriers tighten as witnessed placements accumulate.
    let mut rebound: Vec<([u8; 32], usize)> = Vec::new();
    let mut rebinds = 0u64;
    let (mut upheld, mut demoted, mut slashed) = (0u64, 0u64, 0u64);
    if let Some(s) = restored {
        chain = s.chain;
        federation = s.federation;
        stalks = Stalks::over(federation.iter().map(|(_, e)| e.sub()));
        ruled = s.ruled;
        rebound = s.rebound;
        upheld = s.upheld;
        demoted = s.demoted;
        slashed = s.slashed;
        rebinds = s.rebinds;
        eprintln!(
            "court restored: K={} federation {} words, {} ruled, {} rebound",
            chain.next_step_index(),
            federation.len(),
            ruled.len(),
            rebound.len(),
        );
    }
    if reaudit {
        federation.clear();
        stalks.clear();
        ruled.clear();
        eprintln!(
            "re-audit under the current congruence lens: federation and ruled cleared; K continues at {}",
            chain.next_step_index(),
        );
    }
    // One closure seals the whole judicial state after every ruling — the
    // atomic write-rename pattern; a crash leaves the previous sealed state.
    let persist = |chain: &ReceiptChain,
                   federation: &Vec<([u8; 32], Element)>,
                   ruled: &Vec<[u8; 32]>,
                   rebound: &Vec<([u8; 32], usize)>,
                   counters: (u64, u64, u64, u64)| {
        let s = mycelium::court::ledger::CourtState {
            chain: chain.clone(),
            federation: federation.clone(),
            ruled: ruled.clone(),
            rebound: rebound.clone(),
            upheld: counters.0,
            demoted: counters.1,
            slashed: counters.2,
            rebinds: counters.3,
        };
        let _ = s.save(&state_path);
    };
    let mut last_total = u64::MAX; // voice the summary only when rulings moved
    let mut physics = Physics::default();
    let start = Instant::now();
    eprintln!("court '{name}' voice {:02x} over {} hyphas {hyphas:?}", voice[0], hyphas.len());

    // GENESIS AUDIT (federation seeding) — a drained work relation, not a
    // sweep: each entry is (session index, page cursor). One stride runs
    // per court round: the front entry fetches one page, seals its words,
    // and either advances (more pages), rotates to the back (unreachable —
    // other servers proceed, this one retries later), or completes and
    // LEAVES the relation (end page). Work per round is one page whether
    // the nerve holds three servers or a thousand; the ruled-set
    // deduplication makes any retry idempotent. An empty relation is the
    // audit's completed state — nothing iterates the finished.
    let mut genesis_walk: Vec<(usize, u32)> = (0..sessions.len()).map(|i| (i, 0)).collect();

    loop {
        let mut heard_any = false;
        // ── one genesis stride, when the audit relation still holds work.
        // The direct call decides everything: transport failure is the
        // exchange's io::Result (the session's impedance already measured
        // it), a malformed body is None, and the carrier itself carries the
        // walk's states — a non-empty page is work, the empty page is the
        // end. No wrapper types.
        if let Some(&(si, cursor)) = genesis_walk.first() {
            let page = sessions[si]
                .speak(dialogue::encode_genesis_request(&voice, cursor))
                .map(|body| dialogue::decode_syllabus_reply(&body));
            match page {
                Err(_) | Ok(None) => {
                    // Unreachable or unreadable: the entry retries after the
                    // others advance.
                    let entry = genesis_walk.remove(0);
                    genesis_walk.push(entry);
                }
                Ok(Some(words)) if words.is_empty() => {
                    genesis_walk.remove(0); // audited whole — leaves the relation
                    eprintln!(
                        "genesis audit of {} complete: federation {} words, K={}",
                        hyphas[si],
                        federation.len(),
                        chain.next_step_index(),
                    );
                    persist(
                        &chain,
                        &federation,
                        &ruled,
                        &rebound,
                        (upheld, demoted, slashed, rebinds),
                    );
                }
                Ok(Some(words)) => {
                    genesis_walk[0].1 += words.len() as u32; // non-empty: the walk advances
                    heard_any = true;
                    for wb in &words {
                        let mut pos = 0usize;
                        let spoken = Element::from_bytes(wb, &mut pos);
                        let addr = spoken.as_ref().map(|w| w.address()).unwrap_or([0u8; 32]);
                        if ruled.contains(&addr) {
                            continue; // sealed in a prior block — never re-adjudicated
                        }
                        let outcome = verdict::rule(&federation, &stalks, &addr, spoken.as_ref());
                        let mut op = [0u8; 32];
                        op[0] = outcome.code();
                        chain.append(addr, op);
                        ruled.push(addr);
                        // The tally IS the state map — one arm per outcome.
                        match outcome {
                            verdict::Ruling::Admitted(sealed) => {
                                stalks.admit(&stalk::invariant(sealed.sub()), federation.len());
                                federation.push((addr, sealed));
                                upheld += 1;
                                // Propagation: a count of deterministic queue
                                // acknowledgements across the nerve.
                                let _acks = sessions
                                    .iter()
                                    .filter(|peer| {
                                        matches!(
                                            peer.speak(dialogue::encode_adopt(&voice, wb)),
                                            Ok(ref r) if r.first() == Some(&1)
                                        )
                                    })
                                    .count();
                            }
                            verdict::Ruling::Consilient | verdict::Ruling::Dialect => upheld += 1,
                            verdict::Ruling::Demoted => demoted += 1,
                            verdict::Ruling::Slashed(_) => slashed += 1,
                        }
                    }
                    physics.spend([false, false, true, false]); // the stride is a stroke
                }
            }
        }
        for (hi, session) in sessions.iter().enumerate() {
            let Ok(body) = session.speak(dialogue::encode_docket_request(&voice)) else {
                continue;
            };
            let Some(dockets) = dialogue::decode_docket_reply(&body) else { continue };
            for (addr, elem_bytes, voices, mu) in dockets {
                if ruled.contains(&addr) {
                    continue;
                }
                heard_any = true;
                // Both dockets rule through the isolated verification core:
                // one exhaustive state map — admission is the Admitted arm
                // carrying its sealed element, never a flag.
                let mut pos = 0usize;
                let spoken = Element::from_bytes(&elem_bytes, &mut pos);
                let outcome = verdict::rule(&federation, &stalks, &addr, spoken.as_ref());
                if let verdict::Ruling::Admitted(sealed) = &outcome {
                    stalks.admit(&stalk::invariant(sealed.sub()), federation.len());
                    federation.push((addr, sealed.clone()));
                }
                let ruling = (outcome.code(), outcome.reason());
                // SEAL the ruling onto the court's chain: fp = the word's
                // address, op = the ruling byte spread — then speak the verdict
                // with the head hash as the signature.
                let mut op = [0u8; 32];
                op[0] = ruling.0;
                let receipt = chain.append(addr, op).chain_hash;
                // Every verdict broadcasts to EVERY server (finality is
                // network-wide, not per-origin), and an upheld word is
                // TAUGHT across the nerve so the server bases converge to
                // one universal genome — each adoption door re-verifies the
                // blind replay itself before installing.
                let verdict = dialogue::encode_verdict(&voice, &addr, ruling.0, &receipt);
                let mut applied = false;
                for peer in &sessions {
                    if peer.speak(verdict.clone()).map(|r| r.first() == Some(&1)).unwrap_or(false) {
                        applied = true;
                    }
                    if ruling.0 == dialogue::RULING_UPHELD {
                        let _ = peer.speak(dialogue::encode_adopt(&voice, &elem_bytes));
                    }
                }
                match ruling.0 {
                    dialogue::RULING_UPHELD => upheld += 1,
                    dialogue::RULING_DEMOTED => demoted += 1,
                    _ => slashed += 1,
                }
                ruled.push(addr);
                persist(&chain, &federation, &ruled, &rebound, (upheld, demoted, slashed, rebinds));
                eprintln!(
                    "  [{}] {:02x}{:02x}… voices {} μ⃗ {mu:?} → {} ({}) applied={applied} K={}",
                    hyphas[hi],
                    addr[0],
                    addr[1],
                    voices.len(),
                    match ruling.0 {
                        dialogue::RULING_UPHELD => "UPHELD",
                        dialogue::RULING_DEMOTED => "DEMOTED",
                        _ => "SLASHED",
                    },
                    ruling.1,
                    chain.next_step_index(),
                );
                // The ruling's metabolic weight lands on the lanes the word's
                // escrowed μ⃗ actually occupies — never a hardcoded plane.
                let lanes: [bool; 4] = std::array::from_fn(|l| mu[l] > 0);
                physics.bump(lanes);
                physics.spend(lanes);
            }
        }
        // ── KB ANCHOR REFINEMENT (7c): drain each hypha's birth evidence,
        // pool it per word across the NERVE, and when a family (≥2 evidences)
        // exists, derive π* = the tightest common carrier (the intersection
        // move — one evidence never binds) and speak the REBIND verdict.
        for (hi, session) in sessions.iter().enumerate() {
            let Ok(body) = session.speak(dialogue::encode_evidence_request(&voice)) else {
                continue;
            };
            let Some(items) = dialogue::decode_evidence_reply(&body) else { continue };
            for (addr, fp, scene) in items {
                if fp.is_empty() {
                    continue; // evidence keeps pooling even for bound words
                }
                match evidence.iter_mut().find(|(a, _)| *a == addr) {
                    Some((_, set)) => {
                        if !set.iter().any(|(f, s)| *f == fp && *s == scene) && set.len() < 16 {
                            set.push((fp, scene));
                        }
                    }
                    None => evidence.push((addr, vec![(fp, scene)])),
                }
            }
            let _ = hi;
        }
        evidence.retain(|(addr, set)| {
            let bound_at = rebound.iter().find(|(a, _)| a == addr).map(|(_, n)| *n);
            match bound_at {
                None if set.len() < 2 => return true, // hold until a family exists
                Some(n) if set.len() <= n => return true, // no new evidence since binding
                _ => {}
            }
            // π* from the pooled family, through the verification core.
            let Some(pi) = verdict::derive_pi(set) else {
                return true; // no common carrier yet — more family needed
            };
            // Seal + speak the rebind to every hypha (the Čech-nerve broadcast).
            let mut op = [0u8; 32];
            op[0] = dialogue::RULING_REBIND;
            let receipt = chain.append(*addr, op).chain_hash;
            let verdict = dialogue::encode_verdict_payload(
                &voice,
                addr,
                dialogue::RULING_REBIND,
                &receipt,
                Some(&pi),
            );
            let mut applied_any = false;
            for session in &sessions {
                if session.speak(verdict.clone()).map(|r| r.first() == Some(&1)).unwrap_or(false) {
                    applied_any = true;
                }
            }
            rebinds += 1;
            match rebound.iter_mut().find(|(a, _)| a == addr) {
                Some(entry) => entry.1 = set.len(), // re-refined: new family mark
                None => rebound.push((*addr, set.len())),
            }
            persist(&chain, &federation, &ruled, &rebound, (upheld, demoted, slashed, rebinds));
            eprintln!(
                "  REBIND {:02x}{:02x}… π* {:?} from {} evidences, applied={applied_any} K={}",
                addr[0],
                addr[1],
                pi,
                set.len(),
                chain.next_step_index(),
            );
            physics.bump([false, false, true, false]);
            physics.spend([false, false, true, false]);
            false // spent — the family bound its word
        });

        // The court breathes like every kernel: rulings are the entropic
        // stroke; an empty docket round is the syntropic dwell.
        if !heard_any {
            physics.spend([true; 4]);
        }
        if physics.depleted() {
            if upheld + demoted + slashed + rebinds != last_total {
                last_total = upheld + demoted + slashed + rebinds;
                eprintln!(
                    "[{name}] {:.0}s | upheld {upheld} demoted {demoted} slashed {slashed} rebinds {rebinds} | federation {} words | K={}",
                    start.elapsed().as_secs_f64(),
                    federation.len(),
                    chain.next_step_index(),
                );
            }
            // Regenerate: to EXPAND_MIN when work flowed (quick return), to
            // the FULL reserve on an empty docket round — an idle court dials
            // gently, not hotly.
            let floor = if heard_any { 0 } else { reserve_max() };
            while physics.depleted() || physics.reserve.iter().any(|&r| r < floor) {
                physics.regenerate();
                physics.relax();
                std::thread::sleep(std::time::Duration::from_millis(reserve_max()));
            }
        }
    }
}
