// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! STD1 (rule 11) — THE ONE CHECKED STEP OF A FOUR-LANE MEASURE.
//!
//! A sum of measures that would pass its carrier is REFUSED: the whole
//! vector lands or none of it does, and the caller refuses the reading it was
//! building or counts the refusal where it has no caller to answer. Never
//! pinned at a ceiling — a saturated lane is an invented value, and a vector
//! with one lane pinned is a reading no measure produced.

/// `a + b` per lane, all lanes or none: `None` where any lane would pass its
/// carrier.
pub fn lanes_add(a: [u64; 4], b: [u64; 4]) -> Option<[u64; 4]> {
    Some([a[0].checked_add(b[0])?, a[1].checked_add(b[1])?, a[2].checked_add(b[2])?, a[3].checked_add(b[3])?])
}

/// `acc += b`, all lanes or none. Returns whether it stepped: a refused step
/// leaves `acc` exactly as it was.
pub fn lanes_step(acc: &mut [u64; 4], b: &[u64; 4]) -> bool {
    match lanes_add(*acc, *b) {
        Some(v) => {
            *acc = v;
            true
        }
        None => false,
    }
}

/// The lane-wise sum of measures, or `None` where it would pass the carrier.
pub fn lanes_sum(it: impl IntoIterator<Item = [u64; 4]>) -> Option<[u64; 4]> {
    it.into_iter().try_fold([0u64; 4], lanes_add)
}
