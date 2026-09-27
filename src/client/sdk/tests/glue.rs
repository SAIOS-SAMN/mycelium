// 21 step 3, 24.1 — THE CRYSTAL AT A BODY'S CHART: the closed forms a
// perceived body's chart reaches, which is what the glue asks for.

use crate::arc::Grid;
use crate::client::sdk::*;
use crate::transducer::family::{cipher_form, sentence_measure, Crystal, NoCrystal};
use crate::transducer::sentence::as_element_form;

/// The court's stamp that admitted the one word these genomes hold: a word
/// no court admitted has no coordinates on the genome (78.27, 78.49).
fn admitting() -> crate::chronometry::Stamp {
    crate::chronometry::Stamp::new(1, [7; 32], crate::chronometry::Place::GENESIS)
}

fn digits(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// A closed form and one of the body charts its word vertices act on.
fn closed_form_and_body_chart() -> (Element, crate::transducer::stalk::Invariant) {
    let family = vec![(digits(&["11200"]), digits(&["00112"]))];
    let f = cipher_form(&family[0].0, &family[0].1);
    let mag = sentence_measure(&f, &family, &NoCrystal, 0).expect("both bodies are spoken");
    let e = as_element_form(&f, mag).expect("a tape");
    let chart = crate::transducer::sentence::from_element_form(&e)
        .expect("the tape reads back")
        .vertices
        .entries()
        // A WORD VERTEX's chart, asked of its fibre: `charts()` answers for the
        // Frame too — the root stratum — and the Frame has no members.
        .find_map(|(_, v)| v.acts().and_then(|a| a.scoped().next().map(|(_, m)| m.chart.clone())))
        .expect("a word vertex carries the chart of the body it acts on");
    (e, chart)
}

/// 21 step 3: "Crystal is indexed by the section coordinate (moduli)". The
/// glue asks `on_class(body moduli)` and a stood sentence was filed under
/// `chart_of_form` — `[2, vertices, faces, orbits, …]` against a body's
/// `[components, …harmonics…]`, two coordinate systems that meet only at
/// the empty root. The crystal answered the empty vector to every question
/// the glue could ask, so no rule ever cited a held form and no citation
/// was ever paid (26, 27): the fleet's "comprehending 0 of 2 listings".
#[test]
fn a_stood_form_is_found_at_the_chart_of_the_body_it_carries() {
    let (e, chart) = closed_form_and_body_chart();
    let f = crate::transducer::sentence::from_element_form(&e).expect("the tape reads back");
    assert!(
        f.is_closed(),
        "the fixture's form is closed, so the crystal may return it at all"
    );
    let mut epi = Epigenome::new();
    epi.stand_dialect_at(e.clone(), 1, 1)
        .expect("the mind stands what it spoke");
    let found = Crystal::on_body(&epi, &chart);
    assert!(
        found.iter().any(|(held, _, _)| held.address() == e.address()),
        "a mind holding a closed form finds it at the chart of the body that form carries"
    );
}

/// 21 step 3 END TO END on the live crystal: a mind that stood a closed
/// form cites it when it extracts a rule over a family that form closes
/// part of. The same claim `glued_form_replays_its_trajectory_through_the_court_genome`
/// makes of a stub crystal whose `on_class` ignores the chart it is asked
/// at — made here of the epigenome the fleet runs.
#[test]
fn a_mind_that_stood_a_form_cites_it_when_it_extracts_a_rule() {
    let family = vec![
        (digits(&["1100", "0000"]), digits(&["0011", "0000"])),
        (digits(&["0000", "1100"]), digits(&["0000", "0011"])),
    ];
    let held = cipher_form(&family[0].0, &family[0].1);
    let mag = sentence_measure(&held, &family[..1], &NoCrystal, 0).expect("the body is spoken");
    let e = as_element_form(&held, mag).expect("a tape");
    let mut epi = Epigenome::new();
    epi.stand_dialect_at(e.clone(), 1, 1).expect("stood");

    let rule = crate::transducer::family::extract_rule_with(&family, &epi, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts");
    assert!(
        rule.form.cover.iter().any(|edge| edge.target == e.address()),
        "the rule cites the form the mind stood, by address"
    );
}

/// The read is at the coordinate the listing supplies and is a keyed read
/// of that coordinate alone (24.3, law 3): a chart the mind holds no body
/// on answers nothing, so the crystal never becomes a scan of everything
/// held.
#[test]
fn a_chart_the_mind_holds_no_body_on_answers_nothing() {
    let (e, chart) = closed_form_and_body_chart();
    let mut epi = Epigenome::new();
    epi.stand_dialect_at(e, 1, 1).expect("stood");
    let mut other = chart.0.clone();
    other.push(crate::transducer::stalk::ChartAxis::Count(u64::MAX));
    let elsewhere = crate::transducer::stalk::Invariant(other);
    assert!(
        Crystal::on_body(&epi, &elsewhere).is_empty(),
        "the crystal answers at the coordinate asked and nowhere else"
    );
}

/// 21 step 3, 28 (L8r): the body-chart index is DERIVED from the genome,
/// so a mind that restores its genome answers `on_body` exactly as the
/// mind that stood the form did. Left out of `uncrystallize`, the index
/// was live only in a process that had never restarted — and every fleet
/// client boots from its genome, so on the fleet the glue answered nothing
/// again after every start.
#[test]
fn a_restored_genome_answers_at_the_chart_of_the_bodies_it_carries() {
    let (e, chart) = closed_form_and_body_chart();
    let mut epi = Epigenome::new();
    epi.learn(&e.to_bytes().expect("the record writes"), &mut super::common::budget()).expect("the word enters the genome");
    assert!(
        !Crystal::on_body(&epi, &chart).is_empty(),
        "the mind that learned it answers at the body's chart"
    );
    epi.admit_all_at(admitting());
    let (tape, refused) = epi.crystallize();
    assert_eq!(refused.total(), 0, "the word stands at its admitting stamp");
    let mut booted = Epigenome::new();
    assert!(booted.uncrystallize(&tape).is_some(), "the genome restores");
    assert!(
        booted.genome_words().any(|w| w.same_vertex(&e)),
        "the restored genome holds the same vertex"
    );
    assert!(
        Crystal::on_body(&booted, &chart)
            .iter()
            .any(|(held, _, _)| held.same_vertex(&e)),
        "and answers at the body's chart, as the mind that stood it did"
    );
}

/// 28, 50.2 step 2: THE PRESENTATION SURVIVES A RESTART. The generator
/// index is derived from the genome like the atlas, the axis index and
/// the moduli index, so a mind that boots from its genome factors through
/// the same generators the mind that learned them did. Left out of
/// `uncrystallize`, every fleet client would speak the cipher's language
/// after every start — A22 from the identity, a function of the versor
/// alone — and the mind's own history would be invisible in what it said.
///
/// The comparison is projective rotor equality on the resolved word,
/// never bytes (rule 3, 1.5): what must survive is the generator — the
/// vertex it reaches — and not the letters that spell it.
#[test]
fn a_restored_genome_factors_through_the_generators_it_was_taught() {
    let (e, _) = closed_form_and_body_chart();
    let mut epi = Epigenome::new();
    epi.learn(&e.to_bytes().expect("the record writes"), &mut super::common::budget()).expect("the word enters the genome");

    // The presentation is filed at the versor chart of every word the
    // form carries — the coordinate F reads at, and the one the index is
    // keyed by. This family is a pure translation, so those words are the
    // identity and the chart is the root: the claim is not that a
    // generator turns anything, it is that the mind offers the SAME
    // generators after a restart as before one.
    let held = crate::transducer::sentence::from_element_form(&e).expect("the tape reads back");
    let charts: Vec<_> = crate::transducer::sentence::acting_of_form(&held)
        .iter()
        .filter_map(|t| crate::transducer::sentence::versor_chart(&t.word))
        .collect();
    assert!(!charts.is_empty(), "the form carries a word with a versor chart");

    let read = |epi: &Epigenome, chart: &crate::transducer::stalk::Invariant| {
        let mut budget = crate::physics::Physics::default().stroke();
        Crystal::generators_under(epi, chart, &mut budget)
    };

    epi.admit_all_at(admitting());
    let (tape, refused) = epi.crystallize();
    assert_eq!(refused.total(), 0, "the word stands at its admitting stamp");
    let mut booted = Epigenome::new();
    assert!(booted.uncrystallize(&tape).is_some(), "the genome restores");

    let mut offered = 0usize;
    for chart in &charts {
        let taught = read(&epi, chart);
        let after = read(&booted, chart);
        offered += after.len();
        for (_, _, g) in &taught {
            assert!(
                after.iter().any(|(_, _, h)| h.rotor().projective_eq(&g.rotor())),
                "every generator the taught mind offers, the booted mind offers too"
            );
        }
        assert!(
            after
                .iter()
                .all(|(form, _, _)| booted.working_word(form.as_bytes()).is_some()),
            "and every citation it offers names a form it still holds"
        );
    }
    assert!(
        offered > 0,
        "the restored index offers something: an index rebuilt to empty would pass every \
         comparison above and prove nothing"
    );
}
