// 50.9 — THE SIX CONTRACTION MOVES: what a refusal does to the form that
// was refused, each move a strict shrinking of the closed sub-complex.
//
//! A refused vertex becomes a demand and takes its faces with it; a refused
//! edge becomes a remainder edge; undeclared links are declared as a Source
//! demand; a contact word that does not orbit becomes a demand as a refused
//! vertex does; a face that does not re-derive leaves the form; a malformed
//! complex is not moved at all — the mind is quiescent on it. Every move
//! re-walks the orbits and reindexes; nothing here trusts the tape's own orbit
//! list, and every read is a walk of this form's own vertices and faces
//! (rule 1).

use super::*;

/// What a contraction did: the form moved and its closed sub-complex
/// shrank, or the form is a malformed complex and stays as it is, inert.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Contracted {
    Moved,
    Quiescent,
}

impl Form {
    /// The index of the word vertex at `address`, read off this form's own
    /// vertices — its boundary, never a search over anything else.
    fn word_index(&self, address: &crate::transducer::address::Address) -> Option<u16> {
        for (i, v) in self.vertices.iter().enumerate() {
            if let Vertex::Word { address: a, .. } = v {
                if a == address {
                    return Some(i as u16);
                }
            }
        }
        None
    }

    /// The index of the demand vertex of `kind` on `chart`, read off this
    /// form's own vertices.
    fn demand_index(&self, kind: &crate::transducer::sentence::Demand, chart: &stalk::Invariant) -> Option<usize> {
        for (i, v) in self.vertices.iter().enumerate() {
            if let Vertex::Demand { kind: k, chart: c, .. } = v {
                if k == kind && c == chart {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Execute the move a refusal names at its coordinate (50.9). `None`
    /// when the coordinate names nothing this form holds — a vertex it has
    /// no word at, a face between vertices it does not hold — so a
    /// certificate for it could never re-derive. `remainder` is what the
    /// contracted form leaves on the speaker's family, the bodies a refused
    /// word's demand stands for.
    pub fn contract(&mut self, kind: RefusalKind, at: &At, remainder: &[Link]) -> Option<Contracted> {
        match (kind, at) {
            (RefusalKind::VertexBody, At::Vertex { address, chart }) => {
                let i = self.word_index(address)?;
                let ck = chart.key();
                let Some(Vertex::Word { acts, .. }) = self.vertices.get_mut(i as usize) else {
                    return None;
                };
                // THE MEMBER AT THE REFUSED CHART leaves the vertex (D10 item
                // 13: the coordinate is the member's own), and a demand over
                // that chart carries as its bodies the links the word was to
                // reach. A vertex whose last member left becomes the demand
                // itself, and a demand vertex enters no face (50.5): the
                // faces that reached it leave with it. A vertex that keeps
                // other members keeps its faces at those members' pairs and
                // loses the pairs at the refused one.
                // EVERY SECTION AT THE REFUSED CHART LEAVES (54.7 R1). It
                // removed one entry by the chart key, which named the
                // member while the fibre was keyed by chart alone. The key
                // is `chart ‖ carriage` now, so the chart is a PREFIX and
                // the refusal is about the CHART — every sheet over it goes,
                // not one of them.
                //
                // 54.7 — AND THE PREFIX ARITHMETIC IS GONE WITH IT. This
                // built the chart's opening bytes and tested `starts_with`,
                // which is a stratum read written by hand and one layout
                // change away from being wrong. A section CARRIES its
                // chart; the test is that the charts are equal.
                let gone: Vec<crate::network::nerve::coord::Section> = acts
                    .entries()
                    .filter(|(k, _)| k.chart == *chart)
                    .map(|(k, _)| k)
                    .collect();
                if gone.is_empty() {
                    return None;
                }
                for k in gone {
                    acts.drop_at(&k)?;
                }
                let _ = &ck;
                let last = acts.is_empty();
                if last {
                    self.vertices[i as usize] = Vertex::Demand {
                        kind: crate::transducer::sentence::Demand::Creation,
                        chart: chart.clone(),
                        bodies: remainder.to_vec(),
                    };
                    self.faces.retain(|f| f.i != i && f.j != i);
                } else {
                    for f in &mut self.faces {
                        let side_i = f.i == i;
                        if !side_i && f.j != i {
                            continue;
                        }
                        let stale: Vec<Vec<u8>> = f
                            .at
                            .scoped()
                            .filter(|(pk, _)| {
                                crate::transducer::sentence::read_face_pair_key(pk).is_some_and(|(ik, jk)| {
                                    if side_i { ik == ck.as_bytes() } else { jk == ck.as_bytes() }
                                })
                            })
                            .map(|(pk, _)| pk)
                            .collect();
                        for pk in stale {
                            f.at.remove(&pk);
                        }
                    }
                    self.faces.retain(|f| !(f.i == i || f.j == i) || !f.at.is_empty());
                    let kind = crate::transducer::sentence::Demand::Creation;
                    match self.demand_index(&kind, chart) {
                        Some(d) => {
                            if let Some(Vertex::Demand { bodies, .. }) = self.vertices.get_mut(d) {
                                for c in remainder {
                                    if !bodies.contains(c) {
                                        bodies.push(*c);
                                    }
                                }
                            }
                        }
                        None => {
                            self.vertices.push(Vertex::Demand { kind, chart: chart.clone(), bodies: remainder.to_vec() });
                        }
                    }
                }
                self.rewalk();
                Some(Contracted::Moved)
            }
            (RefusalKind::EdgeHolonomy, At::Face(a, b)) => {
                let i = self.word_index(a)?;
                let j = self.word_index(b)?;
                let mut moved = false;
                for f in &mut self.faces {
                    // EVERY MEMBER PAIR of the face becomes a remainder edge.
                    // The refusal is of the face's edge holonomy, and a face
                    // relating several pairs of bodies carries an edge at each
                    // — clearing one and leaving the rest would contract the
                    // refusal at one pair and assert the others (D10).
                    if ((f.i == i && f.j == j) || (f.i == j && f.j == i)) && f.has_edge() {
                        f.clear_edges();
                        moved = true;
                    }
                }
                if !moved {
                    return None;
                }
                self.rewalk();
                Some(Contracted::Moved)
            }
            // A DEMAND DECLARES SILENCE, NOT A FALSE PROJECTION (54.6).
            //
            // `undeclared_remainder` keeps a link where `spoken != 0`
            // whatever a demand says of it, so mounting `Demand::Source`
            // over a cell the form spoke wrongly cannot cure the refusal —
            // and the contraction MOVES THE ADDRESS, so the seal on the old
            // one no longer matches and the same guaranteed refusal returns
            // at a fresh coordinate. That is a mint per cycle with nothing
            // in the loop to stop it.
            //
            // The court partitions before it sends, so a link arriving
            // under this kind should already be silent; this filters again
            // because the guard belongs where the demand is mounted, not
            // only where the coordinate was packed (a ruling applies at
            // every site). A coordinate with nothing declarable in it
            // contracts to NOTHING — `None`, which `receive_adjudication`
            // reads as `Quiescent`: the form is held inert and no address
            // is minted.
            (RefusalKind::RemainderUndeclared, At::Chart(inv, links)) => {
                let links: Vec<Link> =
                    links.iter().copied().filter(|&(_, _, spoken, _)| spoken == 0).collect();
                let links = &links;
                if links.is_empty() {
                    return None;
                }
                let multiplicity = crate::transducer::sentence::pieces(links).len() as u32;
                let kind = crate::transducer::sentence::Demand::Source { multiplicity };
                match self.demand_index(&kind, inv) {
                    Some(d) => {
                        if let Some(Vertex::Demand { bodies, .. }) = self.vertices.get_mut(d) {
                            for c in links {
                                if !bodies.contains(c) {
                                    bodies.push(*c);
                                }
                            }
                        }
                    }
                    None => {
                        self.vertices.push(Vertex::Demand { kind, chart: inv.clone(), bodies: links.clone() });
                    }
                }
                self.rewalk();
                Some(Contracted::Moved)
            }
            // 50.9 move 5: the contact word the court's family does not
            // orbit becomes a demand over its chart exactly as move 1 makes
            // it; the coordinate is the same shape and so is the move.
            (RefusalKind::OrbitStill, at @ At::Vertex { .. }) => {
                self.contract(RefusalKind::VertexBody, at, remainder)
            }
            // 54.6 — A WRONG REMAINDER CONTRACTS TO NOTHING. There is no
            // move that makes a false assertion true: the form said
            // something at a cell and the family says otherwise, and no
            // demand declares that. `None` here is `Quiescent` at the
            // voice — the form is held until this mind's language moves —
            // and the address does not move, so the seal keeps holding.
            (RefusalKind::RemainderWrong, _) => None,
            // 50.9 move 6: the face that does not re-derive from its
            // vertices leaves the form, and the two vertices stand without
            // it; an orbit through it dissolves with the re-walk.
            (RefusalKind::FaceDerivation, At::Face(a, b)) => {
                let i = self.word_index(a)?;
                let j = self.word_index(b)?;
                let before = self.faces.len();
                self.faces.retain(|f| !((f.i == i && f.j == j) || (f.i == j && f.j == i)));
                if self.faces.len() == before {
                    return None;
                }
                self.rewalk();
                Some(Contracted::Moved)
            }
            (RefusalKind::MalformedComplex, _) => Some(Contracted::Quiescent),
            _ => None,
        }
    }

    /// Orbits and indices re-derived from the faces that remain.
    fn rewalk(&mut self) {
        self.orbits = Form::orbits_of(&self.faces, &self.vertices);
        self.reindex();
    }

    /// A vertex that acts: a word, a glued form, a rebase or an unfolding.
    /// A form with none is a chain wearing a form's tape (50.6).
    pub fn acts(&self) -> bool {
        self.vertices.iter().any(|v| {
            matches!(v, Vertex::Word { .. } | Vertex::Glued { .. } | Vertex::Rebased { .. } | Vertex::Unfolded { .. })
        })
    }

    /// 50.9, as the court reads it: is this form the contraction of a
    /// refused sentence at `at` — the vertex now a demand on its chart, the
    /// face now a remainder edge, the links now declared. The court that
    /// issued the receipt checks this before it replays the form as a
    /// sentence. A malformed complex has no contraction: nothing cites it.
    pub fn is_contraction_at(&self, kind: RefusalKind, at: &At) -> bool {
        match (kind, at) {
            (RefusalKind::VertexBody, At::Vertex { address, chart }) => {
                let acts_there = self
                    .word_index(address)
                    .and_then(|i| self.vertices.get(i as usize))
                    .is_some_and(|v| v.acting_on(chart).is_some());
                !acts_there && self.demand_vertices().any(|(_, _, c, _)| c == chart)
            }
            (RefusalKind::EdgeHolonomy, At::Face(a, b)) => {
                let (Some(i), Some(j)) = (self.word_index(a), self.word_index(b)) else {
                    // A vertex the face reached has since become a demand:
                    // the face is gone with it, which is more than a
                    // remainder edge.
                    return true;
                };
                self.faces
                    .iter()
                    .filter(|f| (f.i == i && f.j == j) || (f.i == j && f.j == i))
                    .all(|f| !f.has_edge())
            }
            (RefusalKind::RemainderUndeclared, At::Chart(_, links)) => {
                let mut declared: Keyed<Vec<u8>> = Keyed::new();
                for (_, _, _, bodies) in self.demand_vertices() {
                    for &(r, c, _, after) in bodies {
                        let k = site_key(r, c);
                        match declared.get_mut(&k) {
                            Some(hues) => hues.push(after),
                            None => {
                                declared.insert(k, vec![after]);
                            }
                        }
                    }
                }
                links.iter().all(|&(r, c, _, observed)| {
                    declared.get(&site_key(r, c)).is_some_and(|hues| hues.contains(&observed))
                })
            }
            (RefusalKind::OrbitStill, at @ At::Vertex { .. }) => {
                self.is_contraction_at(RefusalKind::VertexBody, at)
            }
            (RefusalKind::FaceDerivation, At::Face(a, b)) => {
                let (Some(i), Some(j)) = (self.word_index(a), self.word_index(b)) else {
                    // A vertex the face reached has since become a demand:
                    // the face is gone with it.
                    return true;
                };
                !self.faces.iter().any(|f| (f.i == i && f.j == j) || (f.i == j && f.j == i))
            }
            _ => false,
        }
    }
}

/// The key of a site of one section: its two coordinates written out.
fn site_key(r: i64, c: i64) -> Vec<u8> {
    let mut k = Vec::with_capacity(16);
    k.extend_from_slice(&r.to_le_bytes());
    k.extend_from_slice(&c.to_le_bytes());
    k
}
