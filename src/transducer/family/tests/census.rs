// 53.6, 54.5, 69, 70 — THE CORPUS CENSUSES: readings taken over forms the
// extractor builds from the corpus on disk, each asserting only what the
// corpus in front of it proves and printing the rest as the reading.

use super::retract::{CORPUS, TASKS};

/// A part of a whole as an integer per mille, as Z10 prints it: exact
/// integer arithmetic, no float in a claim (law 1, law 4). A whole of
/// nothing has no ratio, and the reading says so rather than print zero
/// (rule 11).
fn per_mille(part: usize, whole: usize) -> String {
    match (part * 1000).checked_div(whole) {
        Some(m) => format!("{m}‰"),
        None => "unread: the whole is empty".to_string(),
    }
}

/// RC2 — THE PERCEPT'S OWN LINES, printed here off the typed readings it
/// held: its cells by grade and its bends' viscosity, each the observer's
/// reading, never a second one (78.16). A percept that did not run (a
/// memoised family) held nothing and prints nothing.
fn percept_lines() {
    if let Some(cells) = crate::transducer::family::take_cells() {
        let tally = super::tally::CellTally::of(&cells);
        eprintln!("  {}", tally.line());
        eprintln!("  {}", tally.degeneracy_line());
    }
    if let Some(v) = crate::transducer::family::take_viscosity() {
        eprintln!("  {}", crate::transducer::sentence::viscosity_line(&v));
    }
    // CT1, CT2 — THE LADDER, per circuit, by class (79.3): asserted, never
    // hand-probed. D is read across the mesh (CV4), never here; a candidate
    // is exactly a circuit whose constructions were offered.
    if let Some(c) = crate::transducer::sentence::take_circuits() {
        use crate::observe::CircuitClass;
        let mut by_class = [0usize; 6];
        let (mut untested, mut offers, mut flat_offers) = (0usize, 0usize, 0usize);
        for circuit in &c.circuits {
            match &circuit.test {
                Some(t) => {
                    assert!(!t.arms.is_empty() || t.class == CircuitClass::Flat, "a test records the arms it ran: {t:?}");
                    assert_eq!(t.class == CircuitClass::Candidate, !t.offers.is_empty(), "a candidate is what was offered");
                    by_class[CircuitClass::ALL.iter().position(|&k| k == t.class).expect("a class of the six")] += 1;
                    offers += t.offers.len();
                    flat_offers += t.offers.iter().filter(|o| o.flat).count();
                }
                None => untested += 1,
            }
        }
        assert_eq!(by_class[3], 0, "the ladder reads no D");
        let classes: Vec<String> =
            CircuitClass::ALL.iter().zip(by_class).map(|(k, n)| format!("{} {n}", k.label())).collect();
        eprintln!(
            "  circuit tests (CT1, CT2): {} | offers {offers} ({flat_offers} flat) | untested (a class of their own or a coset) {untested} of {}",
            classes.join(" "),
            c.circuits.len()
        );
    }
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
        let rule = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget);
        percept_lines();
        let Some(rule) = rule else { continue };
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
        let rule = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget);
        percept_lines();
        let Some(rule) = rule else { continue };
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
                // other's orbit never closes. `order()` answers `Infinite` for this and
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
///   NO ORBITS — the complex is a tree. There is no 1-cycle, so there is
///   nothing to be flat OR curved: a single fibration between two nodes
///   carries no constraint. The repair is more incidence, not a better
///   classifier.
///
///   ORBITS, NO EVALUATED PATH — `edge_holonomies` answers `None` or an
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
    let (mut forms, mut with_orbits, mut orbits, mut faces, mut verts) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let (mut unread, mut no_path, mut paths, mut ident, mut live) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let (mut faces_with_edge, mut word_live) = (0usize, 0usize);
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget =
            physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let rule = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget);
        percept_lines();
        let Some(rule) = rule else { continue };
        let f = &rule.form;
        forms += 1;
        verts += f.vertices.len();
        faces += f.faces.len();
        faces_with_edge += f.faces.iter().filter(|x| x.has_edge()).count();
        if !f.orbits.is_empty() {
            with_orbits += 1;
        }
        for lp in &f.orbits {
            orbits += 1;
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
         50.3 edge) · {with_orbits} forms with orbits · {orbits} orbits"
    );
    eprintln!(
        "  silence: orbits — unread {unread} · no evaluated path {no_path} · with paths {} ; \
         paths {paths} — identity {ident}, live {live}; word holonomy non-identity {word_live}",
        orbits - unread - no_path
    );
}

/// 53.6 item 3 — WHAT EVERY MEET OF THE CORPUS DID AT THE LIFT TO CLASSES.
///
/// The reading that says why `b₁ = 0`, asked of the carrier that holds it
/// rather than of a line printed about it. `CoverTally` is filled where each
/// decision is made — `meets` when a meet is read, `edges` when it carries a
/// transport, and `chords` / `within_class` / `unclassed` at the lift — so
/// this asks by field and no string is in the path. A count recovered by
/// matching a rendered line is a second carrier of one fact: the text can
/// drift from the writer and nothing makes them agree (law 12).
///
/// WHAT THE THREE MEAN, because they are three and not one:
///
///   `chords` — the meet's two pieces carry DISTINCT transport classes, so
///   it names a face between two of the scene's vertices. These are the
///   pairs spanning distinct sites, which `named_faces` calls in its
///   own comment "the cycles that carry a class", and which nothing offered
///   to the face builder before 54.7.
///
///   `within_class` — both pieces are one class. The meet is real and closes
///   nothing the scene can see; skipping it is correct and not a loss.
///
///   `unclassed` — `transport_class` answered `None` for a piece, so the
///   meet cannot be lifted whatever its geometry. It answers only for
///   `Vertex::Word`, and it gates the orbits' ENDS by the same predicate, so
///   this counter reads a blockage that stops the chord and the torsion
///   chord together.
///
/// The reading is printed with the window it was taken over (rule 13): a
/// zero because nothing ran is not a passing line.
#[test]
fn every_meet_of_the_corpus_is_accounted_for_at_the_lift() {
    let Ok(dir) = std::fs::read_dir(CORPUS) else {
        eprintln!("  lift: {CORPUS} does not read");
        return;
    };
    let mut names: Vec<String> = dir
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort_unstable();
    names.truncate(TASKS);

    // Clear whatever the process has accumulated, so the window is this
    // test's own.
    let _ = crate::transducer::family::cover::take_cover_tally();
    let _ = crate::transducer::cga::take_motor_tally();
    let epi = crate::client::sdk::Epigenome::new();
    let mut physics = crate::physics::Physics::default();
    let (mut forms, mut with_orbits) = (0usize, 0usize);
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget =
            physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let rule = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget);
        percept_lines();
        let Some(rule) = rule else { continue };
        forms += 1;
        if !rule.form.orbits.is_empty() {
            with_orbits += 1;
        }
    }
    let t = crate::transducer::family::cover::take_cover_tally();
    // MT1 — EVERY TRANSITION OVER THE CORPUS READ BY ITS ACTION, by kind.
    eprintln!("  corpus {}", crate::transducer::cga::take_motor_tally().line());
    // BQ5a — AND WHAT EACH CONTACT THE TOPOLOGY NAMED CARRIED. A census
    // that reads `0 condensed` says nothing until it says WHICH arm refused
    // (§8.1); the field and its band are gone, so what is read is the
    // contacts themselves.
    let f = crate::transducer::family::pairing::take_contact_tally();
    eprintln!(
        "  lift over {forms} corpus forms: meets {} ({} carried an edge) \
         → chords {}, within one class {}, unclassed {} \
         | axiom B: geodesic {} off-sheet {} \
         | poses: {} framed {} half-open {} unframed, {} on the tube \
         | contacts: {} offered, {} unlifted | tether: {} apart, {} contained \
         | pose: {} on the tube | {} bodiless, {} isotropy, {} unoriented, {} OBLIQUE, \
         {} mirrored, {} irrational, {} half-open | {} CONDENSED \
         | forms carrying an orbit: {with_orbits}",
        t.meets,
        t.edges,
        t.chords,
        t.within_class,
        t.unclassed,
        t.geodesic,
        t.off_sheet,
        t.framed,
        t.frame_half_open,
        t.unframed,
        t.pose_tube,
        f.offered,
        f.unlifted,
        f.apart,
        f.contained,
        f.tube,
        f.bodiless,
        f.isotropy,
        f.unoriented,
        f.oblique,
        f.mirrored,
        f.irrational,
        f.half_open,
        f.condensed
    );
    // THE ANTIPODAL TEST (54.7): the operators of each Casimir class, and
    // whether a class of exactly two is a mutually inverse pair — one
    // geodesic channel read forward and backward.
    let classes = crate::transducer::sentence::orbit_class_report();
    let pairs = classes.iter().filter(|(n, _)| *n == 2).count();
    let inverse = classes.iter().filter(|(_, inv)| *inv).count();
    eprintln!(
        "  antipodal: {} classes, sizes {:?}, {pairs} of size two, {inverse} mutually inverse",
        classes.len(),
        classes.iter().map(|(n, _)| *n).collect::<Vec<_>>()
    );
    // THE DISCRIMINATING TEST (54.7): does the footprint's own displacement
    // reach the operator, or is the chord carrying the frame transvection
    // alone? `non-simple 4 of 545` cannot answer this, because a pure
    // transvection `R = 1 − v_b v_a` is simple by construction.
    eprintln!(
        "  τ: {} of {} chords carry a footprint displacement \
         (unfooted {}, met {}, refused {})",
        t.tau_active, t.chords_offered, t.tau_unfooted, t.tau_met, t.tau_refused
    );
    assert!(forms > 0, "the lift ran over no form");
    // EVERY MEET IS ACCOUNTED FOR. The three outcomes partition the meets
    // that reached the lift, so a meet that is none of them is a reader
    // that dropped it silently — which is the defect this whole reading
    // exists to make impossible (§8.1).
    assert!(
        t.chords + t.within_class + t.unclassed > 0 || t.meets == 0,
        "meets were read and none reached the lift: {} meets, 0 accounted",
        t.meets
    );
}

/// 69 — THE WITHHOLD CENSUS: how many withheld forms are reachable by a
/// change of perspective, and how many carry a real obstruction.
///
/// THE RATIO IS THE WHOLE QUESTION. The fleet withholds ~113 forms per
/// voice with a positional residue and sheds every one of them on the
/// epoch clock, because nothing distinguishes a form that is off by a
/// COBOUNDARY — the thought right, the perspective wrong — from one
/// carrying a class in `H¹` that no vantage removes.
///
/// `Form::delta1()` answers it exactly and was called in one place in the
/// tree, to build a string for a log line. This counts the arms instead.
///
/// Measured offline against the corpus so the reading costs the running
/// fleet nothing.
#[test]
fn the_withhold_census_separates_the_reachable_from_the_obstructed() {
    use crate::transducer::family::{withheld_of, Withheld};
    let Ok(dir) = std::fs::read_dir(CORPUS) else {
        eprintln!("  withhold: {CORPUS} does not read — no corpus, no reading");
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
    let (mut extracted, mut spoke) = (0usize, 0usize);
    let (mut obstructed, mut coboundary, mut no_trajectory) = (0usize, 0usize, 0usize);
    let (mut orbits_total, mut acyclic) = (0usize, 0usize);
    let mut acyclic_residue = 0usize;
    let mut pairs_hist: [usize; 8] = [0; 8];
    let mut kinds = [0usize; 6];
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget =
            physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let rule = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget);
        percept_lines();
        let Some(rule) = rule else { continue };
        extracted += 1;
        // IS THE DISCRIMINATOR EVEN ASKABLE? `δ¹` reads the orbit
        // holonomies, so a form with NO ORBITS has an empty `delta1()` and
        // can never classify as an obstruction — not because it carries
        // none, but because there is no cycle to carry one. A census that
        // reports `H1 0` without this is a vacuous truth (the pivot rule).
        orbits_total += rule.form.orbits.len();
        if rule.form.orbits.is_empty() {
            acyclic += 1;
        }
        let (_mag, remainders) =
            crate::transducer::family::form_deflation(&rule.form, &task.train, &epi);
        // The arm this form would be withheld on, if it is withheld at all,
        // and its kind (CV1, 78.9).
        let withheld = withheld_of(&rule.form, &remainders);
        kinds[withheld.obstruction_kind().index()] += 1;
        match withheld {
            Withheld::OrbitObstruction { .. } => obstructed += 1,
            Withheld::CoboundaryResidue { pairs, .. } => {
                coboundary += 1;
                pairs_hist[pairs.min(7)] += 1;
            }
            Withheld::Acyclic { pairs, .. } => {
                acyclic_residue += 1;
                pairs_hist[pairs.min(7)] += 1;
            }
            Withheld::NoTrajectory => {
                no_trajectory += 1;
                spoke += 1;
            }
        }
    }
    eprintln!(
        "  withhold census over {} tasks: extracted {extracted} | H1-OBSTRUCTION {obstructed} \
         COBOUNDARY {coboundary} ACYCLIC-UNASKABLE {acyclic_residue} \
         NO-TRAJECTORY {no_trajectory} (spoke {spoke})",
        names.len()
    );
    eprintln!("  coboundary residue by train pairs failed: {pairs_hist:?}");
    let by_kind: Vec<String> = crate::transducer::sentence::ObstructionKind::ALL
        .iter()
        .zip(kinds)
        .map(|(k, n)| format!("{} {n}", k.label()))
        .collect();
    eprintln!("  withheld by kind (CV1): {}", by_kind.join(" "));
    eprintln!(
        "  IS THE DISCRIMINATOR ASKABLE: {acyclic}/{extracted} forms carry NO orbits \
         ({orbits_total} orbits in total) — on those, delta1 is empty and H1 cannot be read"
    );
    // A reading over no extraction is not a reading (rule 11).
    assert!(extracted > 0, "no form extracted from the corpus: nothing to classify");
}

/// A.14 ON THE DIAGONAL (70.4, H2b) — CONTACT BETWEEN HUE-BLIND BODIES.
///
/// A body is a maximal hue-blind region of the 6-incidence, so two bodies
/// never share an edge of the base; what relates two of them is the
/// cell of lower grade, `contact_cells`. This reads the contact graph on it —
/// vertices the BODIES, edges the distinct body pairs that meet at a
/// corner — and takes `V`, `E`, `C` and `b₁ = E − V + C`, with the bodies
/// that touch nothing counted apart so a sparse scene is not read as a
/// graph that failed.
///
/// The 6-incidence edges across two hues are no longer contact at all:
/// they lie inside one body, and they are counted as what they now are,
/// the gauge seams H3 reads.
///
/// AND IT TAKES H1'S OWN READING, which nothing asserted: the bodies
/// perception builds are the hue-blind regions, one for one, on every
/// scene of the corpus.
///
/// NOT A SCALAR VERDICT: the pairs are named by their component roots,
/// and the counts are the SIZE of a reading, not the reading (rule 2).
#[test]
fn a14_contact_is_the_diagonal_between_hue_blind_bodies() {
    use crate::transducer::sentence::{components, contact_cells, face_incidence};
    let Ok(dir) = std::fs::read_dir(CORPUS) else {
        eprintln!("  a14: {CORPUS} does not read — no corpus, no reading");
        return;
    };
    let mut names: Vec<String> = dir
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort_unstable();
    names.truncate(TASKS);

    let (mut scenes, mut bodies_total, mut perceived) = (0usize, 0usize, 0usize);
    let (mut seams, mut interior, mut contact_pairs) = (0usize, 0usize, 0usize);
    let (mut contact_b1, mut scenes_with_cycles) = (0usize, 0usize);
    let (mut isolated, mut contact_components, mut scenes_e_ge_v) = (0usize, 0usize, 0usize);
    let mut b1_hist: [usize; 16] = [0; 16];
    let (mut singletons, mut isolated_singletons, mut pairs_with_singleton) = (0usize, 0usize, 0usize);
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        for (input, _) in &task.train {
            // The scene's laminae, exactly as `perceive_at` builds them.
            let mut laminae: Vec<crate::transducer::blade::Lamina> = Vec::new();
            for (r, row) in input.iter().enumerate() {
                for (c, &h) in row.iter().enumerate() {
                    if h != 0 {
                        laminae.push(crate::transducer::blade::Lamina::from((r as i64, c as i64, 0i64, h)));
                    }
                }
            }
            if laminae.is_empty() {
                continue;
            }
            laminae.sort_unstable();
            laminae.dedup_by(|x, y| x.site == y.site);
            let positions: Vec<crate::transducer::base::Site> =
                laminae.iter().map(|x| x.site.clone()).collect();
            // THE BODIES: hue-blind components of the 6-incidence (70.4).
            let all = face_incidence(&positions);
            let roots = components(laminae.len(), &all);
            // The 6-incidence edges across two hues lie INSIDE one body now:
            // they are its gauge seams, H3's content, and never contact.
            for &(i, j) in &all {
                match laminae[i].hue == laminae[j].hue {
                    true => interior += 1,
                    false => seams += 1,
                }
            }
            // THE CONTACTS: corner incidences between two DIFFERENT bodies,
            // named by their roots. A corner inside one body is its interior.
            let mut pairs: Vec<(usize, usize)> = contact_cells(&positions)
                .into_iter()
                .map(|c| (roots[c.i], roots[c.j]))
                .filter(|(a, b)| a != b)
                .map(|(a, b)| if a <= b { (a, b) } else { (b, a) })
                .collect();
            pairs.sort_unstable();
            pairs.dedup();
            let n_bodies = {
                let mut r = roots.clone();
                r.sort_unstable();
                r.dedup();
                r.len()
            };
            // H1'S READING: what perception builds is exactly these regions.
            perceived += crate::transducer::family::perceive_at(input, 0).len();
            // DOES THE CONTACT GRAPH CARRY CYCLES? `b₁ = E − V + C` on the
            // graph whose vertices are BODIES and whose edges are the
            // corner contacts between them.
            let mut root_ids: Vec<usize> = roots.clone();
            root_ids.sort_unstable();
            root_ids.dedup();
            let ix = |r: usize| match root_ids.binary_search(&r) {
                Ok(i) => i,
                Err(_) => panic!("every root is one of the root ids it was read from"),
            };
            let cg: Vec<(usize, usize)> =
                pairs.iter().map(|&(a, b)| (ix(a), ix(b))).collect();
            let comps = components(root_ids.len(), &cg);
            let n_comp = {
                let mut c = comps.clone();
                c.sort_unstable();
                c.dedup();
                c.len()
            };
            // E − V + C is never negative on a graph: a negative reading is
            // a broken counter and not a Betti number, so it refuses rather
            // than floor at zero (rule 11).
            let b1 = (pairs.len() + n_comp)
                .checked_sub(root_ids.len())
                .expect("b₁ = E − V + C is never negative");
            contact_b1 += b1;
            if b1 > 0 {
                scenes_with_cycles += 1;
                b1_hist[b1.min(15)] += 1;
            }
            // WHY IS IT A FOREST? A body touching NOTHING contributes a
            // vertex and no edge, so a scene of isolated pieces is sparse
            // by its own nature and not by anything the engine filtered.
            // This separates a real scene fact from a second famine.
            let mut degree = vec![0usize; root_ids.len()];
            for &(a, b) in &cg {
                degree[a] += 1;
                degree[b] += 1;
            }
            isolated += degree.iter().filter(|&&d| d == 0).count();
            contact_components += n_comp;
            // Body sizes, and how much of the contact graph is pixels.
            let mut size = vec![0usize; root_ids.len()];
            for r in &roots {
                size[ix(*r)] += 1;
            }
            singletons += size.iter().filter(|&&s| s == 1).count();
            for (i, &d) in degree.iter().enumerate() {
                if d == 0 && size[i] == 1 {
                    isolated_singletons += 1;
                }
            }
            for &(a, b) in &cg {
                if size[a] == 1 || size[b] == 1 {
                    pairs_with_singleton += 1;
                }
            }
            if pairs.len() >= root_ids.len() {
                scenes_e_ge_v += 1;
            }
            scenes += 1;
            bodies_total += n_bodies;
            contact_pairs += pairs.len();
        }
    }
    // Apart from the isolated bodies: the graph of the bodies that touch.
    let (touching, touching_components) = (bodies_total - isolated, contact_components - isolated);
    eprintln!(
        "  a14 on the diagonal over {scenes} scenes: V {bodies_total} bodies E {contact_pairs} \
         corner contacts C {contact_components} b₁ {contact_b1}, {scenes_with_cycles} scenes \
         carrying a cycle | isolated {isolated} ({} touch nothing) · of the rest V {touching} \
         C {touching_components} | scenes with E>=V {scenes_e_ge_v}",
        per_mille(isolated, bodies_total)
    );
    eprintln!("  a14 b₁ per cyclic scene: {b1_hist:?}");
    eprintln!(
        "  a14 inside the bodies: 6-incidence {interior} same-hue edges, {seams} gauge seams \
         across two hues | single-cell bodies {singletons} ({}), of which {isolated_singletons} \
         isolated | contacts touching a single-cell body {pairs_with_singleton} of \
         {contact_pairs} ({}) | perception built {perceived} bodies",
        per_mille(singletons, bodies_total),
        per_mille(pairs_with_singleton, contact_pairs)
    );
    assert!(scenes > 0, "no scene read: nothing to count");
    assert_eq!(
        perceived, bodies_total,
        "perception's bodies are the hue-blind regions, one for one (H1)"
    );

    // C2, 70.6 — THE FRAME AXIS IN CONTACT. One scene is one frame, so the
    // reading above can hold no contact across frames. A family is its
    // frames stacked along `e₃` (51.3): each train input at its own `k`.
    // On that complex the twenty adjacencies are read whole, and a contact
    // is counted by the frames its elements span: within one frame, across
    // two, or both along one interface.
    let (mut families, mut fv, mut fe, mut fc, mut fb1) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let (mut e_in, mut e_across, mut e_both) = (0usize, 0usize, 0usize);
    let (mut elements_in, mut elements_across) = (0usize, 0usize);
    // BQ2 — BY GRADE. A body pair's contact is the grade of the LARGEST cell
    // it shares (71.2(2)); a shared cell is counted at its own grade and by
    // the axes it differs on. Keyed by the grade and by the axis set, so no
    // count of grades is written down here.
    let mut pairs_by_grade: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    let mut cells_by_grade: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    let mut cells_by_axes: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    let bump = |k: &mut crate::network::nerve::Keyed<usize>, key: Vec<u8>| match k.get_mut(&key) {
        Some(n) => *n += 1,
        None => {
            k.insert(key, 1);
        }
    };
    let axes_key = |axes: &[usize]| -> Vec<u8> { axes.iter().map(|&a| u8::try_from(a).expect("an axis of the base, below its n")).collect() };
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut laminae: Vec<crate::transducer::blade::Lamina> = Vec::new();
        for (k, (input, _)) in task.train.iter().enumerate() {
            for (r, row) in input.iter().enumerate() {
                for (c, &h) in row.iter().enumerate() {
                    if h != 0 {
                        laminae.push(crate::transducer::blade::Lamina::from((r as i64, c as i64, k as i64, h)));
                    }
                }
            }
        }
        if laminae.is_empty() {
            continue;
        }
        laminae.sort_unstable();
        laminae.dedup_by(|x, y| x.site == y.site);
        let positions: Vec<crate::transducer::base::Site> = laminae.iter().map(|x| x.site.clone()).collect();
        let roots = components(positions.len(), &face_incidence(&positions));
        // Each body pair with the fibre of its cells — `e₃ ∉ axes`,
        // `e₃ ∈ axes` — and the grade of the largest cell it shares.
        let mut span: crate::network::nerve::Keyed<(bool, bool, usize)> = crate::network::nerve::Keyed::new();
        for cell in contact_cells(&positions) {
            let (a, b) = (roots[cell.i].min(roots[cell.j]), roots[cell.i].max(roots[cell.j]));
            if a == b {
                continue;
            }
            let within = !cell.crosses(crate::transducer::base::FRAME_AXIS);
            match within {
                true => elements_in += 1,
                false => elements_across += 1,
            }
            bump(&mut cells_by_grade, vec![u8::try_from(cell.grade).expect("a contact cell's grade, at most the base's n")]);
            bump(&mut cells_by_axes, axes_key(&cell.axes));
            let mut k = (a as u64).to_le_bytes().to_vec();
            k.extend_from_slice(&(b as u64).to_le_bytes());
            // A pair first met here spans nothing yet; one met before carries
            // what it spanned and the largest cell it shared.
            let (was_in, was_across, was_grade) = match span.get(&k) {
                Some(&held) => held,
                None => (false, false, cell.grade),
            };
            let grade = if cell.grade > was_grade { cell.grade } else { was_grade };
            span.insert(k, (was_in || within, was_across || !within, grade));
        }
        let mut root_ids: Vec<usize> = roots.clone();
        root_ids.sort_unstable();
        root_ids.dedup();
        let ix = |r: usize| match root_ids.binary_search(&r) {
            Ok(i) => i,
            Err(_) => panic!("every root is one of the root ids it was read from"),
        };
        let mut cg: Vec<(usize, usize)> = Vec::new();
        for (k, &(inf, acr, grade)) in span.scoped() {
            match (inf, acr) {
                (true, false) => e_in += 1,
                (false, true) => e_across += 1,
                _ => e_both += 1,
            }
            bump(&mut pairs_by_grade, vec![u8::try_from(grade).expect("the largest shared cell's grade, at most the base's n")]);
            let (Ok(a), Ok(b)) = (
                <[u8; 8]>::try_from(&k[..8]).map(u64::from_le_bytes),
                <[u8; 8]>::try_from(&k[8..]).map(u64::from_le_bytes),
            ) else {
                panic!("a body pair key is two roots")
            };
            cg.push((
                ix(usize::try_from(a).expect("a root id written from a usize")),
                ix(usize::try_from(b).expect("a root id written from a usize")),
            ));
        }
        let n_comp = {
            let mut c = components(root_ids.len(), &cg);
            c.sort_unstable();
            c.dedup();
            c.len()
        };
        families += 1;
        fv += root_ids.len();
        fe += cg.len();
        fc += n_comp;
        fb1 += (cg.len() + n_comp)
            .checked_sub(root_ids.len())
            .expect("b₁ = E − V + C is never negative");
    }
    eprintln!(
        "  a14 with the frame axis over {families} families: V {fv} bodies E {fe} contacts \
         (e₃ ∉ axes {e_in}, e₃ ∈ axes {e_across}, both {e_both}) C {fc} b₁ {fb1} \
         | shared cells e₃ ∉ axes {elements_in}, e₃ ∈ axes {elements_across}"
    );
    let listed = |k: &crate::network::nerve::Keyed<usize>, name: &dyn Fn(&[u8]) -> String| -> String {
        k.scoped().map(|(key, n)| format!("{} {n}", name(&key))).collect::<Vec<_>>().join(" ")
    };
    // A grade key is the grade's one byte, written above.
    let grade_name = |key: &[u8]| format!("g{}", key.iter().map(|g| g.to_string()).collect::<String>());
    let axes_name = |key: &[u8]| {
        format!("{{{}}}", key.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(","))
    };
    eprintln!(
        "  a14 by grade: contacts by their largest shared cell {} | shared cells by grade {} \
         | by differing axes {}",
        listed(&pairs_by_grade, &grade_name),
        listed(&cells_by_grade, &grade_name),
        listed(&cells_by_axes, &axes_name),
    );
    // THE RELATION, NOT A COUNT (BQ2): every contact has one grade, the
    // largest cell it shares, so the contacts by grade sum to the contacts;
    // and every shared cell has one grade and one axis set.
    let total = |k: &crate::network::nerve::Keyed<usize>| k.scoped().map(|(_, n)| *n).sum::<usize>();
    assert_eq!(total(&pairs_by_grade), fe, "the contacts are the sum over grades");
    assert_eq!(total(&cells_by_grade), elements_in + elements_across, "each shared cell has one grade");
    assert_eq!(total(&cells_by_axes), elements_in + elements_across, "and one set of differing axes");
}

/// 70 — THE DEMAND-KIND CENSUS: how many demands can GROW LANGUAGE.
///
/// A fleet reading of "2,751 sentences spoken carrying demands" cannot be
/// read without this split. `Kernel::list_demands` routes exactly two
/// kinds onward:
///
/// ```text
///     RotorPrime  → open_letter        37.7 P2–P3, the Gaussian factor
///     Standing    → open_unspellable   the rotor the peel could not spell,
///                                      which IS the candidate generator
///     Source | Creation | TopologicalDeformation  → `_ => {}`
/// ```
///
/// The other three adjoin nothing. A corpus that raises thousands of
/// deformations and no `Standing` has a court with nothing to open, and
/// "generators adjoined by closure: 0" is then a consequence rather than
/// a finding.
#[test]
fn the_demand_kind_census_says_whether_language_can_grow() {
    use crate::transducer::sentence::Demand;
    let Ok(dir) = std::fs::read_dir(CORPUS) else {
        eprintln!("  demands: {CORPUS} does not read — no corpus, no reading");
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
    let (mut forms, mut with_demands) = (0usize, 0usize);
    let (mut rotor_prime, mut standing) = (0usize, 0usize);
    let (mut source, mut creation, mut deformation, mut other) =
        (0usize, 0usize, 0usize, 0usize);
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget =
            physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let rule = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget);
        percept_lines();
        let Some(rule) = rule else { continue };
        forms += 1;
        let mut any = false;
        for (_, kind, _, _) in rule.form.demand_vertices() {
            any = true;
            match kind {
                Demand::RotorPrime { .. } => rotor_prime += 1,
                Demand::Standing { .. } => standing += 1,
                Demand::Source { .. } => source += 1,
                Demand::Creation => creation += 1,
                Demand::TopologicalDeformation { .. } => deformation += 1,
                _ => other += 1,
            }
        }
        if any {
            with_demands += 1;
        }
    }
    let growing = rotor_prime + standing;
    eprintln!(
        "  demand kinds over {forms} forms ({with_demands} carrying demands): \
         GROWS LANGUAGE → RotorPrime {rotor_prime} Standing {standing} (total {growing}) \
         | adjoins nothing → Source {source} Creation {creation} \
         TopologicalDeformation {deformation} other {other}"
    );
    assert!(forms > 0, "no form extracted: nothing to census");
}
