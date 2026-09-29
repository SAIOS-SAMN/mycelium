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
    pub hearing: [u64; HEARING.len()],
    pub language: [u64; LANGUAGE.len()],
    pub understanding: [u64; UNDERSTANDING.len()],
    pub solving: [u64; SOLVING.len()],
    /// What the solves were paid, per lane.
    pub paid: [u64; 4],
    pub carriers: [u64; GROWING.len()],
}

/// What a court's syllabus pages came to.
pub const SYLLABUS: [&str; 7] =
    ["pages", "words served", "words missed", "ended: order", "ended: frame", "ended: count", "deferred: reserve"];
/// What a court's fetch pages came to.
pub const FETCH: [&str; 2] = ["words served", "deferred: reserve"];
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
    pub refused: u64,
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
}

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
        put(out, &self.hearing);
        put(out, &self.language);
        put(out, &self.understanding);
        put(out, &self.solving);
        put(out, &self.paid);
        put(out, &self.carriers);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Senses> {
        let [k] = take::<1>(b, p)?;
        Some(Senses {
            k,
            sight: take(b, p)?,
            cognition: take(b, p)?,
            speech: take(b, p)?,
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
        vec![
            format!("K {}", self.k),
            format!("sight: {}", line(&SIGHT, &self.sight)),
            format!("cognition: {}", line(&COGNITION, &self.cognition)),
            format!("speech: {}", line(&SPEECH, &self.speech)),
            format!("hearing: {}", line(&HEARING, &self.hearing)),
            format!("language: {}", line(&LANGUAGE, &self.language)),
            format!("understanding: {}", line(&UNDERSTANDING, &self.understanding)),
            format!("solving: {} · paid {:?}", line(&SOLVING, &self.solving), self.paid),
            format!("carriers: {}", line(&GROWING, &self.carriers)),
        ]
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
        put(out, &[self.refused]);
        put(out, &self.syllabus);
        put(out, &self.fetch);
        put(out, &self.judged);
        put(out, &self.paid);
        put(out, &[self.colours, self.colours_derived]);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Acts> {
        let [k] = take::<1>(b, p)?;
        let (reserve, drawn, restored) = (take(b, p)?, take(b, p)?, take(b, p)?);
        let [refused] = take::<1>(b, p)?;
        let (syllabus, fetch, judged, paid) = (take(b, p)?, take(b, p)?, take(b, p)?, take(b, p)?);
        let [colours, colours_derived] = take::<2>(b, p)?;
        Some(Acts { k, reserve, drawn, restored, refused, syllabus, fetch, judged, paid, colours, colours_derived })
    }

    /// The reading, act by act, named.
    pub fn lines(&self) -> Vec<String> {
        vec![
            format!(
                "K {} · reserve {:?} · drawn {:?} · restored {:?} · draws refused {}",
                self.k, self.reserve, self.drawn, self.restored, self.refused
            ),
            format!("syllabus: {}", line(&SYLLABUS, &self.syllabus)),
            format!("fetch: {}", line(&FETCH, &self.fetch)),
            format!("solves: {} · paid {:?}", line(&JUDGED, &self.judged), self.paid),
            format!("colour words installed: {} · admitted by derivation: {}", self.colours, self.colours_derived),
        ]
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
}

/// COUNT ONE EVENT on a named counter of a table — the name is the table's
/// own, so a producer and every reader name one slot (law 12). A name the
/// table does not hold counts nothing; the tests pin every name used.
pub fn count<const N: usize>(slots: &mut [u64; N], names: &[&str; N], name: &str) {
    if let Some((slot, _)) = slots.iter_mut().zip(names).find(|(_, n)| **n == name) {
        *slot += 1;
    }
}
