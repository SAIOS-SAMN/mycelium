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
        let Some(bond) = self.treasury.bond_of(voice) else {
            return [0; 4];
        };
        std::array::from_fn(|l| bond[l].saturating_sub(occ[l]) as usize)
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
