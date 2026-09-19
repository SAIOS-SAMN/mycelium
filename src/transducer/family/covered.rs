// 53.6 — THE COVER'S OWN FORM: its pieces, their meets, and the loops it
// stands, classified and discarded.
//
//! A piece is a reading inside a cover, not an idea the scene holds. Its
//! CLASS is the scene's vertex (53.6 item 1), its meets are incidence the
//! scene builds on supports (item 3), and what its loops carry is a
//! torsion class the scene files by class (item 4).
//!
//! So the pieces get a form of their own, here, whose only purpose is to
//! carry their loops long enough to classify them. Placed into the scene's
//! form instead, a cover of two thousand pieces WAS a form of two thousand
//! vertices — `484b58aa` read 2 055 vertices, 11 167 faces and 3 986 loops
//! — and every carrier the engine has is multiplied by that count.
//!
//! Split from `extract.rs` at the §1.1 1200-line reading bound; the item
//! is moved, not rewritten (§1.6).

use super::pair::{meet_edge, Covered};
use super::perceive::EdgeTally;
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::sentence::{face_pair_key, vertex_ix_key, Form, FormFace};

/// One filed loop of a cover, awaiting a chord the SCENE can name: the
/// holonomy's class key, the loop as filed, and the transport classes its
/// ends belong to.
pub(crate) type PendingTorsion = (
    crate::transducer::address::Address,
    crate::transducer::sentence::FiledLoop,
    Vec<TransportClass>,
);

/// Build the pieces' form, classify its loops, and return what the scene
/// keeps: how many meet faces stood, the torsion each loop filed, and the
/// classes standing per cover.
pub(crate) fn classify_cover(
    covers: &[Covered],
    edges: &mut EdgeTally,
) -> (usize, Vec<PendingTorsion>, Keyed<Vec<crate::transducer::sentence::ClassVector>>) {
    // 53.6 items 3 and 4 — THE PIECES' OWN FORM, BUILT HERE AND DISCARDED.
    //
    // The pieces were placed into the SCENE's form and their meets became
    // its faces, so a cover of two thousand pieces was a form of two
    // thousand vertices. A piece is a reading inside a cover, not an idea
    // the scene holds: its class is already a vertex (item 1), its meets
    // are already incidence the form builds on supports (item 3), and what
    // its loops carry is a torsion class the form files by class (item 4).
    //
    // So the pieces get a form of their own, here, whose only purpose is to
    // carry their loops long enough to classify them. Nothing downstream
    // ever sees it.
    let mut piece_form = Form::default();
    let mut cover_vertex: Keyed<u16> = Keyed::new();
    let mut owner: Keyed<Keyed<()>> = Keyed::new();
    for (ci, c) in covers.iter().enumerate() {
        for (key, (v, _versor, _codomain)) in c.cover.carried.scoped() {
            if v.face_chart().is_none() {
                continue;
            }
            let ix = piece_form.vertices.len() as u16;
            piece_form.vertices.push(v.clone());
            cover_vertex.insert(&key, ix);
            let mut set = Keyed::new();
            set.insert(ci.to_le_bytes(), ());
            owner.insert(vertex_ix_key(ix), set);
        }
    }
    // THE COVER'S MEETS ARE FACES OF THE PIECES' OWN FORM (50.3), not of
    // the scene's. Between the CLASS vertices the meets become incidence
    // the form already builds on supports (53.6 item 3), which is where a
    // meet between two ideas belongs; here they are only what the loops
    // below are read around.
    for c in covers {
        for (mk, ()) in c.cover.meets.scoped() {
            let Some((ka, kb)) = crate::transducer::family::pieces::meet_ends(&mk) else { continue };
            let (Some(&i), Some(&j)) = (cover_vertex.get(ka), cover_vertex.get(kb)) else { continue };
            let (Some(vi), Some(vj)) = (piece_form.vertices.get(i as usize), piece_form.vertices.get(j as usize)) else { continue };
            let (Some(ci), Some(cj)) = (vi.face_chart(), vj.face_chart()) else { continue };
            let edge = edges.take(match (c.cover.carried.get(ka), c.cover.carried.get(kb)) {
                // A meet of two SECTIONS has one `Γ` and therefore one
                // refusal — there is no coset to read it through here, so
                // the single reading is carried as the one-member list the
                // tally counts.
                (Some((va, sa, _)), Some((vb, sb, _))) => {
                    Some(meet_edge(va, sa, vb, sb).map_err(|e| vec![e]))
                }
                _ => None,
            });
            if edge.is_some() {
                crate::transducer::family::cover::tally(|t| t.edges += 1);
            }
            let Some(word) = piece_form.face_word(i, j) else { continue };
            let Some(place) = piece_form.face_place(i, &ci, j, &cj, edge) else { continue };
            let mut at = Keyed::new();
            at.insert(face_pair_key(&ci, &cj), place);
            piece_form.faces.push(FormFace { i, j, word, at, pairs: None });
        }
    }
    let meet_faces = piece_form.faces.len();
    piece_form.loops = Form::loops_of(&piece_form.faces, &piece_form.vertices);
    // EACH LOOP OVER CARRIED PIECES, CLASSIFIED ONCE (50.8 case 3), by the
    // one classifier both organs use, on its edge holonomy per member path.
    let mut stood: Keyed<Vec<crate::transducer::sentence::ClassVector>> = Keyed::new();
    // The cover's filed loops, held until the scene has faces to name a
    // chord in (53.6 item 4).
    let mut pending_torsion: Vec<(
        crate::transducer::address::Address,
        crate::transducer::sentence::FiledLoop,
        Vec<TransportClass>,
    )> = Vec::new();
    for lp in piece_form.loops.clone() {
        // THE LOOP'S COVER: the one cover that placed every vertex of it —
        // a vertex two covers' pieces became is in both — and none where a
        // vertex belongs to no cover or the vertices to different ones.
        let mut common: Option<Keyed<()>> = None;
        for v in &lp.vertices {
            let Some(set) = owner.get(&vertex_ix_key(*v)) else {
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
        // PHASE B — THE LOOP BELONGS TO THE FORM, NOT TO ONE COVER.
        //
        // `owner` holds an entry only for a COVER PIECE, and each piece
        // carries the singleton index of the `Covered { demand, cover,
        // frame }` it came from. A loop reaching a word vertex or a demand
        // vertex therefore met `owner.get(..) == None` and was dropped at
        // `common`; a loop spanning two covers or two frames met an empty
        // intersection and was dropped at `[ok]`. What passed both was a
        // loop confined to ONE cover snapshot at ONE frame — a single
        // coordinate patch, whose transitions were already solved
        // consistent when the cover was read.
        //
        // Measured on that gate: 16 546 loops on 174 forms in ten minutes,
        // 292 classified, and 292 of 292 FLAT. Curvature lives on the
        // overlaps — `g_AB · g_BC · g_CA` around a cycle crossing charts —
        // and the gate admitted only the interiors.
        //
        // The cover index is now read where it is USED and no longer
        // gates the reading: every loop is classified and tallied, and a
        // `Congruent` fill files its torsion by CLASS, which needs no
        // cover. `stood` is keyed by cover index, so a loop with no single
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
            crate::transducer::family::cover::tally_loop(&fill);
            match fill {
                crate::transducer::cech::Fill::Congruent(_, g, ell) => {
                    let Some(class) = g.class_key() else { continue };
                    eprintln!("  simplex filled {:02x}{:02x}… class {ell}", class.as_bytes()[0], class.as_bytes()[1]);
                    // 53.6 item 4: the loop is classified HERE, inside the
                    // cover, and filed by class where the form files
                    // torsion — but its chord indexes the PIECES' form,
                    // which is discarded. A chord is a coordinate of the
                    // form that holds the loop, so the scene's chord is
                    // resolved after the scene's faces exist, between the
                    // two CLASS vertices the loop's ends belong to. Filed
                    // with a piece index it would name a face nothing has
                    // (rule 11).
                    let ends: Vec<TransportClass> = lp
                        .vertices
                        .iter()
                        .filter_map(|v| piece_form.vertices.get(*v as usize))
                        .filter_map(transport_class)
                        .collect();
                    pending_torsion.push((
                        class,
                        crate::transducer::sentence::FiledLoop { sigma, period: ell, chord: 0 },
                        ends,
                    ));
                }
                crate::transducer::cech::Fill::Class(kappa, _) => {
                    // A standing class is filed under the cover it stands
                    // on. An interface loop stands on no single cover:
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

    (meet_faces, pending_torsion, stood)
}
