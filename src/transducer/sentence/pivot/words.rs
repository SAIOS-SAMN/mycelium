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
            Vertex::Word { word, .. } => Some(word.clone()),
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
