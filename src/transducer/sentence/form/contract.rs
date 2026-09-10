// 50.9 — THE FOUR CONTRACTION MOVES: what a refusal does to the form that
// was refused, each move a strict shrinking of the closed sub-complex.
//
//! A refused vertex becomes a demand and takes its faces with it; a refused
//! edge becomes a remainder edge; undeclared cells are declared as a Source
//! demand; a malformed complex is not moved at all — the mind is quiescent
//! on it. Every move re-walks the loops and reindexes; nothing here trusts
//! the tape's own loop list, and every read is a walk of this form's own
//! vertices and faces (rule 1).

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
    pub fn contract(&mut self, kind: RefusalKind, at: &At, remainder: &[Cell]) -> Option<Contracted> {
        match (kind, at) {
            (RefusalKind::VertexBody, At::Vertex { address, chart }) => {
                let i = self.word_index(address)?;
                // The word becomes a demand over its chart, carrying as its
                // bodies the cells it was to reach. A demand vertex enters
                // no face (50.5): the faces that reached the word leave
                // with it.
                self.vertices[i as usize] = Vertex::Demand {
                    kind: crate::transducer::sentence::Demand::Creation,
                    chart: chart.clone(),
                    bodies: remainder.to_vec(),
                };
                self.faces.retain(|f| f.i != i && f.j != i);
                self.rewalk();
                Some(Contracted::Moved)
            }
            (RefusalKind::EdgeHolonomy, At::Face(a, b)) => {
                let i = self.word_index(a)?;
                let j = self.word_index(b)?;
                let mut moved = false;
                for f in &mut self.faces {
                    if ((f.i == i && f.j == j) || (f.i == j && f.j == i)) && f.edge.is_some() {
                        f.edge = None;
                        moved = true;
                    }
                }
                if !moved {
                    return None;
                }
                self.rewalk();
                Some(Contracted::Moved)
            }
            (RefusalKind::RemainderUndeclared, At::Chart(inv, cells)) => {
                if cells.is_empty() {
                    return None;
                }
                let multiplicity = crate::transducer::sentence::pieces(cells).len() as u32;
                let kind = crate::transducer::sentence::Demand::Source { multiplicity };
                match self.demand_index(&kind, inv) {
                    Some(d) => {
                        if let Some(Vertex::Demand { bodies, .. }) = self.vertices.get_mut(d) {
                            for c in cells {
                                if !bodies.contains(c) {
                                    bodies.push(*c);
                                }
                            }
                        }
                    }
                    None => {
                        self.vertices.push(Vertex::Demand { kind, chart: inv.clone(), bodies: cells.clone() });
                    }
                }
                self.rewalk();
                Some(Contracted::Moved)
            }
            (RefusalKind::MalformedComplex, _) => Some(Contracted::Quiescent),
            _ => None,
        }
    }

    /// Loops and indices re-derived from the faces that remain.
    fn rewalk(&mut self) {
        self.loops = Form::loops_of(&self.faces, self.vertices.len());
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
    /// face now a remainder edge, the cells now declared. The court that
    /// issued the receipt checks this before it replays the form as a
    /// sentence. A malformed complex has no contraction: nothing cites it.
    pub fn is_contraction_at(&self, kind: RefusalKind, at: &At) -> bool {
        match (kind, at) {
            (RefusalKind::VertexBody, At::Vertex { address, chart }) => {
                self.word_index(address).is_none() && self.demand_vertices().any(|(_, _, c, _)| c == chart)
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
                    .all(|f| f.edge.is_none())
            }
            (RefusalKind::RemainderUndeclared, At::Chart(_, cells)) => {
                let mut declared: Keyed<Vec<u8>> = Keyed::new();
                for (_, _, _, bodies) in self.demand_vertices() {
                    for &(r, c, _, after) in bodies {
                        let k = cell_pos_key(r, c);
                        match declared.get_mut(&k) {
                            Some(hues) => hues.push(after),
                            None => {
                                declared.insert(k, vec![after]);
                            }
                        }
                    }
                }
                cells.iter().all(|&(r, c, _, observed)| {
                    declared.get(&cell_pos_key(r, c)).is_some_and(|hues| hues.contains(&observed))
                })
            }
            _ => false,
        }
    }
}

fn cell_pos_key(r: i64, c: i64) -> Vec<u8> {
    let mut k = Vec::with_capacity(16);
    k.extend_from_slice(&r.to_le_bytes());
    k.extend_from_slice(&c.to_le_bytes());
    k
}
