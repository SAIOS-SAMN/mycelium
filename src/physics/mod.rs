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

pub mod actions;
pub mod axioms;
pub mod budget;
pub mod capacitor;
pub mod fascicle;
pub mod laws;
pub mod malleability;
pub mod metabolism;
pub mod outflow;
pub mod source;
pub mod tensegrity;
pub use budget::{halt, halted, spent_here, Budget, Depleted, Reserve};

/// A CARRIER'S CIRCUITS (45.7; named so at OR2, 73.7: the circuit is the
/// clock, the orbit is the word): the circuits a stroke ring has completed
/// and the period of the last one in strokes — the kernel's local time
/// beside K, read off the ring and never a clock. One per carrier: the
/// court's docket, the exchange to one peer, the mind's dirty ring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Circuits {
    strokes: u64,
    circuits: u64,
    at: u64,
    period: u64,
}

impl Circuits {
    /// One stroke over the carrier, with the circuits it completed. The
    /// reading — circuits so far and the strokes the last one took — is
    /// returned when a circuit completed, so the caller writes its circuit
    /// line once per circuit (34.2, 34.3) and never per stroke.
    pub fn stroke(&mut self, circuits: u64) -> Option<(u64, u64)> {
        // A count past its carrier is refused and counted, and no circuit is
        // read on the stroke that met it (STD1, rule 11).
        if !metabolism::step_or_refuse(&mut self.strokes, 1) {
            return None;
        }
        let n = std::num::NonZeroU64::new(circuits)?;
        if !metabolism::step_or_refuse(&mut self.circuits, n.get()) {
            return None;
        }
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

/// Deadline for one nerve dial, in strokes: an aperture of the measured
/// transport. Unmeasured RTT has no deadline.
pub fn transport_deadline(rtt: u64) -> Lambda {
    persist_dwell(rtt)
}

/// Persist dwell: a full aperture of measured transports, in strokes.
pub fn persist_dwell(rtt: u64) -> Lambda {
    from_transport(rtt, aperture_cycle() as u64)
}

/// Quiet-digest ceiling: a full reserve of measured transports, in strokes.
pub fn digest_ceiling(rtt: u64) -> Lambda {
    from_transport(rtt, reserve_max())
}

/// Strokes per reserve cycle — the aperture: the full reserve over one
/// spend, the vertices of the sensor base's unit cell (`STROKES_PER_RESERVE`),
/// in the `u32` its readers count circuits in.
pub fn aperture_cycle() -> u32 {
    1 << (crate::transducer::base::FRAME_AXIS + 1)
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
    /// The syntropic count each lane last relaxed at: what a lane settled
    /// since is what its pressure relaxes by (PHY1).
    pub relax_mark: [u64; 4],
    /// What contacts had moved, per lane, when this physics last booked them
    /// — condensed, then refused (`book_contacts`).
    pub contacts_mark: [[u64; 4]; 2],
    /// What the voice's tubes had relieved when this physics last booked its
    /// field (`book_flux`, 78.71).
    pub flux_mark: [u64; 4],
    /// 78.73 — PLASTIC events per lane: generators that entered the language
    /// on it, the medium taking stress by changing shape (`malleability`).
    pub plastic: [u64; 4],
    /// The METABOLIC RESERVE per lane — the two-stroke engine's fuel. ENTROPIC
    /// expansion (germination) SPENDS it (the reserve shrinks as the alphabet
    /// grows); SYNTROPIC optimization (consolidation) REGENERATES it (the reserve
    /// recovers as the basis is pruned toward a minimal, confluent generating set).
    /// When a lane depletes, the mesh switches from expanding to optimizing.
    ///
    /// THE ONE RESERVE (65.5, PHY1): a [`Reserve`], so every draw and every
    /// return passes through the metabolic ledger (`note_draw`,
    /// `note_restore`) — the flux `drawn − restored` is read, never zero by
    /// construction. Read by `lanes()`; moved only by `draw`, `drain` and
    /// `restore`.
    pub reserve: Reserve,
}

impl Default for Physics {
    fn default() -> Self {
        Physics {
            pressure: [0; 4],
            syntropic: [0; 4],
            torsion: [0; 4],
            regen_mark: [0; 4],
            relax_mark: [0; 4],
            contacts_mark: [[0; 4]; 2],
            flux_mark: [0; 4],
            plastic: [0; 4],
            reserve: Reserve::of([reserve_max(); 4]),
        }
    }
}

impl Physics {
    /// Draw one stroke's read token. The only constructor of [`Budget`].
    /// Remaining reads equal one spend; cognition never names that number.
    #[expect(clippy::cast_possible_truncation, reason = "site 4 — spend_unit, a u64 → usize narrowing on a 32-bit target; its opposite operation is asked at site 4")]
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
            metabolism::step_or_refuse(p, *d);
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

    /// The metabolism RELAXES — the entropic backlog DISSIPATES as the mesh
    /// works, so per-lane pressure is a RATE, not a one-way ratchet. EACH LANE
    /// BY WHAT IT SETTLED (PHY1): the conjugate of `bump`, which raised the
    /// lane by the demand's own magnitude, is the order that lane built since
    /// it last relaxed — its syntropic count over its mark — never a literal
    /// taken off every lane alike. A lane that settled nothing does not
    /// relax; one that settled more than it carries is relieved of what
    /// stands on it (`relieve`).
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a lane whose mark stands ahead of its total relaxes by nothing it settled, named at the arm"
    )]
    pub fn relax(&mut self) {
        // The mark is a syntropic total this lane already reached, so the
        // difference is exact; a mark ahead of the total is a broken counter
        // and that lane relaxes by nothing it settled (rule 11).
        let settled: [u64; 4] = std::array::from_fn(|l| match self.syntropic[l].checked_sub(self.relax_mark[l]) {
            Some(d) => d,
            None => 0,
        });
        self.relieve(&settled);
        self.relax_mark = self.syntropic;
    }

    /// PHY1 — WHAT CONTACTS DID, BOOKED: since this physics last booked them,
    /// a contact that condensed built order on the lanes its transport
    /// occupies (syntropy), and one that was refused pressed on the lane its
    /// refusal names (pressure). `seen` is what contacts have moved so far,
    /// per lane — condensed, then refused; a count behind the mark is a
    /// broken counter and books nothing on that lane (rule 11).
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a count behind its mark books nothing, named at the arm"
    )]
    pub fn book_contacts(&mut self, seen: ([u64; 4], [u64; 4])) {
        let since = |now: &[u64; 4], mark: &[u64; 4]| -> [u64; 4] {
            std::array::from_fn(|l| match now[l].checked_sub(mark[l]) {
                Some(d) => d,
                None => 0,
            })
        };
        let condensed = since(&seen.0, &self.contacts_mark[0]);
        let refused = since(&seen.1, &self.contacts_mark[1]);
        self.syntropy(&condensed);
        self.bump(&refused);
        self.contacts_mark = [seen.0, seen.1];
    }

    /// 78.71 (2) — WHAT THE FIELD DID, ITS RELIEF BOOKED: since this physics
    /// last booked it, the open flux the voice's tubes' closures relieved
    /// falls off 𝒯's pressure exactly. `relieved` is what closures relieved
    /// so far, per lane; a count behind its mark books nothing on that lane
    /// (rule 11).
    ///
    /// MU-CLIENT §4.13 — THE RESERVE ON 𝒯 IS CONSERVED, never drawn and
    /// refilled here: it falls by the flux left open and no more, by the one
    /// lock that withholds what the field holds — the tubes' open flux and
    /// the reservoirs (`Field::withheld`, `Entity::lock_reservoirs`) — and
    /// gives it back exactly as it closes, dissipates or condenses — never by
    /// the condensed ledger, which counts face carriages that drew nothing.
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a count behind its mark books nothing, named at the arm"
    )]
    pub fn book_flux(&mut self, relieved: [u64; 4]) {
        let since: [u64; 4] = std::array::from_fn(|l| match relieved[l].checked_sub(self.flux_mark[l]) {
            Some(d) => d,
            None => 0,
        });
        self.flux_mark = relieved;
        self.relieve(&since);
    }

    /// A pivot on `lane` — a torsion charge (the demand for a new fiber). An
    /// unattributable pivot charges all lanes (its resisting axis is unknown).
    pub fn pivot(&mut self, lanes: [bool; 4]) {
        let any = lanes.iter().any(|&b| b);
        for (t, &on) in self.torsion.iter_mut().zip(&lanes) {
            if on || !any {
                metabolism::step_or_refuse(t, 1);
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
                metabolism::step_or_refuse(o, spanned);
            }
        }
        true
    }

    /// A settlement built order on these lanes — a SYNTROPIC event (negentropy).
    /// Order built, by the magnitude of what was settled or composed.
    pub fn syntropy(&mut self, m: &[u64; 4]) {
        for (o, d) in self.syntropic.iter_mut().zip(m) {
            metabolism::step_or_refuse(o, *d);
        }
    }

    /// ENTROPIC EXPANSION, DRAWN (PHY1): one expansion on each lane the act
    /// moved, as ONE vector out of the reserve, all or nothing — the
    /// metabolism SHRINKS as the alphabet grows. A lane that cannot cover it
    /// refuses the draw by name (the ledger counts it on that lane, and the
    /// refusal is answered); the act already happened, so it is then charged
    /// what each lane still holds, never past empty (rule 11). A stroke that
    /// moved no lane draws nothing: there is no tick, since a stroke is the
    /// draw of what it did and not a toll for having run.
    pub fn spend(&mut self, lanes: [bool; 4]) -> Option<Depleted> {
        let unit = spend_unit();
        let cost: [u64; 4] = std::array::from_fn(|l| if lanes[l] { unit } else { 0 });
        // 78.73 (1): a lane that cannot cover it is pressed by what it blocked.
        match self.draw(cost) {
            Ok(()) => None,
            Err(refused) => {
                let _ = self.reserve.drain(cost);
                Some(refused)
            }
        }
    }

    /// SYNTROPIC OPTIMIZATION regenerates each lane toward the deflation
    /// that lane verified since the last aperture — not toward one scalar cap.
    pub fn regenerate(&mut self) {
        let held = self.reserve.lanes();
        let mut back = [0u64; 4];
        for l in 0..4 {
            // The mark is a syntropic total this lane has already reached,
            // so the difference is exact. A mark AHEAD of the total is a
            // broken counter, and this lane regenerates toward nothing it
            // verified rather than toward a difference of zero (rule 11).
            let Some(verified) = self.syntropic[l].checked_sub(self.regen_mark[l]) else {
                continue;
            };
            // Two strokes re-earn one expansion; a lane that verified work
            // since its last regeneration earns the second at once.
            let add = match verified {
                0 => regen_unit(),
                _ => regen_unit() * 2,
            };
            // Toward the work the lane verified, and never below the
            // depletion floor, one expansion (the ruled unit of the
            // two-stroke governor, not a number of its own).
            let target = verified.max(expand_floor());
            // A regeneration past the carrier is refused and counted; the
            // lane stands where it was.
            let mut next = held[l];
            if !metabolism::step_or_refuse(&mut next, add) {
                continue;
            }
            // What returns is the rise, read against what the lane held: a
            // RESTORE, through the metabolic ledger.
            back[l] = next.min(target.max(held[l])) - held[l];
            self.regen_mark[l] = self.syntropic[l];
        }
        self.reserve.restore(back);
    }

    /// 78.56 IV — A COURT'S RESERVE, BY ITS CAPACITY. The ceiling is not a
    /// hardware constant: it is `bonds` (the active bonds staked, per lane)
    /// plus the work verified since the last close (the court's own window,
    /// one orbit). Each close the reserve regenerates toward it at the
    /// regeneration law's own rate (`regenerate`'s step), and a reserve held
    /// above it — an idle orbit after a productive one — contracts to it, the
    /// excess drained. It expands with useful work and contracts when idle.
    pub fn regenerate_within(&mut self, bonds: [u64; 4]) {
        let held = self.reserve.lanes();
        let mut back = [0u64; 4];
        let mut excess = [0u64; 4];
        // Σ ACTIVE BONDS IS THE SUM OVER THE BOND, not each lane's own: a
        // court bonds nothing on 𝒯 ([142545, 29789, 0, 305223] on the
        // deployment), and the per-lane reading capped the 𝒯 reserve at the 𝒯
        // work verified in one orbit — near zero — so every word with any
        // 𝒯 in its magnitude was deferred for reserve (6 257 syllabus words
        // at hypha a, epoch 115). The ruling reads "Σ active bonds plus the
        // work verified", so each lane stands within the whole bond plus its
        // own verified work.
        let Some(bond) = bonds.iter().try_fold(0u64, |s, &b| s.checked_add(b)) else {
            return;
        };
        for l in 0..4 {
            let Some(verified) = self.syntropic[l].checked_sub(self.regen_mark[l]) else {
                continue;
            };
            // A ceiling past the carrier is refused and counted; the lane
            // stands where it was.
            let Some(ceiling) = bond.checked_add(verified) else {
                continue;
            };
            // THE REFILL FOLLOWS THE WORK (ruled
            // 2026-09-30): one regeneration unit, plus the work this lane verified
            // in the orbit — a productive court teaches faster, an idle one
            // no faster, and the ceiling still bounds it. A fixed step
            // deferred 6 080 syllabus words in ten minutes against 56 served
            // at hypha a (epoch 115).
            let Some(add) = regen_unit().checked_add(verified) else {
                continue;
            };
            match held[l].checked_sub(ceiling) {
                Some(over) if over > 0 => excess[l] = over,
                _ => {
                    let mut next = held[l];
                    if metabolism::step_or_refuse(&mut next, add) {
                        back[l] = next.min(ceiling) - held[l];
                    }
                }
            }
            self.regen_mark[l] = self.syntropic[l];
        }
        self.reserve.restore(back);
        let _ = self.reserve.drain(excess);
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
                true => match expand_floor().checked_sub(self.reserve.lanes()[l]) {
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
        std::array::from_fn(|l| match reserve_max().checked_sub(self.reserve.lanes()[l]) {
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
    ///
    /// Exact over the whole of both counters (STD1), as `Metabolism::flux`
    /// is: the difference of two `u64` is taken in `i128`, which holds every
    /// such difference. The `as i64` this was wrapped a counter past 2⁶³
    /// negative, and the difference could overflow — an invented value on
    /// the price path.
    pub fn order_flux(&self) -> [i128; 4] {
        std::array::from_fn(|l| i128::from(self.syntropic[l]) - i128::from(self.torsion[l]))
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
        for (d, s) in dst.iter_mut().zip(src) {
            metabolism::step_or_refuse(d, *s);
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

    /// STD1 (rule 11) — A LEDGER COUNTER PAST ITS CARRIER IS REFUSED AND
    /// COUNTED, never pinned at a ceiling: the lane stands where it was and
    /// the ledger's `overflowed` rises.
    #[test]
    fn bump_past_carrier_is_refused_and_counted() {
        let _ = metabolism::take_metabolism();
        let mut p = Physics::default();
        p.bump(&[u64::MAX, 0, 0, 0]);
        p.bump(&[1, 2, 0, 0]);
        assert_eq!(p.pressure, [u64::MAX, 2, 0, 0], "the full lane stood; the others stepped");
        assert_eq!(metabolism::take_metabolism().overflowed, 1, "one step refused, counted");
    }

    /// PHY1 — A LANE RELAXES BY WHAT IT SETTLED: the conjugate of `bump`,
    /// per lane, never a literal off every lane alike. A lane that settled
    /// nothing keeps its pressure; a second relax with nothing settled since
    /// moves nothing.
    #[test]
    fn relax_lowers_each_lane_by_what_it_settled() {
        let mut p = Physics::default();
        p.bump(&[5, 5, 0, 0]);
        p.syntropy(&[3, 0, 0, 0]);
        p.relax();
        assert_eq!(p.pressure, [2, 5, 0, 0], "𝒮 settled three and relaxed three; 𝒢 settled nothing");
        p.relax();
        assert_eq!(p.pressure, [2, 5, 0, 0], "nothing settled since, nothing relaxed");
    }

    /// PHY1 — WHAT CONTACTS DID, BOOKED ONCE: condensed contacts build order
    /// on the contacts' lanes, refused ones press on the refused contacts' own lanes, and a second booking of
    /// the same totals moves nothing — the mark is what was already booked.
    #[test]
    fn book_contacts_books_condensed_and_refused_once() {
        let mut p = Physics::default();
        p.book_contacts(([0, 2, 0, 1], [3, 0, 0, 0]));
        assert_eq!((p.syntropic, p.pressure), ([0, 2, 0, 1], [3, 0, 0, 0]));
        p.book_contacts(([0, 2, 0, 1], [3, 0, 0, 0]));
        assert_eq!((p.syntropic, p.pressure), ([0, 2, 0, 1], [3, 0, 0, 0]), "booked once");
        p.book_contacts(([0, 3, 0, 1], [3, 1, 0, 0]));
        assert_eq!((p.syntropic, p.pressure), ([0, 3, 0, 1], [3, 1, 0, 0]), "only what moved since");
    }

    /// Pressure is held per lane, never summed, and is relieved per lane.
    #[test]
    fn relieve_lowers_pressure_per_lane() {
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
    fn lanes_of_germinated_recolour_is_gauge() {
        use crate::transducer::element::from_canonical;
        use crate::transducer::germinate::germinate;
        // A two-site recolour: hue transitions on a standing support are
        // GAUGE content, carried alongside the residual defect — the lane
        // projection is measured from the chain, never funneled to one axis.
        let g = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        assert_eq!(g.lanes(), [false, true, false, false], "a recolour is gauge content; temporal is empty until the listing stands");
    }

    /// A pivot is a torsion charge; an unattributable pivot charges every lane.
    #[test]
    fn pivot_unattributed_charges_torsion_on_every_lane() {
        let mut p = Physics::default();
        p.pivot([false, false, false, false]); // unattributable
        assert_eq!(p.torsion, [1, 1, 1, 1], "an unresolved residual charges torsion");
    }

    /// THE TWO-STROKE RESERVE: entropic expansion SPENDS per lane; depletion is
    /// per-lane (polytonic); the syntropic stroke REGENERATES — and one stroke
    /// recovers LESS than one spend (REGEN < SPEND), so sustained expansion is
    /// paced by the recovery the optimization earns.
    #[test]
    fn spend_depletes_and_regenerate_restores_per_lane() {
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

    /// 78.56 IV — A COURT'S RESERVE LIVES WITHIN ITS CAPACITY: Σ bonds plus
    /// the work verified this window. Emptied, it refills toward that ceiling
    /// at the regeneration law's rate and never past it; verified work raises
    /// the ceiling; held above a lowered ceiling — an idle window after a
    /// productive one — it contracts to it.
    #[test]
    fn regenerate_within_bounds_reserve_by_bonds_and_work() {
        let mut p = Physics { reserve: Reserve::of([0; 4]), ..Default::default() };
        // A bond with nothing on 𝒯, as the deployment's courts hold: every lane
        // still stands within the whole bond (Σ = 9), never 𝒯's own 0.
        let bonds = [3, 3, 0, 3];
        for _ in 0..16 {
            p.regenerate_within(bonds);
        }
        assert_eq!(p.reserve.lanes(), [9; 4], "refilled to Σ bonds on every lane, 𝒯 included, and no further");
        p.syntropy(&[5, 0, 0, 0]);
        p.regenerate_within(bonds);
        assert!(p.reserve.lanes()[0] > 9, "verified work raises the ceiling on its lane");
        p.regenerate_within(bonds);
        assert_eq!(p.reserve.lanes()[0], 9, "an idle window contracts it to Σ bonds");
    }

    /// PHY1 — THE STROKE IS A DRAW. A stroke that moved no lane draws
    /// nothing; one that moved lanes draws one expansion on each, as one
    /// vector. A lane that cannot cover it refuses by name, counted on that
    /// lane, and the work already done is charged what the lanes hold —
    /// never past empty.
    #[test]
    fn spend_draws_moved_lanes_and_names_refusing_lane() {
        let _ = metabolism::take_metabolism();
        let mut idle = Physics::default();
        assert_eq!(idle.spend([false; 4]), None, "a stroke that moved nothing draws nothing");
        assert_eq!(idle.reserve.lanes(), [reserve_max(); 4], "and costs nothing");
        let mut p = Physics::default();
        assert_eq!(p.spend([true, false, true, false]), None, "two lanes moved, two expansions drawn");
        assert_eq!(
            p.reserve.lanes(),
            [reserve_max() - spend_unit(), reserve_max(), reserve_max() - spend_unit(), reserve_max()]
        );
        // Spent down on 𝒮 alone, the next draw that needs 𝒮 refuses on 𝒮.
        let mut short = Physics::default();
        for _ in 0..STROKES_PER_RESERVE {
            short.spend([true, false, false, false]);
        }
        let _ = metabolism::take_metabolism();
        let refused = short.spend([true, true, false, false]).expect("𝒮 cannot cover an expansion");
        assert_eq!(refused.lane, 0, "the refusal names 𝒮");
        assert_eq!(metabolism::take_metabolism().depleted, [1, 0, 0, 0], "counted on the lane that refused");
        assert_eq!(short.reserve.lanes()[1], reserve_max() - spend_unit(), "𝒢 is still charged what it did");
        assert_eq!(short.reserve.lanes()[0], 0, "and 𝒮 never past empty");
    }

    /// The metabolism is per-lane syntropic minus entropic — the order flux.
    #[test]
    fn order_flux_is_syntropic_minus_entropic_per_lane() {
        let mut p = Physics::default();
        p.syntropy(&[1, 0, 0, 0]); // built order on 𝒮
        p.syntropy(&[1, 0, 0, 0]);
        p.pivot([false, true, false, false]); // disorder on 𝒢
        assert_eq!(p.order_flux(), [2, -1, 0, 0], "syntropic − entropic, never summed");
    }

    /// Pressure on G dwells G and keeps expanding on S, T, H.
    #[test]
    fn dwell_holds_only_wanted_pressured_lane() {
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

    /// 45.7 — a carrier's circuits are read in circuits and the circuits' period in
    /// strokes, off the ring: three strokes to the first circuit, one to the
    /// next.
    #[test]
    fn circuits_stroke_reads_circuits_and_period() {
        let mut o = Circuits::default();
        assert_eq!(o.stroke(0), None, "no circuit, no reading");
        assert_eq!(o.stroke(0), None);
        assert_eq!(o.stroke(1), Some((1, 3)), "the first circuit took three strokes");
        assert_eq!(o.stroke(1), Some((2, 1)), "a budget that covers the carrier circles it every stroke");
        assert_eq!(o.stroke(2), Some((4, 1)), "two circuits in one stroke are two");
        assert_eq!((o.circuits(), o.period()), (4, 1));
    }

    /// PH1 — THE RATIO STANDS EXACT ON THIS HOST AND THE FLOOR IS THE
    /// HOST'S: the reserve is a whole number of spends, the aperture is the
    /// two-stroke ratio itself, a spend halves exactly, and the stroke floor
    /// is a positive measurement every park is at least one of.
    #[test]
    fn spend_unit_ratio_exact_and_stroke_floor_positive() {
        assert_eq!(spend_unit() * STROKES_PER_RESERVE, reserve_max(), "a whole number of spends");
        assert_eq!(u64::from(aperture_cycle()), STROKES_PER_RESERVE, "the aperture is the ratio");
        assert!(spend_unit() >= 2, "the reserve's floor leaves a spend the regeneration can halve");
        let floor = stroke_floor();
        assert!(!floor.is_zero(), "the floor is a measurement, never zero");
        assert_eq!(idle_park(None), floor, "an unmeasured wire parks one stroke");
        assert_eq!(idle_park(Some(0)), floor, "a same-stroke reply still parks the stroke it parks");
        assert_eq!(idle_park(Some(3)), floor * 3, "a measured wire parks its round trip");
    }

    /// A measured round trip is counted in strokes, and a same-stroke reply
    /// reads zero, not a floor.
    #[test]
    fn from_transport_counts_rtt_in_strokes() {
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
    fn from_transport_overflow_is_beyond_ledger_not_unmeasured() {
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
