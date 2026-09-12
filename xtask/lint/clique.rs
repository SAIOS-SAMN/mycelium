// Law 3 — THE CLIQUE GATE: a loop nested inside a loop over the same
// carrier, and the index pair that walks a set against itself.
//
//! The enumeration gate reads one line at a time, so it cannot see a loop
//! INSIDE a loop; both cliques that took the box passed it clean. This gate
//! is brace-depth aware and reads the two shapes law 3 forbids:
//!
//! - `for b in (a + 1)..` under `for a in 0..xs.len()` — a set walked
//!   against itself, k(k−1)/2 of a carrier the loop already holds whole.
//!   `star_and_cell_faces` and `cover_deformation` step (d) were both this,
//!   and each turned k occupants of one cell into k(k−1)/2 faces.
//! - the same iterable walked inside itself — `for x in c.fiber(ROOT)`
//!   under another `for _ in c.fiber(ROOT)`.
//!
//! A `for` whose `{` is on a later line is attached when that brace opens,
//! so a wrapped header is read like any other.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "clique",
    allow: ALLOW,
    owed: &[],
    scan,
};

/// Two files where the pairing IS the object, not a walk over a carrier
/// that names one index. Each row is earned: the gate does hit the file,
/// and the hit is the operation's own definition.
const ALLOW: &[(&str, &str)] = &[
    // `magnified` and `reduced` are ×k: one cell becomes a k×k block and a
    // k×k block condenses to one hue. The k² is the magnification, and k
    // is the scale asked for, not a carrier that grows.
    (
        "transducer/blade/shells.rs",
        "magnified and reduced are k×k by definition",
    ),
    // The pairs of ONE Čech cell's vertices are that simplex's EDGES —
    // `faces_ready` asks whether the 1-skeleton exists, and `fill` solves
    // the gauge over it. Rule 10 bounds a loop by a Budget or by one
    // cell's boundary, and a cell's vertices are one cell's boundary.
    (
        "transducer/cech/coboundary.rs",
        "the pairs of one cell's vertices are its edges (rule 10)",
    ),
];

/// One open `for` body: the depth its brace opened at, what it binds, and
/// what it walks.
struct Open {
    depth: i32,
    binds: String,
    walks: String,
}

/// The binding and the iterable of a `for` header, or `None` when the line
/// is not one. The iterable keeps its text: two walks are the same walk
/// when they are written the same way, and a gate that normalised them
/// further would be guessing at what a caller meant.
///
/// An iterable that comes back EMPTY is a header whose `in` ends the line —
/// `for b in` with the range wrapped under it. The caller reads the next
/// line as the iterable rather than dropping the header, because a wrapped
/// pair loop is a pair loop.
fn header(t: &str) -> Option<(String, String)> {
    let rest = t.strip_prefix("for ")?;
    if let Some(pat) = rest.strip_suffix(" in") {
        return Some((pat.trim().to_string(), String::new()));
    }
    let (pat, iter) = rest.split_once(" in ")?;
    Some((pat.trim().to_string(), iterable(iter)))
}

/// The text of an iterable as the gate compares it: no trailing brace, no
/// borrow, no surrounding space.
fn iterable(t: &str) -> String {
    let t = t.trim().trim_end_matches('{').trim();
    t.strip_prefix('&').unwrap_or(t).trim().to_string()
}

/// A range between two integer LITERALS — `0..16` for the blades of the
/// algebra, `0..2` for the two sides. It is not a carrier: nothing it
/// walks can grow, because both ends are written in the source. Law 3 is
/// about what the carrier names, so a bound that is a number is not this
/// gate's business at either end of a nesting.
fn constant_range(w: &str) -> bool {
    let flat: String = w.chars().filter(|c| !c.is_whitespace()).collect();
    let flat = flat.trim_end_matches('=').to_string();
    match flat.split_once("..") {
        Some((lo, hi)) => {
            let hi = hi.strip_prefix('=').unwrap_or(hi);
            lo.parse::<i64>().is_ok() && hi.parse::<i64>().is_ok()
        }
        None => false,
    }
}

/// The name a `let` binds on this line. A body that rebinds a name is
/// walking a DIFFERENT carrier under it, however the outer one was
/// spelled: `decide_neighbors` walks `self.complex.link(..)` as `keys` and
/// then, inside, a bubble's own sections as `keys` again. Two carriers,
/// one name, and no clique.
fn rebound(t: &str) -> Option<String> {
    let rest = t.strip_prefix("let ")?;
    let rest = rest.strip_prefix("mut ").unwrap_or(rest);
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

/// The identifiers a `for` pattern binds: `x`, `(a, b)`, `&(r, c)` all
/// answer the names a nested bound can be written against.
fn bound_names(pat: &str) -> Vec<String> {
    pat.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|s| !s.is_empty() && !matches!(*s, "mut" | "ref"))
        .map(|s| s.to_string())
        .collect()
}

/// Does this iterable start its range at an enclosing binding plus one?
/// `(a + 1)..`, `(a+1)..` and `a + 1..` are one shape written three ways.
fn opens_at_successor(walks: &str, name: &str) -> bool {
    let flat: String = walks.chars().filter(|c| !c.is_whitespace()).collect();
    let want = format!("({name}+1)..");
    let bare = format!("{name}+1..");
    flat.starts_with(&want) || flat.starts_with(&bare)
}

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    let mut open: Vec<Open> = Vec::new();
    let mut depth = 0i32;
    // A header whose `{` has not arrived yet. It is attached to the next
    // brace that opens, which is that loop's own body.
    let mut pending: Option<(String, String)> = None;
    for (i, line) in src.lines().enumerate() {
        let t = line.trim();
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        // A header still waiting for its iterable takes this line as that
        // iterable; one waiting only for its brace is carried forward.
        let head = match pending.take() {
            Some((binds, walks)) if walks.is_empty() => Some((binds, iterable(t))),
            Some(h) => Some(h),
            None => header(t),
        };
        if let Some((binds, walks)) = head {
            match opens > 0 && !walks.is_empty() {
                false => pending = Some((binds, walks)),
                true => {
                    for enclosing in open.iter().filter(|o| !constant_range(&o.walks)) {
                        if constant_range(&walks) {
                            break;
                        }
                        if enclosing.walks == walks {
                            hits.push(format!(
                                "{}:{}: `{walks}` is walked inside itself (law 3)",
                                path.display(),
                                i + 1
                            ));
                            break;
                        }
                        if bound_names(&enclosing.binds)
                            .iter()
                            .any(|n| opens_at_successor(&walks, n))
                        {
                            hits.push(format!(
                                "{}:{}: the pairs of one carrier — `{walks}` under `{}` (law 3)",
                                path.display(),
                                i + 1,
                                enclosing.binds
                            ));
                            break;
                        }
                    }
                    open.push(Open { depth: depth + opens, binds, walks });
                }
            }
        }
        if let Some(name) = rebound(t) {
            let dotted = format!("{name}.");
            open.retain(|o| o.walks != name && !o.walks.starts_with(&dotted));
        }
        depth += opens - closes;
        open.retain(|o| o.depth <= depth);
    }
}

#[cfg(test)]
mod tests {
    use super::scan;
    use std::path::Path;

    /// The shape both cliques had: a set walked against itself by index.
    #[test]
    fn the_pairs_of_one_carrier_are_a_hit() {
        let src = "\
fn f(here: &[u8]) {
    for a in 0..here.len() {
        for b in (a + 1)..here.len() {
            let _ = (a, b);
        }
    }
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/transducer/family/cover.rs"), src, &mut hits);
        assert_eq!(hits.len(), 1, "the inner loop is the hit: {hits:?}");
        assert!(hits[0].contains(":3:"), "and it is named at the inner line: {hits:?}");
    }

    /// The same written without the parentheses is the same shape.
    #[test]
    fn a_successor_range_without_parentheses_is_the_same_hit() {
        let src = "\
for a in 0..xs.len() {
    for b in a + 1..xs.len() {
    }
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/x.rs"), src, &mut hits);
        assert_eq!(hits.len(), 1, "one shape, three spellings: {hits:?}");
    }

    /// One carrier walked inside its own walk.
    #[test]
    fn a_carrier_walked_inside_itself_is_a_hit() {
        let src = "\
for (k, v) in c.fiber(ROOT) {
    for (j, w) in c.fiber(ROOT) {
    }
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/x.rs"), src, &mut hits);
        assert_eq!(hits.len(), 1, "the nested walk is the hit: {hits:?}");
        assert!(hits[0].contains("walked inside itself"), "{hits:?}");
    }

    /// THE CHAIN IS NOT A CLIQUE. What both fixes replaced the pair loop
    /// with must pass, or the gate would forbid the remedy.
    #[test]
    fn a_chain_over_one_carrier_is_clean() {
        let src = "\
for (_k, verts) in occ.fiber(ROOT) {
    for w in verts.windows(2) {
        let _ = w;
    }
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/x.rs"), src, &mut hits);
        assert!(hits.is_empty(), "a chain walks each carrier once: {hits:?}");
    }

    /// Two different carriers nested is ordinary work and not this gate's
    /// business: the gate reads a carrier against ITSELF.
    #[test]
    fn two_different_carriers_nested_are_clean() {
        let src = "\
for lp in &self.loops {
    for v in &lp.vertices {
        let _ = v;
    }
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/x.rs"), src, &mut hits);
        assert!(hits.is_empty(), "a loop's own vertices are its boundary: {hits:?}");
    }

    /// A RANGE OF LITERALS IS NOT A CARRIER. `0..16` is the blades of the
    /// algebra and `0..2` the two sides; neither can grow, so nesting them
    /// is a constant and not law 3's business. Checked at both ends of the
    /// nesting, because exempting only the inner loop would still report
    /// the pair when the outer one is the constant.
    #[test]
    fn a_range_of_literals_is_not_a_carrier() {
        let src = "\
for i in 0..16 {
    for j in 0..16 {
    }
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/transducer/cga.rs"), src, &mut hits);
        assert!(hits.is_empty(), "sixteen blades are sixteen blades: {hits:?}");

        let outer_const = "\
for i in 0..16 {
    for x in c.fiber(ROOT) {
        for y in c.fiber(ROOT) {
        }
    }
}
";
        let mut nested = Vec::new();
        scan(Path::new("src/x.rs"), outer_const, &mut nested);
        assert_eq!(nested.len(), 1, "a real clique under a constant still hits: {nested:?}");
    }

    /// A NAME REBOUND IN THE BODY IS A DIFFERENT CARRIER. `decide_neighbors`
    /// walks the complex's link as `keys`, then inside walks a bubble's own
    /// sections as `keys` — two carriers, one name. Reading that as a
    /// carrier walked inside itself is a false hit, and a gate that cries
    /// at correct code is a gate that gets switched off.
    #[test]
    fn a_name_rebound_in_the_body_is_a_different_carrier() {
        let src = "\
for ck in keys {
    let keys: Vec<Vec<u8>> = b.nerve.sections.fiber(ROOT).map(|(k, _)| k).collect();
    for sk in keys {
    }
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/transducer/bubble/merge.rs"), src, &mut hits);
        assert!(hits.is_empty(), "the inner `keys` is not the outer one: {hits:?}");

        // And without the rebinding it IS the same carrier.
        let same = "\
for ck in keys {
    for sk in keys {
    }
}
";
        let mut real = Vec::new();
        scan(Path::new("src/x.rs"), same, &mut real);
        assert_eq!(real.len(), 1, "unshadowed, it is a clique: {real:?}");
    }

    /// A header wrapped onto a second line is read like any other, and the
    /// body it opens is still the body the nesting is measured against.
    #[test]
    fn a_wrapped_header_still_opens_a_body() {
        let src = "\
for a in 0..xs.len() {
    for b in
        (a + 1)..xs.len()
    {
    }
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/x.rs"), src, &mut hits);
        assert_eq!(hits.len(), 1, "a wrapped header is a header: {hits:?}");
    }

    /// The body CLOSES: a pair loop after a sibling loop has ended is not
    /// nested in it, and reporting it would be the squaring gate's F3i
    /// fault in a new file.
    #[test]
    fn a_sibling_loop_is_not_nested() {
        let src = "\
for a in 0..xs.len() {
    let _ = a;
}
for b in 0..xs.len() {
    let _ = b;
}
";
        let mut hits = Vec::new();
        scan(Path::new("src/x.rs"), src, &mut hits);
        assert!(hits.is_empty(), "the first body closed before the second opened: {hits:?}");
    }
}
