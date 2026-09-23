// Y3, §8.1 — INSTRUMENTATION ONLY: what `mint_sentence` is handed and
// what it hands back, for every task of a slice whose rule form closes.
//
//! TRACE LINES AND NOTHING ELSE. No library function is changed, none is
//! made more visible, and nothing here decides anything: every reading is
//! taken from the public API the engine already exposes, by calling the
//! same functions `standable` calls in the same order with the same
//! arguments. If this file were deleted the engine would behave
//! identically, which is the whole of what §8.1 permits and the whole of
//! what this row's first commit is allowed to be.
//!
//! Y1 handed over the instance: over sixty tasks, 54 extract a rule, 5 of
//! those close, and all 5 are refused at the mint. `no_form 0` and
//! `sentence_open 0` place the break at `mint_sentence` and nowhere else.
//! `mint_sentence` is two calls —
//!
//!     let mag = sentence_measure(&rule.form, family, crystal)?;
//!     as_element_form(&rule.form, mag)
//!
//! — so the refusal is one of those two, and this prints which, beside
//! the readings each of them was working from.
//!
//! NO CAUSE IS WRITTEN HERE. The trace is the deliverable.

use mycelium::client::sdk::Epigenome;
use mycelium::transducer::element::measure_lanes;
use mycelium::transducer::family::{extract_rule_with, form_deflation, sentence_measure};
use mycelium::transducer::sentence::{as_element_form, Vertex};

/// The stroke Y1 set for itself, so the same slice reads the same way
/// under both instruments and a task that closes for one closes for the
/// other.
const MINT_STROKE: usize = 1 << 20;

/// Walk the slice and trace the mint for every task whose rule form
/// closes — the five, and any others a larger slice turns up.
pub(crate) fn trace(dir: &str, names: &[String]) {
    let empty = Epigenome::new();
    let mut closed_seen = 0u64;
    let (mut open_seen, mut open_demanded, mut open_minted, mut open_stood) = (0u64, 0u64, 0u64, 0u64);
    let mut open_acts = 0u64;
    let (mut open_rem_empty, mut open_rem_links) = (0u64, 0u64);
    // S9.13's REACH, as a distribution over the families that mint. E2 prices a
    // closure's yield from it.
    //
    // IT IS READ OFF THE FORM, which is the carrier. The first cut of this
    // instrument read `Element::fiber_span` on the minted word and reported 1
    // on every family — not a measurement of the corpus but a constant by
    // construction, because a minted sentence's element leaves its own two
    // fibre fields empty and carries the cycles on its form. A measurement that
    // returns a constant by construction is the vacuous assertion at the
    // instrument, so the reading moved to `Form::fibre_volume`.
    //
    // THE TWO `sections` ARE NOT ONE OBJECT and this prints both rather than a
    // span derived from either. `Element.sections` is the punctured axis —
    // "alternative per-site transition ASSIGNMENTS over the SAME base
    // positions", the free part of the fibre. `Form.sections` is
    // `Keyed<Vec<u16>>` holding PAIR INDICES of the bundle — which pairs a
    // class was measured on, a restriction record. Mirroring one formula onto
    // both would count the second as if it were the first, which is L8r's fault
    // exactly (`on_class` and `on_body`, one reader over two coordinate
    // systems).
    let (mut reach_one, mut reach_more) = (0u64, 0u64);
    let (mut form_sections, mut form_torsion) = (0u64, 0u64);
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else {
            continue;
        };
        let Some(task) = mycelium::arc::parse_task(&json) else {
            continue;
        };
        let mut physics = mycelium::physics::Physics::default();
        let mut budget = physics.stroke_n(MINT_STROKE);
        let Some(rule) = extract_rule_with(&task.train, &empty, &mut budget) else {
            continue;
        };
        // THE OTHER POPULATION, and the instrument's `is_closed()` gate is
        // what hides it: a rule form that does not close is `rule_open` to
        // Y1 and never reaches `mint_sentence` there. Production has no such
        // gate — `speak_form` branches on DEMAND VERTICES and a demanded
        // form goes to `speak_with_demands`, which calls `mint_sentence`
        // directly. So the mint is read on these too, and the counts say
        // whether the branch that Y1 cannot reach actually works.
        //
        // THE GATE NO LONGER SKIPS THE DETAILED READING. D8 makes a demanded
        // form the normal outcome for an unresolved multiplicity, and "a
        // sentence with demands is valid, never closed" (2.2, 50.1), so the
        // task D8 is about is exactly the task this gate hid. The counters
        // stay; the `continue` that followed them is gone, and every task that
        // extracts a rule now gets the same lines.
        // THE REACH READING (§8.1, prints only). Per family that mints: the
        // FORM's reach, which is the reading E2 prices, beside what the minted
        // ELEMENT's own fibre fields hold — printed together so that the two
        // carriers are visibly two and a later reader cannot take one for the
        // other. A reach of 0 is base-only: no filed cycle to reach past it.
        // The sweep mints in the stroke it extracts: the bundle stood over no
        // receipts, so 𝒯 is zero as a reading.
        if let Some(w) = mycelium::transducer::family::mint_sentence(&rule, &task.train, &empty, 0) {
            let reach = rule.form.fibre_volume(&rule.form.torsion);
            if reach > 0 {
                reach_more += 1;
            } else {
                reach_one += 1;
            }
            form_sections += rule.form.sections.len() as u64;
            form_torsion += rule.form.torsion.len() as u64;
            let periods: Vec<u32> =
                rule
                .form
                .torsion
                .scoped()
                .filter_map(|(_, f)| f.periods.yield_units().and_then(|u| u32::try_from(u).ok()))
                .collect();
            println!(
                "  mint {name} reach: form fibre_volume {reach} \
                 (form sections {} torsion {} periods {periods:?}) \
                 | element's own fibre: sections {} torsion {}",
                rule.form.sections.len(),
                rule.form.torsion.len(),
                w.sections().len(),
                w.torsion().len(),
            );
        }
        let closed = rule.form.is_closed();
        if closed {
            closed_seen += 1;
        } else {
            let has_demands = rule.form.demand_vertices().next().is_some();
            let minted =
                mycelium::transducer::family::mint_sentence(&rule, &task.train, &empty, 0);
            open_seen += 1;
            // COUNT TWO: `Form.remainder` on this population. It is what
            // extraction could not ACCOUNT FOR when it built the form, and
            // it is a different carrier from `form_deflation`'s per-pair
            // remainders, which are what the form fails to REPRODUCE when
            // spoken. If it is empty here as well as on the five, it is
            // empty for everyone and was never the carrier for this fact.
            if rule.form.remainder.is_empty() {
                open_rem_empty += 1;
            } else {
                open_rem_links += rule.form.remainder.placed().len() as u64;
            }
            if has_demands {
                open_demanded += 1;
            }
            // A FORM NOTHING IN WHICH ACTS IS A CHAIN, NOT A SENTENCE (23.1
            // step 3, 50.6/L7), and the court says so. A word the mint
            // produces is not a word the court takes, so the two are counted
            // apart or the tally reads as comprehension it is not.
            if minted.is_some() {
                open_minted += 1;
                if rule.form.acts() {
                    open_acts += 1;
                }
                let mut mind = Epigenome::new();
                if let Some(w) = minted {
                    if mind.stand_dialect_withheld(w, 1, 1).is_some() {
                        open_stood += 1;
                    }
                }
            }
        }

        // WHAT THE MINT IS HANDED. The form, by what its vertices are, and
        // the family, by what its pairs are.
        let (mut words, mut glued, mut demands) = (0u64, 0u64, 0u64);
        for v in &rule.form.vertices {
            match v {
                Vertex::Word { .. } => words += 1,
                Vertex::Glued { .. } => glued += 1,
                Vertex::Demand { .. } => demands += 1,
                _ => {}
            }
        }
        println!(
            "  mint {name} handed: pairs {} | vertices words {words} glued {glued} demands {demands} \
             | faces {} orbits {} | closed {} | Form.remainder empty {} links {} | ratio {:?}",
            task.train.len(),
            rule.form.faces.len(),
            rule.form.orbits.len(),
            u8::from(rule.form.is_closed()),
            u8::from(rule.form.remainder.is_empty()),
            rule.form.remainder.placed().len(),
            rule.ratio,
        );
        for (i, (x, y)) in task.train.iter().enumerate() {
            // The pair as the trajectory sees it: the delta where the pair
            // refines, and `refine_delta` answering None where it does
            // not — reported as the engine's own reading and not
            // re-derived here.
            let refined = mycelium::arc::refine_delta(x, y);
            // A grid's width is its rows' common length, and the slice
            // pattern reads the row the shape is stated by without taking
            // a member by position (rule 15).
            let wide = |g: &mycelium::arc::Grid| match g.split_first() {
                Some((row, _)) => row.len(),
                None => 0,
            };
            println!(
                "  mint {name} pair {i}: in {}x{} out {}x{} | refine_delta {} | delta mu {:?}",
                x.len(),
                wide(x),
                y.len(),
                wide(y),
                match refined {
                    Some(_) => "some",
                    None => "none",
                },
                refined.as_ref().map(|r| measure_lanes(&r.delta)),
            );
        }

        // WHAT `form_deflation` RETURNS — the pay it accumulated and the
        // remainder it left on every pair, which is what `sentence_measure`
        // tests when the form carries no demand vertex.
        let (pay, remainders) = form_deflation(&rule.form, &task.train, &empty);
        println!("  mint {name} form_deflation: pay {pay:?} remainders {}", remainders.len());
        for (i, r) in remainders.iter().enumerate() {
            println!(
                "  mint {name} remainder {i}: links {} empty {} mu {:?}",
                r.len(),
                u8::from(r.is_empty()),
                measure_lanes(r),
            );
        }

        // WHICH OF THE MINT'S TWO CALLS REFUSED. `sentence_measure`
        // returns `None` on exactly one path — a form with NO demand
        // vertex that left a remainder — and `as_element_form` is the
        // other half.
        let has_demands = rule.form.demand_vertices().next().is_some();
        let measured = sentence_measure(&rule.form, &task.train, &empty, 0);
        println!(
            "  mint {name} sentence_measure: {} | has_demands {} | all_remainders_empty {}",
            match measured {
                Some(m) => format!("some {m:?}"),
                None => "NONE".to_string(),
            },
            u8::from(has_demands),
            u8::from(remainders.iter().all(Vec::is_empty)),
        );
        match measured {
            None => println!("  mint {name} refused at: sentence_measure"),
            Some(mag) => match as_element_form(&rule.form, mag) {
                Some(e) => println!(
                    "  mint {name} as_element_form: some, {} bytes — and the mint did NOT refuse here",
                    e.to_bytes().len()
                ),
                None => println!("  mint {name} refused at: as_element_form, on mag {mag:?}"),
            },
        }

        // THE FORM'S OWN IDENTITY (17.2, law 2): `form_vertex_key` is M2 —
        // the vertex count, and per word vertex the composed rotor's key,
        // the carriage, the contact and the SORTED gauge. It does not write
        // the class. A representation change that keeps one member per
        // vertex must leave it untouched, and this is the reading that says
        // so; the five never mint an element, so the element's address is
        // unavailable and this is the identity the engine has for them.
        //
        // A TRACE LINE IS NOT THE OBJECT (§3.2). This prints the address so
        // two runs can be compared on it, and the comparison is address
        // equality — 17.2's own predicate — never text of anything else.
        let m2 = mycelium::transducer::sentence::form_vertex_key(&rule.form);
        let m2_hex: String = m2.as_bytes().iter().map(|b| format!("{b:02x}")).collect();
        println!("  mint {name} M2: {} bytes {m2_hex}", m2.as_bytes().len());

        // COUNT ONE OF THREE: THE CLASS AND GAUGE OF EVERY WORD VERTEX in
        // each pair form, and whether they agree. `Vertex::Word.class` is a
        // single `Invariant` — the body's own moduli — and 21 step 1 calls
        // it a section coordinate carried on the vertex. If the several
        // words of one transport class disagree on it, a single-valued
        // coordinate is being asked to carry a section.
        for (i, (x, y)) in task.train.iter().enumerate() {
            let mut pb = physics.stroke_n(MINT_STROKE);
            let Some(pf) = mycelium::transducer::family::scene_form(&empty, &mut pb, x, y)
            else {
                continue;
            };
            let mut classes: mycelium::network::nerve::Keyed<()> = mycelium::network::nerve::Keyed::new();
            let mut gauges: mycelium::network::nerve::Keyed<()> = mycelium::network::nerve::Keyed::new();
            let mut rotors: mycelium::network::nerve::Keyed<()> = mycelium::network::nerve::Keyed::new();
            let mut carriages = 0u64;
            for v in &pf.vertices {
                if let Vertex::Word { derivation, .. } = v {
                    // EVERY MEMBER's chart and gauge, so the count reads the
                    // fibre and not a representative of it (D7).
                    for (ck, a) in v.acts().into_iter().flat_map(|k| k.scoped()) {
                        classes.insert(&ck, ());
                        let mut gk = Vec::new();
                        for (x, y) in &a.gauge {
                            gk.push(*x);
                            gk.push(*y);
                        }
                        gauges.insert(&gk, ());
                    }
                    if let Some(ck) = derivation.rotor().class_key() {
                        rotors.insert(ck.as_bytes(), ());
                    }
                    carriages += 1;
                }
            }
            println!(
                "  mint {name} pairform {i} coords: words {carriages} | distinct class {} gauge {} rotor_class {}",
                classes.len(),
                gauges.len(),
                rotors.len(),
            );
            let _ = y;
        }

        // COUNT TWO: WHAT `speak_rule` MATCHES THE RULE'S VERTEX AGAINST.
        // `apply_form` perceives the input, admits every body into a
        // scratch atlas by its moduli, and for each Word vertex takes
        // `atlas.fiber(class)` — so one vertex CAN speak several bodies
        // (21's design). This counts the bodies present and the bodies the
        // fiber reaches, and reads `unspoken` for a vertex whose fiber was
        // empty.
        for (i, (x, y)) in task.train.iter().enumerate() {
            let bodies = mycelium::transducer::family::perceive(x);
            let mut atlas = mycelium::transducer::stalk::Stalks::default();
            for (bi, b) in bodies.iter().enumerate() {
                atlas.admit(&b.moduli, (bi as u64).to_le_bytes());
            }
            let hits: Vec<usize> = rule
                .form
                .vertices
                .iter()
                .filter_map(|v| match v {
                    // The fibre hits the vertex's members reach, summed:
                    // one member today, so this is the same number D7a
                    // must preserve, and D7b is where it rises.
                    Vertex::Word { .. } => Some(
                        v.acts()
                            .into_iter()
                            .flat_map(|k| k.scoped())
                            .map(|(_, a)| atlas.fiber(&a.chart).len())
                            .sum::<usize>(),
                    ),
                    _ => None,
                })
                .collect();
            let spoken = mycelium::transducer::family::speak_rule(&rule, x, &empty);
            println!(
                "  mint {name} speak {i}: bodies perceived {} | fiber hits per word vertex {hits:?} \
                 | unspoken {:?} | painted sites emitted {} | residual vs observed {}",
                bodies.len(),
                spoken.as_ref().map(|s| s.unspoken.len()),
                spoken.as_ref().map_or(0, |s| s.painted.len()),
                spoken.as_ref().map_or(0, |s| {
                    let g = s.grid();
                    let mut n = 0usize;
                    for (r, row) in y.iter().enumerate() {
                        for (c, &h) in row.iter().enumerate() {
                            // A site the speech does not reach is the
                            // ground, and hue 0 IS absence in this
                            // encoding — `residual_of`'s own reading, not
                            // a default put here for a missing value.
                            let got = match g.as_ref().and_then(|g| g.get(r)).and_then(|gr| gr.get(c)) {
                                Some(h) => *h,
                                None => 0,
                            };
                            if got != h {
                                n += 1;
                            }
                        }
                    }
                    n
                }),
            );
        }

        // COUNT THREE: DOES `carriages` HOLD AN ENTRY FOR THE CLASS, or did
        // `place_conflict` empty it? `reindex` accumulates one carriage per
        // transport key, marks a conflict when a second differs, and writes
        // only the unconflicted keys — its comment says the class "stays two
        // sections" and nothing writes a section.
        println!(
            "  mint {name} carriages: {} entries for {words} word vertices",
            rule.form.carriages.len()
        );

        // PER PAIR, WHAT THE PAIR FORM TYPED AND WHAT SPEAKING IT LEAVES.
        // `finish` already groups every pair form's demand vertices by
        // (kind, chart) and pushes one rule vertex per group, so a rule
        // with no demands is a rule whose PAIR FORMS emitted none. This
        // reads both sides of that: the demands the pair form carries, and
        // the links speaking that same pair form fails to reproduce.
        for (i, (x, y)) in task.train.iter().enumerate() {
            let mut pb = physics.stroke_n(MINT_STROKE);
            let one = vec![(x.clone(), y.clone())];
            let Some(pf) = mycelium::transducer::family::scene_form(&empty, &mut pb, x, y)
            else {
                println!("  mint {name} pairform {i}: none");
                continue;
            };
            let (pf_words, pf_demands) = pf.vertices.iter().fold((0u64, 0u64), |(w, d), v| match v {
                Vertex::Word { .. } => (w + 1, d),
                Vertex::Demand { .. } => (w, d + 1),
                _ => (w, d),
            });
            let (pf_pay, pf_rem) = form_deflation(&pf, &one, &empty);
            println!(
                "  mint {name} pairform {i}: words {pf_words} demands {pf_demands} \
                 | speaking it leaves {} links, mu {:?} | pay {pf_pay:?}",
                pf_rem.iter().map(Vec::len).sum::<usize>(),
                // One pair in, so one remainder out: the slice pattern
                // reads the pair's own entry rather than a member by
                // position (rule 15).
                pf_rem.split_first().map(|(r, _)| measure_lanes(r)),
            );
        }

        // WHAT PRODUCTION DOES WITH THE SAME FORM. `speak_form` takes the
        // trajectory, branches on demand vertices, and where a demandless
        // form leaves a remainder on any pair it calls `withhold_form` —
        // which is `as_element_form(f, mag)` and `stand_dialect_withheld`,
        // with `mag` being `form_deflation`'s pay and `sentence_measure`
        // never called at all. Those two calls are made here, in that
        // order, with those arguments, on a scratch mind: the same
        // sequence, read rather than asserted.
        let demandless_with_remainder =
            !has_demands && remainders.iter().any(|r| !r.is_empty());
        println!(
            "  mint {name} production branch: {}",
            match (has_demands, demandless_with_remainder) {
                (true, _) => "speak_with_demands — mint_sentence, then the wire; nothing stood locally",
                (false, true) => "withhold_form — as_element_form(pay) then stand_dialect_withheld",
                (false, false) => "speak_with — as_element_form(pay) then the wire",
            }
        );
        if demandless_with_remainder {
            match as_element_form(&rule.form, pay) {
                None => println!(
                    "  mint {name} withheld path: as_element_form(pay {pay:?}) NONE — production stands nothing either"
                ),
                Some(word) => {
                    let mut mind = Epigenome::new();
                    let stood = mind.stand_dialect_withheld(word, 1, 1);
                    println!(
                        "  mint {name} withheld path: as_element_form(pay {pay:?}) some, stand_dialect_withheld {} — dialect now {}",
                        match &stood {
                            Some(_) => "SOME",
                            None => "none",
                        },
                        mind.dialect_len(),
                    );
                }
            }
        }
    }
    println!("  mint tasks with a closed rule form: {closed_seen}");
    // S9.13's reach over the minting families, as a distribution, read off
    // the form. Zero reach on all of them says no family on this slice filed a
    // cycle — which is a reading of the corpus, where the same line over
    // `Element::fiber_span` was a reading of the mint path's construction.
    println!(
        "  mint reach: {} minting families, form reach 0 on {reach_one}, above 0 on {reach_more} \
         | form carriers over them: sections {form_sections} torsion {form_torsion}",
        reach_one + reach_more
    );
    // THE POPULATION Y1's GATE HIDES. `rule_open` to the instrument;
    // `speak_with_demands` to production, which has no closure gate.
    println!(
        "  mint rule_open tasks {open_seen} of which demanded {open_demanded} minted {open_minted} \
         of those acting {open_acts} stood_withheld {open_stood}"
    );
    println!(
        "  mint rule_open Form.remainder: empty on {open_rem_empty} of {open_seen}, \
         {open_rem_links} links across the rest"
    );
    // WHAT THE COVERS' ORBITS CLASSIFIED AS. `tally_orbit` has recorded this
    // all along and only the soak read it, so the mint sweep could say how
    // many orbits a scene has and not what any of them turned out to be.
    // `classify` files a cycle only where the carriage is the IDENTITY
    // (`cech::classify`), so an orbit with a finite non-identity rotor carried by
    // a non-identity versor stands as a class rather than filing as torsion.
    // This line is what says which of those two the corpus produces.
    let t = mycelium::transducer::family::cover::take_cover_tally();
    println!(
        "  mint cover orbits: edges {} flat {} filed {} standing {} unspellable {} \
         | covers measured {} unmeasured {} carried {}",
        t.edges, t.flat, t.filed, t.standing, t.unspellable, t.measured, t.unmeasured, t.carried
    );
}
