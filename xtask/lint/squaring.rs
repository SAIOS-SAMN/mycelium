// Law 11 — THE SQUARING GATE: a coordinate pair lives at the sensor
// boundary and nowhere above it.
//
//! Above the sensor everything is versors and words. The functions in
//! `SENSOR` are the boundary itself — they read or write a cell and nothing
//! else does. The exemption is by FUNCTION name and holds only inside that
//! function's braces (§4.2), so a pair in the same file but outside them is
//! a hit.

use super::walker::Gate;
use std::path::Path;

/// The sensor boundary, by function name. `element/sensor.rs` names four of
/// them in its own header; these are the only places a pair exists.
const SENSOR: &[&str] = &[
    "fn pair_form",
    "fn pair_form_uncached",
    "fn speak_rule",
    "fn orbit(",
    "fn cell_of",
    "fn carry_center",
    "fn carry_cell",
    "fn carriage(",
    "fn base(",
    "fn centroid(",
    "fn carry(",
    "fn rotor_between",
    "fn half_rotor",
    "fn rotor_pair",
];

pub(crate) const GATE: Gate = Gate {
    name: "squaring",
    allow: &[],
    scan,
};

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    let mut fn_depth = 0i32;
    let mut in_sensor = false;
    for (i, line) in src.lines().enumerate() {
        let t = line.trim();
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        if !in_sensor && SENSOR.iter().any(|f| t.contains(f)) {
            in_sensor = true;
            fn_depth = 0;
        }
        if in_sensor {
            fn_depth += opens - closes;
            if fn_depth <= 0 && opens > 0 {
                in_sensor = false;
            }
            continue;
        }
        let pair = t.contains("(Q, Q)")
            || t.contains("(Q,Q)")
            || t.contains("&(Q, Q)")
            || t.contains("BigRational, BigRational")
            || t.contains("BigRational, num_rational")
            || t.contains("num_rational::BigRational, num_rational::BigRational");
        // Checklist (f') names `(i64, i64)` beside the rational pairs, and
        // it cannot be matched by TYPE — `(i64, i64)` is the cell coordinate
        // itself, legitimate at the sensor. It is matched by ROLE, which is
        // what (f') says: a pair carried as a placement, a shift or a
        // residue is a squared placement whatever its components are.
        let role = t.contains("placement:") || t.contains("shift:") || t.contains("residue:");
        let int_pair = t.contains("(i64, i64)") || t.contains("(i64,i64)");
        if role && int_pair {
            hits.push(format!(
                "{}:{}: (i64, i64) as a placement/shift/residue above the sensor (law 11)",
                path.display(),
                i + 1
            ));
        }
        if pair {
            hits.push(format!(
                "{}:{}: (Q, Q) above the sensor boundary (law 11)",
                path.display(),
                i + 1
            ));
        }
    }
}
