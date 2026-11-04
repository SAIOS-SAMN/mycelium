/// Per-lane kinematic pressure, torsion, and the SYNTROPIC/ENTROPIC metabolism.
/// Arrays, never a summed scalar (#34). The metabolism is the governing physics
/// of the hypha↔client exchange: syntropic events (settlements — order built,
/// negentropy) and entropic events (pivots/backlog — disorder) net per lane into
/// the order flux that paces serving and demand cadence.
/// THE METABOLIC SCALE IS THE PROCESS'S, NOT A NARRATIVE (as ruled):
/// only the RATIOS are algebra, each read off the geometry — a full pool
/// holds one expansion stroke per vertex of the sensor base's unit cell
/// (`2ⁿ`, every dimension an exponent); regeneration recovers half an
/// expansion, the rotor's double cover (the governor:
/// a depleted lane needs two strokes per install, so sustained germination
/// runs at the regeneration pace and bursts genuinely defer); the depletion
/// floor is one stroke. The SCALE is the process's CAPACITY, sealed ONCE at
/// boot (MU-DERIVATIONS D2.5, O1 ruled 2026-11-04): one calibration stroke
/// measures the bytes one read retains, `b`, and the capacity is the reads
/// the process's own memory `M_proc` holds at `b` each, `C = ⌊M_proc / b⌋`.
/// Read once at the seal: per call, `/proc/meminfo` was re-read on every
/// court stroke (mycelium#14).
static SEALED: std::sync::OnceLock<Calibration> = std::sync::OnceLock::new();

use super::memory::{memory_of, read_cgroup_limit, read_mem_available_kib, Memory};

/// THE BYTES ONE READ RETAINS (D2.5, `b ∈ ℤ>0`): what a stroke left held,
/// over the reads it spent, `b = ⌈ retained / reads ⌉`. A stroke that spent
/// no read, or held nothing at its end, measured no read's size: refused
/// with the reading that caused it.
pub fn footprint_of(retained: i128, reads: u64) -> Result<std::num::NonZeroU64, Uncalibrated> {
    let unread = Uncalibrated::Footprint(Unstroked::Retained { retained, reads });
    let (Ok(held), Some(spent)) = (u64::try_from(retained), std::num::NonZeroU64::new(reads)) else {
        return Err(unread);
    };
    std::num::NonZeroU64::new(held.div_ceil(spent.get())).ok_or(unread)
}

/// THE PROCESS'S CAPACITY, SEALED (D2.5 (a)): the reads `M_proc` holds at
/// the measured bytes per read, `C = ⌊M_proc / b⌋`, with the minimal stroke
/// it must fit. Derived once, at the seal, and held with its inputs
/// (MU-STANDARDS 2.12): the metabolism reads the held `capacity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Calibration {
    memory: Memory,
    footprint: std::num::NonZeroU64,
    minimal: std::num::NonZeroU64,
    capacity: std::num::NonZeroU64,
}

impl Calibration {
    /// THE CALIBRATION OF A PROCESS holding `memory` whose reads retain
    /// `footprint` bytes each, and whose minimal stroke reads `minimal`:
    /// `C = ⌊M_proc / b⌋`. Refused `Uncalibrated::Unfit` where one stroke's
    /// spend of `C` does not cover the minimal stroke (the process cannot
    /// run the least work there is), so `C > 0` on every calibration.
    pub fn of(memory: Memory, footprint: std::num::NonZeroU64, minimal: std::num::NonZeroU64) -> Result<Calibration, Uncalibrated> {
        let reads = memory.bytes().get() / footprint.get();
        let unfit = Uncalibrated::Unfit { capacity: reads, minimal: minimal.get() };
        let capacity = std::num::NonZeroU64::new(reads).ok_or(unfit)?;
        if capacity.get() / STROKES_PER_RESERVE < minimal.get() {
            return Err(unfit);
        }
        Ok(Calibration { memory, footprint, minimal, capacity })
    }

    /// `M_proc`, the process's memory read at the seal, with its source.
    pub fn memory(&self) -> Memory {
        self.memory
    }

    /// `b`, the bytes one read retained on the calibration stroke.
    pub fn footprint(&self) -> std::num::NonZeroU64 {
        self.footprint
    }

    /// The minimal stroke's reads: the founding motions' modes' need.
    pub fn minimal(&self) -> std::num::NonZeroU64 {
        self.minimal
    }

    /// `C`, the process's capacity in reads.
    pub fn capacity(&self) -> std::num::NonZeroU64 {
        self.capacity
    }
}

/// THE SEAL'S WITNESS: the calibration the boot stroke sealed, constructed
/// by [`calibrate`] alone. What serves acts is built from it (the server's
/// nerve), so no act is served on an unsealed capacity. Outside
/// `physics::calibration` a seal cannot be written from a calibration:
///
/// ```compile_fail
/// let m = mycelium::physics::Memory::of(std::num::NonZeroU64::MAX, mycelium::physics::MemorySource::CgroupMax);
/// let unsealed = mycelium::physics::Calibration::of(m, std::num::NonZeroU64::MIN, std::num::NonZeroU64::MIN).ok();
/// let _ = mycelium::physics::Sealed { calibration: unsealed };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sealed {
    calibration: Calibration,
}

impl Sealed {
    /// The sealed calibration.
    pub fn calibration(&self) -> Calibration {
        self.calibration
    }

    /// The sealed capacity, `C`.
    pub fn capacity(&self) -> u64 {
        self.calibration.capacity.get()
    }
}

/// WHY THE CALIBRATION STROKE MEASURED NO READ'S SIZE (D2.5): the arm of
/// `Uncalibrated::Footprint`, carrying the reading that refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unstroked {
    /// The allocator's per-thread counters do not read.
    Allocator,
    /// The stroke's reads did not carry its work to the end: the founding
    /// motions' modes did not read under the motions' own need.
    Incomplete,
    /// The stroke spent `reads` and left `retained` bytes held: no read, or
    /// nothing held (a process whose allocations the counters do not see),
    /// is no size of a read.
    Retained { retained: i128, reads: u64 },
}

/// The bytes the calling thread holds by the allocator's own counters: all
/// it was ever given less all it gave back, exact (jemalloc's
/// `thread.allocatedp` and `thread.deallocatedp`, moved on every
/// allocation). `stats.allocated` is not this reading: it moves only as a
/// thread cache fills and flushes, and read zero across a whole stroke.
fn thread_held() -> Option<i128> {
    let given = tikv_jemalloc_ctl::thread::allocatedp::read().ok()?.get();
    let back = tikv_jemalloc_ctl::thread::deallocatedp::read().ok()?.get();
    Some(i128::from(given) - i128::from(back))
}

/// THE SENSOR BASE'S FOUNDING MOTIONS: the turn of the plane of the first
/// two axes (harmonic) and the slide along the first axis (luminal).
fn founding_motions() -> [crate::transducer::cga::Mv; 2] {
    use crate::transducer::cga::{translator_at, Mv, E1, E2};
    let unit = || crate::core::q::Q::from_integer(1.into());
    let turn = Mv::scalar(unit()).add(&Mv::blade(E1 | E2, unit()));
    let slide: Vec<crate::core::q::Q> = (0..=crate::transducer::base::FRAME_AXIS)
        .map(|axis| match axis {
            0 => unit(),
            _ => crate::core::q::Q::from_integer(0.into()),
        })
        .collect();
    [turn, translator_at(&slide)]
}

/// THE MINIMAL STROKE (D2.5): the reads the founding motions' modes need
/// (`cga::modes_need`, D3.1), the least work a process does. Refused
/// `Unstroked::Incomplete` where a founding motion has no need to read.
/// Its inputs are fixed motions, so it is derived once and held.
pub fn minimal_stroke() -> Result<std::num::NonZeroU64, Uncalibrated> {
    *MINIMAL.get_or_init(founding_need)
}

/// The minimal stroke, derived once (MU-STANDARDS 2.12).
static MINIMAL: std::sync::OnceLock<Result<std::num::NonZeroU64, Uncalibrated>> = std::sync::OnceLock::new();

/// The founding motions' modes' need, summed.
fn founding_need() -> Result<std::num::NonZeroU64, Uncalibrated> {
    let incomplete = Uncalibrated::Footprint(Unstroked::Incomplete);
    let mut need = 0u64;
    for motion in founding_motions() {
        let n = crate::transducer::cga::modes_need(&motion).and_then(|n| u64::try_from(n).ok()).ok_or(incomplete)?;
        need = need.checked_add(n).ok_or(incomplete)?;
    }
    std::num::NonZeroU64::new(need).ok_or(incomplete)
}

/// THE BOOT CALIBRATION STROKE (D2.5, O2 ruled 2026-11-01): one stroke,
/// sized by the minimal stroke's own need, reads the modes of the founding
/// motions and measures the bytes its reads left held while the motions'
/// modes are held: `b = ⌈ (held after − held before) / reads spent ⌉`.
/// `held` is the thread's allocator reading. A stroke that cannot complete
/// is refused by name.
pub fn boot_stroke(held: impl Fn() -> Option<i128>) -> Result<std::num::NonZeroU64, Uncalibrated> {
    use crate::transducer::cga::motion_modes;
    let incomplete = Uncalibrated::Footprint(Unstroked::Incomplete);
    let need = usize::try_from(minimal_stroke()?.get()).map_err(|_| incomplete)?;
    let [turn, slide] = founding_motions();
    let mut stroke = super::Budget { remaining: need };
    let before = held().ok_or(Uncalibrated::Footprint(Unstroked::Allocator))?;
    let spent = super::spent_here();
    let turned = motion_modes(&turn, &mut stroke);
    let slid = motion_modes(&slide, &mut stroke);
    let reads = super::spent_here() - spent;
    let after = held().ok_or(Uncalibrated::Footprint(Unstroked::Allocator))?;
    let (Some(Ok(turned)), Some(Ok(slid))) = (turned, slid) else {
        return Err(incomplete);
    };
    let footprint = footprint_of(after - before, reads);
    drop((turned, slid));
    footprint
}

/// THE PROCESS'S CAPACITY in reads (D2.5): the sealed `C`. Before the seal
/// (a process that never calibrates: a library, a test harness), the
/// capacity whose one spend is the minimal stroke, `2^(FRAME_AXIS+1)`
/// minimal strokes, derived from the least work there is and the same on
/// every host; zero only where the founding motions have no need to read.
pub fn capacity() -> u64 {
    match SEALED.get() {
        Some(c) => c.capacity.get(),
        None => match minimal_stroke().ok().and_then(|m| m.get().checked_mul(STROKES_PER_RESERVE)) {
            Some(c) => c,
            None => 0,
        },
    }
}

/// The sealed calibration, where this process has sealed one.
pub fn sealed() -> Option<Calibration> {
    SEALED.get().copied()
}

/// THE APERTURE — EVERY DIMENSION IS AN EXPONENT (founding, MU-RECURSION
/// §0.0): a full pool holds one expansion stroke per vertex of the sensor
/// base's unit cell, `2ⁿ` for its `n` axes — the two grid axes and the
/// frame axis (`transducer::base`), so `2³`. It is algebra, not a scale —
/// the scale is the process's (`capacity`). It reads the fixed frame axis,
/// which ROADMAP track 26 S4 removes: an open dependency (MU-DERIVATIONS
/// D2.5 O3).
pub const STROKES_PER_RESERVE: u64 = 1 << (crate::transducer::base::FRAME_AXIS + 1);

/// One entropic expansion's spend — the capacity over the ratio.
pub fn spend_unit() -> u64 {
    capacity() / STROKES_PER_RESERVE
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
    // An uncalibrated host parks nothing: no floor was read, and the server,
    // client and soak binaries refuse to boot on it, naming the arm
    // (`calibrate`). A park of no duration is the scheduler's yield.
    *CALIBRATED_STROKE_FLOOR.get_or_init(|| match stroke_floor_of(scheduler_hz(), timer_slack_ns()) {
        Ok(d) => d,
        Err(_) => std::time::Duration::ZERO,
    })
}

/// WHY A HOST IS NOT CALIBRATED, by the arm it does not read (MU-DERIVATIONS
/// D2.6): a host that does not say is fixed by its operator, never run on a
/// guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Uncalibrated {
    /// The host does not say how many cores it has.
    Cores,
    /// The host publishes neither its scheduler tick (`CONFIG_HZ`) nor its
    /// timer slack (`timerslack_ns`): no floor for a park reads.
    StrokeFloor,
    /// The process says no memory limit and the host no available bytes.
    Memory,
    /// The boot calibration stroke measured no read's size (D2.5).
    Footprint(Unstroked),
    /// The process's capacity `C` does not fit the minimal stroke in one
    /// spend: `capacity` reads, `minimal` the least stroke (D2.5).
    Unfit { capacity: u64, minimal: u64 },
}

/// The cores a host's reading names, or the refusal of an unread one.
pub fn cores_of(read: std::io::Result<std::num::NonZeroUsize>) -> Result<std::num::NonZeroU64, Uncalibrated> {
    let n = read.map_err(|_| Uncalibrated::Cores)?;
    std::num::NonZeroU64::new(n.get() as u64).ok_or(Uncalibrated::Cores)
}

/// THE STROKE FLOOR FROM WHAT THE HOST PUBLISHES (D2.6): one tick, `1 s /
/// CONFIG_HZ`, where the kernel publishes its tick; else the calling
/// thread's timer slack, which the kernel rounds every shorter wait up to
/// (the ask then equals the grant); else refused.
#[expect(
    clippy::disallowed_types,
    reason = "law 9 — the parking floor, in the scheduler's own units, read off what the host publishes"
)]
pub fn stroke_floor_of(hz: Option<u32>, slack_ns: Option<u64>) -> Result<std::time::Duration, Uncalibrated> {
    match (hz, slack_ns) {
        (Some(hz), _) => Ok(std::time::Duration::from_secs(1) / hz),
        (None, Some(ns)) if ns > 0 => Ok(std::time::Duration::from_nanos(ns)),
        _ => Err(Uncalibrated::StrokeFloor),
    }
}

/// THE CALLING THREAD'S TIMER SLACK in nanoseconds, as the kernel publishes
/// it (`/proc/thread-self/timerslack_ns`, Linux 4.6 on; `/proc/self/` on
/// older paths); `None` where it is unpublished.
fn timer_slack_ns() -> Option<u64> {
    ["/proc/thread-self/timerslack_ns", "/proc/self/timerslack_ns"]
        .iter()
        .find_map(|p| std::fs::read_to_string(p).ok()?.trim().parse().ok())
}

/// THE CAPACITY'S READINGS, TAKEN IN ORDER (D2.5): `M_proc` from the
/// process's limit or the host's available memory (`memory_of`), then `b`
/// from the one boot calibration stroke, read through `held`, then the fit
/// of the minimal stroke; the first that does not read refuses by name.
/// Memory is read first, so a process whose memory cannot be read is
/// refused before any stroke runs, and no capacity is derived from a guess.
pub fn calibration_of(limit: Option<u64>, available_kib: Option<u64>, held: impl Fn() -> Option<i128>) -> Result<Calibration, Uncalibrated> {
    let memory = memory_of(limit, available_kib)?;
    let footprint = boot_stroke(held)?;
    Calibration::of(memory, footprint, minimal_stroke()?)
}

/// THE BOOT'S CALIBRATION (D2.6, D2.5): every arm the host must say, or the
/// first it does not, by name; then the one boot calibration stroke, which
/// measures the bytes per read and seals the capacity. The binaries call it
/// first and refuse to boot on a refusal, so the capacity is sealed before
/// any act is served. Sealed once: a later call reads every arm again and
/// returns the seal it finds, with no second stroke.
pub fn calibrate() -> Result<Sealed, Uncalibrated> {
    cores_of(std::thread::available_parallelism())?;
    stroke_floor_of(scheduler_hz(), timer_slack_ns())?;
    let (limit, kib) = (read_cgroup_limit(), read_mem_available_kib());
    memory_of(limit, kib)?;
    if let Some(calibration) = SEALED.get() {
        return Ok(Sealed { calibration: *calibration });
    }
    let measured = calibration_of(limit, kib, thread_held)?;
    let calibration = *SEALED.get_or_init(|| measured);
    Ok(Sealed { calibration })
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

/// How long an idle stroke parks: the measured transport where one has
/// been measured — `rtt` in polls, never zero (the poll that read the first
/// byte is counted), each poll a stroke of this host — and the floor itself,
/// one stroke, the unit, where none has, or where the measured wait passes
/// the duration's carrier.
#[expect(
    clippy::disallowed_types,
    reason = "law 9 — a park is told to the scheduler in its own units; the pace is strokes"
)]
pub fn idle_park(rtt: Option<std::num::NonZeroU64>) -> std::time::Duration {
    let floor = stroke_floor();
    match rtt.and_then(|r| u32::try_from(r.get()).ok()).and_then(|n| floor.checked_mul(n)) {
        Some(d) => d,
        None => floor,
    }
}

/// Strokes per pool cycle — the aperture: the full pool over one spend,
/// the vertices of the sensor base's unit cell (`STROKES_PER_RESERVE`), in
/// the `u32` its readers count circuits in.
pub fn aperture_cycle() -> u32 {
    1 << (crate::transducer::base::FRAME_AXIS + 1)
}
