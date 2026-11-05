use super::perceive::SectionSample;

/// ONE TRAIN PAIR — the scene a rule is shown and the scene it must speak
/// (38). Each is a sample on the alphabet's section (track 26 S4d); the
/// pair is what a rule is extracted from and reconstructed against.
pub type TrainPair = (SectionSample, SectionSample);

/// THE FAMILY a rule is extracted from: every train pair of one task. A
/// rule speaks the family or it is not a rule (21).
pub type Family = [TrainPair];

/// ONE TEST PAIR: the input a solve answers and the output the court
/// withholds, absent where the source holds none.
pub type TestPair = (SectionSample, Option<SectionSample>);

/// A TASK AS A FAMILY OF SAMPLES ON THE ALPHABET'S SECTION (MU-SENSING §6):
/// the train family (input → output demonstrations) and the test pairs,
/// whose outputs are withheld by the court and may be absent where the
/// source holds none. A source's decoder writes it (outside `src/`,
/// MU-SENSING §7); the core reads no file format into it. Moved from the
/// former `arc` module (ROADMAP track 26, S1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub train: Vec<TrainPair>,
    pub test: Vec<TestPair>,
}

/// A FIXTURE'S TASK from its line pairs, each read at intake; a test pair's
/// output with no line is absent.
#[cfg(test)]
pub(crate) fn task_of_lines(train: Vec<crate::transducer::spelled::LinePair>, test: Vec<crate::transducer::spelled::LinePair>) -> Task {
    Task {
        train: crate::transducer::spelled::pairs_of(train),
        test: test.into_iter().map(|(x, y)| (SectionSample::from(x), (!y.is_empty()).then(|| SectionSample::from(y)))).collect(),
    }
}
