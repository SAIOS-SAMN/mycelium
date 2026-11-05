// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 — THE CLIENT SDK: the curated surface a mind speaks
// Anastome through, every item the library's own, re-exported.

//! # mycelium-sdk
//!
//! The client surface of the Mycelium network: what a third-party mind needs
//! to take a voice, open a session with a hypha, ask the court for a task,
//! state a rule, submit the solve the court reconstructs and pays, sit a seat in a
//! house, and read its wallet and the hypha's typed readings. Every item is
//! re-exported from the `mycelium` library, so the server and the SDK cannot
//! drift; the SDK holds no protocol logic of its own. A kernel's uplink (its
//! house) speaks through the same SDK: [`house`] carries the domain, universe,
//! endowment, seat, verification and Concordance surface it needs, and the
//! conformance read's kernel profile checks it.
//!
//! ## Modules
//!
//! | Module | What it carries |
//! |---|---|
//! | [`voice`] | The identity: the voice key, loaded or minted; the mesh record of a voice home |
//! | [`session`] | A session with a hypha; the transport aperture; the TUniversal point; the deferral a full hypha answers every act with and its reader; the reference mind, opened on a session |
//! | [`task`] | Task request and envelope; solve with rule; the verdict and its `SOLVE_*` codes |
//! | [`rule`] | The rule seam: typed steps and clauses, extraction, minting and speaking a rule |
//! | [`seat`] | Seats in a house: list, take, call, read mail |
//! | [`wallet`] | Wallet reading; stake and unstake; lease open, reply and close |
//! | [`readings`] | The typed readings a client may ask a hypha for |
//! | [`laws`] | The L0 law register at its `LAW_VERSION`, its read, and the `forms` law ids |
//! | [`house`] | A kernel's uplink (its house): domain founding, the universe, the endowment's manifest and grant, the house side of seats, laws verify, the clock, the Concordance's clause register and signing act, framing |
//! | [`signing`] | A kernel's signing helper: the Concordance's clauses read, each fixture reconstructed through `forms`, and the readings signed clause by clause |
//! | [`conformance`] | The client conformance statement (two profiles, mind and kernel, each with MUST and MAY acts), its in-process check and its suite on a network |
//! | [`forms`] | The law forms of the Concordance's nested shells |
//! | [`prelude`] | The items a client uses first |
//!
//! The crate root carries the wire this SDK speaks: [`WIRE_VERSION`], the
//! [`ACT_TABLE`] the SDK may send, the solve's claim kinds, and the
//! deferral's code [`PENDING_CODE`] with its writer [`encode_deferral`].
//!
//! ## Quick start
//!
//! One task asked, a rule stated on its train pairs, and the solve sent
//! signed by the voice (the whole client is `examples/client.rs`):
//!
//! ```no_run
//! use mycelium_sdk::prelude::*;
//! use mycelium_sdk::{rule, task};
//!
//! let (voice, key) = mycelium_sdk::voice::load_or_mint("home/voice.key");
//! let session = Session::open("127.0.0.1:7500", voice);
//! let reply = session.speak(task::encode_task_request(&key.verifying_key().to_bytes()));
//! let Some(samples) = reply.ok().and_then(|r| task::decode_task_envelope(&r)) else { return };
//! // The envelope carries samples of any rank; the rule seam spells the
//! // alphabet's section.
//! let Ok(envelope) = samples.section() else { return };
//! let mut budget = rule::Physics::default().stroke_n(1 << 20);
//! let Some(stated) = rule::extract_rule(&envelope.train, &mut budget) else { return };
//! let Ok(element) = rule::mint_sentence_why(&stated, &envelope.train, &NoCrystal, 0) else { return };
//! let outputs: Option<Vec<rule::SectionSample>> =
//!     envelope.tests.iter().map(|x| rule::speak_rule(&stated, x, &NoCrystal).and_then(|s| s.sample())).collect();
//! let (Some(outputs), Some(tape)) = (outputs, element.to_bytes()) else { return };
//! let Ok(outputs) = task::spoken_outputs(&outputs) else { return };
//! if let Some(solve) = task::encode_solve_with_rule(&key, &envelope.id, &outputs, Some(&tape)) {
//!     let verdict = session.speak(solve).ok().and_then(|r| task::decode_solve_verdict(&r));
//!     println!("{:?}", verdict.map(|(code, paid, _, _)| (code, paid)));
//! }
//! ```
//!
//! ## Depths
//!
//! A client works at the depth it chooses: Depth 0 runs a node and earns the
//! POWV share; the acts above are Depth 1 (tasks, rules and solves, POUW);
//! citing installed words (POWC) and retiring rewrites (POWR) are spoken
//! through the same session with [`session::Session::speak`].

#![deny(missing_docs)]

pub mod conformance;
pub mod house;
pub mod laws;
pub mod prelude;
pub mod readings;
pub mod rule;
pub mod seat;
pub mod session;
pub mod signing;
pub mod task;
pub mod voice;
pub mod wallet;

/// The law forms of the Concordance's nested shells (the `forms` crate),
/// exact over ℚ.
pub use ::forms;

// MU-PRECURSORS §4.2 — THE WIRE COMPATIBILITY STATEMENT: the wire version
// the SDK speaks and every act it may send, by the library's own codes. An
// act a court sends a court (an offer, a fold, a keeper's tick) is not in
// it.

use mycelium::anastome::dialogue::{
    ACT_ADOPT, ACT_CHALLENGE, ACT_COVER, ACT_DIGESTS, ACT_DOMAIN, ACT_EQUIVOCATION, ACT_EVIDENCE, ACT_FETCH, ACT_HANDOFF, ACT_LAWS, ACT_OBSERVE,
    ACT_PEERS, ACT_PROBE, ACT_QUOTE, ACT_RECEIPTS, ACT_ENTRIES, ACT_SEAT, ACT_SOLVE, ACT_SPEECH, ACT_SYLLABUS, ACT_TASK, ACT_UPLINK, ACT_WALLET,
    ACT_WHY,
};

/// THE SOLVE'S CLAIM KINDS, the byte after its tag (78.126; track 26 S4c):
/// a solve of outputs, or a claim of factorisation.
pub use mycelium::anastome::dialogue::{solve_kind, SOLVE_KIND_FACTORISATION, SOLVE_KIND_OUTPUTS};

/// The wire's version, `major << 8 | minor`: `0x0200` is v2.0, every μ field
/// a reading of the base lanes S, G, T and H, one exact non-negative
/// component each (the `Mu` codec, retiring in v0.5.0 for `Modes`).
pub use mycelium::anastome::dialogue::WIRE_VERSION;

/// EVERY ACT THE SDK MAY SEND, `(code, name)`, in code order.
pub const ACT_TABLE: &[(u8, &str)] = &[
    (ACT_QUOTE, "quote"),
    (ACT_HANDOFF, "handoff"),
    (ACT_EVIDENCE, "evidence"),
    (ACT_TASK, "task"),
    (ACT_SOLVE, "solve"),
    (ACT_SYLLABUS, "syllabus"),
    (ACT_ADOPT, "adopt"),
    (ACT_DIGESTS, "digests"),
    (ACT_FETCH, "fetch"),
    (ACT_PROBE, "probe"),
    (ACT_PEERS, "peers"),
    (ACT_SPEECH, "speech"),
    (ACT_COVER, "cover"),
    (ACT_EQUIVOCATION, "equivocation"),
    (ACT_RECEIPTS, "receipts"),
    (ACT_ENTRIES, "entries"),
    (ACT_OBSERVE, "observe"),
    (ACT_LAWS, "laws"),
    (ACT_WHY, "why"),
    (ACT_WALLET, "wallet"),
    (ACT_UPLINK, "uplink"),
    (ACT_DOMAIN, "domain"),
    (ACT_CHALLENGE, "challenge"),
    (ACT_SEAT, "seat"),
];

/// The pending outcome code a deferral leads with.
pub use mycelium::anastome::dialogue::PENDING_CODE;

/// The deferral's writer (a hypha's): the pending code, no height, the
/// door's deficit per lane, nothing taught. A client reads a deferral with
/// [`session::read_deferral`].
pub use mycelium::anastome::dialogue::encode_deferral;

/// TRANSITIONAL: the deferral's former path, kept only for
/// `examples/house.rs`, which another change owns; the items stand in
/// [`session`], and this module goes when that example reads them there.
#[doc(hidden)]
pub mod deferral {
    pub use crate::session::{deferral_of, read_deferral, Deferral};
}
