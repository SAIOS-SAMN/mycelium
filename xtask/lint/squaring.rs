// Law 11 — THE SQUARING GATE: a coordinate pair lives at the sensor
// boundary and nowhere above it.
//
//! Above the sensor everything is versors and words. The functions in
//! `SENSOR` are the boundary itself; the exemption is by FUNCTION name and
//! holds only inside that function's braces, so a pair outside them is a hit.

use super::walker::Gate;
use std::path::Path;

/// The sensor boundary, as (file, function) pairs. A bare function name is
/// matched by substring in every file, and `fn base(` then exempts
/// `cga.rs`, `fn carry(` exempts `alphabet/rotor.rs`, `fn carriage(`
/// exempts `element/letters.rs`, `element/vertex.rs` and `family/mod.rs` —
/// none of which is the sensor. The file is half of the row.
///
/// Each row is EARNED: if the named function is not in that file, or is and
/// contains no pair, the row is a hit and is deleted.
const SENSOR: &[(&str, &str)] = &[
    ("transducer/element/sensor.rs", "fn cell_of"),
    ("transducer/element/sensor.rs", "fn carry_center"),
    ("transducer/element/sensor.rs", "fn carry_cell"),
    ("transducer/element/letters.rs", "fn carriage("),
    ("transducer/blade/carry.rs", "fn carry("),
    ("transducer/blade/carry.rs", "fn rotor_between"),
    ("transducer/blade/carry.rs", "fn half_rotor"),
    ("transducer/blade/body.rs", "fn centroid("),
    ("transducer/alphabet/letters.rs", "fn rotor_pair"),
    ("transducer/cga.rs", "fn base("),
];

pub(crate) const GATE: Gate = Gate {
    name: "squaring",
    allow: &[],
    scan,
};

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    let p = path.to_string_lossy().replace('\\', "/");
    let rows: Vec<&(&str, &str)> = SENSOR.iter().filter(|(f, _)| p.ends_with(f)).collect();
    let mut earned = vec![false; rows.len()];
    let mut entered = vec![false; rows.len()];
    let mut inside: Option<usize> = None;
    let mut opened = false;
    let mut depth = 0i32;
    for (i, line) in src.lines().enumerate() {
        let t = line.trim();
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        if inside.is_none() {
            if let Some(k) = rows.iter().position(|(_, f)| t.contains(f)) {
                inside = Some(k);
                entered[k] = true;
                opened = false;
                depth = 0;
            }
        }
        let pair = pair_on(t);
        if let Some(k) = inside {
            depth += opens - closes;
            if opens > 0 {
                opened = true;
            }
            if pair {
                earned[k] = true;
            }
            // LEAVE when the body that was opened closes. A closing brace
            // stands alone on its line, so a guard requiring `opens > 0`
            // here never fires and every line after the first sensor
            // function in the file stays exempt (F3i).
            if opened && depth <= 0 {
                inside = None;
            }
            continue;
        }
        if pair {
            hits.push(format!(
                "{}:{}: (Q, Q) above the sensor boundary (law 11)",
                path.display(),
                i + 1
            ));
        }
        let role = t.contains("placement:") || t.contains("shift:") || t.contains("residue:");
        let int_pair = t.contains("(i64, i64)") || t.contains("(i64,i64)");
        if role && int_pair {
            hits.push(format!(
                "{}:{}: (i64, i64) as a placement/shift/residue above the sensor (law 11)",
                path.display(),
                i + 1
            ));
        }
    }
    for (k, (file, f)) in rows.iter().enumerate() {
        if !entered[k] {
            hits.push(format!("{file}: the squaring gate exempts `{f}` and the file has none — delete the row"));
        } else if !earned[k] {
            hits.push(format!("{file}: `{f}` is exempt and carries no pair — delete the row"));
        }
    }
}

/// Checklist (f') names `(i64, i64)` beside the rational pairs, and it
/// cannot be matched by TYPE — `(i64, i64)` is the cell coordinate itself,
/// legitimate at the sensor. It is matched by ROLE above.
fn pair_on(t: &str) -> bool {
    t.contains("(Q, Q)")
        || t.contains("(Q,Q)")
        || t.contains("&(Q, Q)")
        || t.contains("BigRational, BigRational")
        || t.contains("BigRational, num_rational")
        || t.contains("num_rational::BigRational, num_rational::BigRational")
}

#[cfg(test)]
mod tests {
    use super::scan;
    use std::path::Path;

    /// F3i: the exemption CLOSES. A guard requiring `opens > 0` on the exit
    /// never fires — a closing brace stands alone — so every line after the
    /// first sensor function in a file stayed exempt. Thirteen production
    /// files were blind from that line to their end.
    #[test]
    fn a_pair_after_a_sensor_function_is_a_hit() {
        let src = "\
pub fn cell_of(p: &(Q, Q)) -> (i64, i64) {
    (0, 0)
}

pub fn above(t: &(Q, Q)) -> bool {
    t.0 == t.1
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/transducer/element/sensor.rs"), src, &mut hits);
        let pairs: Vec<&String> = hits.iter().filter(|h| h.contains("above the sensor")).collect();
        assert_eq!(pairs.len(), 1, "the pair inside cell_of is exempt, the one after it is not:\n{hits:?}");
        assert!(pairs[0].contains(":5:"), "and it is the one after the function:\n{pairs:?}");
    }

    /// The row is a (file, function) pair: the same function name in
    /// another file exempts nothing.
    #[test]
    fn a_sensor_name_in_another_file_exempts_nothing() {
        let src = "pub fn cell_of(p: &(Q, Q)) -> (i64, i64) {\n    (0, 0)\n}\n";
        let mut hits = Vec::new();
        scan(Path::new("src/transducer/sheaf.rs"), src, &mut hits);
        let pairs: Vec<&String> = hits.iter().filter(|h| h.contains("above the sensor")).collect();
        assert_eq!(pairs.len(), 1, "sheaf.rs is not the sensor:\n{hits:?}");
    }
}
