// MU-CLIENT §4.12, §7.1 — THE OFFER AND THE MIND'S OWN ATTENTION (the operator,
// 2026-10-17: "a sovereign mind treats tasks and percepts as sensory offers,
// not commands"): what a boundary lands — a court's task, a family of the
// feed, a task before a seat — is OFFERED, and the mind takes it only where
// it couples to what its field holds. A field in equilibrium asks for
// nothing.
//
//! THE RESONANCE FILTER. An offer is read once where it lands, to the charts
//! its bodies stand on (`Offer::of`: the family's frames, every body's
//! moduli — the coordinates a perception's thoughts are filed under), and
//! nothing deeper: no extraction, no rotor, no replay. It COUPLES where the
//! mind holds a thought of its scene (a thought perceived in it, or the scene
//! waiting on its comprehension), or where one of its charts is a chart a
//! held thought stands on, a question on the plan covers, or the mind's
//! LANGUAGE speaks on — a closed word of it filed under a body standing
//! there (`Epigenome::speaks_on`; the operator, 2026-10-17: a string resonates
//! because it is tuned to the frequency, not because it already rings; so a
//! restored mind, whose thoughts and plan are volatile, still hears an offer
//! in its own language). An offer that
//! couples is taken as the metabolism can carry it; one that does not draws
//! nothing — no refusal, no penalty, said to the observer by kind
//! (`Board::offers_passed`). A court's task waits, read again as the field
//! moves, and LAPSES at the second return it waits across uncoupled; a feed
//! family is passed at once and left on disk, the feed's walk going on
//! (`feed`); a seat's task is kept by its charts and pulled again only where
//! they couple.
//!
//! THE TENSION. The mind ASKS for an offer (`pull_task`) only where its field
//! holds strain it has not resolved: a demand on its plan, an orbit that
//! rings, a reservoir that neither re-formed nor condensed (`strained`). A
//! full reserve over a settled field is equilibrium; the mind rests.

use crate::client::sdk::*;

/// The kinds of offer a mind passes, as the observer names them, each at its
/// slot of `Board::offers_passed`.
pub(crate) const PASSED: [&str; 3] = ["court task", "feed family", "seat task"];
pub(crate) const PASSED_TASK: usize = 0;
pub(crate) const PASSED_FEED: usize = 1;
pub(crate) const PASSED_SEAT: usize = 2;

/// ONE OFFER STANDING AT A BOUNDARY: the charts its bodies stand on, read
/// once where it landed, and whether it has waited across a return.
#[derive(Clone, Debug, Default)]
pub(crate) struct Offer {
    pub(crate) charts: Vec<Vec<u8>>,
    pub(crate) crossed: bool,
}

impl Offer {
    /// THE OFFER OF A FAMILY: the keys of the charts every body of its frames
    /// stands on, each once, in key order.
    pub(crate) fn of(train: &crate::transducer::family::Family) -> Offer {
        let mut charts: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for (before, after) in crate::transducer::family::perceive_family(train) {
            for body in before.iter().chain(&after) {
                charts.insert(body.moduli.key().as_bytes(), ());
            }
        }
        Offer { charts: charts.scoped().map(|(k, _)| k).collect(), crossed: false }
    }
}

impl Entity {
    /// THE MIND HOLDS TENSION (the module's rule): a demand on its plan, or a
    /// field that is strained (`Field::strained`). Read where the field
    /// stands in the winding carrier's slots; outside them, the plan alone.
    pub(crate) fn strained(&self) -> bool {
        self.contemplation.holds_demand() || matches!(crate::transducer::mind::with_field(|f| f.strained()), Ok(true))
    }

    /// DOES THE OFFER OF SCENE `id`, ON `charts`, COUPLE TO THE MIND (the
    /// module's rule).
    pub(crate) fn couples(&self, id: &[u8], charts: &[Vec<u8>]) -> bool {
        let c = &self.contemplation;
        c.holds_family(id) || self.comprehending.contains(id) || charts.iter().any(|k| c.held_on(k) || c.docket_covers(k) || self.epigenome.speaks_on(k))
    }

    /// An offer of kind `kind` passed: said to the observer.
    pub(crate) fn passed(&mut self, kind: usize) {
        if let Some(slot) = self.board.offers_passed.get_mut(kind) {
            let _ = crate::physics::metabolism::step_or_refuse(slot, 1);
        }
    }

    /// AT A RETURN, THE COURT'S TASK WAITING IS READ AGAIN: one that waited
    /// across the last return and still couples to nothing lapses, let go;
    /// otherwise it crosses this return.
    pub(crate) fn lapse_offers(&mut self) {
        let task = self.board.court.task.as_ref().map(|(e, o)| (o.crossed, self.couples(&e.id, &o.charts)));
        match task {
            Some((true, false)) => {
                self.board.court.task = None;
                self.passed(PASSED_TASK);
            }
            Some(_) => {
                if let Some((_, o)) = self.board.court.task.as_mut() {
                    o.crossed = true;
                }
            }
            None => {}
        }
    }
}
