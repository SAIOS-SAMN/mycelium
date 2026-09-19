// K1, §8.1 — INSTRUMENTATION ONLY: what the scene cover deduces on a
// corpus, per task and in total.
//
//! 53.2 says Bands covers 850 of the 1 120 tasks. That is the parent
//! tree's number and this is where it is READ on this one. Nothing here
//! decides anything and no library function is changed: `scene::cover` and
//! `scene::is_identity` are called exactly as extraction calls them.

use mycelium::transducer::family::{cover, is_identity, restrict, scene_chart, Grid, Reading};

/// Per task: the charts Bands deduces on the train inputs, and the largest
/// chart's extent beside the scene's own.
pub(crate) fn report(dir: &str, names: &[String]) {
    let (mut tasks, mut banded, mut identity, mut none) = (0u64, 0u64, 0u64, 0u64);
    let mut charts_total = 0u64;
    for file in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{file}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        tasks += 1;
        let scene: Vec<Grid> = task.train.iter().map(|(a, _)| a.clone()).collect();
        let Some(cov) = cover(&scene, Reading::Bands) else {
            none += 1;
            println!("  cover {file} none");
            continue;
        };
        let n = cov.scoped().count() as u64;
        charts_total += n;
        if is_identity(&cov, &scene, Reading::Bands) {
            identity += 1;
            continue;
        }
        banded += 1;
        // The largest chart's extent, read off the restriction so it is
        // the extent extraction would actually see.
        let (mut mh, mut mw) = (0usize, 0usize);
        for (_, chart) in cov.scoped() {
            let [first_in, ..] = scene.as_slice() else { continue };
            if let Some(g) = restrict(first_in, chart) {
                // A grid's rows are all its width, so the first row's
                // length IS the grid's width — content, not a member
                // picked by arrival (rule 15). A slice pattern says so.
                let [row, ..] = g.as_slice() else { continue };
                let (h, w) = (g.len(), row.len());
                if h * w > mh * mw {
                    mh = h;
                    mw = w;
                }
            }
        }
        let whole = match (scene_chart(&scene), scene.as_slice()) {
            (Some(_), [g, ..]) => match g.as_slice() {
                [row, ..] => (g.len(), row.len()),
                [] => (0, 0),
            },
            _ => (0, 0),
        };
        println!(
            "  cover {file} charts {n} largest {mh}x{mw} scene {}x{}",
            whole.0, whole.1
        );
    }
    println!(
        "  cover tasks {tasks} banded {banded} identity {identity} none {none} charts {charts_total}"
    );
}
