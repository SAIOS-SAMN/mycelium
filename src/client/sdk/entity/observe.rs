// §8.1 — THE VOICE'S READINGS: what this stroke's instruments hold, folded
// into the voice's totals and pushed to its hypha for the console.

use crate::client::sdk::*;
use crate::observe::{self, Loss, Memory, Planes, Reading, Source};

impl Entity {
    /// FOLD THIS STROKE'S INSTRUMENTS AND PUSH THE TOTALS. The tallies are
    /// taken here, by their one reader (law 12): the plane census, what
    /// perception passed over, the mint's gates, and the cover's — which the
    /// caller has already taken for its own line and hands in. The allocator
    /// is sampled at this voice's own `K`.
    ///
    /// `true` where the hypha held the push. A push not held is a console
    /// that reads this voice's last totals, and nothing in cognition waits
    /// on it.
    pub fn observe(&self, cover: &crate::transducer::family::cover::CoverTally) -> bool {
        use crate::transducer::family::perceive;
        let voice = self.session.kernel().voice();
        let me = Source::Voice(voice);
        let perception =
            [perceive::take_unheard(), perceive::take_sectionless(), crate::transducer::blade::take_coarse()];
        observe::publish(me, Reading::Loss(Loss::of(perception, cover, &perceive::take_demand_tally())));
        observe::publish(me, Reading::Planes(Planes::of(&crate::transducer::cga::take_plane_tally())));
        if let Some(m) = Memory::now(self.own_k()) {
            observe::publish(me, Reading::Memory(m));
        }
        let totals: Vec<Reading> = observe::readings(&me.prefix()).into_iter().map(|(_, r)| r).collect();
        match self.session.speak_keyed(&voice, observe::encode_push(&voice, &totals)) {
            Ok(body) => body.as_slice() == [1],
            Err(_) => false,
        }
    }
}
