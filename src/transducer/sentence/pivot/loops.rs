// 12.2, 18.2 — THE LOOPS of a form, and the faces a pivot writes.

use super::*;

thread_local! {
    /// `(reflexive cells seen, dropped at tree_path, dropped at
    /// paths_carried, dropped as trivial)` — the three gates between a
    /// cycle of the member complex and a `Loop` on the form.
    static CYCLES: std::cell::Cell<(u64, u64, u64, u64)> =
        const { std::cell::Cell::new((0, 0, 0, 0)) };
}

thread_local! {
    /// HOW THE CLOSING CYCLES DIVIDE — `(vacuum, resonant, the charges the
    /// resonant ones carried)` (54.7, ruled 2026-09-21).
    ///
    /// `gate(3)` counts every cycle whose boundary product is the
    /// identity, and two opposite facts land in that one number: a cycle
    /// over a subgraph where the corpus wrote no transport, and a cycle
    /// whose non-zero shears cancel around the perimeter. The first
    /// quotients nothing; the second is a relator of `π₁`.
    ///
    /// OBSERVATIONAL ONLY. `loop_is_trivial` keeps pruning exactly what it
    /// pruned before, so `loops.len()` and the tests asserting incidence
    /// through it are untouched. Both kinds genuinely belong to `ker(ρ)` —
    /// neither deposits net holonomic shear at `x₀` — so the gate is right
    /// and only the reporting was blind.
    ///
    /// The charges are kept whole rather than summed: the distribution is
    /// the content and a total cannot be un-added (rule 2).
    /// The last two count EVERY step of EVERY cycle, closing or not:
    /// `(steps whose charge was read, of those, steps carrying μ ≠ 0)`.
    /// Without them "all closures are vacuum" cannot be distinguished from
    /// "the charge was never computed" — a probe that never fired reporting
    /// zero as though it had measured one (rule 11).
    static RESONANCE: std::cell::RefCell<(u64, u64, Vec<num_rational::BigRational>, u64, u64)> =
        const { std::cell::RefCell::new((0, 0, Vec::new(), 0, 0)) };
}

/// File one STEP's charge, on every cycle the walk reaches.
fn note_step_charge(mu: &num_rational::BigRational) {
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    RESONANCE.with(|r| {
        let mut b = r.borrow_mut();
        b.3 += 1;
        if *mu != zero {
            b.4 += 1;
        }
    });
}

/// File one CLOSING cycle by whether it carried any charge at all.
fn note_closure(carried: &Keyed<PathCarried>) {
    let mut charges: Vec<num_rational::BigRational> = Vec::new();
    for (_, c) in carried.scoped() {
        charges.extend(c.charges.iter().cloned());
    }
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let charged: Vec<num_rational::BigRational> =
        charges.into_iter().filter(|m| *m != zero).collect();
    RESONANCE.with(|r| {
        let mut b = r.borrow_mut();
        match charged.is_empty() {
            true => b.0 += 1,
            false => {
                b.1 += 1;
                b.2.extend(charged);
            }
        }
    });
}

/// `(vacuum closures, resonant closures, the resonant charges)`.
pub fn loop_resonance() -> (u64, u64, Vec<num_rational::BigRational>, u64, u64) {
    RESONANCE.with(|r| r.borrow().clone())
}

pub fn reset_loop_resonance() {
    RESONANCE.with(|r| *r.borrow_mut() = (0, 0, Vec::new(), 0, 0));
}

thread_local! {
    /// THE DEFECT EACH KEPT CYCLE CARRIES, read off its transport
    /// composite: `(trivial, elliptic, hyperbolic, parabolic, non-simple)`.
    ///
    /// A cycle's class is not a label the builder assigns; it is
    /// `class_invariants` of the versor the loop's own faces compose, so
    /// the distribution is a reading of the corpus and not of this code.
    static DEFECTS: std::cell::Cell<(u64, u64, u64, u64, u64)> =
        const { std::cell::Cell::new((0, 0, 0, 0, 0)) };

    /// THE TORAL SPECTRUM OF EACH KEPT CYCLE:
    /// `(closes, open, undecomposed over ℚ, isoclinic)`.
    ///
    /// A toral period is a COMPACT invariant — the order of a rotation. A
    /// pure transvection generates a non-compact one-parameter subgroup and
    /// its geodesic flow has no returning orbit, so it has no finite period
    /// at all: it lands on the projective boundary `[1:0]` and charges
    /// shear rather than sounding a pitch.
    ///
    /// FOUR COUNTERS, NOT TWO. An orbit that does not close and a split
    /// that does not EXIST are different facts about different things, and
    /// folding them together reported 545 "open" when the truth was 541
    /// open and 4 undecomposed (rule 11).
    static SPECTRA: std::cell::Cell<(u64, u64, u64, u64)> =
        const { std::cell::Cell::new((0, 0, 0, 0)) };
}

thread_local! {
    /// `ker(ρ)` ON THE CYCLE BASIS — the based generators, counted by the
    /// OPERATOR each enacts (54.7, ruled 2026-09-21).
    ///
    /// Now that Axiom A puts every cycle in the basepoint's frame, two
    /// generators can be compared: `ρ(γ_i) ~ ±ρ(γ_j)` means two distinct
    /// cellular paths enact the identical physical dislocation — literal
    /// synonyms, which 13.10 predicts ("one theorem has as many proofs as
    /// the form has synonyms of one class") and which nothing has ever
    /// counted.
    ///
    /// `trivial` counts `ρ(γ) ~ ±1`: a chord whose transport the spanning
    /// tree's return path cancelled exactly. Such a generator is in
    /// `ker(ρ)` outright — a real cycle of `π₁` carrying no geometry.
    ///
    /// COUNTED BY A KEYED READ, never a pairwise walk (rule 3): the
    /// operator's own canonical bytes are its coordinate, so uniqueness is
    /// a trie insert and `|im(ρ)|` is the carrier's length.
    /// THE QUOTIENT TOWER, MEASURED AT EACH LEVEL (54.7, ruled
    /// 2026-09-21):
    ///
    /// ```text
    ///   words in π₁(X¹) ↠ π₁(X, x₀) ↠ im(ρ) ↠ Conj(im ρ)
    ///        level 0          level 1      level 2
    /// ```
    ///
    /// The two axioms project onto ORTHOGONAL factors and neither sees the
    /// other's level:
    ///
    ///   AXIOM B is spatial — it fixes `(I₁, I₂, kind)`, the eigenvalues
    ///   and plane signatures, and is completely blind to which tree or
    ///   basepoint was chosen. It determines LEVEL 2.
    ///
    ///   AXIOM A is compositional — it orients the operator inside the
    ///   shared tangent space at `x₀`, acting along the fibre of level 1
    ///   over level 2. Re-rooting applies one inner automorphism `Ad_Θ`,
    ///   which moves the level-1 representative and leaves its level-2
    ///   projection fixed.
    ///
    /// `Ad_θ` is therefore a CONGRUENCE: an equivalence compatible with
    /// the group operation, and the reason basing had to move no reading.
    ///
    /// Held: `(operators by exact projective equality, classes by Casimir,
    /// generators seen, generators with ρ ~ ±1)`.
    static SYNONYMS: std::cell::RefCell<(Keyed<u64>, Keyed<u64>, u64, u64)> =
        std::cell::RefCell::new((Keyed::new(), Keyed::new(), 0, 0));

    /// THE OPERATORS OF EACH CASIMIR CLASS — level 1 grouped by level 2.
    ///
    /// The antipodal hypothesis needs the operators WITHIN a class, not
    /// just how many there are: if a class holds exactly two and they are
    /// mutually inverse, the pair is one undirected geodesic channel
    /// traversed both ways, and the corpus vocabulary is a set of
    /// bidirectional dislocation lines rather than scattered directions.
    ///
    /// Keyed by the class so the question is a keyed read (rule 3); the
    /// value carries each distinct operator's own bytes so the product can
    /// be re-derived and tested rather than assumed.
    static CLASS_OPS: std::cell::RefCell<Keyed<Vec<Vec<u8>>>> =
        std::cell::RefCell::new(Keyed::new());
}

/// THE ANTIPODAL TEST, run over the classes (54.7).
///
/// `(operators in the class, whether the class is exactly a mutually
/// inverse pair)` for every Casimir class, in the carrier's own key order.
/// A class of two whose product is projectively the identity is one
/// geodesic line read forward and backward.
pub fn loop_class_report() -> Vec<(usize, bool)> {
    CLASS_OPS.with(|c| {
        c.borrow()
            .scoped()
            .map(|(_, ops)| {
                let inverse = match ops.as_slice() {
                    [a, b] => {
                        let decode = |v: &Vec<u8>| {
                            let mut p = 0usize;
                            crate::transducer::cga::Mv::from_bytes(v, &mut p)
                        };
                        match (decode(a), decode(b)) {
                            (Some(x), Some(y)) => {
                                x.geo(&y).projective_eq(&element::identity_versor())
                            }
                            _ => false,
                        }
                    }
                    _ => false,
                };
                (ops.len(), inverse)
            })
            .collect()
    })
}

pub fn reset_loop_class_ops() {
    CLASS_OPS.with(|c| *c.borrow_mut() = Keyed::new());
}

/// THE OPERATORS OF ONE FORM — the univalence test.
///
/// `|im(ρ_F)|` over this form's own cycle basis. A form with one operator
/// is an elementary token: however many internal cycles it carries, they
/// are that many geometric proofs of ONE theorem. A form with several is
/// compound — an unresolved sentence rather than a letter — and the
/// distinction between alphabet and grammar depends on which it is.
///
/// Computed where it is reported, from the form's own loops, so there is
/// no cumulative state to reset and no way for one form's reading to leak
/// into another's.
pub fn form_operators(f: &Form) -> usize {
    let mut ops: Keyed<()> = Keyed::new();
    for lp in &f.loops {
        let Some(inv) = lp.based_at.versor_inverse() else { continue };
        for (_, omega) in lp.carriages.scoped() {
            let based = lp.based_at.geo(omega).geo(&inv);
            let Some(canonical) = based.canonical() else { continue };
            let mut bytes = Vec::new();
            canonical.to_bytes(&mut bytes);
            ops.insert(&bytes, ());
        }
    }
    ops.len()
}

/// `(|im(ρ)|, |Conj|, generators seen, generators with ρ ~ ±1)`.
///
/// `b₁ − |im(ρ)|` is the semantic compression: distinct cellular paths
/// enacting one physical displacement. `|im(ρ)| / |Conj|` is the
/// directional dispersion — how many distinct Burgers vectors the corpus
/// spans within each kinematic class.
pub fn loop_synonyms() -> (u64, u64, u64, u64) {
    SYNONYMS.with(|s| {
        let b = s.borrow();
        (b.0.len() as u64, b.1.len() as u64, b.2, b.3)
    })
}

pub fn reset_loop_synonyms() {
    SYNONYMS.with(|s| *s.borrow_mut() = (Keyed::new(), Keyed::new(), 0, 0));
}

/// File one BASED generator by the operator it enacts.
fn note_synonym(based: &element::Versor) {
    let Some(canonical) = based.canonical() else { return };
    let mut bytes = Vec::new();
    canonical.put_canonical(&mut bytes);
    let trivial = based.projective_eq(&element::identity_versor());
    // LEVEL 2 — the conjugacy class, which Axiom A cannot move. A
    // generator whose class cannot be read is counted at level 1 and not
    // at level 2, rather than filed under a class it does not have.
    let class = based.class_key().map(|a| a.as_bytes().to_vec());
    SYNONYMS.with(|s| {
        let mut b = s.borrow_mut();
        b.2 += 1;
        if trivial {
            b.3 += 1;
        }
        let seen = b.0.get(&bytes).copied().unwrap_or(0);
        b.0.insert(&bytes, seen + 1);
        if let Some(c) = class.clone() {
            let seen = b.1.get(&c).copied().unwrap_or(0);
            b.1.insert(&c, seen + 1);
        }
    });
    // AND THE OPERATOR FILED UNDER ITS CLASS, so the antipodal test can ask
    // which operators share a kinematic scale.
    let Some(c) = class else { return };
    let mut wire = Vec::new();
    canonical.to_bytes(&mut wire);
    CLASS_OPS.with(|m| {
        let mut m = m.borrow_mut();
        match m.get_mut(&c) {
            Some(held) => {
                if !held.contains(&wire) {
                    held.push(wire);
                }
            }
            None => {
                m.insert(&c, vec![wire]);
            }
        }
    });
}

/// `(closes, open, undecomposed over ℚ, isoclinic)`.
pub fn loop_spectra() -> (u64, u64, u64, u64) {
    SPECTRA.with(|c| c.get())
}

pub fn reset_loop_spectra() {
    SPECTRA.with(|c| c.set((0, 0, 0, 0)));
}

/// The defect distribution of every cycle kept since the last reset.
pub fn loop_defects() -> (u64, u64, u64, u64, u64) {
    DEFECTS.with(|c| c.get())
}

pub fn reset_loop_defects() {
    DEFECTS.with(|c| c.set((0, 0, 0, 0, 0)));
}

/// Classify one kept cycle by THE WHOLE TRANSPORT IT COMPOSES.
///
/// A first cut read `PathCarried::face.carriage` alone and every one of 545
/// cycles came back Trivial — while `loop_is_trivial` had KEPT them, which
/// can only happen when something non-flat is present. The chord's
/// dislocation rides in 50.3's EDGE transport, not in the member carriage:
/// two sections under one carriage have `φ_b φ_a⁻¹ = 1` by construction,
/// so the carriage is exactly where the Burgers vector is not.
///
/// The cycle's holonomy is the composite of both, which is what
/// `Ω_∂e² = 𝒫 ∏ Ω_e` means.
fn note_defect(carried: &PathCarried) {
    use crate::network::nerve::coord::BivectorKind;
    let composite = match &carried.edge {
        Some(e) => e.carriage.geo(&carried.face.carriage),
        None => carried.face.carriage.clone(),
    };
    let Some((_, _, kind)) = composite.class_invariants() else { return };
    DEFECTS.with(|c| {
        let mut t = c.get();
        match kind {
            BivectorKind::Trivial => t.0 += 1,
            BivectorKind::Elliptic => t.1 += 1,
            BivectorKind::Hyperbolic => t.2 += 1,
            BivectorKind::Parabolic => t.3 += 1,
        }
        // A DISPIRATION IS NOT A FOURTH TYPE, it is a coupling: the
        // generator is non-simple, so the cycle turns and slips at once.
        // Counted beside the type rather than instead of it.
        if !crate::transducer::cga::bivector_is_simple(&composite) {
            t.4 += 1;
        }
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
/// A tree edge whose transport cannot be read leaves its subtree at the
/// identity rather than halting the descent: the cycle that later asks for
/// that node's `θ` is then based at its own node, which is the reading
/// before Axiom A and is honest about what was not measured.
fn tree_transports(
    faces: &[FormFace],
    vertices: &[Vertex],
    adj: &[Vec<(usize, usize)>],
    node_vertex: &[u16],
    node_chart: &[Vec<u8>],
) -> Vec<element::Versor> {
    let n = node_vertex.len();
    let mut theta: Vec<element::Versor> = vec![element::identity_versor(); n];
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
                theta[c] = theta[p].geo(&step);
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
fn tree_step(
    faces: &[FormFace],
    vertices: &[Vertex],
    node_vertex: &[u16],
    node_chart: &[Vec<u8>],
    p: usize,
    c: usize,
    fi: usize,
) -> element::Versor {
    let id = element::identity_versor();
    let Some(f) = faces.get(fi) else { return id };
    let _ = vertices;
    let (vp, cp) = (node_vertex[p], &node_chart[p]);
    let (vc, cc) = (node_vertex[c], &node_chart[c]);
    let forward = f.i == vp && f.j == vc;
    let pk = match forward {
        true => pair_key_bytes(cp, cc),
        false => pair_key_bytes(cc, cp),
    };
    let Some(place) = f.at.get(&pk) else { return id };
    match forward {
        true => place.carriage.clone(),
        false => place.carriage.versor_inverse().unwrap_or(id),
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
                based_at: theta
                    .get(a)
                    .cloned()
                    .unwrap_or_else(element::identity_versor),
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
                gate(2);
                continue;
            };
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
    /// THE KINEMATIC CHARGE EACH STEP OF THIS BOUNDARY DEPOSITED, in
    /// boundary order and exact over ℚ (54.7, ruled 2026-09-21).
    ///
    /// `face` and `edge` above are ORDERED PRODUCTS around `∂f`, so a
    /// composite reading identity is already the resonance condition
    /// `Π Ω_e ∼ ±1`. But a product of identities is the identity too, and
    /// the two are opposite physical facts:
    ///
    /// ```text
    ///   RELATOR  some μ_e ≠ 0, and the versors cancel  — geometry closing
    ///   VACUUM   every μ_e = 0                         — no geometry
    /// ```
    ///
    /// A relator is a Seifert–van Kampen boundary word and quotients `π₁`;
    /// a vacuum cycle quotients nothing and says only that the corpus
    /// wrote no transport there. One count for both reports the ABSENCE of
    /// geometry as the CANCELLATION of it (rule 11).
    ///
    /// Kept as a SEQUENCE and never summed. `Σ μ_e` cannot tell one edge
    /// carrying 12 from twelve edges carrying 1, and those are different
    /// circuits; the sum is a moment any reader can take, while the
    /// distribution cannot be recovered from it (rule 2).
    ///
    /// ```text
    ///     μ = ⟨Ω⟩₀² / ⟨Ω Ω̃⟩₀ − 1
    /// ```
    ///
    /// SCALE-FREE, which the closed form `(w − 4)/4` is not: a versor is
    /// projective, so `Ω` and `λΩ` are one transport while `w → λ²w`, and
    /// the stored identity is `1` rather than the canonical `1 − v_a v_a =
    /// 2` — under `(w − 4)/4` the identity would read `−3/4` instead of
    /// `0`. On the canonical normalisation `w = 2⟨R⟩₀` the two agree. It
    /// reads the TRANSVECTION factor, not the composed step: the identity
    /// is exact only where no rotation contributes cross-terms, and the
    /// coupling belongs at the loop, where `Π Ω_e` already evaluates it.
    /// Over the whole group `μ > 0` is hyperbolic, `μ < 0` elliptic
    /// (`−sin²(θ/2)`), `μ = 0` trivial — `I₁`'s own sign convention.
    pub(crate) charges: Vec<num_rational::BigRational>,
}

/// THE ONE WALK — and it is a WALK now, not a search (54.5, as ruled
/// 2026-09-20).
///
/// This was a frontier: seed with every member of the first face, extend
/// through each face's `at` by whichever members continued the path, keep
/// whatever closed. That search existed because `Loop` named its nodes by
/// VERTEX INDEX and the member coordinates had been projected away, so the
/// walker had to reconstruct what `loops_of` already knew.
///
/// It also could not always reconstruct it. Under the projection a 1-cell
/// between two charts at one vertex, `((v,c_a),(v,c_b))`, becomes `(v,v)`
/// and loses its orientation — 62 of 62 live loops broke there, one step
/// from closing, and `437d253` patched it by trying both directions.
///
/// `Loop.members` carries the cycle, so there is nothing to search for and
/// nothing to guess: the walk steps the members it was given, reads each
/// face at the pair those two members name, and composes. ONE path per
/// loop, always, because a cycle is one cycle.
///
/// THE STAR FACES ARE SKIPPED IN THE EDGE HALF, and the reason written here
/// was wrong (corrected 2026-09-20). It read "a star face is the
/// trivialization every body is read against (2.2)". Chapter 2.2 says no
/// such thing — it defines vertices, faces and holonomies and never
/// mentions the star — and SKIPPING A FACE IS NOT GAUGING IT: §3.6 defines
/// a re-gauging as `g_ij → γ_j · g_ij · γ_i⁻¹`, a conjugation, under which
/// holonomies are conjugated and the identity stays the identity. Dropping
/// a factor out of a product is a different operation with a different
/// answer.
///
/// WHAT IS ACTUALLY TRUE. 50.3's edge is the stabilizer element between two
/// BODIES, and the Frame carries no body at any frame, so `edge_between`
/// measures nothing across a star face — `at(0, c_F, k)` finds nothing and
/// the reading is ABSENT, not identity. The placement of a body against the
/// scene is not lost with it: it is the member's own carriage, and the FACE
/// composite above composes it at every step. Two carriers, one for where a
/// body stands and one for how two bodies relate, and the edge half reads
/// only the second.
///
/// WHAT IS NOT ESTABLISHED, and is not claimed here. The star faces are the
/// spanning forest's first pass, so a loop's star faces are tree edges —
/// but nothing in this walk checks that, and a star face reached as a CHORD
/// would be skipped too. A chord is where the holonomy is; gauging one away
/// would delete the obstruction rather than move it.
///
/// A path that crosses no real face measured no edge at all and carries
/// `None` there — answering "identity" would report a measurement never
/// taken.
///
/// `None` is a loop naming a face this form does not have, or a member pair
/// the face does not relate: the cycle and the incidence disagree, which is
/// a refusal and not a zero (rule 11).
pub(crate) fn paths_carried(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Loop,
) -> Option<Keyed<PathCarried>> {
    // THE EDGE COMPOSITE OPENS AT THE IDENTITY AND EVERY STEP CONTRIBUTES.
    // It used to open at `None` unless the loop crossed a face between two
    // bodies, because the star faces contributed nothing and a star-only
    // loop would then have reported "identity" for a measurement never
    // taken. Under the hybrid rule below every face carries a transport —
    // the stabilizer between bodies, the member's carriage to the Frame —
    // so a star-only loop measures its own equivocation instead of
    // vanishing, and `None` now means a step whose transport is genuinely
    // unread (rule 11).
    let mut acc = PathCarried {
        face: Transport {
            word: Word::identity(),
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        },
        edge: Some(Transport {
            word: Word::identity(),
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        }),
        charges: Vec::with_capacity(lp.faces.len()),
    };
    let mut charts: Vec<Vec<u8>> = Vec::with_capacity(lp.members.len());
    for (win, &fi) in lp.members.windows(2).zip(&lp.faces) {
        let f = faces.get(fi as usize)?;
        // THE STEP IS NAMED BY ITS TWO MEMBERS. Each carries its vertex and
        // its chart, so the face's pair is read off the members rather than
        // guessed from the face's endpoints — which is the whole of what
        // the projection lost.
        let (vi, ci) = crate::transducer::family::read_member_key(&win[0])?;
        let (vj, cj) = crate::transducer::family::read_member_key(&win[1])?;
        let forward = f.i == vi && f.j == vj;
        let pk = match forward {
            true => pair_key_bytes(&ci, &cj),
            false => pair_key_bytes(&cj, &ci),
        };
        // A face relating NO member pair stands at no named chart, and the
        // members say so too — both charts empty. It contributes its word
        // and nothing else.
        let place = match f.at.get(&pk) {
            Some(p) => Some(p.clone()),
            None if ci.is_empty() && cj.is_empty() => None,
            None => return None,
        };
        charts.push(ci);
        let word_step = match forward {
            true => f.word.clone(),
            false => f.word.inverse(),
        };
        acc.face.word = word_step.compose(&acc.face.word);
        if let Some(place) = &place {
            let (carriage, gauge) = match forward {
                true => (place.carriage.clone(), place.gauge.clone()),
                false => (place.carriage.versor_inverse()?, gauge_invert(&place.gauge)),
            };
            acc.face.carriage = carriage.geo(&acc.face.carriage);
            acc.face.gauge = compose_gauge(&gauge, &acc.face.gauge);
        }
        // THE EDGE HALF IS HYBRID, AND EVERY STEP CONTRIBUTES (54.5, as
        // ruled 2026-09-20). A cycle of the member complex crosses two
        // kinds of face and each has its own carrier:
        //
        //   A STAR FACE anchors a member to the Frame. The Frame carries no
        //   body, so there is no stabilizer to measure — and the transport
        //   is not therefore ABSENT: it is the member's own CARRIAGE, where
        //   that body stands in its chart.
        //
        //   AN INTER-BODY FACE relates two bodies, and its transport is
        //   50.3's measured stabilizer element, `place.edge`.
        //
        // This skipped the star faces entirely, which is not the same as
        // gauging them: §3.6 defines a re-gauging as `g_ij → γ_j g_ij γ_i⁻¹`,
        // a CONJUGATION, and dropping a factor out of a product is a
        // different operation with a different answer. Composing the star
        // carriages IS that conjugation — they are the γ — so the composite
        // is the holonomy in the tree gauge rather than the chord's raw
        // transport mistaken for it.
        //
        // AND IT CATCHES WHAT SKIPPING COULD NOT. Two star faces to one
        // member compose `γ₁ · γ₂⁻¹`, which is an equivocation — one body,
        // two transports from the basepoint — and read as identity-and-
        // identity it was exactly nothing.
        let step = match (touches_frame(vertices, f), place.as_ref()) {
            // THE STAR STEP IS THE FACE'S WHOLE TRANSPORT, word included.
            // §3.6's `γ` is a WORD — a re-gauging is `γ_j · g_ij · γ_i⁻¹`
            // — so a star step that contributed the identity word would
            // erase the word-holonomy of every loop whose bodies measured
            // no stabilizer edge, replacing a real obstruction with
            // nothing. What the face measured is what it contributes.
            (true, Some(p)) => Some(Transport {
                word: f.word.clone(),
                carriage: p.carriage.clone(),
                gauge: p.gauge.clone(),
            }),
            (false, Some(p)) => p.edge.clone(),
            (_, None) => None,
        };
        // THE STEP'S KINEMATIC CHARGE, before the step is folded into the
        // product and its identity is lost. Read off the TRANSVECTION
        // factor this step carries; a step whose transport was never read
        // contributes no charge rather than a zero one, because unread is
        // not flat (rule 11) — and `acc.edge` below goes `None` for it, so
        // the whole boundary is already refused.
        if let Some(t) = step.as_ref() {
            if let Some(w) = crate::transducer::cga::is_versor(&t.carriage) {
                let s = t.carriage.scalar_part();
                let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
                let mu = &s * &s / &w - one;
                note_step_charge(&mu);
                acc.charges.push(mu);
            }
        }
        acc.edge = match (acc.edge.take(), step) {
            (Some(held), Some(step)) => {
                let step = match forward {
                    true => Some(step),
                    false => step.inverse(),
                };
                step.map(|step| Transport {
                    carriage: step.carriage.geo(&held.carriage),
                    word: step.word.compose(&held.word),
                    gauge: compose_gauge(&step.gauge, &held.gauge),
                })
            }
            _ => None,
        };
    }
    acc.face.word = acc.face.word.normal_form();
    if let Some(e) = acc.edge.as_mut() {
        e.word = e.word.normal_form();
    }
    let mut out = Keyed::new();
    out.insert(Loop::path_key(&charts), acc);
    Some(out)
}

/// The face-pair key of two chart keys, as `face_pair_key` writes it from
/// two `Invariant`s — the same bytes, read from the keys the members
/// already carry rather than from invariants this walk does not hold.
fn pair_key_bytes(ci: &[u8], cj: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, ci);
    crate::transducer::address::put_bytes(&mut buf, cj);
    buf
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
    pub fn edge_classes(
        &self,
        lp: &Loop,
    ) -> Keyed<crate::network::nerve::coord::Holonomy> {
        let mut out = Keyed::new();
        let Some(hs) = self.edge_holonomies(lp) else {
            return out;
        };
        for (k, h) in hs.scoped() {
            // 54.7 R4 — the holonomy producer, the same one the torsion
            // and axis carriers are read at.
            if let Some(c) = h.word.rotor().holonomy_key() {
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
        // GRADE 1 IS A GEOMETRIC PREDICATE, NOT A PROGRESS FLAG (54.5, as
        // ruled 2026-09-20). Two defects stood here and they compounded.
        //
        // THE WRONG READER (law 12, the fourth site of one ruling).
        // `loop_classes` states its own contract: "ONE READER FOR THREE
        // CALLERS — `file_loop` writes these rows, `Form::coheres` asks
        // whether they are all compact, and `Form::is_cauterized` asks
        // whether what is filed agrees with them", and it names this exact
        // fault as fixed at the third: "a screw was filed at 0 by the first
        // and demanded at 4 by the third." The unification reached
        // `file_loop`, `loop_is_congruent` and `is_cauterized`. It did not
        // reach here. This read `h.word.order()` — the BARE ROTOR ORDER,
        // with no carriage and no fixed-point test — while `file_loop`
        // derives the period through `cech::classify`, which gates on the
        // fixed point and files a screw at `0`. So a screw was filed at 0
        // and demanded at 4. Never equal, so the form never cohered.
        //
        // And every transport a voice measures arrives turning about a
        // body's own centroid: `versor_coset` builds each member as
        // `T(c_b)·D(ρ)·R·T(−c_a)`, so the translation is always there.
        //
        // COHERENCE IS GEOMETRY AND CONSULTS NO FILING (54.5, as ruled
        // 2026-09-20). Two faults stood here and they compounded.
        //
        // THE WRONG READER. `loop_classes` derives a period through
        // `cech::classify`, which GATES ON THE FIXED POINT and gives a
        // screw `0`; this read `h.word.order()` — the bare rotor order,
        // with no carriage and no fixed-point test — and demanded a filing
        // match THAT. A screw was filed at 0 and demanded at 4. Never
        // equal, so no curved form ever cohered. `loop_classes` names this
        // exact fault as fixed at its third caller — "a screw was filed at
        // 0 by the first and demanded at 4 by the third" — and this was its
        // fourth site (law 12).
        //
        // AND COHERENCE IS NOT A PERMIT. Even with the right reader this
        // demanded that an ENTRY EXIST, which asks whether a stroke has got
        // around to the loop. Every field of a `FiledLoop` is DERIVABLE:
        // `loop_classes` computes the class from `holonomy_words_on`, the
        // period from `cech::classify`, σ from `loop_transport_on`, and the
        // chord is the loop's own. Law 12 says a derivable fact is
        // re-derived and never stored beside the coordinates that determine
        // it — so an annotation's PRESENCE cannot be a geometric property,
        // and a predicate reading it made an open dislocation look like a
        // closed manifold.
        //
        // WHAT COHERES, on the complex and nothing else: identity, a class
        // of FINITE order about a fixed point — `ℤ/ℓℤ`, which is content
        // (law 7) — or an axis the form opened over this loop. A screw
        // walks and never returns: `g^ℓ = T(ℓτ)` with `τ ∈ ker(I − R)`, so
        // there is no period at which the circuit closes. It is not an
        // error and it is not coherence; it is a `Demand::Dislocation`,
        // carried as a vertex, in the key, saying "these fibrations shear
        // by τ and `T(−τ)` would close them".
        //
        // THE FILING IS NOT DELETED, IT IS RELOCATED. `Element.filing` is
        // the speaker's ATTESTATION — what this mind's stroke discharged
        // against its own budget — and it prices the yield (`fibre_volume`,
        // E2) and answers whether the stroke finished (`is_cauterized`).
        // Those are mind-state and they say so. Geometry is asked here.
        if self.opened_axis_of(lp).is_some_and(|chord| self.axis_rederives(lp, chord)) {
            return true;
        }
        let compact = self.loop_classes(lp);
        hs.scoped().all(|(_, h)| {
            if h.word.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
                return true;
            }
            // A holonomy with no class is not a versor (15.3) and coheres
            // nowhere; refused, never keyed by the rotor's bytes.
            let Some(class) = h.word.rotor().holonomy_key() else {
                return false;
            };
            // A path `loop_classes` produced no row for carries no measured
            // transport, and a reading that was not taken holds nothing
            // against the form (rule 11, 13.9 step 7). A row with period 0
            // is the screw: non-compact, and owed a dislocation.
            match compact.iter().find(|row| row.class == class) {
                // COMPACT IS BOTH PLANES CLOSING. It read `period > 0` on
                // the `lcm`, which is non-zero whenever EITHER factor is a
                // root of unity — so a loop with one plane turning forever
                // read as compact and was never owed a dislocation (54.7).
                Some(row) if row.periods.closes() => true,
                // A FREE PART THE FORM DECLARES IS CARRIED, NOT HIDDEN
                // (54.5, Directive 4). `REFUSE_LOOP` reads "a free part and
                // NO DISLOCATION DECLARES IT", and this is the clause that
                // makes the second half true.
                //
                // A circuit that walks is an obstruction, and 50.5's answer
                // to an obstruction is a DEMAND: a vertex, in the key,
                // carried as content, saying "these fibrations shear by τ
                // and `T(−τ)` would close them". That is the opposite of a
                // filing — a filing is a side-table entry about a theorem
                // the geometry already determines, and a demand is a
                // coordinate the address names.
                //
                // So the form may carry its shear honestly and still
                // cohere, and `is_closed` is what keeps the distinction:
                // a sentence with a demand is VALID, never closed (50.1).
                Some(_) => self.declares_dislocation(&class),
                None => true,
            }
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
