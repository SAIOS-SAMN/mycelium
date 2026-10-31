use super::calibration::{expand_floor, regen_unit, reserve_max, spend_unit};
use super::{metabolism, Budget, Depleted, Reserve};

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
    /// the reservoirs (`Field::withheld`, `Mind::lock_reservoirs`) — and
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

    /// A pivot on `lane` — a torsion charge (the demand for a new fibre). An
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
    /// torsion charge: condensing a fibre's sections into a cycle converts
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
    /// negative: it is losing ground (a lane demanding a new fibre). The dynamic
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
