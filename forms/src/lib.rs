// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE LAW FORMS — the innermost of the Concordance's NESTED SHELLS (2.10),
//! hosted by mycelium.
//!
//! A form is a law stated over a universe's own constants: it fixes no
//! constant, holds no table of any universe, and computes exactly over ℚ. A
//! universe composes the forms in its own shell and reaches them only
//! through this seam — it calls them, never restates them.
//!
//! WHERE A FORM STANDS (A7, A7a). The core shell — this crate's root — holds
//! exactly the forms EVERY universe selects: L0's law is the intersection of
//! the selections, never the selections' union. A form that one universe alone selects
//! stands in an outer shell even though this crate hosts it, and the shell is
//! named for its law, never for the universe (`forms::shares::…`). A divergence
//! is two outer shells over one shared centre, never a second plane at the
//! core; converging is a new selection, landed by the landing rule.
//!
//! HOW A FORM LANDS (the landing rule). One law, one commit, one JOINT read
//! of both universes: claims by the algebra, and the selecting universes'
//! present readings reproduced as decoded exact values in named frames. Only
//! lawful forms are hosted and selected: derived, exact, composing frame by
//! frame, no invented constant (A7b).
//!
//! OBSERVER (A6). A form fixes no direction or linearity; each is stated with
//! its observer, and a reading moves between frames only by a named exact
//! map, fixed by each universe's own dynamic before any joint read. That map
//! is itself a form ([`frame`]): an exact, invertible conformal map that
//! bends, twists, stretches, expands and contracts — never a floor on a
//! grid, which rounds a reading off and cannot be undone (ruled
//! 2026-10-21). Joint reads compare exact values.

pub mod frame;
pub mod shares;
#[cfg(test)]
mod tests;

/// EVERY SHELL THIS CRATE HOSTS, `(law id, version)`, each read off the
/// shell's own `LAW_ID` and `LAW_VERSION`: the rows the shared-law table
/// (MU-EQUATIONS, *Shared laws*) names the forms by.
pub const LAW_IDS: &[(&str, u16)] = &[(frame::LAW_ID, frame::LAW_VERSION), (shares::LAW_ID, shares::LAW_VERSION)];

/// The exact rationals every form computes over (the same object as
/// mycelium's own `core::Q`).
pub type Q = num_rational::BigRational;
