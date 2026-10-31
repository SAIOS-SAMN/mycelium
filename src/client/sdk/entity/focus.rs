// MU-CLIENT §7.1 — THE CORPUS MET WHERE THE MIND'S ORBITS STAND (ruled
// 2026-10-18: a feed walked in one linear direction leaves the orbits
// unsheared into ellipses; the construction ruled in full).
//
//! THE CORPUS IS A FIELD, NEVER A LIST. Every family of the feed and every
//! open question (a residual chain) is PLACED once at its own coordinates,
//! read off its frames with no extraction: the charts its bodies stand on
//! (`Offer::of`) and the key of 𝒢's orbit its hue fibre moves, read on the
//! map the scenes show before any lift (`fibre_map`, `hue_orbit_key`) — the
//! very coordinate the field files a perception's 𝒢 orbit under.
//! The index is coordinate → items, keyed; no file order, no cursor.
//!
//! THE FOCUS IS THE FIELD'S OWN MOTION. At each winding the mind reads its
//! field (`OrbitRegistry::focus`): the orbits the admissions since the last
//! beat carried, or — none in motion — the orbits that ring. A 𝒢 orbit's key
//! IS a coordinate of the corpus; an orbit on a plane (𝒮, 𝒽) is met through
//! the thoughts threading it, by the charts the thoughts' covers hold. What stands on
//! those coordinates is what the mind meets; the rest lies elsewhere on the
//! manifold, undisturbed — never "passed".
//!
//! PRECESSION, NO SEARCH. The item met — the least by its own coordinates
//! among those on the focus (rule 15: a canonical key, never an arrival) —
//! is taken in: a family perceived on the perception carrier, an open
//! question held as the idea it is. Its admission moves the field: drag
//! carries the orbits of commuting modes along, lift opens orbits on new
//! planes, and those are the next focus. The path through the corpus is the
//! trajectory of the mind's own orbits.
//!
//! THE GROUND STATE. A field with no orbit in motion and none ringing — a
//! mind just restored, its thoughts volatile — is at rest, not deaf: its
//! focus is the charts its language speaks on (`ground_focus`). What stands
//! there resonates sympathetically and is met; its admission excites the
//! field, and the orbits it sets moving take the focus over.

use crate::client::sdk::*;
use crate::network::nerve::Keyed;

/// An item of the corpus: a feed family, by its citation.
const ITEM_FAMILY: u8 = 1;
/// An item of the corpus: an open question, by its residual's address.
const ITEM_QUESTION: u8 = 2;
/// A coordinate of the corpus: a body chart, by its key.
const COORD_CHART: u8 = b'c';
/// A coordinate of the corpus: the key of a 𝒢 orbit.
const COORD_GAUGE: u8 = b'g';

fn tagged(tag: u8, key: &[u8]) -> Vec<u8> {
    let mut k = Vec::with_capacity(key.len() + 1);
    k.push(tag);
    k.extend_from_slice(key);
    k
}

/// THE CORPUS: coordinate → the items standing there, and each item's
/// coordinates (so it leaves every one when it is taken).
#[derive(Default)]
pub(crate) struct Corpus {
    at: Keyed<Keyed<()>>,
    items: Keyed<Vec<Vec<u8>>>,
}

impl Corpus {
    /// An item placed at its coordinates (again: at the coordinates it holds
    /// now).
    fn place(&mut self, item: Vec<u8>, coords: Vec<Vec<u8>>) {
        self.remove(&item);
        for c in &coords {
            self.at.nested_insert(c, &item, ());
        }
        self.items.insert(&item, coords);
    }

    /// An item taken off every coordinate it stood at.
    fn remove(&mut self, item: &[u8]) {
        let Some(coords) = self.items.remove(item) else { return };
        for c in coords {
            let empty = match self.at.get_mut(&c) {
                Some(on) => {
                    on.remove(item);
                    on.is_empty()
                }
                None => false,
            };
            if empty {
                self.at.remove(&c);
            }
        }
    }

    /// THE ITEM THE FOCUS MEETS: the least, by its own coordinates, of the
    /// items standing on any coordinate of `focus`; none where nothing does.
    fn meet(&self, focus: &Keyed<()>) -> Option<Vec<u8>> {
        focus
            .scoped()
            .filter_map(|(c, ())| self.at.get(&c).and_then(|on| on.scoped().next().map(|(item, ())| item)))
            .fold(None, |least: Option<Vec<u8>>, item| match least {
                Some(l) if l <= item => Some(l),
                _ => Some(item),
            })
    }

    /// Items placed, and coordinates held.
    pub(crate) fn size(&self) -> (usize, usize) {
        (self.items.len(), self.at.len())
    }
}

/// THE COORDINATES OF A FAMILY'S FRAMES: its bodies' charts, and the key of
/// the 𝒢 orbit its hue fibre moves, where it moves one.
fn coords_of(train: &crate::transducer::family::Family, fibre: Option<Vec<(u8, u8)>>) -> Vec<Vec<u8>> {
    let mut coords: Vec<Vec<u8>> = super::offer::Offer::of(train).charts.iter().map(|k| tagged(COORD_CHART, k)).collect();
    if let Some(key) = fibre.as_deref().and_then(crate::transducer::cech::orbit::hue_orbit_key) {
        coords.push(tagged(COORD_GAUGE, &key));
    }
    coords
}

impl Mind {
    /// AN OPEN QUESTION PLACED on the corpus: its residual chain's scene read
    /// to its bodies' charts, and its hue map's 𝒢 orbit.
    pub(crate) fn place_question(&mut self, address: &[u8], chain: &[(i64, i64, u8, u8)]) {
        let Some((x, y)) = crate::transducer::family::scene_of_chain(chain) else { return };
        let fibre = crate::transducer::family::hue_map_of_chain(chain);
        let coords = coords_of(&[(x, y)], (!fibre.is_empty()).then_some(fibre));
        self.board.corpus.place(tagged(ITEM_QUESTION, address), coords);
    }

    /// THE FIELD'S FOCUS AS CORPUS COORDINATES (the module's rule); in its
    /// ground state, the charts its language is tuned to.
    fn focus_coords(&self) -> Keyed<()> {
        let mut out: Keyed<()> = Keyed::new();
        // No field to read: no focus at all, never the ground state.
        let Ok(orbits) = crate::transducer::mind::with_field(|f| f.orbits.focus()) else { return out };
        if orbits.is_empty() {
            return self.ground_focus();
        }
        for (lane, key, lines) in orbits {
            if lane == Some(crate::transducer::rewrite::LANE_GAUGE) {
                out.insert(tagged(COORD_GAUGE, &key), ());
                continue;
            }
            for line in lines {
                let Some(b) = self.contemplation.bubbles.get(&line) else { continue };
                for (chart, ()) in b.cover.scoped() {
                    out.insert(tagged(COORD_CHART, &chart), ());
                }
            }
        }
        out
    }

    /// THE GROUND STATE'S FOCUS (ruled 2026-10-18): a field
    /// with no orbit in motion and none ringing is not deaf — an instrument
    /// at rest still has the spectrum its geometry tunes it to. Its focus is
    /// the corpus's charts its language speaks on (`speaks_on`: a closed word
    /// filed under a body there), each read by its key. What stands there is
    /// met sympathetically; its admission excites the field, and the orbits
    /// it sets moving are the focus from then on.
    fn ground_focus(&self) -> Keyed<()> {
        let mut out: Keyed<()> = Keyed::new();
        for (coord, _) in self.board.corpus.at.scoped() {
            if let [COORD_CHART, chart @ ..] = coord.as_slice() {
                if self.epigenome.speaks_on(chart) {
                    out.insert(&coord, ());
                }
            }
        }
        if out.is_empty() {
            return self.curiosity_focus();
        }
        out
    }

    /// THE CORPUS CURIOSITY LAW (ruled 2026-10-23): where the
    /// ground focus meets nothing — the language speaks on no chart the corpus
    /// stands on, and a restarted voice rested at K 0 beside fifty unread
    /// families — the vacuum couples to the NEAREST stratum. A corpus chart's
    /// affinity is the deepest exact prefix of axes (A21, `ChartAxis`
    /// compared exactly) it shares with any chart the language speaks on; the
    /// focus is the corpus charts of greatest affinity, in key order. Where
    /// none shares even one leading axis, the canonically least corpus chart:
    /// a mind is never frozen cold. Engaged only from the ground state, and
    /// only while its exact focus is empty; the first admission's orbits take
    /// the focus over.
    fn curiosity_focus(&self) -> Keyed<()> {
        let language: Vec<crate::transducer::stalk::Invariant> =
            self.epigenome.spoken_charts().iter().filter_map(|k| crate::transducer::stalk::Invariant::from_key(k)).collect();
        curiosity(&language, self.board.corpus.at.scoped().map(|(coord, _)| coord))
    }
}

/// THE CURIOSITY LAW'S SELECTION (`Mind::curiosity_focus`), on the
/// language's charts and the corpus's coordinates in key order: the corpus
/// charts of greatest exact shared-prefix depth with the language; where none
/// shares a leading axis, the least corpus chart alone.
pub(crate) fn curiosity(language: &[crate::transducer::stalk::Invariant], coords: impl Iterator<Item = Vec<u8>>) -> Keyed<()> {
    use crate::transducer::stalk::Invariant;
    {
        let depth = |a: &Invariant, b: &Invariant| a.0.iter().zip(&b.0).take_while(|(x, y)| x == y).count();
        let mut best: Option<usize> = None;
        let mut out: Keyed<()> = Keyed::new();
        for coord in coords {
            let [COORD_CHART, chart @ ..] = coord.as_slice() else { continue };
            let Some(inv) = Invariant::from_key(chart) else { continue };
            // The comparison the ruling states (Concordance 1.10): the deepest
            // prefix shared with any spoken chart; none shared — or a language
            // that speaks on no chart — shares depth 0.
            let affinity = language.iter().fold(0usize, |deepest, l| {
                let d = depth(&inv, l);
                match d > deepest {
                    true => d,
                    false => deepest,
                }
            });
            match best {
                Some(b) if affinity < b => {}
                Some(b) if affinity == b => {
                    out.insert(&coord, ());
                }
                _ => {
                    best = Some(affinity);
                    out = Keyed::new();
                    out.insert(&coord, ());
                }
            }
        }
        // No shared axis at all: the least chart alone (key order is the
        // scoped order, so the first inserted stands first).
        if best == Some(0) {
            if let Some((least, ())) = out.scoped().next() {
                let mut one: Keyed<()> = Keyed::new();
                one.insert(&least, ());
                return one;
            }
        }
        out
    }
}

impl Mind {
    /// THE CORPUS AT A WINDING (the module's rule): the feed's arrivals
    /// placed; then, one scene at a time, the item the focus meets taken in
    /// where the reserve carries its measure — else it waits, and the mind's
    /// strokes are parked meanwhile (`rest`).
    pub(crate) fn corpus_inflow(&mut self, outcome: &mut crate::physics::StrokeOutcome) {
        let arrivals = match self.board.feed.as_mut() {
            Some(feed) => feed.arrivals(),
            None => Vec::new(),
        };
        for (path, task) in arrivals {
            // A task whose members have no A5 form has no identity to think
            // by. MU-CLIENT §5.4 (E1) — the scene is named by the court's own
            // name for it, its 32-byte CITATION.
            let Some(id) = crate::registry::task_address(&task) else { continue };
            let id = crate::registry::citation(&id).to_vec();
            let coords = coords_of(&task.train, crate::transducer::family::fibre_map(&task.train));
            if let Some(feed) = self.board.feed.as_mut() {
                feed.file(&path, &id);
            }
            self.board.corpus.place(tagged(ITEM_FAMILY, &id), coords);
        }
        self.board.feed_waits = false;
        self.board.question_waits = false;
        if self.perceiving() {
            return;
        }
        let focus = self.focus_coords();
        let Some(item) = self.board.corpus.meet(&focus) else { return };
        let (tag, key) = match item.split_first() {
            Some((t, k)) => (*t, k.to_vec()),
            None => return,
        };
        let short = |entity: &Mind, mag: &[u64; 4]| !crate::economy::lanes_short(&entity.physics().reserve_shortfall(crate::physics::Physics::lanes_of(mag))).is_empty();
        match tag {
            ITEM_FAMILY => {
                let Some((path, task)) = self.board.feed.as_ref().and_then(|f| f.take(&key)) else {
                    self.board.corpus.remove(&item);
                    if let Some(f) = self.board.feed.as_mut() {
                        f.forget(&key);
                    }
                    return;
                };
                let mag = super::family::family_magnitude(&task.train);
                if short(self, &mag) {
                    self.board.feed_waits = true;
                    return;
                }
                if let Some(f) = self.board.feed.as_mut() {
                    f.entered(&path, &key);
                }
                self.board.corpus.remove(&item);
                self.physics_mut().bump(&mag);
                crate::physics::StrokeOutcome::add(&mut outcome.fed, &mag);
                self.board.feed_scenes.insert(&key, ());
                // MU-CLIENT §7.1 R1 — perceived on its own carrier, off the
                // field. Its tests are the tests' INPUTS alone (ruled
                // 2026-10-18): what the court withholds never crosses
                // the mind's boundary; a rule known on the family speaks them,
                // and the court judges the solve on its own withheld pairs.
                let tests = task.test.into_iter().map(|(x, _)| x).collect();
                self.perceive_scene(super::perceive::Scene { id: key, train: task.train, tests, reading: super::perceive::Reading::Feed });
            }
            ITEM_QUESTION => {
                let Some(chain) = self.board.open_residuals.get(&key).cloned() else {
                    self.board.corpus.remove(&item);
                    return;
                };
                let mag = crate::transducer::element::measure_lanes(&chain);
                if short(self, &mag) {
                    self.board.question_waits = true;
                    return;
                }
                self.board.open_residuals.remove(&key);
                self.board.corpus.remove(&item);
                self.hold_chain(chain);
            }
            _ => self.board.corpus.remove(&item),
        }
    }
}
