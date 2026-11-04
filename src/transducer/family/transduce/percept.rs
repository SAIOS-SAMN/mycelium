// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A TASK OF ANY RANK PERCEIVED (MU-SENSING §1, §3, §5; ROADMAP track 26
//! S4b): the family's samples lifted into their enclosures and bodies, the
//! ground deduced, and every pair factored as motion ∘ residue. One path at
//! every rank; nothing in it names an axis, and the roles of the axes are
//! the perspective's (`perspective::Perspective`, the default reading every
//! axis as space).
//!
//! The mind holds the percept as an object it reasons about (MU-SENSING §0):
//! its rank and the roles of its axes (so it knows how many dimensions it
//! read and that the world has more, §5), the ground it read, the bodies of
//! each sample by their boundaries, and each pair's vortex of
//! factorisations with their residues. Spelling a percept in words is the
//! alphabet's, which spells one rank (`alphabet::alphabet_admits`); the
//! percept itself is read at any rank.

use super::{factorisations, Enclosed, Factored, Ground, TransductionRefusal};
use crate::transducer::perspective::Perspective;
use crate::transducer::sample::Sample;

/// ONE PAIR PERCEIVED: its two frames lifted, and its factorisations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairFactored {
    pub input: Enclosed,
    pub output: Enclosed,
    pub factored: Factored,
}

/// A TASK PERCEIVED AT ITS OWN RANK.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskPercept {
    /// The roles of the samples' axes: the default perspective of the
    /// family's rank.
    pub perspective: Perspective,
    /// The ground the family's relations show (MU-SENSING §3.4).
    pub ground: Ground<Vec<u8>>,
    /// Each train pair, lifted and factored.
    pub train: Vec<PairFactored>,
    /// Each test input, lifted.
    pub tests: Vec<Enclosed>,
}

/// WHY A TASK IS NOT PERCEIVED, named.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerceptRefusal {
    /// No sample: nothing presents a rank.
    NoSample,
    /// Two samples of the family of different rank: a section of one rank
    /// is never padded or cut to another (MU-SENSING §5).
    Rank { family: usize, sample: usize },
    /// A pair's factorisation refused, named with its place among the
    /// train pairs.
    Pair { pair: usize, why: TransductionRefusal },
}

impl TaskPercept {
    /// PERCEIVE A FAMILY: its train pairs and test inputs, all of one rank,
    /// each lifted; the ground deduced over the train pairs; each pair
    /// factored under `budget` ([`factorisations`]).
    pub fn of(train: &[(Sample, Sample)], tests: &[Sample], budget: &mut crate::physics::Budget) -> Result<TaskPercept, PerceptRefusal> {
        let mut samples = train.iter().flat_map(|(x, y)| [x, y]).chain(tests);
        let rank = samples.next().ok_or(PerceptRefusal::NoSample)?.rank();
        if let Some(other) = samples.find(|s| s.rank() != rank) {
            return Err(PerceptRefusal::Rank { family: rank, sample: other.rank() });
        }
        let mut pairs = Vec::with_capacity(train.len());
        for (pair, (x, y)) in train.iter().enumerate() {
            let factored = factorisations(x, y, budget).map_err(|why| PerceptRefusal::Pair { pair, why })?;
            pairs.push(PairFactored { input: Enclosed::of(x), output: Enclosed::of(y), factored });
        }
        Ok(TaskPercept {
            perspective: Perspective::of_sample(rank),
            ground: Ground::of(train),
            train: pairs,
            tests: tests.iter().map(Enclosed::of).collect(),
        })
    }

    /// The rank the task was read at: the count of its axes, every one of
    /// them read. A dimension past it is unread (MU-SENSING §5).
    pub fn rank(&self) -> usize {
        self.perspective.rank()
    }
}
