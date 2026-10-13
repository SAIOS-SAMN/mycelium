// MU-CLIENT §5.4, 78.99, 78.70 — A WORD READ BY ITS COORDINATES: the one
// reader of what a composition teaches, shared by the mind and the court, so
// a mind's dictionary and a court's agree by construction.
//
//! A WORD IS AN ALGORITHM: a composition that produces its result. What a
//! closed sentence teaches (`compose::teaches`) is one word for its whole
//! path; read by its coordinates (78.99), every one of its modes stands on its
//! own hole on its own law (78.98 (1)) —
//!
//! ```text
//!     elliptic, finite order   a TURN t/ℓ       periodic: returns at ℓ
//!     elliptic, no order       a ROSETTE (c, λ)  named: turns forever
//!     parabolic                a COUNT λ         nilpotent: one null motion per traversal
//!     hyperbolic               a BOOST (c, λ)    named: a gain that never returns
//! ```
//!
//! — and the word is the product of its MODE-WORDS, each that mode's ray
//! (EQ1, `Coordinates::word`), in any order: its modes commute (MR1). That is
//! one way a word decomposes; its letters (the peel) and its carriage and
//! rotation (`conformal_factors`) are others. A composition whose coordinates
//! do not read teaches no word, and is refused by name (rule 11).

use super::compose::Composition;
use super::coordinates::{Coordinates, Hole, Unread, Winding};
use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cga::Mv;

/// THE LAW A MODE STANDS ON, read off its hole.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Law {
    /// An elliptic mode of finite order: a turn `t/ℓ`, returning at `ℓ`.
    Periodic,
    /// A parabolic mode: a null count, one motion per traversal.
    Nilpotent,
    /// An elliptic mode of no finite order: a rosette, named by its point.
    Rosette,
    /// A hyperbolic mode: a boost or dilation, named by its point and side.
    Boost,
}

impl Law {
    /// Every law, in the order a reading counts them.
    pub const ALL: [Law; 4] = [Law::Periodic, Law::Nilpotent, Law::Rosette, Law::Boost];

    /// The law one hole stands on. `None` for a trivial mode, which moves
    /// nothing and is no word.
    pub fn of(h: &Hole) -> Option<Law> {
        match (h.kind, &h.winding) {
            (BivectorKind::Elliptic, Winding::Turn { .. }) => Some(Law::Periodic),
            (BivectorKind::Elliptic, _) => Some(Law::Rosette),
            (BivectorKind::Parabolic, _) => Some(Law::Nilpotent),
            (BivectorKind::Hyperbolic, _) => Some(Law::Boost),
            (BivectorKind::Trivial, _) => None,
        }
    }

    /// Its place in [`Law::ALL`].
    pub fn slot(self) -> usize {
        match self {
            Law::Periodic => 0,
            Law::Nilpotent => 1,
            Law::Rosette => 2,
            Law::Boost => 3,
        }
    }
}

/// ONE MODE OF A WORD AS A WORD OF ITS OWN: its hole, the law it stands on,
/// and its ray — the mode-word.
#[derive(Clone, Debug)]
pub struct ModeWord {
    pub hole: Hole,
    pub law: Law,
    pub word: Mv,
}

/// A WORD READ BY ITS COORDINATES: the word, and its mode-words, whose
/// product it is.
#[derive(Clone, Debug)]
pub struct Read {
    pub word: Mv,
    pub modes: Vec<ModeWord>,
}

/// READ A WORD by its coordinates on the stroke its own modes need: the word
/// and its mode-words. `Err` by name where its coordinates do not read; a word
/// that moves nothing has no mode and reads as itself.
pub fn read(word: &Mv) -> Result<Read, Unread> {
    let need = crate::transducer::cga::modes_need(word).ok_or(Unread::NoAction)?;
    let coords = Coordinates::of(word, &mut crate::physics::Physics::default().stroke_n(need))?;
    let modes = coords
        .holes
        .iter()
        .filter_map(|h| Some(ModeWord { law: Law::of(h)?, word: h.ray(), hole: h.clone() }))
        .collect();
    Ok(Read { word: word.clone(), modes })
}

/// THE WORD A COMPOSITION TEACHES, read by its coordinates: `compose::teaches`
/// under the composition's own theorem, then [`read`]. `None` where it teaches
/// nothing or its word does not read.
pub fn taught(s: &Composition) -> Option<Read> {
    let theorem = super::theorem::Theorem::of(s);
    let word = super::compose::teaches(s, &theorem)?;
    read(word).ok()
}
