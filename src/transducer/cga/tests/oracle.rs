// BQ3, 71.2(3) — THE ORACLE: every operation of the algebra, read at n = 3
// and written down, so a carrier generic in n can be proved to reproduce it.
//
// The readings are taken over two kinds of input: fixtures built by the
// algebra's own constructors, and the distinct versors a corpus form
// actually carries on its faces and its members. Each reading is one line,
// `op input: value`, and a value is written exactly: a versor as the hex of
// its canonical bytes, a rational as its reduced fraction, a kind by name.
// Nothing is rounded and nothing is a digest.
//
// The golden file is written once by `the_oracle_is_written`, which is
// ignored and run by hand; `the_algebra_reproduces_its_oracle` reads it and
// takes the readings again. A difference names the first line that moved.

use crate::transducer::cga::{self, Mv};
use num_rational::BigRational;

const GOLDEN: &str = "src/transducer/cga/tests/oracle.golden";
const CORPUS: &str = "data/arc-agi-2/training";
/// The corpus slice the versors are harvested from, and how many distinct
/// ones are read. A slice of the corpus, in the directory's sorted order.
const TASKS: usize = 40;
const HARVEST: usize = 48;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

/// A versor written exactly: the hex of its canonical coefficients — the
/// value itself, without the `n` the wire frames it with (BQ1), so the
/// oracle the fixed carrier left is read against values and not a layout.
fn hex(m: &Mv) -> String {
    let mut b = Vec::new();
    match m.canonical() {
        Some(c) => c.put_coefficients(&mut b),
        None => m.put_coefficients(&mut b),
    }
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn opt(m: Option<Mv>) -> String {
    match m {
        Some(m) => hex(&m),
        None => "refused".into(),
    }
}

/// THE FIXTURES: the algebra's own constructors over a spread of exact
/// arguments, every bivector plane, and products of them.
fn fixtures() -> Vec<(String, Mv)> {
    let mut out: Vec<(String, Mv)> = Vec::new();
    let args = [q(0, 1), q(1, 1), q(-2, 3), q(5, 2), q(7, 1)];
    for (i, a) in args.iter().enumerate() {
        let b = &args[(i + 1) % args.len()];
        let c = &args[(i + 2) % args.len()];
        out.push((format!("point{i}"), cga::at_three::point(a, b, c)));
        out.push((format!("sphere{i}"), cga::at_three::sphere(a, b, c, &q(i as i64 + 1, 6))));
        out.push((format!("translator{i}"), cga::at_three::translator(a, b, c)));
        out.push((format!("rotator{i}"), cga::rotator(b, &q(1, 1))));
    }
    // The fixtures stand at the corpus's n = 3, named.
    for (i, plane) in cga::bivector_planes(3).iter().enumerate() {
        out.push((format!("rotor_on{i}"), cga::rotator_on_blade(&q(2, 1), &q(1, 3), *plane)));
    }
    for (i, rho) in [q(2, 1), q(1, 3), q(5, 2)].iter().enumerate() {
        out.push((format!("dilator{i}"), cga::dilator(rho)));
    }
    out.push(("reflector".into(), cga::reflector()));
    out.push(("e_o".into(), cga::e_o()));
    out.push(("e_inf".into(), cga::e_inf()));
    out.push(("pseudo".into(), Mv::blade(cga::pseudo(3), q(1, 1))));
    for (i, plane) in cga::transvection_planes(3).iter().enumerate() {
        if let Some(t) = cga::transvection(&q(3, 1), &q(1, 2), *plane) {
            out.push((format!("transvection{i}"), t));
        }
    }
    let lam = |r: i64, c: i64, k: i64| cga::at_three::hyperbolic_lamina(&q(r, 1), &q(c, 1), &q(k, 1), &q(9, 1));
    if let (Some(a), Some(b)) = (lam(1, 2, 0), lam(-3, 1, 2)) {
        out.push(("lamina_a".into(), a.clone()));
        if let Some(g) = cga::geodesic_transvection(&a, &b) {
            out.push(("geodesic".into(), g));
        }
    }
    // A screw: a turn composed with a displacement off its plane.
    out.push((
        "screw".into(),
        cga::rotator(&q(1, 1), &q(1, 1)).geo(&cga::at_three::translator(&q(0, 1), &q(0, 1), &q(1, 1))),
    ));
    out
}

/// THE HARVEST: the distinct versors the corpus's own forms carry, on the
/// faces (each member pair's carriage and 50.3 edge) and on the members.
/// Read in key order of their canonical bytes, so the slice is the corpus's
/// content and not the order the extractor happened to meet them in.
fn harvest() -> Vec<(String, Mv)> {
    let Ok(dir) = std::fs::read_dir(CORPUS) else { return Vec::new() };
    let mut names: Vec<String> = dir
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort_unstable();
    names.truncate(TASKS);
    let epi = crate::client::sdk::Epigenome::new();
    let mut physics = crate::physics::Physics::default();
    let mut held: crate::network::nerve::Keyed<Mv> = crate::network::nerve::Keyed::new();
    let mut hold = |m: &Mv| {
        if let Some(c) = m.canonical() {
            let mut k = Vec::new();
            c.put_coefficients(&mut k);
            held.insert(k, c);
        }
    };
    for name in &names {
        let Ok(json) = std::fs::read_to_string(format!("{CORPUS}/{name}")) else { continue };
        let Some(task) = crate::arc::parse_task(&json) else { continue };
        let mut budget = physics.stroke_n(crate::transducer::family::extraction_spend(&task.train));
        let Some(rule) = crate::transducer::family::extract_rule_with(&task.train, &epi, &mut budget)
        else {
            continue;
        };
        for face in &rule.form.faces {
            for (_, place) in face.at.scoped() {
                hold(&place.carriage);
                if let Some(e) = &place.edge {
                    hold(&e.carriage);
                }
            }
        }
        for v in &rule.form.vertices {
            if let crate::transducer::sentence::Vertex::Word { acts, .. } = v {
                for (_, a) in acts.scoped() {
                    hold(&a.carriage);
                }
            }
        }
    }
    held.scoped()
        .take(HARVEST)
        .enumerate()
        .map(|(i, (_, m))| (format!("corpus{i}"), m.clone()))
        .collect()
}

/// EVERY READING, one line each, over the inputs and their consecutive
/// pairs. The operations are the carrier's (sum, product, reverse, grade,
/// wedge, inverse, sandwich, the canonical writing) and the ones written on
/// it (class invariants, the two periods, the split, Cayley, the even
/// inverse, projective equality, the base point, the boundary dual).
pub(super) fn readings(inputs: &[(String, Mv)]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, (name, m)) in inputs.iter().enumerate() {
        let (next_name, next) = &inputs[(i + 1) % inputs.len()];
        let pair = format!("{name}*{next_name}");
        out.push(format!("canonical {name}: {}", hex(m)));
        out.push(format!("reverse {name}: {}", hex(&m.reverse())));
        for g in 0..=5u32 {
            out.push(format!("grade{g} {name}: {}", hex(&m.grade(g))));
        }
        out.push(format!("norm2 {name}: {}", m.norm2()));
        out.push(format!("inverse {name}: {}", opt(m.versor_inverse())));
        out.push(format!("geo {pair}: {}", hex(&m.geo(next))));
        out.push(format!("add {pair}: {}", hex(&m.add(next))));
        out.push(format!("sub {pair}: {}", hex(&m.sub(next))));
        out.push(format!("sandwich {pair}: {}", hex(&m.sandwich(next))));
        out.push(format!("wedge1 {pair}: {}", hex(&m.grade(1).wedge_vector(1, &next.grade(1)))));
        out.push(format!("quotient {pair}: {}", opt(Mv::quotient(m, next))));
        out.push(format!(
            "ratio {pair}: {}",
            match cga::projective_ratio(m, next) {
                Some(l) => l.to_string(),
                None => "refused".into(),
            }
        ));
        out.push(format!("projective_eq {pair}: {}", m.projective_eq(next)));
        out.push(format!(
            "class {name}: {}",
            match m.class_invariants() {
                Some((i1, i2, kind)) => format!("{i1} {i2} {kind:?}"),
                None => "refused".into(),
            }
        ));
        out.push(format!(
            "holonomy_key {name}: {}",
            match m.class_key() {
                Some(k) => k.as_bytes().iter().map(|x| format!("{x:02x}")).collect::<String>(),
                None => "refused".into(),
            }
        ));
        out.push(format!("finite_order {name}: {:?}", cga::finite_order(m)));
        out.push(format!("compact_period {name}: {:?}", cga::compact_period(m)));
        out.push(format!("simple {name}: {}", cga::bivector_is_simple(m)));
        out.push(format!(
            "split {name}: {}",
            match cga::bivector_split(m, &mut super::common::budget()) {
                Some((a, b)) => format!("{} {}", hex(&a), hex(&b)),
                None => "refused".into(),
            }
        ));
        let small = m.grade(2).scale(&q(1, 7));
        out.push(format!("cayley {name}: {}", opt(cga::cayley(&small))));
        out.push(format!(
            "inverse_even {name}: {}",
            opt(cga::inverse_even(&Mv::scalar(q(1, 1)).add(&m.grade(2))))
        ));
        out.push(format!(
            "base {name}: {}",
            match cga::at_three::base(m) {
                Some((a, b, c)) => format!("{a} {b} {c}"),
                None => "refused".into(),
            }
        ));
        out.push(format!("boundary_dual {name}: {}", opt(cga::boundary_dual(m, 3))));
        out.push(format!("commutator {pair}: {}", opt(cga::transvection_commutator(m, next))));
        out.push(format!("frame_base {name}: {}", opt(cga::frame_base_point(m))));
        let key = m.key();
        out.push(format!(
            "key_roundtrip {name}: {}",
            match key.as_ref().and_then(|k| Mv::from_key(k.as_bytes())) {
                Some(back) => hex(&back),
                None => "refused".into(),
            }
        ));
    }
    for (i, plane) in cga::bivector_planes(3).iter().enumerate() {
        out.push(format!(
            "plane {i}: {} {} harmonic {}",
            plane,
            cga::plane_name(*plane),
            cga::plane_is_harmonic(*plane)
        ));
    }
    out
}

/// The inputs the oracle is read over: the fixtures, then the harvest.
pub(super) fn inputs() -> Vec<(String, Mv)> {
    let mut all = fixtures();
    all.extend(harvest());
    all
}

/// WRITES THE GOLDEN FILE. Run by hand, once, on the carrier the oracle
/// stands for; never part of a verification (§6.4: verification never
/// writes).
#[test]
#[ignore = "writes the golden file; run by hand on the fixed carrier"]
fn the_oracle_is_written() {
    let lines = readings(&inputs());
    std::fs::write(GOLDEN, lines.join("\n") + "\n").expect("the golden file is written");
    eprintln!("  oracle: {} readings written to {GOLDEN}", lines.len());
}

/// THE ALGEBRA REPRODUCES ITS ORACLE AT n = 3: every reading taken again
/// and compared, line for line, with the golden file.
#[test]
fn the_algebra_reproduces_its_oracle() {
    let Ok(golden) = std::fs::read_to_string(GOLDEN) else {
        panic!("{GOLDEN} does not read: the oracle has not been written")
    };
    let want: Vec<&str> = golden.lines().collect();
    let got = readings(&inputs());
    assert!(got.iter().any(|l| l.starts_with("canonical corpus")), "the harvest read no corpus versor");
    let first = want.iter().zip(&got).position(|(w, g)| *w != g.as_str());
    assert!(
        first.is_none() && want.len() == got.len(),
        "the algebra moved at line {:?} of {} (want {}, got {}): {:?} against {:?}",
        first,
        want.len(),
        want.len(),
        got.len(),
        first.map(|i| want[i]),
        first.map(|i| &got[i])
    );
    eprintln!("  oracle: {} readings reproduced", got.len());
}
