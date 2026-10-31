// 78.152 TU-2..4 — THE KEEPERS AS ONE COURT HOLDS THEM: its own line of
// readings and each other keeper's, verified on admission (signature, parent
// chain, cites); the final point, on two keepers' verified readings (TU-3);
// the residual the keepers' disagreement leaves on the keeper triangle (TU-4).

use super::point::{Origin, TPoint};
use super::reading::{Cited, Tick, TickReading};
use super::record::LastSigned;
use crate::network::nerve::Keyed;

/// How many recent readings a keeper carries on its digest: an aperture
/// (`physics::aperture_cycle`), the physics' own ratio.
pub fn ring_len() -> usize {
    crate::physics::aperture_cycle() as usize
}

/// How many readings of one keeper a court holds: two rings, so a ring
/// read after a ring links to it.
fn window_len() -> usize {
    2 * ring_len()
}

/// WHY A KEEPER'S RING WAS NOT ADMITTED (TU-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeeperRefusal {
    /// A reading's signature does not verify under its keeper at this origin.
    Signature,
    /// A reading does not name its predecessor's hash, or the ring links to
    /// nothing held and cannot be a gap (a partial ring, not a restart).
    BrokenParent,
    /// The ring's ticks do not strictly rise.
    NotAscending,
    /// A reading cites its own keeper, one keeper twice, or a reading this
    /// court holds under another hash.
    Cites,
}

/// WHAT A RING DID HERE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admitted {
    /// New readings extended the keeper's held line, this many.
    Extended(usize),
    /// The keeper's line stands anew on the ring: the first heard, a restart
    /// (an empty parent above the held tick), or a full ring past a gap.
    Anchored,
    /// Nothing new.
    Stale,
}

/// One keeper's readings held here, by rising tick, each with its hash.
#[derive(Debug, Clone, Default)]
struct Line {
    readings: Vec<([u8; 32], TickReading)>,
}

impl Line {
    fn newest(&self) -> Option<&([u8; 32], TickReading)> {
        self.readings.iter().next_back()
    }

    fn at_h(&self, h: u64) -> Option<&([u8; 32], TickReading)> {
        self.readings.iter().find(|(_, r)| r.tick.h == h)
    }

    fn holds(&self, hash: &[u8; 32]) -> bool {
        self.readings.iter().any(|(x, _)| x == hash)
    }

    fn push(&mut self, at: [u8; 32], r: TickReading) {
        self.readings.push((at, r));
        let over = self.readings.len().checked_sub(window_len());
        if let Some(n) = over.filter(|n| *n > 0) {
            self.readings.drain(..n);
        }
    }
}

/// ONE EDGE OF THE KEEPER TRIANGLE (TU-4): two keepers' readings at one
/// tick, and the exact lane difference `to − from`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub h: u64,
    pub delta: [i128; 4],
}

/// THE CLOCK'S RESIDUAL (TU-4): each edge of the triangle where its two
/// keepers hold readings at one tick, and the holonomy round the triangle
/// in key order — the sum of the three edges' differences, each read at the
/// greatest tick its two keepers share. Zero where all three were read at one
/// tick (a difference of point values closes); the disagreement the edges
/// carry from different ticks is what stays. `None` while an edge is
/// unmeasured. A reading on the observer, never folded into the point.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Residual {
    pub edges: Vec<Edge>,
    pub holonomy: Option<[i128; 4]>,
}

/// THE KEEPERS, AS THIS COURT HOLDS THEM.
#[derive(Debug, Clone)]
pub struct Keepers {
    origin: Origin,
    own_key: [u8; 32],
    lines: Keyed<Line>,
    /// The greatest point final in this court's view so far.
    held_final: Option<TPoint>,
}

impl Keepers {
    /// The keepers at `origin`, `own_key` this court's own key.
    pub fn new(origin: Origin, own_key: [u8; 32]) -> Keepers {
        Keepers { origin, own_key, lines: Keyed::new(), held_final: None }
    }

    /// This court's own key.
    pub fn own_key(&self) -> [u8; 32] {
        self.own_key
    }

    pub fn origin(&self) -> Origin {
        self.origin
    }

    fn line(&self, keeper: &[u8; 32]) -> Option<&Line> {
        self.lines.get(keeper)
    }

    /// Every keeper held, this court's own included, in key order.
    pub fn keepers(&self) -> Vec<[u8; 32]> {
        self.lines.scoped().filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok()).collect()
    }

    /// A keeper's newest reading held here.
    pub fn newest(&self, keeper: &[u8; 32]) -> Option<&TickReading> {
        self.line(keeper)?.newest().map(|(_, r)| r)
    }

    /// THIS COURT READS ITS OWN POINT (TU-1): where the measure moved since
    /// its last reading and the tick rose, a new reading is signed — parent
    /// its last, citing the newest reading it holds of every other keeper.
    /// Answers whether a reading was signed. `None` where the reading cannot
    /// be written.
    pub fn read_own(&mut self, sk: &ed25519_dalek::SigningKey, tick: Tick) -> Option<bool> {
        self.read_own_kept(sk, tick, |_| true)
    }

    /// THE KEEPER'S READING, KEPT BEFORE IT IS HELD (78.152, persisted):
    /// as `read_own`, the new reading handed to `keep` — which writes it to
    /// the home's keeper record — before it is held or can leave on a ring.
    /// A reading `keep` did not keep is not held and never published:
    /// `Some(false)`, as a reading not due.
    pub fn read_own_kept(&mut self, sk: &ed25519_dalek::SigningKey, tick: Tick, keep: impl FnOnce(&LastSigned) -> bool) -> Option<bool> {
        let (parent, due) = match self.line(&self.own_key).and_then(Line::newest) {
            None => ([0u8; 32], true),
            Some((at, r)) => (*at, tick.h > r.tick.h && tick.measure_moved(&r.tick)),
        };
        if !due {
            return Some(false);
        }
        let cites: Vec<Cited> = self
            .keepers()
            .into_iter()
            .filter(|k| *k != self.own_key)
            .filter_map(|k| {
                let (at, r) = self.line(&k)?.newest()?;
                Some(Cited { keeper: k, reading: *at, h: r.tick.h })
            })
            .collect();
        let r = TickReading::sign(sk, &self.origin, tick, parent, cites)?;
        let rec = LastSigned::of(self.origin, r)?;
        if !keep(&rec) {
            return Some(false);
        }
        self.hold(self.own_key, rec.hash, rec.reading);
        Some(true)
    }

    /// A RESTARTED KEEPER RESUMES ITS LINE (78.152, persisted): the last
    /// reading it signed before the restart, read from its home's record,
    /// held as its own line's newest — so its next reading rises above that
    /// tick and names it as parent, and its ring carries it to peers that
    /// missed it. Answers whether it was taken: only a record of this origin,
    /// signed by this court, onto a line that holds nothing at or above it.
    pub fn resume(&mut self, rec: &LastSigned) -> bool {
        let r = &rec.reading;
        let fits = rec.origin == self.origin
            && r.keeper == self.own_key
            && r.verifies(&self.origin)
            && r.hash(&self.origin) == Some(rec.hash)
            && self.line(&self.own_key).and_then(Line::newest).is_none_or(|(_, n)| n.tick.h < r.tick.h);
        if fits {
            self.hold(self.own_key, rec.hash, r.clone());
        }
        fits
    }

    /// This court's own ring: its newest readings, oldest first.
    pub fn own_ring(&self) -> Vec<TickReading> {
        let Some(line) = self.line(&self.own_key) else { return Vec::new() };
        let skip = line.readings.len().checked_sub(ring_len()).filter(|n| *n > 0);
        line.readings.iter().skip(skip.map_or(0, |n| n)).map(|(_, r)| r.clone()).collect()
    }

    fn hold(&mut self, keeper: [u8; 32], at: [u8; 32], r: TickReading) {
        match self.lines.get_mut(&keeper) {
            Some(l) => l.push(at, r),
            None => {
                let mut l = Line::default();
                l.push(at, r);
                self.lines.insert(keeper, l);
            }
        }
        self.settle();
    }

    /// A KEEPER'S RING ADMITTED (TU-2): every reading signed by `keeper` at
    /// this origin; each names its predecessor's hash and rises in tick;
    /// every cite names another keeper once, and agrees with any reading this
    /// court holds at the tick it cites; and the ring links to the line held
    /// here — or stands anew where nothing is held, where the keeper
    /// restarted (an empty parent above the held tick), or where a full ring
    /// passed a gap. A ring of this court's own key is not admitted from
    /// another.
    pub fn admit(&mut self, keeper: [u8; 32], ring: Vec<TickReading>) -> Result<Admitted, KeeperRefusal> {
        if keeper == self.own_key || ring.is_empty() {
            return Ok(Admitted::Stale);
        }
        let mut hashes = Vec::with_capacity(ring.len());
        for (i, r) in ring.iter().enumerate() {
            if r.keeper != keeper || !r.verifies(&self.origin) {
                return Err(KeeperRefusal::Signature);
            }
            let at = r.hash(&self.origin).ok_or(KeeperRefusal::Signature)?;
            if let Some(prev) = i.checked_sub(1).and_then(|j| ring.get(j)) {
                if r.tick.h <= prev.tick.h {
                    return Err(KeeperRefusal::NotAscending);
                }
                if hashes.get(i - 1) != Some(&r.parent) {
                    return Err(KeeperRefusal::BrokenParent);
                }
            }
            self.cites_agree(keeper, r)?;
            hashes.push(at);
        }
        let new = self.link(&keeper, &ring, &hashes)?;
        let (from, anchored) = match new {
            Link::After(i) => (i, false),
            Link::Anew => {
                self.lines.insert(keeper, Line::default());
                (0, true)
            }
            Link::Stale => return Ok(Admitted::Stale),
        };
        let n = ring.len().checked_sub(from).ok_or(KeeperRefusal::BrokenParent)?;
        for (at, r) in hashes.into_iter().zip(ring).skip(from) {
            self.hold(keeper, at, r);
        }
        Ok(match anchored {
            true => Admitted::Anchored,
            false => Admitted::Extended(n),
        })
    }

    /// Every cite of a reading names another keeper, once, and agrees with
    /// the reading this court holds at that tick, where it holds one.
    fn cites_agree(&self, keeper: [u8; 32], r: &TickReading) -> Result<(), KeeperRefusal> {
        for (i, c) in r.cites.iter().enumerate() {
            if c.keeper == keeper || r.cites.iter().take(i).any(|d| d.keeper == c.keeper) {
                return Err(KeeperRefusal::Cites);
            }
            if let Some((held, _)) = self.line(&c.keeper).and_then(|l| l.at_h(c.h)) {
                if *held != c.reading {
                    return Err(KeeperRefusal::Cites);
                }
            }
        }
        Ok(())
    }

    /// Where a verified ring stands against the line held.
    fn link(&self, keeper: &[u8; 32], ring: &[TickReading], hashes: &[[u8; 32]]) -> Result<Link, KeeperRefusal> {
        let Some((held_at, held)) = self.line(keeper).and_then(Line::newest) else { return Ok(Link::Anew) };
        let (Some(first), Some(newest)) = (ring.iter().next(), hashes.iter().next_back()) else { return Ok(Link::Stale) };
        if self.line(keeper).is_some_and(|l| l.holds(newest)) {
            return Ok(Link::Stale);
        }
        if let Some(i) = hashes.iter().position(|x| x == held_at) {
            return Ok(Link::After(i + 1));
        }
        if first.parent == *held_at {
            return Ok(Link::After(0));
        }
        let above = first.tick.h > held.tick.h;
        if above && (first.parent == [0u8; 32] || ring.len() == ring_len()) {
            return Ok(Link::Anew);
        }
        Err(KeeperRefusal::BrokenParent)
    }

    /// THE HIGHEST TICK OTHER KEEPERS VOUCH FOR ON `keeper`'S LINE: per
    /// other keeper, the greatest tick its newest reading cites of it.
    fn vouched(&self, keeper: &[u8; 32]) -> Vec<u64> {
        self.keepers()
            .into_iter()
            .filter(|y| y != keeper)
            .filter_map(|y| self.newest(&y)?.cites.iter().find(|c| c.keeper == *keeper).map(|c| c.h))
            .collect()
    }

    /// A KEEPER'S FINAL POINT (TU-3): its newest reading held at or below a
    /// tick another keeper's verified reading cites — the keeper holds its
    /// own signed reading, the citer holds it by citation: two keepers. A
    /// reading under a parent chain it heads is held with it. `None` where no
    /// other keeper cites the keeper.
    ///
    /// A STRICT MAJORITY OF THE KEEPER SET (ruled 2026-10-09): 2 of
    /// 3, 3 of 4, 3 of 5 — the keeper itself and enough others vouching for
    /// the tick. Two of four could finalise two conflicting points (a, b one
    /// way, c, d the other); a majority cannot. One keeper alone finalises
    /// nothing: with the others down, time does not advance.
    pub fn final_of(&self, keeper: &[u8; 32]) -> Option<TPoint> {
        let line = self.line(keeper)?;
        let set = self.keepers().len();
        // The others a majority needs beside the keeper: ⌊set/2⌋ + 1 − 1,
        // and never fewer than one.
        let need = match set / 2 {
            0 => 1,
            n => n,
        };
        let mut vouched = self.vouched(keeper);
        vouched.sort_unstable();
        // The tick at least `need` others vouch for: the need-th greatest.
        let at = vouched.len().checked_sub(need)?;
        let upto = *vouched.get(at)?;
        line.readings.iter().rev().find(|(_, r)| r.tick.h <= upto).map(|(_, r)| r.tick.at(self.origin))
    }

    /// THE FINAL TUNIVERSAL IN THIS COURT'S VIEW (TU-3): the greatest tick at
    /// which every keeper's final point stands equal — never averaged. Two
    /// keepers final at one tick and differing on a lane leave that tick to
    /// the residual (TU-4), and the final point is the greatest tick below it
    /// at which the finals agree. `None` before any point is final.
    /// Held: a point once final stays final after its readings leave the
    /// window — the clock never runs back (raised by `settle`).
    pub fn final_point(&self) -> Option<TPoint> {
        self.held_final
    }

    /// The final point re-read off the readings held and raised where it
    /// advances (`TPoint::advances`); called wherever a reading is held.
    fn settle(&mut self) {
        if let Some(p) = self.candidate() {
            if p.advances(self.held_final.as_ref()) {
                self.held_final = Some(p);
            }
        }
    }

    fn candidate(&self) -> Option<TPoint> {
        let mut finals: Vec<TPoint> = self.keepers().iter().filter_map(|k| self.final_of(k)).collect();
        // In the point's own coordinate order — one origin here, so rising by
        // tick, then lanes, then K: a canonical key, never a measure ranked.
        finals.sort_unstable();
        let mut out = None;
        let mut i = 0usize;
        while let Some(p) = finals.get(i) {
            let same: Vec<&TPoint> = finals.iter().filter(|q| q.h == p.h).collect();
            if same.iter().all(|q| **q == *p) {
                out = Some(*p);
            }
            i += same.len();
        }
        out
    }

    /// THE CLOCK'S RESIDUAL (TU-4) over the first three keepers in key order.
    pub fn residual(&self) -> Residual {
        // The triangle's corners in key order: the keepers' own order.
        let mut ks = self.keepers().into_iter();
        let (Some(a), Some(b), Some(c)) = (ks.next(), ks.next(), ks.next()) else {
            return Residual::default();
        };
        let (ab, bc, ac) = (self.edge(a, b), self.edge(b, c), self.edge(a, c));
        // a → b → c → a: the third edge is read backwards. A lane past its
        // carrier leaves the holonomy unmeasured (rule 11).
        let holonomy = match (ab, bc, ac) {
            (Some(ab), Some(bc), Some(ac)) => (0..4)
                .map(|l| ab.delta[l].checked_add(bc.delta[l])?.checked_sub(ac.delta[l]))
                .collect::<Option<Vec<i128>>>()
                .and_then(|v| <[i128; 4]>::try_from(v).ok()),
            _ => None,
        };
        Residual { edges: [ab, bc, ac].into_iter().flatten().collect(), holonomy }
    }

    /// One edge: the greatest tick both keepers hold a reading at, and the
    /// exact lane difference there.
    fn edge(&self, from: [u8; 32], to: [u8; 32]) -> Option<Edge> {
        let (lx, ly) = (self.line(&from)?, self.line(&to)?);
        let (_, rx) = lx.readings.iter().rev().find(|(_, r)| ly.at_h(r.tick.h).is_some())?;
        let (_, ry) = ly.at_h(rx.tick.h)?;
        let delta = std::array::from_fn(|l| i128::from(ry.tick.lanes[l]) - i128::from(rx.tick.lanes[l]));
        Some(Edge { from, to, h: rx.tick.h, delta })
    }
}

/// Where a ring stands against a held line.
enum Link {
    /// Its readings from this index on are new.
    After(usize),
    /// The line stands anew on it.
    Anew,
    /// Nothing new.
    Stale,
}
