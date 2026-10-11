// 78.164 — THE FIELD RELAXES: where the reserve is short or nothing flowed
// in, the mind does its inward work — it sheds, expands its dirty ideas,
// pivots held thoughts on the charts its language now holds, takes in the
// court's language where it is due, proposes the optimization its vocabulary
// holds (a word the rest reproduces retires, MU-CLIENT §5.4), and
// regenerates.

use crate::client::sdk::*;
use crate::physics::StrokeOutcome;

impl Entity {
    pub(crate) fn relax(&mut self, outcome: &mut StrokeOutcome) {
        let _ = self.shed();
        let (pivoted, _) = self.expand_ideas();
        StrokeOutcome::add(&mut outcome.pivoted, &pivoted);
        let _ = self.pivot_thoughts();
        self.language_inflow();
        let mut budget = self.stroke();
        let _ = self.propose_retirement(&mut budget);
        self.physics_mut().regenerate();
        self.physics_mut().relax();
    }
}
