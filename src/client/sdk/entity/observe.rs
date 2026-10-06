// §8.1 — THE VOICE'S READINGS: what this stroke's instruments hold, folded
// into the voice's totals and pushed to its hypha for the console.

use crate::client::sdk::*;
use crate::observe::{self, Loss, Memory, Planes, Reading, Source};

impl Entity {
    /// 78.92 — the reasons this mind's admitted sentences carried, and the
    /// last one's, read off the sentences themselves.
    pub fn reasons(&self) -> crate::transducer::sentence::ReasonsTally {
        self.reasons
    }

    pub fn said(&self) -> Option<&crate::transducer::sentence::Reasons> {
        self.said.as_ref()
    }

    /// Where the court said the last admitted sentence stands.
    pub fn said_at(&self) -> Option<crate::chronometry::Stamp> {
        self.said_at
    }

    /// THIS MIND'S FACULTIES AT ITS OWN `K` (2026-09-29): each read off the
    /// one carrier that holds it (law 12) — sight off the perception's
    /// resonance tally, cognition off the contemplation, speech off the
    /// transcription and the utterances, hearing off the pages and words
    /// that reached the mind, language and learning off the genome and the
    /// dialect, understanding off the pivots and the glue, solving off the
    /// court's verdicts, and the growing carriers' sizes — the leak signals.
    pub fn senses(&self, learning: &observe::Learning) -> observe::Senses {
        let c = &self.contemplation;
        let r = &c.resonance;
        let refused_by_gate =
            [r.monocular, r.collapsed, r.unbalanced, r.destabilizing, r.stability_unread, r.nonrigid, r.unsucceeded, r.torqued].iter().sum();
        let (mut key_bytes, mut bodies, mut grounded) = (0u64, 0u64, 0u64);
        for (k, b) in c.bubbles.scoped() {
            key_bytes += k.len() as u64;
            bodies += b.spores.scoped().map(|(_, on)| on.len() as u64).sum::<u64>();
            grounded += u64::from(b.ground.is_some());
        }
        let merges = self.merge_reading();
        let t = &self.transcription;
        let s = &self.solves;
        observe::Senses {
            k: self.own_k(),
            sight: [
                r.circuits,
                r.cells,
                r.closes,
                r.edges,
                r.scene_motions,
                r.predicted,
                r.consolidated,
                r.reflexive,
                refused_by_gate,
                r.demanded,
            ],
            cognition: [
                c.len() as u64,
                grounded,
                c.closed_thoughts,
                merges.merged,
                c.torsion_merges,
                merges.stayed,
                c.fills,
                c.classes,
            ],
            speech: [self.uttered_n() as u64, t.spelled, t.adjoined, t.ineffable, t.unread, self.colour_theorems],
            reasons: self.reasons.row(),
            said: self.said.clone(),
            derivation: crate::transducer::family::derivation_tally(),
            flux: crate::transducer::family::flux::flux_tally().row(),
            sentences: crate::transducer::sentence::rung::sentence_tally().row(),
            construction: crate::transducer::cech::construction_tally(),
            unread: crate::transducer::sentence::rung::unread_tally(),
            citation: crate::transducer::sentence::rung::citation_tally(),
            encounter: crate::transducer::cech::encounter_tally(),
            hearing: self.hearing,
            language: [
                self.epigenome.dialect_len() as u64,
                self.epigenome.genome_len() as u64,
                learning.installed,
                learning.closing,
                learning.adjoined_by_closure,
                // 78.53 — THE COLOUR VOCABULARY: the court-ruled colour words
                // in the genome (fibre relations, `is_colour_word`). The
                // sensor fixed the labels and a colour word is its relation,
                // so minds that agree hold the same words — and the same
                // count as their court's `colours`.
                self.epigenome.genome_words().filter(|w| w.is_colour_word()).count() as u64,
            ],
            understanding: [
                self.pivot_reading().derivable,
                self.glued.len() as u64,
                self.obstructions.len() as u64,
                self.discordance as u64,
                self.colour_derived,
            ],
            solving: [s.first, s.again, s.refused.iter().sum(), s.unruled, s.unminted, s.refuted, s.withheld_again],
            paid: s.paid,
            carriers: [
                c.len() as u64,
                key_bytes,
                bodies,
                c.pending_rewrites.len() as u64,
                c.grounds.len() as u64,
                c.cavity.len() as u64,
                c.docket_reading().open,
                self.epigenome.dialect_len() as u64,
                self.epigenome.genome_len() as u64,
            ],
        }
    }

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
        // 78.16 C3 — THE PERCEPTS' READINGS, taken each stroke so nothing
        // grows. The percept is the one taker of what perception passed over
        // (law 12): its triples are summed here with what arrived since.
        let mut perception =
            [perceive::take_unheard(), perceive::take_sectionless(), crate::transducer::blade::take_coarse()];
        for held in crate::transducer::family::take_percepts() {
            for (at, n) in perception.iter_mut().zip(held.unheard) {
                *at += n;
            }
        }
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
        // ACT1 — THIS MIND ON THE CLOCK: its own `K`, its court's height as
        // last heard, where its genome was last written (78.21(3)), and its
        // radius. The orbits against other signers' are the court's reading:
        // only the court seals (78.49).
        observe::publish(
            me,
            Reading::Clock(observe::Clock::Voice(observe::VoiceClock {
                k: self.own_k(),
                heard: self.now_tu(),
                persisted: self.genome_persisted,
                radius: learning.radius,
            })),
        );
        // THE FACULTIES, read off this mind's own carriers at its `K`.
        observe::publish(me, Reading::Senses(Box::new(self.senses(&learning))));
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
        // LD1 — THE BYTES THIS VOICE'S FRAMES CARRY, measured whole at its
        // own `K`. A voice carries no receipt: the court's deltas are the
        // court's reading (78.49).
        let mut carriage = observe::Carriage::default();
        carriage.of_frames();
        observe::publish(me, Reading::Carriage(carriage));
        let totals: Vec<Reading> = observe::readings(&me.prefix()).into_iter().map(|(_, r)| r).collect();
        // A push whose readings do not write is not sent (STD1 3c).
        let Some(push) = observe::encode_push(&voice, &totals) else {
            return false;
        };
        match self.session.speak_keyed(&voice, push) {
            Ok(body) => body.as_slice() == [1],
            Err(_) => false,
        }
    }
}
