// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! COMPUTATIONAL PHYSICS — polytonic kinematic pressure + torsion, per lane,
//! never a monotone scalar (the #34 law). Pressure is the per-lane backlog of
//! unresolved demand (𝒮 spatial, 𝒢 gauge, 𝒯 temporal, 𝒽 shell); the metabolism
//! rate is derived per-lane from it, so no lane is ever traded against another.
//! Torsion is the obstruction reading: a demand that will not close is a PIVOT —
//! a torsion charge on the lane it resisted, the demand for a new fiber.

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

fn mem_available_gib() -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("MemAvailable:"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|kb| kb.parse::<u64>().ok()))
        })
        .map(|kb| kb >> 20)
        .unwrap_or(1)
}

/// The calibrated reserve ceiling — measured, not declared: burst capacity
/// per logical core, deepened by each GiB of available memory. The floor is
/// the smallest reserve on which the ratio algebra stays exact (spend ≥ 2 so
/// regeneration can genuinely halve it — the governor survives the smallest
/// hardware).
pub fn reserve_max() -> u64 {
    *CALIBRATED_RESERVE_MAX.get_or_init(|| {
        let cores = std::thread::available_parallelism().map(|n| n.get() as u64).unwrap_or(1);
        // Rounded DOWN to a whole number of strokes so the ratio algebra is
        // exact on every machine: eight spends empty a full reserve, always.
        ((cores * (1 + mem_available_gib())).max(16) / 8) * 8
    })
}

/// One entropic expansion's spend — an eighth of the full reserve (ratio law).
pub fn spend_unit() -> u64 {
    (reserve_max() / 8).max(2)
}

/// Linear token for one stroke's keyed reads. Non-Copy, non-Clone: a
/// function that wants more than a fiber must take a Budget in its
/// signature. Constructed only by [`Physics::stroke`].
#[derive(Debug)]
pub struct Budget {
    remaining: usize,
}

impl Budget {
    /// Reads still granted on this token.
    pub fn remaining(&self) -> usize {
        self.remaining
    }

    /// Spend one read. False when the token is exhausted.
    pub fn spend(&mut self) -> bool {
        if self.remaining == 0 {
            return false;
        }
        self.remaining -= 1;
        true
    }
}

/// A duration derived from a measured transport time and a stroke-count
/// factor. Reserve and spend are counts, never milliseconds. An unmeasured
/// RTT uses a one-millisecond tick so the first dial can complete; every
/// later duration is the measurement times the stroke count.
pub fn from_transport(rtt: std::time::Duration, strokes: u64) -> std::time::Duration {
    let n = strokes.max(1) as u32;
    if rtt.is_zero() {
        std::time::Duration::from_millis(1).saturating_mul(n)
    } else {
        rtt.saturating_mul(n)
    }
}

/// Deadline for one nerve dial: the last RTT times the aperture ratio,
/// never shorter than one millisecond so a TCP handshake can finish.
pub fn transport_deadline(rtt: std::time::Duration) -> std::time::Duration {
    from_transport(rtt, (reserve_max() / spend_unit()).max(1)).max(std::time::Duration::from_millis(1))
}

/// Persist dwell: a full aperture of measured transports, not a wall clock.
pub fn persist_dwell(rtt: std::time::Duration) -> std::time::Duration {
    from_transport(rtt, aperture_cycle() as u64)
}

/// Court federation flush: two apertures of measured transport.
pub fn court_persist_dwell(rtt: std::time::Duration) -> std::time::Duration {
    from_transport(rtt, aperture_cycle() as u64 * 2)
}

/// Quiet-digest ceiling: a full reserve of measured transports.
pub fn digest_ceiling(rtt: std::time::Duration) -> std::time::Duration {
    from_transport(rtt, reserve_max())
}

/// Court federation flush: a count of rulings, the measured reserve.
pub fn court_persist_rulings() -> u64 {
    reserve_max()
}

/// Backward name: a count, equal to the measured reserve.
pub const COURT_PERSIST_RULINGS: u64 = 0;

/// Backward names: callers must pass a measured RTT through persist_dwell.
pub const PERSIST_DWELL: std::time::Duration = std::time::Duration::from_secs(30);
pub const COURT_PERSIST_DWELL: std::time::Duration = std::time::Duration::from_secs(60);

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

    /// Test-only: a stroke whose remaining reads are `n`, so the N vs 4N
    /// counters stay independent of a machine whose spend exceeds N.
    #[cfg(test)]
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
        for l in 0..4 {
            self.pressure[l] = self.pressure[l].saturating_add(m[l]);
        }
    }

    /// A demand closed — pressure falls by the magnitude the settlement
    /// removed from the book.
    pub fn relieve(&mut self, m: &[u64; 4]) {
        for l in 0..4 {
            self.pressure[l] = self.pressure[l].saturating_sub(m[l]);
        }
    }

    /// The metabolism RELAXES — the entropic backlog DISSIPATES as the mesh works,
    /// so per-lane pressure is a RATE (unresolved demand per unit of work), not a
    /// one-way ratchet that saturates and stalls the governor. Called once per
    /// adjudication; steady-state pressure then equilibrates where the germination
    /// rate meets this relaxation — a bounded, self-limiting metabolism.
    pub fn relax(&mut self) {
        for l in 0..4 {
            self.pressure[l] = self.pressure[l].saturating_sub(1);
        }
    }

    /// A pivot on `lane` — a torsion charge (the demand for a new fiber). An
    /// unattributable pivot charges all lanes (its resisting axis is unknown).
    pub fn pivot(&mut self, lanes: [bool; 4]) {
        let any = lanes.iter().any(|&b| b);
        for l in 0..4 {
            if lanes[l] || !any {
                self.torsion[l] = self.torsion[l].saturating_add(1);
            }
        }
    }

    /// TORSE RELIEF — the mathematical torsion extraction dissipates the PHYSICAL
    /// torsion charge: condensing a fiber's sections into a cycle converts
    /// accumulated disorder into structure, per lane (the spanned count applied
    /// on the active lanes only — polytonic, never a summed scalar), and books
    /// the same amount as syntropic order built.
    pub fn torse_relief(&mut self, lanes: [bool; 4], spanned: u64) {
        for l in 0..4 {
            if lanes[l] {
                self.torsion[l] = self.torsion[l].saturating_sub(spanned);
                self.syntropic[l] = self.syntropic[l].saturating_add(spanned);
            }
        }
    }

    /// A settlement built order on these lanes — a SYNTROPIC event (negentropy).
    /// Order built, by the magnitude of what was settled or composed.
    pub fn syntropy(&mut self, m: &[u64; 4]) {
        for l in 0..4 {
            self.syntropic[l] = self.syntropic[l].saturating_add(m[l]);
        }
    }

    /// ENTROPIC EXPANSION spends the reserve on the germinated lanes — the
    /// metabolism SHRINKS as the alphabet grows.
    pub fn spend(&mut self, lanes: [bool; 4]) {
        for l in 0..4 {
            if lanes[l] {
                self.reserve[l] = self.reserve[l].saturating_sub(spend_unit());
            }
        }
    }

    /// SYNTROPIC OPTIMIZATION regenerates the reserve on every lane — the
    /// metabolism RECOVERS as the basis consolidates and rests (capped at full).
    pub fn regenerate(&mut self) {
        for l in 0..4 {
            // Every lane recovers one regeneration; a lane on which order
            // was built since the last recovery earns a second — the
            // reserve differentiates by the relief each lane produced.
            let built = self.syntropic[l] > self.regen_mark[l];
            let units = 1 + built as u64;
            self.reserve[l] = (self.reserve[l] + units * regen_unit()).min(reserve_max());
            self.regen_mark[l] = self.syntropic[l];
        }
    }

    /// Is any lane DEPLETED — the mesh in the SYNTROPIC (optimize + regenerate)
    /// phase, rather than the entropic (expand) one?
    pub fn depleted(&self) -> bool {
        self.reserve.iter().any(|&r| r < expand_floor())
    }

    /// Distance to the full reserve per lane — the DEPLETION PREMIUM the yield
    /// surface charges (a strained metabolism prices its recovery work higher).
    pub fn reserve_deficit(&self) -> [u64; 4] {
        std::array::from_fn(|l| reserve_max().saturating_sub(self.reserve[l]))
    }

    /// May the mesh EXPAND (germinate) on these lanes — is the reserve above the
    /// floor there (the entropic phase)?
    pub fn can_expand(&self, lanes: [bool; 4]) -> bool {
        (0..4).all(|l| !lanes[l] || self.reserve[l] >= expand_floor())
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
        use crate::transducer::element::from_residual;
        use crate::transducer::germinate::germinate;
        // A two-cell recolour: hue transitions on a standing support are
        // GAUGE content, carried alongside the residual defect — the lane
        // projection is measured from the chain, never funneled to one axis.
        let g = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
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
        let mut p = Physics::default();
        assert!(p.can_expand([true; 4]), "full at rest");
        for _ in 0..8 {
            p.spend([false, false, true, false]); // 32 − 8·4 = 0 on the residual lane
        }
        assert!(p.depleted());
        assert!(!p.can_expand([false, false, true, false]), "the spent lane is barred");
        assert!(p.can_expand([true, false, false, false]), "other lanes unaffected — polytonic");
        p.regenerate();
        assert!(!p.can_expand([false, false, true, false]), "one stroke is not recovery");
        p.regenerate();
        assert!(p.can_expand([false, false, true, false]), "two strokes re-earn expansion");
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

    #[test]
    fn durations_are_stroke_counts_of_a_measured_transport() {
        let rtt = std::time::Duration::from_millis(3);
        assert_eq!(from_transport(rtt, 4), std::time::Duration::from_millis(12));
        assert_eq!(from_transport(rtt, 0), rtt, "zero strokes is still one transport");
        let quiet = digest_ceiling(rtt);
        assert!(quiet >= from_transport(rtt, 1));
        assert!(transport_deadline(std::time::Duration::ZERO) >= std::time::Duration::from_millis(1));
    }
}
