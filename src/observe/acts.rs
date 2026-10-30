// A COURT'S ACTS, READ (ruled 2026-09-29: the servers' diagnostics are
// carried into the observer rather than read by grep).
//
//! [`Acts`] — ONE COURT, a fixed table of named counters grouped by what it
//! does, so the console reads one name table (law 12) and nothing is scraped
//! from a log: its reserve and its own ledger (drawn, restored, refused, per
//! lane), the syllabus pages it served and where each ended, its fetch
//! pages, and the solves it judged by verdict and kind.
//!
//! Every counter is cumulative where the producer accumulates it and read at
//! the question where it is a size; a later reading of one kind stands.

/// What a court's syllabus pages came to.
pub const SYLLABUS: [&str; 13] = [
    "pages",
    "words served",
    "words missed",
    "ended: order",
    "ended: frame",
    "ended: count",
    "deferred: reserve",
    // 78.60 — each sentence taught, by the strongest relation it stands in
    // to a word this court already taught its learner on its chart: the
    // held word refracted by one Γ IS it (relayable as (held, Γ)); one
    // spelled word conjugates every vertex and the carriage, gauge or a
    // citation differs; one rotor does and the alphabet cannot spell it (a
    // generator's obstruction); a rotor per vertex does (a coboundary δγ);
    // the chart alone is shared; or nothing is.
    "sentences: refracted (held, Γ)",
    "sentences: conjugate, carriage apart",
    "sentences: unspellable conjugator",
    "sentences: rotor per vertex δγ",
    "sentences: chart only",
    "sentences: unanchored",
];
/// What a court's fetch pages came to — and, 78.100 PS1 (2), the reading
/// before the court-to-court relay of generators (78.61 C4): each word served
/// to a peer court, by the strongest relation it stands in to a word that
/// court said it holds on the word's chart (the same six as the syllabus's),
/// and the bytes served whole against the bytes the refracted ones would
/// take as (held, Γ) — the anchor's address and the transport's word,
/// carriage and gauge, each by its own codec.
pub const FETCH: [&str; 19] = [
    "served to courts",
    "served to voices",
    // 78.100 — the fetch paid by who asks: a peer court's from the serving
    // court's reserve, a voice's from its own account (78.60(d)).
    "deferred: reserve",
    "deferred: learner short",
    "words: refracted (held, Γ)",
    "words: conjugate, carriage apart",
    "words: unspellable conjugator",
    "words: rotor per vertex δγ",
    "words: chart only",
    "words: unanchored",
    "bytes served",
    "bytes refracted, whole",
    "bytes refracted, as (held, Γ)",
    // 78.160 P1 — a mind's residue deconstructed over this court's
    // presentation: asked, its coordinates unread, the generators cited, the
    // asks whose cited powers compose to the motion exactly, the words taught
    // for it, and the learners too short to be taught one.
    "presentation: asked",
    "presentation: unread",
    "presentation: generators cited",
    "presentation: composes",
    "presentation: words served",
    "presentation: learner short",
];
/// 78.102, 47.3 — THE ROAD A PROVISIONAL WORD TAKES TO THE GENOME, where it
/// stops: a letter docketed from a sentence heard (heard again, the voice
/// converges on the same address as a party), every provisional word offered to
/// the witness hyphae covering its chart — or emitted with no hypha covering
/// it, or asked of one that never answered — and what each witness answered —
/// its re-derivation reached and the word installed, reached inexactly, or
/// still awaiting a second vantage; refused; unmeasured.
pub const PROVISIONAL: [&str; 10] = [
    "letters docketed",
    "letters heard again (converged as a party)",
    "offers sent to witnesses",
    "offers with no eligible witness",
    "offers a witness left unanswered",
    "witness reached: installed",
    "witness reached: inexact",
    "witness reached: awaiting a vantage",
    "witness refused",
    "witness unmeasured",
];
/// MU-CLIENT §5.6 — WHAT A COURT PAID, BY KIND (Phase 2): the speech it heard
/// under a scene it holds no family for (measured on nothing, named), and each
/// pay by the measure that read it — a word's section (W), a conjugate's
/// transport (C), a sentence's gluing (S), a demand closed by another mind
/// (POWC, E2), an attesting mind joined to a word's claim (E3), a mint
/// counted toward a universe's growth grant (E4). The observer's diagnosis;
/// the witness is the ledger's mints.
pub const PAYS: [&str; 7] = [
    "scenes on no family held",
    "word sections",
    "conjugate transports",
    "sentence gluings",
    "demands closed by another mind",
    "attestations joined to a claim",
    "mints counted toward a grant",
];
/// What a court's solves came to: by verdict, refusals by kind (P D C I N ?),
/// and (78.153c) the verdicts 78.149 and 78.153 added — NO WORK (a clean
/// replay that discharged nothing) and UNREAD (a stroke that ran out before
/// the replay ruled) — and the door's refusals before any judging: unsigned,
/// no such task, no rule carried; and, BESIDE them and refusing nothing, the
/// solves an inner seat of a founded domain signed and was served (ruled
/// 2026-10-14: the court authenticates keys, never tiers).
/// Named through [`Judged`], so no verdict has a name the table lacks.
pub const JUDGED: [&str; 16] = [
    "first closures",
    "closures again",
    "refused P",
    "refused D",
    "refused C",
    "refused I",
    "refused N",
    "refused ?",
    "not a rule",
    "unverifiable",
    "no work",
    "unread",
    "unsigned",
    "signed by an inner seat (served)",
    "no such task",
    "no rule carried",
];

/// 78.153c — ONE SOLVE VERDICT, AS THE COURT COUNTS IT: each variant is one
/// slot of [`JUDGED`] by an exhaustive match, so a verdict the table has no
/// slot for does not compile, where a name it did not hold was dropped
/// (every 78.149 "no work" was, until 78.153c).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Judged {
    FirstClosure,
    ClosureAgain,
    Refused(crate::transducer::sentence::ObstructionKind),
    NotARule,
    Unverifiable,
    NoWork,
    Unread,
    /// The door's, before judging: the solve names no payee.
    Unsigned,
    /// Counted at the door and REFUSING NOTHING: the solve was signed by an
    /// inner seat of a founded domain (`inner_seat`), and is judged and paid
    /// as any key's.
    InnerSeat,
    /// The door's: the citation names no task on this book.
    NoSuchTask,
    /// The door's: the solve carried no rule (or one that did not parse).
    NoRuleCarried,
}

impl Judged {
    /// Every verdict, one per slot, in [`JUDGED`]'s order.
    pub const ALL: [Judged; JUDGED.len()] = {
        use crate::transducer::sentence::ObstructionKind as K;
        [
            Judged::FirstClosure,
            Judged::ClosureAgain,
            Judged::Refused(K::P),
            Judged::Refused(K::D),
            Judged::Refused(K::C),
            Judged::Refused(K::I),
            Judged::Refused(K::N),
            Judged::Refused(K::Unclassified),
            Judged::NotARule,
            Judged::Unverifiable,
            Judged::NoWork,
            Judged::Unread,
            Judged::Unsigned,
            Judged::InnerSeat,
            Judged::NoSuchTask,
            Judged::NoRuleCarried,
        ]
    };

    /// Its slot in [`JUDGED`].
    pub const fn slot(self) -> usize {
        use crate::transducer::sentence::ObstructionKind as K;
        match self {
            Judged::FirstClosure => 0,
            Judged::ClosureAgain => 1,
            Judged::Refused(K::P) => 2,
            Judged::Refused(K::D) => 3,
            Judged::Refused(K::C) => 4,
            Judged::Refused(K::I) => 5,
            Judged::Refused(K::N) => 6,
            Judged::Refused(K::Unclassified) => 7,
            Judged::NotARule => 8,
            Judged::Unverifiable => 9,
            Judged::NoWork => 10,
            Judged::Unread => 11,
            Judged::Unsigned => 12,
            Judged::InnerSeat => 13,
            Judged::NoSuchTask => 14,
            Judged::NoRuleCarried => 15,
        }
    }

    /// Its name, the table's own.
    pub fn name(self) -> &'static str {
        match JUDGED.get(self.slot()) {
            Some(n) => n,
            None => "an unslotted verdict",
        }
    }
}

/// ONE COURT'S ACTS AND ITS METABOLISM.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Acts {
    pub k: u64,
    /// The reserve per lane as it stands, and its own ledger.
    pub reserve: [u64; 4],
    pub drawn: [u64; 4],
    pub restored: [u64; 4],
    /// Draws refused, by the lane that refused each (78.67).
    pub refused: [u64; 4],
    pub syllabus: [u64; SYLLABUS.len()],
    pub fetch: [u64; FETCH.len()],
    pub judged: [u64; JUDGED.len()],
    /// What the solves were paid, per lane.
    pub paid: [u64; 4],
    /// 78.53 — the colour words this court has installed: the ruled set
    /// every mind it teaches converges on.
    pub colours: u64,
    /// 78.55 — of the colour words it heard, those admitted by derivation
    /// from its ruled colour words rather than by a scene's evidence.
    pub colours_derived: u64,
    /// THE WORDS THE RESERVE KEEPS FROM BEING TAUGHT (ruled
    /// 2026-09-30): each installed word a syllabus page deferred, what it
    /// is, what it costs, where the reserve fell short, and how often it was
    /// tried. "Deferred: reserve" counts attempts; a
    /// few words retried by every page to every voice read as thousands.
    /// In address order.
    pub deferred: Vec<Deferred>,
    /// 78.102, 47.3 — the road a provisional word takes to the genome
    /// ([`PROVISIONAL`]).
    pub provisional: [u64; PROVISIONAL.len()],
    /// MU-CLIENT §5.6 — each [`PAYS`] kind counted, and what it paid per lane
    /// (four lanes a kind, in [`PAYS`]'s order).
    pub pays: [u64; PAYS.len()],
    pub pays_lanes: [u64; PAYS.len() * 4],
}

/// The kinds a deferred word is read as, on [`Deferred::kind`].
pub const DEFERRED_KINDS: [&str; 4] = ["residual word", "connection", "sentence", "colour word"];

/// ONE WORD THE RESERVE KEPT FROM A SYLLABUS PAGE.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Deferred {
    /// The head of its address: enough to name it on the console, never a
    /// key (the address itself stays the court's).
    pub head: [u8; 8],
    /// Its kind, on [`DEFERRED_KINDS`].
    pub kind: u8,
    /// Its magnitude per lane — the cost of teaching it whole.
    pub mag: [u64; 4],
    /// Where its payer fell short of it, per lane, at the last try.
    pub short: [u64; 4],
    /// Its payer at the last try: the head of the learner whose balance
    /// fell short (a sentence, paid by its learner), or `None` — the
    /// court's reserve.
    pub learner: Option<[u8; 8]>,
    /// Pages that tried it and could not pay.
    pub tries: u64,
}

/// How many deferred words the reading carries: what its one-byte count
/// holds, in address order.
pub const DEFERRED_SHOWN: usize = u8::MAX as usize;

fn put<const N: usize>(out: &mut Vec<u8>, xs: &[u64; N]) {
    for x in xs {
        out.extend_from_slice(&x.to_le_bytes());
    }
}

fn take<const N: usize>(b: &[u8], p: &mut usize) -> Option<[u64; N]> {
    let mut xs = [0u64; N];
    for x in xs.iter_mut() {
        *x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
    }
    Some(xs)
}

fn line<const N: usize>(names: &[&str; N], xs: &[u64; N]) -> String {
    names.iter().zip(xs).map(|(n, x)| format!("{n} {x}")).collect::<Vec<_>>().join(" · ")
}

impl Acts {
    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        put(out, &[self.k]);
        put(out, &self.reserve);
        put(out, &self.drawn);
        put(out, &self.restored);
        put(out, &self.refused);
        put(out, &self.syllabus);
        put(out, &self.fetch);
        put(out, &self.judged);
        put(out, &self.paid);
        put(out, &[self.colours, self.colours_derived]);
        put(out, &self.provisional);
        // At most DEFERRED_SHOWN rows, so the count is one byte.
        let shown = &self.deferred[..self.deferred.len().min(DEFERRED_SHOWN)];
        #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — a constant: at most DEFERRED_SHOWN = u8::MAX rows, sliced above")]
        out.push(shown.len() as u8);
        for d in shown {
            out.extend_from_slice(&d.head);
            out.push(d.kind);
            put(out, &d.mag);
            put(out, &d.short);
            put(out, &[d.tries]);
            match d.learner {
                None => out.push(0),
                Some(v) => {
                    out.push(1);
                    out.extend_from_slice(&v);
                }
            }
        }
        // Phase 2's pays, after the rows: a reading written before them ends
        // at the rows and reads as none paid.
        put(out, &self.pays);
        put(out, &self.pays_lanes);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Acts> {
        let [k] = take::<1>(b, p)?;
        let (reserve, drawn, restored) = (take(b, p)?, take(b, p)?, take(b, p)?);
        let refused = take(b, p)?;
        let (syllabus, fetch, judged, paid) = (take(b, p)?, take(b, p)?, take(b, p)?, take(b, p)?);
        let [colours, colours_derived] = take::<2>(b, p)?;
        let provisional = take(b, p)?;
        let n = usize::from(*b.get(*p)?);
        *p += 1;
        let mut deferred = Vec::with_capacity(n.min(DEFERRED_SHOWN));
        for _ in 0..n {
            let head: [u8; 8] = b.get(*p..*p + 8)?.try_into().ok()?;
            *p += 8;
            let kind = *b.get(*p)?;
            *p += 1;
            let (mag, short) = (take(b, p)?, take(b, p)?);
            let [tries] = take::<1>(b, p)?;
            let flag = *b.get(*p)?;
            *p += 1;
            let learner = match flag {
                0 => None,
                1 => {
                    let v: [u8; 8] = b.get(*p..*p + 8)?.try_into().ok()?;
                    *p += 8;
                    Some(v)
                }
                _ => return None,
            };
            deferred.push(Deferred { head, kind, mag, short, tries, learner });
        }
        // A reading that ends at its rows was written before Phase 2's pays:
        // none read, never a short read past the end.
        let (pays, pays_lanes) = match b.get(*p..) {
            Some([]) | None => ([0; PAYS.len()], [0; PAYS.len() * 4]),
            Some(_) => (take(b, p)?, take(b, p)?),
        };
        Some(Acts { k, reserve, drawn, restored, refused, syllabus, fetch, judged, paid, colours, colours_derived, deferred, provisional, pays, pays_lanes })
    }

    /// The reading, act by act, named.
    pub fn lines(&self) -> Vec<String> {
        vec![
            format!(
                "K {} · reserve {:?} · drawn {:?} · restored {:?} · draws refused {:?}",
                self.k, self.reserve, self.drawn, self.restored, self.refused
            ),
            format!("syllabus: {}", line(&SYLLABUS, &self.syllabus)),
            format!("fetch: {}", line(&FETCH, &self.fetch)),
            format!("solves: {} · paid {:?}", line(&JUDGED, &self.judged), self.paid),
            format!("colour words installed: {} · admitted by derivation: {}", self.colours, self.colours_derived),
            format!("provisional: {}", line(&PROVISIONAL, &self.provisional)),
            format!(
                "pays: {}",
                PAYS.iter()
                    .enumerate()
                    .map(|(i, n)| {
                        let lanes: Vec<u64> = self.pays_lanes.iter().skip(i * 4).take(4).copied().collect();
                        format!("{n} {} {lanes:?}", self.pays[i])
                    })
                    .collect::<Vec<_>>()
                    .join(" · ")
            ),
        ]
        .into_iter()
        .chain(self.deferred.iter().map(|d| {
            let kind = match DEFERRED_KINDS.get(usize::from(d.kind)) {
                Some(k) => *k,
                None => "an unnamed kind",
            };
            let payer = match d.learner {
                None => "the reserve".to_string(),
                Some(v) => format!("learner {v:02x?}…"),
            };
            format!(
                "deferred {:02x?}… {kind} · cost {:?} · {payer} short {:?} · tried {}",
                d.head, d.mag, d.short, d.tries
            )
        }))
        .collect()
    }
}

/// A COURT'S ACTS AS THEY HAPPEN: the counters its handlers keep, read into
/// [`Acts`] with its reserve at the question.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ActsTally {
    pub syllabus: [u64; SYLLABUS.len()],
    pub fetch: [u64; FETCH.len()],
    pub judged: [u64; JUDGED.len()],
    pub paid: [u64; 4],
    /// 78.55 — colour words this court admitted by their derivation from
    /// ruled colour words, no scene witnessing them.
    pub colours_derived: u64,
    pub provisional: [u64; PROVISIONAL.len()],
    /// MU-CLIENT §5.6 — [`PAYS`], counted and per lane.
    pub pays: [u64; PAYS.len()],
    pub pays_lanes: [u64; PAYS.len() * 4],
}

impl ActsTally {
    /// MU-CLIENT §5.6 — ONE PAY OF A KIND, counted with what it paid per lane.
    /// A kind [`PAYS`] does not name counts nothing; a lane past its carrier
    /// is refused where it stands (STD1).
    pub fn pay(&mut self, kind: &str, mu: [u64; 4]) {
        let Some(i) = PAYS.iter().position(|n| *n == kind) else { return };
        if let Some(slot) = self.pays.get_mut(i) {
            *slot += 1;
        }
        for (l, m) in mu.iter().enumerate() {
            if let Some(slot) = self.pays_lanes.get_mut(i * 4 + l) {
                let _ = crate::physics::metabolism::step_or_refuse(slot, *m);
            }
        }
    }
}

impl ActsTally {
    /// 78.153c — ONE VERDICT COUNTED on its slot: a [`Judged`] always has
    /// one (the tests pin `slot` against [`JUDGED`]), so none is dropped.
    pub fn judge(&mut self, verdict: Judged) {
        if let Some(slot) = self.judged.get_mut(verdict.slot()) {
            *slot += 1;
        }
    }
}

/// COUNT ONE EVENT on a named counter of a table — the name is the table's
/// own, so a producer and every reader name one slot (law 12). A name the
/// table does not hold counts nothing; the tests pin every name used. A
/// table whose producers can name an unheld slot is counted through a typed
/// verdict instead (78.153c: the court's "no work" fell there unseen, so the
/// court's solves count through [`Judged`]).
pub fn count<const N: usize>(slots: &mut [u64; N], names: &[&str; N], name: &str) {
    if let Some((slot, _)) = slots.iter_mut().zip(names).find(|(_, n)| **n == name) {
        *slot += 1;
    }
}

/// The counter under `name` stepped by `by` — a measure, bytes or mass, not
/// one event — or the step refused and counted where it would pass the
/// carrier (STD1).
pub fn count_by<const N: usize>(slots: &mut [u64; N], names: &[&str; N], name: &str, by: u64) {
    if let Some((slot, _)) = slots.iter_mut().zip(names).find(|(_, n)| **n == name) {
        let _ = crate::physics::metabolism::step_or_refuse(slot, by);
    }
}
