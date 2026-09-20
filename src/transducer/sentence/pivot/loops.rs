// 12.2, 18.2 — THE LOOPS of a form, and the faces a pivot writes.

use super::*;

impl Form {
    /// THE CYCLES OF THE 1-SKELETON, less the ones that enclose nothing.
    ///
    /// Takes the VERTICES and not a count: `loop_is_trivial` reads the
    /// edge holonomies to decide, and `edge_holonomies_on` needs them. A
    /// loop flat on its WORD can carry a real edge composite — that is
    /// what `a_loop_flat_on_the_word_files_under_its_edge_composite`
    /// asserts — and a predicate blind to edges would prune the
    /// obstruction along with the noise.
    pub fn loops_of(faces: &[FormFace], vertices: &[Vertex]) -> Vec<Loop> {
        // THE CYCLE BASIS IS BUILT ON THE MEMBER COMPLEX, NOT ON THE VERTEX
        // INDICES (54.4, as ruled 2026-09-20).
        //
        // There are two complexes here and the engine was computing the
        // homotopy of one and the transport of the other.
        //
        //   V — nodes are `vertices`, edges are `(f.i, f.j)`. This is what
        //   this function walked: `adj` indexed by vertex, union-find over
        //   vertex indices.
        //
        //   M — nodes are MEMBERS, `member_key(vertex, chart)`: a vertex
        //   ACTING ON A CHART. `extract.rs` builds the faces over exactly
        //   this — `star_and_site_faces` walks `scenery.supports`, whose
        //   nodes are member keys — and every face stores its one member
        //   pair under `face_pair_key(&ci, &cj)`. All the transport lives
        //   here: the carriage, the gauge, and 50.3's edge.
        //
        // `π: M → V` forgets the chart, and it is NOT a fibration. A cycle
        // of V lifts to M only where, at every vertex it passes, the
        // incoming face's far chart is the outgoing face's near chart —
        // and nothing arranges that, because the faces were built
        // independently over member supports. So `paths_carried` was handed
        // cycles that had no lift, and reported "no lift" and "no holonomy"
        // as the same empty answer.
        //
        // MEASURED, before the change, on three voices of a live fleet:
        // 281 real loops (crossing a face between two bodies rather than
        // the Frame), every face of every one of them carrying a member
        // pair — 0 tears — and 0 closed member paths. 96 of 96, 97 of 97,
        // 88 of 88 passed through a vertex that splits into several
        // members, which is where the lift has to break. And exactly
        // `members − vertices` of the loops ran through the Frame alone —
        // 285 = 860 − 575, 266 = 702 − 436, 300 = 769 − 469, exact on all
        // three — because `k` charts on one vertex are `k` parallel
        // Frame-edges in V and `k − 1` spurious cycles that enclose
        // nothing.
        //
        // Built on M, every cycle lifts BY CONSTRUCTION: the arrival chart
        // at each node is the departure chart, because the node IS the
        // chart. `paths_carried`'s frontier then has exactly one survivor
        // and it closes.
        //
        // A FACE THAT RELATES NO MEMBER PAIR JOINS ITS VERTICES AT NO
        // NAMED CHART, and that is a node of M like any other — written
        // `member_key(v, &[])`, the empty chart key.
        //
        // This is not a sentinel standing for a chart; it is the ABSENCE
        // of one, written out, and it is a real sector of the complex. A
        // holonomy that moves the sensor plane places no member pair —
        // `face_place_by` derives each pair's carriage through
        // `versor_of_word`, one plane by construction (51.2) — so every
        // off-plane loop lives here, and `Rank.offplane` is a lane that
        // could never be non-zero if these faces were dropped
        // (`rank_offplane_is_its_own_lane`).
        //
        // The unnamed node is DISTINCT from every named one, so the two
        // sectors do not chain: a path carrying a body cannot step across
        // a face that places none, which is the truth about that face and
        // not a refusal invented here.
        if vertices.is_empty() {
            return Vec::new();
        }
        // THE NODES, discovered from the faces' own member pairs. `at` is
        // keyed by `face_pair_key`, so the two member keys of an edge are
        // read out of the key the transport is filed under — the same
        // coordinate, not a second derivation of it (law 12).
        let mut node_of: Keyed<u16> = Keyed::new();
        let mut node_vertex: Vec<u16> = Vec::new();
        let mut edges: Vec<(usize, usize, usize)> = Vec::new();
        let node = |node_of: &mut Keyed<u16>,
                        node_vertex: &mut Vec<u16>,
                        v: u16,
                        chart: &[u8]|
         -> usize {
            let k = crate::transducer::family::member_key(v, chart);
            match node_of.get(&k) {
                Some(&ix) => ix as usize,
                None => {
                    let ix = node_vertex.len();
                    node_of.insert(&k, ix as u16);
                    node_vertex.push(v);
                    ix
                }
            }
        };
        // STAR FACES FIRST, as before, so the spanning tree prefers the
        // basepoint's edges and a chord is a face between two bodies.
        let star = |f: &FormFace| {
            [f.i, f.j].iter().any(|&v| matches!(vertices.get(v as usize), Some(Vertex::Frame)))
        };
        for pass in [true, false] {
            for (fi, f) in faces.iter().enumerate() {
                if star(f) != pass {
                    continue;
                }
                if f.i as usize >= vertices.len() || f.j as usize >= vertices.len() {
                    continue;
                }
                let mut named = 0usize;
                for (pk, _) in f.at.scoped() {
                    let Some((ci, cj)) = crate::transducer::sentence::read_face_pair_key(&pk)
                    else {
                        continue;
                    };
                    named += 1;
                    let a = node(&mut node_of, &mut node_vertex, f.i, &ci);
                    let b = node(&mut node_of, &mut node_vertex, f.j, &cj);
                    edges.push((a, b, fi));
                }
                if named == 0 {
                    let a = node(&mut node_of, &mut node_vertex, f.i, &[]);
                    let b = node(&mut node_of, &mut node_vertex, f.j, &[]);
                    edges.push((a, b, fi));
                }
            }
        }
        let n = node_vertex.len();
        if n == 0 {
            return Vec::new();
        }
        let mut parent: Vec<usize> = (0..n).collect();
        let find = |parent: &mut [usize], mut x: usize| -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        };
        let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
        let mut loops = Vec::new();
        for (a, b, fi) in edges {
            let (pa, pb) = (find(&mut parent, a), find(&mut parent, b));
            if pa != pb {
                parent[pa] = pb;
                adj[a].push((b, fi));
                adj[b].push((a, fi));
                continue;
            }
            let Some((path, mut walked)) = tree_path(&adj, a, b) else {
                continue;
            };
            // THE CYCLE PROJECTED TO ITS VERTICES. `Loop` names its nodes
            // by vertex index because every reader of it — the holonomy
            // composite, the charts of a loop, the face-direction test in
            // `paths_carried` — asks the form for a vertex. The CHART at
            // each step is carried by the faces the cycle walks, which is
            // where the transport is; the projection loses nothing the
            // readers need, and the cycle is a cycle of M whatever it is
            // named by.
            let mut path: Vec<usize> =
                path.into_iter().map(|ix| node_vertex[ix] as usize).collect();
            path.push(node_vertex[a] as usize);
            walked.push(fi);
            let holonomy = form_named_holonomy(faces, &path, &walked);
            // THE CARRIAGES ARE THE TRANSPORT COMPOSITE'S, read from the one
            // walk that computes them. `path_carriages` stood here and walked
            // the loop's member paths accumulating `place.carriage` —
            // step for step the same walk `loop_transport_on` makes, with the
            // word and the gauge thrown away. Two carriers for one quantity
            // (law 12), and the expensive half: the walk is combinatorial over
            // each face's member pairs, and it ran once per loop at
            // construction and again per loop at every class reading. On the
            // fleet at e98e8a2 that is 70311 loops walked twice, and one pair
            // form carried 3361 of them.
            let mut lp = Loop {
                vertices: path.into_iter().map(|x| x as u16).collect(),
                faces: walked.into_iter().map(|x| x as u16).collect(),
                chord: fi as u16,
                holonomy,
                carriages: Keyed::new(),
            };
            // A CHORD THAT ENCLOSES NOTHING IS NOT A LOOP (ruled
            // 2026-09-18). `loop_is_trivial` states the predicate once;
            // a cycle that encloses no rotation, no placement and no hue
            // occupies no axis of 12.6, drives no pivot, and was walked by
            // every reader of `loops` for nothing — 16 546 of them on 174
            // forms in ten minutes.
            // ONE WALK, read twice. `loop_is_trivial` and the carriages
            // are two readings of the same frontier, and calling the two
            // readers would walk it again for each.
            let Some(carried) = paths_carried(faces, vertices, &lp) else {
                continue;
            };
            if loop_is_trivial(faces, vertices, &lp, &carried) {
                continue;
            }
            for (k, c) in carried.scoped() {
                lp.carriages.insert(&k, c.face.carriage.clone());
            }
            loops.push(lp);
        }
        loops
    }
}

// `path_carriages(faces, path, walked)` stood here: a walk of the loop's
// member paths accumulating each face's `place.carriage`, keyed by the path.
// It is `loop_transport_on` with the word and the gauge discarded — the same
// frontier, the same `step_charts`, the same `closed_key`, the same
// `carriage.geo(&acc)` — so one quantity had two carriers (law 12) and the
// walk that produces it ran twice per loop: once at `loops_of` to fill
// `Loop.carriages`, and again at every class reading through
// `loop_transport_on`. `loops_of` fills the field from that one walk now.
//
// The doc it carried is true of `loop_transport_on` and belongs to it: the
// paths are walked FROM THE FACES and not taken as a product of the vertices'
// fibres, because each face's `at` keys are its member pairs; a step extends a
// partial path only where the face relates the chart the path stands at to the
// chart it steps to; a path the next face does not relate simply ends and is
// not a cycle of this complex; and a path is kept only where it ends at the
// chart it began at.

/// A MEMBER PATH BEING WALKED: the charts it has passed, the chart it stands
/// AT, the chart it started at, and what it has accumulated.
///
/// `at` and `first` are carried rather than read back off `charts` — a walk's
/// own frontier is not a member of a collection to be taken by position (rule
/// 15), and reading `charts.last()` for it is that read wearing a different
/// name.
struct PartialPath<T> {
    charts: Vec<Vec<u8>>,
    first: Option<Vec<u8>>,
    at: Option<Vec<u8>>,
    acc: T,
}

impl<T: Clone> PartialPath<T> {
    fn seed(acc: T) -> PartialPath<T> {
        PartialPath { charts: Vec::new(), first: None, at: None, acc }
    }

    /// The two charts a step over `face_pair_key` relates, oriented by the way
    /// the face runs, or `None` where the path does not stand at the near one.
    fn step_charts(&self, k: &[u8], forward: bool) -> Option<(Vec<u8>, Vec<u8>)> {
        let (ik, jk) = crate::transducer::sentence::read_face_pair_key(k)?;
        let (here, there) = if forward { (ik, jk) } else { (jk, ik) };
        match &self.at {
            Some(at) if *at != here => None,
            _ => Some((here, there)),
        }
    }

    fn extend(&self, here: Vec<u8>, there: Vec<u8>, acc: T) -> PartialPath<T> {
        let mut charts = self.charts.clone();
        let first = match &self.first {
            Some(f) => Some(f.clone()),
            None => Some(here.clone()),
        };
        charts.push(here);
        PartialPath { charts, first, at: Some(there), acc }
    }

    /// The path's key where it CLOSES — it ends at the chart it began at, which
    /// is what makes it a cycle of this complex. `None` otherwise.
    fn closed_key(&self) -> Option<Vec<u8>> {
        match (&self.first, &self.at) {
            (Some(f), Some(a)) if f == a => Some(Loop::path_key(&self.charts)),
            _ => None,
        }
    }
}


impl Form {
    /// THE HOLONOMY OF THE EDGE TRANSPORTS around a loop (50.3), ONE PER
    /// MEMBER PATH: the product of the face edges along the loop's faces, in
    /// the base body's stabilizer or exchange; stabilizer cosets do not
    /// telescope, which is what makes it a reading.
    ///
    /// A path carrying a remainder edge anywhere on it is not evaluated and is
    /// absent from the result (13.9); an empty result is "no path of this loop
    /// is evaluated", which is what `None` used to say for the whole loop.
    /// `None` itself is now only a loop naming a face this form does not have.
    pub fn edge_holonomies(&self, lp: &Loop) -> Option<Keyed<Transport>> {
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
/// read in ONE walk of the frontier.
///
/// `loop_transport_on` and `edge_holonomies_on` walked the same frontier
/// twice — same `live`, same branch on `f.at`, same `step_charts`, same
/// `extend`, same `closed_key` — accumulating different fields. This file
/// already records that correction being made once, for `path_carriages`:
/// "the same frontier, the same `step_charts`, the same `closed_key` — so
/// one quantity had two carriers (law 12) and the walk that produces it ran
/// twice per loop." It was never applied to this pair, which is the same
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
}

/// THE ONE WALK. Every member path of the loop, with both composites.
///
/// A star face is the trivialization every body is read against (2.2): it
/// contributes the identity to the edge composite and the path passes
/// through it. A path that crosses no real face measured no edge at all and
/// carries `None` there — answering "identity" would report a measurement
/// never taken.
pub(crate) fn paths_carried(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Loop,
) -> Option<Keyed<PathCarried>> {
    let crosses_a_real_face = lp
        .faces
        .iter()
        .any(|&fi| faces.get(fi as usize).is_some_and(|f| !touches_frame(vertices, f)));
    let seed = PathCarried {
        face: Transport {
            word: Word::identity(),
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        },
        edge: match crosses_a_real_face {
            true => Some(Transport {
                word: Word::identity(),
                carriage: element::identity_versor(),
                gauge: Vec::new(),
            }),
            false => None,
        },
    };
    let mut live = vec![PartialPath::seed(seed)];
    for (win, &fi) in lp.vertices.windows(2).zip(&lp.faces) {
        let f = faces.get(fi as usize)?;
        let forward = f.i == win[0] && f.j == win[1];
        let word_step = if forward { f.word.clone() } else { f.word.inverse() };
        let star = touches_frame(vertices, f);
        let mut next = Vec::new();
        for p in &live {
            // THE STEP IS A DESCENT, NOT A SCAN. `f.at` is keyed
            // `put_bytes(ci) ++ put_bytes(cj)`, so on a forward step the
            // members that continue this path are the fiber under
            // `put_bytes(path.at)` — a byte prefix, which is what the trie
            // is. The scan built every branch and `step_charts` threw away
            // the ones that did not continue; the frontier was the
            // Cartesian product of the faces' member pairs, bounded by
            // nothing (law 3, rule 10).
            //
            // A path that has not started yet stands at no chart and takes
            // the whole carrier — that is `f.at`'s own fiber (rule 1), not
            // a walk of a collection to find something.
            let members: Vec<(Vec<u8>, &FacePlace)> = match (&p.at, forward) {
                (Some(at), true) => {
                    let mut pre = Vec::new();
                    crate::transducer::address::put_bytes(&mut pre, at);
                    match f.at.fiber_at(&pre) {
                        Some(fib) => fib.collect(),
                        None => Vec::new(),
                    }
                }
                _ => f.at.scoped().collect(),
            };
            for (k, place) in members {
                let Some((here, there)) = p.step_charts(&k, forward) else {
                    continue;
                };
                let (carriage, gauge) = if forward {
                    (place.carriage.clone(), place.gauge.clone())
                } else {
                    match place.carriage.versor_inverse() {
                        Some(inv) => (inv, gauge_invert(&place.gauge)),
                        None => continue,
                    }
                };
                let mut held = p.acc.clone();
                held.face.carriage = carriage.geo(&held.face.carriage);
                held.face.word = word_step.compose(&held.face.word);
                held.face.gauge = compose_gauge(&gauge, &held.face.gauge);
                // THE EDGE HALF, failing on its own. A star face composes
                // the identity into it; a real face with no edge, or an
                // edge that does not invert, leaves this path with no edge
                // reading and does not touch the face composite.
                if !star {
                    held.edge = match (held.edge.take(), place.edge.as_ref()) {
                        (Some(acc), Some(edge)) => {
                            let step = if forward {
                                Some(edge.clone())
                            } else {
                                edge.inverse()
                            };
                            step.map(|step| Transport {
                                carriage: step.carriage.geo(&acc.carriage),
                                word: step.word.compose(&acc.word),
                                gauge: compose_gauge(&step.gauge, &acc.gauge),
                            })
                        }
                        _ => None,
                    };
                }
                next.push(p.extend(here, there, held));
            }
        }
        live = next;
    }
    let mut out = Keyed::new();
    for p in live {
        if let Some(k) = p.closed_key() {
            let mut c = p.acc;
            c.face.word = c.face.word.normal_form();
            if let Some(e) = c.edge.as_mut() {
                e.word = e.word.normal_form();
            }
            out.insert(k, c);
        }
    }
    Some(out)
}

/// THE HOLONOMY OF THE EDGE TRANSPORTS around a loop (50.3), one per member
/// path: a reader of the one walk. A path with no edge reading is absent,
/// which is what "not evaluated" means (13.9).
pub(crate) fn edge_holonomies_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Loop,
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
    /// asking "what class did this loop file" gets an answer per path and not
    /// one path's answer for all of them.
    pub fn edge_classes(&self, lp: &Loop) -> Keyed<crate::transducer::address::Address> {
        let mut out = Keyed::new();
        let Some(hs) = self.edge_holonomies(lp) else {
            return out;
        };
        for (k, h) in hs.scoped() {
            if let Some(c) = h.word.rotor().class_key() {
                out.insert(&k, c);
            }
        }
        out
    }

    /// THE CLASS VECTOR OF A LOOP'S EDGE HOLONOMY (12.6 over 50.3): the
    /// four coordinates of the stabilizer composite, as `class_vector`
    /// gives them for the transport composite. A loop carries BOTH and
    /// neither replaces the other — an extraction's loop can hold a half
    /// turn on its transports while its edges compose to the identity, and
    /// a cover's loop is the other way round, since every word it composes
    /// is a sensor-plane word and those commute.
    ///
    /// This is the reading that can occupy `𝒮`: the edge's carriage
    /// composes `D(ρ)` with `T(t)`, which do not commute, so a triangle of
    /// placements is a genuine commutator where a triangle of words is not.
    ///
    /// `None` when the loop is not evaluated (a remainder edge anywhere on
    /// it) or the holonomy is not a versor.
    pub fn edge_class_vectors(&self, lp: &Loop) -> Keyed<ClassVector> {
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

    /// A loop's edge holonomy coheres when EVERY EVALUATED MEMBER PATH is the
    /// identity or is filed under its class at the period its order reads —
    /// `0` for infinite order, A19's spelling of it (50.1 step 2, A17, 52.3).
    /// A path that is not evaluated — a remainder edge on it — holds nothing
    /// against the form and is absent; a holonomy with no class files
    /// nowhere and does not cohere.
    pub(crate) fn edge_coheres(&self, lp: &Loop) -> bool {
        let Some(hs) = self.edge_holonomies(lp) else {
            return true;
        };
        hs.scoped().all(|(_, h)| {
            if h.word.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
                return true;
            }
            let Some(class) = h.word.rotor().class_key() else {
                return false;
            };
            let ell = h.word.order().to_u32();
            matches!(self.torsion.get(&class), Some(filed) if filed.period == ell)
        })
    }
}

pub(crate) fn loop_transport(form: &Form, lp: &Loop) -> Keyed<Transport> {
    loop_transport_on(&form.faces, &form.vertices, lp)
}

/// The face composite of every member path: a reader of the one walk.
///
/// It walked the frontier itself, beside `edge_holonomies_on` walking the
/// identical frontier for the other half. `vertices` is taken because the
/// star-face rule belongs to the walk, not to its readers.
pub(crate) fn loop_transport_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Loop,
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

impl Form {
    /// Apply a certified move (A20 `by`) to this form: a write onto the
    /// chord face's transport, or a chart axis opened over the loop that
    /// closes on the face (I6). Loops are re-walked by the caller.
    pub fn apply_move(&mut self, by: &PivotBy) {
        match by {
            PivotBy::OpenAxis { face, .. } => {
                let _ = self.open_axis_on(*face);
            }
            _ => apply_by(&mut self.faces, by),
        }
    }

    /// The inverse of [`Form::apply_move`]: the court's un-pivot (A20 step 3).
    pub fn unapply_move(&mut self, by: &PivotBy) {
        match by {
            PivotBy::OpenAxis { face, .. } => {
                let _ = self.close_axis_on(*face);
            }
            _ => unapply_by(&mut self.faces, by),
        }
    }
}

pub(crate) fn apply_by(faces: &mut [FormFace], by: &PivotBy) {
    match by {
        PivotBy::OpenAxis { .. } => {}
        // A WORD MOVE WRITES BOTH OF A FACE'S CARRIERS, because a loop's
        // holonomy is read from whichever one its path has. `holonomy_words_on`
        // answers the EDGE COMPOSITE where a member path carries one and the
        // face word where it does not, so a move that wrote `f.word` alone
        // left the class unmoved on every path with an edge — the class was
        // read from a carrier the move never touched.
        //
        // Measured: `Placement` and `Hue` moves discharge on the wire and
        // `Word` moves do not — `axis Orientation(0) no discharge`, every
        // occurrence, across every window. Those two write `p.carriage` and
        // `p.gauge`, which `loop_transport_on` composes and `ClassVector::read`
        // takes; the word arm wrote the one carrier the reading skips. This is
        // the same split R8 removed from `file_loop` and R12 from the
        // `OpenAxis` pipeline, at the site that makes the move.
        //
        // Both are written with the SAME composition, so a path reading either
        // carrier sees the same move, and `unapply_by` inverts both.
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.word = pivoted_face(&f.word, word, *side);
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(e) = f.at.get_mut(&k).and_then(|p| p.edge.as_mut()) {
                        e.word = pivoted_face(&e.word, word, *side);
                    }
                }
            }
        }
        // A PIVOT MOVES THE FACE AT EVERY MEMBER PAIR. The move is a write on
        // the FACE — a translator on its placement, a gauge on its hue map —
        // and a face relating several pairs of bodies is moved at each of them
        // or the pivot moves one pair and leaves the rest where they were.
        PivotBy::Placement { face, translator } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(p) = f.at.get_mut(&k) {
                        p.carriage = translator.geo(&p.carriage);
                    }
                }
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(p) = f.at.get_mut(&k) {
                        p.gauge = compose_gauge(&p.gauge, gauge);
                    }
                }
            }
        }
    }
}

pub(crate) fn unapply_by(faces: &mut [FormFace], by: &PivotBy) {
    match by {
        PivotBy::OpenAxis { .. } => {}
        // The mirror of `apply_by`'s word arm, carrier for carrier — or a
        // trial leaves the edge composite moved on a candidate it refused.
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.word = unpivoted_face(&f.word, word, *side);
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(e) = f.at.get_mut(&k).and_then(|p| p.edge.as_mut()) {
                        e.word = unpivoted_face(&e.word, word, *side);
                    }
                }
            }
        }
        PivotBy::Placement { face, translator } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                if let Some(inv) = translator.versor_inverse() {
                    let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                    for k in pairs {
                        if let Some(p) = f.at.get_mut(&k) {
                            p.carriage = inv.geo(&p.carriage);
                        }
                    }
                }
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                let back = gauge_invert(gauge);
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(p) = f.at.get_mut(&k) {
                        p.gauge = compose_gauge(&p.gauge, &back);
                    }
                }
            }
        }
    }
}

pub fn pivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(&w.inverse()).normal_form(),
        Side::Left => w.compose(g).normal_form(),
    }
}

/// The face word before that move: the inverse of [`pivoted_face`].
pub fn unpivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(w).normal_form(),
        Side::Left => w.inverse().compose(g).normal_form(),
    }
}

/// IS THIS CYCLE ALGEBRAICALLY TRIVIAL — identity word holonomy, and every
/// member path's transport exactly `(1, 0, ∅)`?
///
/// `Form::loops_of` is union-find over faces and returns `b₁` of a graph:
/// every redundant path is a cycle, and a form carried 16 546 of them in
/// ten minutes while 292 reached a classifier. A cycle that encloses
/// nothing occupies no axis of 12.6 and can drive no pivot.
///
/// THE FILTER IS HERE AND NOT IN `loops_of`. `Form.loops` carries two
/// readings: the cycle set of the 1-skeleton — a structural fact about
/// which faces closed, asserted by `the_1_skeleton_is_not_a_star_tree` and
/// `the_triangle_among_the_bodies_closes` — and the obstructions a pivot
/// discharges. Dropping trivial cycles at construction served the second
/// and destroyed the first: fifteen tests, two of them asserting incidence
/// structure through `loops.len()`.
///
/// It is also NOT the whole of `coheres`. A loop can carry an identity
/// WORD holonomy over edges that compose to a rotation, and `edge_coheres`
/// accepts that only where the class is filed in the form's `torsion`. A
/// cycle carrying any edge transport is not trivial by this reader and
/// reaches every reading that holds it to account.
pub(crate) fn loop_is_trivial(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Loop,
    carried: &Keyed<PathCarried>,
) -> bool {
    use crate::transducer::alphabet::Rotor;
    let flat = |t: &Transport| {
        t.word.rotor().projective_eq(&Rotor::identity())
            && element::same_versor(&t.carriage, &element::identity_versor())
            && t.gauge.is_empty()
    };
    // THE EDGE IS A THIRD CARRIER. `loop_transport_on` composes the
    // faces' words and the members' carriage and gauge;
    // `edge_holonomies_on` reads 50.3's edge transport, which
    // `class_vectors_on` and `edge_coheres` both turn on. A loop whose
    // word telescopes to the identity over edges that compose to a
    // rotation is an obstruction, and reading only the first two carriers
    // would prune it.
    // AN UNREAD LOOP IS NOT A FLAT ONE (rule 11). `loop_transport_on`
    // answers empty where no member path CLOSES, and `all(..)` on nothing
    // is true — so a loop whose paths do not join up read as trivial and
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
    // a loop of nothing but star faces crossed no edge, and its reading is
    // the faces' WORDS, already taken above; a loop that DOES cross a real
    // face and still answers empty had every path dropped on a remainder
    // edge, which is unread, not flat (rule 11). `None` is a loop naming a
    // face this form does not have, and that is unread too.
    let crosses_a_real_face = lp
        .faces
        .iter()
        .any(|&fi| faces.get(fi as usize).is_some_and(|f| !touches_frame(vertices, f)));
    let mut any_edge = false;
    for (_, c) in carried.scoped() {
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
