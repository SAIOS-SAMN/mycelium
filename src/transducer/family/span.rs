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

use crate::network::nerve::{coord, Keyed};
use crate::transducer::sentence::{section_of_frames, Form, Orbit};

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

/// THE SECTIONS THE MEMBER UNDER ONE CHART STANDS ON: the frames it holds
/// (AT1, ruled — a member holds its frames, and its class's section is the
/// union over its members, so the meet of the two is the member's own). The
/// Frame's chart and a member holding no frames are claimed whole, as the
/// court reads them (51.8). `None` where the chart names no vertex of the
/// form (rule 11: an unread span is not a whole one).
pub(crate) fn chart_span(form: &Form, chart: &coord::ChartAt) -> Option<Span> {
    let coord::ChartAt::At(m) = chart else { return Some(Span::Whole) };
    form.vertices.at(&m.vertex)?;
    let frames = form.member_frames(&m.vertex, &m.chart);
    Some(match frames.is_empty() {
        true => Span::Whole,
        false => Span::Sections(section_of_frames(frames)),
    })
}

/// A CYCLE'S SPAN: the meet of its charts' spans. `None` where a chart is
/// unread.
pub(crate) fn orbit_span(form: &Form, orbit: &Orbit) -> Option<Span> {
    span_of_members(orbit.charts().into_iter(), |c| chart_span(form, &c))
}

/// THE MEET OVER A CELL'S MEMBERS, by whatever reads a member's span — the
/// form's charts in production, a fixture's in the tests.
pub(crate) fn span_of_members<T>(members: impl Iterator<Item = T>, read: impl Fn(T) -> Option<Span>) -> Option<Span> {
    members.map(read).try_fold(Span::Whole, |acc, s| Some(acc.meet(&s?)))
}

/// FR2 — THE SPANS OF A FORM'S CYCLES, per form (§8.1): how many are whole,
/// how many stand on sections — by how many — how many on none, and how many
/// are unread. A reading only.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpanTally {
    pub whole: u64,
    /// `(sections in the span, cycles)`, in the count's own order.
    pub sections: Vec<(usize, u64)>,
    pub none: u64,
    pub unread: u64,
}

impl SpanTally {
    pub fn of(form: &Form) -> SpanTally {
        let mut t = SpanTally::default();
        for o in &form.orbits {
            match orbit_span(form, o) {
                None => t.unread += 1,
                Some(Span::Whole) => t.whole += 1,
                Some(Span::Sections(s)) if s.is_empty() => t.none += 1,
                Some(Span::Sections(s)) => match t.sections.binary_search_by_key(&s.len(), |&(k, _)| k) {
                    Ok(at) => t.sections[at].1 += 1,
                    Err(at) => t.sections.insert(at, (s.len(), 1)),
                },
            }
        }
        t
    }

    pub fn line(&self) -> String {
        let over: Vec<String> = self.sections.iter().map(|(k, c)| format!("{c} over {k}")).collect();
        format!(
            "spans: whole {} | on sections [{}] | on none {} | unread {}",
            self.whole,
            over.join(", "),
            self.none,
            self.unread
        )
    }
}
