// 16.11, 26 — THE COVER: the trie that homes a word by incidence.
//
//! Descent is by longest common prefix on the chart's lane, never by XOR
//! distance: a word goes where its own coordinates say, and a node's room
//! is its bond against its occupancy. `home` refuses when none dominates.

use super::*;

/// An invariant axis as a Keyed key: the coordinate little-endian, so a
/// prefix of axes is a prefix of the plane.
pub fn axis_key_le(a: u64) -> [u8; 8] {
    a.to_le_bytes()
}

fn axis_of_le(key: &[u8]) -> u64 {
    let mut b = [0u8; 8];
    let n = key.len().min(8);
    b[..n].copy_from_slice(&key[..n]);
    u64::from_le_bytes(b)
}

/// Cover trie: per lane, nested Keyed by invariant axes. The fiber at a
/// node is the hyphas covering that plane (this prefix and every finer
/// chart beneath it). Own charts and peer charts share the trie.
#[derive(Clone, Debug)]
pub struct Cover {
    lanes: [CoverNode; 4],
    /// Cover root per lane, hashed in trie order on admit/retract.
    roots: [[u8; 32]; 4],
    /// Whether each lane has moved since its root was hashed.
    due: [bool; 4],
    /// Charts each hypha holds, so retract walks those paths only.
    held: Keyed<Vec<(u8, crate::transducer::stalk::Invariant)>>,
}

#[expect(
    clippy::disallowed_methods,
    reason = "law 2 — a cover root is the cold routing index over bytes, not an identity of the charts it stands for"
)]
fn empty_cover_root() -> [u8; 32] {
    *blake3::Hasher::new().finalize().as_bytes()
}

impl Default for Cover {
    fn default() -> Self {
        Cover {
            lanes: Default::default(),
            roots: [empty_cover_root(); 4],
            due: [false; 4],
            held: Keyed::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct CoverNode {
    here: Keyed<()>,
    next: Keyed<CoverNode>,
}

impl CoverNode {
    fn is_empty(&self) -> bool {
        self.here.is_empty() && self.next.is_empty()
    }

    fn admit(&mut self, axes: &[u64], hypha: &[u8]) {
        match axes.split_first() {
            None => {
                self.here.insert(hypha, ());
            }
            Some((&a, rest)) => {
                let k = axis_key_le(a);
                match self.next.get_mut(&k) {
                    Some(ch) => ch.admit(rest, hypha),
                    None => {
                        let mut ch = CoverNode::default();
                        ch.admit(rest, hypha);
                        self.next.insert(k, ch);
                    }
                }
            }
        }
    }

    fn collect_into(&self, out: &mut Keyed<()>) {
        // Each carrier here is ALREADY one stratum's fiber: `here` is the set
        // of hyphae standing at this plane, `next` is this node's own
        // children. The descent that scoped them is the axis path above.
        for (id, _) in self.here.scoped() {
            out.insert(id, ());
        }
        for (_, child) in self.next.scoped() {
            child.collect_into(out);
        }
    }

    fn collect(&self) -> Vec<Vec<u8>> {
        let mut out = Keyed::new();
        self.collect_into(&mut out);
        // A carrier this function built, holding exactly what it collected.
        out.scoped().map(|(k, _)| k).collect()
    }

    /// Longest common prefix: descend while axes match. A miss after some
    /// match returns the fiber at that plane. A miss at the first axis is
    /// novel — empty. The empty chart (no axes) is the root fiber.
    fn lcp(&self, axes: &[u64], matched: bool) -> Vec<Vec<u8>> {
        match axes.split_first() {
            None => self.collect(),
            Some((&a, rest)) => match self.next.get(&axis_key_le(a)) {
                Some(child) => child.lcp(rest, true),
                None if matched => self.collect(),
                None => Vec::new(),
            },
        }
    }

    /// Retract one hypha along one chart path (the hypha's own axes).
    fn retract_path(&mut self, axes: &[u64], hypha: &[u8]) {
        match axes.split_first() {
            None => {
                self.here.remove(hypha);
            }
            Some((&a, rest)) => {
                let k = axis_key_le(a);
                let empty = {
                    let Some(ch) = self.next.get_mut(&k) else { return };
                    ch.retract_path(rest, hypha);
                    ch.is_empty()
                };
                if empty {
                    self.next.remove(&k);
                }
            }
        }
    }

    fn hash_charts(&self, hasher: &mut blake3::Hasher, path: &mut Vec<u64>) {
        if !self.here.is_empty() {
            for a in path.iter() {
                hasher.update(&a.to_le_bytes());
            }
            hasher.update(&[0xff]);
        }
        // Trie order is key order: hash without materialising a Vec of
        // charts. `next` is this node's own children (scoped by the axis
        // path above), and every one of them is in the hash by definition.
        for (k, ch) in self.next.scoped() {
            path.push(axis_of_le(k.as_slice()));
            ch.hash_charts(hasher, path);
            path.pop();
        }
    }

    fn walk(&self, path: &mut Vec<u64>, out: &mut Vec<crate::transducer::stalk::Invariant>) {
        if !self.here.is_empty() {
            out.push(crate::transducer::stalk::Invariant(path.clone()));
        }
        for (k, ch) in self.next.scoped() {
            path.push(axis_of_le(k.as_slice()));
            ch.walk(path, out);
            path.pop();
        }
    }
}

impl Cover {
    pub fn new() -> Self {
        Cover::default()
    }

    pub fn admit(&mut self, lane: u8, inv: &crate::transducer::stalk::Invariant, hypha: impl AsRef<[u8]>) {
        let hypha = hypha.as_ref();
        let lane = lane & 3;
        self.lanes[lane as usize].admit(&inv.0, hypha);
        match self.held.get_mut(hypha) {
            Some(charts) => {
                if !charts.iter().any(|(l, i)| *l == lane && i == inv) {
                    charts.push((lane, inv.clone()));
                }
            }
            None => {
                self.held.insert(hypha, vec![(lane, inv.clone())]);
            }
        }
        self.refresh_root(lane);
    }

    /// Drop every chart of this hypha. Retract along the hypha's own chart
    /// paths rather than visiting the whole trie.
    pub fn retract_hypha(&mut self, hypha: &[u8]) {
        let Some(charts) = self.held.remove(hypha) else {
            return;
        };
        let mut touched = [false; 4];
        for (lane, inv) in charts {
            self.lanes[lane as usize].retract_path(&inv.0, hypha);
            touched[lane as usize] = true;
        }
        for (l, t) in touched.iter().enumerate() {
            if *t {
                self.refresh_root(l as u8);
            }
        }
    }

    /// THE LANE MOVED. The root over it is not rehashed here.
    ///
    /// `admit` and `retract` called this per chart, and each call walks the
    /// WHOLE lane trie — every hypha's charts on that lane. So
    /// `rebuild_own_cover`, which admits one chart at a time and runs every
    /// exchange cycle, rehashed the entire mesh cover once per chart it
    /// holds: own charts × the whole trie, per cycle, and both grow — own
    /// charts with every corpus body listed, the trie with peer count.
    ///
    /// A root is a reading of the lane, taken where it is asked for
    /// (law 12): `roots()`, once per digest.
    fn refresh_root(&mut self, lane: u8) {
        self.due[(lane as usize) & 3] = true;
    }

    /// The four lane roots, hashed for any lane that has moved since the
    /// last reading.
    #[expect(
        clippy::disallowed_methods,
        reason = "law 2 — a lane root is the cold routing index over bytes, not an identity of the charts it stands for"
    )]
    pub fn roots(&mut self) -> [[u8; 32]; 4] {
        for lane in 0..4 {
            if self.due[lane] {
                let mut h = blake3::Hasher::new();
                self.lanes[lane].hash_charts(&mut h, &mut Vec::new());
                self.roots[lane] = *h.finalize().as_bytes();
                self.due[lane] = false;
            }
        }
        self.roots
    }

    /// Fiber at the deepest matching prefix on `lane`. Empty at every
    /// prefix means the chart is novel to the mesh. Ordered by the trie's
    /// walk: these identities are the vertices simplex keys are built from.
    pub fn fiber(&self, lane: u8, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        self.lanes[(lane as usize) & 3].lcp(&inv.0, false)
    }

    /// Cover multiplicity of a chart: the vertex count of the Čech nerve,
    /// not the word count. Read, never chosen; the room vector is untouched.
    pub fn multiplicity(&self, lane: u8, inv: &crate::transducer::stalk::Invariant) -> usize {
        self.fiber(lane, inv).len()
    }

    /// 47.3, 42.6: THE HYPHAE THAT MAY WITNESS A FORM — those covering its
    /// chart on a lane the form occupies, and never the home itself.
    ///
    /// 47.3 fixes the set: a witness "holds ruled sections over the chart,
    /// since replay needs them, so it is at nerve distance one from the
    /// home". 42.6 bounds the mesh cover's cost by "the number of covering
    /// hyphae", never by the number of nodes: "a demand goes to the hyphae
    /// covering its stratum; adding hyphae adds coverage and never
    /// coordination". A form offered to every peer indexes the fourth cover
    /// by the size of the mesh, which is the one thing that section forbids.
    ///
    /// An empty answer is a form no hypha covers: it has no eligible
    /// witness and waits. That is the chapter's answer, not a failure to
    /// reach anyone, and a home that offered to everyone could not tell the
    /// two apart.
    pub fn witnesses(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        mag: &[u64; 4],
        home: &[u8; 32],
    ) -> crate::network::nerve::Keyed<()> {
        let mut out: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for lane in crate::network::nerve::occupied_lanes(mag) {
            for v in self.fiber(lane, inv) {
                if v.as_slice() != home.as_slice() {
                    out.insert(&v, ());
                }
            }
        }
        out
    }

    /// Retract one hypha from one chart path.
    pub fn retract(
        &mut self,
        lane: u8,
        inv: &crate::transducer::stalk::Invariant,
        hypha: &[u8],
    ) {
        let lane = lane & 3;
        self.lanes[lane as usize].retract_path(&inv.0, hypha);
        self.refresh_root(lane);
    }

    /// Prefix split: one hypha leaves `old` and is admitted under `new`.
    /// Only the refined path is touched.
    pub fn rehome(
        &mut self,
        lane: u8,
        old: &crate::transducer::stalk::Invariant,
        new: &crate::transducer::stalk::Invariant,
        hypha: impl AsRef<[u8]>,
    ) {
        let hypha = hypha.as_ref();
        let lane = lane & 3;
        self.lanes[lane as usize].retract_path(&old.0, hypha);
        self.lanes[lane as usize].admit(&new.0, hypha);
        if let Some(charts) = self.held.get_mut(hypha) {
            for slot in charts.iter_mut() {
                if slot.0 == lane && slot.1 == *old {
                    slot.1 = new.clone();
                }
            }
        }
        self.refresh_root(lane);
    }

    pub fn charts(&self, lane: u8) -> Vec<crate::transducer::stalk::Invariant> {
        let mut out = Vec::new();
        self.lanes[(lane as usize) & 3].walk(&mut Vec::new(), &mut out);
        out
    }

    pub fn all_charts(&self) -> Vec<(u8, crate::transducer::stalk::Invariant)> {
        let mut out = Vec::new();
        for lane in 0..4u8 {
            for inv in self.charts(lane) {
                out.push((lane, inv));
            }
        }
        out
    }

    /// One lane's root, hashed if that lane has moved since the last read.
    pub fn root(&mut self, lane: u8) -> [u8; 32] {
        self.roots()[(lane as usize) & 3]
    }
}

/// Room on one lane: that lane's bonded μ times the host's remaining
/// strokes per unit bond on that lane, minus occupancy. Other lanes do
/// not enter. A zero bond yields zero room.
/// `None` where the cover a bond buys does not fit the carrier, or where
/// occupancy stands above it — a lane occupied beyond its cover is a
/// broken count, not a lane with no room (rule 11).
pub fn lane_cover_room(bond: u64, reserve: u64, occupancy: u64, spend: u64) -> Option<u64> {
    let per = reserve.checked_div(spend)?;
    bond.checked_mul(per)?.checked_sub(occupancy)
}

/// Occupied lanes of a magnitude — every lane the word moves, not the first.
pub fn occupied_lanes(mag: &[u64; 4]) -> Vec<u8> {
    let v: Vec<u8> = (0..4u8).filter(|&l| mag[l as usize] > 0).collect();
    if v.is_empty() {
        vec![0]
    } else {
        v
    }
}

fn addr_of(
    id: &[u8; 32],
    addrs: &Keyed<String>,
    self_id: Option<(&[u8; 32], &[u64; 4], &str)>,
) -> Option<String> {
    if let Some((sid, _, saddr)) = self_id {
        if id == sid {
            return Some(saddr.to_string());
        }
    }
    addrs.get(id).cloned()
}

/// HOW MUCH ROOM `a` HAS BEYOND `b`, per OCCUPIED lane: `a[l] − b[l]`,
/// signed, paired with the lane it is read on. Negative is a SHORTFALL —
/// `a` has less room there.
///
/// A lane the demand does not occupy is not read at all: comparing room on
/// a lane nothing is asked of is not a comparison, and dropping those lanes
/// from the reading says so more plainly than passing them through as zero.
fn room_reading(a: &[u64; 4], b: &[u64; 4], lanes: &[u8]) -> Vec<(u8, i64)> {
    lanes
        .iter()
        .map(|&l| (l, a[l as usize] as i64 - b[l as usize] as i64))
        .collect()
}

/// Home by incidence: intersection of cover fibers on every occupied lane,
/// then remaining-room dominance on those lanes (a hypha wins only when its
/// room is ≥ on every occupied lane and > on one), then identity when rooms
/// are equal or incomparable.
pub fn home(
    cover: &Cover,
    inv: &crate::transducer::stalk::Invariant,
    mag: &[u64; 4],
    rooms: &Keyed<[u64; 4]>,
    addrs: &Keyed<String>,
    self_id: Option<(&[u8; 32], &[u64; 4], &str)>,
) -> ([u8; 32], String) {
    let lanes = occupied_lanes(mag);
    let fits = |room: &[u64; 4]| lanes.iter().all(|&l| mag[l as usize] == 0 || room[l as usize] > 0);
    let room_of = |id: &[u8; 32]| -> Option<[u64; 4]> {
        if let Some((sid, sroom, _)) = self_id {
            if id == sid {
                return Some(*sroom);
            }
        }
        rooms.get(id).copied()
    };
    // Intersection of fibers across occupied lanes, as a trie union/meet.
    let mut fiber: Option<Keyed<()>> = None;
    for l in &lanes {
        let f = cover.fiber(*l, inv);
        let mut next = Keyed::new();
        for id in f {
            if fiber.as_ref().is_none_or(|acc| acc.contains(&id)) {
                next.insert(id, ());
            }
        }
        fiber = Some(next);
    }
    // A magnitude occupying no lane has no fiber to intersect, and the
    // empty set is what that reads as — not a fiber put there for it.
    let fiber = match fiber {
        Some(f) => f,
        None => Keyed::new(),
    };
    let mut best: Option<([u8; 32], String, [u64; 4])> = None;
    let consider = |best: &mut Option<([u8; 32], String, [u64; 4])>, id: [u8; 32], addr: String, room: [u64; 4]| {
        match best {
            None => *best = Some((id, addr, room)),
            Some((bid, _, broom)) => {
                // The room reading on the occupied lanes (16.11 step 5,
                // `room_reading`), read rather than asked: the candidate
                // takes the place when it falls short on no lane and has
                // more room on one. The shortfall is the reading, and this
                // is where it is spent; no lane is summed and no scalar
                // orders the two (law 5).
                let short = |r: &[(u8, i64)]| r.iter().any(|(_, d)| *d < 0);
                let spare = |r: &[(u8, i64)]| r.iter().any(|(_, d)| *d > 0);
                let mine = room_reading(&room, broom, &lanes);
                if !short(&mine) && spare(&mine) {
                    *best = Some((id, addr, room));
                } else {
                    // The incumbent may not dominate either -- equal room, or
                    // each short on a lane the other is not. Neither is
                    // worse, so identity decides, as a TIEBREAK and never as
                    // a measure.
                    let theirs = room_reading(broom, &room, &lanes);
                    if (short(&theirs) || !spare(&theirs)) && id < *bid {
                        *best = Some((id, addr, room));
                    }
                }
            }
        }
    };
    // THE INTERSECTION THIS FUNCTION JUST BUILT: the hyphae standing on
    // every occupied lane of this chart. It is a fiber by construction --
    // `cover.fiber(lane, inv)` scoped each one and the meet narrowed them.
    for (id, _) in fiber.scoped() {
        let Some(vk) = <[u8; 32]>::try_from(id.as_slice()).ok() else { continue };
        if let Some(room) = room_of(&vk) {
            if fits(&room) {
                if let Some(a) = addr_of(&vk, addrs, self_id) {
                    consider(&mut best, vk, a, room);
                }
            }
        }
    }
    if let Some((id, addr, _)) = best {
        return (id, addr);
    }
    // No covering hypha with room: novel (or the fiber is full), room then identity.
    if let Some((sid, sroom, saddr)) = self_id {
        if fits(sroom) {
            consider(&mut best, *sid, saddr.to_string(), *sroom);
        }
    }
    // A CHART NO COVER NAMES HAS NO COORDINATE TO DESCEND ON: that is what
    // novel means, and it is why this reading is the nerve's room table
    // whole. The carrier is the mesh's peers -- bounded by how many hyphae
    // this node knows, not by the corpus or the book -- so its whole
    // content is the candidate set and walking it is one stratum's reading.
    // The choice among them is still the room reading, never a scalar.
    for (id, room) in rooms.scoped() {
        if fits(room) {
            let Some(vk) = <[u8; 32]>::try_from(id.as_slice()).ok() else { continue };
            if let Some(a) = addrs.get(&vk) {
                consider(&mut best, vk, a.clone(), *room);
            }
        }
    }
    // The best hypha the cover named; failing that this node, which is a
    // candidate at its own room; failing that no home at all, which is the
    // empty identity and the empty address read out.
    match (best, self_id) {
        (Some((id, addr, _)), _) => (id, addr),
        (None, Some((id, _, a))) => (*id, a.to_string()),
        (None, None) => ([0u8; 32], String::new()),
    }
}
