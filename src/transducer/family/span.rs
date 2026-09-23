// 72.1c(3), FR2 — THE SPAN OF A CELL ALONG `e₃`: the sections its relation
// closes over, read from the form's sections at the read and never stored.
//
//! A stratum here is a keyed FIBER of `e₃` coordinates — a set of sections,
//! not a contiguous run — keyed by `frame_key` as `Form.sections` keys them.
//! A transport between two members exists only where both stand (51.4), so a
//! cycle's span is the INTERSECTION of its members' sections: the sections
//! on which every transport it composes exists. The union of what its
//! members touch is the cycle's REACH, a different reading, not built here.
//!
//! THE SPAN IS DERIVED, NEVER CARRIED (law 12). `Form.sections` is the one
//! carrier of which frames a class or a member stands on; the span is the
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
use crate::transducer::sentence::{Acting, Orbit, Vertex};

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

fn declared(section: Option<&Keyed<()>>) -> Span {
    match section {
        Some(s) => Span::Sections(s.clone()),
        None => Span::Whole,
    }
}

/// THE SECTIONS ONE MEMBER STANDS ON: its class's section met with its own,
/// the two coordinates the court reads (51.8) — a class with none is
/// claimed whole, and so is a member with none. `None` where the member key
/// does not name a vertex of `vertices` (rule 11: an unread span is not a
/// whole one).
pub(crate) fn member_span(
    vertices: &[Vertex],
    sections: &Keyed<Keyed<()>, coord::SectionAt>,
    member: &[u8],
) -> Option<Span> {
    let (ix, chart) = super::read_member_key(member)?;
    let v = vertices.get(usize::from(ix))?;
    let class = super::transport_class(v)
        .and_then(|tc| sections.at(&coord::SectionAt::Class(super::class_section_key(&tc))));
    // The member acting on this chart, read at the chart's stratum of the
    // vertex's fibre; a vertex acting on none has no member section.
    let own = v
        .acts()
        .and_then(|acts| acts.fiber_at(&Acting::chart_prefix_bytes(&chart)))
        .and_then(|mut f| f.next())
        .and_then(|(_, acting)| super::member_section_key(v, &acting.chart))
        .and_then(|mk| sections.at(&coord::SectionAt::Member(Box::new(mk))));
    Some(declared(class).meet(&declared(own)))
}

/// A CYCLE'S SPAN: the meet of its members' spans, each member once. `None`
/// where a member is unread.
pub(crate) fn orbit_span(
    vertices: &[Vertex],
    sections: &Keyed<Keyed<()>, coord::SectionAt>,
    orbit: &Orbit,
) -> Option<Span> {
    span_of_members(orbit.members.iter().map(Vec::as_slice), |m| member_span(vertices, sections, m))
}

/// THE MEET OVER A CELL'S MEMBERS, by whatever reads a member's span — the
/// form's sections in production, a fixture's in the tests.
pub(crate) fn span_of_members<'a>(
    members: impl Iterator<Item = &'a [u8]>,
    read: impl Fn(&[u8]) -> Option<Span>,
) -> Option<Span> {
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
    pub fn of(vertices: &[Vertex], sections: &Keyed<Keyed<()>, coord::SectionAt>, orbits: &[Orbit]) -> SpanTally {
        let mut t = SpanTally::default();
        for o in orbits {
            match orbit_span(vertices, sections, o) {
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
