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
        let Some(back) = from_address(key.as_bytes()) else {
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
        let Some(back) = from_address(form_vertex_key(f).as_bytes()) else {
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
