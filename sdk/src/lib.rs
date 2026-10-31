// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 — THE CLIENT SDK: the curated surface a mind speaks
// Anastome through, every item the library's own, re-exported.

//! # mycelium-sdk
//!
//! The client surface of the Mycelium network: what a third-party mind needs
//! to take a voice, open a session with a hypha, ask the court for a task,
//! state a rule, submit the solve the court replays and pays, sit a seat in a
//! house, and read its wallet and the hypha's typed readings. Every item is
//! re-exported from the `mycelium` library, so the server and the SDK cannot
//! drift; the SDK holds no protocol logic of its own.
//!
//! ## Modules
//!
//! | Module | What it carries |
//! |---|---|
//! | [`voice`] | The identity: the voice key, loaded or minted; the mesh record of a voice home |
//! | [`session`] | A session with a hypha; the transport aperture; the TUniversal point |
//! | [`task`] | Task request and envelope; solve with rule; the verdict and its `SOLVE_*` codes |
//! | [`rule`] | The rule seam: typed steps and clauses, extraction, minting and speaking a rule |
//! | [`seat`] | Seats in a house: list, take, call, read mail |
//! | [`wallet`] | Wallet reading; stake and unstake; lease open, reply and close |
//! | [`readings`] | The typed readings a client may ask a hypha for |
//! | [`mind`] | The reference mind, opened on a session |
//! | [`wire`] | [`WIRE_VERSION`] and the [`ACT_TABLE`] the SDK may send |
//! | [`forms`] | The law forms of the Concordance's nested shells |
//! | [`prelude`] | The items a client uses first |
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
//! let Some(envelope) = reply.ok().and_then(|r| task::decode_task_envelope(&r)) else { return };
//! let mut budget = rule::Physics::default().stroke_n(1 << 20);
//! let Some(stated) = rule::extract_rule(&envelope.train, &mut budget) else { return };
//! let Ok(element) = rule::mint_sentence_why(&stated, &envelope.train, &NoCrystal, 0) else { return };
//! let outputs: Option<Vec<Grid>> =
//!     envelope.tests.iter().map(|x| rule::speak_rule(&stated, x, &NoCrystal).and_then(|s| s.grid())).collect();
//! let (Some(outputs), Some(tape)) = (outputs, element.to_bytes()) else { return };
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

pub mod mind;
pub mod prelude;
pub mod readings;
pub mod rule;
pub mod seat;
pub mod session;
pub mod task;
pub mod voice;
pub mod wallet;
pub mod wire;

/// The law forms of the Concordance's nested shells (the `forms` crate),
/// exact over ℚ.
pub use ::forms;
pub use wire::{ACT_TABLE, WIRE_VERSION};
