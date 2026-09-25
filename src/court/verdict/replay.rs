// 23, 42 — REPLAY: a spoken form is re-derived from its own tape, move by
// move, against the cover that resolves what it cites.
//
//! Every vertex re-derives from its tape, every face from its vertices,
//! every orbit's holonomy to the identity, and every pivot certificate from
//! the axis it names. A form that does not is REFUSED, never scored.

use super::body::{edges_rederive_frames, undeclared_remainder, vertices_carry_frames};
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
pub fn replay_form(spoken: &Element, resolve: &dyn Fn(&[u8]) -> Option<Form>) -> Option<&'static str> {
    replay_form_with(spoken, resolve, None, &crate::transducer::family::NoCrystal)
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
) -> Option<&'static str> {
    replay_form_at(spoken, resolve, family, crystal).map(|r| r.reason)
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
fn face_rederives(f: &Form, o: &crate::network::nerve::coord::Overlap, face: &crate::transducer::sentence::FormFace) -> bool {
    // §8.1 — WHICH BRANCH REFUSED. `REFUSE_FACE` names the face and not the
    // reading inside it, and the ways a face can fail to re-derive are
    // different defects.
    let label = crate::transducer::sentence::At::face(o.from_vertex(), o.to_vertex()).label();
    let (Some(_), Some(_)) = (f.vertices.at(&o.from_vertex()), f.vertices.at(&o.to_vertex())) else {
        eprintln!("  {label} refused: an end is not a vertex of the form");
        return false;
    };
    if face.producer == crate::transducer::sentence::Producer::Motor {
        return true;
    }
    // The edge is evidence the court re-measures against the FAMILY
    // (`edges_rederive_frames`), not something the vertices re-derive, so it
    // is carried in and compared by its presence alone here.
    match f.face_place(&o.from, &o.to, face.place.edge.clone()) {
        Some(p) if same_place(&p, &face.place) => true,
        Some(_) => {
            eprintln!("  {label} refused: the place does not re-derive");
            false
        }
        None => {
            eprintln!("  {label} refused: the place does not derive at all");
            false
        }
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
    derivations_compose(f, env, resolve)
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

fn derivations_compose(
    f: &Form,
    env: &crate::transducer::element::ReceiptsEnvelope,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
) -> Option<Refusal> {
    use crate::transducer::alphabet::Verified;
    for cite in &env.derivations {
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
            Verified::Composes => {}
            Verified::Differs => {
                return Some(Refusal::new(REFUSE_DERIVATION, sentence::At::vertex(vertex.clone(), class.clone())))
            }
            // The door (`hear_sentence`) decides the WAIT before any
            // replay runs, so an unruled cited FORM never reaches here.
            // What reaches here is a citation naming a vertex the ruled
            // form does not carry — the form resolved and the vertex was
            // not in it — which is a claim about the tape and a refusal
            // at the vertex that made it, not a deferral.
            Verified::Unresolved { .. } => {
                return Some(Refusal::new(REFUSE_DERIVATION, sentence::At::vertex(vertex.clone(), class.clone())))
            }
        }
    }
    None
}

/// 37.7 P2 — EVERY `RotorPrime` DEMAND CARRIES ITS PRIME. The witness is
/// the residue block the shells left (50.5 as amended); `π_p =
/// gcd_{ℤ[i]}(s + t·i, p)` must have norm `p`, or the demand names a
/// prime no witness of it carries and there is nothing to open. Refused
/// at the demand vertex, addressed by the demand's own key — which is
/// its coordinates written out (A21), so the coordinate is content.
fn witnesses_carry_their_primes(f: &Form) -> Option<Refusal> {
    use crate::transducer::sentence::Demand;
    for (vertex, kind, chart, _) in f.demand_vertices() {
        let Demand::RotorPrime { prime, witness, .. } = kind else {
            continue;
        };
        let carries = crate::core::gaussian::from_block(witness).is_some_and(|w| {
            crate::core::gaussian::prime_factor(&w, &num_bigint::BigInt::from(*prime)).is_some()
        });
        if !carries {
            return Some(Refusal::new(
                REFUSE_LETTER_WITNESS,
                sentence::At::vertex(vertex, chart.clone()),
            ));
        }
    }
    None
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
) -> Option<Refusal> {
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
        return Some(Refusal::of_tape(REFUSE_FORM_TAPE, spoken));
    };
    replay_form_on(&f, spoken, resolve, family, crystal)
}

/// [`replay_form_at`] on a form already decoded — the whole verdict, with
/// the decode lifted out so it can be supplied.
pub fn replay_form_on(
    f: &Form,
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
) -> Option<Refusal> {
    let tape = |reason: &'static str| Refusal::of_tape(reason, spoken);
    // A FORM'S OWN DEPTH AGREES WITH ITS OWN CITATIONS. This is a reading
    // of the sentence against itself — no cited form is resolved for it —
    // so it holds whatever the citation relation's shape is, and it is
    // what catches a tape whose `depth` was moved under its vertices.
    // A depth that cannot be written (STD1) agrees with no tape's.
    if Some(f.depth) != f.depth_of() {
        return Some(tape(REFUSE_GLUED_DEPTH));
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
    // The Glued vertices are one fibre of the atlas, read at their tag.
    for (vertex, v) in f.glued_vertices() {
        // THE CHART COMES OFF THE VERTEX ITSELF, not through
        // `face_chart().unwrap_or_default()`: `Invariant::default()` is the
        // chart trie's ROOT and naming it would put the refusal at the top
        // of the atlas rather than at the boundary that is open.
        let Vertex::Glued { chart, .. } = v else { continue };
        let site = crate::transducer::sentence::vertex_bytes(v);
        let at = |reason| Refusal::new(reason, sentence::At::vertex(vertex.clone(), chart.clone()));
        // A MULTI-SHEETED SEAM IS ADMISSIBLE, A REPEATED SHEET IS NOT.
        // Several edges at one site is a junction between several charts —
        // gauge universality means the citing chart cannot tell them apart
        // locally, and each distinct target is an independent interface. The
        // same target twice is one geometric overlap declared as two, which
        // would inflate the yield, so the sheets must be distinct.
        let mut sheets: Keyed<()> = Keyed::new();
        let mut n = 0usize;
        for e in cover.iter().filter(|e| e.host().as_bytes() == site) {
            n += 1;
            sheets.insert(&e.target, ());
        }
        if n == 0 {
            return Some(at(REFUSE_ORPHAN_PORT));
        }
        if sheets.len() != n {
            return Some(at(REFUSE_SHEET_REPEATED));
        }
    }
    // AND NO BOND WITHOUT A PORT (54.7 B5a). The converse of the orphan
    // check, and the direction that does not fail safe: an orphan port is an
    // incomplete attachment and every reader of it refuses or reports
    // unspoken, while a phantom edge is a claim of connectivity the complex
    // never constructed — it WOULD resolve, replay and be paid for against a
    // boundary the form does not hold. Never erased, because erasure masks
    // perjury and corrupts the attestation that reads the cover for credit.
    for e in cover {
        // The host is the Glued vertex's site, which IS its coordinate: a
        // keyed read, never a walk of the vertices.
        let vertex = crate::network::nerve::coord::VertexAt::Glued(crate::network::nerve::coord::Site(e.host().as_bytes().to_vec()));
        if !matches!(f.vertices.at(&vertex), Some(Vertex::Glued { .. })) {
            return Some(Refusal::new(REFUSE_PHANTOM_EDGE, sentence::At::vertex(vertex, e.chart.clone())));
        }
    }
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
    for edge in &spoken.receipts().nerve {
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
        let Some(cited) = resolve(address) else {
            return Some(tape(REFUSE_GLUED_UNRULED));
        };
        if !cited.is_closed() {
            return Some(tape(REFUSE_GLUED_UNRULED));
        }
        // 18: A FORM OPENS AT ITS FRAME, read at the Frame's own coordinate
        // (its tag alone, AT1).
        let frame_is_identity = match cited.vertices.at(&crate::network::nerve::coord::VertexAt::Frame) {
            Some(fr @ Vertex::Frame) => fr.word_in(&cited.vertices).is_some_and(|w| w.is_identity()),
            _ => false,
        };
        if !frame_is_identity {
            return Some(tape(REFUSE_GLUED_FRAME_GAUGED));
        }
    }
    // 50.2 step 2 — G COMPOSES THE DERIVATION. Every word vertex names
    // the generators it factored through; the court resolves each on its
    // own ruled cover, composes them with the residue, and compares the
    // product projectively (1.5) against the word the vertex carries. A
    // citation the cover does not hold is NOT a refusal: it is the
    // deferral a `Glued` citation already takes above — the sentence is
    // docketed and the syllabus serves the generator (21 step 3), one
    // mechanism at two levels.
    if let Some(refusal) = derivations_compose(f, spoken.receipts(), resolve) {
        return Some(refusal);
    }
    // 37.7 P2: a demand that names a prime must carry a witness that
    // carries it. Read before anything is docketed, so `list_demands`
    // never opens a letter on a witness the arithmetic refuses.
    if let Some(refusal) = witnesses_carry_their_primes(f) {
        return Some(refusal);
    }
    // 23.1 step 3, 50.6 (L7): a tape whose only vertices are the Frame and
    // demands is a chain wearing a form's tape — nothing in it acts.
    if !f.acts() {
        return Some(tape(REFUSE_NOT_A_SENTENCE));
    }
    if let Some(refusal) = vertex_tape_and_moves(f, family, spoken) {
        return Some(refusal);
    }
    // G steps 2–3 (50.2) on the court's copy of the family: every word
    // vertex carries its body onto its image, exactly (L7).
    if let Some(fam) = family {
        if let Some(refusal) = vertices_carry_frames(f, fam) {
            return Some(refusal);
        }
    }
    // ORBITS ARE DERIVED, AND SINCE 54.5 THE TAPE DOES NOT CARRY THEM, so
    // there is no taped forest to compare a traversed one against. The decoder
    // runs `orbits_of` on the faces it just parsed, and this re-read is the
    // same call on the same faces: `f.orbits` and `traversed.orbits` are one
    // set. What remains is what was always the real reading — whether the
    // rank is MEASURABLE at all, which an orbit whose holonomy is not a
    // versor makes it not (15.3).
    let mut traversed = f.clone();
    traversed.orbits = Form::orbits_of(&f.faces, &f.vertices);
    let by_chord = chord_orbits(&traversed);
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
    for (o, face) in f.faces.entries() {
        // 50.3, 52.3: an edge transport of any order is a face. What it
        // carries is re-derived on the family below (`edges_rederive_frames`),
        // and a period is never a gate.
        // EVERY OVERLAP RE-DERIVES AT ITS MEMBER PAIR (AT1): the placement
        // and hue map re-derive from the two members its charts name, and an
        // overlap whose ends the form no longer holds does not re-derive.
        if face_rederives(f, &o, face) {
            continue;
        }
        // A PLACE THAT DOES NOT RE-DERIVE IS ADMITTED ONLY BY A CERTIFICATE
        // (AT1 A3): the pivot is the one producer of content the vertices do
        // not derive, and its certificate names the overlap it wrote. A
        // chord whose orbit carries is not a producer, so it never admits a
        // place alone.
        if !named.holds(&o) || !torsion_chord_carries(&traversed, &by_chord, &o) {
            return Some(face_at(&o));
        }
    }
    // 23.1 step 4, 50.3 (L7): every carried edge re-measured on the union
    // of its two bodies, on every pair of the family. BEFORE the orbits,
    // because an edge that does not re-derive is a defect at a FACE and
    // the reading names it there; the orbit reading over that same edge
    // would answer second-hand, as an unfiled class.
    if let Some(fam) = family {
        if let Some(refusal) = edges_rederive_frames(&traversed, fam) {
            return Some(refusal);
        }
    }
    // THE SPOKEN WORD'S OWN ATTESTATION, checked against the forest the
    // court just traversed (54.5).
    if !form_orbits_replay(&traversed) {
        // CV1 — the orbit's whole transport, read by the one reader (78.9):
        // no versor is P, a versor the dialect does not spell is D, one it
        // spells whose closure still refused is C.
        let composite = traversed.incoherent_orbit().and_then(|lp| {
            let carried = crate::transducer::sentence::circuit::paths_carried(&traversed.faces, &traversed.vertices, lp)?;
            carried.scoped().find_map(|(_, c)| c.composite())
        });
        return Some(Refusal::closure(
            REFUSE_ORBIT,
            sentence::At::Tape(spoken.address()),
            sentence::Content::Holonomy { versor: composite.as_ref(), chart: None, proof: "the orbit's transport lifts to no versor" },
        ));
    }
    // AND THE SPEAKER'S CLAIM, CHECKED AGAINST THE GEOMETRY THE COURT JUST
    // TRAVERSED (54.5). The terrain is priced above; this is the labour, and
    // the two are never one reading.
    if let Some(why) = receipts_price_the_terrain(&traversed, spoken.filing()) {
        return Some(tape(why));
    }
    let Some(derived) = traversed.rank() else {
        return Some(tape(REFUSE_RANK_LOST));
    };
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
    let _ = derived;
    if let Some(fam) = family {
        // "Does this form cite anything" is a read of the Glued kind's own
        // stratum of the vertex carrier (AT1), not a read of every vertex
        // asking each whether it is a citation.
        let glued = traversed.glued_vertices().next().is_some();
        let deflation_crystal: &dyn crate::transducer::family::Crystal = if glued {
            crystal
        } else {
            &crate::transducer::family::NoCrystal
        };
        let (pay, rems) =
            crate::transducer::family::form_deflation(&traversed, fam, deflation_crystal);
        // S9.4 — CLOSURE IS THE CERTIFICATE. A remainder on any pair means
        // the form does not close the family it claims, and that is the
        // refusal. The μ⃗ it measured is NOT: a form that closes every pair
        // while measuring nothing has still closed them, and pricing is the
        // economy's reading, taken after the court's. Under 50.5 (L7) the
        // clause reads the CLOSED SUB-COMPLEX: a remainder link a demand
        // vertex names is what the sentence said it could not close — valid,
        // not closed — and only an undeclared remainder refuses, at the
        // chart of the links it read (50.9).
        let _ = pay;
        let undeclared = undeclared_remainder(&rems, &traversed);
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
            let total: usize = rems.iter().map(Vec::len).sum();
            let (silent, wrong): (Vec<_>, Vec<_>) =
                undeclared.iter().copied().partition(|&(_, _, spoken, _)| spoken == 0);
            let chart = stalk::invariant(&undeclared);
            let prefix: String =
                chart.key().as_bytes().iter().take(2).map(|b| format!("{b:02x}")).collect();
            eprintln!(
                "  court remainder chart {prefix}… {} of {total} links: {} silent, {} wrong",
                undeclared.len(),
                silent.len(),
                wrong.len()
            );
            // THE WRONG ONES FIRST. A form carrying both said something
            // false, and that is the refusal it owes; contracting its
            // silence while leaving the falsehood would mint an address
            // that refuses again.
            // CV1 — A REMAINDER IS A RESIDUE ON THE COVER: its links are the
            // cocycle, read by the one reader (78.9).
            if !wrong.is_empty() {
                let content = sentence::Content::Residue(&wrong);
                let obstruction = sentence::read_obstruction(content);
                return Some(Refusal {
                    reason: REFUSE_NOT_CLOSED,
                    at: sentence::At::Chart(stalk::invariant(&wrong), wrong),
                    obstruction,
                });
            }
            let obstruction = sentence::read_obstruction(sentence::Content::Residue(&silent));
            return Some(Refusal {
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
    if traversed.faces.entries().any(|(o, face)| face.is_pivoted() && !named.holds(&o)) {
        return Some(tape(REFUSE_PIVOT_INEXACT));
    }
    replay_certificates(&traversed, &spoken.receipts().certificates, &by_chord).map(tape)
}

/// `REFUSE_FACE` at the overlap's two vertices, by their coordinates (50.9,
/// AT1): every end of an overlap is a vertex coordinate — a citation's is its
/// site (54.7 B5a) — so the refusal always has a coordinate to name.
fn face_at(o: &crate::network::nerve::coord::Overlap) -> Refusal {
    Refusal::new(REFUSE_FACE, sentence::At::face(o.from_vertex(), o.to_vertex()))
}

/// Chapter 23.1 steps 3 and 6: each Word vertex re-derives its rotor (M1)
/// and its affine; a Rebased whose word is the identity equals its base;
/// an Unfolded Remainder with nothing left does not move; an orbit whose
/// word is identity does not move. With a family, unfold and orbit execute
/// on the trains. The orbit's refusal is at the contact vertex with the
/// chart it acts on (50.9); every other refusal here is of the tape.
pub(crate) fn vertex_tape_and_moves(
    f: &Form,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    spoken: &Element,
) -> Option<Refusal> {
    use crate::transducer::family::{self, OrbitEnd};
    use crate::transducer::sentence::Measure;
    let tape = |reason: &'static str| Refusal::of_tape(reason, spoken);
    for (vertex, v) in f.vertices.entries() {
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
                            let painted = crate::arc::painted_of_grid(x);
                            // A grid with no rows has no width.
                            let frame = (x.len(), crate::arc::grid_width(x));
                            let anchor: Vec<(i64, i64)> = painted.iter().map(|&(r, c, _)| (r, c)).collect();
                            if matches!(family::orbit(v, &painted, &anchor, frame), OrbitEnd::NoOrbit) {
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
    for s in &lp.steps {
        if &s.overlap == chord {
            continue;
        }
        let Some(tf) = f.faces.at(&s.overlap) else {
            return false;
        };
        if !face_rederives(f, &s.overlap, tf) {
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

/// 54.5 — THE LABOUR, CHECKED AGAINST THE TERRAIN.
///
/// A receipt is a SUB-SELECTION: it names which cycles this mind's stroke
/// discharged, and the complex says what each closes at. So the court does
/// not take `ℓ` from the claimant — it takes the class from the claimant
/// and the period from the geometry.
///
/// ```text
///     yield(F, r) = Σ_{κ ∈ r ∩ dom(orbit_classes F)} (ℓ_derived(κ) − 1)
/// ```
///
/// A class the form does not carry projects to ZERO and is not a refusal.
/// An observer may track cycles outside this form's horizon — that is
/// pruning, not fraud, and refusing it would make a mind's own memory a
/// liability.
///
/// What IS refused is a receipt that carries a period the complex does not
/// derive. `Σ(ℓ − 1)` prices reserve regeneration (E2), so an unchecked
/// period mints capacity from nothing; and unlike the class, the period is
/// a claim ABOUT THE TERRAIN, which the terrain can contradict. That is
/// the one lie in the envelope the court can catch, and catching it is
/// what makes the attestation worth carrying.
pub(crate) fn receipts_price_the_terrain(
    f: &Form,
    filed: &crate::network::nerve::Keyed<
        crate::transducer::sentence::FiledOrbit,
        crate::network::nerve::coord::Holonomy,
    >,
) -> Option<&'static str> {
    if filed.is_empty() {
        return None;
    }
    // THE PERIODS THE COMPLEX DERIVES, by class, over every orbit. One
    // reader (`orbit_classes`), the same one `file_orbit` writes through and
    // `edge_coheres` asks.
    let mut derived: Keyed<
        crate::transducer::sentence::ToralPeriod,
        crate::network::nerve::coord::Holonomy,
    > = Keyed::new();
    for lp in &f.orbits {
        for row in f.orbit_classes(lp) {
            derived.put(&row.class, row.periods);
        }
    }
    for (class, claim) in filed.entries() {
        match derived.at(&class) {
            // The period is a claim about the terrain and the terrain
            // disagrees.
            // BOTH PLANES. A claim that agrees on the `lcm` and differs on
            // the pair is a different orbit, and the terrain is what the
            // complex derives — so the comparison is the spectrum (54.7).
            Some(&p) if claim.periods != p => return Some(REFUSE_INFLATED_YIELD),
            // Claimed, and the complex agrees.
            Some(_) => {}
            // A class this form does not carry. It earns nothing here and
            // says nothing against the speaker (rule 11).
            None => {}
        }
    }
    None
}

/// A20: each certificate is the un-pivot of one chord face, re-read.
/// The orbit's class key and rank before and after must match the tape.
/// The orbit is reached by its chord's coordinates on each side (AT1), never
/// by its position in either form's list.
pub(crate) fn replay_certificates(
    f: &Form,
    certificates: &[crate::transducer::sentence::PivotCertificate],
    by_chord: &Keyed<crate::transducer::sentence::Orbit, crate::network::nerve::coord::Overlap>,
) -> Option<&'static str> {
    // THE CERTIFICATES ARE THE SPEAKER'S (54.5 Road (d), B3). They hung off
    // the form, and a certificate is not geometry: it is what ONE mind's
    // stroke did to this complex against ONE budget. Two minds holding the
    // same form pivot differently, so the map does not factor through the
    // form and `from_address` cannot rebuild it — the same argument that
    // moved the filing at `df1eeaa`, one carrier later.
    //
    // Every certificate the envelope carries is re-read or the word is
    // refused; there is no subset of them that would do.
    for cert in certificates {
        let face = cert.by.face();
        let Some(after) = by_chord.at(face) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        let mut pre = f.clone();
        // A move with no inverse cannot be un-pivoted: the certificate is
        // not re-read, and it is refused rather than read against a form the
        // skip left unrestored (VS1's ruling, item 6).
        if !pre.unapply_move(&cert.by) {
            return Some(REFUSE_PIVOT_INEXACT);
        }
        pre.orbits = Form::orbits_of(&pre.faces, &pre.vertices);
        let before = chord_orbits(&pre);
        let Some(lp_before) = before.at(face) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        // An opened axis (I6) is seeded by the orbit's own holonomy versor
        // (16.20): the court re-derives the seed from the orbit it re-read
        // and refuses a certificate whose seed is anything else (law 12).
        if let crate::transducer::sentence::PivotBy::OpenAxis { seed, .. } = &cert.by {
            let rederived = pre.axis_seed_of(lp_before);
            if !rederived.is_some_and(|v| element::same_versor(&v, seed)) {
                return Some(REFUSE_PIVOT_INEXACT);
            }
        }
        let Some(kappa_before) = pre.class_vector_of(lp_before) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        if kappa_before.class_key() != cert.before {
            return Some(REFUSE_PIVOT_INEXACT);
        }
        let Some(kappa_after) = f.class_vector_of(after) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        if kappa_after.class_key() != cert.after || kappa_after.occupies(cert.axis) {
            return Some(REFUSE_PIVOT_INEXACT);
        }
        let Some(rb) = pre.rank() else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        let Some(ra) = f.rank() else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        // S9.1 — THE COURT RE-DERIVES THE MOVE. The certificate claims what
        // the pivot did to the rank; the court traverses the form before and
        // after for itself and computes the same move. The two absolute
        // audits it used to compare against are the court's own here, so
        // carrying them on the tape only invited a second comparison.
        if crate::transducer::sentence::Rank::moved(rb, ra) != cert.rank_move {
            return Some(REFUSE_PIVOT_INEXACT);
        }
    }
    None
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
