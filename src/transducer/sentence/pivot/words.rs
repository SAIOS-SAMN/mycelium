// 18.2, A21 — THE AXIS INDEX: the words a form speaks on each axis of each
// chart, admitted and retracted as coordinates.

use super::*;
use crate::transducer::sentence::tape::from_element_form;

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
            // WHAT THE VERTEX CARRIES: its letters when it spells, its
            // action when it is an adjoined generator (37.7). This read
            // `word_local`, which answers `None` for a letter, so every
            // reader of this list — the axis index at the voice and at the
            // court (`admit_words`), the language index, the algebraic
            // probe — held a letter and offered it to nobody: no pivot was
            // ever offered an adjoined generator as a candidate, and the
            // probe at a letter had no operator to ask about. A cited
            // vertex still spells none until its citations resolve and
            // contributes nothing here (L14).
            Vertex::Word { derivation, .. } => derivation.word_carried(),
            _ => None,
        })
        .collect()
}

/// The words a held element speaks: a form's Word vertices, or a
/// connection's letters.
pub fn words_of_element(e: &Element) -> Vec<Word> {
    if e.is_sentence() {
        return match from_element_form(e) {
            Some(f) => words_of_form(&f),
            None => Vec::new(),
        };
    }
    match e.letters() {
        Some(l) => vec![l.spelling.word],
        None => Vec::new(),
    }
}

/// ADMIT A FORM'S WORDS UNDER THE CHARTS ITS VERTICES ACT ON (18.2).
///
/// `admit_words` files under ONE chart for the whole element, and
/// `axis_admit` supplied `invariant_of(element)` — for a sentence that is
/// `chart_of_form`, `[2, rank, loop classes]`, a projection of what the FORM
/// HOLDS. `candidates_for` asks under the charts a loop's face vertices act
/// on, which `Vertex::charts` answers and `body_charts_of` already uses for
/// `by_moduli`: `[1, moduli]`, a statement about the BODY A WORD ACTS ON.
/// Both charts are well formed and they answer different questions, so the
/// index was filed under the answer to one and read under the answer to the
/// other. Measured across the fleet: `crystal.on_axis` answered 0 on 123 of
/// 123 reads, and 75 of 76 admits filed on no axis at all because the forms
/// admitted are closed — `[2, 0, 0, 0, 0]`, rank zero, no loops — so their
/// words occupy no coordinate.
///
/// A word is filed where it ACTS. That is the question the pivot asks: not
/// "what does the sentence carrying this word hold" but "what do I hold that
/// acts on this body and turns on this axis". A vertex acting on two charts
/// is admitted at both, because it speaks to bodies on both and one left out
/// is a word the pivot can never be offered.
pub fn admit_words_acting(index: &mut AxisIndex, f: &Form) {
    // WHAT THE ADMISSION ACTUALLY CARRIES (§8.1, measurement only — nothing
    // below branches on these). Filing where a word acts did not move
    // `crystal.on_axis` off zero: measured at a3b78c8, `answered 0` on 113
    // of 113 and `held 0` on 50 of 50. So the chart was not the binding
    // constraint and the question moves upstream of the index — whether the
    // words admitted occupy any coordinate at all. `admit_words` files a
    // word under `ClassVector::of_word(w).coordinates()`, which is empty for
    // an identity rotor, so a genome of identity-carrying vertices files
    // nowhere however it is keyed. These four counts separate "the mind
    // holds no turning word" from "the words are there and something drops
    // them": the vertices seen, the ones carrying a word at all, the ones
    // whose rotor is not the identity, and the coordinates filed.
    let mut seen = 0usize;
    let mut carried = 0usize;
    let mut turning = 0usize;
    let mut filed: Vec<Axis> = Vec::new();
    for v in &f.vertices {
        let Vertex::Word { derivation, acts, .. } = v else {
            continue;
        };
        seen += 1;
        // What the vertex CARRIES: its letters when it spells, its action
        // when it is an adjoined generator (37.7) — the same reader
        // `word_at_vertex` uses, so a generator offered by one is offered by
        // the other.
        let Some(w) = derivation.word_carried() else {
            continue;
        };
        carried += 1;
        if !w.rotor().is_identity() {
            turning += 1;
        }
        if let Some(cv) = ClassVector::of_word(&w) {
            filed.extend(cv.coordinates());
        }
        for (_, a) in acts.scoped() {
            admit_words(index, &a.chart, std::slice::from_ref(&w));
        }
    }
    // WHERE THE TURNING ACTUALLY IS: Curvature resides on FormFace.at[pair].edge,
    // the edge transports, and on turning face words. We thread them into the
    // axis index under the member chart so the crystal ingests the curvature.
    let mut members = 0usize;
    let mut turning_edges = 0usize;
    for face in &f.faces {
        for (k, p) in face.at.scoped() {
            members += 1;
            if let Some(ref e) = p.edge {
                if !e.word.rotor().is_identity() {
                    turning_edges += 1;
                    if let Some(cv) = ClassVector::of_word(&e.word) {
                        filed.extend(cv.coordinates());
                    }
                    if let Some((ci_bytes, _)) = crate::transducer::sentence::read_face_pair_key(&k) {
                        if let Some(ci) = stalk::Invariant::from_key(&ci_bytes) {
                            admit_words(index, &ci, std::slice::from_ref(&e.word));
                        }
                    }
                }
            }
        }
        if !face.word.rotor().is_identity() {
            if let Some(cv) = ClassVector::of_word(&face.word) {
                filed.extend(cv.coordinates());
            }
            if let Some(vi) = f.vertices.get(face.i as usize) {
                for chart in vi.charts() {
                    admit_words(index, &chart, std::slice::from_ref(&face.word));
                }
            }
        }
    }
    eprintln!(
        "  axis admit vertices {seen} carried {carried} turning {turning} \
         members {members} turning_edges {turning_edges} axes {filed:?}"
    );
}

/// The mirror of [`admit_words_acting`], member for member — or one chart's
/// axes are emptied and the word is left named at the others.
pub fn retract_words_acting(index: &mut AxisIndex, f: &Form) {
    for v in &f.vertices {
        let Vertex::Word { derivation, acts, .. } = v else {
            continue;
        };
        let Some(w) = derivation.word_carried() else {
            continue;
        };
        for (_, a) in acts.scoped() {
            retract_words(index, &a.chart, std::slice::from_ref(&w));
        }
    }
    for face in &f.faces {
        for (k, p) in face.at.scoped() {
            if let Some(ref e) = p.edge {
                if !e.word.rotor().is_identity() {
                    if let Some((ci_bytes, _)) = crate::transducer::sentence::read_face_pair_key(&k) {
                        if let Some(ci) = stalk::Invariant::from_key(&ci_bytes) {
                            retract_words(index, &ci, std::slice::from_ref(&e.word));
                        }
                    }
                }
            }
        }
        if !face.word.rotor().is_identity() {
            if let Some(vi) = f.vertices.get(face.i as usize) {
                for chart in vi.charts() {
                    retract_words(index, &chart, std::slice::from_ref(&face.word));
                }
            }
        }
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

/// The fiber of the axis index at (chart, axis), coarsening the CHART until
/// the read is not empty — exactly as `generators_under` coarsens, and for
/// the same reason.
///
/// THE CHART IS A PREFILTER; THE AXIS IS THE COORDINATE. An exact keyed read
/// on the chart made a stratum mismatch indistinguishable from an empty
/// index: one byte apart and the answer is `None`, with nothing to say which
/// it was. A21 §2 has the reading — a demand is reduced against the
/// generators of one STRATUM, never the language, and no further than it
/// must — so the chart names where to look first and coarsens when that
/// level holds nothing, and the root stratum is the same subtree read as
/// every other.
///
/// What the prefilter does NOT do is decide. A word this returns is a
/// CANDIDATE: the trial applies it, re-reads the class vectors off the
/// written faces and refuses unless every member path discharges the axis
/// exactly, so a word offered from a coarser stratum than the loop stands on
/// is confirmed by the plane arithmetic or it is refused there. The index
/// narrows the search; it never certifies a match.
pub fn on_axis_of(index: &AxisIndex, chart: &stalk::Invariant, axis: Axis) -> Vec<Word> {
    let slot = axis_slot(axis);
    let mut at = chart.clone();
    loop {
        let held: Vec<Word> = match index.get(&at.key()).and_then(|mid| mid.get(&slot)) {
            // Two keyed reads reached this bucket -- the chart, then the axis
            // slot -- so what it holds is the words on that axis, all of them.
            Some(bucket) => bucket.scoped().map(|(_, w)| w.clone()).collect(),
            None => Vec::new(),
        };
        if !held.is_empty() || at.0.is_empty() {
            return held;
        }
        at = at.peel();
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
        let Vertex::Word { address, derivation, acts, .. } = v else {
            continue;
        };
        // EVERY MEMBER'S GAUGE. The lanes a generator occupies are read from
        // its class vector, which the gauge enters, so a vertex acting on
        // several charts occupies the lanes of EACH and is admitted to all of
        // them. Reading one member admitted it to one chart's lanes and the
        // cover never named it at the others.
        for (_, a) in acts.scoped() {
            let Some(cv) = class_of_vertex(derivation, &a.gauge) else {
                continue;
            };
            for (lane, chart) in lane_charts_of(&cv, &derivation.rotor()) {
                let Some(cover) = index.get_mut(lane) else { continue };
                cover.nested_insert(chart.key(), address, form.clone());
            }
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
        let Vertex::Word { address, derivation, acts, .. } = v else {
            continue;
        };
        // The retract mirrors the admit member for member, or it empties one
        // chart's lanes and leaves the generator named at the others.
        for (_, a) in acts.scoped() {
            let Some(cv) = class_of_vertex(derivation, &a.gauge) else {
                continue;
            };
            for (lane, chart) in lane_charts_of(&cv, &derivation.rotor()) {
                let Some(cover) = index.get_mut(lane) else { continue };
                if let Some(bucket) = cover.get_mut(chart.key().as_bytes()) {
                    bucket.remove(address);
                }
            }
        }
    }
}

/// The class vector of a vertex's derivation, read from its ROTOR so an
/// adjoined generator the seventeen do not spell still carries a class
/// (L11e's `of_rotor`), and from its GAUGE, so a generator that recolours
/// is filed on `𝒢` as well as on `𝒽` and a target that recolours can
/// reach it (L12).
///
/// The carriage is the identity: a vertex's own placement rides on
/// `Vertex::Word.carriage`, 18.2 quotients the body's absolute position,
/// and a derivation claims a rotor and never a placement. This is the same
/// reading `family::derivation::occupied_lanes` asks the index with — one
/// class vector on both sides, or a generator is filed under one key and
/// looked up under another.
fn class_of_vertex(
    d: &crate::transducer::alphabet::Derivation,
    gauge: &[(u8, u8)],
) -> Option<crate::transducer::sentence::ClassVector> {
    crate::transducer::sentence::ClassVector::of_rotor(
        &d.rotor(),
        crate::transducer::element::identity_versor(),
        crate::transducer::sentence::CycleType::of(gauge),
    )
}

/// EVERY LANE A CLASS OCCUPIES, AND THE CHART IT OCCUPIES EACH ON (42.6
/// as amended, L12). Lane 3 is `versor_chart_of` — the rotor's conjugacy
/// axes ARE the holonomic lane's chart, so nothing filed under it moves —
/// and the other three are `lane_chart` over that lane's own axes. A lane
/// the class does not occupy is absent from the answer: a member of no
/// basis, never a member of an empty one.
///
/// ONE READER. Admission, retraction and the meet read each had their own
/// copy of the `match lane { 3 => …, _ => … }`, which is three chances for
/// a generator to be filed under one key and looked up under another.
pub fn lane_charts_of(
    cv: &crate::transducer::sentence::ClassVector,
    rotor: &crate::transducer::alphabet::Rotor,
) -> Vec<(usize, stalk::Invariant)> {
    (0..4)
        .filter_map(|lane| {
            let chart = match lane {
                3 => versor_chart_of(rotor),
                _ => lane_chart(cv, lane),
            };
            chart.map(|c| (lane, c))
        })
        .collect()
}

/// THE ADDRESS OF ONE GENERATOR inside a presentation: the form that
/// carries it and the vertex inside that form, in that order, each
/// length-prefixed. Content, so the same generator reached through two
/// lanes' covers is recognised as ONE member of the meet.
pub fn generator_key(
    form: &crate::transducer::address::Address,
    vertex: &crate::transducer::address::Address,
) -> Vec<u8> {
    let (f, v) = (form.as_bytes(), vertex.as_bytes());
    let mut k = Vec::with_capacity(f.len() + v.len() + 8);
    k.extend_from_slice(&(f.len() as u32).to_le_bytes());
    k.extend_from_slice(f);
    k.extend_from_slice(&(v.len() as u32).to_le_bytes());
    k.extend_from_slice(v);
    k
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
    // The outer read IS a prefix descent -- one chart's buckets -- and each
    // bucket it yields is one axis slot's words.
    // A chart with no axes is the root stratum and names no coordinate to
    // descend on: the pairs on it are read through `serve_classes`, not
    // here, so this answers nothing rather than everything (rule 11).
    let key = chart.key();
    let Some(at) = crate::network::nerve::Prefix::new(key.as_bytes()) else {
        return out;
    };
    for (_, bucket) in index.fiber(at) {
        for (v, f) in bucket.scoped() {
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
