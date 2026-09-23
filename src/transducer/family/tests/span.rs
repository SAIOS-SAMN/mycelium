// 72.1c(3), FR2 — A cycle's span is the meet of its members' sections, and a
// reading at a stratum holds, covers or is absent without cutting the span.

use crate::network::nerve::Keyed;
use crate::transducer::family::span::{span_of_members, AtStratum, Span};
use crate::transducer::sentence::{frame_key, frames_of_section};

fn frames(ks: &[u16]) -> Keyed<()> {
    let mut s = Keyed::new();
    for &k in ks {
        s.insert(frame_key(k), ());
    }
    s
}

#[test]
fn a_span_is_read_from_the_cycle_and_a_reading_at_a_shorter_stratum_holds() {
    // Three members around one cycle, standing on frames {0,1,2,5}, {1,2,5}
    // and {1,2,3,5}; a fourth claimed whole narrows nothing. The cycle's
    // transports all exist on {1,2,5} only — a set, with a gap, not a run.
    let held: [(&[u8], Option<Keyed<()>>); 4] = [
        (b"a", Some(frames(&[0, 1, 2, 5]))),
        (b"b", Some(frames(&[1, 2, 5]))),
        (b"c", Some(frames(&[1, 2, 3, 5]))),
        (b"d", None),
    ];
    let read = |m: &[u8]| {
        held.iter().find(|(k, _)| *k == m).map(|(_, s)| match s {
            Some(s) => Span::Sections(s.clone()),
            None => Span::Whole,
        })
    };
    let span = span_of_members(held.iter().map(|(k, _)| *k), read).expect("every member is read");
    let Span::Sections(s) = &span else { panic!("sections declared, so not whole: {span:?}") };
    assert_eq!(frames_of_section(s), vec![1, 2, 5], "the intersection, gap and all");

    // A reading at a stratum the span does not fit inside HOLDS, and the span
    // is untouched: it is never cut to the sections that were asked.
    assert_eq!(span.at(&frames(&[1, 2])), AtStratum::Held);
    assert_eq!(frames_of_section(s), vec![1, 2, 5], "the reading changed nothing");
    // Covered where the stratum holds every section; absent by name where it
    // holds none.
    assert_eq!(span.at(&frames(&[0, 1, 2, 3, 4, 5])), AtStratum::Within);
    assert_eq!(span.at(&frames(&[3, 4])), AtStratum::Absent);
    // A cycle every member of which is claimed whole is whole, and held at
    // any stratum a reading names.
    let whole = span_of_members([b"d".as_slice()].into_iter(), read).expect("read");
    assert!(matches!(whole, Span::Whole));
    assert_eq!(whole.at(&frames(&[0])), AtStratum::Held);
    // A member the form does not hold leaves the span unread — never whole.
    assert!(span_of_members([b"a".as_slice(), b"x".as_slice()].into_iter(), read).is_none());
}
