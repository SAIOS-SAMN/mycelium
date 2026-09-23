// 12.2, 18.2 — THE LOOPS of a form: the cycle basis on the member complex,
// each cycle based by Axiom A, and the gates it passes to become a `Loop`.

use super::*;

thread_local! {
    /// `(reflexive cells seen, dropped at tree_path, dropped at
    /// paths_carried, dropped as trivial)` — the three gates between a
    /// cycle of the member complex and a `Loop` on the form.
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
/// A DESCENT OVER THE FOREST, NOT A SEARCH. The spanning forest is already
/// built; this walks it once from each root, and every node is reached
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
fn tree_transports(
    faces: &[FormFace],
    vertices: &[Vertex],
    adj: &[Vec<(usize, usize)>],
    node_vertex: &[u16],
    node_chart: &[Vec<u8>],
) -> Vec<Result<element::Versor, Unread>> {
    let n = node_vertex.len();
    let mut theta: Vec<Result<element::Versor, Unread>> = vec![Ok(element::identity_versor()); n];
    let mut seen = vec![false; n];
    // index: the forest's own nodes, each visited once (rule 1).
    for root in 0..n {
        if seen[root] {
            continue;
        }
        seen[root] = true;
        let mut frontier = vec![root];
        while let Some(p) = frontier.pop() {
            for &(c, fi) in &adj[p] {
                if seen[c] {
                    continue;
                }
                seen[c] = true;
                let step = tree_step(faces, vertices, node_vertex, node_chart, p, c, fi);
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
    theta
}

/// THE TRANSPORT ACROSS ONE TREE EDGE, in the direction `p → c`.
///
/// Read off the face's own member pair, exactly as `paths_carried` reads a
/// step — the same coordinate, not a second derivation of it (rule 12) —
/// and inverted where the walk runs against the face's orientation.
/// Identity where the face carries no transport at that pair, which is a
/// face relating no member and is a real sector of the complex.
///
/// `Cone` where the inverse does not exist: `versor_inverse` fails only at
/// `MM̃ = 0`, a carriage on the light cone (57.4), and that transport was
/// never read. `NoFace` where the forest names a face the form does not
/// hold.
fn tree_step(
    faces: &[FormFace],
    vertices: &[Vertex],
    node_vertex: &[u16],
    node_chart: &[Vec<u8>],
    p: usize,
    c: usize,
    fi: usize,
) -> Result<element::Versor, Unread> {
    let f = faces.get(fi).ok_or(Unread::NoFace)?;
    let _ = vertices;
    let (vp, cp) = (node_vertex[p], &node_chart[p]);
    let (vc, cc) = (node_vertex[c], &node_chart[c]);
    let forward = f.i == vp && f.j == vc;
    let pk = match forward {
        true => pair_key_bytes(cp, cc),
        false => pair_key_bytes(cc, cp),
    };
    let Some(place) = f.at.get(&pk) else { return Ok(element::identity_versor()) };
    match forward {
        true => Ok(place.carriage.clone()),
        false => place.carriage.versor_inverse().ok_or(Unread::Cone),
    }
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
    /// Takes the VERTICES and not a count: `loop_is_trivial` reads the
    /// edge holonomies to decide, and `edge_holonomies_on` needs them. A
    /// loop flat on its WORD can carry a real edge composite — that is
    /// what `a_loop_flat_on_the_word_files_under_its_edge_composite`
    /// asserts — and a predicate blind to edges would prune the
    /// obstruction along with the noise.
    /// WHERE A CYCLE DIES BETWEEN THE BASIS AND THE FORM (§8.1).
    ///
    /// `b₁` stayed at zero while the chords reached the face builder, and
    /// three gates stand between a cycle of the member complex and a
    /// `Loop` on the form. A reading that says only "0 loops" cannot say
    /// which, so each is counted at the step that decides it.
    pub fn loop_gates() -> (u64, u64, u64, u64) {
        CYCLES.with(|c| c.get())
    }

    pub fn reset_loop_gates() {
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
        //   this — `named_faces` walks `scenery.supports`, whose
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
        let mut node_chart: Vec<Vec<u8>> = Vec::new();
        let mut edges: Vec<(usize, usize, usize)> = Vec::new();
        let node = |node_of: &mut Keyed<u16>,
                        node_vertex: &mut Vec<u16>,
                        node_chart: &mut Vec<Vec<u8>>,
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
                    node_chart.push(chart.to_vec());
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
                    let a = node(&mut node_of, &mut node_vertex, &mut node_chart, f.i, &ci);
                    let b = node(&mut node_of, &mut node_vertex, &mut node_chart, f.j, &cj);
                    edges.push((a, b, fi));
                }
                if named == 0 {
                    let a = node(&mut node_of, &mut node_vertex, &mut node_chart, f.i, &[]);
                    let b = node(&mut node_of, &mut node_vertex, &mut node_chart, f.j, &[]);
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
        // AXIOM A — THE FOREST IS BUILT WHOLE BEFORE ANY CHORD IS CLOSED.
        //
        // This ran one pass: a tree edge extended `adj`, a chord closed a
        // cycle, in the order the edges happened to arrive. A cycle closed
        // early therefore saw a PARTIAL tree, and its transport from the
        // basepoint could not be known — so every loop was based wherever
        // its own chord sat, in that node's frame, and two loops of one
        // form were operators on different tangent spaces.
        //
        // Two passes: the spanning forest first, then `θ` by descent from
        // each component's root, then the chords. Nothing about which
        // edges are tree edges changes — the union-find is the same and in
        // the same order — only when the chords are read.
        let mut chords: Vec<(usize, usize, usize)> = Vec::new();
        for (a, b, fi) in edges {
            let (pa, pb) = (find(&mut parent, a), find(&mut parent, b));
            if pa != pb {
                parent[pa] = pb;
                adj[a].push((b, fi));
                adj[b].push((a, fi));
                continue;
            }
            chords.push((a, b, fi));
        }
        // THE TRANSPORT FROM EACH COMPONENT'S ROOT TO EVERY NODE OF IT.
        //
        // `θ_root = 1` and `θ_c = θ_p · Ω_e` along each tree edge, by
        // descent over the forest the union-find just built — the carrier's
        // own structure, walked once, never a search (rule 1).
        //
        // A component's root is its least node index, which is the order
        // the nodes were discovered in, which is the faces' own content
        // order — so two minds reading one form choose the same basepoint
        // and the gauge is not a matter of arrival order.
        let theta = tree_transports(faces, vertices, &adj, &node_vertex, &node_chart);
        for (a, b, fi) in chords {
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
            let Some((path, mut walked)) = (if a == b {
                Some((vec![a], Vec::new()))
            } else {
                tree_path(&adj, a, b)
            }) else {
                gate(1);
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
            // THE CYCLE, NAMED BY ITS MEMBERS, and the projection beside
            // it. The members are the cycle; `vertices` is `π` of them,
            // kept because every reader of a loop asks the form for a
            // vertex, and lossy at exactly the edge between two charts of
            // one vertex.
            let mut members: Vec<Vec<u8>> = path
                .iter()
                .map(|&ix| crate::transducer::family::member_key(node_vertex[ix], &node_chart[ix]))
                .collect();
            members.push(crate::transducer::family::member_key(node_vertex[a], &node_chart[a]));
            let mut path: Vec<usize> =
                path.into_iter().map(|ix| node_vertex[ix] as usize).collect();
            path.push(node_vertex[a] as usize);
            walked.push(fi);
            // A CYCLE WITH AN END THE FOREST COULD NOT GAUGE IS UNREAD. Its
            // basepoint transport, or the one across its forest path, crossed
            // a carriage on the light cone (57.4), and a loop based through
            // an unread transport is not a flat one (rule 11).
            let theta_a = match (&theta[a], &theta[b]) {
                (Ok(t), Ok(_)) => t.clone(),
                (Err(why), _) | (_, Err(why)) => {
                    unread(*why);
                    continue;
                }
            };
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
                members,
                vertices: path.into_iter().map(|x| x as u16).collect(),
                faces: walked.into_iter().map(|x| x as u16).collect(),
                chord: fi as u16,
                holonomy,
                carriages: Keyed::new(),
                // AXIOM A — the cycle closes at `a` through the tree, so
                // `a` is where it is based and `θ_a` is what carries it to
                // the form's basepoint.
                based_at: theta_a,
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
            if loop_is_trivial(faces, vertices, &lp, &carried) {
                gate(3);
                // OBSERVE ONLY: the gate is unchanged, but a closure that
                // cancelled real charge is a relator and a closure over an
                // uncharged subgraph is vacuum, and `gate(3)` alone cannot
                // tell them apart.
                note_closure(&carried);
                continue;
            }
            for (k, c) in carried.scoped() {
                // THE DEFECT THIS CYCLE CARRIES, read once per member path
                // off the transport it composes (§8.1).
                note_defect(c);
                // AND THE OPERATOR THIS GENERATOR ENACTS, at the basepoint
                // (Axiom A). Two cycles agreeing here are synonyms: distinct
                // cellular paths, one physical dislocation.
                if let Some(inv) = lp.based_at.versor_inverse() {
                    let composite = match &c.edge {
                        Some(e) => e.carriage.geo(&c.face.carriage),
                        None => c.face.carriage.clone(),
                    };
                    note_synonym(&lp.based_at.geo(&composite).geo(&inv));
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
                let composite = match &c.edge {
                    Some(e) => e.carriage.geo(&c.face.carriage),
                    None => c.face.carriage.clone(),
                };
                SPECTRA.with(|s| {
                    use crate::transducer::cga::Spectrum;
                    let mut t = s.get();
                    match crate::transducer::cga::compact_period(&composite) {
                        Spectrum::Closes(_) => t.0 += 1,
                        Spectrum::Open => t.1 += 1,
                        Spectrum::Undecomposed => t.2 += 1,
                        Spectrum::Isoclinic => t.3 += 1,
                    }
                    s.set(t);
                });
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

// `PartialPath` STOOD HERE and is gone with the search it was the state of
// (54.5, as ruled 2026-09-20). It held a frontier's charts, the chart it
// stood AT, the chart it began at, and the accumulator — the machinery for
// reconstructing a member path that `loops_of` already had and threw away
// when it projected the cycle onto vertex indices to fit the tape.
//
// `Loop.members` carries the cycle, `paths_carried` walks it, and there is
// no frontier to keep the state of.
