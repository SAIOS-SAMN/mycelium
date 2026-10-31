// 26, 23.8 — ROOM: what a voice may draw against its bond, the shortfall a
// draw reads, the door's deferral, and the leases on T.
//
//! Room is the uttering voice's remaining bond, per lane; an unbonded voice
//! has room for one claim in flight per lane (MU-ECONOMY §3.1 as ruled).
//! No ×(reserve/spend): the genesis bond already covers a pair. Split from
//! `market.rs` (MU-STANDARDS 1.1, one organ per file): escrow and
//! settlement stay there.

use super::*;

/// STD1 — WHY A ROOM READ DID NOT ANSWER, by its own name: a `None` never
/// stands for two facts (78.26(4)). A lane drawn past its cover is a BROKEN
/// count; a cover or a credit whose sum passes its carrier is PAST CARRIER.
/// Each is counted on the kernel where the read answers `None`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoomRefused {
    Broken,
    PastCarrier,
}

impl Court {
    /// A room read's answer, its refusal counted by name.
    fn room_answer(&self, r: Result<[u64; 4], RoomRefused>) -> Option<[u64; 4]> {
        use std::sync::atomic::Ordering::Relaxed;
        match r {
            Ok(v) => Some(v),
            Err(RoomRefused::Broken) => {
                self.room_broken.fetch_add(1, Relaxed);
                None
            }
            Err(RoomRefused::PastCarrier) => {
                self.room_overflowed.fetch_add(1, Relaxed);
                None
            }
        }
    }

    /// `None` where this court's own room cannot be read: a lane occupied
    /// beyond its cover is a broken count, not a court with no room.
    /// Room, cover and occupancy are the mesh's stake per lane, `u64` end to
    /// end (STD1 3c-5): they were narrowed to `usize` at the read and widened
    /// back at the draw, a round trip that truncates on a narrower target.
    pub fn lane_room(&self) -> Option<[u64; 4]> {
        self.lane_room_of(&self.identity)
    }

    /// THE LANE'S COVER: what a voice staked plus what its language earned,
    /// before anything it drew is taken off. `lane_room_of` is this minus
    /// the occupancy, and a reader asking "how much does this lane HOLD"
    /// must not reconstruct it by adding the two back together — that is
    /// two readers of one fact (law 12), and it is how a saturation test
    /// came to compare a quantity with itself.
    pub fn lane_cover_of(&self, voice: &[u8; 32]) -> Option<[u64; 4]> {
        self.room_answer(self.cover_of(voice))
    }

    fn cover_of(&self, voice: &[u8; 32]) -> Result<[u64; 4], RoomRefused> {
        if let Some(cap) = self.lane_cap {
            return Ok(cap);
        }
        let credit = self.use_credit(voice).ok_or(RoomRefused::PastCarrier)?;
        let bond = match self.treasury.bond_of(voice) {
            Some(b) => b,
            None if credit.iter().all(|&c| c == 0) => return Ok([0; 4]),
            None => [0; 4],
        };
        crate::core::lanes::lanes_add(bond, credit).ok_or(RoomRefused::PastCarrier)
    }

    /// `None` on a lane occupied beyond its cover: that is a broken count,
    /// as a lane releasing more than it holds is, and zero room would be
    /// the invariant's failure read as a measurement (rule 11). `None` too
    /// where the cover passes its carrier, counted apart (STD1).
    pub fn lane_room_of(&self, voice: &[u8; 32]) -> Option<[u64; 4]> {
        self.room_answer(self.room_of(voice))
    }

    fn room_of(&self, voice: &[u8; 32]) -> Result<[u64; 4], RoomRefused> {
        // A voice that has drawn nothing occupies nothing: the reading of
        // absence on a keyed carrier, exactly zero per lane by definition.
        let occ = match self.drawn.get(voice) {
            Some(d) => *d,
            None => [0; 4],
        };
        if let Some(cap) = self.lane_cap {
            // A lane drawn PAST its cap is a broken count: the draw was
            // admitted against room that was not there.
            let mut free = [0u64; 4];
            for (l, f) in free.iter_mut().enumerate() {
                *f = cap[l].checked_sub(occ[l]).ok_or(RoomRefused::Broken)?;
            }
            return Ok(free);
        }
        // E2, 26 as ruled 2026-09-10: a voice whose forms other voices
        // derive through has more ROOM to speak, per lane. It is added to
        // the bond at the read and stored nowhere — nothing is minted for
        // a use, so no balance moves and no ledger entry is written; what
        // a use earns is standing to say more.
        //
        // A voice with NEITHER a bond nor a use has no room, as before. A
        // voice with a use and no bond has the room its language earned
        // it, which is the whole of what E2 says a use is worth: the bond
        // is what a voice staked, the credit is what its language did.
        let credit = self.use_credit(voice).ok_or(RoomRefused::PastCarrier)?;
        let bond = match self.treasury.bond_of(voice) {
            Some(b) => b,
            None if credit.iter().all(|&c| c == 0) => return Ok([0; 4]),
            None => [0; 4],
        };
        let cover = crate::core::lanes::lanes_add(bond, credit).ok_or(RoomRefused::PastCarrier)?;
        // A voice drawn BEYOND its cover is a broken count, as above.
        let mut free = [0u64; 4];
        for (l, f) in free.iter_mut().enumerate() {
            *f = cover[l].checked_sub(occ[l]).ok_or(RoomRefused::Broken)?;
        }
        Ok(free)
    }

    /// E2, 26 — WHAT THIS VOICE'S LANGUAGE EARNS IT. Lane-wise, over the
    /// forms this voice discovered, each form's μ⃗ once per DISTINCT
    /// citer of it. Two derivations in one sentence citing one form is
    /// one use of one verdict; two sentences citing it are two.
    ///
    /// `discovered_by` gives this voice's own addresses and `cited_by`
    /// gives each address's citers. Computed at the read and stored
    /// nowhere (law 12), summed per lane and never across lanes (law 5).
    ///
    /// THE CITERS ARE COUNTED, NOT READ ONE BY ONE. This nested
    /// `discovered.scoped()` × `citers.scoped()` — a product of two
    /// carriers that only ever grow, one with the corpus and one with the
    /// mesh — and the inner loop added the SAME `mu` once per citer, which
    /// is `mu × citers`, arithmetic. The comment above it read "neither
    /// walks"; both did. It is on the request path: `lane_room_of` is
    /// reached from ACT_PEERS, from every exchange cycle, from every draw
    /// and from every court stroke.
    ///
    /// What remains is one read of this voice's own discoveries. That is
    /// still a whole read of a monotone carrier on a hot path, and the shape
    /// that removes it is a memo sealed at `basis_gen`, as `closure` and
    /// `language` are — owed, and named here so it is not lost.
    ///
    /// `None` where the credit passes its carrier (STD1): the product
    /// `mu × uses` and the lane sum are both checked. The product was an
    /// unchecked `*`, which wraps in a release build — an invented credit.
    pub fn use_credit(&self, voice: &[u8; 32]) -> Option<[u64; 4]> {
        let Some(discovered) = self.discovered_by.get(voice.as_slice()) else {
            return Some([0; 4]);
        };
        let mut credit = [0u64; 4];
        for (addr, ()) in discovered.scoped() {
            let Some(citers) = self.cited_by.get(&addr) else {
                continue;
            };
            let Some(form) = self.word_at(&addr) else {
                continue;
            };
            let mu = form.magnitude();
            // `mu × uses` is the sum the inner loop was computing, term by
            // identical term, exact or refused.
            let uses = citers.len() as u64;
            let mut earned = [0u64; 4];
            for (e, m) in earned.iter_mut().zip(mu) {
                *e = m.checked_mul(uses)?;
            }
            credit = crate::core::lanes::lanes_add(credit, earned)?;
        }
        Some(credit)
    }

    /// 37.7, 45.9 — THE ADJOINED COUNT, READ FROM CONTENT. A generator
    /// the ENGINE opened is a ruled form whose word vertex cites nothing
    /// and carries a rotor outside the seventeen's span: it spells no
    /// letters, and A22 cannot spell its action either. Nothing is
    /// counted that was not derived, and no counter is kept — the number
    /// is read off the ruled cover at the read (law 12).
    ///
    /// This is the one number that bears on the architecture. It is zero
    /// until a court opens a letter (37.7 P2–P3).
    ///
    /// X3: THE RULED COVER IS THE OBJECT OF THIS READING. "Adjoined" is a
    /// property of the court's ruled words taken together, so every one of
    /// them is asked and no prefix would be honest — a subset would answer
    /// a different question. `no counter is kept` above is the same
    /// decision from the other side: the number is derived at the read, as
    /// law 12 asks, and that is what makes the whole read the reading.
    pub fn adjoined(&self) -> u64 {
        // The court's own ruled word, read off the word it HOLDS (L16): the
        // form is a field of the basis entry, decoded once when the court
        // installed it, and this read parses nothing. `word_at` falls
        // through to the cavity and the provisional on purpose, and neither
        // is a word this court installed. What counts as an opened generator
        // is `Held::generators_opened`'s, the one reader a mind shares (LC1):
        // "cites nothing" is read off the envelope (54.5 Road (d), B3),
        // never off the tape, which a rebuilt vertex always spells.
        self.ruled_addrs
            .scoped()
            .filter_map(|(addr, ())| self.held_word(&addr).map(|h| h.generators_opened()))
            .sum()
    }

    /// 50.8, 45.9 — THE SIMPLICES FILLED, READ FROM CONTENT. A filled
    /// simplex is an orbit whose holonomy has finite order and no residue,
    /// filed on the form's own torsion carrier as a `FiledOrbit` (15.5,
    /// A17); a ruled form carrying one has closed a simplex of its cover.
    ///
    /// Read off the ruled cover at the read and never counted into a
    /// carrier beside it (law 12), exactly as `adjoined` is. It is zero
    /// until a cover carries two pieces whose transition is a finite-order
    /// rotation with no placement or dilation left over.
    pub fn simplices_filled(&self) -> u64 {
        let mut n = 0u64;
        for (addr, ()) in self.ruled_addrs.scoped() {
            let Some(f) = self.held_word(&addr).and_then(|e| e.form()) else {
                continue;
            };
            // The form's torsion carrier holds one entry per filled simplex,
            // keyed by the holonomy's class. A form with none filled none.
            n += f.torsion.scoped().count() as u64;
        }
        n
    }

    /// Record a discovery on both readings at once, so `discoverers` and
    /// its inverse cannot disagree.
    pub(crate) fn note_discovered(&mut self, addr: &[u8], who: [u8; 32]) {
        self.discoverers.insert(addr, who);
        match self.discovered_by.get_mut(who.as_slice()) {
            Some(discovered) => {
                discovered.insert(addr, ());
            }
            None => {
                let mut discovered = crate::network::nerve::Keyed::new();
                discovered.insert(addr, ());
                self.discovered_by.insert(who, discovered);
            }
        }
    }

    /// Remove a discovery from both readings, pruning the voice entry if
    /// it holds no remaining addresses.
    pub(crate) fn forget_discoverer(&mut self, addr: &[u8]) {
        if let Some(who) = self.discoverers.remove(addr) {
            let empty = if let Some(discovered) = self.discovered_by.get_mut(who.as_slice()) {
                discovered.remove(addr);
                discovered.is_empty()
            } else {
                false
            };
            if empty {
                self.discovered_by.remove(who.as_slice());
            }
        }
    }

    /// Rebuild `discovered_by` from `discoverers` so the two stay synchronized.
    pub(crate) fn rebuild_discovered_by(&mut self) {
        let mut ndb: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>> =
            crate::network::nerve::Keyed::new();
        for (addr, who) in self.discoverers.scoped() {
            match ndb.get_mut(who.as_slice()) {
                Some(discovered) => {
                    discovered.insert(&addr, ());
                }
                None => {
                    let mut discovered = crate::network::nerve::Keyed::new();
                    discovered.insert(&addr, ());
                    ndb.insert(*who, discovered);
                }
            }
        }
        self.discovered_by = ndb;
    }

    pub fn occupancy(&self) -> [u64; 4] {
        self.occupancy
    }

    /// Checkout a T-lease for a quoting voice. One lease per voice.
    pub fn grant_t_lease(&mut self, voice: [u8; 32]) {
        if self.t_leases.contains(&voice) {
            return;
        }
        // STD1 — the T lane also carries listings' measures, so a lease is a
        // step of a sum and not only of a count: one past the lane's carrier
        // grants nothing, counted.
        let Some(next) = self.occupancy[2].checked_add(1) else {
            self.overflowed += 1;
            return;
        };
        self.t_leases.insert(voice, ());
        self.occupancy[2] = next;
    }

    /// Return a T-lease when the voice's quoted work settles or expires.
    pub fn release_t_lease(&mut self, voice: [u8; 32]) {
        if self.t_leases.remove(&voice).is_some() {
            // The lease was held, so the lane carries it; a lane at zero
            // holding a lease is a broken count, and it stays at zero
            // rather than wrapping (rule 11).
            if let Some(next) = self.occupancy[2].checked_sub(1) {
                self.occupancy[2] = next;
            }
        }
    }

    pub fn reserve(&self) -> [u64; 4] {
        self.physics.reserve.lanes()
    }

    /// Test helper: remaining listing room, from each lane's own bond.
    /// `None` where a lane's room plus its occupancy passes its carrier,
    /// counted as a room read past its carrier (STD1).
    pub fn dynamic_lane_capacity(&self) -> Option<[u64; 4]> {
        let room = self.lane_room()?;
        let mut cap = [0u64; 4];
        for (l, c) in cap.iter_mut().enumerate() {
            match room[l].checked_add(self.occupancy[l]) {
                Some(v) => *c = v,
                None => return self.room_answer(Err(RoomRefused::PastCarrier)),
            }
        }
        Some(cap)
    }

    /// THE SHORTFALL of a draw against the uttering voice's room, per lane:
    /// what the draw needs beyond the room, zero where the lane fits (S9.6,
    /// §2.3). A reading, never a verdict. The bit this replaced threw the
    /// lane away, and its one caller then read the room a second time to
    /// learn which lane had refused — the recorded proof that a `bool` over
    /// four lanes destroys what the caller needs.
    ///
    /// Room is what bond buys (MU-ECONOMY §3.1 as ruled 2026-09-09,
    /// S13.15): a bonded voice draws against its bond less what it has
    /// drawn; an unbonded voice has room for exactly one claim in flight
    /// per lane — a lane it already draws on is short by the whole draw,
    /// a lane it does not is not short at all, since speaking needs no
    /// bond. A lane the voice owes an unpaid recall on (`recall_short`)
    /// is short by the whole draw while its claim in flight on that lane
    /// stands. A test cap (`lane_cap`) reads as a bond.
    /// `None` where the voice's room cannot be read — a lane occupied
    /// beyond its cover — since a shortfall against a broken count is not
    /// a shortfall.
    pub(super) fn room_shortfall(&self, voice: &[u8; 32], draw: &[u64; 4]) -> Option<[u64; 4]> {
        // A voice with no draw and no unpaid recall occupies and owes
        // nothing: the reading of absence on a keyed carrier.
        let occ = match self.drawn.get(voice) {
            Some(d) => *d,
            None => [0; 4],
        };
        let owed = match self.recall_short.get(voice) {
            Some(o) => *o,
            None => [0; 4],
        };
        let bonded = self.lane_cap.is_some() || self.treasury.bond_of(voice).is_some();
        let room = self.lane_room_of(voice)?;
        // A bonded voice is short by `shortfall(draw, room)` — the reading
        // that already exists. The other arms are not shortfalls of a room
        // at all: an unpaid recall or an unbonded claim in flight closes
        // the lane, and the whole draw is what does not fit.
        let against = crate::economy::shortfall(draw, &room);
        Some(std::array::from_fn(|l| match (draw[l], bonded) {
            (0, _) => 0,
            (d, _) if owed[l] > 0 && occ[l] > 0 => d,
            (_, true) => against[l],
            (d, false) if occ[l] > 0 => d,
            (_, false) => 0,
        }))
    }

    /// A recall the voice could not pay (§3.1 as ruled, S13.15): the
    /// shortfall is recorded per lane; it closes the voice's room on each
    /// short lane while a claim of the voice is in flight there, and clears
    /// when that claim closes (`occupy`). Nothing is written for a recall
    /// that was paid in full.
    pub(super) fn record_recall_short(&mut self, voice: [u8; 32], short: &[u64; 4]) {
        if crate::economy::lanes_short(short).is_empty() {
            return;
        }
        // A voice with no recorded shortfall owes nothing: absence.
        let owed = match self.recall_short.get(&voice) {
            Some(o) => *o,
            None => [0; 4],
        };
        // A lane not short adds zero, so the whole vector is the step. One
        // past its carrier leaves what is owed where it stood, counted (STD1).
        match crate::core::lanes::lanes_add(owed, *short) {
            Some(next) => {
                self.recall_short.insert(voice, next);
            }
            None => self.overflowed += 1,
        }
    }

    /// THE CONGESTION DOOR (26, 48; S9.6b): the fee on a lane is the
    /// reserve deficit on that lane itself — no divisor, since none is
    /// measured — and the speaker pays what its balance holds. The reading
    /// returned is per lane: on a lane the mesh has spent to nothing, the
    /// fee that went unpaid; zero on every lane that was paid on, or that
    /// still holds reserve. A calm mesh keeps a free door and reads zero
    /// everywhere. The caller acts on the lanes it reads; the one `bool`
    /// this replaced said "somewhere" and named nothing.
    pub(super) fn door_deferral(&mut self, voice: [u8; 32]) -> [u64; 4] {
        let deficit = self.physics.reserve_deficit();
        if crate::economy::lanes_short(&deficit).is_empty() {
            return [0; 4];
        }
        let anchor = self.anchor();
        // TWO READINGS, BOTH READ. A lane defers when the mesh has spent
        // that lane to nothing AND the speaker did not cover the fee on it;
        // the amount deferred is what it did not cover. The second half was
        // `paid[l] == 0`, a zero test standing for the shortfall, which
        // cannot tell a lane half-paid from one wholly unpaid and reported
        // the whole deficit for a single unit short. `settle_door` now
        // returns that reading, so the door reads it instead of guessing it.
        let (_paid, short) = self.treasury.settle_door(voice, deficit, anchor);
        let held = self.physics.reserve.lanes();
        std::array::from_fn(|l| match held[l] == 0 {
            true => short[l],
            false => 0,
        })
    }
}
