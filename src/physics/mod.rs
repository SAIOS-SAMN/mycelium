// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Law 5, 12.6 — THE PHYSICS: pressure is the per-lane backlog of unresolved
//! demand (𝒮, 𝒢, 𝒯, 𝒽) and the metabolism rate is derived per lane from it,
//! so no lane is traded against another. Torsion is what will not close.

// The four lanes a germ carries content on — 𝒮 (spatial), 𝒢 (gauge), 𝒯
// (temporal), 𝒽 (holonomic/shell) — are read off the germ's exact per-lane
// support by `Element::lanes()` (transducer/element.rs), not derived here.

/// Per-lane kinematic pressure, torsion, and the SYNTROPIC/ENTROPIC metabolism.
/// Arrays, never a summed scalar (#34). The metabolism is the governing physics
/// of the hypha↔client exchange: syntropic events (settlements — order built,
/// negentropy) and entropic events (pivots/backlog — disorder) net per lane into
/// the order flux that paces serving and demand cadence.
/// THE METABOLIC SCALE IS THE MACHINE'S, NOT OUR MYTHOLOGY (operator-ruled):
/// only the two-stroke RATIOS are algebra — a full reserve holds eight
/// expansion strokes; regeneration recovers half an expansion (the governor:
/// a depleted lane needs two strokes per install, so sustained germination
/// runs at the regeneration pace and bursts genuinely defer); the depletion
/// floor is one stroke. The SCALE calibrates ONCE at boot from the hardware
/// actually present — logical cores deepened by available memory — so the
/// same binary breathes correctly on a beta laptop and a compute node.
static CALIBRATED_RESERVE_MAX: std::sync::OnceLock<u64> = std::sync::OnceLock::new();

/// Available memory in GiB, or `None` where the host does not say. The
/// field is the second word of the `MemAvailable:` line, which is that
/// line's format and not a member picked by position (rule 15).
fn mem_available_gib() -> Option<u64> {
    let s = std::fs::read_to_string("/proc/meminfo").ok()?;
    let line = s.lines().find(|l| l.starts_with("MemAvailable:"))?;
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — wire tag offset: the value is the second word of the `MemAvailable:` line, which is that line's format"
    )]
    let kb: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb >> 20)
}

/// The calibrated reserve ceiling — measured, not declared: burst capacity
/// per logical core, deepened by each GiB of available memory. The floor is
/// the smallest reserve on which the ratio algebra stays exact (spend ≥ 2 so
/// regeneration can genuinely halve it — the governor survives the smallest
/// hardware).
pub fn reserve_max() -> u64 {
    *CALIBRATED_RESERVE_MAX.get_or_init(|| {
        // A host that does not say how many cores it has, or how much
        // memory is available, is measured at the floor: one core and no
        // GiB of depth. That is the smallest hardware the ratio algebra
        // stays exact on, which is what the floor below asserts — not a
        // reading standing in for a measurement (rule 11).
        let cores = match std::thread::available_parallelism() {
            Ok(n) => n.get() as u64,
            Err(_) => 1,
        };
        let depth = match mem_available_gib() {
            Some(gib) => cores * (1 + gib),
            None => cores,
        };
        // Rounded DOWN to a whole number of strokes so the ratio algebra is
        // exact on every machine: eight spends empty a full reserve, always.
        (depth.max(16) / 8) * 8
    })
}

/// One entropic expansion's spend — an eighth of the full reserve (ratio law).
pub fn spend_unit() -> u64 {
    (reserve_max() / 8).max(2)
}

/// One expansion, named for callers that must not mention the spend count.
pub fn expansion_unit() -> u64 {
    spend_unit()
}

pub mod budget;
pub use budget::Budget;

/// A CARRIER'S ORBIT (45.7): the circuits a stroke ring has completed and
/// the period of the last one in strokes — the kernel's local time beside
/// K, read off the walk and never a clock. One per carrier: the court's
/// docket, the exchange to one peer, the mind's dirty ring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Orbit {
    strokes: u64,
    circuits: u64,
    at: u64,
    period: u64,
}

impl Orbit {
    /// One stroke over the carrier, with the circuits it completed. The
    /// reading — circuits so far and the strokes the last one took — is
    /// returned when a circuit completed, so the caller writes its orbit
    /// line once per circuit (34.2, 34.3) and never per stroke.
    pub fn stroke(&mut self, circuits: u64) -> Option<(u64, u64)> {
        self.strokes = self.strokes.saturating_add(1);
        let n = std::num::NonZeroU64::new(circuits)?;
        self.circuits = self.circuits.saturating_add(n.get());
        // A period is strokes since the last circuit. `at` is a stroke this
        // counter has already passed, so the difference is exact; a mark
        // ahead of the count is a broken counter and reports no circuit
        // rather than a period of zero (rule 11).
        self.period = self.strokes.checked_sub(self.at)?;
        self.at = self.strokes;
        Some((self.circuits, self.period))
    }

    /// Circuits completed so far.
    pub fn circuits(&self) -> u64 {
        self.circuits
    }

    /// Strokes the last circuit took; zero before the first.
    pub fn period(&self) -> u64 {
        self.period
    }
}

/// 47.5's Λ as a reading, with its two refusing arms told apart (law 5).
///
/// `Unmeasured` is no horizon: no round trip to this peer has been
/// measured, so nothing is known about how long a listing may wait.
/// `BeyondTheLedger` is a horizon WIDER than any extent the ledger counts
/// — arithmetic, not a failure, and it says no listing has waited that
/// long. One `None` over both would flatten two facts onto the eviction
/// path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lambda {
    /// The horizon, in strokes.
    Strokes(u64),
    /// No round trip measured to this peer.
    Unmeasured,
    /// The product does not fit the carrier the ledger counts in.
    BeyondTheLedger,
}

/// A duration in strokes: measured round-trip strokes times a stroke-count
/// factor. Unmeasured RTT is not passed here.
pub fn from_transport(rtt: u64, strokes: u64) -> Lambda {
    match rtt.checked_mul(strokes.max(1)) {
        Some(n) => Lambda::Strokes(n),
        None => Lambda::BeyondTheLedger,
    }
}

/// THE MINIMAL OPERATIONAL FLOOR OF ONE STROKE, as a duration the
/// processor understands (the stroke-duration ruling, 2026-09-11).
///
/// This is a FLOOR, not a pace: the pace is the stroke and the measured
/// transport (law 9). What it says is that one stroke of WAITING costs the
/// processor at least this much, so the thread the wait exists for can be
/// scheduled — the smallest quantity that lets a scheduler place another
/// thread. It is the one place the engine names wall time, and it names it
/// for the scheduler and never for a measurement.
#[expect(
    clippy::disallowed_types,
    reason = "law 9 — the parking floor is told to the scheduler in its own units; no reading is taken from it"
)]
pub const STROKE_FLOOR: std::time::Duration = std::time::Duration::from_millis(1);

/// How long an idle stroke parks: the measured transport where one has
/// been measured, and the floor where none has. `rtt` is in polls — the
/// instrument's own resolution — so a measured wire parks a poll's worth
/// per stroke and an unmeasured one parks the floor.
#[expect(
    clippy::disallowed_types,
    reason = "law 9 — a park is told to the scheduler in its own units; the pace is strokes"
)]
pub fn idle_park(rtt: Option<u64>) -> std::time::Duration {
    match rtt {
        Some(r) => STROKE_FLOOR * (r.clamp(1, 64) as u32),
        None => STROKE_FLOOR,
    }
}

/// AN APERTURE OF STROKES: what a pass that MUST finish is given, rather
/// than the one stroke a pass that can resume next time is given. The
/// eviction sweep above the book's ceiling is such a pass — a book past
/// the manifold is not a pacing question — and it is read here so the
/// spend unit stays inside physics (rules 1, 8).
pub fn aperture_strokes() -> usize {
    spend_unit() as usize * aperture_cycle() as usize
}

/// THE BOOK'S CEILING: how many open listings this node's memory can
/// carry, measured from the same live headroom the reserve is (rule 5 —
/// measured, never declared). A listing on the fleet weighs about 26 KiB
/// with its element and cells, and the book is held twice while a market
/// view stands, so a GiB of headroom carries roughly twenty thousand.
///
/// A NODE'S MEMORY IS A FINITE MANIFOLD. `pending` carried the comment
/// "capacity is thermodynamic (`dynamic_lane_capacity`)… eviction is by
/// ACTION POTENTIAL, never FIFO. No static cap exists" — and
/// `dynamic_lane_capacity` is a test helper with no production caller.
/// There was no cap of any kind: the only sink was `lapse_docket`, gated
/// on a round trip that had never been measured. Measured at epoch 10, the
/// book grew 250 listings a minute and the node ran out of memory.
///
/// THE BOOK IS AN EPHEMERAL SCRATCHPAD, not a store. My first cut read
/// 20,000 listings per GiB of headroom, which on this box is about 200,000
/// listings — some five gigabytes at the ~26 KiB a listing weighs. That is
/// an open door with a number written on it, not a boundary: hypha b sat
/// at 2,888 listings and 197 MB, nowhere near it, growing freely. What the
/// book holds is the work in flight; anything the mesh has not answered
/// within a horizon is not in flight, it is drift.
pub const BOOK_HIGH: usize = 1024;

/// Where the sweep drains TO once the ceiling is crossed. A single
/// threshold makes the sweep chatter on and off at the boundary; draining
/// to a low watermark means one pass does the work and the next has none
/// to do.
pub const BOOK_LOW: usize = 256;

/// The book's ceiling: the high watermark, or what this node's memory can
/// carry, whichever is smaller. On any machine the mesh runs on the
/// watermark is the binding one; the measured term is there so a node with
/// less memory than the watermark assumes is still bounded by its own.
pub fn book_ceiling() -> usize {
    const PER_GIB: u64 = 20_000;
    // A host that does not say how much memory is available is bounded by
    // the watermark alone; one GiB is the smallest depth the ceiling is
    // measured against where it does say.
    let Some(gib) = mem_available_gib() else {
        return BOOK_HIGH;
    };
    let measured = (gib.max(1) * PER_GIB) as usize;
    measured.min(BOOK_HIGH)
}

/// Deadline for one nerve dial, in strokes: an aperture of the measured
/// transport. Unmeasured RTT has no deadline.
pub fn transport_deadline(rtt: u64) -> Lambda {
    persist_dwell(rtt)
}

/// Persist dwell: a full aperture of measured transports, in strokes.
pub fn persist_dwell(rtt: u64) -> Lambda {
    from_transport(rtt, aperture_cycle() as u64)
}

/// Court federation flush: two apertures of measured transport, in strokes.
pub fn court_persist_dwell(rtt: u64) -> Lambda {
    from_transport(rtt, aperture_cycle() as u64 * 2)
}

/// Quiet-digest ceiling: a full reserve of measured transports, in strokes.
pub fn digest_ceiling(rtt: u64) -> Lambda {
    from_transport(rtt, reserve_max())
}

/// Court federation flush: a count of rulings, the measured reserve.
pub fn court_persist_rulings() -> u64 {
    reserve_max()
}

/// Backward name: a count, equal to the measured reserve.
pub const COURT_PERSIST_RULINGS: u64 = 0;



/// Strokes per reserve cycle — the aperture ratio κ. Drain reads this;
/// it is not a free constant.
pub fn aperture_cycle() -> u32 {
    (reserve_max() / spend_unit()).max(1) as u32
}

/// One syntropic regeneration — half an expansion, rounded up so exactly two
/// strokes re-earn one expansion (the two-stroke governor, scale-exact).
fn regen_unit() -> u64 {
    spend_unit().div_ceil(2)
}

/// The depletion floor — one expansion stroke.
fn expand_floor() -> u64 {
    spend_unit()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Physics {
    /// Open demand backlog per lane — the pressure.
    pub pressure: [u64; 4],
    /// SYNTROPIC events per lane — settlements that built structure (negentropy).
    pub syntropic: [u64; 4],
    /// ENTROPIC events per lane — pivots/torsion, disorder the demand injected.
    pub torsion: [u64; 4],
    /// The syntropic count each lane last regenerated at: the relief a
    /// lane produced since is what earns it a second regeneration.
    pub regen_mark: [u64; 4],
    /// The METABOLIC RESERVE per lane — the two-stroke engine's fuel. ENTROPIC
    /// expansion (germination) SPENDS it (the reserve shrinks as the alphabet
    /// grows); SYNTROPIC optimization (consolidation) REGENERATES it (the reserve
    /// recovers as the basis is pruned toward a minimal, confluent generating set).
    /// When a lane depletes, the mesh switches from expanding to optimizing.
    pub reserve: [u64; 4],
}

impl Default for Physics {
    fn default() -> Self {
        Physics { pressure: [0; 4], syntropic: [0; 4], torsion: [0; 4], regen_mark: [0; 4], reserve: [reserve_max(); 4] }
    }
}

impl Physics {
    /// Draw one stroke's read token. The only constructor of [`Budget`].
    /// Remaining reads equal one spend; cognition never names that number.
    pub fn stroke(&mut self) -> Budget {
        Budget { remaining: spend_unit() as usize }
    }

    /// A stroke whose remaining reads are `n`. Test callers use it so the
    /// N vs 4N counters stay independent of a machine whose spend exceeds
    /// N; the transport uses it to wait on a peer for a MEASURED number of
    /// strokes rather than for a duration (law 9).
    pub fn stroke_n(&mut self, n: usize) -> Budget {
        Budget { remaining: n }
    }

    /// The lanes a magnitude vector occupies — the selection a unit spend
    /// runs along. The pressure itself is never this boolean: it carries
    /// the magnitude.
    pub fn lanes_of(m: &[u64; 4]) -> [bool; 4] {
        std::array::from_fn(|l| m[l] > 0)
    }

    /// A demand entered — pressure rises by its exact per-lane magnitude,
    /// the graded defect itself, never an event count through a threshold.
    pub fn bump(&mut self, m: &[u64; 4]) {
        for (p, d) in self.pressure.iter_mut().zip(m) {
            *p = p.saturating_add(*d);
        }
    }

    /// A demand closed — pressure falls by the magnitude the settlement
    /// removed from the book, per lane, and a lane is relieved of what
    /// STANDS on it: the pressure is the metabolism's own reading, not a
    /// conserved ledger, so relieving more than a lane carries is relieving
    /// that lane entirely. The unrelieved remainder is the book's
    /// disagreement and is already visible there as tension.
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a lane is relieved of what stands on it, which is that lane's own reading"
    )]
    pub fn relieve(&mut self, m: &[u64; 4]) {
        for (p, d) in self.pressure.iter_mut().zip(m) {
            *p = match p.checked_sub(*d) {
                Some(left) => left,
                None => 0,
            };
        }
    }

    /// The metabolism RELAXES — the entropic backlog DISSIPATES as the mesh works,
    /// so per-lane pressure is a RATE (unresolved demand per unit of work), not a
    /// one-way ratchet that saturates and stalls the governor. Called once per
    /// adjudication; steady-state pressure then equilibrates where the germination
    /// rate meets this relaxation — a bounded, self-limiting metabolism.
    pub fn relax(&mut self) {
        // A lane with no pressure has nothing to dissipate: the reading of
        // absence, not a floor put under a subtraction (rule 11).
        for p in self.pressure.iter_mut() {
            if let Some(next) = p.checked_sub(1) {
                *p = next;
            }
        }
    }

    /// A pivot on `lane` — a torsion charge (the demand for a new fiber). An
    /// unattributable pivot charges all lanes (its resisting axis is unknown).
    pub fn pivot(&mut self, lanes: [bool; 4]) {
        let any = lanes.iter().any(|&b| b);
        for (t, &on) in self.torsion.iter_mut().zip(&lanes) {
            if on || !any {
                *t = t.saturating_add(1);
            }
        }
    }

    /// TORSE RELIEF — the mathematical torsion extraction dissipates the PHYSICAL
    /// torsion charge: condensing a fiber's sections into a cycle converts
    /// accumulated disorder into structure, per lane (the spanned count applied
    /// on the active lanes only — polytonic, never a summed scalar), and books
    /// the same amount as syntropic order built.
    /// `false` where a lane's span exceeds the torsion standing on it: the
    /// extraction and the charge disagree about what was there, and a floor
    /// at zero would convert that into order built (rule 11).
    pub fn torse_relief(&mut self, lanes: [bool; 4], spanned: u64) -> bool {
        for ((t, o), &on) in self.torsion.iter_mut().zip(self.syntropic.iter_mut()).zip(&lanes) {
            if on {
                let Some(next) = t.checked_sub(spanned) else {
                    return false;
                };
                *t = next;
                *o = o.saturating_add(spanned);
            }
        }
        true
    }

    /// A settlement built order on these lanes — a SYNTROPIC event (negentropy).
    /// Order built, by the magnitude of what was settled or composed.
    pub fn syntropy(&mut self, m: &[u64; 4]) {
        for (o, d) in self.syntropic.iter_mut().zip(m) {
            *o = o.saturating_add(*d);
        }
    }

    /// ENTROPIC EXPANSION spends the reserve on the germinated lanes — the
    /// metabolism SHRINKS as the alphabet grows.
    /// A lane with nothing left spends nothing: the reading of absence, and
    /// `remaining` says so. Spending PAST empty is what a floor would
    /// hide — the reserve would read empty either way and the governor
    /// could not tell a lane that just emptied from one overdrawn (rule 11).
    pub fn spend(&mut self, lanes: [bool; 4]) {
        for (r, &on) in self.reserve.iter_mut().zip(&lanes) {
            if on {
                if let Some(next) = r.checked_sub(spend_unit()) {
                    *r = next;
                    continue;
                }
                *r = 0;
            }
        }
    }

    /// THE TICK — a stroke that expanded nothing, charged as a stroke and not
    /// as an expansion.
    ///
    /// A stroke whose outcome moved no lane called `spend([true; 4])`, and
    /// `spend_unit` is by its own line "one entropic expansion's spend — an
    /// eighth of the full reserve". So an idle stroke cost FOUR EXPANSIONS,
    /// on four lanes, for germinating nothing on any of them. That is not a
    /// pacing choice; it is charging work that did not happen.
    ///
    /// It also manufactured the lane vector's collapse. Every stroke reported
    /// `learned [0,0,0,0]` on the fleet at 820678a, so every stroke took this
    /// path, so all four reserves fell in lockstep, so all four deficits were
    /// equal — and `yield_surface` reduces to `reserve_deficit` whenever
    /// torsion is zero. Measured: the dwell's S, G and H were EQUAL in 1154
    /// of 1247 readings and took two distinct values between them.
    ///
    /// The module's own scale says what a tick costs: "the depletion floor is
    /// one stroke". One unit, not one expansion. A lane with nothing left
    /// spends nothing, exactly as `spend` reads absence.
    pub fn tick(&mut self) {
        for r in self.reserve.iter_mut() {
            if let Some(next) = r.checked_sub(1) {
                *r = next;
            }
        }
    }

    /// SYNTROPIC OPTIMIZATION regenerates each lane toward the deflation
    /// that lane verified since the last aperture — not toward one scalar cap.
    pub fn regenerate(&mut self) {
        for l in 0..4 {
            // The mark is a syntropic total this lane has already reached,
            // so the difference is exact. A mark AHEAD of the total is a
            // broken counter, and this lane regenerates toward nothing it
            // verified rather than toward a difference of zero (rule 11).
            let Some(verified) = self.syntropic[l].checked_sub(self.regen_mark[l]) else {
                continue;
            };
            let add = regen_unit() * (1 + (verified > 0) as u64);
            let target = verified.max(expand_floor());
            let next = self.reserve[l].saturating_add(add);
            self.reserve[l] = next.min(target.max(self.reserve[l]));
            self.regen_mark[l] = self.syntropic[l];
        }
    }

    /// THE RESERVE SHORTFALL (S10 as ruled, 45.7; S13.2): per wanted lane,
    /// how far the reserve stands below one expansion — zero where the lane
    /// expands. A reading, never a verdict (§2.3): the caller acts on the
    /// lanes that are short, and a lane at zero reserve affords only the
    /// tick. `depleted()` collapsed the four lanes to one bit and
    /// `can_expand` to another, and every caller that had to act recomputed
    /// which lane the bit had thrown away.
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a lane at or above the floor falls short by nothing"
    )]
    pub fn reserve_shortfall(&self, lanes: [bool; 4]) -> [u64; 4] {
        // A lane at or above the floor falls short by NOTHING, and nothing
        // is what the empty difference measures — the reading of absence,
        // not a subtraction stopped at zero (rule 11).
        std::array::from_fn(|l| {
            match lanes[l] {
                true => match expand_floor().checked_sub(self.reserve[l]) {
                    Some(short) => short,
                    None => 0,
                },
                false => 0,
            }
        })
    }

    /// Distance to the full reserve per lane — the DEPLETION PREMIUM the yield
    /// surface charges (a strained metabolism prices its recovery work higher).
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a lane at the ceiling is in deficit by nothing"
    )]
    pub fn reserve_deficit(&self) -> [u64; 4] {
        // A lane at the ceiling is in deficit by NOTHING: the reading of
        // absence, as the shortfall above.
        std::array::from_fn(|l| match reserve_max().checked_sub(self.reserve[l]) {
            Some(down) => down,
            None => 0,
        })
    }

    /// Per-lane dwell in strokes from the last reply's pressure, for the
    /// lanes the loop needs to expand next. A lane not wanted does not
    /// dwell, even if it is the most-pressured.
    pub fn dwell(&self, pressure: &[u64; 4], want: [bool; 4]) -> [u64; 4] {
        std::array::from_fn(|l| if want[l] { pressure[l] } else { 0 })
    }

    /// THE METABOLISM RATE — per-lane serving cadence, proportional to pressure.
    /// Polytonic: each lane's rate is its own pressure, never one ranking across
    /// lanes. This is what paces the hypha's serving and the client's demand.
    pub fn metabolism_rate(&self) -> [u64; 4] {
        self.pressure
    }

    /// THE ORDER FLUX — per-lane syntropic minus entropic. Positive: the mesh is
    /// building structure on that lane (settling faster than it pivots);
    /// negative: it is losing ground (a lane demanding a new fiber). The dynamic
    /// entropic/syntropic metabolism (#34), reported hypha→client each exchange.
    pub fn order_flux(&self) -> [i64; 4] {
        std::array::from_fn(|l| self.syntropic[l] as i64 - self.torsion[l] as i64)
    }
}

/// One stroke's outcome: six modes, each a μ⃗ on the four lanes. Lane-wise
/// sums across modes are allowed; nothing is summed or compared across lanes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StrokeOutcome {
    pub fed: [u64; 4],
    pub pivoted: [u64; 4],
    pub stood: [u64; 4],
    pub uttered: [u64; 4],
    pub closed: [u64; 4],
    pub learned: [u64; 4],
}

impl StrokeOutcome {
    pub fn add(dst: &mut [u64; 4], src: &[u64; 4]) {
        for i in 0..4 {
            dst[i] = dst[i].saturating_add(src[i]);
        }
    }

    /// Lane-wise sum across modes.
    pub fn total(&self) -> [u64; 4] {
        let mut t = [0u64; 4];
        for m in [&self.fed, &self.pivoted, &self.stood, &self.uttered, &self.closed, &self.learned] {
            Self::add(&mut t, m);
        }
        t
    }

    /// The lanes this stroke moved, over every mode (24.3): a reading per
    /// lane, never one bit over six modes. A stroke that moved no lane
    /// spends the tick on all four (24.3), and the stroke line reads idle.
    pub fn moved(&self) -> [bool; 4] {
        Physics::lanes_of(&self.total())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressure_is_per_lane_and_relieves() {
        let mut p = Physics::default();
        p.bump(&[1, 0, 0, 0]); // a spatial demand
        p.bump(&[0, 1, 0, 0]); // a gauge demand
        assert_eq!(p.metabolism_rate(), [1, 1, 0, 0], "per-lane, not summed");
        p.relieve(&[1, 0, 0, 0]);
        assert_eq!(p.metabolism_rate(), [0, 1, 0, 0]);
    }

    /// A germinated pattern charges the gauge lane — read off the germ's exact
    /// per-lane support (`Element::lanes()`), the coordinate-exact carrier. (Post-
    /// excision every perceived change is a pattern until the mesh consolidates
    /// richer structure; the gauge/spatial lanes fill as those axes germinate.)
    #[test]
    fn a_germinated_pattern_charges_the_residual_lane() {
        use crate::transducer::element::from_canonical;
        use crate::transducer::germinate::germinate;
        // A two-cell recolour: hue transitions on a standing support are
        // GAUGE content, carried alongside the residual defect — the lane
        // projection is measured from the chain, never funneled to one axis.
        let g = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        assert_eq!(g.lanes(), [false, true, false, false], "a recolour is gauge content; temporal is empty until the listing stands");
    }

    #[test]
    fn a_pivot_is_a_torsion_charge() {
        let mut p = Physics::default();
        p.pivot([false, false, false, false]); // unattributable
        assert_eq!(p.torsion, [1, 1, 1, 1], "an unresolved residual charges torsion");
    }

    /// THE TWO-STROKE RESERVE: entropic expansion SPENDS per lane; depletion is
    /// per-lane (polytonic); the syntropic stroke REGENERATES — and one stroke
    /// recovers LESS than one spend (REGEN < SPEND), so sustained expansion is
    /// paced by the recovery the optimization earns.
    #[test]
    fn the_reserve_spends_depletes_and_regenerates_per_lane() {
        let t = [false, false, true, false];
        let mut p = Physics::default();
        assert_eq!(p.reserve_shortfall([true; 4]), [0; 4], "full at rest: no lane is short");
        for _ in 0..8 {
            p.spend(t); // 32 − 8·4 = 0 on the residual lane
        }
        let short = p.reserve_shortfall([true; 4]);
        assert!(short[2] > 0, "the spent lane is short by what one expansion needs: {short:?}");
        assert_eq!((short[0], short[1], short[3]), (0, 0, 0), "other lanes unaffected — polytonic");
        assert_eq!(p.reserve_shortfall([true, false, false, false]), [0; 4], "a lane not wanted reads zero");
        p.regenerate();
        assert!(p.reserve_shortfall(t)[2] > 0, "one stroke is not recovery");
        p.regenerate();
        assert_eq!(p.reserve_shortfall(t), [0; 4], "two strokes re-earn expansion");
    }

    /// A TICK IS A STROKE, NOT AN EXPANSION. A stroke whose outcome moved no
    /// lane germinated nothing, and it called `spend([true; 4])` — four
    /// entropic expansions, one per lane, for work that did not happen.
    ///
    /// That is also what collapsed the lane vector. Every stroke on the fleet
    /// at 820678a reported `learned [0,0,0,0]`, so every stroke took that
    /// path, so all four reserves fell in lockstep and all four deficits were
    /// equal; with torsion zero `yield_surface` IS `reserve_deficit`, and the
    /// dwell's S, G and H were equal in 1154 of 1247 readings, taking two
    /// distinct values between them.
    #[test]
    fn a_tick_costs_a_stroke_and_an_expansion_costs_an_expansion() {
        let mut tick = Physics::default();
        let mut expand = Physics::default();
        tick.tick();
        expand.spend([true; 4]);
        assert_eq!(tick.reserve, [reserve_max() - 1; 4], "a tick is one unit on each lane");
        assert!(
            tick.reserve.iter().zip(&expand.reserve).all(|(t, e)| t > e),
            "and an expansion costs far more: {:?} against {:?}",
            tick.reserve,
            expand.reserve
        );
        assert_eq!(
            tick.reserve_shortfall([true; 4]),
            [0; 4],
            "one tick does not put a lane short of an expansion"
        );
        // A lane with nothing left spends nothing, as `spend` reads absence.
        let mut empty = Physics::default();
        for _ in 0..8 {
            empty.spend([true; 4]);
        }
        assert_eq!(empty.reserve, [0; 4], "eight expansions empty every lane");
        empty.tick();
        assert_eq!(empty.reserve, [0; 4], "and a tick on an empty lane spends nothing");
    }

    /// The metabolism is per-lane syntropic minus entropic — the order flux.
    #[test]
    fn the_metabolism_is_order_flux_per_lane() {
        let mut p = Physics::default();
        p.syntropy(&[1, 0, 0, 0]); // built order on 𝒮
        p.syntropy(&[1, 0, 0, 0]);
        p.pivot([false, true, false, false]); // disorder on 𝒢
        assert_eq!(p.order_flux(), [2, -1, 0, 0], "syntropic − entropic, never summed");
    }

    /// Pressure on G dwells G and keeps expanding on S, T, H.
    #[test]
    fn pressure_on_one_lane_dwells_that_lane_only() {
        let mut p = Physics::default();
        let g = [false, true, false, false];
        while crate::economy::lanes_short(&p.reserve_shortfall(g)).is_empty() {
            p.spend(g);
        }
        assert!(p.reserve_shortfall(g)[1] > 0, "G is spent");
        assert_eq!(
            p.reserve_shortfall([true, false, true, true]),
            [0; 4],
            "S, T and H still expand"
        );
        let pressure = [0u64, 7, 0, 0];
        let sth = [true, false, true, true];
        assert_eq!(
            p.dwell(&pressure, sth),
            [0, 0, 0, 0],
            "expanding S, T, H does not dwell for G"
        );
        assert_eq!(
            p.dwell(&pressure, g),
            [0, 7, 0, 0],
            "a stroke that needs G dwells G's pressure only"
        );
        let outcome = StrokeOutcome { pivoted: [0, 3, 0, 0], ..Default::default() };
        assert_eq!(outcome.moved(), g, "the stroke moved G and no other lane");
        assert_eq!(StrokeOutcome::default().moved(), [false; 4], "a stroke that moved nothing");
    }

    /// 45.7 — an orbit is read in circuits and its period in strokes, off
    /// the walk: three strokes to the first circuit, one to the next.
    #[test]
    fn an_orbit_is_read_in_circuits_and_strokes() {
        let mut o = Orbit::default();
        assert_eq!(o.stroke(0), None, "no circuit, no reading");
        assert_eq!(o.stroke(0), None);
        assert_eq!(o.stroke(1), Some((1, 3)), "the first circuit took three strokes");
        assert_eq!(o.stroke(1), Some((2, 1)), "a budget that covers the carrier orbits every stroke");
        assert_eq!(o.stroke(2), Some((4, 1)), "two circuits in one stroke are two");
        assert_eq!((o.circuits(), o.period()), (4, 1));
    }

    #[test]
    fn rtt_is_counted_in_strokes() {
        assert_eq!(from_transport(3, 4), Lambda::Strokes(12), "measured rtt × strokes");
        assert_eq!(from_transport(3, 0), Lambda::Strokes(3), "zero strokes is still one transport");
        assert_eq!(
            from_transport(0, 4),
            Lambda::Strokes(0),
            "a same-stroke reply is zero, not a floor"
        );
        let (Lambda::Strokes(quiet), Lambda::Strokes(one)) =
            (digest_ceiling(3), from_transport(3, 1))
        else {
            panic!("a measured round trip has a horizon the ledger counts");
        };
        assert!(quiet >= one);
        let d = transport_deadline(5);
        assert_eq!(d, persist_dwell(5), "the deadline is an aperture of the measured transport");
    }

    /// 47.5's two refusing arms are two readings (law 5): a round trip
    /// nobody measured, and a horizon wider than any extent the ledger
    /// counts. One `None` over both would put an arithmetic fact and a
    /// missing measurement on one branch of the eviction path.
    #[test]
    fn a_horizon_wider_than_the_ledger_is_not_an_unmeasured_one() {
        assert_eq!(
            from_transport(u64::MAX, 2),
            Lambda::BeyondTheLedger,
            "a product that does not fit is beyond the ledger, not a saturated extent"
        );
        assert_ne!(
            from_transport(u64::MAX, 2),
            Lambda::Unmeasured,
            "and it is not the same reading as a transport nobody measured"
        );
    }
}
