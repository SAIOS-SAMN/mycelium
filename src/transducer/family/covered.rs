// 53.6 — THE COVER'S OWN FORM: its pieces, the pieces' meets, and the orbits it
// stands, classified and discarded.
//
//! A piece is a reading inside a cover, not an idea the scene holds. Its
//! CLASS is the scene's vertex (53.6 item 1), its meets are incidence the
//! scene builds on supports (item 3), and what its orbits carry is a
//! torsion class the scene files by class (item 4).
//!
//! So the pieces get a separate form, here, whose only purpose is
//! to carry the pieces' orbits long enough to classify them. Placed into the scene's
//! form instead, a cover of two thousand pieces WAS a form of two thousand
//! vertices — `484b58aa` read 2 055 vertices, 11 167 faces and 3 986 orbits
//! — and every carrier the engine has is multiplied by that count.
//!
//! Split from `extract.rs` at the §1.1 1200-line reading bound; the item
//! is moved, not rewritten (§1.6).

use super::pair::{meet_edge, Covered};
use super::perceive::EdgeTally;
use super::*;
use crate::network::nerve::coord::{ChartAt, Overlap, PieceAt};
use crate::network::nerve::{Coordinate, Keyed};
use crate::transducer::sentence::{Form, FormFace, Producer};

/// One filed orbit of a cover, awaiting a chord the SCENE can name: the
/// holonomy's class key, its hue generator and plane periods — the filed
/// orbit but for the chord, which is an overlap of the scene's and does not
/// exist here (AT1) — the transport classes its ends belong to, and the
/// holonomy's rotor.
pub(crate) type PendingTorsion = (
    crate::network::nerve::coord::Holonomy,
    (Vec<(u8, u8)>, crate::transducer::sentence::ToralPeriod),
    Vec<TransportClass>,
    crate::transducer::alphabet::Rotor,
);

/// A PIECE AS A CHART NAMES IT (AT1, ruled (a) on the chords): the nucleus
/// of the body its footprint lifts to — the body's sphere, 78.31 — and the
/// shape's fixed-size canonical coordinates, never the support sites. `None`
/// where the footprint lifts to no body with a nucleus.
pub(crate) fn piece_at(footprint: &[crate::transducer::base::Site]) -> Option<PieceAt> {
    let blade = crate::transducer::blade::shape_of(footprint).blade;
    Some(PieceAt { nucleus: blade.nucleus()?, moments: blade.moments() })
}


/// THE CHORD A COVER'S MEET NAMES, AT THE MEMBER AND NOT AT THE VERTEX
/// (53.6 item 3, corrected 2026-09-21 by the lift's own reading).
///
/// A meet is between two PIECES; the scene holds classes. The lift is the
/// transport class of each end — the coordinate `Form::by_transport` files
/// the scene's vertices under — TOGETHER WITH the chart that end stands at,
/// because a face is between two MEMBERS and not between two vertices.
///
/// MEASURED: 186 meets over the 36 corpus forms, 186 of them carrying an
/// edge, `unclassed` 0 — and 186 of 186 WITHIN ONE CLASS. A first cut kept
/// only meets whose classes differed and so kept none of them.
///
/// That is not a defect of the corpus, it is what a `TransportClass` IS. It
/// is `⟨kind, conjugacy of the rotor⟩` and 21 step 1 excludes the carriage
/// deliberately — "never the hue map and never the body's moduli" — which
/// is exactly what makes two copies of one shape at two places CORRESPOND,
/// so a rule generalises. A deformation that moves a shape carries every
/// piece by the same rotation and differs only in TRANSLATION, so every
/// piece lands in one class by construction.
///
/// So the cycle is not between two class vertices; it is between two
/// MEMBERS of one class vertex — two `Acting` entries on two charts, each
/// with its own carriage — and its holonomy is the relative transport
/// between them, which is a translation: a Burgers vector, a dislocation.
/// 18.2's own sentence is "two carriages on one class are two sections".
///
/// Widening the class to separate them would fix the count and destroy
/// correspondence. The translation belongs at the member level, which is
/// where `orbits_of` already builds: `orbit_words_lift_every_orbit`.
/// THE FOOTPRINT RIDES THE CHORD (54.7, ruled 2026-09-21).
///
/// MEASURED: `classify_cover`'s gate keys a meet's end by
/// `(class, chart, carriage, FOOTPRINT)` and admitted 186 of 186. The chord
/// it pushed carried only the first three, so `extract`'s face builder
/// rebuilt each end as `section_key(ix, chart, φ)` — identical at both ends
/// for 184 of them — and `(ma != mb)` dropped every one. 186 chords in, 2
/// faces out, `b₁ = 0` on every form of the corpus.
///
/// The footprint is what distinguishes them: two disjoint bodies under the
/// identical transformation have `φ_a = φ_b` and `D_a ≠ D_b`, so the
/// relative carriage is `1` and the transport across the interface is the
/// SOLDER FORM displacement between the two basepoints — a hyperbolic
/// transvection, `B = eᵢ∧e₋`, `I₁ > 0`, carrying a Burgers vector. A
/// coordinate excluded from IDENTITY, correctly, and then excluded from
/// INCIDENCE, which does not follow.
///
/// AT1, ruled (a): the end is its PIECE-CHART in the pieces' form — the
/// class's vertex, the chart, the frame, the piece by nucleus and shape —
/// and the scene re-reads it at the scene's vertex of the same class. The
/// footprint's sites ride beside it for the dispiration's measurement only;
/// no key is written from them.
pub(crate) type ChordEnd = (TransportClass, ChartAt, Versor, Vec<crate::transducer::base::Site>);

///
/// AND THE FRAME ITS COVER WAS READ ON: a chord is an overlap between two
/// piece-charts of one member AT ONE FRAME (AT1, ruled (a)).
pub(crate) type MeetChord = (
    ChordEnd,
    ChordEnd,
    Option<crate::transducer::sentence::Transport>,
    u64,
);

/// Build the pieces' form, classify its orbits, and return what the scene
/// keeps: the chords its meets name between classes, how many meet faces
/// stood, the torsion each orbit filed, and the classes standing per cover.
pub(crate) fn classify_cover(
    covers: &[Covered],
    edges: &mut EdgeTally,
    budget: &mut crate::physics::Budget,
) -> (
    Vec<MeetChord>,
    usize,
    Vec<PendingTorsion>,
    Keyed<Vec<crate::transducer::sentence::ClassVector>>,
) {
    // 53.6 items 3 and 4 — THE PIECES' OWN FORM, BUILT HERE AND DISCARDED.
    //
    // The pieces were placed into the SCENE's form and the pieces' meets became
    // its faces, so a cover of two thousand pieces was a form of two
    // thousand vertices. A piece is a reading inside a cover, not an idea
    // the scene holds: its class is already a vertex (item 1), its meets
    // are already incidence the form builds on supports (item 3), and what
    // its orbits carry is a torsion class the form files by class (item 4).
    //
    // So the pieces get a separate form, here, whose only purpose
    // is to carry the pieces' orbits long enough to classify them. Nothing downstream
    // ever sees it.
    let mut piece_form = Form::default();
    // EACH CARRIED PIECE AT ITS PIECE-CHART (AT1, ruled (a)): its class's
    // vertex, its chart, its cover's frame, and the piece by nucleus and
    // shape. The chart is what a meet's end is, and the cover that placed it
    // is filed under the same coordinate.
    let mut cover_chart: Keyed<ChartAt, crate::network::nerve::coord::Carriage> = Keyed::new();
    let mut owner: Keyed<Keyed<()>, ChartAt> = Keyed::new();
    for (ci, c) in covers.iter().enumerate() {
        let (carried, _) = c.cover.carried.decoded();
        let Ok(frame) = u64::try_from(c.frame) else { continue };
        for (key, (v, _versor, sup)) in carried {
            // THE VERTEX A CARRIED PIECE STANDS AT: one per class, the pieces
            // of one class its members (AT1), at the frame its cover was read
            // on.
            let Some(piece) = piece_at(&sup.codomain) else { continue };
            let Some(ChartAt::At(member)) = piece_form.stand_member(v, frame) else { continue };
            let pc = ChartAt::piece(member.vertex, member.chart, frame, piece);
            match owner.at_mut(&pc) {
                Some(set) => {
                    set.insert(ci.to_le_bytes(), ());
                }
                None => {
                    let mut set = Keyed::new();
                    set.insert(ci.to_le_bytes(), ());
                    owner.put(&pc, set);
                }
            }
            cover_chart.put(&key, pc);
        }
    }
    // THE COVER'S MEETS ARE FACES OF THE PIECES' OWN FORM (50.3), not of
    // the scene's. Between the CLASS vertices the meets become incidence
    // the form already builds on supports (53.6 item 3), which is where a
    // meet between two ideas belongs; here they are only what the orbits
    // below are read around.
    // 53.6 item 3 — THE MEETS, LIFTED TO THE CLASSES THEY RELATE.
    //
    // This said the meets "become incidence the form already builds on
    // supports", and the telemetry said otherwise: `faces == site+ring` on
    // every row of every recut, so a form with 45 measured meets carried
    // one face and that face was a star face. The supports builder
    // (`named_faces`, then `star_and_site_faces`) offered the Frame's star,
    // a spanning CHAIN of each site's occupants, and the ring pairs — by
    // construction a forest,
    // as its own comment says: "the only cycles that survive are the ones
    // spanning DISTINCT sites". Those cycles are exactly what a meet
    // between two classes would close, and nothing offered one.
    //
    // So b₁ = E − V + C = 0 identically, every form carried `0 orbits`, and
    // everything downstream of a cycle — `pivot_orbit`, the stand, the
    // close, the torsion filing, the court's basis — was starved rather
    // than broken. Measured at epoch 94: `pivoted [0,0,0,0]` on all five
    // voices, `court basis 0` on all three courts.
    //
    // The lift already existed for the orbits' ENDS, one screen down: a
    // piece's `transport_class` is the coordinate the scene files vertices
    // under. It was never applied to the meets themselves.
    let mut chords: Vec<MeetChord> = Vec::new();
    let mut named: Keyed<()> = Keyed::new();
    for c in covers {
        let Ok(frame) = u64::try_from(c.frame) else { continue };
        // THE MEETS, AS COORDINATES (§4.4 E8). A key that does not decode is
        // this reader failing on its own layout; it is not a meet that named
        // nothing.
        let (meets, _) = c.cover.meets.decoded();
        for (mk, ()) in meets {
            let (ka, kb) = mk.ends();
            let (Some(pi), Some(pj)) = (cover_chart.at(ka), cover_chart.at(kb)) else { continue };
            let (Some(vi), Some(vj)) = (piece_form.vertices.at(&pi.vertex()), piece_form.vertices.at(&pj.vertex())) else {
                continue;
            };
            let (class_i, class_j) = (transport_class(vi), transport_class(vj));
            let edge = edges.take(match (c.cover.carried.at(ka), c.cover.carried.at(kb)) {
                // A meet of two SECTIONS has one `Γ` and therefore one
                // refusal — there is no coset to read it through here, so
                // the single reading is carried as the one-member list the
                // tally counts.
                // THE SUPPORTS TRAVEL WITH THE VERSORS (56.5). Both were
                // in hand here and dropped as `_`, so the two charts met
                // as unoriented bodies and no interface could carry a
                // turn. They are the same supports the chord's ends
                // already carry.
                (Some((va, sa, supa)), Some((vb, sb, supb))) => {
                    Some(meet_edge(va, sa, supa, vb, sb, supb, budget).map_err(|e| vec![e]))
                }
                _ => None,
            });
            let carried_edge = edge.clone();
            // THE MEET IS AN OVERLAP OF THE TWO PIECE-CHARTS (AT1): its word
            // derived from the ends, its place measured here. A meet of one
            // piece-chart with itself is no overlap.
            // One overlap per pair of charts: the meet read from the other
            // end is the same overlap backward, not a second face.
            if pi != pj && piece_form.faces.at(&Overlap { from: pj.clone(), to: pi.clone() }).is_none() {
                if let Some(place) = piece_form.face_place(pi, pj, edge) {
                    piece_form.put_face(Overlap { from: pi.clone(), to: pj.clone() }, FormFace::placed(place, Producer::Place));
                }
            }
            // AND THE CHORD IT NAMES BETWEEN TWO CLASSES. Two pieces of ONE
            // class meet inside that class and close nothing the scene can
            // see, so only a meet between DISTINCT classes is a chord. Named
            // once: several pieces of two classes meeting is one incidence
            // between those two classes, and the scene has one face for it.
            // WHY A MEET IS NOT A CHORD, counted at the step that decides
            // it (§8.1). Three outcomes and they mean different things: a
            // meet inside ONE class closes nothing the scene can see, a
            // meet whose pieces carry no transport class cannot be lifted
            // at all, and the rest are chords.
            // The chord END is named by where the piece LANDED — that is
            // the coordinate that tells two sections of one class apart —
            // so identity reads the codomain here. The nerve's OVERLAPS
            // are a different question and read the domain (`Supports`).
            let (Some((_, pa, supa)), Some((_, pb, supb))) =
                (c.cover.carried.at(ka), c.cover.carried.at(kb))
            else {
                continue;
            };
            let (da, db) = (&supa.codomain, &supb.codomain);
            if let (Some(ta), Some(tb)) = (class_i, class_j) {
                // THE CHORD IS BETWEEN TWO SECTIONS, so the coordinate that
                // must differ is `(class, chart, CARRIAGE)`.
                //
                // Gated on `(class, chart)` it admitted nothing: 186 meets
                // over the 36 corpus forms, 186 of them `within one class`,
                // `0 orbits` on every form. That is what 53.6 item 1 IS —
                // two pieces of one rotor class quotient to one base vertex,
                // and on one chart they are one member. The meet is not in
                // the base; it is in the TOTAL space, between two sections
                // `σ₁ = (V,U,φ₁)` and `σ₂ = (V,U,φ₂)` of the fibre `π⁻¹(V)`,
                // and the only coordinate separating them is the carriage.
                //
                // Projected to the base this 1-cell is a SELF-LOOP on `V`:
                // `∂e = e⁰ − e⁰ = 0`, `b₁ = 1`, holonomy `τ = φ₂·φ₁⁻¹`. A
                // graph forbids that edge by convention; a CW complex does
                // not, and the circle's minimal decomposition IS it.
                //
                // Two copies of one shape never share a support site, so
                // `named_faces` — which relates members whose supports meet
                // or touch at a corner (70.4) — cannot see
                // them at all. The cover's meet is the only carrier of that
                // relation, which is why the chord has to come from here.
                // AND THE FOOTPRINT, which is the coordinate two pieces
                // carried by ONE versor differ in (54.7, as ruled
                // 2026-09-21).
                //
                // Gated on `(class, chart, carriage)` this admitted 2 of 186
                // meets. The other 184 have both ends at one class, one
                // chart and ONE CARRIAGE — two disjoint bodies undergoing
                // the identical transformation, so `φ₂·φ₁⁻¹ = 1` and the
                // gate read them as one section meeting itself at zero
                // distance.
                //
                // They are not one section. `D₁ ≠ D₂`, so the two basepoints
                // on the absolute differ, and the transport across the
                // interface is not the relative carriage but the SOLDER
                // FORM displacement between them:
                //
                //     Ω₁₂ = φ · T(Δx) · φ⁻¹ ,   Δx = p(D₂) − p(D₁) ≠ 0
                //
                // `T(Δx)` is a hyperbolic transvection — `B = eᵢ∧e₋`,
                // `B² = +1 > 0` — so `Ω₁₂ ≠ 1` and the chord carries a
                // Burgers vector `τ = Δx`. Omitting the footprint is the
                // same collapse the session has now found four times: a
                // coordinate excluded from IDENTITY, correctly, and then
                // excluded from INCIDENCE, which does not follow.
                //
                // AT1, ruled (a): the footprint is named by the piece's
                // fixed-size coordinates — nucleus and shape — on its chart,
                // so the two ends are told apart by the PIECE-CHARTS and
                // never by the sites.
                let (ma, mb) = (pi.encode(), pj.encode());
                // ONE SECTION MEETING ITSELF — one class, one chart, one
                // carriage — has the identity for its transition and carries
                // no `τ`, so there is no cycle and none is invented (rule 11).
                if ma != mb {
                    let mut pk = Vec::new();
                    let (lo, hi) = if ma < mb { (&ma, &mb) } else { (&mb, &ma) };
                    crate::transducer::address::put_key_bytes(&mut pk, lo.as_bytes());
                    crate::transducer::address::put_key_bytes(&mut pk, hi.as_bytes());
                    if named.get(&pk).is_none() {
                        named.insert(pk, ());
                        // AND THE MEET'S OWN EDGE RIDES WITH IT. The
                        // transition across this interface was measured
                        // just above — `meet_edge(va, sa, vb, sb)`, the
                        // `V_b ∘ V_a⁻¹` between the two pieces' own frames
                        // — and it was spent on the piece form that is
                        // discarded.
                        //
                        // The scene's face cannot re-derive it:
                        // `edge_between` is keyed by `(vertex, chart)`,
                        // which is identical at both ends of a section
                        // chord, so it answers nothing and the orbit stands
                        // with no holonomy to read. An orbit that exists and
                        // cannot be evaluated is the empty-answer defect —
                        // "there is no lift" and "there is no holonomy"
                        // returned as one silence — so the measurement
                        // travels to the carrier that needs it.
                        // AND THE FOOTPRINT TRAVELS WITH IT. The gate above
                        // separated these two ends BY the footprint; the
                        // chord dropped it and the face builder then could
                        // not tell them apart either (54.7).
                        chords.push((
                            (ta, pi.clone(), pa.clone(), da.clone()),
                            (tb, pj.clone(), pb.clone(), db.clone()),
                            carried_edge.clone(),
                            frame,
                        ));
                    }
                }
            }
        }
    }
    let meet_faces = piece_form.faces.len();
    piece_form.orbits = Form::orbits_of(&piece_form.faces, &piece_form.vertices);
    // EACH ORBIT OVER CARRIED PIECES, CLASSIFIED ONCE (50.8 case 3), by the
    // one classifier both organs use, on its edge holonomy per member path.
    let mut stood: Keyed<Vec<crate::transducer::sentence::ClassVector>> = Keyed::new();
    // The cover's filed orbits, held until the scene has faces to name a
    // chord in (53.6 item 4).
    let mut pending_torsion: Vec<PendingTorsion> = Vec::new();
    for lp in piece_form.orbits.clone() {
        // THE ORBIT'S COVER: the one cover that placed every vertex of it —
        // a vertex two covers' pieces became is in both — and none where a
        // vertex belongs to no cover or the vertices to different ones.
        let mut common: Option<Keyed<()>> = None;
        for pc in lp.charts() {
            let Some(set) = owner.at(&pc) else {
                common = None;
                break;
            };
            common = Some(match common {
                None => set.clone(),
                Some(held) => {
                    let mut meet = Keyed::new();
                    for (k, ()) in held.scoped() {
                        if set.contains(&k) {
                            meet.insert(&k, ());
                        }
                    }
                    meet
                }
            });
        }
        // PHASE B — THE ORBIT BELONGS TO THE FORM, NOT TO ONE COVER.
        //
        // `owner` holds an entry only for a COVER PIECE, and each piece
        // carries the singleton index of the `Covered { demand, cover,
        // frame }` it came from. An orbit reaching a word vertex or a demand
        // vertex therefore met `owner.get(..) == None` and was dropped at
        // `common`; an orbit spanning two covers or two frames met an empty
        // intersection and was dropped at `[ok]`. What passed both was an
        // orbit confined to ONE cover snapshot at ONE frame — a single
        // coordinate patch, whose transitions were already solved
        // consistent when the cover was read.
        //
        // Measured on that gate: 16 546 orbits on 174 forms in ten minutes,
        // 292 classified, and 292 of 292 FLAT. Curvature lives on the
        // overlaps — `g_AB · g_BC · g_CA` around a cycle crossing charts —
        // and the gate admitted only the interiors.
        //
        // The cover index is now read where it is USED and no longer
        // gates the reading: every orbit is classified and tallied, and a
        // `Congruent` fill files its torsion by CLASS, which needs no
        // cover. `stood` is keyed by cover index, so an orbit with no single
        // owner has no cover to be attributed to and is classified without
        // one — the reading is taken, and the carrier that cannot hold it
        // is not written with an invented index (rule 11).
        let ci: Option<usize> = common.and_then(|c| {
            let owners: Vec<Vec<u8>> = c.scoped().map(|(k, _)| k).collect();
            match owners.as_slice() {
                [ok] => <[u8; 8]>::try_from(ok.as_slice()).ok().map(usize::from_le_bytes),
                _ => None,
            }
        });
        let Some(es) = piece_form.edge_holonomies(&lp) else { continue };
        for (_, e) in es.scoped() {
            let hue = crate::transducer::sentence::CycleType::of(&e.gauge);
            let sigma = e.gauge.clone();
            let fill = crate::transducer::cech::classify(&e.word.rotor(), &e.carriage, hue);
            match fill {
                crate::transducer::cech::Fill::Congruent(_, g, _) => {
                    let Some(class) = g.holonomy_key() else { continue };
                    // 53.6 item 4: the orbit is classified HERE, inside the
                    // cover, and filed by class where the form files
                    // torsion — but its chord indexes the PIECES' form,
                    // which is discarded. A chord is a coordinate of the
                    // form that holds the orbit, so the scene's chord is
                    // resolved after the scene's faces exist, between the
                    // two CLASS vertices the orbit's ends belong to. Filed
                    // with a piece index it would name a face nothing has
                    // (rule 11).
                    let ends: Vec<TransportClass> = lp
                        .vertices()
                        .iter()
                        .filter_map(|v| piece_form.vertices.at(v))
                        .filter_map(transport_class)
                        .collect();
                    pending_torsion.push((class, (sigma, g.toral_period()), ends, g.as_ref().clone()));
                }
                crate::transducer::cech::Fill::Class(kappa, _) => {
                    // A standing class is filed under the cover it stands
                    // on. An interface orbit stands on no single cover:
                    // it is classified and counted, and no cover is
                    // invented to file it under.
                    if let Some(ci) = ci {
                        let k = ci.to_le_bytes();
                        match stood.get_mut(&k) {
                            Some(v) => v.push(*kappa),
                            None => {
                                stood.insert(k, vec![*kappa]);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    (chords, meet_faces, pending_torsion, stood)
}
