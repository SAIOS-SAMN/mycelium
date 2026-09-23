// K6.4's reading — WHAT A GENOME FILE HOLDS THAT ITS ADDRESSES DO NOT.
//
//! 54.5 rules the genome on disk to be the trie of addresses, "and a word's
//! value on disk is nothing, because the key is the form". Before that is
//! built, the claim has to be measured: `Value = Form ∖ Key` is a
//! statement about real words, and a word that carries something the key
//! does not determine would be lost by an empty value (law 7).
//!
//! This reads a GEN3 file word by word and reports, per word and in total:
//! the tape it occupies now, the address it would occupy instead, and what
//! it carries BEYOND its form — the magnitude, the A5 chain, the punctured
//! sections, the torsion cycles, the anchor. Nothing is decided here; it
//! is a reading, and the fix follows it rather than this file (§8.1).
//!
//! IT READS THE ENGINE, NOT A LOG. Every other number this session came
//! from an `eprintln!` line grepped out of a journal, which is string
//! matching a trace rather than asking the engine a question. This asks
//! the carrier directly, at a coordinate the operator names.

use mycelium::transducer::element::Element;

/// One word's accounting, in bytes.
#[derive(Default)]
struct Word {
    /// The tape the file holds for it today.
    tape: usize,
    /// The address it is reached by — the trie path under 54.5, and the
    /// whole of its identity.
    address: usize,
    /// The magnitude: four lanes, never summed (law 5). NOT derivable from
    /// the form — it is the descent measured over the bundle, which needs
    /// the train — so it is `Form ∖ Key` and the value must carry it.
    mag: usize,
    /// The A5 chain a residual word carries. For a SENTENCE this is empty
    /// and the remainder rides the form; for a taught residual it is the
    /// word's substance, and it is already inside the address (the key IS
    /// the chain).
    sub: usize,
    /// The punctured axis and its torsion cycles — the fibre a word
    /// deepened. Neither is in a form's key.
    sections: usize,
    torsion: usize,
    /// Whether the word is a sentence (a form) or a residual chain.
    sentence: bool,
}

/// A word whose FORM does not decode has no address reading here, and the
/// reading says so rather than taking what `Element::address` falls back
/// to. For a sentence the address IS `form_vertex_key` of the decoded
/// form; when the tape's version is not this build's, `vertex_address`
/// answers `None` and the fallback is the non-sentence path — a short key
/// that is not the form's and would read as a 50x saving that is not
/// there. Rule 11: exact or refused.
fn read(e: &Element) -> Option<Word> {
    // A sentence whose tape this build cannot read has no address reading:
    // `vertex_address` answers `None` and `Element::address` falls back to
    // the non-sentence path, a short key that is NOT the form's.
    let readable = !e.is_sentence()
        || e.description().get(1).copied()
            == Some(mycelium::transducer::sentence::FORM_TAPE_VERSION);
    if !readable {
        return None;
    }
    let mut w = Word {
        tape: e.description().len(),
        address: e.address().len(),
        mag: 32,
        sub: e.sub().len() * 18,
        sentence: e.is_sentence(),
        ..Word::default()
    };
    // ASK THE CARRIER THAT HOLDS IT, AND FOR A SENTENCE THAT IS THE FORM.
    //
    // This read `e.sections()` and `e.torsion()` for every word, and those
    // are the ELEMENT's two fibre carriers. A minted sentence leaves both
    // empty because its cycles are on the form — `Form::fibre_volume` says
    // so in its own doc, and says that asking the element "was how the
    // instrument came to report 1 everywhere by construction". This
    // instrument then made the same mistake one row down and reported
    // `torsion cycles 0` on every voice of a live fleet, which is a fact
    // about where it looked and not about the engine.
    //
    // `tape_parts` reads the form, and using it here is what makes the two
    // halves of this file agree rather than two readers of one tape
    // disagreeing (law 12).
    //
    // BUT ONLY TORSION MOVES CARRIER, AND SECTIONS DO NOT. They are two
    // different objects with one name, which this line got wrong in both
    // directions before it got it right:
    //
    //   `Element.sections` — transition ASSIGNMENTS over a base, the free
    //   part. NOT in the key, so it belongs in `Form ∖ Key`. Empty for a
    //   sentence, because a form has no free-section carrier and none is
    //   substituted (`Form::fibre_volume`'s own line).
    //
    //   `Form.sections` — class key → PAIR INDICES of the bundle, which
    //   `form_vertex_key` writes out whole. It IS in the key, and adding it
    //   to the value would count 1.3 MB of the key as what the key does not
    //   carry — which is the reading this instrument printed for one
    //   commit.
    //
    // Torsion is the one that genuinely changed carrier: `Form.torsion` is
    // what a sentence files and the key does not write it.
    w.torsion = match mycelium::transducer::sentence::tape_parts(e) {
        Some(p) => p.torsion,
        None => e.torsion().iter().map(|t| 4 + t.sigma.len() * 2).sum(),
    };
    w.sections = e.sections().iter().map(|s| 4 + s.len() * 2).sum();
    Some(w)
}

/// Walk a GEN3 genome file and report what its words carry.
pub(crate) fn report(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("  genome refused: {path} does not read");
        std::process::exit(1);
    };
    match bytes.get(..4) {
        Some(b"GEN3") => {}
        _ => {
            eprintln!("  genome refused: {path} is not GEN3");
            std::process::exit(1);
        }
    }
    let Some(n) = bytes.get(4..8).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes) else {
        eprintln!("  genome refused: no count");
        std::process::exit(1);
    };
    let mut pos = 8usize;
    let mut words: Vec<Word> = Vec::new();
    let mut unread = 0usize;
    for _ in 0..n {
        let Some(len) = bytes.get(pos..pos + 4).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes)
        else {
            break;
        };
        pos += 4;
        let len = len as usize;
        let Some(slice) = bytes.get(pos..pos + len) else { break };
        pos += len;
        let mut ep = 0usize;
        match Element::from_bytes(slice, &mut ep).as_ref().and_then(read) {
            Some(w) => words.push(w),
            None => unread += 1,
        }
    }
    let n = words.len();
    if n == 0 {
        println!(
            "  {path}: {unread} words whose form this build does not decode, and no reading taken."
        );
        println!(
            "    The file's tapes are not this build's `FORM_TAPE_VERSION`, so every address \n    would be `Element::address`'s non-sentence FALLBACK rather than the form's key.\n    A bytes-per-word reading needs a genome written by this build: recut first."
        );
        return;
    }
    let sum = |f: fn(&Word) -> usize| words.iter().map(f).sum::<usize>();
    let tape = sum(|w| w.tape);
    let address = sum(|w| w.address);
    let mag = sum(|w| w.mag);
    let sub = sum(|w| w.sub);
    let sections = sum(|w| w.sections);
    let torsion = sum(|w| w.torsion);
    let sentences = words.iter().filter(|w| w.sentence).count();
    let file = bytes.len();

    println!("  {path}: {n} words read, {unread} unread, {file} bytes on disk");
    println!("    {sentences} sentences, {} residual chains", n - sentences);
    println!();
    // EXACT INTEGERS, NOT FLOATS (law 4). A reading printed for a human
    // is still a reading: bytes and an integer quotient say what a
    // rounded decimal only suggests, and the engine holds no float.
    println!("    component                 total bytes   bytes/word");
    for (name, v) in [
        ("tape (today's value)", tape),
        ("address (the trie path)", address),
        ("magnitude", mag),
        ("A5 chain (in the address)", sub),
        ("punctured sections", sections),
        ("torsion cycles", torsion),
    ] {
        println!("    {name:<26} {v:>13} {:>12}", v / n);
    }
    // WHAT THE VALUE MUST CARRY under 54.5: what the key does not
    // determine. The A5 chain is inside the address already (`the key IS
    // the chain`), so it is NOT counted here; the magnitude is not
    // derivable from the form and the fibre's sections and torsion are
    // not in a form's key.
    let value = mag + sections + torsion;
    println!();
    println!(
        "    VALUE = Form \\ Key: {value} bytes, {} per word, {} parts in 10000 of the tape",
        value / n,
        match tape { 0 => 0, t => value * 10000 / t }
    );
    println!("    the addresses alone: {address} bytes, {} per word", address / n);
}

/// WHY THE TAPE IS SO MUCH LARGER THAN THE ADDRESS, reconciled by the
/// tape's own decomposition rather than left as a gap. A key of 0.9 KB
/// beside a tape of 46 KB is only believable once the 45 KB is named.
pub(crate) fn reconcile(path: &str) {
    let Ok(bytes) = std::fs::read(path) else { return };
    let Some(n) = bytes.get(4..8).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes) else {
        return;
    };
    let mut pos = 8usize;
    let mut total = mycelium::transducer::sentence::TapeParts::default();
    let mut read = 0usize;
    // The sentences themselves, kept for the homotopy reading below: it
    // asks each form for its orbits and its rank, which the byte accounting
    // does not carry.
    let mut held: Vec<((), Element)> = Vec::new();
    for _ in 0..n {
        let Some(len) = bytes.get(pos..pos + 4).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes)
        else {
            break;
        };
        pos += 4;
        let len = len as usize;
        let Some(slice) = bytes.get(pos..pos + len) else { break };
        pos += len;
        let mut ep = 0usize;
        let Some(e) = Element::from_bytes(slice, &mut ep) else { continue };
        let Some(p) = mycelium::transducer::sentence::tape_parts(&e) else { continue };
        read += 1;
        held.push(((), e.clone()));
        total.total += p.total;
        total.addr_word += p.addr_word;
        total.addr_glued += p.addr_glued;
        total.carriage += p.carriage;
        total.charts += p.charts;
        total.sections += p.sections;
        total.faces += p.faces;
        total.orbits += p.orbits;
        total.rotor += p.rotor;
        total.certs += p.certs;
        total.demands += p.demands;
        // THREE CARRIERS WERE COMPUTED AND NEITHER SUMMED NOR PRINTED, so
        // every byte of them landed in `rest` and the row that would have
        // shown the filed cycles did not exist. `torsion` is the one that
        // matters — the reading above reported it as zero from the wrong
        // carrier, and this row is where a non-zero would have contradicted
        // that. `axes` and `remainder` sat in `rest` with it.
        total.torsion += p.torsion;
        total.axes += p.axes;
        total.remainder += p.remainder;
        // AND THE FOUR `tape_parts` NAMES THAT THIS PRINTOUT DID NOT. Its
        // own `rest` subtracts eighteen components; this summed twelve, so
        // six of them read as "unaccounted" here while the carrier had
        // named them. A reading whose gap is its own omission is not a
        // reading of the tape (rule 13).
        total.cites += p.cites;
        total.gauge += p.gauge;
        total.glue_body += p.glue_body;
        total.frame += p.frame;
        total.key_total += p.key_total;
        total.key_cited += p.key_cited;
        total.glued_n += p.glued_n;
        total.glued_distinct += p.glued_distinct;
        total.rest += p.rest;
    }
    if read == 0 {
        return;
    }
    println!();
    println!("    the tape's own decomposition over {read} sentences:");
    let t = total.total.max(1);
    // DERIVED means `from_address` recomputes it and the file need not
    // hold it: the orbits and their carriages come from `orbits_of` on the
    // faces the key carries (§1.5: "loops and holonomies are derived and
    // never keyed"), and the certificates are evidence (54.2).
    for (name, v, derived) in [
        ("glued addresses", total.addr_glued, false),
        ("orbits + carriages", total.orbits, true),
        ("member carriages", total.carriage, false),
        ("charts", total.charts, false),
        ("sections", total.sections, false),
        ("faces", total.faces, false),
        ("filed torsion", total.torsion, false),
        ("opened axes", total.axes, false),
        ("A5 remainder", total.remainder, false),
        ("derivations' citations", total.cites, false),
        ("member gauges", total.gauge, false),
        ("glue bodies", total.glue_body, false),
        ("the tape's own frame", total.frame, false),
        ("certificates", total.certs, true),
        ("word addresses", total.addr_word, false),
        ("rotors", total.rotor, false),
        ("demands", total.demands, false),
    ] {
        println!(
            "      {name:<20} {v:>12} bytes {:>5} parts in 10000  {}",
            v * 10000 / t,
            if derived { "derived by from_address" } else { "in the key" }
        );
    }
    println!(
        "      {:<20} {:>12} bytes {:>5} parts in 10000",
        "unaccounted",
        total.rest,
        total.rest * 10000 / t as i64
    );
    // 1.7 PART 1, WHICH `key_cited` EXISTS TO ANSWER: how much of a word's
    // OWN NAME is cited addresses copied into it, and how many distinct
    // forms those bytes name. A citation that inlines the cited form's
    // whole subtree compounds; one that names it at a bounded coordinate
    // does not, and the two are told apart by `key_cited / key_total`
    // against `glued_distinct / glued_n` — the share of the name that is
    // copied, beside the number of distinct things it copies.
    let k = total.key_total.max(1);
    println!();
    println!("    the name itself, over the same {read} sentences:");
    println!(
        "      key {} bytes, {} per word; of which cited addresses {} bytes, {} parts in 10000",
        total.key_total,
        total.key_total / read,
        total.key_cited,
        total.key_cited * 10000 / k
    );
    println!(
        "      citations {} in all, {} distinct addresses among them",
        total.glued_n, total.glued_distinct
    );
    // H^1 AS THE GENOME ACTUALLY HOLDS IT. `coheres` decides what may be
    // STOOD, and it does so silently — a form it refuses is a `continue` at
    // `epigenome::crystal`, never a line on the wire — so the only way to
    // ask whether curvature reaches the genome is to ask the genome.
    //
    // Four lanes, never summed with each other (law 5): the rank's torsion,
    // free, off-plane and axes coordinates, each totalled over the words
    // that have a rank at all. `curved` counts the sentences holding at
    // least one orbit whose holonomy is not the identity — the number that is
    // zero exactly when the genome is flat.
    let (mut orbited, mut curved, mut ranked) = (0usize, 0usize, 0usize);
    let (mut paths, mut classed) = (0usize, 0usize);
    let (mut star_orbits, mut real_orbits) = (0usize, 0usize);
    let (mut chain_broke, mut chain_closed, mut chain_open, mut chain_break_step) = (0usize, 0usize, 0usize, 0usize);
    let (mut tape_orbits, mut rederived_orbits) = (0usize, 0usize);
    let mut dumped = 0usize;
    let (mut edged_orbits, mut edgeless_orbits) = (0usize, 0usize);
    let (mut torn_orbits, mut unchained_orbits) = (0usize, 0usize);
    let (mut split_orbits, mut verts, mut members, mut split_verts) = (0usize, 0usize, 0usize, 0usize);
    let mut rank = [0u64; 4];
    for (_, e) in &held {
        let held_word = mycelium::transducer::element::Held::of(e.clone());
        let Some(f) = held_word.form() else { continue };
        if f.orbits.is_empty() {
            continue;
        }
        orbited += 1;
        // `lp.holonomy` IS A COBOUNDARY AND CANNOT BE ASKED THIS. A face's
        // word is `w_j ∘ w_i⁻¹`, so a product of them around a cycle
        // telescopes to the identity: `delta1` reads `Finite(1)` on every
        // orbit of every form, always, and a reading taken off it says only
        // that `δ∘δ = 0`. `Form::orbit_words` is the true carrier — the EDGE
        // composite where a member path has one, the word where it does not
        // — which is what R8/D1 moved the FILER onto and what `coheres`
        // reaches through `orbit_is_congruent`.
        let words = f.orbit_words();
        if words.iter().any(|(w, _)| {
            !w.rotor().projective_eq(&mycelium::transducer::alphabet::Rotor::identity())
        }) {
            curved += 1;
        }
        paths += words.len();
        classed += words.iter().filter(|(_, k)| k.as_ref().is_some_and(|k| !k.is_zero())).count();
        // AN ORBIT OF FRAME STAR FACES CARRIES NOTHING BY CONSTRUCTION.
        // `paths_carried` seeds an orbit's edge half only where the orbit
        // crosses a face that does NOT touch the Frame, and 18 puts the
        // Frame at vertex zero by the form's own shape. An orbit every face
        // of which is a star face is a cycle through the basepoint and
        // relates no two bodies to each other, so it has no member path to
        // carry a holonomy — which is a different fact from a real orbit
        // whose bodies fail to close around it, and the two are what this
        // pair of counts tells apart.
        // THE ORBITS THE TAPE CARRIES, AND THE ORBITS THE FACES PROVE. The
        // tape writes `Orbit` whole and the decoder reads it back, so a
        // genome word's orbits are whatever the build that MINTED it
        // computed. Re-deriving them from the same form's faces says
        // whether the two agree; if they do not, the orbits on the tape are
        // a second statement of the incidence (law 12), gone stale.
        tape_orbits += f.orbits.len();
        rederived_orbits += mycelium::transducer::sentence::Form::orbits_of(&f.faces, &f.vertices).len();
        for lp in &f.orbits {
            let star = lp.faces.iter().all(|&fi| {
                f.faces.get(fi as usize).is_some_and(|fa| fa.i == 0 || fa.j == 0)
            });
            if star {
                star_orbits += 1;
                continue;
            }
            real_orbits += 1;
            // WHERE THE WALK DIES, on an orbit that crosses between bodies.
            // `paths_carried` steps a member from face to face and keeps the
            // paths that return to the chart they began at (`closed_key`
            // compares CHARTS, not carrier state — it does not demand flat
            // holonomy). So there are exactly two ways to get no path:
            //
            //   the orbit has a face with NO MEMBER PAIR, and the walk has
            //   nothing to step onto there — a tear in the cover; or
            //
            //   every face carries a pair and the CHARTS DO NOT CHAIN: the
            //   member standing at the far chart of one face is not at the
            //   near chart of the next, so the frontier empties.
            //
            // The two are different defects with different fixes and this
            // is what tells them apart.
            match lp.faces.iter().any(|&fi| {
                f.faces.get(fi as usize).is_some_and(|fa| fa.at.is_empty())
            }) {
                true => torn_orbits += 1,
                false => unchained_orbits += 1,
            }
            // DOES THE ORBIT EVEN PASS THROUGH A VERTEX THAT SPLITS? A cycle
            // of the vertex graph lifts to the member graph only where each
            // vertex it passes through contributes ONE member; a vertex
            // acting on several charts is several nodes to the face builder
            // and one node to the cycle basis, and that is where the lift
            // has to break. If no orbit passed through such a vertex the
            // account would be wrong, so it is counted rather than assumed.
            if lp.vertices.iter().any(|&vi| {
                f.vertices.get(vi as usize).is_some_and(|v| v.charts().len() > 1)
            }) {
                split_orbits += 1;
            }
            // WHERE THE CHART CHAIN BREAKS, walked the way `paths_carried`
            // walks it: at each step the face's pair must relate the chart
            // the path STANDS AT to the one it steps to, and the last step
            // must return to the chart it began at. A cycle built on the
            // member complex satisfies both by construction, so a break
            // here is a break between the basis and the walker — and the
            // step it happens at is the reading.
            let mut at: Option<Vec<u8>> = None;
            let mut first: Option<Vec<u8>> = None;
            let mut broke = false;
            for (win, &fi) in lp.vertices.windows(2).zip(&lp.faces) {
                let Some(fa) = f.faces.get(fi as usize) else { broke = true; break };
                // BOTH ORIENTATIONS WHERE THE VERTICES CANNOT SAY. A face
                // between two members of one vertex has `fa.i == fa.j`, so
                // the direction is not readable off the step's vertex pair
                // — the same ambiguity `paths_carried` resolves by stepping
                // both. Read one way here and this instrument would report
                // a break the engine does not have (law 12: two readers of
                // one walk).
                let runs: &[bool] = match fa.i == fa.j {
                    true => &[true, false],
                    false => &[fa.i == win[0] && fa.j == win[1]],
                };
                let mut stepped = false;
                'step: for &forward in runs {
                    for (pk, _) in fa.at.scoped() {
                        let Some((ik, jk)) =
                            mycelium::transducer::sentence::read_face_pair_key(&pk)
                        else {
                            continue;
                        };
                        let (here, there) = if forward { (ik, jk) } else { (jk, ik) };
                        if at.as_ref().is_some_and(|a| *a != here) {
                            continue;
                        }
                        if first.is_none() {
                            first = Some(here);
                        }
                        at = Some(there);
                        stepped = true;
                        break 'step;
                    }
                }
                if !stepped {
                    broke = true;
                    chain_break_step += 1;
                    if dumped < 1 {
                        dumped += 1;
                        println!("      --- one broken orbit ---");
                        println!("      vertices {:?} faces {:?}", lp.vertices, lp.faces);
                        println!("      standing at {:?}", at.as_ref().map(|a| &a[..a.len().min(6)]));
                        for (w2, &f2) in lp.vertices.windows(2).zip(&lp.faces) {
                            let Some(fb) = f.faces.get(f2 as usize) else { continue };
                            let ps: Vec<String> = fb.at.scoped().filter_map(|(k, _)| {
                                let (i2, j2) = mycelium::transducer::sentence::read_face_pair_key(&k)?;
                                Some(format!("({:?},{:?})", &i2[..i2.len().min(6)], &j2[..j2.len().min(6)]))
                            }).collect();
                            println!("        step {:?} face {f2} ends ({},{}) pairs {}", w2, fb.i, fb.j, ps.join(" "));
                        }
                    }
                    break;
                }
            }
            // DOES THE ORBIT CARRY 50.3'S EDGE AT ALL? `holonomy_words_on`
            // composes the EDGE transport where a member path has one and
            // falls back to the orbit's own WORD where it does not — and the
            // word is the coboundary `w_j ∘ w_i⁻¹`, which telescopes to the
            // identity around any cycle. So an orbit whose faces carry no
            // edge is flat by algebra whatever it turns by, and this is
            // what tells that apart from an orbit that is genuinely flat.
            // ON THE REAL FACES ONLY. A STAR FACE HAS NO EDGE BY DESIGN:
            // it is the trivialization every body is read against (2.2),
            // `edge_between` measures nothing across it because the Frame
            // carries no body at any frame, and `paths_carried` composes
            // the identity through it. Demanding one there reported "0
            // orbits carry the edge" on every voice — which was a fact
            // about this check, since every orbit's spanning tree is built
            // star-first and so every orbit has star faces.
            match lp.faces.iter().all(|&fi| {
                f.faces.get(fi as usize).is_some_and(|fa| {
                    let star = [fa.i, fa.j].iter().any(|&v| {
                        matches!(
                            f.vertices.get(v as usize),
                            Some(mycelium::transducer::sentence::Vertex::Frame)
                        )
                    });
                    star || fa.at.scoped().all(|(_, pl)| pl.edge.is_some())
                })
            }) {
                true => edged_orbits += 1,
                false => edgeless_orbits += 1,
            }
            match (broke, first == at) {
                (true, _) => chain_broke += 1,
                (false, true) => chain_closed += 1,
                (false, false) => chain_open += 1,
            }
        }
        for v in &f.vertices {
            let c = v.charts().len();
            verts += 1;
            members += c;
            if c > 1 {
                split_verts += 1;
            }
        }
        if let Some(r) = f.rank() {
            ranked += 1;
            rank[0] += u64::from(r.torsion);
            rank[1] += u64::from(r.free);
            rank[2] += u64::from(r.offplane);
            rank[3] += u64::from(r.axes);
        }
    }
    println!();
    println!("    the homotopy the genome holds, over the same {read} sentences:");
    println!(
        "      {orbited} carry an orbit, {curved} carry an orbit with NON-IDENTITY holonomy"
    );
    println!(
        "      {paths} member paths in all, {classed} of them standing at a NON-ZERO class"
    );
    println!(
        "      {star_orbits} orbits through the Frame alone, {real_orbits} crossing a face between two bodies"
    );
    println!(
        "      of the {real_orbits}: {torn_orbits} carry a face with NO member pair, {unchained_orbits} carry a pair on every face"
    );
    println!(
        "      {verts} vertices carry {members} members; {split_verts} act on more than one chart"
    );
    println!(
        "      {split_orbits} orbits pass through a vertex that splits into several members"
    );
    println!(
        "      the chart chain: {chain_closed} close, {chain_open} walk but do not return,          {chain_broke} break mid-walk ({chain_break_step} on a face with no matching pair)"
    );
    println!(
        "      {tape_orbits} orbits on the tapes, {rederived_orbits} re-derived from the same faces"
    );
    println!(
        "      50.3's edge: {edged_orbits} orbits carry it on every REAL face, {edgeless_orbits} do not"
    );
    println!(
        "      rank over the {ranked} that have one: torsion {} free {} off-plane {} axes {}",
        rank[0], rank[1], rank[2], rank[3]
    );
}

/// 54.5 — THE CITATION NERVE, WALKED AS A COMPLEX.
///
/// Every reading this session looked for curvature INSIDE a form and found
/// none: 36 corpus forms carry 361 vertices, 126 faces and ZERO orbits —
/// `E − V + C = 126 − 361 + 235 = 0`, a forest by edge starvation. Grade 1
/// is silent because there is no 1-cycle to be flat or curved.
///
/// THE NETWORK IS THE NERVE, and it was never traversed. A citation is a
/// 1-simplex `⟨A_i, A_j, φ⟩` of `N(𝒰)` and the engine treated it as a
/// pointer inlined into a key. That inlining is not merely expensive, it is
/// a PROOF OF ACYCLICITY: if `key(A)` contains `key(B)` then `|A| > |B|`,
/// so by well-ordering of lengths no directed cycle `A → … → A` can exist
/// in the address space at all. The cognitive manifold was forced to be a
/// directed acyclic forest by its own naming scheme.
///
/// The UNDIRECTED 1-skeleton is not so constrained — `A → B`, `A → C`,
/// `B → D`, `C → D` is a diamond and carries a fundamental cycle — so
/// `b₁ = |E| − |V| + C` may be large even under inlining, and the
/// holonomy around those cycles is the Čech coboundary
/// `(δφ)_ijk = φ_jk · φ_ik⁻¹ · φ_ij` evaluated for real.
///
/// THE ADDRESSES ARE READ WITHOUT THE ELEMENT DECODER, and that matters
/// here: the citations are INLINED IN THE KEY, so `from_address` recovers
/// every `Glued` target from the address alone. The element's own record
/// layout moved at 54.5 and this reading is indifferent to it — the address
/// sits in the prefix, before anything that changed.
///
/// `|V|` IS `|citers ∪ targets|` AND NOT THE TARGET COUNT. 621 citations
/// naming 440 distinct targets from 288 sentences is a forest necessarily
/// if the two sets are disjoint (`|V| = 728 > |E| = 621`) and carries ~182
/// cycles if they coincide. The overlap is the quantity that decides it and
/// nothing has ever measured it.
pub(crate) fn nerve(path: &str) {
    use mycelium::network::nerve::Keyed;
    use mycelium::transducer::address::Address;
    use mycelium::transducer::sentence::from_address;
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("  nerve refused: {path} does not read");
        std::process::exit(1);
    };
    if bytes.get(..4) != Some(b"GEN3") {
        eprintln!("  nerve refused: {path} is not GEN3");
        std::process::exit(1);
    }
    let Some(n) = bytes.get(4..8).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes) else {
        eprintln!("  nerve refused: no count");
        std::process::exit(1);
    };
    // THE ADDRESS OUT OF A RECORD'S PREFIX: magic, the description's length
    // and bytes, then the address's. Nothing past it is touched.
    let address_of = |rec: &[u8]| -> Option<Address> {
        let u32_at = |p: usize| -> Option<usize> {
            rec.get(p..p + 4).and_then(|b| b.try_into().ok()).map(|b| u32::from_le_bytes(b) as usize)
        };
        // THE MAGIC BY SLICE PATTERN, not by position (rule 15): the tag
        // leads the record by the layout's own shape.
        let [0xA3, ..] = rec else {
            return None;
        };
        let dlen = u32_at(1)?;
        let alen = u32_at(1 + 4 + dlen)?;
        let at = 1 + 4 + dlen + 4;
        Some(Address::new(rec.get(at..at + alen)?.to_vec()))
    };

    let mut pos = 8usize;
    let (mut records, mut unread, mut sentences) = (0usize, 0usize, 0usize);
    // Tapes this build does not read, by the version they claim.
    // KEYED, NEVER A BTREE (the standing rule): the version byte is its own
    // key and the trie walks it in order.
    let mut stale: mycelium::network::nerve::Keyed<usize> = mycelium::network::nerve::Keyed::new();
    // The 1-skeleton: every form that cites or is cited, and the edges.
    let mut node: Keyed<usize> = Keyed::new();
    let mut nodes: Vec<Address> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut citers = 0usize;
    for _ in 0..n {
        let Some(len) = bytes.get(pos..pos + 4).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes)
        else {
            break;
        };
        pos += 4;
        let Some(rec) = bytes.get(pos..pos + len as usize) else { break };
        pos += len as usize;
        records += 1;
        // WHAT VERSION THE TAPE SAYS IT IS, before anything is parsed off
        // it (rule 11). A reading taken against the wrong layout answers
        // ZEROS, and a zero that means "I could not read this" is
        // indistinguishable from a zero that means "there is nothing here"
        // — the empty-carrier collapse, which has misled six times this
        // session. The version byte is one byte and it ends the question.
        if let Some(v) = rec.get(5..7) {
            if v[0] == mycelium::transducer::element::SENTENCE_TAG
                && v[1] != mycelium::transducer::sentence::FORM_TAPE_VERSION
            {
                // A COUNT NOT YET SEEN IS THE FIRST (rule 11): absence here
                // is the honest "none of these yet", said by the match rather
                // than by a default standing in for a read.
                let at = match stale.get(&[v[1]]) {
                    Some(n) => *n + 1,
                    None => 1,
                };
                stale.insert([v[1]], at);
                continue;
            }
        }
        let Some(addr) = address_of(rec) else {
            unread += 1;
            continue;
        };
        // A FORM, BY ITS OWN TAG. A residual chain cites nothing.
        // A FORM BY ITS OWN TAG, read as the coordinate's leading byte
        // (A21) rather than as element zero of a list (rule 15).
        let [tag, ..] = addr.as_bytes() else { continue };
        if *tag != mycelium::transducer::address::TAG_FORM {
            continue;
        }
        sentences += 1;
        let Some(f) = from_address(&addr) else {
            unread += 1;
            continue;
        };
        let id_of = |k: &Address, node: &mut Keyed<usize>, nodes: &mut Vec<Address>| -> usize {
            if let Some(&i) = node.get(k.as_bytes()) {
                return i;
            }
            let i = nodes.len();
            node.insert(k.clone(), i);
            nodes.push(k.clone());
            i
        };
        let here = id_of(&addr, &mut node, &mut nodes);
        let mut cited_any = false;
        // 54.7 B5a — THE 1-SKELETON IS THE FORM'S COVER. It was walked off
        // the `Glued` vertices, and a vertex states a boundary rather than
        // an incidence; the incidence is what this audit measures.
        for e in &f.cover {
            let there = id_of(&e.target, &mut node, &mut nodes);
            edges.push((here, there));
            cited_any = true;
        }
        if cited_any {
            citers += 1;
        }
    }

    // CONNECTED COMPONENTS of the UNDIRECTED 1-skeleton, by union-find.
    let mut up: Vec<usize> = (0..nodes.len()).collect();
    fn find(up: &mut [usize], mut x: usize) -> usize {
        while up[x] != x {
            up[x] = up[up[x]];
            x = up[x];
        }
        x
    }
    for &(a, b) in &edges {
        let (ra, rb) = (find(&mut up, a), find(&mut up, b));
        if ra != rb {
            up[ra] = rb;
        }
    }
    let mut roots: Keyed<()> = Keyed::new();
    for i in 0..nodes.len() {
        let r = find(&mut up, i);
        roots.insert((r as u64).to_be_bytes(), ());
    }
    let (v, e, c) = (nodes.len(), edges.len(), roots.len());
    // b₁ = E − V + C, the cycle rank of the 1-skeleton. Zero is a forest:
    // every citation is a leaf and no two paths ever meet again.
    let b1 = (e as i64) - (v as i64) + (c as i64);
    // How many distinct forms are BOTH cited and citing — the overlap that
    // decides whether the graph can close at all.
    let mut cites_out: Keyed<()> = Keyed::new();
    let mut cited_in: Keyed<()> = Keyed::new();
    for &(a, b) in &edges {
        cites_out.insert((a as u64).to_be_bytes(), ());
        cited_in.insert((b as u64).to_be_bytes(), ());
    }
    let both = (0..nodes.len())
        .filter(|i| {
            cites_out.contains(&(*i as u64).to_be_bytes())
                && cited_in.contains(&(*i as u64).to_be_bytes())
        })
        .count();
    println!("  nerve {path}");
    println!(
        "      {records} records, {sentences} forms, {unread} unread; {citers} of them cite"
    );
    // THE REFUSAL IS THE READING when the layout is not this build's.
    // §10: a layout change is a recut and state written before it does not
    // restore, so a stale genome is not a genome with no nerve — it is a
    // genome this build cannot ask.
    if !stale.is_empty() {
        let rows: Vec<(u8, usize)> =
            stale
                .scoped()
                .filter_map(|(k, n)| match k.as_slice() {
                    [v, ..] => Some((*v, *n)),
                    [] => None,
                })
                .collect();
        let total: usize = rows.iter().map(|(_, n)| *n).sum();
        println!(
            "      REFUSED: {total} tapes are not this build's — versions {rows:?} against \
             FORM_TAPE_VERSION {}. The nerve is not measurable on this file; it needs the recut \
             (§10), and the numbers below are of what remained, not of the genome.",
            mycelium::transducer::sentence::FORM_TAPE_VERSION
        );
    }
    println!(
        "      1-skeleton: V {v} · E {e} · components {c} · b₁ = E − V + C = {b1}"
    );
    println!(
        "      {} distinct forms both cite and are cited — the overlap a cycle needs",
        both
    );
    println!(
        "      {} cite only, {} are cited only",
        cites_out.len() - both,
        cited_in.len() - both
    );
}
