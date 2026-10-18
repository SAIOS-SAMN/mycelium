// 50.9 — THE SIX CONTRACTION MOVES: what a refusal does to the form that
// was refused, each move a strict shrinking of the closed sub-complex.
//
//! A refused vertex becomes a demand and takes its faces with it; a refused
//! edge becomes a remainder edge; undeclared links are declared as a Source
//! demand; a contact word that does not orbit becomes a demand as a refused
//! vertex does; a face that does not re-derive leaves the form; a malformed
//! complex is not moved at all — the mind is quiescent on it. Every move
//! re-reads the orbits and reindexes; nothing here trusts the tape's own orbit
//! list, and every read is a read of this form's own vertices and faces
//! (rule 1). AT1 — every one of them by the vertices' and overlaps'
//! COORDINATES, which is what a refusal names.

use super::*;

/// What a contraction did: the form moved and its closed sub-complex
/// shrank, or the form is a malformed complex and stays as it is, inert.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Contracted {
    Moved,
    Quiescent,
}

impl Form {
    /// STAND A DEMAND'S BODIES under its (kind, chart), one vertex per pair
    /// (50.5): a vertex already standing for that kind on that chart takes
    /// the bodies, and its coordinates — the chain's measure and pieces —
    /// move with them, so the vertex is re-filed at its new key.
    pub(crate) fn file_demand(&mut self, kind: crate::transducer::sentence::Demand, chart: &stalk::Invariant, links: &[Link]) {
        let held = self
            .demand_vertices()
            .find(|(_, k, c, _)| **k == kind && *c == chart)
            .map(|(at, _, _, bodies)| (at, bodies.to_vec()));
        let bodies = match held {
            Some((at, mut bodies)) => {
                self.vertices.drop_at(&at);
                for c in links {
                    if !bodies.contains(c) {
                        bodies.push(*c);
                    }
                }
                bodies
            }
            None => links.to_vec(),
        };
        let _ = self.put_vertex(Vertex::demand(kind, chart.clone(), bodies));
    }

    /// Execute the move a refusal names at its coordinate (50.9). `None`
    /// when the coordinate names nothing this form holds — a vertex it does
    /// not hold, a face between vertices it does not hold — so a certificate
    /// for it could never re-derive. `remainder` is what the contracted form
    /// leaves on the speaker's family, the bodies a refused word's demand
    /// stands for.
    pub fn contract(&mut self, kind: RefusalKind, at: &At, remainder: &[Link]) -> Option<Contracted> {
        match (kind, at) {
            (RefusalKind::VertexBody, At::Vertex { vertex, chart }) => {
                let Some(Vertex::Word { acts, .. }) = self.vertices.at_mut(vertex) else {
                    return None;
                };
                // THE MEMBER AT THE REFUSED CHART leaves the vertex (D10 item
                // 13), every section over the chart with it (54.7 R1): the
                // refusal is about the CHART. A vertex whose last member left
                // becomes the demand itself, and a demand enters no face
                // (50.5): the overlaps that reached it leave with it. A vertex
                // that keeps other members keeps the overlaps at those
                // members and loses the ones at the refused chart.
                let gone: Vec<coord::Section> = acts.entries().filter(|(k, _)| k.chart == *chart).map(|(k, _)| k).collect();
                if gone.is_empty() {
                    return None;
                }
                for k in gone {
                    acts.drop_at(&k)?;
                }
                let last = acts.is_empty();
                let touching: Vec<coord::Overlap> = self
                    .overlaps_at(vertex)
                    .into_iter()
                    .filter(|o| {
                        last || [&o.from, &o.to]
                            .iter()
                            .any(|c| c.vertex() == **vertex && c.invariant() == Some(chart))
                    })
                    .collect();
                for o in touching {
                    self.faces.drop_at(&o);
                }
                if last {
                    self.vertices.drop_at(vertex);
                }
                self.file_demand(crate::transducer::sentence::Demand::Creation, chart, remainder);
                self.reread();
                Some(Contracted::Moved)
            }
            (RefusalKind::EdgeHolonomy, At::Face(a, b)) => {
                let mut moved = false;
                // EVERY OVERLAP between the two vertices, either way, becomes a
                // remainder edge: the refusal is of the face's edge holonomy,
                // and a face relating several pairs of bodies carries an edge
                // at each (D10).
                for o in self.overlaps_between_vertices(a, b) {
                    if let Some(f) = self.faces.at_mut(&o) {
                        if f.has_edge() {
                            f.clear_edges();
                            moved = true;
                        }
                    }
                }
                if !moved {
                    return None;
                }
                self.reread();
                Some(Contracted::Moved)
            }
            // A DEMAND DECLARES SILENCE, NOT A FALSE PROJECTION (54.6).
            //
            // `undeclared_remainder` keeps a link where `spoken != 0`
            // whatever a demand says of it, so mounting `Demand::Source`
            // over a cell the form spoke wrongly cannot cure the refusal —
            // and the contraction MOVES THE ADDRESS, so the same guaranteed
            // refusal returns at a fresh coordinate. The court partitions
            // before it sends; this filters again because the guard belongs
            // where the demand is mounted. A coordinate with nothing
            // declarable in it contracts to NOTHING.
            (RefusalKind::RemainderUndeclared, At::Chart(inv, links)) => {
                let links: Vec<Link> = links.iter().copied().filter(|&(_, _, spoken, _)| spoken == crate::arc::ground()).collect();
                if links.is_empty() {
                    return None;
                }
                let multiplicity = crate::transducer::sentence::pieces(&links).len();
                self.file_demand(crate::transducer::sentence::Demand::Source { multiplicity }, inv, &links);
                self.reread();
                Some(Contracted::Moved)
            }
            // 50.9 move 5: the contact word the court's family does not
            // orbit becomes a demand over its chart exactly as move 1 makes
            // it; the coordinate is the same shape and so is the move.
            (RefusalKind::OrbitStill, at @ At::Vertex { .. }) => self.contract(RefusalKind::VertexBody, at, remainder),
            // 54.6 — A WRONG REMAINDER CONTRACTS TO NOTHING. There is no
            // move that makes a false assertion true.
            (RefusalKind::RemainderWrong, _) => None,
            // 50.9 move 6: the face that does not re-derive from its
            // vertices leaves the form — every overlap between the two — and
            // an orbit through it dissolves with the re-read.
            (RefusalKind::FaceDerivation, At::Face(a, b)) => {
                let gone = self.overlaps_between_vertices(a, b);
                if gone.is_empty() {
                    return None;
                }
                for o in gone {
                    self.faces.drop_at(&o);
                }
                self.reread();
                Some(Contracted::Moved)
            }
            (RefusalKind::MalformedComplex, _) => Some(Contracted::Quiescent),
            _ => None,
        }
    }

    /// EVERY OVERLAP BETWEEN TWO VERTICES, either way: the fibre leaving
    /// each, read at the other end.
    pub(crate) fn overlaps_between_vertices(&self, a: &coord::VertexAt, b: &coord::VertexAt) -> Vec<coord::Overlap> {
        let mut out: Vec<coord::Overlap> = self.overlaps_from(a).into_iter().filter(|o| &o.to_vertex() == b).collect();
        if a != b {
            out.extend(self.overlaps_from(b).into_iter().filter(|o| &o.to_vertex() == a));
        }
        out
    }

    /// Orbits and fibres re-derived from the faces that remain.
    fn reread(&mut self) {
        self.orbits = Form::orbits_of(&self.faces, &self.vertices);
        self.reindex();
    }

    /// A vertex that acts: a word, a glued form, a rebase or an unfolding.
    /// A form with none is a chain wearing a form's tape (50.6). A clause
    /// acts on what it selects (78.143).
    pub fn acts(&self) -> bool {
        !self.clauses.is_empty() || self.vertices.scoped().any(|(_, v)| {
            matches!(v, Vertex::Word { .. } | Vertex::Glued { .. } | Vertex::Rebased { .. } | Vertex::Unfolded { .. })
        })
    }

    /// 50.9, as the court reads it: is this form the contraction of a
    /// refused sentence at `at` — the vertex now a demand on its chart, the
    /// face now a remainder edge, the links now declared. A malformed
    /// complex has no contraction: nothing cites it.
    pub fn is_contraction_at(&self, kind: RefusalKind, at: &At) -> bool {
        match (kind, at) {
            (RefusalKind::VertexBody, At::Vertex { vertex, chart }) => {
                let acts_there = self.vertices.at(vertex).is_some_and(|v| v.acting_on(chart).is_some());
                !acts_there && self.demand_vertices().any(|(_, _, c, _)| c == chart)
            }
            (RefusalKind::EdgeHolonomy, At::Face(a, b)) => {
                // A vertex the face reached has since become a demand: the
                // face is gone with it, which is more than a remainder edge.
                self.overlaps_between_vertices(a, b)
                    .iter()
                    .all(|o| self.faces.at(o).is_none_or(|f| !f.has_edge()))
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
                links
                    .iter()
                    .all(|&(r, c, _, observed)| declared.get(&site_key(r, c)).is_some_and(|hues| hues.contains(&observed)))
            }
            (RefusalKind::OrbitStill, at @ At::Vertex { .. }) => self.is_contraction_at(RefusalKind::VertexBody, at),
            (RefusalKind::FaceDerivation, At::Face(a, b)) => self.overlaps_between_vertices(a, b).is_empty(),
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
