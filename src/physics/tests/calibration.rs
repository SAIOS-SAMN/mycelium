use super::super::*;

/// PH1 — THE RATIO STANDS EXACT AND THE FLOOR IS THE HOST'S: an unsealed
/// process's capacity is a whole number of spends, the aperture is the
/// two-stroke ratio itself, and the stroke floor is a positive measurement
/// every park is at least one of.
#[test]
fn spend_unit_ratio_is_exact_and_stroke_floor_positive() {
    assert_eq!(spend_unit() * STROKES_PER_RESERVE, capacity(), "a whole number of spends");
    assert_eq!(u64::from(aperture_cycle()), STROKES_PER_RESERVE, "the aperture is the ratio");
    let floor = stroke_floor();
    assert!(!floor.is_zero(), "the floor is a measurement, never zero");
    assert_eq!(idle_park(None), floor, "an unmeasured wire parks one stroke, the unit");
    assert_eq!(idle_park(std::num::NonZeroU64::new(3)), floor * 3, "a measured wire parks its round trip");
}

/// D2.5 — AN UNSEALED PROCESS SPENDS THE MINIMAL STROKE: before any seal
/// (this harness never calibrates), one spend of the capacity is the
/// founding motions' modes' need, the least work there is, read off the
/// motions and the same on every host; no fixed floor stands in for it.
#[test]
fn capacity_unsealed_spends_minimal_stroke() {
    let minimal = minimal_stroke().expect("the founding motions' modes have a need");
    assert_eq!(spend_unit(), minimal.get(), "one spend is the minimal stroke");
}

/// D2.6 — A HOST THAT DOES NOT SAY HOW MANY CORES IT HAS IS REFUSED BY NAME,
/// never run on one core: the read's refusal is `Uncalibrated::Cores`.
#[test]
fn calibration_refuses_unread_cores() {
    let unread = Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "not said"));
    assert_eq!(cores_of(unread), Err(Uncalibrated::Cores));
    assert_eq!(cores_of(Ok(std::num::NonZeroUsize::MIN)).map(std::num::NonZeroU64::get), Ok(1));
}

fn bytes(n: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(n).expect("a fixture size is positive")
}

fn limit(n: u64) -> Memory {
    Memory::of(bytes(n), MemorySource::CgroupMax)
}

/// D2.5 — THE CAPACITY IS THE PROCESS'S MEMORY OVER THE MEASURED BYTES PER
/// READ: `C = ⌊M_proc / b⌋` exactly, so its reads at `b` each fit `M_proc`
/// and one read more would not; no unit of its own (a GiB) and no rounding
/// to strokes enters. `M = 10 000`, `b = 37`: `C = 270`.
#[test]
fn capacity_equals_memory_over_footprint() {
    let c = Calibration::of(limit(10_000), bytes(37), bytes(2)).expect("fits");
    assert_eq!(c.capacity().get(), 270, "⌊10 000 / 37⌋");
    assert!(c.capacity().get() * c.footprint().get() <= c.memory().bytes().get(), "its reads fit");
    assert!((c.capacity().get() + 1) * c.footprint().get() > c.memory().bytes().get(), "one read more would not");
    let doubled = Calibration::of(limit(10_000), bytes(74), bytes(2)).expect("fits");
    assert_eq!(doubled.capacity().get(), 135, "twice the read's size, half the reads");
}

/// D2.5 — A PROCESS WHOSE MINIMAL STROKE CANNOT FIT IS REFUSED BY NAME:
/// where one spend of `C` is less than the minimal stroke, the calibration
/// is `Uncalibrated::Unfit`, carrying the capacity and the stroke, so such
/// a process never boots.
#[test]
fn calibration_refuses_unfit_minimal_stroke() {
    let refused = Calibration::of(limit(100), bytes(37), bytes(1));
    assert_eq!(refused, Err(Uncalibrated::Unfit { capacity: 2, minimal: 1 }), "a capacity of 2 spends nothing");
    let fits = Calibration::of(limit(37 * 16), bytes(37), bytes(2)).expect("a spend of two covers two reads");
    assert_eq!(fits.capacity().get(), 16);
}

/// D2.5 (a) — `M_proc` IS THE PROCESS'S OWN LIMIT FIRST: the cgroup limit
/// where one is set, named `CgroupMax`; `1024 · MemAvailable_kB` only where
/// none is, named `MemAvailable`; and a process that says neither, says
/// none, or says past the carrier is refused `Uncalibrated::Memory`.
#[test]
fn memory_of_reads_cgroup_limit_before_available() {
    assert_eq!(memory_of(Some(6_442_450_944), Some(4)), Ok(limit(6_442_450_944)), "the limit, where set");
    assert_eq!(memory_of(None, Some(4)), Ok(Memory::of(bytes(4_096), MemorySource::MemAvailable)), "else the host's");
    assert_eq!(memory_of(None, None), Err(Uncalibrated::Memory));
    assert_eq!(memory_of(Some(0), Some(4)), Err(Uncalibrated::Memory), "a limit of none");
    assert_eq!(memory_of(None, Some(0)), Err(Uncalibrated::Memory));
    assert_eq!(memory_of(None, Some(u64::MAX)), Err(Uncalibrated::Memory), "past the carrier");
}

/// D2.5 (a) — THE CGROUP LIMIT IS THE LEAST ON THE PROCESS'S PATH: the
/// unified-hierarchy line `0::<path>` names the cgroup; its `memory.max`
/// and every ancestor's are read, `max` is no limit, and the least number
/// binds. A process in no unified hierarchy, or under `max` throughout,
/// has none.
#[test]
fn cgroup_limit_of_reads_least_limit_on_path() {
    let tree = |pairs: &'static [(&'static str, &'static str)]| move |dir: &str| pairs.iter().find(|(d, _)| *d == dir).map(|(_, v)| v.to_string());
    let line = Some("0::/user.slice/run.scope\n");
    assert_eq!(cgroup_limit_of(line, tree(&[("/user.slice/run.scope", "max\n"), ("/user.slice", "6442450944\n")])), Some(6_442_450_944));
    assert_eq!(cgroup_limit_of(line, tree(&[("/user.slice/run.scope", "1000\n"), ("/user.slice", "5000\n")])), Some(1_000), "the least");
    assert_eq!(cgroup_limit_of(line, tree(&[("/user.slice/run.scope", "max\n")])), None, "max throughout");
    assert_eq!(cgroup_limit_of(Some("1:cpu:/x\n"), tree(&[("/x", "1000\n")])), None, "no unified line");
    assert_eq!(cgroup_limit_of(None, tree(&[])), None, "membership unread");
}

/// D2.5 — THE FOOTPRINT IS BYTES PER READ ON A FIXTURE STROKE: what a
/// stroke left held over the reads it spent, rounded up, so the capacity's
/// reads at `b` never undercount what they held.
#[test]
fn footprint_is_bytes_per_read_on_fixture_stroke() {
    assert_eq!(footprint_of(999, 3), Ok(bytes(333)), "exact where the reads divide");
    assert_eq!(footprint_of(1_000, 3), Ok(bytes(334)), "rounded up where they do not");
    assert_eq!(footprint_of(1, 2), Ok(bytes(1)), "a byte over two reads is a byte a read");
}

/// D2.5 — A STROKE THAT MEASURED NO READ'S SIZE IS REFUSED BY NAME, with the
/// reading that refused: no read spent, nothing held, or less held at the
/// end than at the start.
#[test]
fn footprint_of_refuses_unread_stroke() {
    let refused = |retained, reads| Err(Uncalibrated::Footprint(Unstroked::Retained { retained, reads }));
    assert_eq!(footprint_of(328, 0), refused(328, 0), "no read spent");
    assert_eq!(footprint_of(0, 2), refused(0, 2), "nothing held");
    assert_eq!(footprint_of(-64, 2), refused(-64, 2), "less held than before");
}

/// D2.5 — A HOST WHOSE STROKE CANNOT COMPLETE IS REFUSED BY NAME: where the
/// allocator's counters do not read, the boot stroke is
/// `Uncalibrated::Footprint(Unstroked::Allocator)`, never a size guessed.
#[test]
fn boot_stroke_refuses_unread_allocator() {
    assert_eq!(boot_stroke(|| None), Err(Uncalibrated::Footprint(Unstroked::Allocator)));
}

/// D2.5 — A STROKE THAT HOLDS NOTHING IS REFUSED BY NAME: where the
/// allocator reads the same before and after (a process whose allocations
/// the counters do not see), the boot stroke spent its reads and measured
/// no size. The boot is refused `Unstroked::Retained { retained: 0, .. }`,
/// carrying the reads spent, never a size guessed, and no capacity is
/// derived from it.
#[test]
fn stroke_that_holds_nothing_refuses() {
    match calibration_of(None, Some(4), || Some(0)) {
        Err(Uncalibrated::Footprint(Unstroked::Retained { retained: 0, reads })) => assert!(reads > 0, "the stroke read"),
        other => panic!("a stroke that holds nothing is refused by name: {other:?}"),
    }
}

/// D2.5 — A PROCESS WHOSE MEMORY CANNOT BE READ IS REFUSED AT BOOT BY NAME:
/// where neither a cgroup limit nor the host's available memory reads, or
/// the host says none, the calibration is `Uncalibrated::Memory` before any
/// stroke runs (a held reader that would itself refuse is never reached),
/// so no capacity is sealed on a guess of `M_proc`.
#[test]
fn unreadable_memory_refuses_boot() {
    assert_eq!(calibration_of(None, None, || None), Err(Uncalibrated::Memory), "a process that does not say");
    assert_eq!(calibration_of(None, Some(0), || None), Err(Uncalibrated::Memory), "a host that says none");
}

/// D2.5 — THE OPEN-PHYSICS REGISTER READS THE CAPACITY MEASURED: no entry
/// stands at a site of the calibration or the process's memory, or cites
/// D2.5, so the capacity is a reading of the process (`M_proc`, `b`) taken
/// at the seal, never a registered number.
#[test]
fn open_physics_reads_capacity_measured() {
    for n in crate::physics::axioms::OPEN_PHYSICS {
        assert!(!n.site.starts_with("src/physics/calibration.rs"), "{} is registered at the calibration", n.name);
        assert!(!n.site.starts_with("src/physics/memory.rs"), "{} is registered at the memory", n.name);
        assert!(!n.doc.contains("D2.5"), "{} is registered under D2.5", n.name);
    }
}

/// D2.6 — THE STROKE FLOOR READS THE TIMER SLACK WHERE THE TICK IS
/// UNPUBLISHED: one tick where `CONFIG_HZ` is published; else the thread's
/// slack, the grant every shorter ask is rounded to; else refused by name —
/// never a nanosecond asked in its place.
#[test]
fn stroke_floor_reads_slack_where_hz_unpublished() {
    let ns = |hz, slack| stroke_floor_of(hz, slack).map(|d| d.as_nanos());
    assert_eq!(ns(Some(250), Some(50_000)), Ok(4_000_000), "the tick, where published");
    assert_eq!(ns(None, Some(50_000)), Ok(50_000), "the slack, where it is not");
    assert_eq!(stroke_floor_of(None, None), Err(Uncalibrated::StrokeFloor));
    assert_eq!(stroke_floor_of(None, Some(0)), Err(Uncalibrated::StrokeFloor), "a zero slack is no floor");
}

/// Issue #234 — IDLE BACKOFF: back off consecutive idle cycles up to the
/// aperture bound, starting at one scheduler floor.
#[test]
fn idle_backoff_backs_off_up_to_aperture() {
    let floor = crate::physics::stroke_floor();
    assert_eq!(crate::physics::idle_backoff(0), floor, "0 consecutive idle cycles parks 1 floor");
    assert_eq!(crate::physics::idle_backoff(1), floor * 2, "1 consecutive idle cycle parks 2 floors");
    assert_eq!(crate::physics::idle_backoff(2), floor * 4, "2 consecutive idle cycles parks 4 floors");
    let max = crate::physics::idle_backoff(100);
    assert_eq!(max, crate::physics::idle_backoff(10), "idle park caps at aperture shift");
    assert_eq!(max, floor * crate::physics::aperture_cycle(), "capped backoff is aperture_cycle floors");
}
