use super::perceive;

/// ONE TRAIN PAIR — the scene a rule is shown and the scene it must speak
/// (38). The grids are the corpus's own carrier; the pair is what a rule is
/// extracted from and replayed against.
pub type TrainPair = (perceive::Grid, perceive::Grid);

/// THE FAMILY a rule is extracted from: every train pair of one task. A
/// rule speaks the family or it is not a rule (21).
pub type Family = [TrainPair];

/// A TASK AS A FAMILY OF SAMPLES' RASTERS (MU-SENSING §6): the train family
/// (input → output demonstrations) and the test pairs, whose outputs are
/// withheld by the court and may be absent (empty) where the source holds
/// none. A source's decoder writes it (outside `src/`, MU-SENSING §7); the
/// core reads no file format into it. Moved from the former `arc` module
/// (ROADMAP track 26, S1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub train: Vec<TrainPair>,
    pub test: Vec<TrainPair>,
}
