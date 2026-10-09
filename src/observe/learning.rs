// 78.1, 78.3, LC1 — THE LEARNING CURVE, one mind at one `K`: its perspective
// by content and what its closures did, read off the mind's own genome and
// served to the console. Honest at zero; a field no producer yet reads is
// OWED, by the row that owes it, never printed as zero.
//
//! The three coordinates of a perspective (74, 78.1) are carried by what
//! they are, never by an orbital label:
//!
//! - the DIALECT SPAN — every stratum of the mind's genome atlas, by its
//!   chart key, with the words it holds;
//! - the COVER — the readings the mind's charts are cut by (53.2). OWED: the
//!   roster holds one reading, Bands, and a genome chart does not record the
//!   reading it was cut by, so every mind would print the same constant — a
//!   default standing in for a measurement (rule 11);
//! - the PLANES READ — the voice's own [`super::Planes`] reading, read
//!   beside this one and never copied into it (law 12).
//!
//! What the closures did: the generators a closure opened in this genome
//! (`Held::generators_opened`, the reader a court's `Kernel::adjoined`
//! shares), the genome's words, and of those the ones whose form closes
//! (50.1: coheres, no remainder, no demand).

use crate::client::sdk::Epigenome;

/// The fields LC1 names that no producer reads yet, each with the row that
/// owes it. The ONE statement of what is owed (law 12), read by every
/// renderer: a row that lands its producer moves its field into
/// [`Learning`] and off this list.
/// The coboundary's agreement per grade is PG1's (`Progress`, the round kept
/// per grade), read beside this one and never copied into it (law 12).
pub const OWED: [(&str, &str); 4] = [
    ("cover", "53.2's readings per mind: K4, K5 widen the roster past Bands"),
    ("warm-cold delta", "Y1, re-taken on the fleet"),
    ("distinct perspectives per class", "CV3"),
    ("mass per lane per layer per grade", "E5 (MS1, Phase P)"),
];

/// ONE MIND'S CURVE AT ONE `K`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Learning {
    /// The mind's own `K` when read.
    pub k: u64,
    /// The dialect span: each stratum of the genome atlas, by its chart key,
    /// and its words, in the key's own order.
    pub strata: Vec<(Vec<u8>, u64)>,
    /// THE RADIUS (78.23 as amended): the deepest A21 prefix the span
    /// reaches — the most axes any stratum's chart carries, chart-space
    /// distance being prefix depth. Read with the span, once; the clock
    /// carries it from here.
    pub radius: u64,
    /// Generators a closure opened in this genome (I1's reading, per mind).
    pub adjoined_by_closure: u64,
    /// Words the genome holds: the court-ruled words taught to this mind.
    pub installed: u64,
    /// Of them, those whose form closes.
    pub closing: u64,
    /// CS2, 78.32(c) — this mind's pivots against the cells their orbits
    /// cross, by content: the count the admission gate waits on.
    pub pivots: crate::transducer::sentence::PivotTally,
    /// 13.12, 78.26(6) — the contemplation's merges: merged, stayed two,
    /// unnamed (TP1's line, which adds decohered and collapsed).
    pub merges: crate::transducer::bubble::merge::MergeTally,
    /// 78.37(1), TP1 — the meet graph as it stands at `k`, and its
    /// decoherences since the contemplation began.
    /// Boxed: its counts are the reading's bulk, and the Reading enum holds
    /// every kind at the size of its largest.
    pub meet: Box<crate::transducer::bubble::MeetGraph>,
    /// PR1 — what this voice's probes read, cumulative.
    pub probes: ProbeTally,
    /// 78.47 — why this mind did not move, by kind (78.9), cumulative: the
    /// input the demand arm's construction is read against.
    pub reasons: Reasons,
}

/// 78.47 — A MIND'S REFUSALS AND WHAT IT LEFT OPEN, BY KIND (P, D, C, I, N,
/// unclassified — `ObstructionKind::ALL`'s order): every refusal it formed
/// (the Gödel step's among them), the orbits its demand arm left
/// undischarged, and the perturbation reading (78.46(4′)) — cells
/// unperturbed, flat, curved, the vacuum circuits read in them, trial moves
/// admitted and refused unclosed, routes the register refused.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Reasons {
    pub refused: [u64; 6],
    pub undischarged: [u64; 6],
    pub perturbed: [u64; 7],
}

impl Reasons {
    /// This process's counters, read without clearing them.
    pub fn now() -> Reasons {
        let t = crate::transducer::sentence::perturb_tally();
        Reasons {
            refused: super::store::obstructed(),
            undischarged: super::store::undischarged(),
            perturbed: [t.unperturbed, t.flat, t.curved, t.vacua, t.admitted, t.unclosed, t.refused],
        }
    }

    pub fn line(&self) -> String {
        let [p, f, c, v, a, u, r] = self.perturbed;
        format!(
            "reasons: refused by kind (P D C I N ?) {:?} | undischarged by kind {:?} | cells unperturbed {p} flat {f} curved {c} (vacua {v}) | trial moves admitted {a} unclosed {u} | routes refused {r}",
            self.refused, self.undischarged
        )
    }

    fn put(&self, out: &mut Vec<u8>) {
        for v in self.refused.iter().chain(&self.undischarged).chain(&self.perturbed) {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Reasons> {
        let mut r = Reasons::default();
        for v in r.refused.iter_mut().chain(r.undischarged.iter_mut()).chain(r.perturbed.iter_mut()) {
            *v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
        }
        Some(r)
    }
}

/// PR1 — A VOICE'S PROBES, READ: how many left each closure state (closed,
/// residual, residual whose versor did not read, nothing, gone), the μ⃗
/// they settled and the μ⃗ they left remaining — each summed per lane, never
/// across — and the residual versors by their CLASS KEY (VA1's class
/// reading), each with its count: the classes met, bounded by the classes and
/// never a list of probes, the coordinate a court matches a residual
/// against. A residual versor with no class key is counted `unclassed`.
/// WHAT THIS VOICE'S SOLVES CAME TO, as the court ruled each (the actions'
/// physics): the task's FIRST closure (paid what the rule discharged), a
/// closure AGAIN (verified, paid nothing), REFUSED by the replay's kind
/// (P, D, C, I, N, unclassified — 78.9, in the reader's own order; an
/// unknown kind in the last place), and UNRULED (no verdict decoded); and
/// what the solves were paid, per lane.
///
/// 78.153c — and BY THE REPLY'S CODE, each code its own count (`by_code`, in
/// [`SOLVE_CODES`]' order, a code this build does not know last), and
/// UNREAD apart (`SOLVE_UNREAD`, a retry the court asks for, never a
/// refusal: 66 solves filed and none paid on 2026-10-09 read as "refused N"
/// with nothing to say which).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SolveTally {
    pub first: u64,
    pub again: u64,
    /// `SOLVE_NOTHING` with its refusal, by the refusal's kind
    /// ([`SOLVE_KINDS`]).
    pub refused: [u64; 7],
    /// `SOLVE_UNREAD`: not judged yet — asked again, the court resumes.
    pub unread: u64,
    /// Every verdict by the reply's code ([`SOLVE_CODES`]).
    pub by_code: [u64; 5],
    pub unruled: u64,
    pub paid: [u64; 4],
    /// Solves not sent, by why (`Entity::solve_task`): no rule minted; a
    /// rule its own train refutes; a rule the court already refused.
    pub unminted: u64,
    pub refuted: u64,
    pub withheld_again: u64,
}

/// 78.153c — THE SOLVE REPLY'S CODES BY NAME, in the wire's own order
/// (`SOLVE_NOTHING` 0, `SOLVE_FLOOR` 1, `SOLVE_BOUNTY` 2, `SOLVE_UNREAD` 3),
/// and a code this build does not know last.
pub const SOLVE_CODES: [&str; 5] = ["nothing", "floor", "bounty", "unread", "unknown code"];

/// A refusal's kind by name, in the byte's order on the wire
/// (`ObstructionKind::ALL`: P D C I N, unclassified `?`), and a kind byte
/// this build does not know last.
pub const SOLVE_KINDS: [&str; 7] = ["P", "D", "C", "I", "N", "?", "unknown kind"];

/// The name a code is read by.
pub fn solve_code_name(code: u8) -> &'static str {
    match SOLVE_CODES.get(usize::from(code)) {
        Some(n) => n,
        None => "unknown code",
    }
}

impl SolveTally {
    /// One verdict, folded in.
    pub fn note(&mut self, code: u8, mu: &[u64; 4], refusal: Option<&crate::anastome::dialogue::SolveRefusal>) {
        use crate::anastome::dialogue::{SOLVE_BOUNTY, SOLVE_FLOOR, SOLVE_UNREAD};
        let at = usize::from(code).min(self.by_code.len() - 1);
        if let Some(slot) = self.by_code.get_mut(at) {
            *slot += 1;
        }
        match (code, refusal) {
            (SOLVE_BOUNTY, _) => self.first += 1,
            (SOLVE_FLOOR, _) => self.again += 1,
            // A retry, never a refusal, whatever reason rides with it.
            (SOLVE_UNREAD, _) => self.unread += 1,
            (_, Some(r)) => {
                let place = usize::from(r.kind).min(self.refused.len() - 1);
                if let Some(slot) = self.refused.get_mut(place) {
                    *slot += 1;
                }
            }
            (_, None) => self.unruled += 1,
        }
        // A lane's total past its carrier stands where it was, counted by
        // the ledger (STD1).
        if let Some(total) = crate::core::lanes::lanes_add(self.paid, *mu) {
            self.paid = total;
        }
    }

    /// 78.153c — ONE VERDICT, AS ONE LINE: the task by its citation's head,
    /// the code by name and number, the refusal's kind by name (`none`
    /// where the reply carried no refusal) and its reason as the court
    /// wrote it.
    pub fn verdict_line(task: &[u8; 32], code: u8, refusal: Option<&crate::anastome::dialogue::SolveRefusal>) -> String {
        let head: String = task.iter().take(4).map(|b| format!("{b:02x}")).collect();
        let (kind, reason) = match refusal {
            Some(r) => {
                let kind = match SOLVE_KINDS.get(usize::from(r.kind)) {
                    Some(k) => *k,
                    None => "unknown kind",
                };
                (kind, format!(" · {}", String::from_utf8_lossy(&r.reason)))
            }
            None => ("none", String::new()),
        };
        format!("solve verdict · task {head}… · code {} ({code}) · kind {kind}{reason}", solve_code_name(code))
    }

    /// The tally by code, for the mind's `K` line.
    pub fn line(&self) -> String {
        let [nothing, floor, bounty, unread, unknown] = self.by_code;
        format!(
            "solves nothing {nothing} floor {floor} bounty {bounty} unread {unread} unknown {unknown} · refused (P D C I N ? x) {:?} unruled {} · kept home unminted {} refuted {} withheld {}",
            self.refused, self.unruled, self.unminted, self.refuted, self.withheld_again
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// 78.52 — WHAT THIS VOICE'S CLOSED THOUGHTS BECAME, read where each is
/// transcribed against the mind's own letters (78.47 (i)): SPELLED and
/// spoken as a connection; DERIVABLE only through a letter this mind
/// adjoined (said by that letter, which already stands); INEFFABLE — its
/// turn unspelled by every held letter, the thought kept, nothing emitted;
/// UNREAD — no rotation to spell.
pub struct TranscriptionTally {
    pub spelled: u64,
    pub adjoined: u64,
    pub ineffable: u64,
    pub unread: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProbeTally {
    pub closed: u64,
    pub residual: u64,
    pub residual_unread: u64,
    pub nothing: u64,
    pub gone: u64,
    pub unclassed: u64,
    pub settled: [u64; 4],
    pub remaining: [u64; 4],
    pub classes: Vec<(Vec<u8>, u64)>,
}

impl ProbeTally {
    /// One probe's reading, folded in.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 11 — an instrument's total (PR1, §8.1), read by no cognition and no price: the μ⃗ a voice's own probes settled and left, each a shortfall of a resident listing's measure (MEASURE_BOUND)"
    )]
    pub fn note(&mut self, r: &crate::anastome::dialogue::ProbeReading) {
        use crate::anastome::dialogue::ProbeClosure;
        match &r.closure {
            ProbeClosure::Closed => self.closed += 1,
            ProbeClosure::Residual(Some(v)) => {
                self.residual += 1;
                match v.class_key() {
                    Some(k) => match self.classes.binary_search_by(|(c, _)| c.as_slice().cmp(k.as_bytes())) {
                        Ok(at) => self.classes[at].1 += 1,
                        Err(at) => self.classes.insert(at, (k.as_bytes().to_vec(), 1)),
                    },
                    None => self.unclassed += 1,
                }
            }
            ProbeClosure::Residual(None) => {
                self.residual += 1;
                self.residual_unread += 1;
            }
            ProbeClosure::Nothing => self.nothing += 1,
            ProbeClosure::Gone => self.gone += 1,
        }
        for (acc, x) in self.settled.iter_mut().zip(&r.settled) {
            *acc = acc.saturating_add(*x);
        }
        for (acc, x) in self.remaining.iter_mut().zip(&r.remaining) {
            *acc = acc.saturating_add(*x);
        }
    }

    pub fn line(&self) -> String {
        format!(
            "probes: closed {} residual {} (versor unread {}, unclassed {}) nothing {} gone {} | settled {:?} remaining {:?} | residual classes {}",
            self.closed,
            self.residual,
            self.residual_unread,
            self.unclassed,
            self.nothing,
            self.gone,
            self.settled,
            self.remaining,
            self.classes.len()
        )
    }

    #[must_use]
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        for v in [self.closed, self.residual, self.residual_unread, self.nothing, self.gone, self.unclassed] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in self.settled.iter().chain(&self.remaining) {
            out.extend_from_slice(&v.to_le_bytes());
        }
        crate::transducer::address::put_u32_count(out, self.classes.len())?;
        for (k, n) in &self.classes {
            crate::transducer::address::put_bytes(out, k)?;
            out.extend_from_slice(&n.to_le_bytes());
        }
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<ProbeTally> {
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let mut t = ProbeTally::default();
        for v in [&mut t.closed, &mut t.residual, &mut t.residual_unread, &mut t.nothing, &mut t.gone, &mut t.unclassed] {
            *v = u64_at(b, p)?;
        }
        for v in t.settled.iter_mut().chain(t.remaining.iter_mut()) {
            *v = u64_at(b, p)?;
        }
        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
        *p += 4;
        // Each entry is at least a length and a count: 12 bytes.
        if b.len().checked_sub(*p)? < n.checked_mul(12)? {
            return None;
        }
        for _ in 0..n {
            let k = crate::transducer::address::take_bytes(b, p)?;
            t.classes.push((k, u64_at(b, p)?));
        }
        Some(t)
    }
}

impl Learning {
    /// THE CURVE OF THIS MIND, read off its own genome at its own `k`, at the
    /// read, with its contemplation's pivot reading beside it; nothing is
    /// stored beside the carriers (law 12).
    pub fn of(
        epigenome: &Epigenome,
        k: u64,
        pivots: crate::transducer::sentence::PivotTally,
        merges: crate::transducer::bubble::merge::MergeTally,
        meet: crate::transducer::bubble::MeetGraph,
        probes: ProbeTally,
    ) -> Learning {
        let mut radius = 0u64;
        let strata = epigenome
            .atlas
            .strata()
            .into_iter()
            .map(|(inv, members)| {
                radius = radius.max(inv.0.len() as u64);
                (inv.key().as_bytes().to_vec(), members.len() as u64)
            })
            .collect();
        let (mut installed, mut closing, mut adjoined_by_closure) = (0u64, 0u64, 0u64);
        for held in epigenome.genome_words() {
            installed += 1;
            adjoined_by_closure += held.generators_opened();
            if held.form().is_some_and(|f| f.is_closed()) {
                closing += 1;
            }
        }
        Learning {
            k,
            strata,
            radius,
            adjoined_by_closure,
            installed,
            closing,
            pivots,
            merges,
            meet: Box::new(meet),
            probes,
            reasons: Reasons::now(),
        }
    }

    /// `None` where a count or a key passes its field (STD1 3c): the reading
    /// is refused, never written short.
    #[must_use]
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.extend_from_slice(&self.k.to_le_bytes());
        out.extend_from_slice(&self.radius.to_le_bytes());
        crate::transducer::address::put_u32_count(out, self.strata.len())?;
        for (key, n) in &self.strata {
            crate::transducer::address::put_bytes(out, key)?;
            out.extend_from_slice(&n.to_le_bytes());
        }
        for v in [self.adjoined_by_closure, self.installed, self.closing] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let t = &self.pivots;
        for v in [t.reached, t.derivable, t.demanded, t.adjoin, t.starved, t.unread_fibre, t.unread_algebra] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let m = &self.merges;
        for v in [m.merged, m.stayed, m.unnamed] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        // TP1 — the meet graph, field by field in its own order.
        let g = &self.meet;
        for v in [g.vertices, g.thoughts, g.components] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in g.edges {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&g.cycle_rank.to_le_bytes());
        for v in g.rings.iter().chain(&g.orbits) {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let d = &g.decoherence;
        for v in d.decohered.iter().chain(&d.classed).chain(&d.rejoined).flatten() {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let t = &g.transport;
        for v in [
            g.advanced,
            g.sources,
            t.fixed[0],
            t.fixed[1],
            t.collapsed,
            t.collapsed_filled,
            t.held,
            t.classified,
            t.unsectioned,
            t.ungauged,
            t.unclassed,
            g.docket.pending,
            g.docket.open,
            g.docket.owed,
            g.docket.unwritten,
            g.docket.answered,
            g.docket.taken,
            g.docket.lapsed,
        ] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        // TC1 — the nerve's shape, its Ȟ¹ flagged where it did not read, and
        // the shedding doors.
        let c = &g.core;
        for v in [c.free_vertices, c.isolated, c.free_faces[0], c.free_faces[1], c.core, c.unfaced, c.malformed] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&c.h1.to_le_bytes());
        let s = &g.shed;
        for v in s.removed.iter().chain(&s.in_core) {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in s.rank_lost {
            out.extend_from_slice(&v.to_le_bytes());
        }
        let t = &g.truncation;
        for v in [t.truncated, t.held_by_class, t.glued, t.deferred] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&t.rank_moved.to_le_bytes());
        out.extend_from_slice(&t.malformed.to_le_bytes());
        // FX1 — the flux: a window or a count that did not read flagged.
        let f = &g.flux;
        let opt = |out: &mut Vec<u8>, x: Option<u64>| match x {
            Some(n) => {
                out.push(1);
                out.extend_from_slice(&n.to_le_bytes());
            }
            None => out.push(0),
        };
        for v in [f.closes, f.orbit.traversed, f.orbit.zero, f.orbit.held] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        match &f.aperture {
            Some(w) => {
                out.push(1);
                for v in [w.traversed, w.zero, w.held] {
                    out.extend_from_slice(&v.to_le_bytes());
                }
            }
            None => out.push(0),
        }
        for v in [f.gone, f.unfilamented, f.credited, f.acyclic, f.unfiled, f.unsealed, f.unnamed] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for (lanes, n) in f.extent.iter().zip(&f.extent_n) {
            for v in lanes.iter().chain(std::iter::once(n)) {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        out.extend_from_slice(&f.dialect.0.to_le_bytes());
        opt(out, f.dialect.1);
        opt(out, f.dialect.2);
        // RM1 — the mind's remainders.
        g.remainders.put(out)?;
        // STD1 — the sums the contemplation refused.
        out.extend_from_slice(&g.overflowed.to_le_bytes());
        // N4 — perception across scenes: circuits, cells, thoughts, demands.
        g.resonance.put(out);
        // PR1 — the voice's probes.
        self.probes.put(out)?;
        // 78.47 — its reasons by kind.
        self.reasons.put(out);
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Learning> {
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let u32_at = |b: &[u8], p: &mut usize| -> Option<usize> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v as usize)
        };
        let keyed = |b: &[u8], p: &mut usize| -> Option<Vec<(Vec<u8>, u64)>> {
            let m = u32_at(b, p)?;
            // Each entry is at least a length and a count; a count the body
            // cannot hold is refused before anything is reserved for it.
            if b.len().checked_sub(*p)? < m.checked_mul(12)? {
                return None;
            }
            let mut out = Vec::with_capacity(m);
            for _ in 0..m {
                let len = u32_at(b, p)?;
                let key = b.get(*p..*p + len)?.to_vec();
                *p += len;
                out.push((key, u64_at(b, p)?));
            }
            Some(out)
        };
        let k = u64_at(b, p)?;
        let radius = u64_at(b, p)?;
        let strata = keyed(b, p)?;
        let (adjoined_by_closure, installed, closing) = (u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?);
        let pivots = crate::transducer::sentence::PivotTally {
            reached: u64_at(b, p)?,
            derivable: u64_at(b, p)?,
            demanded: u64_at(b, p)?,
            adjoin: u64_at(b, p)?,
            starved: u64_at(b, p)?,
            unread_fibre: u64_at(b, p)?,
            unread_algebra: u64_at(b, p)?,
        };
        let merges = crate::transducer::bubble::merge::MergeTally {
            merged: u64_at(b, p)?,
            stayed: u64_at(b, p)?,
            unnamed: u64_at(b, p)?,
        };
        let six = |b: &[u8], p: &mut usize| -> Option<[u64; 6]> {
            let mut xs = [0u64; 6];
            for x in xs.iter_mut() {
                *x = u64_at(b, p)?;
            }
            Some(xs)
        };
        // A decoherence count: by cut (filled, congruent), then by adjunct.
        let two = |b: &[u8], p: &mut usize| -> Option<[[u64; 2]; 2]> {
            Some([[u64_at(b, p)?, u64_at(b, p)?], [u64_at(b, p)?, u64_at(b, p)?]])
        };
        let (vertices, thoughts, components) = (u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?);
        let edges = six(b, p)?;
        let cycle_rank = u64_at(b, p)? as i64;
        let (rings, orbits) = (six(b, p)?, six(b, p)?);
        let decoherence = crate::transducer::bubble::Decoherence {
            decohered: two(b, p)?,
            classed: two(b, p)?,
            rejoined: two(b, p)?,
        };
        let (advanced, sources) = (u64_at(b, p)?, u64_at(b, p)?);
        let transport = crate::transducer::bubble::TransportTally {
            fixed: [u64_at(b, p)?, u64_at(b, p)?],
            collapsed: u64_at(b, p)?,
            collapsed_filled: u64_at(b, p)?,
            held: u64_at(b, p)?,
            classified: u64_at(b, p)?,
            unsectioned: u64_at(b, p)?,
            ungauged: u64_at(b, p)?,
            unclassed: u64_at(b, p)?,
        };
        let docket = crate::transducer::bubble::DocketReading {
            pending: u64_at(b, p)?,
            open: u64_at(b, p)?,
            owed: u64_at(b, p)?,
            unwritten: u64_at(b, p)?,
            answered: u64_at(b, p)?,
            taken: u64_at(b, p)?,
            lapsed: u64_at(b, p)?,
        };
        let (free_vertices, isolated, faces_1, faces_2, core_size) =
            (u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?);
        let (unfaced, malformed) = (u64_at(b, p)?, u64_at(b, p)?);
        let h1 = u64_at(b, p)? as i64;
        let core = crate::transducer::bubble::CoreReading {
            free_vertices,
            isolated,
            free_faces: [faces_1, faces_2],
            core: core_size,
            h1,
            unfaced,
            malformed,
        };
        let three = |b: &[u8], p: &mut usize| -> Option<[u64; 3]> { Some([u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?]) };
        let (removed, in_core) = (three(b, p)?, three(b, p)?);
        let lost = three(b, p)?;
        let shed = crate::transducer::bubble::ShedTally {
            removed,
            in_core,
            rank_lost: [lost[0] as i64, lost[1] as i64, lost[2] as i64],
        };
        // TC1's fix — the truncation, its Ȟ¹ movement signed.
        let truncation = crate::transducer::bubble::Truncation {
            truncated: u64_at(b, p)?,
            held_by_class: u64_at(b, p)?,
            glued: u64_at(b, p)?,
            deferred: u64_at(b, p)?,
            rank_moved: u64_at(b, p)? as i64,
            malformed: u64_at(b, p)?,
        };
        // FX1 — the flux, as written.
        let flag = |b: &[u8], p: &mut usize| -> Option<bool> {
            let f = match b.get(*p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            *p += 1;
            Some(f)
        };
        let window = |b: &[u8], p: &mut usize| -> Option<crate::transducer::bubble::FluxWindow> {
            Some(crate::transducer::bubble::FluxWindow { traversed: u64_at(b, p)?, zero: u64_at(b, p)?, held: u64_at(b, p)? })
        };
        let mut flux = crate::transducer::bubble::FluxTally { closes: u64_at(b, p)?, orbit: window(b, p)?, ..Default::default() };
        if flag(b, p)? {
            flux.aperture = Some(window(b, p)?);
        }
        [flux.gone, flux.unfilamented, flux.credited, flux.acyclic, flux.unfiled, flux.unsealed, flux.unnamed] =
            [u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?];
        for (lanes, n) in flux.extent.iter_mut().zip(flux.extent_n.iter_mut()) {
            for l in lanes.iter_mut() {
                *l = u64_at(b, p)?;
            }
            *n = u64_at(b, p)?;
        }
        flux.dialect.0 = u64_at(b, p)?;
        if flag(b, p)? {
            flux.dialect.1 = Some(u64_at(b, p)?);
        }
        if flag(b, p)? {
            flux.dialect.2 = Some(u64_at(b, p)?);
        }
        let remainders = crate::transducer::cech::RemainderReading::take(b, p)?;
        let overflowed = u64_at(b, p)?;
        let resonance = crate::transducer::bubble::ResonanceTally::take(b, p)?;
        let meet = crate::transducer::bubble::MeetGraph {
            vertices,
            thoughts,
            components,
            edges,
            cycle_rank,
            rings,
            orbits,
            decoherence,
            advanced,
            sources,
            transport,
            docket,
            core,
            shed,
            truncation,
            flux,
            remainders,
            overflowed,
            resonance,
        };
        let probes = ProbeTally::take(b, p)?;
        let reasons = Reasons::take(b, p)?;
        Some(Learning {
            k,
            strata,
            radius,
            adjoined_by_closure,
            installed,
            closing,
            pivots,
            merges,
            meet: Box::new(meet),
            probes,
            reasons,
        })
    }
}
