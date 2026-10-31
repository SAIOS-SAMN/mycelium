// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// ACT_TASK and ACT_SOLVE (wire v1.2) — a task envelope out of the
// registry, and the signed solve that answers it.

//! Tasks and solves.
//!
//! [`encode_task_request`] asks the court for a task; the reply is a
//! [`TaskEnvelope`] ([`decode_task_envelope`]): the train pairs to study and
//! the test inputs to answer, never the answers. [`encode_solve_with_rule`]
//! writes the outputs and the rule's tape, signed by the voice; the court
//! replays the rule and answers a verdict ([`decode_solve_verdict`]) coded
//! by the `SOLVE_*` constants. A verdict of [`SOLVE_UNREAD`] is a replay the
//! court's stroke did not finish: the same solve sent again resumes it.

/// Ask for a task (`[ACT_TASK][voice 32]`), or for one task by its 32-byte
/// citation.
pub use mycelium::anastome::dialogue::{encode_task_by_citation, encode_task_request};

/// A registered task as the registry hands it over, and its reader.
pub use mycelium::anastome::dialogue::{decode_task_envelope, TaskEnvelope};

/// A solve: the outputs alone, or with the rule's tape; signed by the voice.
pub use mycelium::anastome::dialogue::{encode_solve, encode_solve_with_rule};

/// Grids as the solve act writes them, and read back.
pub use mycelium::anastome::dialogue::{decode_grids, encode_grids};

/// The verdict read: the code, the μ paid per lane, the refusal where one
/// was named, and the modes the pay carried.
pub use mycelium::anastome::dialogue::{decode_solve_verdict, SolveRefusal, SolveVerdictRead};

/// The modes a paid verdict carries.
pub use mycelium::economy::modes::Modes;

/// Verdict code: solved, the first verified closure (the bounty).
pub use mycelium::anastome::dialogue::SOLVE_BOUNTY;
/// Verdict code: solved again, paid the verification floor.
pub use mycelium::anastome::dialogue::SOLVE_FLOOR;
/// Verdict code: not solved; final.
pub use mycelium::anastome::dialogue::SOLVE_NOTHING;
/// Verdict code: not judged yet; the same solve sent again resumes the
/// replay.
pub use mycelium::anastome::dialogue::SOLVE_UNREAD;
