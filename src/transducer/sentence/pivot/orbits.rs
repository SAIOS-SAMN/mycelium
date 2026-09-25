// 12.2, 18.2 — THE ORBITS of a form: the cycle basis on the member complex,
// each cycle based by Axiom A, and the gates it passes to become an `Orbit`.

use super::*;

thread_local! {
    /// `(reflexive cells seen, dropped at tree_path, dropped at
    /// paths_carried, dropped as trivial)` — the three gates between a
    /// cycle of the member complex and an `Orbit` on the form.
    static CYCLES: std::cell::Cell<(u64, u64, u64, u64)> =
        const { std::cell::Cell::new((0, 0, 0, 0)) };
    /// H2c — the `unread` gate split by what refused it, and the cycles
    /// that were read but whose edge half was refused, split the same way.
    static UNREAD: std::cell::Cell<(UnreadTally, UnreadTally)> =
        std::cell::Cell::new((UnreadTally::default(), UnreadTally::default()));
}

/// Count one refusal on the `unread` gate, by what refused it (H2c).
fn unread(why: Unread) {
    gate(2);
    UNREAD.with(|c| {
        let mut t = c.get();
        t.0.note(why);
        c.set(t);
    });
}

/// AXIOM A — THE TRANSPORT FROM EACH COMPONENT'S BASEPOINT TO EVERY NODE
/// OF IT (54.7, ruled 2026-09-21).
///
/// ```text
///     θ_{x₀} = 1 ,     θ_c = θ_p · Ω_e   along each tree edge p → c
/// ```
///
/// A TRAVERSAL OF THE FOREST, NOT A SEARCH. The spanning forest is already
/// built; this traverses it once from each root, and every node is reached
/// exactly once because a tree has one path to each of its nodes — which
/// is also why `θ` is well defined and needs no choice (rule 1).
///
/// THE ROOT IS THE COMPONENT'S LEAST NODE, which is the order the nodes
/// were discovered in, which is the faces' own content order. So the
/// basepoint is intrinsic to the form and two minds reading one form pick
/// the same one; it is not the order edges happened to arrive in.
///
/// `b₀ > 1` GIVES ONE BASEPOINT PER COMPONENT. A node in no component of
/// the forest keeps the identity, and a chord spanning two components has
/// no transport between them — that is refused where it is read, never
/// synthesised here (rule 11).
///
/// A FOREST EDGE WHOSE TRANSPORT REFUSES LEAVES THE NODES BEYOND IT
/// UNGAUGED (57.4).
/// The refusal is counted on the `unread` gate, every node beyond it takes
/// no `θ`, and a cycle with an end there reads unread rather than flat: an
/// identity written for a transport that was never read would be a
/// refusal recorded as a value (rule 11). A node beyond it carries the
/// refusal's own reason, so a cycle ending there is counted by it (H2c).
fn tree_transports(faces: &Faces, forest: &Forest) -> (Vec<Result<element::Versor, Unread>>, Vec<usize>) {
    let n = forest.charts.len();
    let mut theta: Vec<Result<element::Versor, Unread>> = vec![Ok(element::identity_versor()); n];
    let mut root_of: Vec<usize> = (0..n).collect();
    let mut seen = vec![false; n];
    // index: the forest's own nodes, each visited once (rule 1).
    for root in 0..n {
        if seen[root] {
            continue;
        }
        seen[root] = true;
        let mut frontier = vec![root];
        while let Some(p) = frontier.pop() {
            for &(c, e) in &forest.adj[p] {
                if seen[c] {
                    continue;
                }
                seen[c] = true;
                root_of[c] = root;
                let step = tree_step(faces, forest, p, e);
                if let Err(why) = &step {
                    unread(*why);
                }
                theta[c] = match (&theta[p], &step) {
                    (Ok(held), Ok(step)) => Ok(held.geo(step)),
                    (Err(why), _) | (_, Err(why)) => Err(*why),
                };
                frontier.push(c);
            }
        }
    }
    (theta, root_of)
}

/// THE TRANSPORT ACROSS ONE TREE EDGE, leaving node `p`.
///
/// Read off the overlap's own place, exactly as `paths_carried` reads a
/// step — the same coordinate, not a second derivation of it (rule 12) —
/// and reversed where the edge is crossed against its orientation.
///
/// `Cone` where the carriage is no versor — a carriage on the light cone
/// (57.4), whose action collapses — and that transport was never read;
/// backward, the carriage is read by its reverse (78.32(1)). `NoFace` where
/// the forest names an overlap the form does not hold.
fn tree_step(faces: &Faces, forest: &Forest, p: usize, e: usize) -> Result<element::Versor, Unread> {
    let (from, _, o) = forest.edges.get(e).ok_or(Unread::NoFace)?;
    let c = &faces.at(o).ok_or(Unread::NoFace)?.place.carriage;
    match *from == p {
        true => Ok(c.clone()),
        false => crate::transducer::cga::is_versor(c).then(|| c.reverse()).ok_or(Unread::Cone),
    }
}

/// THE SPANNING FOREST OF THE CHART COMPLEX, on coordinates (AT1): a node
/// per chart, an edge per overlap. The ordinals are this reading's own and
/// leave it only as the charts and overlaps they stand for; (iii) retires
/// the forest.
struct Forest {
    charts: Vec<coord::ChartAt>,
    edges: Vec<(usize, usize, coord::Overlap)>,
    adj: Vec<Vec<(usize, usize)>>,
}

impl Forest {
    /// The steps a node path crosses, each overlap with the way it runs.
    fn steps(&self, path: &[usize], crossed: &[usize]) -> Vec<OrbitStep> {
        path.iter()
            .zip(crossed)
            .filter_map(|(&from, &e)| {
                let (a, _, o) = self.edges.get(e)?;
                Some(OrbitStep { overlap: o.clone(), forward: *a == from })
            })
            .collect()
    }
}

/// The tree path from `src` to `dst` as its nodes and the edges crossed
/// between them, named in the same pass that finds them. A SEARCH, counted
/// at its site (AT1 r4): (iii) retires it.
fn tree_path(adj: &[Vec<(usize, usize)>], src: usize, dst: usize) -> Option<(Vec<usize>, Vec<usize>)> {
    if src == dst {
        return Some((vec![src], Vec::new()));
    }
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; adj.len()];
    prev[src] = Some((src, 0));
    let mut q = vec![src];
    let mut qi = 0usize;
    while qi < q.len() {
        let u = q[qi];
        qi += 1;
        if u == dst {
            break;
        }
        for &(v, e) in &adj[u] {
            if prev[v].is_some() {
                continue;
            }
            prev[v] = Some((u, e));
            q.push(v);
        }
    }
    crate::transducer::sentence::form::atlas::note_tree_path(qi);
    prev[dst]?;
    let mut path = vec![dst];
    let mut crossed = Vec::new();
    let mut cur = dst;
    while cur != src {
        let (p, e) = prev[cur]?;
        crossed.push(e);
        path.push(p);
        cur = p;
    }
    path.reverse();
    crossed.reverse();
    Some((path, crossed))
}

impl Form {
    /// AXIOM A, READ (ruled: never a field): the transport from the orbit's
    /// component root to where the cycle is based, composed along its stem.
    /// `None` where a step is unread.
    pub fn orbit_based_at(&self, lp: &Orbit) -> Option<element::Versor> {
        orbit_based_on(&self.faces, lp)
    }
}

/// [`Form::orbit_based_at`] on faces a reader holds — a trial's own, after
/// its move (78.46(4′)).
pub(crate) fn orbit_based_on(faces: &Faces, lp: &Orbit) -> Option<element::Versor> {
    let mut theta = element::identity_versor();
    for s in &lp.stem {
        let c = &faces.at(&s.overlap)?.place.carriage;
        let step = match s.forward {
            true => c.clone(),
            false => crate::transducer::cga::is_versor(c).then(|| c.reverse())?,
        };
        theta = theta.geo(&step);
    }
    Some(theta)
}

fn gate(which: usize) {
    CYCLES.with(|c| {
        let mut t = c.get();
        match which {
            0 => t.0 += 1,
            1 => t.1 += 1,
            2 => t.2 += 1,
            _ => t.3 += 1,
        }
        c.set(t);
    });
}

impl Form {
    /// THE CYCLES OF THE 1-SKELETON, less the ones that enclose nothing.
    ///
    /// Takes the VERTICES and not a count: `orbit_is_trivial` reads the
    /// edge holonomies to decide, and `edge_holonomies_on` needs them. An
    /// orbit flat on its WORD can carry a real edge composite — that is
    /// what `an_orbit_flat_on_the_word_files_under_its_edge_composite`
    /// asserts — and a predicate blind to edges would prune the
    /// obstruction along with the noise.
    /// WHERE A CYCLE DIES BETWEEN THE BASIS AND THE FORM (§8.1).
    ///
    /// `b₁` stayed at zero while the chords reached the face builder, and
    /// three gates stand between a cycle of the member complex and a
    /// `Orbit` on the form. A reading that says only "0 orbits" cannot say
    /// which, so each is counted at the step that decides it.
    pub fn orbit_gates() -> (u64, u64, u64, u64) {
        CYCLES.with(|c| c.get())
    }

    pub fn reset_orbit_gates() {
        CYCLES.with(|c| c.set((0, 0, 0, 0)));
        UNREAD.with(|c| c.set((UnreadTally::default(), UnreadTally::default())));
    }

    /// H2c — WHY THE CYCLES WENT UNREAD, and why the edge half of a cycle
    /// that was read was refused. The first sums to the `unread` gate. The
    /// second counts cycles that passed that gate with their edge composite
    /// refused. A measurement, never a gate.
    pub fn unread_gates() -> (UnreadTally, UnreadTally) {
        UNREAD.with(|c| c.get())
    }

    pub fn orbits_of(faces: &Faces, vertices: &Vertices) -> Vec<Orbit> {
        Self::cycles_of(faces, vertices).kept
    }

    /// THE CYCLE BASIS WHOLE (78.46(4′), 79.3): the orbits, and beside them
    /// the cycles that read trivial alone — the `vacuum` of the cycles line —
    /// KEPT, never cauterized: a trivial circuit is a route, and read against
    /// another route of its anastomosis it can be curved (T1).
    pub fn cycles_of(faces: &Faces, vertices: &Vertices) -> Cycles {
        // THE CYCLE BASIS IS BUILT ON THE MEMBER COMPLEX, NOT ON THE VERTEX
        // INDICES (54.4, as ruled 2026-09-20).
        //
        // There are two complexes here and the engine was computing the
        // homotopy of one and the transport of the other.
        //
        //   V — nodes are `vertices`, edges are `(f.i, f.j)`. This is what
        //   this function traversed: `adj` indexed by vertex, union-find over
        //   vertex indices.
        //
        //   M — nodes are MEMBERS, `member_key(vertex, chart)`: a vertex
        //   ACTING ON A CHART. `extract.rs` builds the faces over exactly
        //   this — `named_faces` reads `scenery.supports`, whose
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
        // 281 real orbits (crossing a face between two bodies rather than
        // the Frame), every face of every one of them carrying a member
        // pair — 0 tears — and 0 closed member paths. 96 of 96, 97 of 97,
        // 88 of 88 passed through a vertex that splits into several
        // members, which is where the lift has to break. And exactly
        // `members − vertices` of the orbits ran through the Frame alone —
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
        // off-plane orbit lives here, and `Rank.offplane` is a lane that
        // could never be non-zero if these faces were dropped
        // (`rank_offplane_is_its_own_lane`).
        //
        // The unnamed node is DISTINCT from every named one, so the two
        // sectors do not chain: a path carrying a body cannot step across
        // a face that places none, which is the truth about that face and
        // not a refusal invented here.
        if vertices.is_empty() {
            return Cycles::default();
        }
        // THE NODES ARE THE CHARTS AND THE EDGES THE OVERLAPS (AT1): each
        // overlap names its two charts, so the node pair is read off the key
        // the transport is filed under — the same coordinate, never a second
        // derivation of it (law 12). A node's ordinal is this reading's own.
        let mut node_of: Keyed<usize> = Keyed::new();
        let mut charts: Vec<coord::ChartAt> = Vec::new();
        let mut edges: Vec<(usize, usize, coord::Overlap)> = Vec::new();
        let mut node = |c: &coord::ChartAt| -> usize {
            let k = c.encode();
            match node_of.get(k.as_bytes()) {
                Some(&ix) => ix,
                None => {
                    let ix = charts.len();
                    node_of.insert(k.as_bytes(), ix);
                    charts.push(c.clone());
                    ix
                }
            }
        };
        // STAR OVERLAPS FIRST, as before, so the spanning tree prefers the
        // basepoint's edges and a chord is an overlap between two bodies.
        for pass in [true, false] {
            for (o, _) in faces.entries() {
                if touches_frame(&o) != pass {
                    continue;
                }
                if !vertices.holds(&o.from_vertex()) || !vertices.holds(&o.to_vertex()) {
                    continue;
                }
                let (a, b) = (node(&o.from), node(&o.to));
                edges.push((a, b, o));
            }
        }
        let n = charts.len();
        if n == 0 {
            return Cycles::default();
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
        let mut orbits = Vec::new();
        let mut vacua = Vec::new();
        // AXIOM A — THE FOREST IS BUILT WHOLE BEFORE ANY CHORD IS CLOSED.
        //
        // This ran one pass: a tree edge extended `adj`, a chord closed a
        // cycle, in the order the edges happened to arrive. A cycle closed
        // early therefore saw a PARTIAL tree, and its transport from the
        // basepoint could not be known — so every orbit was based wherever
        // its own chord sat, in that node's frame, and two orbits of one
        // form were operators on different tangent spaces.
        //
        // Two passes: the spanning forest first, then `θ` by traversal from
        // each component's root, then the chords. Nothing about which
        // edges are tree edges changes — the union-find is the same and in
        // the same order — only when the chords are read.
        let mut chords: Vec<(usize, usize, usize)> = Vec::new();
        for (e, (a, b, _)) in edges.iter().enumerate() {
            let (a, b) = (*a, *b);
            let (pa, pb) = (find(&mut parent, a), find(&mut parent, b));
            if pa != pb {
                parent[pa] = pb;
                adj[a].push((b, e));
                adj[b].push((a, e));
                continue;
            }
            chords.push((a, b, e));
        }
        let forest = Forest { charts, edges, adj };
        // THE TRANSPORT FROM EACH COMPONENT'S ROOT TO EVERY NODE OF IT.
        //
        // `θ_root = 1` and `θ_c = θ_p · Ω_e` along each tree edge, by
        // traversal of the forest the union-find just built — the carrier's
        // own structure, traversed once, never a search (rule 1).
        //
        // A component's root is its least node index, which is the order
        // the nodes were discovered in, which is the faces' own content
        // order — so two minds reading one form choose the same basepoint
        // and the gauge is not a matter of arrival order.
        let (theta, root_of) = tree_transports(faces, &forest);
        for (a, b, e) in chords {
            // A 1-CELL ATTACHED AT ONE 0-CELL IS A CYCLE (54.7, ruled
            // 2026-09-21). `tree_path(adj, a, a)` yields nothing, so this
            // fell through the `continue` below and every reflexive chord
            // was dropped — which is a GRAPH's convention, not a CW
            // complex's. The minimal decomposition of `S¹` is exactly this
            // cell: one vertex, one edge, `∂e¹ = e⁰ − e⁰ = 0`, `b₁ = 1`.
            //
            // It is the shape a footprint chord has by construction. Two
            // disjoint bodies under the identical transformation are one
            // class, one chart and one carriage, so both ends resolve to
            // ONE member — and the 1-cell between them is a self-loop whose
            // holonomy is the Burgers transvection `τ = p(D_b) − p(D_a)`.
            // Refusing the edge because its ends coincide discards the
            // dislocation the cover measured.
            if a == b {
                gate(0);
            }
            let Some((path, crossed)) = (if a == b {
                Some((vec![a], Vec::new()))
            } else {
                tree_path(&forest.adj, a, b)
            }) else {
                gate(1);
                continue;
            };
            // A CYCLE WITH AN END THE FOREST COULD NOT GAUGE IS UNREAD. Its
            // basepoint transport, or the one across its forest path, crossed
            // a carriage on the light cone (57.4), and an orbit based through
            // an unread transport is not a flat one (rule 11).
            let theta_a = match (&theta[a], &theta[b]) {
                (Ok(t), Ok(_)) => t.clone(),
                (Err(why), _) | (_, Err(why)) => {
                    unread(*why);
                    continue;
                }
            };
            // THE CYCLE AS THE OVERLAPS IT CROSSES (AT1): the forest path
            // from `a` to `b`, then the chord back from `b` to `a`, each step
            // with the way it runs; and the stem from the component's root
            // to `a`, where the cycle is based (Axiom A). The holonomy, the
            // carriage and `θ_a` are READ off these (ruled), never held.
            let Some((chord_from, _, chord)) = forest.edges.get(e) else {
                unread(Unread::NoFace);
                continue;
            };
            let mut steps = forest.steps(&path, &crossed);
            steps.push(OrbitStep { overlap: chord.clone(), forward: *chord_from == b });
            let Some((stem_path, stem_crossed)) = tree_path(&forest.adj, root_of[a], a) else {
                unread(Unread::NoFace);
                continue;
            };
            let lp = Orbit { steps, chord: chord.clone(), stem: forest.steps(&stem_path, &stem_crossed) };
            // A CHORD THAT ENCLOSES NOTHING IS NOT AN ORBIT (ruled
            // 2026-09-18). `orbit_is_trivial` states the predicate once;
            // a cycle that encloses no rotation, no placement and no hue
            // occupies no axis of 12.6, drives no pivot, and was traversed by
            // every reader of `orbits` for nothing — 16 546 of them on 174
            // forms in ten minutes.
            // ONE TRAVERSAL, read twice. `orbit_is_trivial` and the carriages
            // are two readings of the same frontier, and calling the two
            // readers would traverse it again for each.
            let carried = match paths_carried_why(faces, vertices, &lp) {
                Ok(c) => c,
                Err(why) => {
                    unread(why);
                    continue;
                }
            };
            // A CYCLE READ WITH ITS EDGE HALF REFUSED is not unread, and it is
            // counted apart by what refused it (H2c).
            for (_, c) in carried.scoped() {
                if let Some(why) = c.edge_refused {
                    UNREAD.with(|u| {
                        let mut t = u.get();
                        t.1.note(why);
                        u.set(t);
                    });
                }
            }
            if orbit_is_trivial(faces, vertices, &lp, &carried) {
                gate(3);
                // OBSERVE ONLY: the gate is unchanged, but a closure that
                // cancelled real charge is a relator and a closure over an
                // uncharged subgraph is vacuum, and `gate(3)` alone cannot
                // tell them apart.
                note_closure(&carried);
                // KEPT BESIDE THE ORBITS (78.46(4′)): trivial alone, and
                // read against its anastomosis's other circuits.
                vacua.push(lp);
                continue;
            }
            for (_, c) in carried.scoped() {
                // THE DEFECT THIS CYCLE CARRIES, read once per member path
                // off the transport it composes (§8.1).
                note_defect(c);
                // OR1 — AND THE ORBIT IT IS (73.1, 73.2), read off the same
                // composite where a reader is listening, in the corpus's
                // algebra at least.
                //
                // OR3 — through the one reader: a cycle whose edge half is
                // refused is UNREAD at its step, never classed off the face
                // half alone.
                let composite = c.composite();
                note_orbit(|| OrbitNote {
                    composite: composite.clone(),
                    at: c.edge_lost_at,
                    why: c.edge_refused,
                    gauge_static: c.gauge_static(),
                    coset: c.holds_coset().then(|| c.clone()),
                });
                // PL1 — AND THE SAME CIRCUIT'S BRICKS, step by step (78.12(4)).
                super::trajectory::note_circuit(|| super::trajectory::read_circuit(&lp.chord, c, faces));
                // AND THE OPERATOR THIS GENERATOR ENACTS, at the basepoint
                // (Axiom A). Two cycles agreeing here are synonyms: distinct
                // cellular paths, one physical dislocation.
                if let (true, Some(composite)) = (crate::transducer::cga::is_versor(&theta_a), &composite) {
                    note_synonym(&theta_a.sandwich(composite));
                }
                // AND WHETHER ITS ORBIT CLOSES — asked of the CONNECTION,
                // not of the alphabet (54.7).
                //
                // This read `lp.holonomy.rotor().toral_period()`: the
                // composite of the faces' WORDS, which live in the
                // `Cl(4,0)` letter chart where every letter is a Euclidean
                // rotation and therefore every word is compact. 545 of 545
                // read as CLOSING while 541 classified Hyperbolic — two
                // readings of one object that cannot both be true. That
                // reader interrogates the ALPHABET; the transvection is in
                // the transport composite.
                //
                // `Ω_e = Ω_carriage · Ω_edge` is the TOTAL transport, and
                // dropping either half is the projection defect one level
                // out from the algebra — measured: dropping the carriage
                // left the four parabolics completely unchanged, which is
                // what proves the null generator is sealed inside `edge` at
                // the moment `meet_edge` evaluates `V_b ∘ V_a⁻¹`.
                if let Some(composite) = &composite {
                    SPECTRA.with(|s| {
                        use crate::transducer::cga::Spectrum;
                        let mut t = s.get();
                        match crate::transducer::cga::compact_period(composite) {
                            Spectrum::Closes(_) => t.0 += 1,
                            Spectrum::Open => t.1 += 1,
                            Spectrum::Undecomposed => t.2 += 1,
                            Spectrum::Isoclinic => t.3 += 1,
                        }
                        s.set(t);
                    });
                }
            }
            orbits.push(lp);
        }
        Cycles { kept: orbits, vacua }
    }
}

/// THE CYCLE BASIS, SPLIT BY ITS OWN READING: the orbits that carry a
/// holonomy alone, and the cycles that read trivial alone (78.46(4′)).
#[derive(Debug, Clone, Default)]
pub struct Cycles {
    pub kept: Vec<Orbit>,
    pub vacua: Vec<Orbit>,
}

// `path_carriages(faces, path, walked)` stood here: a traversal of the orbit's
// member paths accumulating each face's `place.carriage`, keyed by the path.
// It is `orbit_transport_on` with the word and the gauge discarded — the same
// frontier, the same `step_charts`, the same `closed_key`, the same
// `carriage.geo(&acc)` — so one quantity had two carriers (law 12) and the
// traversal that produces it ran twice per orbit: once at `orbits_of` to fill
// `Orbit.carriages`, and again at every class reading through
// `orbit_transport_on`. `orbits_of` fills the field from that one traversal now.
//
// The doc it carried is true of `orbit_transport_on` and belongs to it: the
// paths are traversed FROM THE FACES and not taken as a product of the vertices'
// fibres, because each face's `at` keys are its member pairs; a step extends a
// partial path only where the face relates the chart the path stands at to the
// chart it steps to; a path the next face does not relate simply ends and is
// not a cycle of this complex; and a path is kept only where it ends at the
// chart it began at.

// `PartialPath` STOOD HERE and is gone with the search it was the state of
// (54.5, as ruled 2026-09-20). It held a frontier's charts, the chart it
// stood AT, the chart it began at, and the accumulator — the machinery for
// reconstructing a member path that `orbits_of` already had and threw away
// when it projected the cycle onto vertex indices to fit the tape.
//
// `Orbit.members` carries the cycle, `paths_carried` traverses it, and there is
// no frontier to keep the state of.
