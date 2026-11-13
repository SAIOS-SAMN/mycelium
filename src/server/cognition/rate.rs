// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.183 §7.5 — THE COURT'S METABOLIC RATE: a bell span closed each time the
// court's own line stands at a new height, read per lane off its physics.

use super::Court;
use crate::physics::rate::LaneRate;

impl Court {
    /// THE SPAN CLOSED where the court's bell (its line's head height) moved
    /// since the last close: per lane the entropic and syntropic events it
    /// booked, its holding and the draw its open share sizes. `None` while
    /// the bell stands.
    pub fn close_rate(&mut self) -> Option<[LaneRate; 4]> {
        let h = self.chain.head_height();
        self.rate.close(&self.physics, h)
    }

    /// The last span's rates; `None` before the first.
    pub fn rate(&self) -> Option<[LaneRate; 4]> {
        self.rate.last()
    }
}
