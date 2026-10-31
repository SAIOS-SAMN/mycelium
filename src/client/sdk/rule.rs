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

//!
//! 78.143 — THE RULE LANGUAGE GROWS. A rule may SELECT the bodies its steps
//! act on by a typed predicate over each body's own exact coordinates (its
//! class, hues, shell, extent, position, contact), COUNT and COMPARE them in
//! an order it names, ties included (Concordance 1.10), address a REGION
//! the bodies' boundary encloses or touches, CREATE cells relative to them (a line
//! to the border, a template stamped on a seed), and crop the output to one
//! of them. Such a rule is spoken as the form's CLAUSES
//! (`sentence::form::clause`), which the court decodes, speaks and judges
//! on every pair it replays; a rule of motions and hue maps alone is the
//! one word vertex it always was.

use crate::arc::Grid;
use crate::core::Q;
use crate::transducer::cga::{self, Mv};
use crate::transducer::element::{self, Element};
use crate::transducer::family::{self, Crystal};
use crate::transducer::sentence::{self, Form};
use crate::transducer::sentence::{Acting, Vertex};
pub use crate::transducer::family::statement::Unstated;
pub use crate::transducer::sentence::form::clause::{
    About, Act, Clause, Cmp, Coord, Extent, Order, Paint, Pred, Region, Stop, Tie, Value,
};

/// ONE STEP OF A RULE: a motion of the base, or a map of the hue fibre —
/// and since 78.143 the selection, region, creation or crop they act under.
#[derive(Clone, Debug)]
pub enum Step {
    /// A versor of `Cl(4,1)(ℚ)` acting on the sites by its sandwich.
    Motion(Mv),
    /// A hue map, `(from, to)`; a hue it does not name stands still. Under
    /// a selection a hue sent to 0 is erased; on a region, the hue `0` is
    /// sent to is the hue the region is filled with.
    Hue(Vec<(u8, u8)>),
    /// SELECT (COUNT, COMPARE): the steps after it act on the bodies the
    /// predicate selects, until the next selection.
    Select(Pred),
    /// REGION: the hue step after it acts on the region of the selection.
    Region(Region),
    /// A motion read about each selected body's own centre.
    InPlace(Mv),
    /// CREATE: cells placed relative to each selected body.
    Create(Create),
    /// SIZE: the output is the extent of the one selected body.
    Crop(Extent),
    /// SIZE: the output scaled by `up / down`, one of them 1.
    Scale { up: u8, down: u8 },
}

/// WHAT A RULE CREATES relative to a selected body.
#[derive(Clone, Debug)]
pub enum Create {
    /// A line from every cell of the body along `ray`, to its stop.
    Extend { ray: (i8, i8), paint: Paint, stop: Stop },
    /// The one body `template` selects, stamped centre on centre onto the
    /// body, which it replaces.
    Copy { template: Pred },
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
    /// 78.158a U2 — UNJUDGED, NEVER REFUTED: the replay at home holds a unit
    /// no stroke it ran on could read (`verdict::REFUSE_UNREAD`, kind N).
    /// Nothing was ruled: the solve goes to the court, whose replay resumes
    /// on its cursor and answers `SOLVE_UNREAD` until it rules.
    Unread,
    /// 78.158a, 78.157 §3a — CITES UNHELD: the rule cites a word the
    /// resolver given does not hold — at home, a word no court admitted
    /// (`Epigenome::court_word`). The court resolves only the words it
    /// holds, so a home pass on such a citation would be a pass the court
    /// cannot give. Its own outcome, by the cited address; never a pass.
    CitesUnheld(Vec<u8>),
    /// 78.143 — a clause is not stated on a pair of the family, by name: a
    /// tie its order does not name, a coordinate that is not exact, no
    /// single body where one is asked, a cell landing between cells.
    Unstated(Unstated),
    /// A motion asked of a region: a region is a target of hues, not a body.
    NotOnRegion,
    /// The clauses do not close the family they were stated over: the links
    /// they leave on the first pair that has any.
    NotClosed(Vec<(i64, i64, u8, u8)>),
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
    /// scale, a ray, exact — and a vector for a mirror. What
    /// [`Step::InPlace`] reads about each selected body's own centre.
    pub(crate) fn linear(self) -> Mv {
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
    /// `C × R`; the 180° turn and the mirrors keep `R × C`. Public for a
    /// client stating its programs through [`sentence`] (2026-10-22).
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
            // The clause steps are read by `clauses_of`; a rule carrying one
            // never reaches the one-vertex composition.
            _ => return Err(NoRule::NotASentence),
        }
    }
    let moved: Vec<(u8, u8)> = sigma.into_iter().filter(|(h, t)| h != t).collect();
    Ok((v, moved))
}

/// A run of motion and hue steps on one target, composed as it arrives.
struct Pending {
    v: Mv,
    about: About,
    sigma: Vec<(u8, u8)>,
}

impl Pending {
    fn new(about: About) -> Pending {
        Pending { v: element::identity_versor(), about, sigma: (1..=9u8).map(|h| (h, h)).collect() }
    }

    /// The clause the run is: a carriage where it moves, a recolour where it
    /// only maps hues, nothing where it does neither.
    fn act(self) -> Option<Act> {
        let gauge: Vec<(u8, u8)> = self.sigma.into_iter().filter(|(h, t)| h != t).collect();
        let still = self.v.sub(&element::identity_versor()).is_zero();
        match (still, gauge.is_empty()) {
            (true, true) => None,
            (true, false) => Some(Act::Recolour(gauge)),
            (false, _) => Some(Act::Carry { motion: self.v, about: self.about, gauge }),
        }
    }
}

/// THE STEPS AS CLAUSES (78.143), or `None` for a rule of motions and hue
/// maps alone, which is one word vertex. Steps before the first selection
/// act on every body; a run of motions and hue maps on one target composes
/// to one act, the first step acting first; a hue step on a region fills
/// it with the hue `0` is sent to.
fn clauses_of(steps: &[Step]) -> Result<Option<Vec<Clause>>, NoRule> {
    let stated = steps
        .iter()
        .any(|s| {
            matches!(s, Step::Select(_) | Step::Region(_) | Step::InPlace(_) | Step::Create(_) | Step::Crop(_) | Step::Scale { .. })
        });
    if !stated {
        return Ok(None);
    }
    let mut out: Vec<Clause> = Vec::new();
    let mut select = Pred::All;
    let mut region: Option<Region> = None;
    let mut pending: Option<Pending> = None;
    let flush = |pending: &mut Option<Pending>, out: &mut Vec<Clause>, select: &Pred| {
        if let Some(act) = pending.take().and_then(Pending::act) {
            out.push(Clause { select: select.clone(), act });
        }
    };
    for s in steps {
        match s {
            Step::Motion(m) | Step::InPlace(m) => {
                if region.is_some() {
                    return Err(NoRule::NotOnRegion);
                }
                if !cga::is_versor(m) {
                    return Err(NoRule::NotAVersor);
                }
                let about = if matches!(s, Step::InPlace(_)) { About::Body } else { About::Grid };
                if pending.as_ref().is_some_and(|p| p.about != about && !p.v.sub(&element::identity_versor()).is_zero()) {
                    flush(&mut pending, &mut out, &select);
                }
                let p = pending.get_or_insert_with(|| Pending::new(about));
                p.about = about;
                p.v = m.geo(&p.v);
            }
            Step::Hue(map) => match region {
                Some(r) => {
                    let void = crate::arc::ground();
                    let hue = hue_under(map, void).ok_or(NoRule::NotAHueMap)?;
                    if hue == void {
                        return Err(NoRule::NotAHueMap);
                    }
                    out.push(Clause { select: select.clone(), act: Act::Fill { region: r, hue } });
                }
                None => {
                    let p = pending.get_or_insert_with(|| Pending::new(About::Grid));
                    for (_, to) in p.sigma.iter_mut() {
                        *to = hue_under(map, *to).ok_or(NoRule::NotAHueMap)?;
                    }
                }
            },
            Step::Select(p) => {
                flush(&mut pending, &mut out, &select);
                select = p.clone();
                region = None;
            }
            Step::Region(r) => {
                flush(&mut pending, &mut out, &select);
                region = Some(*r);
            }
            Step::Create(c) => {
                flush(&mut pending, &mut out, &select);
                let act = match c {
                    Create::Extend { ray, paint, stop } => Act::Extend { ray: *ray, paint: paint.clone(), stop: *stop },
                    Create::Copy { template } => Act::Copy { template: template.clone() },
                };
                out.push(Clause { select: select.clone(), act });
            }
            Step::Crop(e) => {
                flush(&mut pending, &mut out, &select);
                out.push(Clause { select: select.clone(), act: Act::Crop(*e) });
            }
            Step::Scale { up, down } => {
                flush(&mut pending, &mut out, &select);
                out.push(Clause { select: Pred::All, act: Act::Scale { up: *up, down: *down } });
            }
        }
    }
    flush(&mut pending, &mut out, &select);
    Ok(Some(out))
}

/// THE SENTENCE OF A RULE THAT STATES CLAUSES (78.143): a form of its Frame
/// and the clauses, stated on every pair of the family by name, closing
/// every pair, minted through the one mint. No vertex names a training
/// frame or a training cell: the clauses select on whatever scene they are
/// spoken onto, so the word crosses to pairs it never saw.
fn clause_sentence(clauses: Vec<Clause>, family: &[(Grid, Grid)], crystal: &dyn Crystal) -> Result<Element, NoRule> {
    if clauses.is_empty() || family.is_empty() {
        return Err(NoRule::NoFamily);
    }
    let elements = crate::transducer::sentence::form::rule_element::lower_all(&clauses).ok_or(NoRule::Unstated(family::statement::Unstated::Nested))?;
    for (x, _) in family {
        family::statement::speak_elements(&elements, x, &family::perceive(x)).map_err(NoRule::Unstated)?;
    }
    let mut form = Form { elements, ..Form::default() };
    form.orbits = Form::orbits_of(&form.faces, &form.vertices);
    let read = family::form_deflation_read(&form, family, crystal, &crate::transducer::alphabet::Word::identity());
    if let Some(left) = read.remainders.iter().find(|r| !r.is_empty()) {
        return Err(NoRule::NotClosed(left.clone()));
    }
    let rule = sentence::rule_of_form(form);
    family::mint_sentence_why(&rule, family, crystal, 0).map_err(NoRule::Mint)
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
///
/// A rule that states a selection, a region, a creation or a crop is spoken
/// as the form's clauses instead ([`clause_sentence`], 78.143).
///
/// PUBLIC FOR A CLIENT THAT STATES ITS OWN PROGRAMS IN THE MESH'S TONGUE
/// (2026-10-22, reversing Strip 7 for this path): netstratum's `tongue`
/// mints its solver programs as rule elements only through this
/// constructor, never as forged bytes (its D10b).
pub fn sentence(steps: &[Step], family: &[(Grid, Grid)], crystal: &dyn Crystal) -> Result<Element, NoRule> {
    if let Some(clauses) = clauses_of(steps)? {
        return clause_sentence(clauses, family, crystal);
    }
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
pub(crate) fn crossing(rule: &Element, train: &[(Grid, Grid)], crystal: &dyn Crystal) -> Result<Element, NoRule> {
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
/// not a rule; every word it cites resolves by `resolve` or it CITES
/// UNHELD; then `verdict::blind_replay_at` on the pairs, its cited forms
/// resolved by `resolve`, spoken through `crystal`. A replay no stroke
/// could read is `Unread` — unjudged, never refuted (78.158a U2).
#[cfg(test)]
pub(crate) fn would_judge(
    rule: &Element,
    family: &[(Grid, Grid)],
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    crystal: &dyn Crystal,
) -> Result<(), NoRule> {
    would_judge_with(rule, family, resolve, crystal, &mut crate::physics::Physics::default().stroke())
}

/// [`would_judge`] on the caller's stroke: the replay's rounds are as wide
/// as the caller's stroke where it grants more than a default one
/// (`verdict::blind`'s `to_completion`), and the caller's stroke is left as
/// it stood.
///
/// 78.157 §3a — THE HOME CHECK ON COURT WORDS. Every citation the rule
/// carries (`receipts().cited_addresses()`: the cover's and the
/// derivations') is asked of `resolve` before the replay. The replay itself
/// defers a derivation citation its cover does not hold (the syllabus
/// serves it), so a rule citing a word only this mind holds read clean at
/// home and was refused or deferred at the court; asked here, it is
/// `CitesUnheld` by its address. A client resolves through the court's
/// words alone (`Epigenome::court_word`).
pub(crate) fn would_judge_with(
    rule: &Element,
    family: &[(Grid, Grid)],
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Result<(), NoRule> {
    if family.is_empty() {
        return Err(NoRule::NotASentence);
    }
    if element::Held::of(rule.clone()).form().is_none() {
        return Err(NoRule::NotASentence);
    }
    if let Some(unheld) = rule.receipts().cited_addresses().into_iter().find(|a| resolve(a.as_bytes()).is_none()) {
        return Err(NoRule::CitesUnheld(unheld.as_bytes().to_vec()));
    }
    match crate::verdict::blind_replay_at(rule, resolve, Some(family), crystal, budget) {
        Some(refusal) => Err(NoRule::of_refusal(refusal)),
        None => Ok(()),
    }
}

impl NoRule {
    /// A HOME REPLAY'S REFUSAL, read: `REFUSE_UNREAD` is no reading of the
    /// word (kind N) and is `Unread`, which a client submits; every other
    /// refusal is the judge's, `Refused` with its reason, coordinate and
    /// kind.
    pub(crate) fn of_refusal(refusal: crate::verdict::Refusal) -> NoRule {
        match refusal.reason == crate::verdict::REFUSE_UNREAD {
            true => NoRule::Unread,
            false => NoRule::Refused(refusal),
        }
    }
}
