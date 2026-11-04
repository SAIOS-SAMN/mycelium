// MU-CLIENT §4.12, §7.1 — THE OFFER AND THE MIND'S OWN ATTENTION (ruled
// 2026-10-17: a seated mind treats tasks and percepts as sensory offers,
// not commands): what a boundary lands — a court's task, a family of the
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
//! there (`Epigenome::speaks_on`; ruled 2026-10-17: a string resonates
//! because it is tuned to the frequency, not because it already rings; so a
//! restored mind, whose thoughts and plan are volatile, still hears an offer
//! in its own language). An offer that
//! couples is taken as the metabolism can carry it; one that does not draws
//! nothing — no refusal, no penalty, said to the observer by kind
//! (`Board::offers_passed`). A court's task waits, read again as the field
//! moves, and LAPSES at the second return it waits across uncoupled; a feed
//! family is never offered at all — it stands on the corpus at its own
//! coordinates and is met where the mind's orbits stand (`focus`); a seat's
//! task is kept by its charts and pulled again only where they couple.
//!
//! THE TENSION. The mind ASKS for an offer (`pull_task`) where it holds
//! strain it has not resolved: an open question (a residual chain, never a
//! word), a demand on its plan, an orbit that rings, a reservoir that neither
//! re-formed nor condensed (`strained`) — or where its corpus holds no item
//! (2026-10-24). A full reserve over a settled field whose corpus holds items
//! asks for nothing; the mind rests.

use crate::client::sdk::*;

/// The kinds of offer a mind passes, as the observer names them, each at its
/// slot of `Board::offers_passed`.
/// A seat passes nothing: its house's tasks are every one evaluated (ruled
/// 2026-10-23), so the court's task is the one kind.
pub(crate) const PASSED: [&str; 1] = ["court task"];
pub(crate) const PASSED_TASK: usize = 0;

/// ONE OFFER STANDING AT A BOUNDARY: the charts its bodies stand on, read
/// once where it landed, whether it has waited across a return, the strokes
/// it has waited in the ground state, and the court's final point `h` it
/// landed under (`None`: no final carried yet).
#[derive(Clone, Debug, Default)]
pub(crate) struct Offer {
    pub(crate) charts: Vec<Vec<u8>>,
    pub(crate) crossed: bool,
    pub(crate) waited: u64,
    pub(crate) landed: Option<u64>,
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
        Offer { charts: charts.scoped().map(|(k, _)| k).collect(), crossed: false, waited: 0, landed: None }
    }
}

impl Mind {
    /// THE MIND HOLDS TENSION (the module's rule): a demand on its plan, or a
    /// field that is strained (`Field::strained`). Read where the field
    /// stands in the winding carrier's slots; outside them, the plan alone.
    pub(crate) fn strained(&self) -> bool {
        !self.board.open_residuals.is_empty() || self.contemplation.holds_demand() || matches!(crate::transducer::mind::with_field(|f| f.strained()), Ok(true))
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
            Some((true, false)) => self.let_go_task(),
            Some(_) => {
                if let Some((_, o)) = self.board.court.task.as_mut() {
                    o.crossed = true;
                }
            }
            None => {}
        }
    }

    /// THE TASK'S BOUND (ruled 2026-10-24): the court's task waiting
    /// uncoupled lapses at the first of the second return it waits across
    /// (`lapse_offers`), its wait reaching one aperture of the measured
    /// transport (`physics::persist_dwell`), or a court final past the one
    /// it landed under. A field that never returns would otherwise hold it
    /// without end. Read once a stroke; with the transport unmeasured, the
    /// return and the final bound it.
    pub(crate) fn lapse_standing_task(&mut self) {
        let aperture = self.transport_rtt().map(crate::physics::persist_dwell);
        let finality = self.hypha.tuniversal().map(|p| p.h);
        let coupled = match self.board.court.task.as_ref() {
            Some((e, o)) => self.couples(&e.id, &o.charts),
            None => return,
        };
        let Some((_, o)) = self.board.court.task.as_mut() else { return };
        let _ = crate::physics::metabolism::step_or_refuse(&mut o.waited, 1);
        if coupled {
            return;
        }
        let past_aperture = matches!(aperture, Some(crate::physics::Lambda::Strokes(n)) if o.waited >= n);
        let next_final = match (o.landed, finality) {
            (Some(at), Some(now)) => now > at,
            (None, Some(_)) => true,
            (_, None) => false,
        };
        if past_aperture || next_final {
            self.let_go_task();
        }
    }

    /// The court's task let go uncoupled: kept as lapsed by its citation, and
    /// passed, said to the observer.
    fn let_go_task(&mut self) {
        if let Some((e, _)) = self.board.court.task.take() {
            self.note_task(&e.id, super::perceive::Reading::Task, |t| t.stop = Some(super::perceive::Stop::Lapsed));
        }
        self.passed(PASSED_TASK);
    }
}
