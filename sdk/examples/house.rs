// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// A MINIMAL HOUSE ON THE SDK: what a kernel's uplink does first on the
// network, with `mycelium-sdk` as its only Mycelium dependency — the
// Concordance signed, the core clock read, a domain founded — then the
// conformance read of the kernel profile against the same hypha.
//
//! `house <house home> <hypha address>`
//!
//! 1. Identity — the house's Ed25519 key, read from `<house home>/voice.key`
//!    or minted there; its session inside Noise on the home's static key
//!    (`Session::with_noise`, the hyphae pinned in `<house home>/noise.pins`).
//! 2. The Concordance — its clause register read (`ACT_LAWS` mode 7), each
//!    clause's fixtures reconstructed here (`house::sign_concordance`) and
//!    the readings signed by the key, presented to the hypha (mode 8), which
//!    records the verdict on its ledger; the house's alignment then read
//!    from that ledger (mode 9).
//! 3. The clock — the court's head stamp (`ACT_LAWS` mode 4).
//! 4. The domain — founded on the house's key, its one tier the same key
//!    (`ACT_DOMAIN`): sealed, or founded already on a run before.
//! 5. The crossing — the tier designated the domain's sitting reserve, then
//!    the reserve's fold stating a lawful transition sent through the
//!    universal core tube (`ACT_UPLINK`): the hypha rebuilds it under the
//!    universal laws and admits it, the reserve having signed every network
//!    clause in step 2 (track 24 B3, B4).
//! 6. Conformance — `conformance::run(endpoint, key, &[Profile::Kernel])`:
//!    every MUST act of the kernel profile, checked by decoded exact values;
//!    an act the hypha deferred is reported deferred, with its deficit.
//!
//! A SEATED MIND'S SOLVE IS ITS OWN (Concordance 6.2a, keys for a key's own
//! acts; MU-HOSTING §5a.6). This house founds a domain and seats no mind.
//! A house that seats minds and sets tasks before them may read the seat's
//! answer as the in-game call `FN_SOLVE` (fn 1) and relay its in-game calls
//! as it likes. The court solve is not the house's to file: the court pays
//! the key that signs a solve, and a seated mind files its own under the
//! key it sat by and is paid itself. A house that files a seat's solve with
//! the court under the house's key is paid to the house's key, never the
//! seat's.
//!
//! A hypha whose worker pool is full answers any act with the deferral; the
//! house reads it (`deferral::deferral_of`, `deferral::read_deferral`) and
//! says so; the act is spoken again on a later run.
//!
//! Exit 0 where the house crossed and conforms; 1 where it does not or the
//! hypha could not be reached; 2 on a usage refusal.

use mycelium_sdk::conformance::{run, Profile};
use mycelium_sdk::deferral::{deferral_of, read_deferral, Deferral};
use mycelium_sdk::house::{
    decode_alignment_reply, decode_clock_reply, decode_concordance_reply, decode_signature_reply, encode_alignment_request, encode_clock_request,
    encode_concordance_request, encode_signature_request, payload_hash, rotator, sign_concordance, membrane_at, carried_body, Assent, BodyMoments, BodyState,
    ClauseReading, DomainGenesis, Founded, Reservation, Transition,
};
use mycelium_sdk::prelude::*;
use mycelium_sdk::voice::hex32;
use std::process::ExitCode;

/// A deferral, said: the hypha's pool was full, nothing was ruled, and the
/// act is spoken again later.
fn said(d: Deferral) -> String {
    format!("a deferral: the hypha's pool is full, deficit S:G:T:H {:?}; retry later", d.deficit)
}

/// The house's alignment as the hypha's ledger reads it (`ACT_LAWS` mode
/// 9): each clause version converses, vetoed, collides or unread, at the
/// stamp of the signing that set it; the signings of the past beside it.
fn aligned(session: &Session, kernel: &[u8; 32]) {
    let read = match session.speak(encode_alignment_request(Some(kernel))) {
        Ok(reply) => decode_alignment_reply(&reply),
        Err(e) => {
            println!("  alignment: {e}");
            return;
        }
    };
    match read {
        Some(Ok(a)) => {
            for k in &a.kernels {
                for c in &k.clauses {
                    let reading = match &c.reading {
                        ClauseReading::Converses { at } => format!("converses at height {}", at.h),
                        ClauseReading::Vetoed { at } => format!("vetoed at height {}", at.h),
                        ClauseReading::Collides { at, residue } => format!("collides at height {}: {residue:?}", at.h),
                        ClauseReading::Unread => "unread".to_string(),
                    };
                    println!("  alignment: {} v{} {reading}", c.id.law, c.id.version);
                }
            }
            for h in &a.history {
                println!("  history: {} signed Concordance version {} at {} ({:?}, retired)", h.kernel, h.concordance, h.release, h.mechanism);
            }
        }
        Some(Err(r)) => println!("  alignment: refused, {r:?}"),
        None => println!("  alignment: a reply this build does not read"),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let (Some(home), Some(endpoint)) = (args.get(1), args.get(2)) else {
        eprintln!("usage: house <house home> <hypha address>");
        return ExitCode::from(2);
    };
    if let Err(e) = std::fs::create_dir_all(home) {
        eprintln!("{home}: {e}");
        return ExitCode::from(2);
    }
    let (voice, key) = mycelium_sdk::voice::load_or_mint(&format!("{home}/voice.key"));
    let domain = key.verifying_key().to_bytes();
    println!("house {}", hex32(&domain));
    // Every act inside Noise where the hypha speaks it, on the house home's
    // static key, the hyphae pinned beside it (MU-PRECURSORS §3.1).
    let noise = match mycelium_sdk::session::NoiseHome::open(std::path::Path::new(home)) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{home}: the Noise home does not open: {e}");
            return ExitCode::from(2);
        }
    };
    let session = Session::open(endpoint, voice).with_noise(noise);

    let served = match session.speak(encode_concordance_request()) {
        Ok(reply) => decode_concordance_reply(&reply),
        Err(e) => match deferral_of(&e) {
            Some(d) => {
                println!("concordance: {}", said(d));
                None
            }
            None => {
                eprintln!("{endpoint}: {e}");
                return ExitCode::from(1);
            }
        },
    };
    match served {
        Some(Ok(clauses)) => {
            let named: Vec<String> = clauses.iter().map(|c| format!("{} v{} ({} fixtures)", c.id.law, c.id.version, c.fixtures.len())).collect();
            println!("concordance: {}", named.join(", "));
            match sign_concordance(&key, &clauses, |_| Assent::Enabled) {
                Some(signed) => match session.speak(encode_signature_request(&signed)) {
                    Ok(reply) => {
                        println!("  signed by reconstruction; the hypha's verdict: {:?}", decode_signature_reply(&reply));
                        aligned(&session, &domain);
                    }
                    Err(e) => match deferral_of(&e) {
                        Some(d) => println!("  {}", said(d)),
                        None => {
                            eprintln!("{endpoint}: {e}");
                            return ExitCode::from(1);
                        }
                    },
                },
                None => println!("  a served fixture this house does not reconstruct"),
            }
        }
        Some(Err(r)) => println!("concordance: the hypha cannot read clause {:?} of its register", r.id),
        None => println!("concordance: a reply this build does not read"),
    }

    match session.speak(encode_clock_request()) {
        Ok(reply) => match (read_deferral(&reply), decode_clock_reply(&reply)) {
            (Some(d), _) => println!("clock: {}", said(d)),
            (None, Some(Some(stamp))) => println!("clock: the head at {stamp:?}"),
            (None, Some(None)) => println!("clock: no head yet"),
            (None, None) => println!("clock: a reply this build does not read"),
        },
        Err(e) => match deferral_of(&e) {
            Some(d) => println!("clock: {}", said(d)),
            None => println!("clock: {e}"),
        },
    }

    let genesis = DomainGenesis { domain, tiers: vec![domain] }.sign(&key);
    match session.found_domain(&genesis) {
        Ok(Founded::Sealed { at, receipt }) => println!("domain: sealed at height {}, receipt {}", at.h, hex32(&receipt)),
        Ok(Founded::AlreadyFounded) => println!("domain: founded already"),
        Ok(other) => println!("domain: refused, {other:?}"),
        Err(e) => match deferral_of(&e) {
            Some(d) => println!("domain: {}", said(d)),
            None => println!("domain: {e}"),
        },
    }
    let crossed = reserve_and_cross(&session, &key);

    println!("\nconformance, kernel profile ({endpoint}):");
    let report = run(endpoint, &key, &[Profile::Kernel]);
    print!("{report}");
    match report.conforms() && crossed {
        true => ExitCode::SUCCESS,
        false => ExitCode::from(1),
    }
}

/// The (3,4,5) turn carrying a unit membrane at `(1, 0, 0)`, heights 7 → 8:
/// a transition lawful under every law of the register.
fn lawful() -> Option<Transition> {
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let turn = rotator(&q(2), &q(1));
    let body = BodyMoments { centre: vec![q(1), q(0), q(0)], mass: q(1), moments: vec![vec![q(1), q(0), q(0)], vec![q(0), q(0), q(0)], vec![q(0), q(0), q(0)]] };
    let nucleus = membrane_at(&body.centre, &q(1))?;
    let after = BodyState { nucleus: turn.sandwich(&nucleus), body: carried_body(&body, &turn)? };
    Some(Transition { motion: turn, heights: (7, 8), bodies: vec![(BodyState { nucleus, body }, after)], hues: Default::default(), fluxes: Vec::new() })
}

/// TRACK 24 B3, B4 — THE HOUSE CROSSES ITS UNIVERSAL CORE TUBE: its one
/// tier designated its sitting reserve where it is none yet, then a fold of
/// the reserve stating a lawful transition, which the hypha rebuilds under
/// the universal laws and admits only while the reserve converses on every
/// network clause it signed above. Whether it crossed.
fn reserve_and_cross(session: &Session, key: &mycelium_sdk::prelude::SigningKey) -> bool {
    use mycelium_sdk::house::uplink::{decode_uplink_reply, encode_uplink, TierFold, Uplinked};
    let domain = key.verifying_key().to_bytes();
    if !matches!(session.reserve_reading(domain), Ok(r) if r.current == Some(domain)) {
        let reservation = Reservation { domain, tier: domain, nonce: payload_hash(&domain) }.sign(key);
        match session.reserve(&reservation) {
            Ok(f) => println!("reserve: {f:?}"),
            Err(e) => println!("reserve: {e}"),
        }
    }
    let Some(transition) = lawful() else {
        println!("crossing: the turn carries no body rigidly");
        return false;
    };
    let fold = TierFold {
        tier: domain,
        prefix: payload_hash(b"mycelium house crossing"),
        rows: Vec::new(),
        declared: Vec::new(),
        children: Vec::new(),
        handoffs: Vec::new(),
        transitions: vec![transition],
    };
    match session.speak(encode_uplink(&fold.sign(key))) {
        Ok(reply) => match decode_uplink_reply(&reply) {
            Some(Uplinked::Admitted { at, receipt }) => {
                println!("crossing: a lawful transition through the universal core tube, admitted at height {}, receipt {}", at.h, hex32(&receipt));
                true
            }
            Some(refused) => {
                println!("crossing: refused, {refused:?}");
                false
            }
            None => match read_deferral(&reply) {
                Some(d) => {
                    println!("crossing: {}", said(d));
                    true
                }
                None => {
                    println!("crossing: a reply this build does not read");
                    false
                }
            },
        },
        Err(e) => match deferral_of(&e) {
            Some(d) => {
                println!("crossing: {}", said(d));
                true
            }
            None => {
                println!("crossing: {e}");
                false
            }
        },
    }
}
