// 53.1, 53.2 — THE SCENE COVER: what a reading deduces, and where the
// recursion ends.

use super::common::*;
use crate::transducer::family::*;

/// 53.1's IDENTITY END, as a key comparison and not a count. A scene with
/// no uniform separator is its own single chart, and that is exactly when
/// the cover's key is the scene's own key — so the recursion stops there
/// with nothing counted, no floor, and no constant (rules 2 and 5).
#[test]
fn is_identity_ends_cut_at_scene_key() {
    // No row and no column of this scene is one hue throughout, so Bands
    // deduces no separator and the cover is the whole.
    let solid = vec![grid(&["120", "201", "012"])];
    let whole_cover = cover(&solid, Reading::Bands).expect("Bands deduces a cover");
    assert!(
        is_identity(&whole_cover, &solid, Reading::Bands),
        "a scene with no separator is its own single chart: {:?}",
        whole_cover.scoped().map(|(k, _)| k.len()).collect::<Vec<_>>()
    );
    let whole = scene_chart(&solid).expect("the scene names a chart");
    assert!(
        whole_cover.get(&whole.key(Reading::Bands)).is_some(),
        "and the chart the cover holds IS the scene's own"
    );

    // A scene a uniform row and a uniform column cut into four bands is
    // NOT at the identity end: the cover's key is four chart keys written
    // out and the scene's is one.
    let banded = vec![grid(&["1201", "0000", "2102", "1021"])];
    let cut = cover(&banded, Reading::Bands).expect("Bands deduces a cover");
    assert!(
        !is_identity(&cut, &banded, Reading::Bands),
        "a separated scene is not its own chart"
    );
    assert_ne!(
        cover_key(&cut),
        cover_key(&whole_cover),
        "and the two covers are two coordinates"
    );
}

/// 53.2 BANDS: the rows and columns uniform across EVERY input bound the
/// charts, and each chart restricts to a scene of its own extent.
///
/// The three readings here are the whole of what the cut claims. The
/// separator's own row and column are in NO chart — nothing of the scene's
/// content crosses there, which is what the reading deduced. Every chart's
/// sites are the scene's sites, so the restriction reads a grid of the
/// chart's own extent at the chart's origin. And a row uniform in one
/// input but not the other is not a separator: the cut is a fact of the
/// FAMILY, not of one grid.
#[test]
fn cover_bands_scene_into_restricted_charts() {
    let banded = vec![grid(&["12", "00", "21"])];
    let banded_cover = cover(&banded, Reading::Bands).expect("Bands deduces a cover");
    let keys: Vec<Vec<u8>> = banded_cover.scoped().map(|(k, _)| k).collect();
    assert_eq!(keys.len(), 2, "one uniform row cuts this scene into two bands");
    let mut sites: Vec<crate::transducer::base::Site> = Vec::new();
    for (_, chart) in banded_cover.scoped() {
        sites.extend_from_slice(&chart.support);
    }
    sites.sort_unstable();
    assert_eq!(
        sites,
        [(0i64, 0i64, 0i64), (0, 1, 0), (2, 0, 0), (2, 1, 0)].map(crate::transducer::base::Site::from).to_vec(),
        "the separator's own row is in no chart: nothing of the scene crosses it"
    );
    // The restriction is the chart's own extent, read at its origin.
    let mut read: Vec<Grid> = banded_cover
        .scoped()
        .filter_map(|(_, chart)| restrict(&banded[0], chart))
        .collect();
    read.sort();
    assert_eq!(read, vec![grid(&["12"]), grid(&["21"])], "each chart is a scene of its own");

    // THE CUT IS A FACT OF THE FAMILY. Row 1 is uniform in the first input
    // and not in the second, so it is no separator and the scene is its
    // own single chart.
    let disagreeing = vec![grid(&["12", "00", "21"]), grid(&["12", "01", "21"])];
    let one = cover(&disagreeing, Reading::Bands).expect("Bands deduces a cover");
    assert!(
        is_identity(&one, &disagreeing, Reading::Bands),
        "a row uniform in one input only is not a separator of the family"
    );
}
