// 78.153, defect 4 of the lock audit — A CYCLE OF CITATIONS IS REFUSED BY
// NAME, never a stack overflow: the speech names the cited form it reached
// again while still speaking it, the mint refuses it, and the court's replay
// refuses it as `REFUSE_CITATION_CYCLE`.

use super::common::*;
use crate::transducer::family::*;

/// A crystal holding two forms at two addresses, each citing the other.
struct Mutual {
    e: Element,
    a: (crate::transducer::address::Address, Form),
    b: (crate::transducer::address::Address, Form),
}

impl Crystal for Mutual {
    fn on_class(&self, _class: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        Vec::new()
    }
    fn on_body(&self, _chart: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        Vec::new()
    }
    fn speak_form(&self, f: &Form, x: &Grid, placement: &crate::transducer::alphabet::Word) -> Option<Speech> {
        apply_form(f, x, placement, self)
    }
    fn by_address(&self, addr: &[u8]) -> Option<(&Element, std::borrow::Cow<'_, Form>)> {
        if addr == self.a.0.as_bytes() {
            return Some((&self.e, std::borrow::Cow::Borrowed(&self.a.1)));
        }
        (addr == self.b.0.as_bytes()).then(|| (&self.e, std::borrow::Cow::Borrowed(&self.b.1)))
    }
    fn on_axis(&self, _chart: &Invariant, _axis: crate::transducer::sentence::Axis) -> Vec<crate::transducer::alphabet::Word> {
        Vec::new()
    }
    fn generators_under(
        &self,
        _chart: &Invariant,
        _budget: &mut crate::physics::Budget,
    ) -> Vec<(crate::transducer::address::Address, crate::transducer::address::Address, crate::transducer::alphabet::Word)> {
        Vec::new()
    }
}

/// The fixture: the bar's shift `s`, and two forms — the form at `a` cites
/// `b`, the form at `b` cites `a`. A form's citations are its cover, which
/// its key leaves out (the envelope is evidence), so the mutual incidence the
/// nerve admits is constructible exactly as it stands on a court.
fn mutual() -> (Grid, Grid, Mutual, Form) {
    let (x, y) = (grid(&["1100", "0000"]), grid(&["0011", "0000"]));
    let s = cipher_form(&x, &y);
    let e = crate::transducer::sentence::as_element_form(&s, [1, 0, 0, 0]).expect("tape");
    let a = crate::transducer::address::Address::new(b"the form at a".to_vec());
    let b = crate::transducer::address::Address::new(b"the form at b".to_vec());
    let identity = crate::transducer::alphabet::Word::identity();
    let citing = |target: &crate::transducer::address::Address| {
        let mut f = Form::default();
        f.glue(&s, target.clone(), identity.clone(), e.magnitude()).expect("the closed form glues");
        f
    };
    let (cites_b, cites_a) = (citing(&b), citing(&a));
    let crystal = Mutual { e, a: (a, cites_b.clone()), b: (b, cites_a) };
    (x, y, crystal, cites_b)
}

/// THE SPEECH ENDS, AND NAMES THE CYCLE: speaking the form at `a` speaks
/// `b`, which speaks `a`, which reaches `b` while it is still being spoken.
/// It recursed without end; the speech now names `b`, lays down no grid, and
/// the deflation and the mint read the cycle by name.
#[test]
fn apply_form_refuses_mutual_citation_by_name() {
    let (x, y, crystal, top) = mutual();
    let identity = crate::transducer::alphabet::Word::identity();
    let spoken = apply_form(&top, &x, &identity, &crystal).expect("the speech ends");
    assert_eq!(spoken.cycled, vec![crystal.b.0.clone()], "the form reached again is named by its address");
    assert_eq!(spoken.grid(), None, "a speech that cycles lays down no grid");
    let family = vec![(x, y)];
    let read = form_deflation_read(&top, &family, &crystal, &identity);
    assert_eq!(read.cycled, vec![crystal.b.0.clone()], "the deflation reads the cycle");
    // The court's replay on the same crystal: every phase before the
    // speech passes or is read, and the speech refuses by the cycle's name.
    let word = crate::transducer::sentence::as_element_form(&top, [1, 0, 0, 0]).expect("tape");
    let resolve = |addr: &[u8]| crystal.by_address(addr).map(|(_, f)| f);
    let mut cursor = crate::verdict::ReplayCursor::default();
    let out = crate::verdict::replay_resume(
        &word,
        &resolve,
        Some(&family),
        &crystal,
        &mut crate::physics::Physics::default().stroke_n(1 << 20),
        &mut cursor,
        true,
    );
    assert!(
        matches!(&out, crate::verdict::Replayed::Refused(r) if r.reason == crate::verdict::REFUSE_CITATION_CYCLE),
        "the court refuses the cycle by name: {out:?}"
    );
}
