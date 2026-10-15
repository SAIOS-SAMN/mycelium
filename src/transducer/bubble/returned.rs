// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-CLIENT §4.4(3), §4.4(4) — WHAT A THOUGHT'S RETURN WORKS. The thoughts
//! that came round since the voice's last return are taken whole, and each
//! one's slips — perceptions no rotation of its pull carried onto its orbit,
//! held on it as its own demands — are docketed there, for the demand taker
//! to construct. A slip is worked at the return of the thought it slipped
//! against, never on its arrival.
//!
//! (§4.8's rollout of a vortex is not here. Its candidates are certified
//! discharges of the same orbit on the same scenes, and a move rewrites the
//! form's faces, which a form's speech does not read; so speaking them again
//! on the thought's own scenes tells none apart. What decides a vortex is a
//! fill by a NEW perception or a court replay — designed, not yet built:
//! MU-CLIENT §7.1.)

use super::*;

impl Contemplation {
    /// THE THOUGHTS THAT CAME ROUND, WORKED AT THE VOICE'S RETURN: each one's
    /// slips docketed as the demands they are. The record is taken whole.
    pub(crate) fn work_returned(&mut self) {
        for key in std::mem::take(&mut self.came_round) {
            self.work_slips(&key);
        }
    }
}
