// 23, 42 — REPLAY: a spoken form is re-derived from its own tape, move by
// move, against the cover that resolves what it cites.
//
//! Every vertex re-derives from its tape, every face from its vertices,
//! every orbit's holonomy to the identity, and every pivot certificate from
//! the axis it names. A form that does not is REFUSED, never scored.

use super::body::undeclared_remainder;
use super::*;

/// Chapter 23.1: replay a crystallized form from its tape and the forms it
/// cites. `resolve` is the hypha's ruled cover (a caller without one
/// resolves nothing, and every citation refuses): each cited form must be
/// ruled, closed, at the depth the tape claims and strictly below the
/// citer's, with an identity frame (chapter 32); every face re-derives from
/// its two vertices by M1, or is the chord of an orbit whose holonomy it
/// carries and whose class is filed on torsion; every orbit of the
/// recomputed spanning forest is projectively identity or filed finite
/// torsion. The trajectory is checked by the caller on the element.
pub fn replay_form(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    budget: &mut crate::physics::Budget,
) -> Option<&'static str> {
    replay_form_with(spoken, resolve, None, &crate::transducer::family::NoCrystal, budget)
}

/// Chapter 23.1 with the bound family, so rebase/unfold/orbit execute on
/// the trains when the kernel has them (step 6). The trajectory is measured
/// through `crystal` (the court's genome). `NoCrystal` is only for a tape
/// with no Glued vertex. The reason alone; `replay_form_at` carries the
/// coordinate.
pub fn replay_form_with(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<&'static str> {
    replay_form_at(spoken, resolve, family, crystal, budget).map(|r| r.reason)
}

/// 50.2 step 2 at the court. Each cited `(form, vertex)` is TWO keyed
/// reads and no whole-carrier read (law 3): the cover resolves the form by address, and
/// the form's own `by_address` resolves the vertex inside it. `Composes`
/// passes; `Differs` refuses at that vertex; `Unresolved` is the cited
/// form the court has not ruled, which takes `REFUSE_GLUED_UNRULED` on
/// the tape — the same deferral a `Glued` citation takes, so the sentence
/// is docketed and the syllabus serves the generator rather than the
/// voice being refused for speaking ahead of this court's cover.
/// DOES THIS FACE RE-DERIVE FROM ITS TWO VERTICES — the word once, and the
/// placement and hue map at every member pair the face records (D10).
///
/// The pairs are the face's own: re-deriving them from the vertices would take
/// the product of the two fibres, and which bodies a face relates is the
/// scene's fact, not something the court recomputes. A pair the two vertices no
/// longer relate does not re-derive, and the face refuses.
/// TWO PLACES ARE ONE PLACE, projectively (§1.5, 54.5 as ruled
/// 2026-09-20).
///
/// `FacePlace` derives `PartialEq`, so `p == *place` demanded BYTE
/// equality of a versor. §1.5 is titled "byte-exactness is not identity"
/// and this site never got it.
///
/// It matters exactly where a carriage has been through the key. A versor
/// acts by the sandwich `x ↦ V x Ṽ`, and for any `λ ∈ ℚ×`
///
/// ```text
///     (λV) x (λV)~ = λ² (V x Ṽ)
/// ```
///
/// so a ray of versors is ONE transformation. A4 canonicalization picks a
/// representative of that ray, and canonicalization is not a homomorphism:
///
/// ```text
///     can(a)·can(b)⁻¹ = λ · can(a·b⁻¹),  some λ ∈ ℚ×
/// ```
///
/// So `face_rederives` recomputing a member pair's RELATIVE carriage from
/// two canonical absolutes gets the right geometry and the wrong scalar,
/// and an exact comparison refuses it. That is not an approximation being
/// admitted — `same_versor` is `projective_ratio(a,b).is_some()`, an exact
/// rational ratio over ℚ× with no tolerance anywhere. It is the quotient
/// the algebra already is.
///
/// The gauge is a permutation of hues and is compared exactly, because a
/// permutation has no ray.
fn same_place(
    a: &crate::transducer::sentence::FacePlace,
    b: &crate::transducer::sentence::FacePlace,
) -> bool {
    use crate::transducer::element::same_versor;
    if !same_versor(&a.carriage, &b.carriage) || a.gauge != b.gauge {
        return false;
    }
    match (&a.edge, &b.edge) {
        (None, None) => true,
        // `Word`'s own equality is already M1's — the rotor up to `λ ∈ ℚ×`,
        // either sign — so the edge's word needs no separate reading.
        (Some(x), Some(y)) => {
            x.word == y.word && same_versor(&x.carriage, &y.carriage) && x.gauge == y.gauge
        }
        _ => false,
    }
}

/// AN OVERLAP RE-DERIVES from its two charts (AT1): its word is derived on
/// read and holds nothing to compare, so what is asked is that both ends
/// stand and that the PLACE re-derives from the members at the overlap's
/// frame. A frame-axis overlap's place is the motor its member's body took
/// between two frames — measured content its producer names, which the
/// vertices do not derive — so only its ends are asked here.
///
/// WHICH BRANCH REFUSED is typed (A.17): three different defects, carried
/// into the `REFUSE_FACE` refusal's obstruction and counted with it.
fn face_rederives(
    f: &Form,
    o: &crate::network::nerve::coord::Overlap,
    face: &crate::transducer::sentence::FormFace,
) -> Result<(), sentence::Refused> {
    use sentence::Refused;
    let (Some(_), Some(_)) = (f.vertices.at(&o.from_vertex()), f.vertices.at(&o.to_vertex())) else {
        return Err(Refused::FaceEndNotVertex);
    };
    if face.producer == crate::transducer::sentence::Producer::Motor {
        return Ok(());
    }
    // The edge is evidence the court re-measures against the FAMILY
    // (`edges_rederive_frames`), not something the vertices re-derive, so it
    // is carried in and compared by its presence alone here.
    match f.face_place(&o.from, &o.to, face.place.edge.clone()) {
        Some(p) if same_place(&p, &face.place) => Ok(()),
        Some(_) => Err(Refused::FacePlaceNotRederived),
        None => Err(Refused::FacePlaceUnderived),
    }
}

/// 50.2 step 2 — THE CITATIONS COMPOSE TO THE ACTION THE VERTEX CARRIES.
///
/// THE EVIDENCE COMES FROM THE ENVELOPE AND THE ACTION FROM THE FORM (54.5
/// Road (d), B3), and that separation is the check rather than a relocation
/// of it. The rotor is the vertex's KEY — §1.5 makes the composed action the
/// coordinate, so two minds with different presentations reach one address —
/// and the citations are a presentation, which the key deliberately excludes
/// (`evidence_is_not_identity`). A mind that lies about its citations is
/// caught here by composing them against a rotor it did not get to write;
/// a mind that lies about the rotor has renamed its own word.
///
/// A VERTEX WITH NO CITE IS SPELLED, and composes vacuously — which is
/// exactly what the vertex the key rebuilds is. That is the whole reason the
/// envelope has to exist: reading the citations off the form meant reading
/// them off the tape, and a form decoded from its address has none.
/// [`derivations_compose`] by name, for the pre-B6 parity oracle. The
/// reader is otherwise reached only through the whole verdict, and the
/// guard has to ask it on its own so a divergence has ONE line.
pub fn derivations_compose_at(
    f: &Form,
    env: &crate::transducer::element::ReceiptsEnvelope,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
) -> Option<Refusal> {
    let owned = |a: &[u8]| resolve(a).map(std::borrow::Cow::Owned);
    match derivations_compose(f, env, &owned, &mut Meter::Free, 0) {
        Err(Halt::Refused(r)) => Some(r),
        Err(Halt::Spent(_)) | Ok(()) => None,
    }
}

/// [`replay_certificates`] with the chord index built from the form the
/// certificates are being replayed against — which is the coordinate the
/// parity oracle is testing, since a chord is an index into `faces` and a
/// decoder that reordered them would diverge here and nowhere else.
pub fn replay_certificates_at(
    f: &Form,
    certificates: &[crate::transducer::sentence::PivotCertificate],
) -> Option<&'static str> {
    replay_certificates(f, certificates, &chord_orbits(f))
}

/// 78.153 — one read per cite, from the `from`-th of the envelope's order;
/// the cited form is BORROWED from the court where it holds it, never cloned
/// per citation.
pub(crate) fn derivations_compose<'r>(
    f: &Form,
    env: &crate::transducer::element::ReceiptsEnvelope,
    resolve: &dyn Fn(&[u8]) -> Option<std::borrow::Cow<'r, Form>>,
    meter: &mut Meter<'_>,
    from: usize,
) -> Step {
    use crate::transducer::alphabet::Verified;
    for (i, cite) in env.derivations.iter().enumerate().skip(from) {
        if !meter.spend() {
            return Err(Halt::Spent(i));
        }
        // THE VERTEX THE CITE NAMES, in this form. A cite naming a vertex
        // the form does not carry is evidence about nothing; it is skipped
        // rather than refused, because the envelope is per-mind and a mind
        // may carry a cite for a vertex a later stroke replaced.
        // THE CITE'S BYTES BECOME A MEMBER COORDINATE HERE, at the court's
        // boundary, and a cite whose bytes are not one is evidence about
        // nothing exactly as an unheld vertex is (54.7 R5, rule 11).
        // Read at the vertex's own coordinate — the class its address's
        // rotor names (AT1) — and held only where the address is the one
        // the vertex carries.
        let Some(v @ Vertex::Word { derivation, .. }) = crate::transducer::sentence::vertex_cited(f, cite.vertex.as_bytes())
        else {
            continue;
        };
        let Some(vertex) = v.at() else {
            continue;
        };
        // The derivation reassembled: the citations from the envelope, the
        // action from the key.
        let derivation = cite.against(&derivation.rotor());
        // THE CHART IN THE QUESTION (D10 item 10). A derivation is a vertex
        // coordinate, so the composition is asked once; what the refusal NAMES
        // is a chart, and the chart a vertex acting on several is refused at is
        // the one its derivation was read for — which is the vertex's whole
        // fibre, because the derivation is the same at each. A refusal at the
        // representative named one chart of several arbitrarily, so this names
        // every chart it acts on, one refusal each, and the first is the one
        // returned.
        // The refusal's chart is the LEAST the vertex stands at, and the slice
        // pattern says so: `charts()` yields them in the fibre's trie order,
        // which is the chart keys sorted, so the least is CONTENT and the same
        // on every node — not the one that happened to arrive first (rule 15).
        let charts = v.charts();
        let [class, ..] = charts.as_slice() else {
            continue;
        };
        // The ACTION at the cited vertex (L14). A citation of an adjoined
        // generator resolves here exactly as one of a spelled generator
        // does, because rotors compose whether or not the seventeen
        // spell them.
        let held = |form: &crate::transducer::address::Address,
                    vertex: &crate::transducer::address::Address| {
            let cited = resolve(form.as_bytes())?;
            sentence::rotor_at_vertex(&cited, vertex.as_bytes())
        };
        match derivation.verify_with(&held) {
            Verified::Composes if carriage_claim_holds(v, &derivation, resolve) => {}
            Verified::Composes | Verified::Differs => {
                return Err(Halt::Refused(Refusal::new(
                    REFUSE_DERIVATION,
                    sentence::At::vertex(vertex.clone(), class.clone()),
                )))
            }
            // The door (`hear_sentence`) decides the WAIT before any
            // replay runs, so an unruled cited FORM never reaches here.
            // What reaches here is a citation naming a vertex the ruled
            // form does not carry — the form resolved and the vertex was
            // not in it — which is a claim about the tape and a refusal
            // at the vertex that made it, not a deferral.
            Verified::Unresolved { .. } => {
                return Err(Halt::Refused(Refusal::new(
                    REFUSE_DERIVATION,
                    sentence::At::vertex(vertex.clone(), class.clone()),
                )))
            }
        }
    }
    Ok(())
}

/// MU-CLIENT §5.4, D4 — A LETTER THAT TURNS NOTHING IS CITED FOR ITS
/// CARRIAGE. `verify_with` composes the cited ROTORS (L14), and a motion
/// letter's rotor is the identity (78.68 row 8): its whole action is the
/// slide or dilation on its vertex, so a citation of it at any exponent
/// composed — a check that checked nothing. Where a derivation cites such a
/// letter it claims the whole motion, and the claim is replayed whole: the
/// cited vertices' motions at their exponents, times the residue's letters at
/// the identity carriage, against the citing vertex's motion, exact (1.5).
/// A derivation citing only letters that turn claims its rotor, as before.
///
/// THE EXPONENT COMES OFF THE TAPE (rule 10). A unipotent motion (a slide)
/// grows in height as `log |e|`, so its power costs the exponent's bits. Any
/// other has a factor whose height grows with `|e|` itself, so a power whose
/// exponent passes the bits the target's own coordinates are written in is
/// refused: no such power is the target, short of a cancellation against
/// another citation, which a speaker writes in its least exponents.
fn carriage_claim_holds<'r>(
    v: &Vertex,
    derivation: &crate::transducer::alphabet::Derivation,
    resolve: &dyn Fn(&[u8]) -> Option<std::borrow::Cow<'r, Form>>,
) -> bool {
    use crate::transducer::cga::Mv;
    use crate::transducer::sentence::presentation::{motion_key, motion_power, vertex_motion};
    let identity = crate::transducer::alphabet::Rotor::identity();
    let mut powers: Vec<(Mv, i64)> = Vec::with_capacity(derivation.generators.len());
    let mut claims_carriage = false;
    for (form, vertex, e) in &derivation.generators {
        let Some(cited) = resolve(form.as_bytes()) else { return false };
        let Some(g @ Vertex::Word { derivation: d, .. }) = sentence::vertex_cited(&cited, vertex.as_bytes()) else {
            return false;
        };
        claims_carriage |= d.rotor().projective_eq(&identity);
        let Some(m) = vertex_motion(g) else { return false };
        powers.push((m, *e));
    }
    if !claims_carriage {
        return true;
    }
    let Some(target) = vertex_motion(v) else { return false };
    let Some(bits) = motion_key(&target).and_then(|k| u64::try_from(k.len()).ok()).and_then(|n| n.checked_mul(8)) else {
        return false;
    };
    let mut composed = Mv::scalar(num_rational::BigRational::from_integer(1.into()));
    for (m, e) in &powers {
        let n = m.sub(&Mv::scalar(m.scalar_part()));
        let unipotent = n.geo(&n).is_zero();
        if !unipotent && e.unsigned_abs() > bits {
            return false;
        }
        composed = composed.geo(&motion_power(m, *e));
    }
    let Some(residue) = crate::transducer::element::conformal_of(&derivation.residue, &crate::transducer::element::identity_versor()) else {
        return false;
    };
    let composed = composed.geo(&residue);
    // 78.166 1c — a transported derivation's motion is `G · ∏ · G̃`, the
    // conjugator's motion at the identity carriage.
    let composed = match derivation.conjugator() {
        None => composed,
        Some(g) => match crate::transducer::element::conformal_of_rotor(g, &crate::transducer::element::identity_versor()) {
            Some(gm) => gm.geo(&composed).geo(&gm.reverse()),
            None => return false,
        },
    };
    composed.projective_eq(&target)
}

/// 37.7 P2 — EVERY `RotorPrime` DEMAND CARRIES ITS PRIME. The witness is
/// the residue block the shells left (50.5 as amended); `π_p =
/// gcd_{ℤ[i]}(s + t·i, p)` must have norm `p`, or the demand names a
/// prime no witness of it carries and there is nothing to open. Refused
/// at the demand vertex, addressed by the demand's own key — which is
/// its coordinates written out (A21), so the coordinate is content.
pub(crate) fn witnesses_carry_their_primes(f: &Form, meter: &mut Meter<'_>, from: usize) -> Step {
    use crate::transducer::sentence::Demand;
    for (i, (vertex, kind, chart, _)) in f.demand_vertices().enumerate().skip(from) {
        let Demand::RotorPrime { prime, witness, .. } = kind else {
            continue;
        };
        // 78.153 — one read per prime factored.
        if !meter.spend() {
            return Err(Halt::Spent(i));
        }
        let carries = crate::core::gaussian::from_block(witness).is_some_and(|w| {
            crate::core::gaussian::prime_factor(&w, &num_bigint::BigInt::from(*prime)).is_some()
        });
        if !carries {
            return Err(Halt::Refused(Refusal::new(
                REFUSE_LETTER_WITNESS,
                sentence::At::vertex(vertex, chart.clone()),
            )));
        }
    }
    Ok(())
}

/// The same replay, refusing WITH THE COORDINATE (50.9): the vertex that
/// does not carry its body, the face whose edge does not re-derive, the
/// chart with the links no demand declares; every other refusal is of the
/// whole tape.
pub fn replay_form_at(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<Refusal> {
    let owned = |a: &[u8]| resolve(a).map(std::borrow::Cow::Owned);
    to_completion(spoken, budget, &mut |b, cursor| replay_form_resume(spoken, &owned, family, crystal, b, cursor))
}

/// 78.153 — [`replay_form_at`] on the stroke, from the cursor: the decode,
/// then [`replay_form_on_resume`].
pub fn replay_form_resume<'r>(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<std::borrow::Cow<'r, Form>>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    cursor: &mut ReplayCursor,
) -> Replayed {
    // A SPOKEN word under replay: the court re-derives it from the tape it
    // was sent, which is the point of a replay, so the decode is named
    // (L16) — and it is named HERE, once, so the verdict below is a
    // function of the FORM and not of which decoder produced it.
    //
    // That seam is the pre-B6 parity guard's (54.5 Phase A). While both
    // decoders exist the tape is an oracle, and
    // `every_court_reader_agrees_on_both_decoders` feeds each of them into
    // [`replay_form_on`] and asserts one verdict. Without the seam the
    // legacy path could not be reached at all, and the guard would be
    // asserting the decoupled reader against itself.
    let Some(f) = crate::transducer::sentence::from_element_form(spoken) else {
        return Replayed::Refused(Refusal::of_tape(REFUSE_FORM_TAPE, spoken));
    };
    replay_form_on_resume(&f, spoken, resolve, family, crystal, budget, cursor)
}

/// [`replay_form_at`] on a form already decoded — the whole verdict, with
/// the decode lifted out so it can be supplied.
pub fn replay_form_on(
    f: &Form,
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<Refusal> {
    let owned = |a: &[u8]| resolve(a).map(std::borrow::Cow::Owned);
    to_completion(spoken, budget, &mut |b, cursor| replay_form_on_resume(f, spoken, &owned, family, crystal, b, cursor))
}

/// 78.153 — THE WHOLE VERDICT, IN PHASES, ON THE STROKE. Every input-sized
/// loop spends one read per item; where the stroke runs out the cursor
/// keeps the phase and the item it stopped at (and the exact partials the
/// later phases need) and the answer is `Unread` — not ruled this stroke,
/// never a refusal. Resumed from the cursor, a replay rules exactly as one
/// run on a stroke that never ran out: every phase before the cursor
/// passed, and passing is a function of the word's content and of cited
/// forms that are content-addressed, so it does not change between strokes.
pub fn replay_form_on_resume<'r>(
    f: &Form,
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<std::borrow::Cow<'r, Form>>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    cursor: &mut ReplayCursor,
) -> Replayed {
    let tape = |reason: &'static str| Refusal::of_tape(reason, spoken);
    // One phase: skipped where the cursor is past it, entered at the item
    // the cursor names where it stopped in it, and settled — passed (the
    // cursor moves to the next phase), refused, or spent (the cursor keeps
    // the item and the replay is unread).
    macro_rules! phase {
        ($phase:expr, |$from:ident| $step:expr) => {
            if let Some($from) = cursor.enter($phase) {
                let step: Step = $step;
                if let Some(out) = cursor.settle($phase, step) {
                    return out;
                }
            }
        };
    }
    // A FORM'S OWN DEPTH AGREES WITH ITS OWN CITATIONS. This is a reading
    // of the sentence against itself — no cited form is resolved for it —
    // so it holds whatever the citation relation's shape is, and it is
    // what catches a tape whose `depth` was moved under its vertices.
    // A depth that cannot be written (STD1) agrees with no tape's.
    if Some(f.depth) != f.depth_of() {
        return Replayed::Refused(tape(REFUSE_GLUED_DEPTH));
    }
    // EVERY BOUNDARY IS BONDED (54.7 B5a). A `Vertex::Glued` is a PORT —
    // "this chart exposes a boundary interface at this rank, depth,
    // placement and φ" — and an edge of the cover is what says which form
    // it mates with. B5 put those two facts in two carriers, and nothing
    // held them together: a vertex whose site no edge stands at is an
    // un-bonded port, and it reached the readers as silence.
    //
    // The court states it instead. The cover is the SPOKEN word's, since
    // that is where a received sentence carries it, and the site is the
    // vertex's own key contribution — a coordinate, never an index.
    let cover = &spoken.receipts().nerve;
    // 78.153 — THE EDGES BY THEIR HOST, filed once per stroke (linear in the
    // cover, as the decode above is in the tape): each port below is then a
    // keyed read, where it walked the whole cover per Glued vertex.
    let mut by_host: Keyed<Vec<usize>> = Keyed::new();
    for (j, e) in cover.iter().enumerate() {
        let host = e.host();
        match by_host.get_mut(host.as_bytes()) {
            Some(on) => on.push(j),
            None => {
                by_host.insert(host.as_bytes(), vec![j]);
            }
        }
    }
    // The Glued vertices are one fibre of the atlas, read at their tag.
    phase!(Phase::Ports, |from| {
        let mut out: Step = Ok(());
        for (i, (vertex, v)) in f.glued_vertices().enumerate().skip(from) {
            // THE CHART COMES OFF THE VERTEX ITSELF, not through
            // `face_chart().unwrap_or_default()`: `Invariant::default()` is the
            // chart trie's ROOT and naming it would put the refusal at the top
            // of the atlas rather than at the boundary that is open.
            let Vertex::Glued { chart, .. } = v else { continue };
            if !budget.spend() {
                out = Err(Halt::Spent(i));
                break;
            }
            let site = crate::transducer::sentence::vertex_bytes(v);
            let at = |reason| Halt::Refused(Refusal::new(reason, sentence::At::vertex(vertex.clone(), chart.clone())));
            // A MULTI-SHEETED SEAM IS ADMISSIBLE, A REPEATED SHEET IS NOT.
            // Several edges at one site is a junction between several charts —
            // gauge universality means the citing chart cannot tell them apart
            // locally, and each distinct target is an independent interface. The
            // same target twice is one geometric overlap declared as two, which
            // would inflate the yield, so the sheets must be distinct.
            let mut sheets: Keyed<()> = Keyed::new();
            let mut n = 0usize;
            for e in by_host.get(&site).into_iter().flatten().filter_map(|&j| cover.get(j)) {
                n += 1;
                sheets.insert(&e.target, ());
            }
            if n == 0 {
                out = Err(at(REFUSE_ORPHAN_PORT));
                break;
            }
            if sheets.len() != n {
                out = Err(at(REFUSE_SHEET_REPEATED));
                break;
            }
        }
        out
    });
    // AND NO BOND WITHOUT A PORT (54.7 B5a). The converse of the orphan
    // check, and the direction that does not fail safe: an orphan port is an
    // incomplete attachment and every reader of it refuses or reports
    // unspoken, while a phantom edge is a claim of connectivity the complex
    // never constructed — it WOULD resolve, replay and be paid for against a
    // boundary the form does not hold. Never erased, because erasure masks
    // perjury and corrupts the attestation that reads the cover for credit.
    phase!(Phase::Phantoms, |from| {
        let mut out: Step = Ok(());
        for (i, e) in cover.iter().enumerate().skip(from) {
            if !budget.spend() {
                out = Err(Halt::Spent(i));
                break;
            }
            // The host is the Glued vertex's site, which IS its coordinate: a
            // keyed read, never a walk of the vertices.
            let vertex = crate::network::nerve::coord::VertexAt::Glued(crate::network::nerve::coord::Site(e.host().as_bytes().to_vec()));
            if !matches!(f.vertices.at(&vertex), Some(Vertex::Glued { .. })) {
                out = Err(Halt::Refused(Refusal::new(REFUSE_PHANTOM_EDGE, sentence::At::vertex(vertex, e.chart.clone()))));
                break;
            }
        }
        out
    });
    // THE CITATIONS ARE THE NERVE'S 1-SIMPLICES, READ FROM THE ENVELOPE
    // (54.5 Road (d), B4 — and this is the reader B5 turns on).
    //
    // It read `f.vertices` for `Vertex::Glued { address }`. The host form
    // says "I have an open boundary that mates under φ"; WHICH form that
    // boundary was instantiated against is the cover's fact, not the
    // chart's, and B5 evicts the target address from `form_vertex_key`
    // entirely — after which this loop would have found nothing to resolve
    // and every citation would have passed unchecked. Reading the edge list
    // now means the eviction changes this reader not at all.
    phase!(Phase::Citations, |from| {
    let mut out: Step = Ok(());
    for (i, edge) in spoken.receipts().nerve.iter().enumerate().skip(from) {
        if !budget.spend() {
            out = Err(Halt::Spent(i));
            break;
        }
        let address = edge.target.as_bytes();
        // TWO DEPTH REFUSALS STOOD HERE AND BOTH ARE GONE (54.2, as ruled
        // 2026-09-20).
        //
        // `*depth >= f.depth` said nothing: `f.depth == f.depth_of()` was
        // checked one line above and `depth_of` IS one more than the
        // deepest citation, so every citation is already strictly below it
        // and the branch was unreachable. It read as the DAG's statement
        // and was a tautology.
        //
        // `cited.depth_of() != *depth` was the DAG, and it is not a fact
        // about this edge. `depth` is the cited form's depth WHEN IT WAS
        // CITED; a cited form goes on being cited after, so its depth
        // moves and the citation's coordinate does not. In a nerve where
        // `U_i` cites `U_j` and `U_j` cites `U_i`, the second citation
        // raises `U_j`'s depth and this check then refuses the first —
        // which is how a posetal height forbade mutual incidence and with
        // it every 2-simplex, and so `H²`.
        //
        // WHAT THE COURT STILL VERIFIES OF A CITATION is below and is
        // unchanged: the cited form resolves on this court's ruled cover,
        // it is CLOSED, and it opens at an ungauged frame. Those are
        // properties of the cited object. Its depth is a reading the
        // citation carries, and the court does not re-derive a reading
        // from an object that has moved since.
        // 78.153 — the cited form BORROWED where the court holds it: the
        // check reads it and keeps nothing.
        let Some(cited) = resolve(address) else {
            out = Err(Halt::Refused(tape(REFUSE_GLUED_UNRULED)));
            break;
        };
        if !cited.is_closed() {
            out = Err(Halt::Refused(tape(REFUSE_GLUED_UNRULED)));
            break;
        }
        // 18: A FORM OPENS AT ITS FRAME, read at the Frame's own coordinate
        // (its tag alone, AT1).
        let frame_is_identity = match cited.vertices.at(&crate::network::nerve::coord::VertexAt::Frame) {
            Some(fr @ Vertex::Frame) => fr.word_in(&cited.vertices).is_some_and(|w| w.is_identity()),
            _ => false,
        };
        if !frame_is_identity {
            out = Err(Halt::Refused(tape(REFUSE_GLUED_FRAME_GAUGED)));
            break;
        }
    }
    out
    });
    // 50.2 step 2 — G COMPOSES THE DERIVATION. Every word vertex names
    // the generators it factored through; the court resolves each on its
    // own ruled cover, composes them with the residue, and compares the
    // product projectively (1.5) against the word the vertex carries. A
    // citation the cover does not hold is NOT a refusal: it is the
    // deferral a `Glued` citation already takes above — the sentence is
    // docketed and the syllabus serves the generator (21 step 3), one
    // mechanism at two levels.
    phase!(Phase::Derivations, |from| derivations_compose(
        f,
        spoken.receipts(),
        resolve,
        &mut Meter::Stroke(&mut *budget),
        from
    ));
    // 37.7 P2: a demand that names a prime must carry a witness that
    // carries it. Read before anything is docketed, so `list_demands`
    // never opens a letter on a witness the arithmetic refuses.
    phase!(Phase::Witnesses, |from| witnesses_carry_their_primes(f, &mut Meter::Stroke(&mut *budget), from));
    // 23.1 step 3, 50.6 (L7): a tape whose only vertices are the Frame and
    // demands is a chain wearing a form's tape — nothing in it acts.
    phase!(Phase::Moves, |from| match (from, f.acts()) {
        (0, false) => Err(Halt::Refused(tape(REFUSE_NOT_A_SENTENCE))),
        _ => vertex_tape_and_moves_from(f, family, spoken, &mut Meter::Stroke(&mut *budget), from),
    });
    // 78.153 — THE FAMILY PERCEIVED ONCE PER REPLAY, one read a pair, the
    // perceptions held on the cursor: every later phase of every stroke
    // reads them and the sensor reads each grid once (law 12).
    if let Some(fam) = family {
        phase!(Phase::Perceive, |from| {
            let mut out: Step = Ok(());
            for (i, (x, y)) in fam.iter().enumerate().skip(from) {
                if !budget.spend() {
                    out = Err(Halt::Spent(i));
                    break;
                }
                cursor.frames.push(super::body::Perceived::of(x, y));
            }
            out
        });
    }
    let frames = std::mem::take(&mut cursor.frames);
    let out = replay_after_perceiving(f, spoken, family, crystal, budget, cursor, &frames);
    cursor.frames = frames;
    out
}

/// The phases after the family is perceived, over the cursor's perceptions.
fn replay_after_perceiving(
    f: &Form,
    spoken: &Element,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    cursor: &mut ReplayCursor,
    frames: &[super::body::Perceived],
) -> Replayed {
    let tape = |reason: &'static str| Refusal::of_tape(reason, spoken);
    macro_rules! phase {
        ($phase:expr, |$from:ident| $step:expr) => {
            if let Some($from) = cursor.enter($phase) {
                let step: Step = $step;
                if let Some(out) = cursor.settle($phase, step) {
                    return out;
                }
            }
        };
    }
    // 78.143 — EVERY CLAUSE IS STATED ON EVERY PAIR, or the form is refused
    // by the clause's own name before its speech is read: a tie its order
    // does not name, a coordinate that is not exact, no single body where
    // one is asked, a cell landing between cells.
    if let Some(fam) = family {
        phase!(Phase::Clauses, |from| clauses_stated_from(f, fam, frames, spoken, &mut Meter::Stroke(&mut *budget), from));
    }
    // G steps 2–3 (50.2) on the court's copy of the family: every word
    // vertex carries its body onto its image, exactly (L7).
    if let Some(fam) = family {
        phase!(Phase::Bodies, |from| super::body::vertices_carry_frames_from(
            f,
            fam,
            frames,
            &mut Meter::Stroke(&mut *budget),
            from
        ));
    }
    // ORBITS ARE DERIVED, AND SINCE 54.5 THE TAPE DOES NOT CARRY THEM, so
    // there is no taped forest to compare a traversed one against. The decoder
    // runs `orbits_of` on the faces it just parsed, and this re-read is the
    // same call on the same faces: `f.orbits` and `traversed.orbits` are one
    // set. What remains is what was always the real reading — whether the
    // rank is MEASURABLE at all, which an orbit whose holonomy is not a
    // versor makes it not (15.3).
    //
    // 78.153 — ONE READ for the traversal, and the form is BORROWED where the
    // traversal is the forest it already carries (every decoded form): the
    // whole-form clone is taken only where a supplied form's orbits differ.
    if !budget.spend() {
        return cursor.unread_here();
    }
    let orbits = Form::orbits_of(&f.faces, &f.vertices);
    let traversed: std::borrow::Cow<'_, Form> = match orbits == f.orbits {
        true => std::borrow::Cow::Borrowed(f),
        false => {
            let mut t = f.clone();
            t.orbits = orbits;
            std::borrow::Cow::Owned(t)
        }
    };
    let traversed: &Form = &traversed;
    let by_chord = chord_orbits(traversed);
    // Chapter 23.1 step 4: a face equals face_of(i, j), or it is the
    // Kruskal chord of an orbit whose holonomy is what the chord carries
    // (tree faces of that orbit are face_of). Filing of that class is
    // the orbit check; a missing torsion entry is REFUSE_ORBIT, a chord
    // that does not carry the holonomy is REFUSE_FACE.
    // THE OVERLAPS A CERTIFICATE NAMES, by their coordinates: the only
    // producers of a face's content the vertices do not derive (A20, AT1).
    let mut named: Keyed<(), crate::network::nerve::coord::Overlap> = Keyed::new();
    for cert in &spoken.receipts().certificates {
        named.put(cert.by.face(), ());
    }
    phase!(Phase::Faces, |from| {
        let mut out: Step = Ok(());
        for (i, (o, face)) in f.faces.entries().enumerate().skip(from) {
            if !budget.spend() {
                out = Err(Halt::Spent(i));
                break;
            }
            // 50.3, 52.3: an edge transport of any order is a face. What it
            // carries is re-derived on the family below (`edges_rederive_frames`),
            // and a period is never a gate.
            // EVERY OVERLAP RE-DERIVES AT ITS MEMBER PAIR (AT1): the placement
            // and hue map re-derive from the two members its charts name, and an
            // overlap whose ends the form no longer holds does not re-derive.
            let Err(branch) = face_rederives(f, &o, face) else {
                continue;
            };
            // A PLACE THAT DOES NOT RE-DERIVE IS ADMITTED ONLY BY A CERTIFICATE
            // (AT1 A3): the pivot is the one producer of content the vertices do
            // not derive, and its certificate names the overlap it wrote. A
            // chord whose orbit carries is not a producer, so it never admits a
            // place alone.
            if !named.holds(&o) || !torsion_chord_carries(traversed, &by_chord, &o) {
                out = Err(Halt::Refused(face_at(&o, branch)));
                break;
            }
        }
        out
    });
    // 23.1 step 4, 50.3 (L7): every carried edge re-measured on the union
    // of its two bodies, on every pair of the family. BEFORE the orbits,
    // because an edge that does not re-derive is a defect at a FACE and
    // the reading names it there; the orbit reading over that same edge
    // would answer second-hand, as an unfiled class.
    if let Some(fam) = family {
        phase!(Phase::Edges, |from| super::body::edges_rederive_frames_from(traversed, fam, frames, budget, from));
    }
    // THE SPOKEN WORD'S OWN ATTESTATION, checked against the forest the
    // court just traversed (54.5) — one read per orbit (78.153), in the
    // order `incoherent_orbit` reads them, so the first that does not
    // cohere is the one the refusal carries.
    phase!(Phase::Coherence, |from| {
        let mut out: Step = Ok(());
        for (i, lp) in traversed.orbits.iter().enumerate().skip(from) {
            if !budget.spend() {
                out = Err(Halt::Spent(i));
                break;
            }
            if traversed.orbit_coheres(lp) {
                continue;
            }
            // CV1 — the orbit's whole transport, read by the one reader (78.9):
            // no versor is P, a versor the dialect does not spell is D, one it
            // spells whose closure still refused is C.
            let composite = crate::transducer::sentence::circuit::paths_carried(&traversed.faces, &traversed.vertices, lp)
                .and_then(|carried| carried.scoped().find_map(|(_, c)| c.composite()));
            out = Err(Halt::Refused(Refusal::closure(
                REFUSE_ORBIT,
                sentence::At::Tape(spoken.address()),
                sentence::Content::Holonomy { versor: composite.as_ref(), chart: None, proof: "the orbit's transport lifts to no versor" },
            )));
            break;
        }
        out
    });
    // AND THE SPEAKER'S CLAIM, CHECKED AGAINST THE GEOMETRY THE COURT JUST
    // TRAVERSED (54.5). The terrain is priced above; this is the labour, and
    // the two are never one reading.
    phase!(Phase::Terrain, |from| receipts_price_the_terrain_from(
        traversed,
        spoken,
        &mut Meter::Stroke(&mut *budget),
        from,
        &mut cursor.derived
    ));
    // The rank, one read.
    phase!(Phase::Rank, |from| match (budget.spend(), traversed.rank()) {
        (false, _) => Err(Halt::Spent(from)),
        (true, None) => Err(Halt::Refused(tape(REFUSE_RANK_LOST))),
        (true, Some(_)) => Ok(()),
    });
    // S9.2'S COMPARISON IS GONE WITH THE CARRIER IT COMPARED (54.5). It
    // read `Rank::moved(taped_rank, derived) != Equal` and refused a tape
    // whose stored orbits carried a different rank from the traversed forest —
    // in either direction, which was the right shape for the question.
    //
    // The question no longer exists. The tape carries no orbits, so the
    // decoder's `f.orbits` IS `orbits_of(&f.faces, &f.vertices)` and the two
    // sides of that comparison were the same call on the same faces. A
    // forged tape can forge FACES, and `face_rederives` above is what reads
    // those; it cannot forge a forest, because it does not carry one.
    if let Some(fam) = family {
        // ONE CRYSTAL, ALWAYS: the court speaks the form through the words it
        // holds, whatever the form cites. A reader chosen by whether the
        // form cites was a switch standing where the object decides.
        //
        // 78.153 — ONE PAIR PER READ, and a read per cited form spoken
        // afresh inside it. The cursor keeps the undeclared links of the
        // pairs already read — the exact partial the refusal is built from —
        // so a resumed deflation reads each pair once.
        phase!(Phase::Deflation, |from| {
            let mut out: Step = Ok(());
            for pi in from..fam.len() {
                let here = crate::transducer::alphabet::Word::identity();
                // The scene the pair is spoken onto is the perception the
                // cursor holds: the sensor is not read again for it.
                let bodies = frames.get(pi).map(|p| p.bodies.clone());
                match crate::transducer::family::deflation_pair_metered(traversed, fam, pi, crystal, &here, budget, bodies) {
                    crate::transducer::family::PairRead::Spent => {
                        out = Err(Halt::Spent(pi));
                        break;
                    }
                    // A CYCLE OF CITATIONS is no speech at all (defect 4):
                    // refused by name before anything it would have spoken.
                    crate::transducer::family::PairRead::Read { cycled, .. } if !cycled.is_empty() => {
                        out = Err(Halt::Refused(tape(REFUSE_CITATION_CYCLE)));
                        break;
                    }
                    // A CITATION THE COURT DOES NOT HOLD, reached inside a cited form,
                    // is the dialect the court lacks (D, 78.9), never the form's silence:
                    // the sites it would have spoken are not a remainder. It
                    // refused whatever any pair left, so it refuses at the
                    // first pair that shows it.
                    crate::transducer::family::PairRead::Read { unresolved, .. } if !unresolved.is_empty() => {
                        out = Err(Halt::Refused(tape(REFUSE_GLUED_UNRULED)));
                        break;
                    }
                    crate::transducer::family::PairRead::Read { remainder, .. } => {
                        // The filter is per link, so the union of the pairs'
                        // undeclared links is the undeclared links of the union.
                        cursor.undeclared.extend(undeclared_remainder(&[remainder], traversed));
                        cursor.undeclared.sort_unstable();
                        cursor.undeclared.dedup();
                    }
                }
            }
            out
        });
        // S9.4 — CLOSURE IS THE CERTIFICATE. A remainder on any pair means
        // the form does not close the family it claims, and that is the
        // refusal. The μ⃗ it measured is NOT: a form that closes every pair
        // while measuring nothing has still closed them, and pricing is the
        // economy's reading, taken after the court's. Under 50.5 (L7) the
        // clause reads the CLOSED SUB-COMPLEX: a remainder link a demand
        // vertex names is what the sentence said it could not close — valid,
        // not closed — and only an undeclared remainder refuses, at the
        // chart of the links it read (50.9).
        let undeclared = std::mem::take(&mut cursor.undeclared);
        if !undeclared.is_empty() {
            // ρ_rem — HOW MUCH OF THE FAMILY THE FORM MISSED (§8.1, the
            // reading before the fix).
            //
            // This refusal is 223 of 230 on the wire and it is the engine's
            // wall: nothing stands, so `court basis` is 0 on every hypha,
            // so the syllabus teaches only genesis vocabulary. What it does
            // NOT say is whether the extractor is near or nowhere — a form
            // missing four cells of two hundred and a form missing a
            // hundred and eighty are one refusal here, and they are two
            // different defects with two different repairs.
            //
            //   ρ ≪ 1 — the rule is right and the form does not DECLARE
            //   what it leaves: untouched background, a frame boundary, a
            //   hue plane nothing acts on. The repair is a declaration.
            //
            //   ρ ≈ 1 — the form is not the family's rule at all, and the
            //   links are not a generator to synthesise.
            //
            // The denominator is the links the family's own remainders
            // carry, which is what the deflation measured against; the
            // numerator is what no demand vertex declared. Both are in hand
            // here and neither was written down.
            // THE TWO REMAINDERS ARE TWO REFUSALS (54.6).
            //
            //   `spoken == 0` — the form was SILENT at that cell. The
            //   language has no word for it, which is what a demand is:
            //   the speaker mounts `Demand::Source` over exactly these
            //   links, the form becomes valid-not-closed (50.1), and the
            //   mesh is asked. `contract.rs` builds that demand from this
            //   coordinate.
            //
            //   `spoken != 0` — the form ASSERTED something the family
            //   contradicts. No demand declares a false projection, and
            //   `undeclared_remainder` keeps such a link whatever a demand
            //   says of it, so a contraction over it cannot succeed. And a
            //   contraction MOVES THE ADDRESS, so the old seal stops
            //   matching and the same refusal returns at a fresh
            //   coordinate — one mint per cycle.
            //
            // They arrived merged under one kind, so a speaker could not
            // tell which it had and contracted over the union. Partitioned,
            // each carries its own verdict and its own links.
            let (silent, wrong): (Vec<_>, Vec<_>) =
                undeclared.iter().copied().partition(|&(_, _, spoken, _)| spoken == 0);
            // THE WRONG ONES FIRST. A form carrying both said something
            // false, and that is the refusal it owes; contracting its
            // silence while leaving the falsehood would mint an address
            // that refuses again.
            // CV1 — A REMAINDER IS A RESIDUE ON THE COVER: its links are the
            // cocycle, read by the one reader (78.9).
            if !wrong.is_empty() {
                let content = sentence::Content::Residue(&wrong);
                let obstruction = sentence::read_obstruction(content);
                return Replayed::Refused(Refusal {
                    reason: REFUSE_NOT_CLOSED,
                    at: sentence::At::Chart(stalk::invariant(&wrong), wrong),
                    obstruction,
                });
            }
            let obstruction = sentence::read_obstruction(sentence::Content::Residue(&silent));
            return Replayed::Refused(Refusal {
                reason: REFUSE_NOT_CLOSED,
                at: sentence::At::Chart(stalk::invariant(&silent), silent),
                obstruction,
            });
        }
    }
    // A FACTOR NO CERTIFICATE NAMES IS REFUSED (AT1, ruled): an overlap's
    // `left`/`right` are produced content, written only by the pivot's Word
    // move, and the certificate whose chord names the overlap is their
    // producer. A factor with none was written by nothing the court can
    // replay.
    phase!(Phase::Pivots, |from| {
        let mut out: Step = Ok(());
        for (i, (o, face)) in traversed.faces.entries().enumerate().skip(from) {
            if !budget.spend() {
                out = Err(Halt::Spent(i));
                break;
            }
            if face.is_pivoted() && !named.holds(&o) {
                out = Err(Halt::Refused(tape(REFUSE_PIVOT_INEXACT)));
                break;
            }
        }
        out
    });
    phase!(Phase::Certificates, |from| replay_certificates_from(
        traversed,
        &spoken.receipts().certificates,
        &by_chord,
        &mut Meter::Stroke(&mut *budget),
        from
    )
    .map_err(|cut| match cut {
        Cut::Refused(reason) => Halt::Refused(tape(reason)),
        Cut::Spent(at) => Halt::Spent(at),
    }));
    Replayed::Clean
}

/// `REFUSE_FACE` at the overlap's two vertices, by their coordinates (50.9,
/// AT1): every end of an overlap is a vertex coordinate — a citation's is its
/// site (54.7 B5a) — so the refusal always has a coordinate to name. Its
/// obstruction is the branch that refused (A.17), counted with the refusal.
fn face_at(o: &crate::network::nerve::coord::Overlap, branch: sentence::Refused) -> Refusal {
    Refusal {
        reason: REFUSE_FACE,
        at: sentence::At::face(o.from_vertex(), o.to_vertex()),
        obstruction: branch.obstruction(),
    }
}

/// Chapter 23.1 steps 3 and 6: each Word vertex re-derives its rotor (M1)
/// and its affine; a Rebased whose word is the identity equals its base;
/// an Unfolded Remainder with nothing left does not move; an orbit whose
/// word is identity does not move. With a family, unfold and orbit execute
/// on the trains. The orbit's refusal is at the contact vertex with the
/// chart it acts on (50.9); every other refusal here is of the tape.
///
/// 78.153 — on a meter, one read per vertex from the `from`-th of the form's
/// own order.
pub(crate) fn vertex_tape_and_moves_from(
    f: &Form,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    spoken: &Element,
    meter: &mut Meter<'_>,
    from: usize,
) -> Step {
    match vertex_moves(f, family, spoken, meter, from) {
        Ok(None) => Ok(()),
        Ok(Some(r)) => Err(Halt::Refused(r)),
        Err(at) => Err(Halt::Spent(at)),
    }
}

fn vertex_moves(
    f: &Form,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    spoken: &Element,
    meter: &mut Meter<'_>,
    from: usize,
) -> Result<Option<Refusal>, usize> {
    for (i, (vertex, v)) in f.vertices.entries().enumerate().skip(from) {
        if !meter.spend() {
            return Err(i);
        }
        if let Some(r) = vertex_move(f, family, spoken, &vertex, v) {
            return Ok(Some(r));
        }
    }
    Ok(None)
}

/// One vertex of [`vertex_tape_and_moves_from`].
fn vertex_move(
    f: &Form,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    spoken: &Element,
    vertex: &crate::network::nerve::coord::VertexAt,
    v: &Vertex,
) -> Option<Refusal> {
    use crate::transducer::family;
    use crate::transducer::sentence::Measure;
    let tape = |reason: &'static str| Refusal::of_tape(reason, spoken);
    {
        match v {
            Vertex::Word { derivation, contact, .. } => {
                // EVERY MEMBER'S CARRIAGE must give a conformal versor, or
                // the tape carries a placement nothing can be spoken at.
                // One member today; the traversal is the vertex's own fibre.
                let Some(acts) = v.acts() else {
                    return Some(tape(REFUSE_VERTEX_TAPE));
                };
                for (_, a) in acts.scoped() {
                    if element::conformal_of_rotor(&derivation.rotor(), &a.carriage).is_none() {
                        return Some(tape(REFUSE_VERTEX_TAPE));
                    }
                }
                if contact.is_some() {
                    let still = || match v.face_chart() {
                        Some(chart) => {
                            Refusal::new(REFUSE_ORBIT_STILL, sentence::At::vertex(vertex.clone(), chart))
                        }
                        None => tape(REFUSE_ORBIT_STILL),
                    };
                    if derivation.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
                        return Some(still());
                    }
                    if let Some(fam) = family {
                        for (x, _) in fam {
                            // THE MOTION MOVES THE BODY, read by one exact
                            // application (78.62) — never a walk.
                            if !family::moves(v, &crate::arc::painted_of_grid(x)) {
                                return Some(still());
                            }
                        }
                    }
                }
            }
            Vertex::Rebased { .. }
                if v.word_in(&f.vertices).is_some_and(|w| w.is_identity()) => {
                    return Some(tape(REFUSE_REBASE_TRIVIAL));
                }
            Vertex::Unfolded { measure: Measure::Remainder } if f.remainder.is_empty() => {
                return Some(tape(REFUSE_UNFOLD_STILL));
            }
            Vertex::Unfolded { measure: Measure::Occupied } => {
                if let Some(fam) = family {
                    for (x, y) in fam {
                        if x == y {
                            return Some(tape(REFUSE_UNFOLD_STILL));
                        }
                    }
                }
            }
            // 50.5: a demand vertex names a kind that carries its
            // coordinates, or it is not a demand.
            Vertex::Demand { kind, .. } => {
                if let Err(why) = kind.typed() {
                    return Some(tape(why));
                }
            }
            _ => {}
        }
    }
    None
}

/// 78.143 — THE CLAUSES SPOKEN ON EVERY INPUT OF THE FAMILY the court holds,
/// each scene perceived as the speech perceives it; the first that is
/// unstated names the refusal. `None` where every clause is stated on every
/// pair (or the form states none).
///
/// 78.153 — one read per pair perceived, from the `from`-th pair.
pub(crate) fn clauses_stated_from(
    f: &Form,
    family: &[(crate::arc::Grid, crate::arc::Grid)],
    frames: &[super::body::Perceived],
    spoken: &Element,
    meter: &mut Meter<'_>,
    from: usize,
) -> Step {
    use crate::transducer::family::statement::{speak_clauses, Unstated};
    if f.clauses.is_empty() {
        return Ok(());
    }
    for (i, ((x, _), seen)) in family.iter().zip(frames).enumerate().skip(from) {
        if !meter.spend() {
            return Err(Halt::Spent(i));
        }
        if let Err(why) = speak_clauses(&f.clauses, x, &seen.bodies) {
            let reason = match why {
                Unstated::Tie => REFUSE_RULE_TIE,
                Unstated::Inexact => REFUSE_RULE_INEXACT,
                Unstated::NotOne => REFUSE_RULE_NOT_ONE,
                Unstated::OffLattice => REFUSE_RULE_OFF_LATTICE,
                Unstated::Still => REFUSE_RULE_STILL,
            };
            return Err(Halt::Refused(Refusal::of_tape(reason, spoken)));
        }
    }
    Ok(())
}

/// Each orbit by the chord it closes on (AT1: the chord an overlap, its key
/// the overlap's coordinates), to the orbit the replay reads.
pub(crate) fn chord_orbits(f: &Form) -> Keyed<crate::transducer::sentence::Orbit, crate::network::nerve::coord::Overlap> {
    let mut by_chord = Keyed::new();
    for lp in &f.orbits {
        by_chord.put(&lp.chord, lp.clone());
    }
    by_chord
}

/// 23.1 step 4 — A FACE THAT IS NOT `face_of` IS THE KRUSKAL CHORD of a
/// orbit whose every other face is. The pivot writes the chord's transport
/// and nothing else, so the chord is exactly where a form's own word
/// differs from the coboundary of its two vertices.
///
/// THE CHORD IS A FACE LIKE ANY OTHER and carries its edge like any other.
/// What the orbit then reads — flat, filed at its period, or not evaluated
/// at all — is [`form_orbits_replay`]'s, on the orbit's EDGE holonomy (50.3).
/// Asking for the TRANSPORT composite here instead, matched against the
/// chord's own word, would be a reading that telescopes over words that
/// commute: it would certify an orbit the voice never evaluated and demand a
/// filing under a class nothing files.
pub(crate) fn torsion_chord_carries(
    f: &Form,
    by_chord: &Keyed<crate::transducer::sentence::Orbit, crate::network::nerve::coord::Overlap>,
    chord: &crate::network::nerve::coord::Overlap,
) -> bool {
    let Some(lp) = by_chord.at(chord) else {
        return false;
    };
    for s in &lp.transitions {
        if &s.overlap == chord {
            continue;
        }
        let Some(tf) = f.faces.at(&s.overlap) else {
            return false;
        };
        if face_rederives(f, &s.overlap, tf).is_err() {
            return false;
        }
    }
    true
}

/// THE ORBITS THE FORM EVALUATED, each flat or filed at its period.
///
/// The reading is the orbit's EDGE HOLONOMY (50.3) — the composite of the
/// transitions between the bodies' own frames — and it is the same reader
/// the voice classifies with. The TRANSPORT composite is a different
/// object: it telescopes around a cycle over words that commute, and every
/// word a pair form composes is a sensor-plane word, so read on it a
/// cover's orbit is flat however curved the scene is, while the class the
/// voice actually files — under the edge holonomy's key — is never looked
/// for.
///
/// AN ORBIT THE FORM DID NOT EVALUATE IS NOT REPLAYED. `edge_holonomy`
/// answers `None` where a remainder edge lies anywhere on the cycle (13.9
/// step 7), and an orbit with no transition to read carries no obligation
/// (rule 11: that is a refusal to read, not a flat reading).
///
/// 78.153 — the court reads it one orbit per read (`Phase::Coherence`,
/// `Form::orbit_coheres`); this whole reading stays for the tests.
#[cfg(test)]
pub(crate) fn form_orbits_replay(f: &Form) -> bool {
    // ONE READER FOR ONE QUESTION (law 12, 54.5 as ruled 2026-09-20).
    //
    // This read the orbits itself and asked `orbit_reading_replays` of each
    // holonomy, which is the question `Form::coheres` already answers — two
    // readers of one fact, and they disagreed. `coheres` was corrected at
    // 54.2 to admit monodromy through `orbit_is_congruent`; this was not,
    // and it is what the court actually ran.
    //
    // Both halves stay, because an orbit HAS two obstructions and neither
    // replaces the other (`Form::edge_class_vector`): the transport
    // composite the pivot discharges, and the edge holonomy the cover
    // reads. `coheres` is exactly that conjunction.
    //
    // AND IT TAKES NO ATTESTATION. It read `filed` and demanded an entry
    // per class, so a form was refused for a permit rather than for its
    // geometry — and with the period taken from `word.order()`, the bare
    // rotor order, a screw was filed at 0 by `file_orbit` and demanded at 4
    // here. Fifth site of one law-12 ruling and the second of the permit
    // conflation. What the speaker CLAIMED is a separate reading and is
    // [`receipts_price_the_terrain`].
    f.coheres()
}

/// Every torsion cycle the word speaks is run round from its base assignment.
/// A period is believed only as a witnessed closed orbit; an open circuit names
/// the anholonomity the label was filing over, and a closure at another
/// period is a winding the word misspoke. Replay was Δ = 0 on the chain;
/// this is ∮ dX = 0 on the fibre.
pub(crate) fn witness_torsion(spoken: &Element) -> Option<&'static str> {
    for (stored, witness) in torsion_witness::witness_stored(spoken) {
        match witness {
            HolonomicOrbitWitness::Closed { period, .. } if period == stored => {}
            HolonomicOrbitWitness::Closed { .. } => return Some("torsion period does not re-derive — the winding was misspoken"),
            HolonomicOrbitWitness::Open { .. } => return Some("torsion cycle does not close — the period was a filing"),
        }
    }
    None
}
