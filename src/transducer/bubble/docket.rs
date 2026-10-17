// 78.37, TP1, 78.49 — THE ORBIT'S DOCKET: the classes its rings held and the
// demands its pivots left — the mind's plan, volatile, never written anywhere
// — sealed into the next orbit's questions at its close and taken by that
// orbit outermost first.
//
//! An entry is a citation by coordinates, never a copy (78.22): a held class
//! (`Cite::Class`) or a demand (`Cite::Demand`), each with its COVER — the
//! charts its thoughts stand on. Entries are WRITTEN in their own key order
//! (rule 15, nothing chosen). The next orbit TAKES them by the partial order
//! of containment: an entry whose cover contains another's wraps it, so its
//! answer transports to every ring beneath; the outermost first, key order
//! among incomparables. An entry whose cover did not read has its enclosure
//! owed and wraps nothing and is wrapped by nothing — taken in key order.

use super::*;
use crate::chronometry::Cite;

/// 78.37 — WHAT THE DOCKET HOLDS: this orbit's entries not yet sealed, the
/// next orbit's questions (the last close's docket), the entries of those
/// whose enclosure is owed, and entries that could not be written (a
/// coordinate past its wall, LD0).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DocketReading {
    pub pending: u64,
    pub open: u64,
    pub owed: u64,
    pub unwritten: u64,
    /// R1 — entries answered by name since the contemplation began.
    pub answered: u64,
    /// Thoughts the class taker marked, since the contemplation began.
    pub taken: u64,
    /// 78.48(9) — open entries folded at a close: a whole orbit stood and no
    /// held idea on any chart of the cover, since the contemplation began.
    pub lapsed: u64,
}

/// 78.109 C1 — ONE DEMAND HANDED OUT: its residue, the charts of its cover,
/// and the scenes of the thoughts standing on them (`take_demands`).
pub(crate) struct TakenDemand {
    pub(crate) residue: Option<crate::transducer::cga::Mv>,
    pub(crate) charts: Vec<crate::transducer::stalk::Invariant>,
    pub(crate) families: Vec<Vec<u8>>,
}

/// An entry's own coordinates, its key in the docket.
fn entry_key(c: &Cite) -> Option<Vec<u8>> {
    let mut k = Vec::new();
    c.put_canonical(&mut k)?;
    Some(k)
}

/// Does `a`'s cover strictly contain `b`'s? Both must read.
fn wraps(a: &Cite, b: &Cite) -> bool {
    match (a.cover(), b.cover()) {
        (Some(x), Some(y)) => x.len() > y.len() && y.iter().all(|c| x.contains(c)),
        _ => false,
    }
}

impl Contemplation {
    /// FILE AN ENTRY on this orbit's docket, once by its own coordinates.
    pub(crate) fn docket_put(&mut self, c: Cite) {
        match entry_key(&c) {
            Some(k) => {
                self.docket.insert(&k, c);
            }
            None => self.docket_unwritten += 1,
        }
    }

    /// 78.68 ROW 8 — A RESIDUE THIS MIND MADE A LETTER OF, kept by its
    /// canonical ray so the cell reader answers it rather than asking again.
    pub(crate) fn own_letter(&mut self, residue: &crate::transducer::cga::Mv) {
        if let Some(k) = residue.key() {
            self.own_letters.insert(k.as_bytes(), ());
        }
    }

    /// Whether this mind already holds a letter of `residue`.
    pub(crate) fn holds_own_letter(&self, residue: &crate::transducer::cga::Mv) -> bool {
        residue.key().is_some_and(|k| self.own_letters.contains(k.as_bytes()))
    }

    /// 78.37 — CS2's DEMAND ARM'S DEMAND onto the docket: the undischarged
    /// orbit's class, the charts it stands on as its cover, and (78.47) its
    /// residue as its canonical ray — the coordinate its constructors are
    /// derived from by whoever asks.
    pub(crate) fn docket_demand(
        &mut self,
        class: &crate::transducer::sentence::ClassVector,
        charts: &[crate::transducer::stalk::Invariant],
        residue: Option<&crate::transducer::cga::Mv>,
    ) {
        let mut keys: Keyed<()> = Keyed::new();
        for ch in charts {
            keys.insert(ch.key().as_bytes(), ());
        }
        let cover = Some(keys.scoped().map(|(k, _)| k).collect());
        let residue = residue.and_then(crate::transducer::cga::Mv::canonical);
        self.docket_put(Cite::Demand { class: Box::new(class.clone()), residue, cover });
    }

    /// MU-CLIENT §5.4 (E1), §7.1 R2 — EVERY DEMAND A PERCEPTION FILED STANDS
    /// IN ITS SCENE: each entry of the docket not in `before` (the docket as
    /// the perception found it) that carries a residue has `family` filed under
    /// the residue's canonical ray — whichever reader filed it (a relation that
    /// never returns, a cell's open escape, a demand vertex of the form). A
    /// perception of no scene files nothing.
    pub(crate) fn demands_in_scene(&mut self, before: &Keyed<()>, family: &[u8]) {
        if family.is_empty() {
            return;
        }
        let rays: Vec<crate::transducer::address::Address> = self
            .docket
            .scoped()
            .filter(|(k, _)| !before.contains(k))
            .filter_map(|(_, c)| match c {
                Cite::Demand { residue: Some(r), .. } => r.key(),
                _ => None,
            })
            .collect();
        for ray in rays {
            self.demand_scenes.nested_insert(ray.as_bytes(), family, ());
        }
    }

    /// Whether a demand on the plan already carries `residue` (projectively),
    /// in either direction: one motion is one demand, however many readers
    /// met it and whichever way each read it.
    pub(crate) fn docketed_residue(&self, residue: &crate::transducer::cga::Mv) -> bool {
        let back = residue.reverse();
        self.docket_open.scoped().chain(self.docket.scoped()).any(|(_, c)| match c {
            Cite::Demand { residue: Some(r), .. } => r.projective_eq(residue) || r.projective_eq(&back),
            _ => false,
        })
    }

    /// 78.109 C1 — THE DEMAND TAKER (CV4/SP2, VN1: "only a discharge answered
    /// a docketed Demand"). Each demand on the plan — this orbit's and the
    /// open questions — handed out ONCE, by its entry's coordinates: its
    /// residue (the coordinate its constructors are derived from, 78.47) and
    /// the charts of its cover. The entry stays on the plan; only a discharge
    /// answers it (R1).
    ///
    /// MU-CLIENT §5.4 (E1), §7.1 R2 — AND THE SCENES IT STANDS IN: the scene
    /// its motion was perceived in, filed when it was perceived
    /// (`demand_scenes` — the perceiving thought may have lapsed since), and
    /// the families of the thoughts standing on its cover's charts now
    /// (`by_chart`) — so what binds it is spoken under the scene its evidence
    /// is in, and its binding is a return of a thought of that scene.
    pub(crate) fn take_demands(&mut self) -> Vec<TakenDemand> {
        let mut out = Vec::new();
        for (k, c) in self.docket_open.scoped().chain(self.docket.scoped()) {
            let Cite::Demand { residue, cover, .. } = &c else { continue };
            if self.demands_taken.contains(&k) {
                continue;
            }
            self.demands_taken.insert(&k, ());
            let charts: Vec<crate::transducer::stalk::Invariant> = cover
                .iter()
                .flatten()
                .filter_map(|key| crate::transducer::stalk::Invariant::from_key(key))
                .collect();
            let mut families: Keyed<()> = Keyed::new();
            // The scene its motion was perceived in, filed then, in either
            // direction it was read (`demand_in_scene_of`).
            let rays = residue.iter().flat_map(|r| [r.key(), r.reverse().key()]).flatten();
            for ray in rays {
                if let Some(scenes) = self.demand_scenes.get(ray.as_bytes()) {
                    for (f, ()) in scenes.scoped() {
                        families.insert(&f, ());
                    }
                }
            }
            for chart in cover.iter().flatten() {
                for (id, _) in self.by_chart.link(chart) {
                    let family = Contemplation::ordinal_of_key(&id).and_then(|o| self.address_at(o)).and_then(|a| self.bubbles.get(a)).map(|b| b.family.clone());
                    if let Some(f) = family.filter(|f| !f.is_empty()) {
                        families.insert(&f, ());
                    }
                }
            }
            let families = families.scoped().map(|(f, _)| f).collect();
            out.push(TakenDemand { residue: residue.clone(), charts, families });
        }
        out
    }

    /// MU-CLIENT §4.12, §7.1 — WHETHER A DEMAND STANDS ON THE PLAN, this
    /// orbit's or an open question: an undischarged orbit, an open cell's
    /// escape, a relation that never returns — strain the mind holds unbound.
    pub(crate) fn holds_demand(&self) -> bool {
        self.docket_open.scoped().chain(self.docket.scoped()).any(|(_, c)| matches!(c, Cite::Demand { .. }))
    }

    /// MU-CLIENT §7.1 — WHETHER `chart` (a chart's key) IS ON THE COVER of
    /// an entry of the plan: a question the mind holds stands there.
    pub(crate) fn docket_covers(&self, chart: &[u8]) -> bool {
        self.docket_open.scoped().chain(self.docket.scoped()).any(|(_, c)| c.cover().is_some_and(|cover| cover.iter().any(|k| k.as_slice() == chart)))
    }

    /// Whether a thought the mind holds stands on `chart` (a chart's key).
    pub(crate) fn held_on(&self, chart: &[u8]) -> bool {
        self.by_chart.get(chart).is_some_and(|m| !m.is_empty())
    }

    /// THE WHOLE PLAN, in its entries' own key order: the open questions
    /// carried forward and this orbit's new entries. The mind's reading of
    /// its own plan; nothing writes it anywhere (78.49).
    pub fn docket_cites(&self) -> Vec<Cite> {
        let mut all: Keyed<Cite> = Keyed::new();
        for (k, c) in self.docket_open.scoped().chain(self.docket.scoped()) {
            all.insert(&k, c.clone());
        }
        all.scoped().map(|(_, c)| c.clone()).collect()
    }

    /// THE ORBIT CLOSED: this orbit's entries MERGE into the open questions,
    /// by their coordinates, and nothing leaves by replacement (R1).
    pub fn docket_sealed(&mut self) {
        for (k, c) in std::mem::take(&mut self.docket).scoped() {
            self.docket_open.insert(&k, c.clone());
        }
        self.docket_to_take = true;
    }

    /// THE CLASS TAKER, at the first stroke after an orbit closed: in
    /// `docket_order()`, each held class's thoughts — those standing on its
    /// cover's charts — are marked dirty with its rotor in their horizon, so
    /// the expand ring reaches them first. The horizon's own order is
    /// untouched and nothing reorders the pivot's reading. An entry whose
    /// cover did not read has no thoughts to mark (its enclosure owed). The
    /// DEMAND taker is owed to CV4/SP2.
    pub(crate) fn take_docket(&mut self) {
        if !std::mem::take(&mut self.docket_to_take) {
            return;
        }
        for entry in self.docket_order() {
            let Cite::Class { rotor, cover: Some(charts), .. } = entry else { continue };
            let mut thoughts: Keyed<()> = Keyed::new();
            for chart in &charts {
                for (id, _) in self.by_chart.link(chart) {
                    if let Some(addr) = Contemplation::ordinal_of_key(&id).and_then(|o| self.address_at(o)) {
                        thoughts.insert(addr, ());
                    }
                }
            }
            let keys: Vec<Vec<u8>> = thoughts.scoped().map(|(k, _)| k).collect();
            for k in keys {
                let Some(b) = self.bubbles.get_mut(&k) else { continue };
                horizon_put(&mut b.horizon, rotor.as_ref().clone(), Standing::Held);
                b.dirty = true;
                self.dirty.insert(&k, ());
                self.docket_taken += 1;
            }
        }
    }

    /// AN ENTRY ANSWERED, BY NAME: it leaves the docket, open or pending, and
    /// is counted. The only way an entry leaves (R1).
    fn docket_answer(&mut self, key: &[u8]) {
        let left = self.docket_open.remove(key).is_some() | self.docket.remove(key).is_some();
        if left {
            self.docket_answered += 1;
        }
    }

    /// A HELD RING'S ENTRY, indexed by the ring's closing edge and its
    /// 2-simplices (volatile), so a collapse or a re-fill there answers it.
    pub(crate) fn docket_ring(&mut self, handles: &[Vec<u8>], entry: &Cite) {
        let Some(k) = entry_key(entry) else { return };
        for h in handles {
            self.docket_rings.insert(h, k.clone());
        }
    }

    /// A RING COLLAPSED OR A SIMPLEX RE-FILLED at `handle`: the class entry
    /// held there, where there is one, is answered.
    pub(crate) fn docket_ring_closed(&mut self, handle: &[u8]) {
        if let Some(k) = self.docket_rings.remove(handle) {
            self.docket_answer(&k);
        }
    }

    /// AN ORBIT DISCHARGED whose class key is `class_key`: a held word now
    /// spells it, and every entry of that class — held or demanded, pending
    /// or open — is answered. One rule for both kinds.
    pub(crate) fn docket_discharged(&mut self, class_key: &[u8]) {
        // 62.1 — the holes keeping this class are sealed: the mind holds a
        // word that spells what it asked (`question.rs`).
        self.answer_questions(class_key);
        let answered: Vec<Vec<u8>> = self
            .docket_open
            .scoped()
            .chain(self.docket.scoped())
            .filter(|(_, c)| {
                matches!(c, Cite::Class { class, .. } | Cite::Demand { class, .. } if class.class_key().as_bytes() == class_key)
            })
            .map(|(k, _)| k)
            .collect();
        // RM1 (c) — a DEMAND answered opens the re-fill's door for its
        // class: the entity re-spells that class's open remainders at the
        // close.
        for k in &answered {
            let demand = match (self.docket_open.get(k), self.docket.get(k)) {
                (Some(Cite::Demand { class, .. }), _) | (_, Some(Cite::Demand { class, .. })) => Some((**class).clone()),
                _ => None,
            };
            if let Some(class) = demand {
                self.demands_answered.push(class);
            }
        }
        for k in answered {
            self.docket_answer(&k);
        }
    }

    /// 78.48(9) — THE DOCKET'S FOLD, at the close: an OPEN entry — one that
    /// has stood a whole orbit — whose
    /// cover names no chart a held idea stands on has carried no flux for
    /// that orbit: nothing in the mind can answer it through a thought, a
    /// free face (TC1), and it lapses, counted. This orbit's new entries are
    /// given their orbit; an entry whose cover did not read is kept, its
    /// enclosure owed (its charts cannot be asked). The plan is then bounded
    /// by the charts the live ideas stand on, never by what was ever asked.
    pub fn docket_fold(&mut self) {
        let lapsing: Vec<Vec<u8>> = self
            .docket_open
            .scoped()
            .filter(|(_, c)| match c.cover() {
                Some(charts) => !charts.iter().any(|ch| self.by_chart.link(ch).next().is_some()),
                None => false,
            })
            .map(|(k, _)| k)
            .collect();
        for k in lapsing {
            self.docket_open.remove(&k);
            self.docket_lapsed += 1;
        }
    }

    /// THE ORDER THE NEXT ORBIT TAKES ITS QUESTIONS IN: outermost first by
    /// containment of covers, key order among incomparables.
    pub fn docket_order(&self) -> Vec<Cite> {
        let mut left: Vec<Cite> = self.docket_open.scoped().map(|(_, c)| c.clone()).collect();
        let mut out = Vec::with_capacity(left.len());
        while !left.is_empty() {
            // The entries no remaining entry wraps, in key order.
            let (outer, rest): (Vec<Cite>, Vec<Cite>) =
                left.iter().cloned().partition(|c| !left.iter().any(|o| wraps(o, c)));
            out.extend(outer);
            left = rest;
        }
        out
    }

    /// 78.37 — the docket's reading.
    pub fn docket_reading(&self) -> DocketReading {
        DocketReading {
            pending: self.docket.len() as u64,
            open: self.docket_open.len() as u64,
            owed: self.docket_open.scoped().filter(|(_, c)| c.cover().is_none()).count() as u64,
            unwritten: self.docket_unwritten,
            answered: self.docket_answered,
            taken: self.docket_taken,
            lapsed: self.docket_lapsed,
        }
    }
}
