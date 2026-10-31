// 23.8, 26 — THE DISPATCH: which act a frame is, and which judge it goes to.
//
//! The percept, the gossiped percept, the demand and the routed demand are
//! retired acts (law 8, ruled 2026-09-10): a court hears sentences and
//! nothing else, so a frame carrying one is refused at the door with
//! `REFUSE_NOT_A_SENTENCE` and nothing of it reaches the court. An act
//! this dispatch does not know is not answered.

mod dispatch;
pub(crate) mod equivocation;
pub(crate) mod fold;
pub(crate) mod laws;
pub(crate) mod observe;
pub(crate) mod offer;
pub(crate) mod probe;
pub(crate) mod quote;
pub(crate) mod receipts;
pub(crate) mod rows;
pub(crate) mod speech;
pub(crate) mod syllabus;
pub(crate) mod uplink;
pub(crate) mod challenge;
pub(crate) mod handoff;
pub(crate) mod seat;
pub(crate) mod tick;
pub(crate) mod wallet;
pub(crate) mod domain;
pub(crate) mod why;

#[cfg(test)]
mod tests;

pub(crate) use dispatch::{judge, serve_published};
