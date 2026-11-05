// 72.1c(3), FR2 — THE SPAN OF A CELL ALONG `e₃`: the sections its relation
// closes over, read from the form's sections at the read and never stored.
//
//! A stratum here is a keyed FIBRE of `e₃` coordinates — a set of sections,
//! not a contiguous run — keyed by `frame_key` as a member's frames are.
//! A transport between two members exists only where both stand (51.4), so a
//! cycle's span is the INTERSECTION of its members' sections: the sections
//! on which every transport it composes exists. The union of what its
//! members touch is the cycle's REACH, a different reading, not built here.
//!
//! THE SPAN IS DERIVED, NEVER CARRIED (law 12). The members' own frames are
//! the one carrier of which frames a member stands on (AT1); the span is the
//! meet of those fibres, `Keyed::meet`, read along the trajectory the
//! sections' shared coordinates trace — no direction preferred and nothing
//! chosen. No `k` is enumerated to find it: the frames it holds are the keys the
//! sections hold.
//!
//! A READING AT A STRATUM NEVER CHANGES THE CONTENT. Asked at a stratum the
//! cell's span does not fit inside, the reading HOLDS (rule 15) with the
//! span whole, or says the cell is ABSENT there by name; it never truncates
//! the cell to the part the stratum names.

use crate::network::nerve::Keyed;

/// WHERE A CELL STANDS ALONG `e₃`.
#[derive(Clone, Debug)]
pub enum Span {
    /// No section is declared on any of its members: each is claimed whole,
    /// as the court reads a vertex whose class and member carry no section
    /// (51.8), and so is the cell.
    Whole,
    /// The sections, keyed by `frame_key`. Empty is a reading: the cell's
    /// transports never stand on one section together.
    Sections(Keyed<()>),
}

/// A SPAN READ AT A STRATUM.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtStratum {
    /// Every section of the span is in the stratum: the reading covers it.
    Within,
    /// The span reaches past the stratum. The reading holds, and the span is
    /// untouched — the cell is never cut to the sections that were asked.
    Held,
    /// The span and the stratum share no section: the cell is not there.
    Absent,
}

impl Span {
    /// The sections on which both stand.
    pub fn meet(&self, other: &Span) -> Span {
        match (self, other) {
            (Span::Whole, s) | (s, Span::Whole) => s.clone(),
            (Span::Sections(a), Span::Sections(b)) => Span::Sections(a.meet(b)),
        }
    }

    /// THIS SPAN READ AT `stratum`. A whole cell claims every section and a
    /// stratum names some, so it is held.
    pub fn at(&self, stratum: &Keyed<()>) -> AtStratum {
        let Span::Sections(s) = self else {
            return AtStratum::Held;
        };
        let shared = s.meet(stratum);
        match (shared.is_empty(), shared.len() == s.len()) {
            (true, _) => AtStratum::Absent,
            (false, true) => AtStratum::Within,
            (false, false) => AtStratum::Held,
        }
    }
}

/// THE MEET OVER A CELL'S MEMBERS, by whatever reads a member's span (a
/// fixture's in the tests).
#[cfg(test)]
pub(crate) fn span_of_members<T>(members: impl Iterator<Item = T>, read: impl Fn(T) -> Option<Span>) -> Option<Span> {
    members.map(read).try_fold(Span::Whole, |acc, s| Some(acc.meet(&s?)))
}
