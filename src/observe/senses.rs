// THE FACULTIES, READ (2026-09-29, the operator's word: diagnostics beyond
// grep into the observer — learning, understanding, the anastome's language,
// cognition, speech, hearing and sight, for the minds and the servers).
//
//! Two typed readings, each a fixed table of named counters grouped by what
//! the organism does, so the console and a live-wire test read one name
//! table (law 12) and nothing is scraped from a log:
//!
//! - [`Senses`] — ONE MIND: what it SAW (perception across scenes), THOUGHT
//!   (thoughts held, grounded, closed, merged), SAID (transcription and
//!   utterance), HEARD (pages and words from its court), holds as LANGUAGE
//!   (dialect, genome), UNDERSTOOD (derivable, glued, obstructed), SOLVED
//!   (verdicts by kind, what they paid), and the size of each carrier that
//!   grows — the leak signals, read as counts and bytes at one `K`.
//! - [`Acts`] — ONE COURT: its reserve and its own ledger (drawn, restored,
//!   refused, per lane), the syllabus pages it served and where each ended,
//!   its fetch pages, and the solves it judged by verdict and kind.
//!
//! Every counter is cumulative where the producer accumulates it and read at
//! the question where it is a size; a later reading of one kind stands.

/// What a mind SAW: perception across scenes (78.48–78.52).
pub const SIGHT: [&str; 10] = [
    "circuits",
    "cells",
    "closes",
    "edges",
    "scene motions",
    "predicted hours",
    "consolidated",
    "reflexive",
    "refused by the gate",
    "demanded",
];
/// What a mind THOUGHT.
pub const COGNITION: [&str; 8] =
    ["thoughts held", "grounded", "closed", "merged", "torsion merges", "stayed two", "fills", "classes"];
/// What a mind SAID: its closed thoughts transcribed (78.52), and uttered.
pub const SPEECH: [&str; 6] = ["uttered", "spelled", "said by an adjoined letter", "ineffable", "unread", "colour theorems"];
/// 78.92 — WHAT A MIND'S ADMITTED SENTENCES CARRIED AS REASONS
/// (`sentence::ReasonsTally`, in its row's order).
pub const REASONS: [&str; 9] = [
    "sentences",
    "words",
    "built on held language",
    "generators cited",
    "new letters",
    "loops",
    "loops curved",
    "left unexplained",
    "cells unsaid",
];
/// What a mind HEARD from its court.
pub const HEARING: [&str; 3] = ["pages", "words heard", "words admitted"];
/// What a mind holds as LANGUAGE.
pub const LANGUAGE: [&str; 6] = ["dialect", "genome", "installed", "closing", "adjoined by closure", "colour words"];
/// What a mind UNDERSTOOD.
pub const UNDERSTANDING: [&str; 5] = ["derivable", "glued", "obstructed", "discordant strata", "colour derived"];
/// What a mind's solves came to (the actions' physics).
pub const SOLVING: [&str; 7] =
    ["first closures", "closures again", "refused", "unruled", "unminted", "refuted at home", "withheld again"];
/// THE GROWING THAT GROW — the leak signals: sizes at one `K`.
pub const GROWING: [&str; 9] = [
    "bubbles",
    "bubble key bytes",
    "bodies held",
    "pending rewrites",
    "grounds pending",
    "cavity",
    "docket open",
    "dialect",
    "genome",
];

/// ONE MIND'S FACULTIES AT ONE `K`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Senses {
    pub k: u64,
    pub sight: [u64; SIGHT.len()],
    pub cognition: [u64; COGNITION.len()],
    pub speech: [u64; SPEECH.len()],
    /// 78.92 — the reasons its admitted sentences carried, cumulative, and
    /// the last one's whole: what it said with, what its loops mean, what it
    /// could not explain, what it left unsaid.
    pub reasons: [u64; REASONS.len()],
    pub said: Option<crate::transducer::sentence::Reasons>,
    /// 78.95 — what each word vertex's derivation came to against the mind's
    /// own language (`family::DERIVATION`), and what it perceived as flux
    /// (`family::flux::FLUX`).
    pub derivation: [u64; crate::transducer::family::DERIVATION.len()],
    pub flux: [u64; crate::transducer::family::flux::FLUX.len()],
    /// 78.96 — the sentences its rings composed: closed, open by why, the
    /// theorems, the shortcuts and the top rung (`rung::SENTENCES`).
    pub sentences: [u64; crate::transducer::sentence::rung::SENTENCES.len()],
    /// 78.99 — its construction: the holes its nerve punctured and capped,
    /// what each glue of two thoughts read, and the words it relayed as
    /// coordinates (`cech::CONSTRUCTION`).
    pub construction: [u64; crate::transducer::cech::CONSTRUCTION.len()],
    /// 78.100 — why the language loses what it perceives
    /// (`rung::WHY_UNREAD`).
    pub unread: [u64; crate::transducer::sentence::rung::WHY_UNREAD.len()],
    /// 78.100 D6 — what deconstruction by motion read
    /// (`rung::CITATION`).
    pub citation: [u64; crate::transducer::sentence::rung::CITATION.len()],
    /// 78.110 E2 — what the encounters where circuits meet read
    /// (`cech::ENCOUNTER`).
    pub encounter: [u64; crate::transducer::cech::ENCOUNTER.len()],
    /// 78.109 C1 — what the mind's constructions came to
    /// (`cga::CONSTRUCTIONS`).
    pub constructions: [u64; crate::transducer::cga::CONSTRUCTIONS.len()],
    pub hearing: [u64; HEARING.len()],
    pub language: [u64; LANGUAGE.len()],
    pub understanding: [u64; UNDERSTANDING.len()],
    pub solving: [u64; SOLVING.len()],
    /// What the solves were paid, per lane.
    pub paid: [u64; 4],
    pub carriers: [u64; GROWING.len()],
}

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
pub const FETCH: [&str; 13] = [
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
];
/// 78.102, 47.3 — THE ROAD A PROVISIONAL WORD TAKES TO THE GENOME, where it
/// stops: a letter docketed from a sentence heard (heard again, the voice
/// converges on the same address as a party), every provisional word offered to
/// the witness hyphae covering its chart, and what each witness answered —
/// its re-derivation reached and the word installed, reached inexactly, or
/// still awaiting a second vantage; refused; unmeasured.
pub const PROVISIONAL: [&str; 8] = [
    "letters docketed",
    "letters heard again (converged as a party)",
    "offers sent to witnesses",
    "witness reached: installed",
    "witness reached: inexact",
    "witness reached: awaiting a vantage",
    "witness refused",
    "witness unmeasured",
];
/// What a court's solves came to: by verdict, refusals by kind (P D C I N ?).
pub const JUDGED: [&str; 10] = [
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
];

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
    /// THE WORDS THE RESERVE KEEPS FROM BEING TAUGHT (the operator,
    /// 2026-09-30): each installed word a syllabus page deferred, what it
    /// is, what it costs, where the reserve fell short, and how often it was
    /// tried. "Deferred: reserve" counts attempts; a
    /// few words retried by every page to every voice read as thousands.
    /// In address order.
    pub deferred: Vec<Deferred>,
    /// 78.102, 47.3 — the road a provisional word takes to the genome
    /// ([`PROVISIONAL`]).
    pub provisional: [u64; PROVISIONAL.len()],
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

impl Senses {
    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        put(out, &[self.k]);
        put(out, &self.sight);
        put(out, &self.cognition);
        put(out, &self.speech);
        put(out, &self.reasons);
        match &self.said {
            None => out.push(0),
            Some(r) => {
                out.push(1);
                r.put(out);
            }
        }
        put(out, &self.derivation);
        put(out, &self.flux);
        put(out, &self.sentences);
        put(out, &self.construction);
        put(out, &self.unread);
        put(out, &self.citation);
        put(out, &self.encounter);
        put(out, &self.constructions);
        put(out, &self.hearing);
        put(out, &self.language);
        put(out, &self.understanding);
        put(out, &self.solving);
        put(out, &self.paid);
        put(out, &self.carriers);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Senses> {
        let [k] = take::<1>(b, p)?;
        let (sight, cognition, speech, reasons) = (take(b, p)?, take(b, p)?, take(b, p)?, take(b, p)?);
        let flag = *b.get(*p)?;
        *p += 1;
        let said = match flag {
            0 => None,
            1 => Some(crate::transducer::sentence::Reasons::take(b, p)?),
            _ => return None,
        };
        let (derivation, flux, sentences, construction, unread) = (take(b, p)?, take(b, p)?, take(b, p)?, take(b, p)?, take(b, p)?);
        Some(Senses {
            k,
            sight,
            cognition,
            speech,
            reasons,
            said,
            derivation,
            flux,
            sentences,
            construction,
            unread,
            citation: take(b, p)?,
            encounter: take(b, p)?,
            constructions: take(b, p)?,
            hearing: take(b, p)?,
            language: take(b, p)?,
            understanding: take(b, p)?,
            solving: take(b, p)?,
            paid: take(b, p)?,
            carriers: take(b, p)?,
        })
    }

    /// The reading, faculty by faculty, named.
    pub fn lines(&self) -> Vec<String> {
        let mut out = vec![
            format!("K {}", self.k),
            format!("sight: {}", line(&SIGHT, &self.sight)),
            format!("cognition: {}", line(&COGNITION, &self.cognition)),
            format!("speech: {}", line(&SPEECH, &self.speech)),
            format!("reasons: {}", line(&REASONS, &self.reasons)),
        ];
        // 78.92 — the last admitted sentence, in its own words.
        if let Some(r) = &self.said {
            out.extend(r.says().into_iter().map(|s| format!("  last said: {s}")));
        }
        out.push(format!("derivation: {}", line(&crate::transducer::family::DERIVATION, &self.derivation)));
        out.push(format!("flux: {}", line(&crate::transducer::family::flux::FLUX, &self.flux)));
        out.push(format!("sentences: {}", line(&crate::transducer::sentence::rung::SENTENCES, &self.sentences)));
        out.push(format!("construction: {}", line(&crate::transducer::cech::CONSTRUCTION, &self.construction)));
        out.push(format!("unread: {}", line(&crate::transducer::sentence::rung::WHY_UNREAD, &self.unread)));
        out.push(format!("citation: {}", line(&crate::transducer::sentence::rung::CITATION, &self.citation)));
        out.push(format!("encounter: {}", line(&crate::transducer::cech::ENCOUNTER, &self.encounter)));
        out.push(format!("constructions: {}", line(&crate::transducer::cga::CONSTRUCTIONS, &self.constructions)));
        out.extend([
            format!("hearing: {}", line(&HEARING, &self.hearing)),
            format!("language: {}", line(&LANGUAGE, &self.language)),
            format!("understanding: {}", line(&UNDERSTANDING, &self.understanding)),
            format!("solving: {} · paid {:?}", line(&SOLVING, &self.solving), self.paid),
            format!("carriers: {}", line(&GROWING, &self.carriers)),
        ]);
        out
    }

    /// The counter under `name` in the faculty `faculty`, read by name.
    pub fn read(&self, faculty: &str, name: &str) -> Option<u64> {
        fn at<const N: usize>(names: &[&str; N], xs: &[u64; N], name: &str) -> Option<u64> {
            names.iter().zip(xs).find(|(n, _)| **n == name).map(|(_, x)| *x)
        }
        match faculty {
            "sight" => at(&SIGHT, &self.sight, name),
            "cognition" => at(&COGNITION, &self.cognition, name),
            "speech" => at(&SPEECH, &self.speech, name),
            "reasons" => at(&REASONS, &self.reasons, name),
            "derivation" => at(&crate::transducer::family::DERIVATION, &self.derivation, name),
            "flux" => at(&crate::transducer::family::flux::FLUX, &self.flux, name),
            "sentences" => at(&crate::transducer::sentence::rung::SENTENCES, &self.sentences, name),
            "construction" => at(&crate::transducer::cech::CONSTRUCTION, &self.construction, name),
            "unread" => at(&crate::transducer::sentence::rung::WHY_UNREAD, &self.unread, name),
            "citation" => at(&crate::transducer::sentence::rung::CITATION, &self.citation, name),
            "encounter" => at(&crate::transducer::cech::ENCOUNTER, &self.encounter, name),
            "constructions" => at(&crate::transducer::cga::CONSTRUCTIONS, &self.constructions, name),
            "hearing" => at(&HEARING, &self.hearing, name),
            "language" => at(&LANGUAGE, &self.language, name),
            "understanding" => at(&UNDERSTANDING, &self.understanding, name),
            "solving" => at(&SOLVING, &self.solving, name),
            "carriers" => at(&GROWING, &self.carriers, name),
            _ => None,
        }
    }
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
        Some(Acts { k, reserve, drawn, restored, refused, syllabus, fetch, judged, paid, colours, colours_derived, deferred, provisional })
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
}

/// COUNT ONE EVENT on a named counter of a table — the name is the table's
/// own, so a producer and every reader name one slot (law 12). A name the
/// table does not hold counts nothing; the tests pin every name used.
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
