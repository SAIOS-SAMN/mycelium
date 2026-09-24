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
//! cycles, and each orbit's holonomy word up to projective equality (§1.5 —
//! byte-exactness is not identity). A decode that "succeeds" and returns a
//! poorer complex is the failure this exists to catch.

use crate::transducer::sentence::{form_vertex_key, from_address, Form};

/// The corpus this build reads. A tree without it skips rather than
/// passing: an assertion over no data is not a reading (rule 11).
pub(super) const CORPUS: &str = "data/arc-agi-2/training";

/// How many tasks to extract. The property is per-form and the count is
/// only how much of the corpus one gate run affords.
pub(super) const TASKS: usize = 40;

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
    if a.orbits.len() != b.orbits.len() {
        return Err(format!("orbits {} vs {}", a.orbits.len(), b.orbits.len()));
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
    let (mut seen, mut with_orbits) = (0usize, 0usize);
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
        if !f.orbits.is_empty() {
            with_orbits += 1;
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
    eprintln!("  retract: {seen} corpus forms round-tripped, {with_orbits} of them carrying an orbit");
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
/// So this compares the form the extractor made with the form its ADDRESS
/// decodes to — `from_address(form_vertex_key(f))` — through the readings
/// the court actually makes: every face re-derived, the rank audit,
/// closure, the citation depth and the orbits' member paths. It compared
/// the tape's decoder with the address's while the tape stood; the tape
/// and its codec are retired (54.7 B6, LD3-c), and the address is the one
/// decoder the court holds.
#[test]
fn the_decoded_form_agrees_with_the_extracted_on_every_reading_the_court_makes() {
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
        if f.orbit_words().len() != back.orbit_words().len() {
            note("the orbits carry a different number of member paths", &mut why);
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
    assert_eq!(diverged, 0, "the decoded form and the extracted must agree on every reading");
}

/// 54.7 B6 — THE ORACLE IS SPENT, AND THESE ARE THE INVARIANTS THAT STAND
/// WITHOUT IT (as ruled 2026-09-21).
///
/// `every_court_reader_agrees_on_both_decoders` stood here. It fed the
/// legacy tape's decoder (retired with its codec at LD3-c) and the
/// address's `from_address` +
/// `ReceiptsEnvelope` into the four readers the purge was about and
/// asserted one verdict, over corpus and fixtures. It passed at `bceeb56`:
/// 39 words on both decoders, 3 citing, 3 cover sites — 2 of arity 1
/// through all four readers, 1 multi-sheeted audited without the oracle,
/// because the tape's citation record is per VERTEX and the cover is per
/// INCIDENCE, so a dead format cannot be asked to anticipate the feature
/// that replaces it.
///
/// B6 deletes `Element.description` for sentences, so there is no second
/// decoder left to consult and the comparison would assert nothing. What
/// replaces redundancy is three SELF-CONTAINED invariants, each a property
/// of the one remaining carrier:
///
/// ```text
///   1  retraction      from_address(address(F)) ≡ F
///   2  dual integrity  every port has a bond, every bond has a port,
///                      and the sheets at one site are distinct
///   3  round stability Ω S Ω̃ = ±S, on the hyperboloid, terminating
///                      at grade 5 by ⋀⁶(ℚ^{4,1}) ≡ 0
/// ```
///
/// Over the corpus AND the fixtures, because they fail differently: a
/// fixture is a shape somebody chose and is where adjudication executes
/// today, and a corpus form is whatever the scene turned out to be.
#[test]
fn the_carrier_holds_its_own_invariants_without_the_oracle() {
    use crate::transducer::sentence::as_element_form;

    let mut forms: Vec<Form> = Vec::new();
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

    // THE CITING FIXTURES, BUILT ON THE CORPUS FORMS. The extractor's own
    // output cites rarely and the invariants here are the CITATION ones, so
    // the fixtures are hosts that cite: one citation, two of two different
    // forms at one site (the multi-sheeted seam), and a nested host citing
    // a citing form. Built rather than hand-written so they carry real
    // geometry and not a shape chosen to pass.
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

    let (mut checked, mut with_cites) = (0usize, 0usize);
    let (mut site_total, mut site_arity1) = (0usize, 0usize);
    for f in &forms {
        let Some(e) = as_element_form(f, [1, 0, 0, 0]) else { continue };
        checked += 1;
        let env = e.receipts();

        // 1 — RETRACTION. The key is what the form IS (law 2), so decoding
        // it returns the form and re-keying that returns the key. A decode
        // that "succeeds" and yields a poorer complex is what this catches.
        let key = crate::transducer::sentence::form_vertex_key(f);
        let back = from_address(&key).expect("every minted form decodes from its own key");
        assert_eq!(form_vertex_key(&back), key, "from_address(address(F)) ≡ F on the key");
        assert_eq!(
            e.address().tag(),
            Some(crate::transducer::address::TAG_FORM),
            "and the word is a sentence by its grade-0 coordinate, not by a tape"
        );
        assert!(e.description().is_empty(), "which it no longer carries");

        // 2 — DUAL INTEGRITY. Ports and bonds in exact bijection, and the
        // sheets at one site distinct. An orphan port fails safe at every
        // reader; a phantom edge does not — it would resolve, replay and be
        // paid for against a boundary the complex never constructed.
        for x in &env.nerve {
            let host = x.host();
            assert!(
                back.vertices.iter().any(|v| {
                    matches!(v, crate::transducer::sentence::Vertex::Glued { .. })
                        && crate::transducer::sentence::vertex_bytes(v) == host.as_bytes()
                }),
                "no phantom edge: every citation anchors to a boundary the complex holds"
            );
        }
        for v in &back.vertices {
            if !matches!(v, crate::transducer::sentence::Vertex::Glued { .. }) {
                continue;
            }
            let site = crate::transducer::sentence::vertex_bytes(v);
            let mut sheets: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
            let mut n = 0usize;
            for x in env.nerve.iter().filter(|x| x.host().as_bytes() == site) {
                n += 1;
                sheets.insert(&x.target, ());
            }
            assert!(n > 0, "no orphan port: every boundary carries a citation");
            assert_eq!(sheets.len(), n, "the sheets at a site are distinct targets");
            site_total += 1;
            if n == 1 {
                site_arity1 += 1;
            }
        }

        // 3 — ROUND STABILITY. Every round the cover generates is built on
        // the hyperboloid, so each base point is strictly null and the
        // wedge terminates where the algebra does. The identity versor
        // stabilises every round it is asked about, which is the floor of
        // the reading: a transition that moves nothing moves no round.
        if !env.nerve.is_empty() {
            with_cites += 1;
            let q = |n: i64| {
                num_rational::BigRational::from(num_bigint::BigInt::from(n))
            };
            let round = crate::transducer::element::NerveRound {
                charts: env.nerve.iter().map(|x| x.target.clone()).collect(),
                points: vec![
                    [q(0), q(0), q(0), q(1)],
                    [q(3) / q(4), q(0), q(0), q(5) / q(4)],
                ],
                radius: q(1),
                attached: None,
            };
            assert_eq!(round.grade(), 2, "two charts wedge to a geodesic point pair");
            assert_eq!(
                round.stabilises(&crate::transducer::cga::Mv::scalar(q(1))),
                Some(true),
                "and the identity transition carries the round onto itself"
            );
            assert_eq!(
                round.shear(&crate::transducer::cga::Mv::scalar(q(1))),
                None,
                "with no shear to declare"
            );
        }
    }

    // THE READING THE TEST TOOK (rule 13): a pass over nothing is not a
    // pass, and a pass over nothing that CITES would not exercise the
    // invariants the purge is about.
    eprintln!(
        "  invariants: {checked} words, {with_cites} citing, {site_total} cover sites \
         ({site_arity1} of arity 1)"
    );
    assert!(checked > 0, "the invariants ran over no word");
    assert!(with_cites > 0, "and over no citation, so integrity asserted nothing");
}
