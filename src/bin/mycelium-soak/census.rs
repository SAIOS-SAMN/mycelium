// 53.2's probe — THE CENSUS OF A FAMILY BY READING, not by its size.
//
//! WHICH FAMILIES ARE LARGE IS NOT WHAT DECIDES THE ORDER. A family is
//! large or small by how much of the grid it paints, and that number says
//! nothing about which move will read it. What decides K5's order is what
//! the family IS: whether the scene is cut by a separator, whether the
//! frames stand in integer ratio, whether the paint is one body or many.
//! Three readings, each a yes or a no on exact integers, taken on the task
//! files the journal's family ids map to through `registry::task_address`.
//!
//! THIS READS THE CORPUS AND CLAIMS NOTHING ABOUT THE MESH. As
//! ruled, this binary does not reflect the live feed and no mesh
//! number may be taken from it. Every number below is a property of the
//! ARC files on disk, which is why this instrument could be built while the
//! mesh was cut and soaking — it depends on no reading of it.
//!
//! EXACT OR REFUSED (law 4, rule 11). A frame ratio is integer division
//! with a zero remainder, never a float compared to a tolerance; a
//! separator is a full extent of one hue, never "mostly"; a component is
//! walked, never estimated.
//!
//! THE GATE REFUSED THE FIRST CUT OF THIS FILE AND WAS RIGHT EVERY TIME.
//! It sorted the page by painted-site count — rule 4, an order chosen by a
//! measurement — and took a `max` over the pairs to call one of them the
//! family's size, which is the same defect twice. The rows are keyed by the
//! task id and read back in the trie's order, which is canonical and is
//! content (law 2); the paint is the SUM over the train inputs, which is a
//! measure of the family and not a member elected to stand for it.

use mycelium::arc::{Grid, Task};
use mycelium::network::nerve::Keyed;

/// What one family reads as. Each field is a reading taken over EVERY
/// train pair, because a property that holds on some pairs and not others
/// is not a property of the family (rule 13: the claim, not the outcome).
struct Reading {
    /// Every train input carries a full row or column of one non-background
    /// hue — the scene is cut into panels before anything else is read.
    separator: bool,
    /// Every pair's output frame is an exact integer multiple — or exact
    /// integer divisor — of its input frame, on both axes.
    ratio: Option<(i64, i64)>,
    /// Every train input's painted sites form ONE 4-connected body.
    one_body: bool,
    /// The painted sites over the WHOLE train, summed. The only size
    /// reading here, reported beside the three and never instead of them.
    paint: usize,
    /// How many pairs the family demonstrates over.
    pairs: usize,
}

/// A FULL EXTENT OF ONE HUE. A separator row is a row whose every cell is
/// the same non-zero hue; a separator column is the same read the other
/// way. A grid too short to be cut carries none.
fn has_separator(g: &Grid) -> bool {
    let w = match g.as_slice() {
        [row, ..] => row.len(),
        [] => return false,
    };
    if w == 0 || g.len() < 3 {
        return false;
    }
    let row = g.iter().any(|r| match r.split_first() {
        Some((&h, rest)) => h != 0 && rest.iter().all(|&c| c == h) && r.len() == w,
        None => false,
    });
    if row {
        return true;
    }
    (0..w).any(|c| match g.split_first() {
        Some((first, rest)) => match first.get(c) {
            Some(&h) => h != 0 && rest.iter().all(|r| r.get(c) == Some(&h)),
            None => false,
        },
        None => false,
    })
}

/// THE FRAME RATIO, EXACT. `(a, b)` means the output is `a` times the input
/// on rows and `b` times on columns, where a NEGATIVE factor is the exact
/// reciprocal — the output is `|a|` times SMALLER. A ratio that is not a
/// whole number either way is no ratio at all and refuses (rule 11), and a
/// pair whose factors disagree with an earlier pair's refuses the family.
fn frame_ratio(input: &Grid, output: &Grid) -> Option<(i64, i64)> {
    let width = |g: &Grid| -> i64 {
        match g.as_slice() {
            [row, ..] => row.len() as i64,
            [] => 0,
        }
    };
    let (ih, iw) = (input.len() as i64, width(input));
    let (oh, ow) = (output.len() as i64, width(output));
    if ih == 0 || iw == 0 || oh == 0 || ow == 0 {
        return None;
    }
    let axis = |i: i64, o: i64| -> Option<i64> {
        match (o % i, i % o) {
            (0, _) => Some(o / i),
            (_, 0) => Some(-(i / o)),
            _ => None,
        }
    };
    Some((axis(ih, oh)?, axis(iw, ow)?))
}

/// The site's coordinate as the key it is written under (law 2, rule 6):
/// row and column big-endian, so the trie's order is the reading order.
///
/// Each as `u64`, which every `usize` coordinate fits: `as u32` truncated,
/// and two sites a multiple of 2³² apart were one key (a defect, not a
/// width).
fn site_key(r: usize, c: usize) -> Vec<u8> {
    let mut k = (r as u64).to_be_bytes().to_vec();
    k.extend_from_slice(&(c as u64).to_be_bytes());
    k
}

/// ONE BODY, WALKED. The painted sites are 4-connected from the first site
/// the grid paints in READING ORDER — the grid's own coordinate order, not
/// a member elected by a measurement (rule 15) — and the reading is whether
/// the walk reaches all of them. Connectivity is symmetric, so every seed
/// inside one body gives one answer.
///
/// An empty grid paints nothing and is not one body: there is no body, and
/// the `paint` column beside this one says which case a `false` is.
fn one_body(g: &Grid) -> bool {
    let mut painted: Keyed<()> = Keyed::new();
    let mut n = 0usize;
    let mut seed: Option<(usize, usize)> = None;
    for (r, row) in g.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            if h != 0 {
                painted.insert(site_key(r, c), ());
                n += 1;
                seed = match seed {
                    None => Some((r, c)),
                    held => held,
                };
            }
        }
    }
    let Some(start) = seed else { return false };
    let mut seen: Keyed<()> = Keyed::new();
    seen.insert(site_key(start.0, start.1), ());
    let mut reached = 1usize;
    let mut stack = vec![start];
    while let Some((r, c)) = stack.pop() {
        let mut neighbours: Vec<(usize, usize)> = vec![(r + 1, c), (r, c + 1)];
        if r > 0 {
            neighbours.push((r - 1, c));
        }
        if c > 0 {
            neighbours.push((r, c - 1));
        }
        for (nr, nc) in neighbours {
            let k = site_key(nr, nc);
            if painted.contains(&k) && !seen.contains(&k) {
                seen.insert(&k, ());
                reached += 1;
                stack.push((nr, nc));
            }
        }
    }
    reached == n
}

fn read(task: &Task) -> Reading {
    let separator = !task.train.is_empty() && task.train.iter().all(|(i, _)| has_separator(i));
    // One ratio for the family or none: the pairs must agree, or the frames
    // do not stand in a ratio at all.
    let ratio = task.train.iter().try_fold(None, |acc, (i, o)| {
        let r = frame_ratio(i, o)?;
        match acc {
            None => Some(Some(r)),
            Some(was) if was == r => Some(Some(r)),
            Some(_) => None,
        }
    });
    Reading {
        separator,
        ratio: ratio.flatten(),
        one_body: !task.train.is_empty() && task.train.iter().all(|(i, _)| one_body(i)),
        paint: task
            .train
            .iter()
            .map(|(i, _)| i.iter().flatten().filter(|&&h| h != 0).count())
            .sum(),
        pairs: task.train.len(),
    }
}

/// The census. With no prefixes every task in the slice is read; with them,
/// only the families the journal named — the prefixes the voice's
/// `family ….` lines carry, mapped here through `task_address`.
///
/// A PREFIX NAMES A STRATUM, NOT A TASK, AND THE CENSUS LISTS THE STRATUM.
/// Measured over the training corpus: 2 bytes separate 120 of 1000 tasks,
/// 4 bytes 299, 32 bytes 908, and only the whole address separates all
/// 1000 — the id's leading bytes are the address tag and the length
/// prefixes every `task_address` shares, so the discrimination is at the back.
/// Law 2 holds (no two tasks share an id) and no prefix is made to stand
/// for one family: the journal prints four bytes because that is three
/// times the reading two gave, and the census answers a prefix with every
/// task under it rather than with a guess at one.
pub(crate) fn report(dir: &str, names: &[String], want: &[String]) {
    // Keyed by the task id, so the page comes back in the trie's order —
    // canonical, content, and no position chosen by a measurement (rule 4).
    let mut rows: Keyed<(String, Reading)> = Keyed::new();
    for file in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{file}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let Some(id) = mycelium::server::registry::task_address(&task) else { continue };
        let hx: String = id.as_bytes().iter().map(|b| format!("{b:02x}")).collect();
        if !want.is_empty() && !want.iter().any(|w| hx.starts_with(w.trim_end_matches('…'))) {
            continue;
        }
        rows.insert(id.as_bytes(), (file.clone(), read(&task)));
    }
    let n = rows.len();
    // HOW MANY BYTES IT TAKES TO NAME ONE TASK, measured rather than
    // assumed. This is why the census answers a prefix with a stratum.
    if want.is_empty() {
        for w in [2usize, 4, 8, 16, 32, 64] {
            let mut d: Keyed<()> = Keyed::new();
            for (k, _) in rows.scoped() {
                d.insert(&k[..w.min(k.len())], ());
            }
            println!("  {w:>2} bytes names {} of {n} families", d.len());
        }
        println!("  the whole id names {n} of {n} (law 2 holds)");
    }
    println!("  census of {n} families over {dir}");
    println!("  {:<10} {:<16} {:>5} {:>7}  sep  ratio      body", "id", "file", "pairs", "paint");
    let (mut sep, mut rat, mut body, mut none) = (0usize, 0usize, 0usize, 0usize);
    for (k, (file, r)) in rows.scoped() {
        sep += usize::from(r.separator);
        rat += usize::from(r.ratio.is_some());
        body += usize::from(r.one_body);
        if !r.separator && r.ratio.is_none() && !r.one_body {
            none += 1;
        }
        let ratio = match r.ratio {
            None => "—".to_string(),
            Some((a, b)) => format!("{a}×{b}"),
        };
        println!(
            "  {:<10} {file:<16} {:>5} {:>7}  {:<4} {ratio:<10} {}",
            k.iter().take(4).map(|b| format!("{b:02x}")).collect::<String>(),
            r.pairs,
            r.paint,
            if r.separator { "yes" } else { "no" },
            // A family that paints nothing has no body to be one or many
            // of, and saying "many" of it would be a reading of the label
            // rather than of the grid.
            match (r.paint, r.one_body) {
                (0, _) => "none",
                (_, true) => "one",
                (_, false) => "many",
            },
        );
    }
    println!("  ── {sep} of {n} cut by a separator, {rat} in integer frame ratio, {body} one body");
    // THE FAMILIES NO READING NAMES are the ones K5's order cannot be taken
    // from, and they are counted rather than left to be inferred from the
    // three totals above (which overlap).
    println!("  ── {none} of {n} answer none of the three");
}
