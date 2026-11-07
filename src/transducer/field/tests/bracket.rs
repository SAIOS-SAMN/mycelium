// 78.26(2), CB1, ruled on #148 (2026-11-06) — a lost bracket is a demand on
// the docket, naming the two chambers and the bracket; no enclosure is
// formed without a witness (NB1); a cell a witness certifies that holds the
// bracket answers the demand.

use crate::chronometry::Cite;
use crate::network::nerve::keyed::coord;
use crate::network::nerve::Keyed;
use crate::transducer::cech::tests::bracket::{bodies, chamber, stroke, turns};
use crate::transducer::cech::{answering_cell, cells_over, cross_brackets, Bracketed, ChamberAlgebra};
use crate::transducer::field::*;

fn cover() -> Keyed<()> {
    let mut c = Keyed::new();
    c.insert(b"chart", ());
    c
}

/// The docket's bracket entries.
fn brackets_on(c: &Contemplation) -> Vec<Cite> {
    c.docket_cites().into_iter().filter(|e| matches!(e, Cite::Bracket { .. })).collect()
}

/// TWO CHAMBERS MEETING AT A BODY, TURNING ON PLANES THAT SHARE AN AXIS,
/// WITH NO CELL WRAPPING BOTH: their bracket is lost, and it is listed on
/// the docket as a demand naming both chambers by their bodies and the
/// bracket exactly, on the thought's cover. Listed again, it is one entry.
#[test]
fn lost_bracket_is_listed_as_a_demand() {
    let (a, b) = turns();
    let (ca, cb) = (chamber(std::slice::from_ref(&a)), chamber(std::slice::from_ref(&b)));
    let (xa, xb) = (bodies(&[1, 2]), bodies(&[2, 3]));
    let chambers: Vec<(&[Vec<u8>], &ChamberAlgebra)> = vec![(&xa, &ca), (&xb, &cb)];
    let read = cross_brackets(&chambers, &mut stroke());
    let [lost] = read.brackets.as_slice() else { panic!("one bracket: {read:?}") };
    assert_eq!(lost.held, Bracketed::Lost);
    let mut c = Contemplation::new();
    assert!(!c.holds_demand());
    c.docket_brackets(&read, &chambers, &cover());
    let listed = brackets_on(&c);
    let [Cite::Bracket { chambers: named, bracket, cover: Some(on) }] = listed.as_slice() else {
        panic!("one bracket demand on the docket: {:?}", c.docket_cites())
    };
    assert_eq!(**named, [xa.clone(), xb.clone()], "the two chambers, by their bodies");
    // Compared by its coordinates written out (A21).
    let written = |f: &crate::transducer::cga::LinearFunction| {
        let mut out = Vec::new();
        f.put_canonical(&mut out);
        out
    };
    assert_eq!(written(bracket), written(&lost.bracket), "the bracket, exact");
    assert_eq!(on, &vec![b"chart".to_vec()], "on the thought's cover");
    assert!(c.holds_demand(), "a lost bracket is a demand the mind holds");
    // The entry's key is its coordinates, and they read back as the entry.
    let mut key = Vec::new();
    listed[0].put_canonical(&mut key).expect("the entry writes");
    let back = Cite::take_canonical(&key, &mut 0).expect("the entry reads back");
    assert!(back.restates(&listed[0]), "the written entry is the entry");
    c.docket_brackets(&read, &chambers, &cover());
    assert_eq!(c.docket_reading().pending, 1, "one entry by its own coordinates");
}

/// NO ENCLOSURE WITHOUT A WITNESS: the closure of the two chambers' actions
/// would hold the bracket, but no cell wraps both bodies, so nothing answers
/// the demand and no cell is formed for it. A chamber on both chambers'
/// bodies whose algebra does not hold the bracket answers nothing either.
#[test]
fn no_enclosure_without_a_witness() {
    let (a, b) = turns();
    let (ca, cb) = (chamber(std::slice::from_ref(&a)), chamber(std::slice::from_ref(&b)));
    let (xa, xb) = (bodies(&[1, 2]), bodies(&[2, 3]));
    let chambers: Vec<(&[Vec<u8>], &ChamberAlgebra)> = vec![(&xa, &ca), (&xb, &cb)];
    let read = cross_brackets(&chambers, &mut stroke());
    let [lost] = read.brackets.as_slice() else { panic!("one bracket: {read:?}") };
    // The closure of both chambers' actions holds it: an enclosure formed
    // from the two chambers alone would answer, which NB1 refuses.
    let closure = chamber(&[a.clone(), b]);
    assert_eq!(crate::transducer::cech::algebra_contains(&closure, &lost.bracket), Some(true));
    let mut c = Contemplation::new();
    c.docket_brackets(&read, &chambers, &cover());
    assert_eq!(c.answer_brackets(&chambers), Vec::<usize>::new(), "no cell wraps both: nothing answers");
    let named = lost.named(&chambers).expect("both places read");
    assert_eq!(answering_cell(&named, &lost.bracket, &chambers), None);
    // A cell on all three bodies carrying one turn only: it wraps both, and
    // its algebra does not hold the bracket.
    let (xw, one) = (bodies(&[1, 2, 3]), chamber(&[a]));
    let with_one: Vec<(&[Vec<u8>], &ChamberAlgebra)> = vec![(&xa, &ca), (&xb, &cb), (&xw, &one)];
    assert_eq!(c.answer_brackets(&with_one), Vec::<usize>::new(), "a cell that does not hold it answers nothing");
    assert_eq!(brackets_on(&c).len(), 1, "the demand stands");
    assert_eq!(c.docket_reading().answered, 0);
}

/// A WITNESS CERTIFIES THE HIGHER CELL, AND THAT ANSWERS THE DEMAND: an
/// element in which all three bodies stand mints one cell over them (NB1,
/// `cells_over`); read with both turns, that cell holds the bracket, the
/// pair's bracket now reads held by the enclosure, and the docketed demand
/// is answered by name and leaves the docket.
#[test]
fn witnessed_bracket_answers_its_demand_and_mints_the_cell() {
    let (a, b) = turns();
    let (ca, cb) = (chamber(std::slice::from_ref(&a)), chamber(std::slice::from_ref(&b)));
    let (xa, xb) = (bodies(&[1, 2]), bodies(&[2, 3]));
    let before: Vec<(&[Vec<u8>], &ChamberAlgebra)> = vec![(&xa, &ca), (&xb, &cb)];
    let mut c = Contemplation::new();
    c.docket_brackets(&cross_brackets(&before, &mut stroke()), &before, &cover());
    assert!(c.holds_demand());
    // The witness: one element where bodies 1, 2 and 3 stand. NB1 mints the
    // 2-cell over them, keyed by their coordinates.
    let witness = coord::CoverSite { site: crate::transducer::base::Site::from((0, 0, 0)) };
    let standing = [(witness, vec![0usize, 1, 2])];
    let minted = cells_over(standing.iter().map(|(w, m)| (w.clone(), m.as_slice())), |m| u8::try_from(m + 1).ok().map(|b| vec![b]), |_| None);
    let [cell] = minted.cells.as_slice() else { panic!("one cell minted at the witness: {} cells", minted.cells.len()) };
    assert_eq!(cell.coords, bodies(&[1, 2, 3]), "the cell over the three bodies");
    let wrap = chamber(&[a, b]);
    let after: Vec<(&[Vec<u8>], &ChamberAlgebra)> = vec![(&xa, &ca), (&xb, &cb), (cell.coords.as_slice(), &wrap)];
    let reread = cross_brackets(&after, &mut stroke());
    let [held] = reread.brackets.as_slice() else { panic!("one cross pair: {reread:?}") };
    assert_eq!(held.held, Bracketed::Enclosure(2), "the witnessed cell holds the bracket");
    assert_eq!(c.answer_brackets(&after), vec![2], "the witnessed cell answers the demand");
    assert!(brackets_on(&c).is_empty() && !c.holds_demand(), "the demand left the docket");
    assert_eq!(c.docket_reading().answered, 1, "answered by name");
}
