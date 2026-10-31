/// Per-lane kinematic pressure, torsion, and the SYNTROPIC/ENTROPIC metabolism.
/// Arrays, never a summed scalar (#34). The metabolism is the governing physics
/// of the hypha↔client exchange: syntropic events (settlements — order built,
/// negentropy) and entropic events (pivots/backlog — disorder) net per lane into
/// the order flux that paces serving and demand cadence.
/// THE METABOLIC SCALE IS THE MACHINE'S, NOT A NARRATIVE (as ruled):
/// only the RATIOS are algebra, each read off the geometry — a full reserve
/// holds one expansion stroke per vertex of the sensor base's unit cell
/// (`2ⁿ`, every dimension an exponent); regeneration recovers half an
/// expansion, the rotor's double cover (the governor:
/// a depleted lane needs two strokes per install, so sustained germination
/// runs at the regeneration pace and bursts genuinely defer); the depletion
/// floor is one stroke. The SCALE calibrates ONCE at boot from the hardware
/// actually present — logical cores deepened by available memory — so the
/// same binary breathes correctly on a beta laptop and a compute node.
static CALIBRATED_RESERVE_MAX: std::sync::OnceLock<u64> = std::sync::OnceLock::new();

static CALIBRATED_HEADROOM: std::sync::OnceLock<Option<u64>> = std::sync::OnceLock::new();

/// Available memory in GiB at boot, or `None` where the host does not say:
/// the reserve and the book's ceiling are calibrated against ONE reading,
/// taken once. (Read per call, `book_ceiling` re-read `/proc/meminfo` on
/// every court stroke — 4.3 million read calls and 870 MB in eleven minutes
/// of one idle court on the live wire, 2026-09-29.)
fn mem_available_gib() -> Option<u64> {
    *CALIBRATED_HEADROOM.get_or_init(read_mem_available_gib)
}

/// The field is the second word of the `MemAvailable:` line, which is that
/// line's format and not a member picked by position (rule 15).
fn read_mem_available_gib() -> Option<u64> {
    let s = std::fs::read_to_string("/proc/meminfo").ok()?;
    let line = s.lines().find(|l| l.starts_with("MemAvailable:"))?;
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — wire tag offset: the value is the second word of the `MemAvailable:` line, which is that line's format"
    )]
    let kb: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb >> 20)
}

/// THE APERTURE — EVERY DIMENSION IS AN EXPONENT (founding, MU-RECURSION
/// §0.0): a full reserve holds one expansion stroke per vertex of the
/// sensor base's unit cell, `2ⁿ` for its `n` axes — the two grid axes and the
/// frame axis (`transducer::base`), so `2³`. It was declared `8` (the
/// "two-stroke ratio", 78.8): the same number, now read off the geometry it
/// always stood for. It is algebra, not a scale — the scale is the host's
/// (`reserve_max`).
pub const STROKES_PER_RESERVE: u64 = 1 << (crate::transducer::base::FRAME_AXIS + 1);

/// THE SMALLEST RESERVE THE RATIO STAYS EXACT ON: a spend of two, so the
/// regeneration's half of it is a whole unit. The two is the rotor's double
/// cover — `R` and `−R` are one motion, so a return is earned in halves —
/// never a number of its own.
const RESERVE_FLOOR: u64 = STROKES_PER_RESERVE * 2;

/// The calibrated reserve ceiling — measured, not declared: burst capacity
/// per logical core, deepened by each GiB of available memory, at least the
/// floor the ratio is exact on, and a whole number of strokes.
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
        // exact on every machine: the ratio's spends empty a full reserve,
        // always.
        (depth.max(RESERVE_FLOOR) / STROKES_PER_RESERVE) * STROKES_PER_RESERVE
    })
}

/// One entropic expansion's spend — the full reserve over the ratio, at
/// least two by the reserve's floor.
pub fn spend_unit() -> u64 {
    reserve_max() / STROKES_PER_RESERVE
}

/// One expansion, named for callers that must not mention the spend count.
pub fn expansion_unit() -> u64 {
    spend_unit()
}

#[expect(
    clippy::disallowed_types,
    reason = "law 9 — the host's stroke floor, calibrated once and told to the scheduler; no reading is taken from it"
)]
static CALIBRATED_STROKE_FLOOR: std::sync::OnceLock<std::time::Duration> = std::sync::OnceLock::new();

/// THE MINIMAL OPERATIONAL FLOOR OF ONE STROKE, as a duration the processor
/// understands (the stroke-duration ruling, 2026-09-11; PH1): the smallest
/// quantity that lets a scheduler place another thread — THIS HOST'S
/// scheduler tick, read from the kernel it runs (`CONFIG_HZ`), or, where the
/// host does not say, the least wait asked, which the kernel rounds to its
/// own timer (78.152: nothing timed). Read once, never a number of
/// milliseconds. 78.152: THIS IS THE ONE SCHEDULER FLOOR in the tree — the
/// bound of a park on no work (the court's exchange, the transport's poll,
/// a mind's idle stroke), told to the OS scheduler and read by no one; every
/// bound on work is counted in strokes or in final-point advances.
/// It is a FLOOR, not a pace: the pace is the stroke and the measured
/// transport (law 9), and it is told to the scheduler and never read as a
/// measurement of anything else.
#[expect(
    clippy::disallowed_types,
    reason = "law 9 — the parking floor is told to the scheduler in its own units; no reading is taken from it"
)]
pub fn stroke_floor() -> std::time::Duration {
    *CALIBRATED_STROKE_FLOOR.get_or_init(|| match scheduler_hz() {
        Some(hz) => std::time::Duration::from_secs(1) / hz,
        None => shortest_granted_wait(),
    })
}

/// The kernel's tick rate, from the build configuration it publishes beside
/// itself (`/boot/config-<release>`'s `CONFIG_HZ=`); `None` where the host
/// does not publish it. The line's value is that line's format.
fn scheduler_hz() -> Option<u32> {
    let release = std::fs::read_to_string("/proc/sys/kernel/osrelease").ok()?;
    let config = std::fs::read_to_string(format!("/boot/config-{}", release.trim())).ok()?;
    let hz: u32 = config.lines().find_map(|l| l.strip_prefix("CONFIG_HZ="))?.trim().parse().ok()?;
    (hz > 0).then_some(hz)
}

/// THE SHORTEST WAIT THIS KERNEL GRANTS: one nanosecond ASKED, which the
/// kernel rounds up to its own timer — the quantity a park needs — so no
/// wait is measured here at all (78.152: the `Instant` calibration that
/// timed the grant is retired; the scheduler applies its own floor).
#[expect(
    clippy::disallowed_types,
    reason = "law 9, 78.152 — the least wait asked of the scheduler, in its own units; nothing is measured and no reading is taken"
)]
fn shortest_granted_wait() -> std::time::Duration {
    std::time::Duration::from_nanos(1)
}

/// How long an idle stroke parks: the measured transport where one has
/// been measured — `rtt` in polls, the instrument's own resolution, each
/// poll a stroke of this host — and one stroke where none has, or where the
/// measured wait passes the duration's carrier. A same-stroke reply (`0`)
/// is still one stroke: a park is at least the stroke it parks.
#[expect(
    clippy::disallowed_types,
    reason = "law 9 — a park is told to the scheduler in its own units; the pace is strokes"
)]
pub fn idle_park(rtt: Option<u64>) -> std::time::Duration {
    let floor = stroke_floor();
    let strokes = match rtt {
        Some(r) => r.max(1),
        None => 1,
    };
    match u32::try_from(strokes).ok().and_then(|n| floor.checked_mul(n)) {
        Some(d) => d,
        None => floor,
    }
}

/// Strokes per reserve cycle — the aperture: the full reserve over one
/// spend, the vertices of the sensor base's unit cell (`STROKES_PER_RESERVE`),
/// in the `u32` its readers count circuits in.
pub fn aperture_cycle() -> u32 {
    1 << (crate::transducer::base::FRAME_AXIS + 1)
}

/// One syntropic regeneration — half an expansion, rounded up so exactly two
/// strokes re-earn one expansion (the two-stroke governor, scale-exact).
pub(super) fn regen_unit() -> u64 {
    spend_unit().div_ceil(2)
}

/// The depletion floor — one expansion stroke.
pub(super) fn expand_floor() -> u64 {
    spend_unit()
}
