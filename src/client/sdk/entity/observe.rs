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
        // LC1 — THE CURVE, read off this mind's own genome at its own `K`,
        // once: the clock's radius is its span's depth.
        let learning = observe::Learning::of(
            &self.epigenome,
            self.own_k(),
            self.pivot_reading(),
            self.merge_reading(),
            self.meet_graph(),
            self.probes.clone(),
        );
        // ACT1 — THIS VOICE ON THE CLOCK: its `K`, its court's height as last
        // heard, where its genome was last written (78.21(3)), its radius
        // and its orbits against the others' (78.23 as amended, OK1).
        let voice_seat = self.session.voice();
        observe::publish(
            me,
            Reading::Clock(observe::Clock::Voice(observe::VoiceClock {
                k: self.own_k(),
                heard: self.now_tu(),
                persisted: self.genome_persisted,
                radius: learning.radius,
                orbits: observe::Orbits { withheld: voice_seat.withheld(), distance: voice_seat.chain().separation() },
            })),
        );
        observe::publish(me, Reading::Learning(learning));
        // PL1 — THE LAST PERCEPT'S CIRCUITS, step by step, where a percept
        // read any since the last push; otherwise the circuits held stand.
        if let Some(c) = crate::transducer::sentence::take_circuits() {
            observe::publish(me, Reading::Circuit(c));
        }
        // CO1 — AND ITS CELLS, as their orbit algebras read them.
        if let Some(c) = crate::transducer::family::take_cells() {
            observe::publish(me, Reading::Cells(c));
        }
        // VO1, SP1, PG1 — AND ITS BENDS' VISCOSITY, THE STROKE BY READER AND
        // ITS PROGRESS, each summed over this voice's percepts per axis.
        if let Some(v) = crate::transducer::family::take_viscosity() {
            observe::publish(me, Reading::Viscosity(v));
        }
        if let Some(s) = crate::transducer::family::take_spend() {
            observe::publish(me, Reading::Spend(s));
        }
        if let Some(p) = crate::transducer::family::take_progress() {
            observe::publish(me, Reading::Progress(p));
        }
        // LD1 — THE BYTES THIS VOICE'S RECEIPT DELTAS AND FRAMES CARRY,
        // measured whole at its own `K`.
        let mut carriage = observe::Carriage::default();
        carriage.of_chain(self.session.voice.chain());
        carriage.of_frames();
        observe::publish(me, Reading::Carriage(carriage));
        let totals: Vec<Reading> = observe::readings(&me.prefix()).into_iter().map(|(_, r)| r).collect();
        match self.session.speak_keyed(&voice, observe::encode_push(&voice, &totals)) {
            Ok(body) => body.as_slice() == [1],
            Err(_) => false,
        }
    }
}
