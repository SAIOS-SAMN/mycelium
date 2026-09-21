// 54.3, 54.5 — THE COORDINATE MAP IS A FAITHFUL RETRACT, ON FORMS THE
// ENGINE BUILT.
//
//! `from_address(address(f)) ≡ f` is asserted in
//! `sentence::tests::properties` over FIXTURES — forms a test wrote. This
//! asserts it over forms the EXTRACTOR built from the corpus on disk, which
//! is a different claim: a fixture is a shape somebody chose, and a corpus
//! form is whatever the scene turned out to be.
//!
//! IT IS THE GUARD FOR THE TAPE PURGE (54.5). `Element.description` is
//! about to go and `from_address` becomes the only reader, so the tape
//! stops being available as a second opinion. It has been one all session:
//! the `v11`/`v13` fallthrough and the 50× false genome reading were both
//! caught because two decoders disagreed. Before removing the oracle, the
//! property it was checking has to hold on real data.
//!
//! WHAT IS COMPARED is the whole complex, not the bytes: the vertex kinds
//! and count, the faces with their ends and member pairs, the member
//! cycles, and each loop's holonomy word up to projective equality (§1.5 —
//! byte-exactness is not identity). A decode that "succeeds" and returns a
//! poorer complex is the failure this exists to catch.

use crate::transducer::sentence::{form_vertex_key, from_address, Form};

/// The corpus this build reads. A tree without it skips rather than
/// passing: an assertion over no data is not a reading (rule 11).
const CORPUS: &str = "data/arc-agi-2/training";

/// How many tasks to extract. The property is per-form and the count is
/// only how much of the corpus one gate run affords.
const TASKS: usize = 40;

/// The two forms agree as COMPLEXES, compared ORDER-INDEPENDENTLY.
///
/// `form_vertex_key` writes the vertices in `canonical_vertices` order —
/// the Frame, then each vertex by its own coordinate bytes — so
/// `from_address` returns them canonically while the extractor's `f` holds
/// them in the order the scene built them. Comparing index by index asserts
/// an order the key deliberately removes, and it is the ADDRESS that says
/// two forms are one (§1.5).
///
/// So the address equality below is the identity test, and this is the
/// guard against the other failure: a decode that SUCCEEDS and hands back a
/// poorer complex. Counts, the multiset of vertex kinds, the chains the key
/// carries as another object's address, and the bundle's own coordinates.
fn same_complex(a: &Form, b: &Form) -> Result<(), String> {
    use crate::transducer::sentence::Vertex;
    if a.vertices.len() != b.vertices.len() {
        return Err(format!("vertices {} vs {}", a.vertices.len(), b.vertices.len()));
    }
    let kinds = |f: &Form| -> [usize; 6] {
        let mut n = [0usize; 6];
        for v in &f.vertices {
            let i = match v {
                Vertex::Frame => 0,
                Vertex::Word { .. } => 1,
                Vertex::Glued { .. } => 2,
                Vertex::Rebased { .. } => 3,
                Vertex::Unfolded { .. } => 4,
                Vertex::Demand { .. } => 5,
            };
            n[i] += 1;
        }
        n
    };
    if kinds(a) != kinds(b) {
        return Err(format!("vertex kinds {:?} vs {:?}", kinds(a), kinds(b)));
    }
    let members = |f: &Form| -> usize { f.vertices.iter().map(|v| v.charts().len()).sum() };
    if members(a) != members(b) {
        return Err(format!("members {} vs {}", members(a), members(b)));
    }
    if a.faces.len() != b.faces.len() {
        return Err(format!("faces {} vs {}", a.faces.len(), b.faces.len()));
    }
    let pairs = |f: &Form| -> usize { f.faces.iter().map(|x| x.at.len()).sum() };
    if pairs(a) != pairs(b) {
        return Err(format!("member pairs {} vs {}", pairs(a), pairs(b)));
    }
    let edges = |f: &Form| -> usize {
        f.faces.iter().map(|x| x.at.scoped().filter(|(_, p)| p.edge.is_some()).count()).sum()
    };
    if edges(a) != edges(b) {
        return Err(format!("faces carrying 50.3's edge {} vs {}", edges(a), edges(b)));
    }
    if a.loops.len() != b.loops.len() {
        return Err(format!("loops {} vs {}", a.loops.len(), b.loops.len()));
    }
    // THE TWO CHAIN-ADDRESS SITES. A `Demand`'s bodies and the A5 remainder
    // are the only coordinates the key carries as another object's ADDRESS,
    // so they are the two places a round-trip can return a different chain.
    // Sorted, because the vertex order is not preserved and a demand's
    // identity is its bodies rather than its slot.
    let bodies = |f: &Form| -> Vec<usize> {
        let mut n: Vec<usize> = f
            .vertices
            .iter()
            .filter_map(|v| match v {
                Vertex::Demand { bodies, .. } => Some(bodies.len()),
                _ => None,
            })
            .collect();
        n.sort_unstable();
        n
    };
    if bodies(a) != bodies(b) {
        return Err(format!("demand bodies {:?} links vs {:?}", bodies(a), bodies(b)));
    }
    if a.remainder.placed() != b.remainder.placed() {
        return Err(format!(
            "the A5 remainder differs: {} links vs {}",
            a.remainder.placed().len(),
            b.remainder.placed().len()
        ));
    }
    if a.sections.len() != b.sections.len() {
        return Err(format!("sections {} vs {}", a.sections.len(), b.sections.len()));
    }
    if a.axes.len() != b.axes.len() {
        return Err(format!("axes {} vs {}", a.axes.len(), b.axes.len()));
    }
    if a.ratio != b.ratio {
        return Err(format!("the frame ratio differs: {:?} vs {:?}", a.ratio, b.ratio));
    }
    Ok(())
}

/// `from_address(address(f)) ≡ f` on every form the extractor builds from
/// the corpus, and the address re-derives from the recovery.
#[test]
fn from_address_retracts_every_corpus_form() {
    let Ok(dir) = std::fs::read_dir(CORPUS) else {
        eprintln!("  retract: {CORPUS} does not read — no corpus, no reading");
        return;
    };
    let mut names: Vec<String> = dir
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.ends_with(".json"))
        .collect();
    // COORDINATE ORDER, never the directory's: a walk that depends on the
    // filesystem's ordering reads a different slice on every machine.
    names.sort_unstable();
    names.truncate(TASKS);
    assert!(!names.is_empty(), "the corpus directory holds no tasks");

    let epi = crate::client::sdk::Epigenome::new();
    let mut physics = crate::physics::Physics::default();
    let (mut seen, mut with_loops) = (0usize, 0usize);
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget =
            physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let Some(rule) = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget)
        else {
            continue;
        };
        let f = &rule.form;
        seen += 1;
        if !f.loops.is_empty() {
            with_loops += 1;
        }
        let key = form_vertex_key(f);
        let Some(back) = from_address(&key) else {
            panic!("{name}: a form's own address does not decode")
        };
        if let Err(why) = same_complex(f, &back) {
            panic!("{name}: the recovery is not the form — {why}");
        }
        // THE FIRST DIFFERING BYTE, not the whole address. A key is tens of
        // kilobytes and printing two of them says only that they differ;
        // the offset says WHERE, which is the only part that locates a
        // reader against its writer.
        let again = form_vertex_key(&back);
        if again != key {
            let (a, b) = (key.as_bytes(), again.as_bytes());
            let at = (0..a.len().min(b.len())).find(|&i| a[i] != b[i]);
            // The bytes AROUND the difference, and `lo` is a subtraction
            // that cannot go below zero rather than one floored there
            // (rule 11: a floor at zero is an invented value).
            let window = |v: &[u8], i: usize| {
                let lo = match i > 40 {
                    true => i - 40,
                    false => 0,
                };
                let hi = (i + 24).min(v.len());
                format!("{:?}", &v[lo..hi])
            };
            panic!(
                "{name}: the recovery does not re-derive its address — \
                 wrote {} bytes, re-wrote {}, first difference at {:?}\n  was  {}\n  now  {}",
                a.len(),
                b.len(),
                at,
                at.map_or_else(|| "(one is a prefix of the other)".into(), |i| window(a, i)),
                at.map_or_else(|| "(one is a prefix of the other)".into(), |i| window(b, i)),
            );
        }
    }
    // THE READING THE TEST TOOK, so a pass over nothing cannot look like a
    // pass over the corpus (rule 13).
    eprintln!("  retract: {seen} corpus forms round-tripped, {with_loops} of them carrying a loop");
    assert!(seen > 0, "no task of the slice extracted a rule; the property ran over nothing");
}

/// 54.5 PHASE A — THE PREDICATE GUARD: the address is a retract for what
/// the ENGINE asks of a form, not merely for its own key.
///
/// `from_address_retracts_every_corpus_form` asserts a fixed point of the
/// key map — `key(from_address(key)) == key` — and that is weaker than
/// what the purge needs. Every failure of the two attempts at it was
/// DOWNSTREAM of that: a decoded form whose depth was zero, whose citation
/// carried a default chart, whose member carriage matched only up to a
/// rational scale, whose face word came off a presentation rather than an
/// action. The key round-tripped through all four.
///
/// So this compares the two decoders — the tape's `decode_form_v2` and the
/// address's `from_address` — through the readings the court actually
/// makes. While both exist the tape is an oracle, and this is the
/// deliberate form of the accidental one it has been all session.
///
/// It is the precondition for deleting `Element.description`: what the
/// tape has been doing by redundancy, this does by assertion.
#[test]
fn the_address_and_the_tape_agree_on_every_reading_the_court_makes() {
    let Ok(dir) = std::fs::read_dir(CORPUS) else {
        eprintln!("  predicate guard: {CORPUS} does not read");
        return;
    };
    let mut names: Vec<String> = dir
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort_unstable();
    names.truncate(TASKS);

    let epi = crate::client::sdk::Epigenome::new();
    let mut physics = crate::physics::Physics::default();
    let (mut seen, mut diverged) = (0usize, 0usize);
    let mut why: crate::network::nerve::Keyed<u64> = crate::network::nerve::Keyed::new();
    let note = |k: &str, why: &mut crate::network::nerve::Keyed<u64>| {
        match why.get_mut(k.as_bytes()) {
            Some(n) => *n += 1,
            None => {
                why.insert(k.as_bytes(), 1);
            }
        }
    };
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget =
            physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let Some(rule) = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget)
        else {
            continue;
        };
        let f = &rule.form;
        let Some(back) = from_address(&form_vertex_key(f)) else {
            note("the address does not decode", &mut why);
            diverged += 1;
            continue;
        };
        seen += 1;
        let mut bad = false;
        // EVERY FACE RE-DERIVES FROM ITS OWN VERTICES, which is what
        // `face_rederives` asks and what refused 28 tests. Asked here of
        // the DECODED form, since that is the one the court would hold.
        for face in &back.faces {
            if back.face_word(face.i, face.j).as_ref() != Some(&face.word) {
                note("a face word does not re-derive", &mut why);
                bad = true;
                break;
            }
        }
        // THE BUNDLE'S AUDIT, the closure test, and the cycles' holonomy —
        // the three readings a court takes after the faces.
        if f.rank().map(|r| (r.torsion, r.free, r.offplane, r.axes))
            != back.rank().map(|r| (r.torsion, r.free, r.offplane, r.axes))
        {
            note("the rank audit differs", &mut why);
            bad = true;
        }
        if f.is_closed() != back.is_closed() {
            note("closure differs", &mut why);
            bad = true;
        }
        if f.depth_of() != back.depth_of() {
            note("the citation depth differs", &mut why);
            bad = true;
        }
        if f.loop_words().len() != back.loop_words().len() {
            note("the loops carry a different number of member paths", &mut why);
            bad = true;
        }
        if bad {
            diverged += 1;
        }
    }
    eprintln!("  predicate guard: {seen} corpus forms, {diverged} diverged");
    for (k, n) in why.scoped() {
        eprintln!("    {n:>4}  {}", String::from_utf8_lossy(&k));
    }
    assert!(seen > 0, "the guard ran over nothing");
    assert_eq!(diverged, 0, "the address and the tape must agree on every reading");
}

/// GRADE 0 ON THE CORPUS — how many vertices are not one 0-cell, and does
/// that account for what `face_rederives` has been refusing (L37).
///
/// This is an INSTRUMENT and not a gate (§8.1). `body_coheres` is a new
/// predicate and the first thing owed of one is a measurement: how much it
/// refuses, where, and whether the number explains a refusal already on the
/// board. L37 reads `face does not re-derive from its vertices` at 192 on
/// voices and 22 on hyphae in 2 h 10 m, cause unverified.
///
/// THE PREDICTION BEING TESTED. `face_rederives` re-derives a face at EVERY
/// member pair it records, `face_place(i, &c_i, j, &c_j, …)`. If a vertex's
/// own fibre is twisted — two members turning relative to one another in
/// their own charts — those per-pair places disagree, and the refusal is
/// reported at the FACE because that is where the reader stands. A missing
/// grade-0 predicate attributes its faults to grade 1. So a form with an
/// incoherent body should be a form whose faces do not all re-derive, and a
/// form with none should not fail there for this reason.
///
/// It asserts only what it can prove from the corpus in front of it; the
/// counts are printed so the reading is the reading and not a claim about
/// the fleet.
#[test]
fn grade_zero_over_the_corpus_and_what_it_accounts_for() {
    let Ok(dir) = std::fs::read_dir(CORPUS) else {
        eprintln!("  grade 0: {CORPUS} does not read — no corpus, no reading");
        return;
    };
    let mut names: Vec<String> = dir
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort_unstable();
    names.truncate(TASKS);
    assert!(!names.is_empty(), "the corpus directory holds no tasks");

    let epi = crate::client::sdk::Epigenome::new();
    let mut physics = crate::physics::Physics::default();
    let (mut forms, mut vertices, mut fibred, mut incoherent) = (0usize, 0usize, 0usize, 0usize);
    let (mut forms_with_incoherent, mut widest) = (0usize, 0usize);
    let (mut standing, mut unspellable, mut other, mut slipping) = (0usize, 0usize, 0usize, 0usize);
    // KEYED, NEVER A BTREE (the standing rule, and rule 15 under it): a
    // `Keyed` is a PATRICIA trie over the key's own bytes, so the walk order
    // IS the coordinate's order. The period is written big-endian so the
    // trie's order is the period's order and nothing sorts.
    let mut compact: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget =
            physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let Some(rule) = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget)
        else {
            continue;
        };
        let f = &rule.form;
        forms += 1;
        for v in &f.vertices {
            let Some(acts) = v.acts() else { continue };
            vertices += 1;
            let n = acts.scoped().count();
            widest = widest.max(n);
            // A FIBRE WITH ONE MEMBER HAS NO VERTICAL TRANSITION, so it is
            // not evidence either way and is counted apart.
            if n > 1 {
                fibred += 1;
            }
        }
        // THE CLASSES, NOT A COUNT OF REFUSALS (rule 4). Each is an exact
        // conformal element; what is tallied is how it CLASSIFIES, because
        // a compact one is a coordinate the pivot can open and a standing
        // one is a parallax the form has not absorbed.
        for (_, t, fill) in f.gauge_obstructions() {
            forms_with_incoherent += 1;
            incoherent += 1;
            match fill {
                crate::transducer::cech::Fill::Congruent(_, _, ell) => {
                    let at = match compact.get(&ell.to_be_bytes()) {
                        Some(n) => *n + 1,
                        None => 1,
                    };
                    compact.insert(ell.to_be_bytes(), at);
                }
                crate::transducer::cech::Fill::Class(..) => standing += 1,
                crate::transducer::cech::Fill::Unspellable(..) => unspellable += 1,
                _ => other += 1,
            }
            if let Some(s) = t.screw() {
                if !s.twist.iter().all(num_traits::Zero::is_zero) {
                    slipping += 1;
                }
            }
        }
    }
    eprintln!(
        "  grade 0: {forms} forms, {vertices} vertices with a fibre, {fibred} standing on more \
         than one chart (widest {widest}); {incoherent} vertices are not one 0-cell, across \
         {forms_with_incoherent} transitions"
    );
    let compact_rows: Vec<(u32, usize)> = compact
        .scoped()
        .filter_map(|(k, n)| {
            let b: [u8; 4] = k.as_slice().try_into().ok()?;
            Some((u32::from_be_bytes(b), *n))
        })
        .collect();
    eprintln!(
        "  grade 0 classes: compact {compact_rows:?} · standing {standing} · unspellable \
         {unspellable} · other {other} · with a fibre slip {slipping}"
    );
    // WHAT IS ASSERTED: a vertex standing on ONE chart has no vertical
    // transition and cannot be incoherent, so the incoherent count is
    // bounded by the multi-chart count. If that ever fails the predicate is
    // reading something other than a fibre.
    assert!(
        incoherent <= fibred,
        "a vertex on one chart has no vertical transition to be twisted: \
         {incoherent} twisted of {fibred} fibred"
    );
}

/// THE CORPUS'S SPECTRUM — what the engine actually sounds like, note by
/// note, with nothing summed (54.5).
///
/// `coheres() -> bool` answered one bit for a whole form and the fleet read
/// `REFUSE_NOT_CLOSED` 36 times in 39 refusals with no way to ask what was
/// wrong. This prints the chord: the plane periods of every cycle
/// UNCOLLAPSED, the gauge cycle types of every fibre, and the shears no
/// origin absorbs — separately, because a lane is never summed into another
/// (law 5).
///
/// It asserts nothing about how much should ring. It is an instrument and
/// the numbers are the reading (§8.1).
#[test]
fn the_corpus_spectrum() {
    use crate::transducer::alphabet::Order;
    let Ok(dir) = std::fs::read_dir(CORPUS) else {
        eprintln!("  spectrum: {CORPUS} does not read — no corpus, no reading");
        return;
    };
    let mut names: Vec<String> = dir
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort_unstable();
    names.truncate(TASKS);

    let epi = crate::client::sdk::Epigenome::new();
    let mut physics = crate::physics::Physics::default();
    let (mut forms, mut cycles, mut both_compact, mut one_plane_only) = (0usize, 0usize, 0usize, 0usize);
    let (mut neither, mut slips, mut undeclared, mut gauge) = (0usize, 0usize, 0usize, 0usize);
    // KEYED, NEVER A BTREE (the standing rule): the period big-endian, so
    // the trie's order is the period's.
    let mut periods: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget =
            physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let Some(rule) = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget)
        else {
            continue;
        };
        forms += 1;
        let s = rule.form.spectrum();
        gauge += s.gauge_cycles.len();
        slips += s.dislocations.len();
        undeclared += s.undeclared().len();
        for c in &s.cycles {
            cycles += 1;
            let fin = |o: &Order| matches!(o, Order::Finite(l) if *l > 0);
            match (fin(&c.planes[0]), fin(&c.planes[1])) {
                (true, true) => both_compact += 1,
                (false, false) => neither += 1,
                // THE READING THE lcm DESTROYS: one plane rings and the
                // other walks. `order()` answers `Infinite` for this and
                // the compact half is lost.
                _ => one_plane_only += 1,
            }
            for p in &c.planes {
                if let Order::Finite(l) = p {
                    let at = match periods.get(&l.to_be_bytes()) {
                        Some(n) => *n + 1,
                        None => 1,
                    };
                    periods.insert(l.to_be_bytes(), at);
                }
            }
        }
    }
    eprintln!(
        "  spectrum: {forms} forms · {cycles} evaluated cycles — both planes compact \
         {both_compact}, one plane only {one_plane_only}, neither {neither}"
    );
    let rows: Vec<(u32, usize)> = periods
        .scoped()
        .filter_map(|(k, n)| {
            let b: [u8; 4] = k.as_slice().try_into().ok()?;
            Some((u32::from_be_bytes(b), *n))
        })
        .collect();
    eprintln!("  spectrum: plane periods {rows:?}");
    eprintln!(
        "  spectrum: gauge cycles {gauge} · dislocations {slips} ({undeclared} undeclared)"
    );
}

/// WHERE THE SILENCE BEGINS (54.5). The spectrum reads zero on every note
/// over the corpus; this says which step produced nothing, because "no
/// curvature" has three different causes and three different repairs.
///
///   NO LOOPS — the complex is a tree. There is no 1-cycle, so there is
///   nothing to be flat OR curved: a single fibration between two nodes
///   carries no constraint. The repair is more incidence, not a better
///   classifier.
///
///   LOOPS, NO EVALUATED PATH — `edge_holonomies` answers `None` or an
///   empty listing: a remainder edge lies on the cycle and the transport
///   between two bodies was never measured (13.9 step 7). The repair is at
///   the EDGE measurement — 50.3's stabilizer — and the classifier would
///   have nothing to say however it were written.
///
///   PATHS, ALL IDENTITY — the transport composes to `1` around every
///   cycle. The cover is genuinely flat and the reading is honest.
#[test]
fn where_the_corpus_goes_silent() {
    let Ok(dir) = std::fs::read_dir(CORPUS) else {
        eprintln!("  silence: {CORPUS} does not read");
        return;
    };
    let mut names: Vec<String> = dir
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort_unstable();
    names.truncate(TASKS);

    let epi = crate::client::sdk::Epigenome::new();
    let mut physics = crate::physics::Physics::default();
    let (mut forms, mut with_loops, mut loops, mut faces, mut verts) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let (mut unread, mut no_path, mut paths, mut ident, mut live) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let (mut faces_with_edge, mut word_live) = (0usize, 0usize);
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget =
            physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let Some(rule) = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget)
        else {
            continue;
        };
        let f = &rule.form;
        forms += 1;
        verts += f.vertices.len();
        faces += f.faces.len();
        faces_with_edge += f.faces.iter().filter(|x| x.has_edge()).count();
        if !f.loops.is_empty() {
            with_loops += 1;
        }
        for lp in &f.loops {
            loops += 1;
            // THE WORD HALF, for comparison: the transport composite.
            if !lp.holonomy.is_identity() {
                word_live += 1;
            }
            match f.edge_holonomies(lp) {
                None => unread += 1,
                Some(hs) if hs.scoped().next().is_none() => no_path += 1,
                Some(hs) => {
                    for (_, h) in hs.scoped() {
                        paths += 1;
                        if h.word.rotor().projective_eq(
                            &crate::transducer::alphabet::Rotor::identity(),
                        ) {
                            ident += 1;
                        } else {
                            live += 1;
                        }
                    }
                }
            }
        }
    }
    eprintln!(
        "  silence: {forms} forms · {verts} vertices · {faces} faces ({faces_with_edge} carrying a \
         50.3 edge) · {with_loops} forms with loops · {loops} loops"
    );
    eprintln!(
        "  silence: loops — unread {unread} · no evaluated path {no_path} · with paths {} ; \
         paths {paths} — identity {ident}, live {live}; word holonomy non-identity {word_live}",
        loops - unread - no_path
    );
}

/// 54.5 PHASE A — THE DUAL-READER PARITY ORACLE, at the four readers the
/// purge is about (as ruled 2026-09-21).
///
/// `the_address_and_the_tape_agree_on_every_reading_the_court_makes`
/// compares the two DECODERS on the geometry: faces, rank, closure, depth,
/// loops. That is necessary and not sufficient. What B6 deletes is
/// `Element.description`, and what has to hold before it goes is that no
/// COURT VERDICT depends on which carrier produced the form:
///
/// ```text
///                      one spoken Element
///                              │
///          ┌───────────────────┴───────────────────┐
///          ▼                                       ▼
///   description → decode_form_v2           address → from_address
///   (the legacy tape)                      + ReceiptsEnvelope
///          │                                       │
///          ├──────────┬────────────┬───────────────┤
///          ▼          ▼            ▼               ▼
///  derivations  certificates  unruled cite   replay_form verdict
///          └──────────┴────────────┴───────────────┘
///                              ▼
///                   assert_eq!(legacy, decoupled)
/// ```
///
/// THE SEAM IS `replay_form_on`. `replay_form_at` decodes internally, so
/// without lifting that out this guard would feed the decoupled reader to
/// itself and assert nothing. The decode is named once and the verdict is
/// a function of the form it is handed.
///
/// `unruled_citation` is on the envelope already and has no form in it, so
/// its parity is the one that can be stated as an equality of SETS: the
/// addresses the envelope's edges name are exactly the addresses the legacy
/// tape's citations named. That is the whole content of "no hidden
/// semantics depend on the description" for the citation reader.
///
/// Over the corpus AND the fixtures, because they fail differently: a
/// fixture is a shape somebody chose and is where adjudication actually
/// executes today, and a corpus form is whatever the scene turned out to
/// be. The two attempts at B4 that failed were caught by fixtures, not by
/// the fleet.
#[test]
fn every_court_reader_agrees_on_both_decoders() {
    use crate::transducer::sentence::as_element_form;
    let no_resolve = |_: &[u8]| None;

    // THE TWO PATHS, off one word. `legacy` is the tape under its version
    // gate — the same decision `vertex_address` still makes — and
    // `decoupled` is the address plus the word's own envelope.
    let both = |e: &crate::transducer::element::Element| -> Option<(Form, Form)> {
        let b = e.description();
        if b.get(1).copied() != Some(crate::transducer::sentence::FORM_TAPE_VERSION) {
            return None;
        }
        let mut pos = 2usize;
        let legacy = crate::transducer::sentence::decode_form_v2(b, &mut pos)?;
        let decoupled = crate::transducer::sentence::from_element_form(e)?;
        Some((legacy, decoupled))
    };

    let mut checked = 0usize;
    let mut with_cites = 0usize;
    let (mut site_total, mut site_arity1) = (0usize, 0usize);
    let mut reader3 = 0usize;
    let mut arity_gt1 = 0usize;
    let mut forms: Vec<Form> = Vec::new();

    // THE CORPUS: whatever the scene turned out to be.
    if let Ok(dir) = std::fs::read_dir(CORPUS) {
        let mut names: Vec<String> = dir
            .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
            .filter(|n| n.ends_with(".json"))
            .collect();
        names.sort_unstable();
        names.truncate(TASKS);
        let epi = crate::client::sdk::Epigenome::new();
        let mut physics = crate::physics::Physics::default();
        for name in &names {
            let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
            let Some(task) = crate::arc::parse_task(&json) else { continue };
            let mut budget =
                physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
            if let Some(rule) =
                crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget)
            {
                forms.push(rule.form);
            }
        }
    }

    // THE FIXTURES, BUILT ON THE CORPUS FORMS. A corpus form is whatever
    // the scene turned out to be, and the extractor's own output cites
    // rarely; the readers the purge is about are the CITATION readers, so
    // the fixtures are hosts that cite — one citation, two citations of
    // two different forms at one site (gauge universality), and a nested
    // host that cites a citing form. Built rather than hand-written so
    // they carry real geometry and not a shape chosen to pass.
    let cited: Vec<(Form, crate::transducer::address::Address)> = forms
        .iter()
        .filter(|f| f.is_closed())
        .take(2)
        .filter_map(|f| as_element_form(f, [1, 0, 0, 0]).map(|e| (f.clone(), e.address())))
        .collect();
    if let [(a, a_at), (b, b_at)] = cited.as_slice() {
        let identity = crate::transducer::alphabet::Word::identity();
        let mut one = Form::default();
        if one.glue(a, a_at.clone(), identity.clone(), [0, 0, 0, 1]).is_some() {
            let mut two = one.clone();
            if two.glue(b, b_at.clone(), identity.clone(), [0, 0, 0, 1]).is_some() {
                if let Some(one_e) = as_element_form(&one, [1, 0, 0, 0]) {
                    let mut nested = Form::default();
                    if nested.glue(&one, one_e.address(), identity, [0, 0, 0, 1]).is_some() {
                        forms.push(nested);
                    }
                }
                forms.push(two);
            }
            forms.push(one);
        }
    }

    for f in &forms {
        let Some(e) = as_element_form(f, [1, 0, 0, 0]) else { continue };
        let env = e.receipts();

        // THE ORACLE'S DOMAIN, AND THE PARTITION IS THE RULING (54.7, as
        // ruled 2026-09-21). The legacy tape's citation record is per
        // `Glued` VERTEX; the cover is per INCIDENCE. Before B5 those
        // arities were equal because the target sat on the vertex. After it
        // a site can carry several edges — gauge universality, and what B5
        // was for — so the tape is structurally an ARITY-1 CODEC.
        //
        // Demanding that it act as a faithful second opinion for a
        // multi-sheeted cover demands that a dead format anticipate the
        // feature designed to replace it. So the oracle is consulted where
        // it has a domain, and the multi-sheeted fixtures assert their own
        // integrity instead — retraction and envelope, with no legacy side.
        let mut sites: crate::network::nerve::Keyed<u64> = crate::network::nerve::Keyed::new();
        for x in &env.nerve {
            let h = x.host();
            match sites.get_mut(h.as_bytes()) {
                Some(n) => *n += 1,
                None => {
                    sites.insert(h.as_bytes(), 1);
                }
            }
        }
        let mut multi = false;
        for (_, n) in sites.scoped() {
            site_total += 1;
            if *n == 1 {
                site_arity1 += 1;
            } else {
                multi = true;
            }
        }
        if multi {
            arity_gt1 += 1;
            // RETRACTION PARITY: the key is a fixed point of the decoder.
            let key = form_vertex_key(f);
            let back = from_address(&key).expect("a multi-sheeted form decodes from its key");
            assert_eq!(form_vertex_key(&back), key, "key(from_address(key)) == key");
            // ENVELOPE INTEGRITY: every edge stands at a port and every
            // port carries a bond, and the sheets at a site are distinct.
            for x in &env.nerve {
                let host = x.host();
                assert!(
                    back.vertices.iter().any(|v| {
                        matches!(v, crate::transducer::sentence::Vertex::Glued { .. })
                            && crate::transducer::sentence::vertex_bytes(v) == host.as_bytes()
                    }),
                    "no phantom edge: every citation stands at a boundary the complex holds"
                );
            }
            for v in &back.vertices {
                if !matches!(v, crate::transducer::sentence::Vertex::Glued { .. }) {
                    continue;
                }
                let site = crate::transducer::sentence::vertex_bytes(v);
                let mut sheets: crate::network::nerve::Keyed<()> =
                    crate::network::nerve::Keyed::new();
                let mut n = 0usize;
                for x in env.nerve.iter().filter(|x| x.host().as_bytes() == site) {
                    n += 1;
                    sheets.insert(&x.target, ());
                }
                assert!(n > 0, "no orphan port: every boundary carries a citation");
                assert_eq!(sheets.len(), n, "the sheets at a site are distinct targets");
            }
            checked += 1;
            if !env.nerve.is_empty() {
                with_cites += 1;
            }
            continue;
        }

        let Some((legacy, decoupled)) = both(&e) else { continue };
        checked += 1;

        // READER 1 — `derivations_compose`. The evidence is the envelope's
        // either way; what is under test is whether the FORM it is checked
        // against changes the verdict.
        assert_eq!(
            crate::verdict::derivations_compose_at(&legacy, env, &no_resolve).map(|r| r.reason),
            crate::verdict::derivations_compose_at(&decoupled, env, &no_resolve).map(|r| r.reason),
            "derivations compose to the same verdict on both decoders"
        );

        // READER 2 — `replay_certificates`, each certificate un-pivoted
        // against the chord face it names. The chord is an index into
        // `faces`, so a decoder that reordered faces would diverge here and
        // nowhere else.
        assert_eq!(
            crate::verdict::replay_certificates_at(&legacy, &env.certificates),
            crate::verdict::replay_certificates_at(&decoupled, &env.certificates),
            "the certificates replay to the same verdict on both decoders"
        );

        // READER 3 — the unruled citation's ADDRESS SET. The envelope names
        // the targets; the legacy tape named them on its `Glued` records.
        // Equality of the two sets is what "nothing depends on the
        // description" means for the citation reader.
        // ARITY 1, SO THE TAPE IS A FAITHFUL SECOND OPINION and the
        // citation sets are compared outright.
        let from_legacy: Vec<Vec<u8>> =
            legacy.cover.iter().map(|x| x.target.as_bytes().to_vec()).collect();
        // ONE PER SITE, IN THE COVER'S OWN ORDER. `env.nerve` is already
        // canonical, so the first edge at each host is the one the tape's
        // per-vertex `find` reaches.
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let mut from_env_one: Vec<Vec<u8>> = Vec::new();
        for x in &env.nerve {
            let h = x.host();
            if seen.get(h.as_bytes()).is_none() {
                seen.insert(h.as_bytes(), ());
                from_env_one.push(x.target.as_bytes().to_vec());
            }
        }
        if from_env_one != from_legacy {
            reader3 += 1;
        }
        if !env.nerve.is_empty() {
            with_cites += 1;
        }

        // READER 4 — THE WHOLE VERDICT, every return variant.
        assert_eq!(
            crate::verdict::replay_form_on(
                &legacy,
                &e,
                &no_resolve,
                None,
                &crate::transducer::family::NoCrystal
            )
            .map(|r| r.reason),
            crate::verdict::replay_form_on(
                &decoupled,
                &e,
                &no_resolve,
                None,
                &crate::transducer::family::NoCrystal
            )
            .map(|r| r.reason),
            "the form replays to the same verdict on both decoders"
        );
    }

    // THE READING THE TEST TOOK (rule 13): a pass over nothing is not a
    // pass, and a pass over nothing that CITES would not exercise the
    // reader the purge is about.
    eprintln!("  parity: {checked} words on both decoders, {with_cites} of them citing");
    eprintln!(
        "  cover arity: {site_total} sites, {site_arity1} of arity 1, {} of arity >1; \
         {arity_gt1} words multi-sheeted and audited without the oracle",
        site_total - site_arity1
    );
    assert!(checked > 0, "the parity oracle ran over no word");
    assert!(with_cites > 0, "the parity oracle saw no citation, so reader 3 asserted nothing");
    assert_eq!(reader3, 0, "the tape names the envelope's citations, one per site");
}
