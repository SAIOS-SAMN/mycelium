// 18.2, A21 — THE AXIS INDEX: the words a form speaks on each axis of each
// chart, admitted and retracted as coordinates.

use super::*;

/// The axis index of a crystal (12.6, A16 step 3): chart → axis slot → M1
/// word. Nested keyed reads, never a hash of the pair.
pub type AxisIndex = Keyed<Keyed<Keyed<Word>>>;

pub(crate) fn axis_slot(axis: Axis) -> Vec<u8> {
    vec![axis.to_byte()]
}

/// The words a form speaks: its Word vertices'.
pub fn words_of_form(f: &Form) -> Vec<Word> {
    f.vertices
        .iter()
        .filter_map(|v| match v {
            // The residue: the letters this vertex spells on its own.
            // A cited vertex spells none until its citations resolve, so
            // it contributes nothing here (L14).
            Vertex::Word { derivation, .. } => derivation.word_local().cloned(),
            _ => None,
        })
        .collect()
}

/// The words a held element speaks: a form's Word vertices, or a
/// connection's letters.
pub fn words_of_element(e: &Element) -> Vec<Word> {
    if e.is_sentence() {
        return match e.form() {
            Some(f) => words_of_form(f),
            None => Vec::new(),
        };
    }
    match e.letters() {
        Some(l) => vec![l.spelling.word],
        None => Vec::new(),
    }
}

/// Admit words to the axis index under every coordinate their class vector
/// occupies on `chart`.
pub fn admit_words(index: &mut AxisIndex, chart: &stalk::Invariant, words: &[Word]) {
    let ck = chart.key();
    for w in words {
        let Some(cv) = ClassVector::of_word(w) else {
            continue;
        };
        for axis in cv.coordinates() {
            let ak = axis_slot(axis);
            if index.get(ck.as_bytes()).is_none() {
                index.insert(ck.clone(), Keyed::new());
            }
            if let Some(mid) = index.get_mut(ck.as_bytes()) {
                mid.nested_insert(ak, w.key(), w.clone());
            }
        }
    }
}

/// Retract words from the axis index (shed, retire).
pub fn retract_words(index: &mut AxisIndex, chart: &stalk::Invariant, words: &[Word]) {
    let ck = chart.key();
    let Some(mid) = index.get_mut(&ck) else {
        return;
    };
    for w in words {
        let Some(cv) = ClassVector::of_word(w) else {
            continue;
        };
        for axis in cv.coordinates() {
            if let Some(bucket) = mid.get_mut(&axis_slot(axis)) {
                bucket.remove(&w.key());
            }
        }
    }
}

/// The fiber of the axis index at (chart, axis).
pub fn on_axis_of(index: &AxisIndex, chart: &stalk::Invariant, axis: Axis) -> Vec<Word> {
    match index
        .get(&chart.key())
        .and_then(|mid| mid.get(&axis_slot(axis)))
    {
        Some(bucket) => bucket
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, w)| w.clone())
            .collect(),
        None => Vec::new(),
    }
}

/// 1.8, 50.2 step 2 — THE GENERATOR INDEX: the word vertices a mind or a
/// court holds, by the chart of the versor each carries. Versor chart →
/// (vertex address → the address of the form the vertex lives in).
///
/// The generators of a presentation are the ruled words (1.8), and since
/// L5 a ruled word is a FORM: `Element::letters` answers `None` for every
/// sentence, so an index that reads a word off `letters` sees only the v1
/// connection words the utterance path stopped producing. This index reads
/// the word vertices themselves, which is where the generators are.
pub type GeneratorIndex = Keyed<Keyed<crate::transducer::address::Address>>;

/// The chart of a word's own versor: `connection_chart`'s shape — the
/// leading 0 that keeps versor charts disjoint from residual charts (M4
/// step 2), then the rotor's conjugacy axes (15.3). A word with no
/// conjugacy axes is not a generator and is not filed.
pub fn versor_chart(w: &Word) -> Option<stalk::Invariant> {
    versor_chart_of(&w.rotor())
}

/// The chart from the ACTION alone (L14): the coordinate the generator
/// index is keyed by, read off a vertex's canonical rotor with no
/// citation resolved.
///
/// This is the `𝒽` LANE'S CHART (42.6's table): the rotor's conjugacy
/// axes are exactly the holonomic lane's, so the chart that was the only
/// chart is that lane's own and does not move.
pub fn versor_chart_of(
    r: &crate::transducer::alphabet::Rotor,
) -> Option<stalk::Invariant> {
    let mut axes = vec![0u64];
    axes.extend(r.class_axes()?);
    Some(stalk::Invariant(axes))
}

/// THE CHART OF A CLASS ON ONE LANE (42.6 as amended, L12): the leading
/// lane tag, then that lane's OCCUPIED AXES in 12.6's order, each written
/// out as its own byte.
///
/// The axes are the key, not a count of them and not a flag saying the
/// lane is occupied — the key says WHICH coordinates the class carries on
/// that lane, so `peel` coarsens by dropping one coordinate and a prefix
/// read is the stratum, exactly as the rotor chart's is (rule 2, rule 6).
///
/// The lane tag leads because the four covers are independent (law 5) and
/// their keys must not collide: a class occupying placement on `𝒮` and a
/// class occupying hue on `𝒢` are two members of two covers, never one.
/// `𝒽` keeps `versor_chart_of`'s leading 0 and its conjugacy axes
/// unchanged, so nothing filed under it moves.
///
/// `None` where the class occupies nothing on the lane: a member of no
/// basis, rather than a member of an empty one.
pub fn lane_chart(
    cv: &crate::transducer::sentence::ClassVector,
    lane: usize,
) -> Option<stalk::Invariant> {
    let occupied: Vec<u64> = cv
        .coordinates()
        .into_iter()
        .filter(|a| a.lane() == lane)
        .map(|a| a.to_byte() as u64)
        .collect();
    if occupied.is_empty() {
        return None;
    }
    let mut axes = vec![LANE_TAG + lane as u64];
    axes.extend(occupied);
    Some(stalk::Invariant(axes))
}

/// The leading value that keeps a lane chart disjoint from a versor chart
/// (leading 0) and from a residual chart. Content, and the one place the
/// four covers' key spaces are separated.
pub const LANE_TAG: u64 = 1 << 32;

/// The generator index of each lane (42.6 as amended): four independent
/// covers, never one index with a lane written into its values.
pub type LaneIndex = [GeneratorIndex; 4];

/// File every word vertex of a form under the chart of the versor it
/// carries. A form with no word vertices files nothing.
pub fn admit_generators(
    index: &mut GeneratorIndex,
    form: &crate::transducer::address::Address,
    f: &Form,
) {
    for v in &f.vertices {
        let Vertex::Word { address, derivation, .. } = v else {
            continue;
        };
        let Some(chart) = versor_chart_of(&derivation.rotor()) else {
            continue;
        };
        index.nested_insert(chart.key(), address, form.clone());
    }
}

/// THE SAME ADMISSION, ONCE PER LANE THE VERTEX OCCUPIES (42.6, L12).
///
/// A generator that turns AND recolours is a generator on `𝒽` and a
/// generator on `𝒢`, filed in both covers under each lane's own axes.
/// That is not a double count: the covers are independent, and a
/// reduction on `𝒢` is entitled to it exactly as a reduction on `𝒽` is.
/// What the split refuses is the other direction — a target occupying
/// `𝒢` reduced against a generator that occupies only `𝒽`, which is a
/// basis no lane has.
pub fn admit_generators_by_lane(
    index: &mut LaneIndex,
    form: &crate::transducer::address::Address,
    f: &Form,
) {
    for v in &f.vertices {
        let Vertex::Word { address, derivation, .. } = v else {
            continue;
        };
        let Some(cv) = class_of_vertex(derivation) else {
            continue;
        };
        for (lane, cover) in index.iter_mut().enumerate() {
            let chart = match lane {
                3 => versor_chart_of(&derivation.rotor()),
                _ => lane_chart(&cv, lane),
            };
            let Some(chart) = chart else { continue };
            cover.nested_insert(chart.key(), address, form.clone());
        }
    }
}

/// Empty what `admit_generators_by_lane` filled.
pub fn retract_generators_by_lane(
    index: &mut LaneIndex,
    _form: &crate::transducer::address::Address,
    f: &Form,
) {
    for v in &f.vertices {
        let Vertex::Word { address, derivation, .. } = v else {
            continue;
        };
        let Some(cv) = class_of_vertex(derivation) else {
            continue;
        };
        for (lane, cover) in index.iter_mut().enumerate() {
            let chart = match lane {
                3 => versor_chart_of(&derivation.rotor()),
                _ => lane_chart(&cv, lane),
            };
            let Some(chart) = chart else { continue };
            if let Some(bucket) = cover.get_mut(chart.key().as_bytes()) {
                bucket.remove(address);
            }
        }
    }
}

/// The class vector of a vertex's derivation, read from its ROTOR so an
/// adjoined generator the seventeen do not spell still carries a class
/// (L11e's `of_rotor`). The carriage is the identity here: a vertex's own
/// placement rides on `Vertex::Word.carriage` and is not part of the
/// generator's class.
fn class_of_vertex(
    d: &crate::transducer::alphabet::Derivation,
) -> Option<crate::transducer::sentence::ClassVector> {
    crate::transducer::sentence::ClassVector::of_rotor(
        &d.rotor(),
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::Identity,
    )
}

/// THE BASIS ON ONE LANE (42.6 as amended): the fiber of that lane's
/// index under that lane's axes alone, coarsened by `peel` exactly as
/// `generators_under` coarsens.
pub fn generators_under_lane(
    index: &LaneIndex,
    lane: usize,
    chart: &stalk::Invariant,
    budget: &mut crate::physics::Budget,
) -> Vec<(crate::transducer::address::Address, crate::transducer::address::Address)> {
    match index.get(lane) {
        Some(ix) => generators_under(ix, chart, budget),
        None => Vec::new(),
    }
}

/// Empty what `admit_generators` filled.
pub fn retract_generators(
    index: &mut GeneratorIndex,
    _form: &crate::transducer::address::Address,
    f: &Form,
) {
    for v in &f.vertices {
        let Vertex::Word { address, derivation, .. } = v else {
            continue;
        };
        let Some(chart) = versor_chart_of(&derivation.rotor()) else {
            continue;
        };
        if let Some(bucket) = index.get_mut(chart.key().as_bytes()) {
            bucket.remove(address);
        }
    }
}

/// THE GENERATORS UNDER A CHART PREFIX: (form address, vertex address).
/// A chart key is `TAG_CHART ‖ axes`, so a peeled chart's key is a byte
/// prefix of the finer one's and `Keyed::fiber` reads the whole subtree —
/// the same read `Stalks::fiber_under` performs. Read with `get` on the
/// exact key instead, every level answered only its own node and the
/// coarsest level had to be special-cased into a scan of everything held
/// (law 3).
pub fn generators_at(
    index: &GeneratorIndex,
    chart: &stalk::Invariant,
    budget: &mut crate::physics::Budget,
) -> Vec<(crate::transducer::address::Address, crate::transducer::address::Address)> {
    let mut out = Vec::new();
    for (_, bucket) in index.fiber(chart.key().as_bytes()) {
        for (v, f) in bucket.fiber(crate::network::nerve::ROOT) {
            if !budget.spend() {
                return out;
            }
            out.push((f.clone(), crate::transducer::address::Address::from(v)));
        }
    }
    out
}

/// THE STRATUM'S GENERATORS (42.6, A21 §2): the chart coarsened one axis at
/// a time until the read is not empty, exactly as `WorkingAtlas::stratum`
/// coarsens — a demand is reduced against the generators of one stratum,
/// never the language, and no further than it must. The empty chart's key
/// is `TAG_CHART` alone, so the coarsest level is the same subtree read as
/// every other and needs no arm of its own.
pub fn generators_under(
    index: &GeneratorIndex,
    chart: &stalk::Invariant,
    budget: &mut crate::physics::Budget,
) -> Vec<(crate::transducer::address::Address, crate::transducer::address::Address)> {
    let mut at = chart.clone();
    loop {
        let held = generators_at(index, &at, budget);
        if !held.is_empty() || at.0.is_empty() {
            return held;
        }
        at = at.peel();
    }
}

/// The word a form's vertex carries, by the address that names it. A keyed
/// read of the form's own index, never a walk (law 3).
pub fn word_at_vertex(f: &Form, vertex: &[u8]) -> Option<Word> {
    let i = *f.by_address.get(vertex)?;
    match f.vertices.get(i as usize)? {
        // What the vertex CARRIES: its letters when it spells, its action
        // when it is an adjoined generator (37.7). `word_local` alone
        // answered `None` for a letter, so a mind holding one offered it
        // as a generator to nobody.
        Vertex::Word { derivation, .. } => derivation.word_carried(),
        _ => None,
    }
}

/// The ACTION at a form's vertex, by the address that names it (L14).
/// What a court composes when it replays a citation: rotors compose by
/// the geometric product whether or not the alphabet spells them, so a
/// citation of an adjoined generator verifies exactly as one of a
/// spelled generator does.
pub fn rotor_at_vertex(f: &Form, vertex: &[u8]) -> Option<crate::transducer::alphabet::Rotor> {
    let i = *f.by_address.get(vertex)?;
    match f.vertices.get(i as usize)? {
        Vertex::Word { derivation, .. } => Some(derivation.rotor()),
        _ => None,
    }
}
