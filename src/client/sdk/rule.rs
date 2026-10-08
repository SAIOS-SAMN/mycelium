// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.142 — THE SDK RULE SEAM: a client states a rule as STEPS — rigid
//! motions in `Cl(4,1)(ℚ)` and hue maps on the fibre — and the seam speaks
//! them as the sentence the court judges. The sentence is built from the
//! family's own perception (its bodies' charts), carried by the derivation
//! the motion peels to, and minted through the one mint
//! (`family::mint_sentence_why`): no Element byte is forged here. Before the
//! word leaves, `would_judge` runs exactly the judge's own steps — the
//! decoder, then `verdict::blind_replay_at` on the pairs given — so a client
//! knows the verdict its rule earns without asking for it.

use crate::arc::Grid;
use crate::core::Q;
use crate::transducer::cga::{self, Mv};
use crate::transducer::element::{self, Element};
use crate::transducer::family::{self, Crystal};
use crate::transducer::sentence::{self, Acting, Form, Vertex};

/// ONE STEP OF A RULE: a motion of the base, or a map of the hue fibre.
#[derive(Clone, Debug)]
pub enum Step {
    /// A versor of `Cl(4,1)(ℚ)` acting on the sites by its sandwich.
    Motion(Mv),
    /// A hue map, `(from, to)`; a hue it does not name stands still.
    Hue(Vec<(u8, u8)>),
}

/// THE SIX GRID SYMMETRIES beside the identity (the dihedral group of the
/// square, read on a rectangle where the shapes fit). Turns are clockwise
/// as a grid is read: `Rotate90` sends row `r`, column `c` of an `R × C`
/// grid to row `c`, column `R − 1 − r`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symmetry {
    Rotate90,
    Rotate180,
    Rotate270,
    /// Mirror left to right: column `c` → `C − 1 − c`.
    FlipH,
    /// Mirror top to bottom: row `r` → `R − 1 − r`.
    FlipV,
    /// Row and column exchanged.
    Transpose,
}

/// WHY NO RULE STANDS, by name.
#[derive(Clone, Debug)]
pub enum NoRule {
    /// A motion step is no versor: it has no inverse, so no action.
    NotAVersor,
    /// A hue step sends one hue to two.
    NotAHueMap,
    /// A motion's linear part did not construct as a congruent thought
    /// (`cga::construct`, T1).
    Unconstructed(cga::NotConstructed),
    /// The motion peels to no word of the alphabet: the demand it leaves.
    Unspelled(sentence::Demand),
    /// The family's pairs do not stand in one frame ratio, or it has no
    /// body to act on.
    NoFamily,
    /// The one mint refused the sentence, by its named refusal.
    Mint(sentence::Refused),
    /// The word is not a sentence the decoder reads, or there is nothing to
    /// judge it against.
    NotASentence,
    /// The judge's replay refused it, with its reason, coordinate and kind.
    Refused(crate::verdict::Refusal),
}

fn q(n: i64) -> Q {
    Q::from_integer(num_bigint::BigInt::from(n))
}

/// THE CENTRE OF AN `R × C` GRID WHERE THE SENSOR READS IT. In site indices
/// the centre is `((R − 1)/2, (C − 1)/2)`; the sensor carries a
/// compartment by its CENTRE, the site plus ½ on each grid axis
/// (`element::carry_site_center`), so the index centre stands at
/// `(R − 1)/2 + ½ = R/2` in the coordinates the versor acts on. Exact.
fn centre((rows, cols): (usize, usize)) -> Option<[Q; 2]> {
    let half = Q::new(num_bigint::BigInt::from(1), num_bigint::BigInt::from(2));
    let index = |n: usize| -> Option<Q> { Some(Q::new(num_bigint::BigInt::from(i64::try_from(n).ok()? - 1), num_bigint::BigInt::from(2))) };
    Some([index(rows)? + &half, index(cols)? + &half])
}

impl Symmetry {
    /// THE LINEAR PART, about the origin, on the base's `e1` (row) and `e2`
    /// (column): a rotor in the `e1e2` plane for a turn — `1 ∓ e1e2` up to
    /// scale, a ray, exact — and a vector for a mirror.
    fn linear(self) -> Mv {
        use crate::transducer::cga::{E1, E2};
        match self {
            Symmetry::Rotate90 => cga::rotator(&q(1), &q(1)),
            Symmetry::Rotate180 => cga::rotator(&q(0), &q(1)),
            Symmetry::Rotate270 => cga::rotator(&q(1), &q(-1)),
            Symmetry::FlipH => Mv::blade(E2, q(1)),
            Symmetry::FlipV => Mv::blade(E1, q(1)),
            Symmetry::Transpose => Mv::blade(E1, q(1)).sub(&Mv::blade(E2, q(1))),
        }
    }

    /// Does the symmetry exchange the grid's two extents?
    fn exchanges(self) -> bool {
        matches!(self, Symmetry::Rotate90 | Symmetry::Rotate270 | Symmetry::Transpose)
    }
}

impl Step {
    /// THE CENTRED RIGID MOTION `T(c′) · L · T(−c)`: the input's centre
    /// `c = ((R−1)/2, (C−1)/2)` taken to the origin, the symmetry's linear
    /// part `L` applied there, and the origin taken to the output's centre
    /// `c′` — half-integers, exact, read in the sensor's compartment-centre
    /// coordinates (each index plus ½, [`centre`]), which is the one frame
    /// the court carries a site in. `None` where the shapes do not fit the
    /// symmetry: a turn by 90° or 270° and the transpose take `R × C` to
    /// `C × R`; the 180° turn and the mirrors keep `R × C`.
    pub fn grid_motion(kind: Symmetry, in_shape: (usize, usize), out_shape: (usize, usize)) -> Option<Step> {
        let fits = match kind.exchanges() {
            true => out_shape == (in_shape.1, in_shape.0),
            false => out_shape == in_shape,
        };
        if !fits || in_shape.0 == 0 || in_shape.1 == 0 {
            return None;
        }
        let [cr, cc] = centre(in_shape)?;
        let to_origin = cga::translator_at(&[-cr, -cc]);
        let [or, oc] = centre(out_shape)?;
        let to_centre = cga::translator_at(&[or, oc]);
        Some(Step::Motion(to_centre.geo(&kind.linear()).geo(&to_origin)))
    }
}

/// The hue `h` lands on under `map`; a hue it does not name stands still.
fn hue_under(map: &[(u8, u8)], h: u8) -> Option<u8> {
    sentence::hue_of(map, h)
}

/// THE STEPS COMPOSED: the motion `V = Vₙ ⋯ V₁` (the first step acts first
/// under the sandwich) and the hue map `σ = σₙ ∘ … ∘ σ₁` on the painted hues.
fn composed(steps: &[Step]) -> Result<(Mv, Vec<(u8, u8)>), NoRule> {
    let mut v = element::identity_versor();
    let mut sigma: Vec<(u8, u8)> = (1..=9u8).map(|h| (h, h)).collect();
    for s in steps {
        match s {
            Step::Motion(m) => {
                // A versor has an inverse; anything else carries no site
                // back and acts as nothing.
                if !cga::is_versor(m) {
                    return Err(NoRule::NotAVersor);
                }
                v = m.geo(&v);
            }
            Step::Hue(map) => {
                for (_, to) in sigma.iter_mut() {
                    *to = hue_under(map, *to).ok_or(NoRule::NotAHueMap)?;
                }
            }
        }
    }
    let moved: Vec<(u8, u8)> = sigma.into_iter().filter(|(h, t)| h != t).collect();
    Ok((v, moved))
}

/// THE SENTENCE OF A RULE ON A FAMILY. The steps compose to ONE action —
/// the motion and the hue map the court will ask every body to be carried
/// by — and that action is placed as one acting vertex: the derivation the
/// motion peels to (A22), its linear part certified by its constructors
/// (`cga::construct`, T1), and one member per chart the family's input
/// bodies stand on (perceived as the extractor perceives them), each with
/// the carriage `D · T` the motion leaves after its rotor and the composed
/// hue map. A member names NO frame: the rule is claimed whole, so it is
/// asked of every pair it is judged on, and no training frame index rides
/// it. Minted through the one mint over the family.
pub fn sentence(steps: &[Step], family: &[(Grid, Grid)], crystal: &dyn Crystal) -> Result<Element, NoRule> {
    let (v, gauge) = composed(steps)?;
    // THE DERIVATION: the rotor the motion's base action is, peeled into
    // the alphabet's letters.
    let word = match family::peel_versor(&v) {
        Some(Ok(w)) => w,
        Some(Err(d)) => return Err(NoRule::Unspelled(d)),
        None => return Err(NoRule::NotAVersor),
    };
    // T1 — the linear part constructs: its reflections meet on its fixed
    // locus and compose back to it as one ray. A motion with no linear part
    // (a translation, the identity) has nothing to construct.
    if let Some(linear) = element::versor_of_word(&word) {
        let identity = linear.sub(&element::identity_versor()).is_zero();
        if !identity {
            let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
            cga::construct(&linear, &mut budget).map_err(NoRule::Unconstructed)?;
        }
    }
    let derivation = crate::transducer::alphabet::Derivation::spelled(word);
    let t = element::translator_of(&v).ok_or(NoRule::NotAVersor)?;
    let d = element::dilator_of(&v).ok_or(NoRule::NotAVersor)?;
    let carriage = d.geo(&t);
    // THE CHARTS THE FAMILY'S INPUT BODIES STAND ON, perceived once.
    let frames = family::perceive_family(family);
    let mut acts: crate::network::nerve::Keyed<Acting, crate::network::nerve::coord::Section> =
        crate::network::nerve::Keyed::new();
    for (ins, _) in &frames {
        for body in ins {
            let member = Acting {
                chart: body.moduli.clone(),
                carriage: Box::new(carriage.clone()),
                gauge: gauge.clone(),
                frames: crate::network::nerve::Keyed::new(),
            };
            acts.put(&member.key(), member);
        }
    }
    if acts.is_empty() {
        return Err(NoRule::NoFamily);
    }
    let ratio = family::family_ratio(family).ok_or(NoRule::NoFamily)?;
    let address = sentence::word_vertex_address(&derivation, &acts, None);
    let mut form = Form { ratio, ..Form::default() };
    form.put_vertex(Vertex::Frame).ok_or(NoRule::NotASentence)?;
    form.put_vertex(Vertex::Word { address, derivation, acts, contact: None }).ok_or(NoRule::NotASentence)?;
    form.orbits = Form::orbits_of(&form.faces, &form.vertices);
    let rule = sentence::rule_of_form(form);
    family::mint_sentence_why(&rule, family, crystal, 0).map_err(NoRule::Mint)
}

/// A RULE CROSSING TO PAIRS IT NEVER SAW (78.142, the I-defect). An
/// extracted rule carries the FRAME-AXIS overlaps of its training family —
/// each joining a chart at training frame `k` to one at `k′`, the motor its
/// body took between two TRAINING PAIRS (`family/percept.rs`, MT1, AT1 A2).
/// That is a relation among the pairs the rule was extracted from, not the
/// rule's action on a pair: judged on a withheld family the court reads
/// those frame indices on pairs that are not the ones they name, and
/// refuses a frame it lacks as a claim over nothing it holds (I). So the
/// word that crosses is the form WITHOUT the overlaps whose two ends stand
/// at different frames, its orbits re-read and re-minted over the training
/// family through the one mint — which re-checks every vertex on every
/// training pair — at the span the word stood over. A rule with no such
/// overlap crosses as it is.
pub fn crossing(rule: &Element, train: &[(Grid, Grid)], crystal: &dyn Crystal) -> Result<Element, NoRule> {
    use crate::network::nerve::coord::ChartAt;
    let held = element::Held::of(rule.clone());
    let form = held.form().ok_or(NoRule::NotASentence)?;
    let across: Vec<_> = form
        .faces
        .entries()
        .filter(|(o, _)| matches!((&o.from, &o.to), (ChartAt::At(x), ChartAt::At(y)) if x.frame != y.frame))
        .map(|(o, _)| o)
        .collect();
    if across.is_empty() {
        return Ok(rule.clone());
    }
    let mut form = form.clone();
    for o in &across {
        let _ = form.faces.drop_at(o);
    }
    form.orbits = Form::orbits_of(&form.faces, &form.vertices);
    let rule_form = sentence::rule_of_form(form);
    family::mint_sentence_why(&rule_form, train, crystal, rule.magnitude()[2]).map_err(NoRule::Mint)
}

/// WHAT THE JUDGE WOULD SAY, by the judge's own steps (`judge_solve`): no
/// pair to replay against is no verdict; the decoder reads the word's form
/// from its address (`Held::of`, which runs `from_element_form`) or it is
/// not a rule; then `verdict::blind_replay_at` on the pairs, its cited
/// forms resolved by `resolve`, spoken through `crystal`.
pub fn would_judge(
    rule: &Element,
    family: &[(Grid, Grid)],
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    crystal: &dyn Crystal,
) -> Result<(), NoRule> {
    if family.is_empty() {
        return Err(NoRule::NotASentence);
    }
    if element::Held::of(rule.clone()).form().is_none() {
        return Err(NoRule::NotASentence);
    }
    let mut budget = crate::physics::Physics::default().stroke();
    match crate::verdict::blind_replay_at(rule, resolve, Some(family), crystal, &mut budget) {
        Some(refusal) => Err(NoRule::Refused(refusal)),
        None => Ok(()),
    }
}
