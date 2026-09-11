// 26, 23.8 — ROOM: what a voice may draw against its bond, the shortfall a
// draw reads, the door's deferral, and the leases on T.
//
//! Room is the uttering voice's remaining bond, per lane; an unbonded voice
//! has room for one claim in flight per lane (MU-ECONOMY §3.1 as ruled).
//! No ×(reserve/spend): the genesis bond already covers a pair. Split from
//! `market.rs` (MU-STANDARDS 1.1, one organ per file): escrow and
//! settlement stay there.

use super::*;

impl Kernel {
    pub fn lane_room(&self) -> [usize; 4] {
        self.lane_room_of(&self.identity)
    }

    pub fn lane_room_of(&self, voice: &[u8; 32]) -> [usize; 4] {
        let occ = self.drawn.get(voice).copied().unwrap_or([0; 4]);
        if let Some(cap) = self.lane_cap {
            return std::array::from_fn(|l| cap[l].saturating_sub(occ[l] as usize));
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
        let credit = self.use_credit(voice);
        let bond = match self.treasury.bond_of(voice) {
            Some(b) => b,
            None if credit.iter().all(|&c| c == 0) => return [0; 4],
            None => [0; 4],
        };
        std::array::from_fn(|l| {
            bond[l].saturating_add(credit[l]).saturating_sub(occ[l]) as usize
        })
    }

    /// E2, 26 — WHAT THIS VOICE'S LANGUAGE EARNS IT. Lane-wise, over the
    /// forms this voice discovered, each form's μ⃗ once per DISTINCT
    /// citer of it. Two derivations in one sentence citing one form is
    /// one use of one verdict; two sentences citing it are two.
    ///
    /// Both readings are keyed and neither walks: `discovered_by` gives
    /// this voice's own addresses, and `cited_by` gives each address's
    /// citers. Computed at the read and stored nowhere (law 12), summed
    /// per lane and never across lanes (law 5).
    pub fn use_credit(&self, voice: &[u8; 32]) -> [u64; 4] {
        let Some(mine) = self.discovered_by.get(voice.as_slice()) else {
            return [0; 4];
        };
        let mut credit = [0u64; 4];
        for (addr, ()) in mine.fiber(crate::network::nerve::ROOT) {
            let Some(citers) = self.cited_by.get(&addr) else {
                continue;
            };
            let Some(form) = self.word_at(&addr) else {
                continue;
            };
            let mu = form.magnitude();
            for (_citer, ()) in citers.fiber(crate::network::nerve::ROOT) {
                for l in 0..4 {
                    credit[l] = credit[l].saturating_add(mu[l]);
                }
            }
        }
        credit
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
    pub fn adjoined(&self) -> u64 {
        let mut n = 0u64;
        for (addr, ()) in self.ruled_addrs.fiber(crate::network::nerve::ROOT) {
            let Some(e) = self.word_at(&addr) else {
                continue;
            };
            // The court's own ruled word, read off the element it holds
            // (L16): the tape is parsed once per word, not once per room
            // reading.
            let Some(f) = e.form() else {
                continue;
            };
            for v in &f.vertices {
                let crate::transducer::sentence::Vertex::Word { derivation, .. } = v else {
                    continue;
                };
                if !derivation.generators.is_empty() {
                    continue;
                }
                let spelled = crate::transducer::alphabet::factor_rotor(&derivation.rotor());
                if !matches!(spelled, Some(Ok(_))) {
                    n += 1;
                }
            }
        }
        n
    }

    /// Record a discovery on both readings at once, so `discoverers` and
    /// its inverse cannot disagree.
    pub(crate) fn note_discovered(&mut self, addr: &[u8], who: [u8; 32]) {
        self.discoverers.insert(addr, who);
        match self.discovered_by.get_mut(who.as_slice()) {
            Some(mine) => {
                mine.insert(addr, ());
            }
            None => {
                let mut mine = crate::network::nerve::Keyed::new();
                mine.insert(addr, ());
                self.discovered_by.insert(who, mine);
            }
        }
    }

    pub fn occupancy(&self) -> [u64; 4] {
        self.occupancy
    }

    /// Checkout a T-lease for a quoting voice. One lease per voice.
    pub fn grant_t_lease(&mut self, voice: [u8; 32]) {
        if self.t_leases.contains(&voice) {
            return;
        }
        self.t_leases.insert(voice, ());
        self.occupancy[2] = self.occupancy[2].saturating_add(1);
    }

    /// Return a T-lease when the voice's quoted work settles or expires.
    pub fn release_t_lease(&mut self, voice: [u8; 32]) {
        if self.t_leases.remove(&voice).is_some() {
            self.occupancy[2] = self.occupancy[2].saturating_sub(1);
        }
    }

    pub fn reserve(&self) -> [u64; 4] {
        self.physics.reserve
    }

    /// Test helper: remaining listing room, from each lane's own bond.
    pub fn dynamic_lane_capacity(&self) -> [usize; 4] {
        let room = self.lane_room();
        std::array::from_fn(|l| room[l].saturating_add(self.occupancy[l] as usize))
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
    pub(super) fn room_shortfall(&self, voice: &[u8; 32], draw: &[u64; 4]) -> [u64; 4] {
        let occ = self.drawn.get(voice).copied().unwrap_or([0; 4]);
        let owed = self.recall_short.get(voice).copied().unwrap_or([0; 4]);
        let bonded = self.lane_cap.is_some() || self.treasury.bond_of(voice).is_some();
        let room = self.lane_room_of(voice);
        std::array::from_fn(|l| match (draw[l], bonded) {
            (0, _) => 0,
            (d, _) if owed[l] > 0 && occ[l] > 0 => d,
            (d, true) => d.saturating_sub(room[l] as u64),
            (d, false) if occ[l] > 0 => d,
            (_, false) => 0,
        })
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
        let mut owed = self.recall_short.get(&voice).copied().unwrap_or([0; 4]);
        for l in crate::economy::lanes_short(short) {
            owed[l] = owed[l].saturating_add(short[l]);
        }
        self.recall_short.insert(voice, owed);
    }

    /// One line per short lane (34.2), read off the shortfall the caller
    /// holds: room is the named voice's, never the hypha's, and it is not
    /// measured again here.
    pub(super) fn log_room_refused(&self, voice: &[u8; 32], draw: &[u64; 4], short: &[u64; 4]) {
        for l in crate::economy::lanes_short(short) {
            eprintln!(
                "  court room refused voice {:02x}{:02x}… lane {l} draw {} room {}",
                voice[0],
                voice[1],
                draw[l],
                draw[l] - short[l]
            );
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
        std::array::from_fn(|l| match self.physics.reserve[l] == 0 {
            true => short[l],
            false => 0,
        })
    }
}
