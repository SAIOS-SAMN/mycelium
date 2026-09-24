// 50.3, 54.5 — THE ONE CIRCUIT of an orbit's member path: what it carries on the
// faces and on the edges, and every reader of it — the edge holonomies and
// their classes, the face transport, coherence, and triviality.

use super::*;
pub use super::refracted::CosetClosure;

impl Form {
    /// THE HOLONOMY OF THE EDGE TRANSPORTS around an orbit (50.3), ONE PER
    /// MEMBER PATH: the product of the face edges along the orbit's faces, in
    /// the base body's stabilizer or exchange; stabilizer cosets do not
    /// telescope, which is what makes it a reading.
    ///
    /// A path carrying a remainder edge anywhere on it is not evaluated and is
    /// absent from the result (13.9); an empty result is "no path of this orbit
    /// is evaluated", which is what `None` used to say for the whole orbit.
    /// `None` itself is now only an orbit naming a face this form does not have.
    pub fn edge_holonomies(&self, lp: &Orbit) -> Option<Keyed<Transport>> {
        edge_holonomies_on(&self.faces, &self.vertices, lp)
    }
}

/// A FACE THAT TOUCHES THE FRAME, asked of the vertex rather than of its
/// slot. The Frame is a `Vertex` variant — it is what a vertex IS — and
/// `f.i == 0` names it by the integer it happens to sit at, which is the
/// vertex's identity collapsed onto a position (rule 15). Nine other sites
/// still spell it that way and are owed the same correction.
fn touches_frame(vertices: &[Vertex], f: &FormFace) -> bool {
    [f.i, f.j]
        .iter()
        .any(|&v| matches!(vertices.get(v as usize), Some(Vertex::Frame)))
}


/// WHAT ONE MEMBER PATH CARRIES: the face composite and the edge composite,
/// read in ONE traversal of the frontier.
///
/// `orbit_transport_on` and `edge_holonomies_on` traversed the same frontier
/// twice — same `live`, same branch on `f.at`, same `step_charts`, same
/// `extend`, same `closed_key` — accumulating different fields. This file
/// already records that correction being made once, for `path_carriages`:
/// "the same frontier, the same `step_charts`, the same `closed_key` — so
/// one quantity had two carriers (law 12) and the traversal that produces it ran
/// twice per orbit." It was never applied to this pair, which is the same
/// two facts about the same structure.
///
/// THE TWO FAIL INDEPENDENTLY, which is why they are two fields and not one
/// composite. A remainder edge drops the EDGE reading for that path and
/// leaves the face composite standing; `edge_holonomies_on` dropped the
/// whole path, and fusing them without this would delete a carriage and a
/// gauge that were measured.
#[derive(Clone)]
pub(crate) struct PathCarried {
    /// The faces' words with the members' carriage and gauge.
    pub(crate) face: Transport,
    /// 50.3's edge composite: `None` where a remainder edge stands on this
    /// path, or where the path crossed no real face and measured no edge.
    pub(crate) edge: Option<Transport>,
    /// THE KINEMATIC CHARGE EACH STEP OF THIS BOUNDARY DEPOSITED, in
    /// boundary order and exact over ℚ (54.7, ruled 2026-09-21).
    ///
    /// `face` and `edge` above are ORDERED PRODUCTS around `∂f`, so a
    /// composite reading identity is already the resonance condition
    /// `Π Ω_e ∼ ±1`. But a product of identities is the identity too, and
    /// the two are opposite physical facts:
    ///
    /// ```text
    ///   RELATOR  some μ_e ≠ 0, and the versors cancel  — geometry closing
    ///   VACUUM   every μ_e = 0                         — no geometry
    /// ```
    ///
    /// A relator is a Seifert–van Kampen boundary word and quotients `π₁`;
    /// a vacuum cycle quotients nothing and says only that the corpus
    /// wrote no transport there. One count for both reports the ABSENCE of
    /// geometry as the CANCELLATION of it (rule 11).
    ///
    /// Kept as a SEQUENCE and never summed: `Σ μ_e` cannot tell one edge
    /// carrying 12 from twelve edges carrying 1, and those are different
    /// circuits. The composed product is no better — `Π Ω_e` is an
    /// irreversible reduction and does not determine its factors.
    ///
    /// AND KEPT AS THE VERSORS, NOT AS THE CHARGES. A first cut stored
    /// `Vec<BigRational>` — the scalar `μ_e` per step — which is a LOSSY
    /// PROJECTION of the step, discarding the bivector plane of the shear,
    /// its orientation, and the Clifford sign. That is LAW-0's "the
    /// gradient is the state; the scalars are projections of it, never
    /// stored as independent fields" (`ipsal-autic`, `orbital/tether.rs`),
    /// and here it is not merely a purity argument: A.8's ANNIHILATION
    /// rule discharges strain only by exhibiting an opposing pair,
    /// `Ω_a · Ω_b ∼ ±1`, and two opposite Burgers vectors have the SAME
    /// `μ`. Storing magnitudes destroys exactly the coordinate that
    /// identifies a partner, so a drain could never find one.
    ///
    /// The charge remains an `O(1)` projection any reader takes on demand:
    ///
    /// ```text
    ///     μ = ⟨Ω⟩₀² / ⟨Ω Ω̃⟩₀ − 1
    /// ```
    ///
    /// SCALE-FREE, which the closed form `(w − 4)/4` is not: a versor is
    /// projective, so `Ω` and `λΩ` are one transport while `w → λ²w`, and
    /// the stored identity is `1` rather than the canonical `1 − v_a v_a =
    /// 2` — under `(w − 4)/4` the identity would read `−3/4` instead of
    /// `0`. On the canonical normalisation `w = 2⟨R⟩₀` the two agree. It
    /// reads the TRANSVECTION factor, not the composed step: the identity
    /// is exact only where no rotation contributes cross-terms, and the
    /// coupling belongs at the orbit, where `Π Ω_e` already evaluates it.
    /// Over the whole group `μ > 0` is hyperbolic, `μ < 0` elliptic
    /// (`−sin²(θ/2)`), `μ = 0` trivial — `I₁`'s own sign convention.
    ///
    /// PL1 — EVERY STEP, IN BOUNDARY ORDER: the face crossed, which way, and
    /// the transport the step carried where it was read. The charge above is
    /// read off each step's carriage by `step_charge`; the circuit's reading
    /// (`trajectory::read_circuit`) is the same steps read whole.
    pub(crate) steps: Vec<Step>,
    /// WHY THE EDGE HALF WAS REFUSED, where it was (H2c, rule 11). The face
    /// composite can stand while the edge composite does not, and the two
    /// refusals are different facts. `NotRigid` is a real face carrying no
    /// edge, a remainder edge where no versor carried the union (50.3).
    /// `Cone` is a face crossed against its orientation whose transport has
    /// no inverse (`MM̃ = 0`, 57.4).
    pub(crate) edge_refused: Option<Unread>,
    /// OR3 — THE STEP, counted from one, at which the edge half was lost:
    /// where the circuit went unread, whatever refused it.
    pub(crate) edge_lost_at: Option<usize>,
    /// BQ5a — THE EDGE HALF THROUGH A CONTACT THAT IS NOT ONE TRANSPORT
    /// (78.6): the composite before the first coset or static step, and
    /// every step from it on, each as crossed — the factors, never their
    /// product, so the `2ᵏ` candidates are formed only by a reader holding
    /// the stroke's budget ([`Refracted`], rule 10). `edge` is `None` then;
    /// this is the edge half, never a refusal of it.
    pub(crate) refracted: Option<super::refracted::Refracted>,
}

/// PL1 — ONE STEP OF A CIRCUIT: the face it crosses, whether along the
/// face's own orientation, and what it carried — `None` where the step's
/// transport was not read (rule 11: unread is not flat).
#[derive(Clone, Debug)]
pub(crate) struct Step {
    pub(crate) face: u16,
    pub(crate) forward: bool,
    pub(crate) transport: Option<Transport>,
}

/// WHY A TRANSPORT WAS NOT READ (H2c, rule 11). One `unread` counter could
/// not say which, and the three are different facts wanting different
/// rulings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Unread {
    /// A carriage on the light cone: `MM̃ = 0` and the inverse does not
    /// exist (57.4).
    Cone,
    /// The face holds no member pair at the two members the circuit names.
    NoPlace,
    /// A face between two bodies that carries no edge, because no versor
    /// carries their union (50.3's `NotRigid`).
    NotRigid,
    /// BQ5a — A CONTACT FACE whose refraction `g = (θ_bθ_a⁻¹)·V` was refused:
    /// the pose or the peel said no, and the contact line names the arm. Not
    /// `NotRigid` — a contact's edge is never the rigidity of the union.
    Unrefracted,
    /// The cycle names a face the form does not hold, or a member key that
    /// does not read. The cycle and the incidence disagree.
    NoFace,
    /// The face relates the two members once each way and the cycle does
    /// not say which it crossed (O1).
    TwoWays,
}

/// THE UNREAD READINGS, split by why (H2c). A measurement and never a gate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnreadTally {
    pub cone: u64,
    pub no_place: u64,
    pub not_rigid: u64,
    pub unrefracted: u64,
    pub no_face: u64,
    pub two_ways: u64,
}

impl Unread {
    /// The refusal by name, for the lines that read it.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Unread::Cone => "cone",
            Unread::NoPlace => "no place",
            Unread::NotRigid => "not rigid",
            Unread::Unrefracted => "unrefracted",
            Unread::NoFace => "no face",
            Unread::TwoWays => "two ways",
        }
    }
}

impl UnreadTally {
    pub(crate) fn note(&mut self, why: Unread) {
        match why {
            Unread::Cone => self.cone += 1,
            Unread::NoPlace => self.no_place += 1,
            Unread::NotRigid => self.not_rigid += 1,
            Unread::Unrefracted => self.unrefracted += 1,
            Unread::NoFace => self.no_face += 1,
            Unread::TwoWays => self.two_ways += 1,
        }
    }
}

/// WHICH WAY A CYCLE CROSSES A FACE, between the two members it names.
pub(super) enum Crossing {
    /// The face's own orientation: the first member is its `i` end. The key
    /// is the member pair the face holds.
    Forward(Vec<u8>),
    /// Against it: the first member is its `j` end.
    Backward(Vec<u8>),
    /// The face relates the two members in both orientations, as two member
    /// pairs, and the cycle does not say which one it crossed.
    TwoWays,
    /// The face holds no member pair between the two.
    Unplaced,
}

/// O1, 70.7 — A FACE IS CROSSED THE WAY ITS MEMBER PAIR RUNS, never the way
/// its vertex pair does.
///
/// The circuit read `f.i == v_i && f.j == v_j`. On a face between two charts
/// of ONE vertex both halves are true whichever way the cycle runs, so it
/// read the pair key backwards half the time. That is the orientation
/// `437d253` lost under the vertex projection, at the circuit this time. H2c
/// measured it: 18 of 18 `no place` refusals over the corpus were such
/// faces, each holding the reversed key. The member pair names the
/// orientation, so it is read: forward where the face holds
/// `(c_from, c_to)` with `from` at its `i` end, and backward where it holds
/// `(c_to, c_from)` with `to` there.
///
/// A face holding BOTH, with two different charts, relates the members
/// twice, once each way. A cycle names the face and not the pair, so it
/// does not say which it crossed. That refuses by name rather than taking
/// one (rule 15). One member pair on one chart, `(c, c)`, is one key and is
/// crossed in the face's own orientation.
pub(super) fn crossing(f: &FormFace, from: (u16, &[u8]), to: (u16, &[u8])) -> Crossing {
    let fwd = pair_key_bytes(from.1, to.1);
    let bwd = pair_key_bytes(to.1, from.1);
    let can_fwd = f.i == from.0 && f.j == to.0 && f.at.contains(&fwd);
    let can_bwd = f.i == to.0 && f.j == from.0 && f.at.contains(&bwd);
    match (can_fwd, can_bwd) {
        (true, false) => Crossing::Forward(fwd),
        (false, true) => Crossing::Backward(bwd),
        (true, true) if fwd == bwd => Crossing::Forward(fwd),
        (true, true) => Crossing::TwoWays,
        (false, false) => Crossing::Unplaced,
    }
}

/// THE ONE CIRCUIT — and it is a TRAVERSAL now, not a search (54.5, as ruled
/// 2026-09-20).
///
/// This was a frontier: seed with every member of the first face, extend
/// through each face's `at` by whichever members continued the path, keep
/// whatever closed. That search existed because `Orbit` named its nodes by
/// VERTEX INDEX and the member coordinates had been projected away, so the
/// traversal had to reconstruct what `orbits_of` already knew.
///
/// It also could not always reconstruct it. Under the projection a 1-cell
/// between two charts at one vertex, `((v,c_a),(v,c_b))`, becomes `(v,v)`
/// and loses its orientation — 62 of 62 live orbits broke there, one step
/// from closing, and `437d253` patched it by trying both directions.
///
/// `Orbit.members` carries the cycle, so there is nothing to search for and
/// nothing to guess: the traversal steps the members it was given, reads each
/// face at the pair those two members name, and composes. ONE path per
/// orbit, always, because a cycle is one cycle.
///
/// THE STAR FACES ARE SKIPPED IN THE EDGE HALF, and the reason written here
/// was wrong (corrected 2026-09-20). It read "a star face is the
/// trivialization every body is read against (2.2)". Chapter 2.2 says no
/// such thing — it defines vertices, faces and holonomies and never
/// mentions the star — and SKIPPING A FACE IS NOT GAUGING IT: §3.6 defines
/// a re-gauging as `g_ij → γ_j · g_ij · γ_i⁻¹`, a conjugation, under which
/// holonomies are conjugated and the identity stays the identity. Dropping
/// a factor out of a product is a different operation with a different
/// answer.
///
/// WHAT IS ACTUALLY TRUE. 50.3's edge is the stabilizer element between two
/// BODIES, and the Frame carries no body at any frame, so `edge_between`
/// measures nothing across a star face — `at(0, c_F, k)` finds nothing and
/// the reading is ABSENT, not identity. The placement of a body against the
/// scene is not lost with it: it is the member's own carriage, and the FACE
/// composite above composes it at every step. Two carriers, one for where a
/// body stands and one for how two bodies relate, and the edge half reads
/// only the second.
///
/// WHAT IS NOT ESTABLISHED, and is not claimed here. The star faces are the
/// spanning forest's first pass, so an orbit's star faces are tree edges —
/// but nothing in this traversal checks that, and a star face reached as a CHORD
/// would be skipped too. A chord is where the holonomy is; gauging one away
/// would delete the obstruction rather than move it.
///
/// A path that crosses no real face measured no edge at all and carries
/// `None` there — answering "identity" would report a measurement never
/// taken.
///
/// `None` is an orbit naming a face this form does not have, or a member pair
/// the face does not relate: the cycle and the incidence disagree, which is
/// a refusal and not a zero (rule 11).
pub(crate) fn paths_carried(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Orbit,
) -> Option<Keyed<PathCarried>> {
    paths_carried_why(faces, vertices, lp).ok()
}

/// VS1 — THE SAME CIRCUIT, READ QUIETLY: no step's charge is noted. A reader
/// that re-reads a circuit under a trial move (a bend) reads what the move
/// does and must not write the census's cumulative counters, or the counters
/// would count the trial's traversals as the corpus's.
pub(crate) fn paths_carried_quiet(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Orbit,
) -> Option<Keyed<PathCarried>> {
    traverse(faces, vertices, lp, false).ok()
}

/// THE ONE CIRCUIT, SAYING WHY IT REFUSED (H2c). [`paths_carried`] is this read
/// with the reason dropped. `orbits_of` asks here, so the `unread` gate is
/// counted by what refused it.
pub(crate) fn paths_carried_why(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Orbit,
) -> Result<Keyed<PathCarried>, Unread> {
    traverse(faces, vertices, lp, true)
}

/// The traversal itself; `note` files each step's charge on the census counters.
fn traverse(faces: &[FormFace], vertices: &[Vertex], lp: &Orbit, note: bool) -> Result<Keyed<PathCarried>, Unread> {
    // THE EDGE COMPOSITE OPENS AT THE IDENTITY AND EVERY STEP CONTRIBUTES.
    // It used to open at `None` unless the orbit crossed a face between two
    // bodies, because the star faces contributed nothing and a star-only
    // orbit would then have reported "identity" for a measurement never
    // taken. Under the hybrid rule below every face carries a transport —
    // the stabilizer between bodies, the member's carriage to the Frame —
    // so a star-only orbit measures its own equivocation instead of
    // vanishing, and `None` now means a step whose transport is genuinely
    // unread (rule 11).
    let mut acc = PathCarried {
        face: Transport {
            word: Word::identity(),
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        },
        edge: Some(Transport {
            word: Word::identity(),
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        }),
        steps: Vec::with_capacity(lp.faces.len()),
        edge_refused: None,
        edge_lost_at: None,
        refracted: None,
    };
    let mut charts: Vec<Vec<u8>> = Vec::with_capacity(lp.members.len());
    for (win, &fi) in lp.members.windows(2).zip(&lp.faces) {
        let f = faces.get(fi as usize).ok_or(Unread::NoFace)?;
        // THE STEP IS NAMED BY ITS TWO MEMBERS. Each carries its vertex and
        // its chart, so the face's pair is read off the members rather than
        // guessed from the face's endpoints — which is the whole of what
        // the projection lost.
        let (vi, ci) = crate::transducer::family::read_member_key(&win[0]).ok_or(Unread::NoFace)?;
        let (vj, cj) = crate::transducer::family::read_member_key(&win[1]).ok_or(Unread::NoFace)?;
        // O1 — THE CROSSING IS READ OFF THE MEMBER PAIR THE FACE HOLDS. A
        // face relating NO member pair stands at no named chart, and the
        // members say so too — both charts empty. It contributes its word
        // and nothing else, crossed the way its vertex pair runs.
        let (forward, place) = match crossing(f, (vi, &ci), (vj, &cj)) {
            Crossing::Forward(pk) => (true, f.at.get(&pk).cloned()),
            Crossing::Backward(pk) => (false, f.at.get(&pk).cloned()),
            Crossing::TwoWays => return Err(Unread::TwoWays),
            Crossing::Unplaced if ci.is_empty() && cj.is_empty() => (f.i == vi && f.j == vj, None),
            Crossing::Unplaced => return Err(Unread::NoPlace),
        };
        charts.push(ci);
        let word_step = match forward {
            true => f.word.clone(),
            false => f.word.inverse(),
        };
        acc.face.word = word_step.compose(&acc.face.word);
        if let Some(place) = &place {
            let (carriage, gauge) = match forward {
                true => (place.carriage.clone(), place.gauge.clone()),
                // Backward by the carriage's reverse, a ray (78.32(1)).
                false => (
                    crate::transducer::cga::is_versor(&place.carriage)
                        .then(|| place.carriage.reverse())
                        .ok_or(Unread::Cone)?,
                    gauge_invert(&place.gauge),
                ),
            };
            acc.face.carriage = carriage.geo(&acc.face.carriage);
            acc.face.gauge = compose_gauge(&gauge, &acc.face.gauge);
        }
        // THE EDGE HALF IS HYBRID, AND EVERY STEP CONTRIBUTES (54.5, as
        // ruled 2026-09-20). A cycle of the member complex crosses two
        // kinds of face and each has its own carrier:
        //
        //   A STAR FACE anchors a member to the Frame. The Frame carries no
        //   body, so there is no stabilizer to measure — and the transport
        //   is not therefore ABSENT: it is the member's own CARRIAGE, where
        //   that body stands in its chart.
        //
        //   AN INTER-BODY FACE relates two bodies, and its transport is
        //   50.3's measured stabilizer element, `place.edge`.
        //
        // This skipped the star faces entirely, which is not the same as
        // gauging them: §3.6 defines a re-gauging as `g_ij → γ_j g_ij γ_i⁻¹`,
        // a CONJUGATION, and dropping a factor out of a product is a
        // different operation with a different answer. Composing the star
        // carriages IS that conjugation — they are the γ — so the composite
        // is the holonomy in the tree gauge rather than the chord's raw
        // transport mistaken for it.
        //
        // AND IT CATCHES WHAT SKIPPING COULD NOT. Two star faces to one
        // member compose `γ₁ · γ₂⁻¹`, which is an equivocation — one body,
        // two transports from the basepoint — and read as identity-and-
        // identity it was exactly nothing.
        let step = match (touches_frame(vertices, f), place.as_ref()) {
            // THE STAR STEP IS THE FACE'S WHOLE TRANSPORT, word included.
            // §3.6's `γ` is a WORD — a re-gauging is `γ_j · g_ij · γ_i⁻¹`
            // — so a star step that contributed the identity word would
            // erase the word-holonomy of every orbit whose bodies measured
            // no stabilizer edge, replacing a real obstruction with
            // nothing. What the face measured is what it contributes.
            (true, Some(p)) => Some(Transport {
                word: f.word.clone(),
                carriage: p.carriage.clone(),
                gauge: p.gauge.clone(),
            }),
            (false, Some(p)) => p.edge.clone(),
            (_, None) => None,
        };
        // THE STEP ITSELF, before it is folded into the product and its
        // identity is lost. Every step is kept, in boundary order, with the
        // face it crosses and which way — a step whose transport was never
        // read is kept as unread rather than as an identity, because unread
        // is not flat (rule 11) — and `acc.edge` below goes `None` for it,
        // so the whole boundary is already refused. The charge is noted
        // where `step_charge` reads one: where the carriage is a versor.
        if let Some(mu) = step.as_ref().and_then(|t| step_charge(&t.carriage)).filter(|_| note) {
            note_step_charge(&mu);
        }
        acc.steps.push(Step {
            face: fi,
            forward,
            transport: step.clone(),
        });
        // WHY THE EDGE HALF GOES, at the face that loses it (H2c): a real
        // face with a place and no edge is not rigid, and a transport whose
        // inverse does not exist is on the light cone. The first refusal is
        // the one that stands.
        // BQ5a — A CONTACT WHOSE EDGE IS NOT ONE TRANSPORT (78.6): the held
        // coset or the static tether its place carries, where no edge formed.
        let refraction = match (&step, place.as_ref()) {
            (None, Some(p)) if !f.interface.is_empty() => p.refraction.clone(),
            _ => None,
        };
        let lost = match (&step, place.is_some(), f.interface.is_empty(), &refraction) {
            (None, true, true, _) => Some(Unread::NotRigid),
            (None, true, false, None) => Some(Unread::Unrefracted),
            _ => None,
        };
        let inverted = |step: Transport| match forward {
            true => Some(step),
            false => Some(step.reverse()),
        };
        match (acc.edge.take(), acc.refracted.take(), step, &refraction) {
            (Some(held), _, Some(step), _) => {
                let step = inverted(step);
                if step.is_none() && acc.edge_refused.is_none() {
                    acc.edge_refused = Some(Unread::Cone);
                }
                acc.edge = step.map(|step| Transport {
                    carriage: step.carriage.geo(&held.carriage),
                    word: step.word.compose(&held.word),
                    gauge: compose_gauge(&step.gauge, &held.gauge),
                });
            }
            // THE FIRST REFRACTED STEP: the edge composite so far is the
            // head, and every later step is kept as a factor.
            (Some(held), _, None, Some(r)) => {
                acc.refracted = super::refracted::Refracted::open(held, r, forward);
            }
            (None, Some(refracted), Some(step), _) => {
                acc.refracted = inverted(step).map(|t| refracted.then(super::refracted::RefractedStep::Fixed(Box::new(t))));
            }
            (None, Some(refracted), None, Some(r)) => acc.refracted = refracted.cross(r, forward),
            _ => {}
        }
        if acc.edge_refused.is_none() {
            acc.edge_refused = lost;
        }
        if acc.edge.is_none() && acc.refracted.is_none() && acc.edge_lost_at.is_none() {
            acc.edge_lost_at = Some(acc.steps.len());
        }
    }
    acc.face.word = acc.face.word.normal_form();
    if let Some(e) = acc.edge.as_mut() {
        e.word = e.word.normal_form();
    }
    let mut out = Keyed::new();
    out.insert(Orbit::path_key(&charts), acc);
    Ok(out)
}

/// The face-pair key of two chart keys, as `face_pair_key` writes it from
/// two `Invariant`s — the same bytes, read from the keys the members
/// already carry rather than from invariants this traversal does not hold.
pub(super) fn pair_key_bytes(ci: &[u8], cj: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, ci);
    crate::transducer::address::put_bytes(&mut buf, cj);
    buf
}

/// THE HOLONOMY OF THE EDGE TRANSPORTS around an orbit (50.3), one per member
/// path: a reader of the one circuit. A path with no edge reading is absent,
/// which is what "not evaluated" means (13.9).
pub(crate) fn edge_holonomies_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Orbit,
) -> Option<Keyed<Transport>> {
    let carried = paths_carried(faces, vertices, lp)?;
    let mut out = Keyed::new();
    for (k, c) in carried.scoped() {
        if let Some(e) = &c.edge {
            out.insert(&k, e.clone());
        }
    }
    Some(out)
}

impl Form {
    /// The classes the edge holonomies belong to (15.4), read up to conjugacy,
    /// one per member path that is evaluated. Keyed by the path, so a caller
    /// asking "what class did this orbit file" gets an answer per path and not
    /// one path's answer for all of them.
    pub fn edge_classes(
        &self,
        lp: &Orbit,
    ) -> Keyed<crate::network::nerve::coord::Holonomy> {
        let mut out = Keyed::new();
        let Some(hs) = self.edge_holonomies(lp) else {
            return out;
        };
        for (k, h) in hs.scoped() {
            // 54.7 R4 — the holonomy producer, the same one the torsion
            // and axis carriers are read at.
            if let Some(c) = h.word.rotor().holonomy_key() {
                out.insert(&k, c);
            }
        }
        out
    }

    /// THE CLASS VECTOR OF AN ORBIT'S EDGE HOLONOMY (12.6 over 50.3): the
    /// four coordinates of the stabilizer composite, as `class_vector`
    /// gives them for the transport composite. An orbit carries BOTH and
    /// neither replaces the other — an extraction's orbit can hold a half
    /// turn on its transports while its edges compose to the identity, and
    /// a cover's orbit is the other way round, since every word it composes
    /// is a sensor-plane word and those commute.
    ///
    /// This is the reading that can occupy `𝒮`: the edge's carriage
    /// composes `D(ρ)` with `T(t)`, which do not commute, so a triangle of
    /// placements is a genuine commutator where a triangle of words is not.
    ///
    /// `None` when the orbit is not evaluated (a remainder edge anywhere on
    /// it) or the holonomy is not a versor.
    pub fn edge_class_vectors(&self, lp: &Orbit) -> Keyed<ClassVector> {
        let mut out = Keyed::new();
        let Some(hs) = self.edge_holonomies(lp) else {
            return out;
        };
        for (k, e) in hs.scoped() {
            if let Some(cv) =
                ClassVector::read(&e.word, e.carriage.clone(), CycleType::of(&e.gauge))
            {
                out.insert(&k, cv);
            }
        }
        out
    }

    /// An orbit's edge holonomy coheres when EVERY EVALUATED MEMBER PATH is the
    /// identity or is filed under its class at the period its order reads —
    /// `0` for infinite order, A19's spelling of it (50.1 step 2, A17, 52.3).
    /// A path that is not evaluated — a remainder edge on it — holds nothing
    /// against the form and is absent; a holonomy with no class files
    /// nowhere and does not cohere.
    pub(crate) fn edge_coheres(&self, lp: &Orbit) -> bool {
        let Some(hs) = self.edge_holonomies(lp) else {
            return true;
        };
        // GRADE 1 IS A GEOMETRIC PREDICATE, NOT A PROGRESS FLAG (54.5, as
        // ruled 2026-09-20). Two defects stood here and they compounded.
        //
        // THE WRONG READER (law 12, the fourth site of one ruling).
        // `orbit_classes` states its own contract: "ONE READER FOR THREE
        // CALLERS — `file_orbit` writes these rows, `Form::coheres` asks
        // whether they are all compact, and `Form::is_cauterized` asks
        // whether what is filed agrees with them", and it names this exact
        // fault as fixed at the third: "a screw was filed at 0 by the first
        // and demanded at 4 by the third." The unification reached
        // `file_orbit`, `orbit_is_congruent` and `is_cauterized`. It did not
        // reach here. This read `h.word.order()` — the BARE ROTOR ORDER,
        // with no carriage and no fixed-point test — while `file_orbit`
        // derives the period through `cech::classify`, which gates on the
        // fixed point and files a screw at `0`. So a screw was filed at 0
        // and demanded at 4. Never equal, so the form never cohered.
        //
        // And every transport a voice measures arrives turning about a
        // body's own centroid: `versor_coset` builds each member as
        // `T(c_b)·D(ρ)·R·T(−c_a)`, so the translation is always there.
        //
        // COHERENCE IS GEOMETRY AND CONSULTS NO FILING (54.5, as ruled
        // 2026-09-20). Two faults stood here and they compounded.
        //
        // THE WRONG READER. `orbit_classes` derives a period through
        // `cech::classify`, which GATES ON THE FIXED POINT and gives a
        // screw `0`; this read `h.word.order()` — the bare rotor order,
        // with no carriage and no fixed-point test — and demanded a filing
        // match THAT. A screw was filed at 0 and demanded at 4. Never
        // equal, so no curved form ever cohered. `orbit_classes` names this
        // exact fault as fixed at its third caller — "a screw was filed at
        // 0 by the first and demanded at 4 by the third" — and this was its
        // fourth site (law 12).
        //
        // AND COHERENCE IS NOT A PERMIT. Even with the right reader this
        // demanded that an ENTRY EXIST, which asks whether a stroke has got
        // around to the orbit. Every field of a `FiledOrbit` is DERIVABLE:
        // `orbit_classes` computes the class from `holonomy_words_on`, the
        // period from `cech::classify`, σ from `orbit_transport_on`, and the
        // chord is the orbit's own. Law 12 says a derivable fact is
        // re-derived and never stored beside the coordinates that determine
        // it — so an annotation's PRESENCE cannot be a geometric property,
        // and a predicate reading it made an open dislocation look like a
        // closed manifold.
        //
        // WHAT COHERES, on the complex and nothing else: identity, a class
        // of FINITE order about a fixed point — `ℤ/ℓℤ`, which is content
        // (law 7) — or an axis the form opened over this orbit. A screw
        // slips and never returns: `g^ℓ = T(ℓτ)` with `τ ∈ ker(I − R)`, so
        // there is no period at which the circuit closes. It is not an
        // error and it is not coherence; it is a `Demand::Dislocation`,
        // carried as a vertex, in the key, saying "these fibrations shear
        // by τ and `T(−τ)` would close them".
        //
        // THE FILING IS NOT DELETED, IT IS RELOCATED. `Element.filing` is
        // the speaker's ATTESTATION — what this mind's stroke discharged
        // against its own budget — and it prices the yield (`fibre_volume`,
        // E2) and answers whether the stroke finished (`is_cauterized`).
        // Those are mind-state and they say so. Geometry is asked here.
        if self.opened_axis_of(lp).is_some_and(|chord| self.axis_rederives(lp, chord)) {
            return true;
        }
        let compact = self.orbit_classes(lp);
        hs.scoped().all(|(_, h)| {
            if h.word.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
                return true;
            }
            // A holonomy with no class is not a versor (15.3) and coheres
            // nowhere; refused, never keyed by the rotor's bytes.
            let Some(class) = h.word.rotor().holonomy_key() else {
                return false;
            };
            // A path `orbit_classes` produced no row for carries no measured
            // transport, and a reading that was not taken holds nothing
            // against the form (rule 11, 13.9 step 7). A row with period 0
            // is the screw: non-compact, and owed a dislocation.
            match compact.iter().find(|row| row.class == class) {
                // COMPACT IS BOTH PLANES CLOSING. It read `period > 0` on
                // the `lcm`, which is non-zero whenever EITHER factor is a
                // root of unity — so an orbit with one plane turning forever
                // read as compact and was never owed a dislocation (54.7).
                Some(row) if row.periods.closes() => true,
                // A FREE PART THE FORM DECLARES IS CARRIED, NOT HIDDEN
                // (54.5, Directive 4). `REFUSE_ORBIT` reads "a free part and
                // NO DISLOCATION DECLARES IT", and this is the clause that
                // makes the second half true.
                //
                // A circuit that never closes is an obstruction, and 50.5's answer
                // to an obstruction is a DEMAND: a vertex, in the key,
                // carried as content, saying "these fibrations shear by τ
                // and `T(−τ)` would close them". That is the opposite of a
                // filing — a filing is a side-table entry about a theorem
                // the geometry already determines, and a demand is a
                // coordinate the address names.
                //
                // So the form may carry its shear honestly and still
                // cohere, and `is_closed` is what keeps the distinction:
                // a sentence with a demand is VALID, never closed (50.1).
                Some(_) => self.declares_dislocation(&class),
                None => true,
            }
        })
    }
}

pub(crate) fn orbit_transport(form: &Form, lp: &Orbit) -> Keyed<Transport> {
    orbit_transport_on(&form.faces, &form.vertices, lp)
}

/// The face composite of every member path: a reader of the one circuit.
///
/// It traversed the frontier itself, beside `edge_holonomies_on` traversing the
/// identical frontier for the other half. `vertices` is taken because the
/// star-face rule belongs to the traversal, not to its readers.
pub(crate) fn orbit_transport_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Orbit,
) -> Keyed<Transport> {
    let Some(carried) = paths_carried(faces, vertices, lp) else {
        return Keyed::new();
    };
    let mut out = Keyed::new();
    for (k, c) in carried.scoped() {
        out.insert(&k, c.face.clone());
    }
    out
}

/// IS THIS CYCLE ALGEBRAICALLY TRIVIAL — identity word holonomy, and every
/// member path's transport exactly `(1, 0, ∅)`?
///
/// `Form::orbits_of` is union-find over faces and returns `b₁` of a graph:
/// every redundant path is a cycle, and a form carried 16 546 of them in
/// ten minutes while 292 reached a classifier. A cycle that encloses
/// nothing occupies no axis of 12.6 and can drive no pivot.
///
/// THE FILTER IS HERE AND NOT IN `orbits_of`. `Form.orbits` carries two
/// readings: the cycle set of the 1-skeleton — a structural fact about
/// which faces closed, asserted by `the_1_skeleton_is_not_a_star_tree` and
/// `the_triangle_among_the_bodies_closes` — and the obstructions a pivot
/// discharges. Dropping trivial cycles at construction served the second
/// and destroyed the first: fifteen tests, two of them asserting incidence
/// structure through `orbits.len()`.
///
/// It is also NOT the whole of `coheres`. An orbit can carry an identity
/// WORD holonomy over edges that compose to a rotation, and `edge_coheres`
/// accepts that only where the class is filed in the form's `torsion`. A
/// cycle carrying any edge transport is not trivial by this reader and
/// reaches every reading that holds it to account.
pub(crate) fn orbit_is_trivial(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Orbit,
    carried: &Keyed<PathCarried>,
) -> bool {
    use crate::transducer::alphabet::Rotor;
    let flat = |t: &Transport| {
        t.word.rotor().projective_eq(&Rotor::identity())
            && element::same_versor(&t.carriage, &element::identity_versor())
            && t.gauge.is_empty()
    };
    // THE EDGE IS A THIRD CARRIER. `orbit_transport_on` composes the
    // faces' words and the members' carriage and gauge;
    // `edge_holonomies_on` reads 50.3's edge transport, which
    // `class_vectors_on` and `edge_coheres` both turn on. An orbit whose
    // word telescopes to the identity over edges that compose to a
    // rotation is an obstruction, and reading only the first two carriers
    // would prune it.
    // AN UNREAD ORBIT IS NOT A FLAT ONE (rule 11). `orbit_transport_on`
    // answers empty where no member path CLOSES, and `all(..)` on nothing
    // is true — so an orbit whose paths do not join up read as trivial and
    // was pruned, which is a refusal reported as a measurement. A cycle
    // is trivial only where a path was read and every one of them is
    // flat.
    if carried.scoped().next().is_none() {
        return false;
    }
    if !lp.holonomy.rotor().projective_eq(&Rotor::identity())
        || !carried.scoped().all(|(_, c)| flat(&c.face))
    {
        return false;
    }
    // THE EDGE READING, AND WHAT AN EMPTY ONE MEANS. `edge_holonomies_on`
    // answers empty in two different situations and the difference decides:
    // an orbit of nothing but star faces crossed no edge, and its reading is
    // the faces' WORDS, already taken above; an orbit that DOES cross a real
    // face and still answers empty had every path dropped on a remainder
    // edge, which is unread, not flat (rule 11). `None` is an orbit naming a
    // face this form does not have, and that is unread too.
    let crosses_a_real_face = lp
        .faces
        .iter()
        .any(|&fi| faces.get(fi as usize).is_some_and(|f| !touches_frame(vertices, f)));
    let mut any_edge = false;
    for (_, c) in carried.scoped() {
        // BQ5a — a refracted edge half is read, and never flat: its gauge
        // half is static or held (78.6).
        if c.refracted.is_some() {
            return false;
        }
        if let Some(e) = &c.edge {
            any_edge = true;
            if !flat(e) {
                return false;
            }
        }
    }
    match any_edge {
        true => true,
        false => !crosses_a_real_face,
    }
}
