// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// ACT_TASK and ACT_SOLVE (wire v1.2) — a task envelope out of the
// registry, and the signed solve that answers it.

//! Tasks and solves.
//!
//! [`encode_task_request`] asks the court for a task; the reply is a
//! [`TaskEnvelope`] ([`decode_task_envelope`]): the train pairs to study and
//! the test inputs to answer, never the answers, each a [`Sample`] of any
//! rank at its source's symbol width (track 26 S5). A client perceives the
//! samples at any rank; one whose rule seam spells words takes
//! [`TaskEnvelope::section`] ([`TaskSection`]), the alphabet's section,
//! refused by name ([`SectionRefusal`]) for a sample of another rank or
//! width (track 26 S4b).
//! [`encode_solve_with_rule`] writes the outputs, as samples
//! ([`spoken_outputs`] places rasters), and the rule's tape, signed by the
//! voice; the court replays the rule and answers a verdict
//! ([`decode_solve_verdict`]) coded by the `SOLVE_*` constants. A verdict of
//! [`SOLVE_UNREAD`] is a replay the court's stroke did not finish: the same
//! solve sent again resumes it.

/// Ask for a task (`[ACT_TASK][voice 32]`), or for one task by its 32-byte
/// citation.
pub use mycelium::anastome::dialogue::{encode_task_by_citation, encode_task_request};

/// A registered task as the registry hands it over, its writer and its
/// reader, and the envelope read on the alphabet's section.
pub use mycelium::anastome::dialogue::{decode_task_envelope, encode_task_envelope, TaskEnvelope, TaskSection};

/// Why a sample is off the alphabet's section: its rank, or its width.
pub use mycelium::transducer::sentence::SectionRefusal;

/// A solve: the outputs alone, or with the rule's tape, and optionally the
/// ground its claim was spoken under (`encode_solve_spoken`); signed by the
/// voice.
pub use mycelium::anastome::dialogue::{encode_solve, encode_solve_spoken, encode_solve_with_rule};

/// THE SAMPLE (MU-SENSING §2): rank, extents, symbols at the source's width
/// and a place; how a sample is written on the wire and read back, refused
/// by name ([`SampleWire`]); the outputs as the solve act writes them; and
/// rasters placed as spoken outputs.
pub use mycelium::anastome::dialogue::{
    decode_samples, encode_samples, push_sample, read_sample, spoken_outputs, SampleWire,
};
pub use mycelium::anastome::dialogue::samples::{position, Member, Split};
pub use mycelium::transducer::sample::{Place, Sample, SampleRefusal};

/// A task as a source posts it, as samples, written and read back.
pub use mycelium::anastome::dialogue::{decode_posted_samples, encode_posted_samples, PostedTask};

/// The verdict read: the code, the μ paid per lane, the refusal where one
/// was named, and the modes the pay carried.
pub use mycelium::anastome::dialogue::{decode_solve_verdict, SolveRefusal, SolveVerdictRead};

/// The verdict's receipt alone: the code and the μ paid per lane.
pub use mycelium::anastome::dialogue::decode_solve_receipt;

/// A CLAIM OF FACTORISATION (MU-SENSING §3.9; track 26 S4c): a train pair's
/// motion and residue at any rank, with the ground it was spoken under,
/// signed by the voice; the court reconstructs the motion, derives the
/// residue and answers a [`FactorVerdict`]. The pair's factorisations are
/// read by [`factorisations`]; each holds the [`Motion`] and the residue as
/// [`Link`]s.
pub use mycelium::anastome::dialogue::{decode_factor_verdict, decode_factorisation, encode_factorisation, FactorVerdict, FactorisationAct};
pub use mycelium::transducer::family::transduce::{factorisations, Enclosure, Factored, Factorisation, Motion, Sense, SignedPermutation};
pub use mycelium::transducer::sentence::Link;

/// The modes a paid verdict carries.
pub use mycelium::economy::modes::Modes;

/// Verdict code: solved, the first verified closure (the bounty).
pub use mycelium::anastome::dialogue::SOLVE_BOUNTY;
/// Verdict code: solved again — a repeat closure of a closed task, verified
/// and paid nothing.
pub use mycelium::anastome::dialogue::SOLVE_AGAIN;
/// Verdict code: not solved; final.
pub use mycelium::anastome::dialogue::SOLVE_NOTHING;
/// Verdict code: not judged yet; the same solve sent again resumes the
/// replay.
pub use mycelium::anastome::dialogue::SOLVE_UNREAD;
