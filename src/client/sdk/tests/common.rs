// 39 — fixture constructors for the client's tests: a closed turn form and
// a two-body sentence.

/// THE FIXTURES' FRAME (AT1: a member holds its frames): every member a
/// client fixture stands, stands at frame 0.
const FRAME: u64 = 0;

pub fn closed_turn(contact: Option<crate::transducer::alphabet::Contact>) -> crate::transducer::sentence::Form {
use crate::network::nerve::coord;
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{Form, Vertex};
let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("90° turn");
let w = Word::from_letters(&[(q, false)]);
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
let v = Vertex::word_on(
    crate::transducer::alphabet::Derivation::spelled(w),
    crate::transducer::stalk::Invariant::default(),
    Box::new(crate::transducer::element::carriage(&one, &(zero.clone(), zero))),
    vec![(1, 2)],
    [FRAME],
    contact,
);
let c = f.stand_member(&v, FRAME).expect("a word vertex stands");
f.place_overlap(coord::ChartAt::Frame, c, None).expect("star");
f.orbits = crate::transducer::sentence::Form::orbits_of(&f.faces, &f.vertices);
f
}

/// TWO BODIES OF THE IDENTITY'S CLASS, each on its own gauge. AT1 (Q1): one
/// class is ONE vertex, and a member is keyed by its chart and carriage, so
/// two bodies with one chart and one carriage are one member whatever their
/// gauges. TWO BODIES ARE TWO SITES, and the site is the chart (the sentence
/// fixtures' `star_form` convention): body `i` stands on chart `[i + 1]`,
/// each overlapped from the Frame's chart.
pub fn two_body_sentence(gauges: [&[(u8, u8)]; 2]) -> crate::transducer::element::Element {
use crate::network::nerve::coord;
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{as_element_form, Form, Vertex};
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
for (i, g) in gauges.iter().enumerate() {
    let j = u64::try_from(i).expect("a fixture star's body, one per gauge, a handful") + 1;
    let v = Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(Word::identity()),
        crate::transducer::stalk::Invariant::counts(&[j]),
        Box::new(crate::transducer::element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        g.to_vec(),
        [FRAME],
        None,
    );
    let c = f.stand_member(&v, FRAME).expect("a word vertex stands");
    f.place_overlap(coord::ChartAt::Frame, c, None).expect("star");
}
f.orbits = Form::orbits_of(&f.faces, &f.vertices);
as_element_form(&f, [1, 0, 0, 0]).expect("v2 sentence")
}

pub fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 20)
}

/// Two hours of one scene: two Ls apart, then both carried one column along —
/// one motion, a slide, carries every body of the first onto the second.
pub fn two_hours_of_a_slide() -> [(crate::arc::Grid, crate::arc::Grid); 2] {
    let la = [(0usize, 1usize), (1, 1), (2, 1), (2, 2)];
    let lb = [(4usize, 4usize), (5, 4), (6, 4), (6, 5)];
    let hour = |cells: &[(usize, usize)]| -> (crate::arc::Grid, crate::arc::Grid) {
        let draw = |shift: usize| -> crate::arc::Grid {
            (0..7).map(|r| (0..9).map(|c| u8::from(c >= shift && cells.contains(&(r, c - shift)))).collect()).collect()
        };
        (draw(0), draw(1))
    };
    let slid = |cells: &[(usize, usize)]| -> Vec<(usize, usize)> { cells.iter().map(|&(r, c)| (r, c + 1)).collect() };
    [hour(&[&la[..], &lb[..]].concat()), hour(&[slid(&la), slid(&lb)].concat())]
}

/// MU-CLIENT §4.12, §7.1 R2/R3 — A THOUGHT OF `scene` COMES ROUND BY THE
/// FIELD'S OWN MOTION, and the winding acts at it. Two hours of the scene
/// (`two_hours_of_a_slide`) are perceived and held under it; the relation
/// between them carries every body and never returns — a slide — so it
/// ESCAPES, and the drive binds it, here by the word the mind knows for it;
/// the binding is the thought's close. The winding reads its returns
/// (`act_on_returns`) and acts — what was held for the scene's return, and
/// for the mind's own, leaves. Answers how many returned.
pub fn bind_a_thought(mind: &mut crate::client::sdk::Entity, scene: &[u8]) -> u64 {
    use crate::transducer::family::{perceive_family, scene_form_on, NoCrystal};
    let mut field = std::mem::take(&mut mind.field);
    let returned = field.think(|| {
        let mut b = budget();
        for (x, y) in two_hours_of_a_slide() {
            let frames = perceive_family(&[(x.clone(), y.clone())]);
            let f = scene_form_on(&NoCrystal, &mut b, &x, &y, &frames).expect("the hour reads");
            mind.contemplation.hold_perceived(f, &frames, &Vec::new(), &[], [0; 4], scene, &mut b).expect("held");
        }
        let escaping: Vec<crate::transducer::cga::Mv> = mind.contemplation.escaping.scoped().map(|(_, e)| e.generator.clone()).collect();
        assert!(!escaping.is_empty(), "the slide's thought escaped, filed for the drive");
        for g in &escaping {
            crate::transducer::sentence::rung::learn_by(g, &[]);
        }
        let bound = mind.drive(&mut b);
        mind.act_on_returns(&mut b, bound, true)
    });
    mind.field = field;
    returned
}
