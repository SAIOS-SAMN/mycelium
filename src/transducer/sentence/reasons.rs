// 78.92 — SPEECH THAT CARRIES ITS REASONS (ruled 2026-10-04:
// speech carries its reasons, so the cognition stays legible and its
// alignment can be read).
//
//! A sentence already IS its reasons: its address is its form (law 2) and its
//! receipts envelope carries the evidence the address excludes — the
//! generators each word vertex factored through, the demands' bodies, the
//! remainder's chain. A court replays all of it blind against its own copy
//! of the scene. What no reader did was say it: what the sentence was said
//! with, what its loops mean, what it could not explain, and what it left
//! unsaid.
//!
//! [`Reasons::of`] is that reader, the one (law 12): DERIVED from the word a
//! mind minted or a court holds, never stored beside it, so the speaker and
//! every court that admitted it read the same reasons off the same object.
//! [`Reasons::says`] renders them as plain sentences for a human; the typed
//! fields are the reading, the sentences only its view.

use super::{from_element_form, Vertex};
use crate::transducer::element::Element;

/// ONE WORD THE SENTENCE WAS SAID WITH: how many held generators it was
/// factored through (the language it stands on), whether it is a new letter
/// (adjoined: an action the alphabet cannot spell, carried as itself), and
/// how many charts it acts on across how many frames.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WordReason {
    pub generators: u64,
    pub adjoined: bool,
    pub charts: u64,
    pub frames: u64,
}

/// ONE LOOP OF THE SENTENCE, ONE MEMBER PATH: whether it returns curved —
/// its holonomy is not the identity, which is what the loop MEANS — and the
/// axes its curvature occupies.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LoopReason {
    pub curved: bool,
    pub axes: Vec<super::Axis>,
}

/// ONE THING THE SENTENCE COULD NOT EXPLAIN: a demand by its kind
/// ([`super::Demand::kind_byte`]), its measure per lane and its pieces.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OpenReason {
    pub kind: u8,
    pub measure: [u64; 4],
    pub pieces: u64,
}

/// WHY A SENTENCE SAYS WHAT IT SAYS, read off the sentence itself.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Reasons {
    pub words: Vec<WordReason>,
    pub loops: Vec<LoopReason>,
    pub open: Vec<OpenReason>,
    /// The cells its remainder holds: what the sentence left unsaid.
    pub unsaid: u64,
    /// Its measure per lane `S:G:T:H`.
    pub measure: [u64; 4],
}

/// The demand kinds by [`super::Demand::kind_byte`], for the rendering.
const KINDS: [&str; 6] = ["rotor prime", "standing", "source", "creation", "deformation", "dislocation"];

impl Reasons {
    /// THE REASONS OF A SENTENCE, from its form and its evidence. `None` for a
    /// word that is not a sentence (a letter or a connection says itself) or
    /// a form that does not decode — no reasons are invented for it.
    pub fn of(e: &Element) -> Option<Reasons> {
        let f = from_element_form(e)?;
        let mut words = Vec::new();
        let mut open = Vec::new();
        for (_, v) in f.vertices.entries() {
            match v {
                Vertex::Word { derivation, acts, .. } => {
                    let frames = acts.scoped().map(|(_, a)| a.frames.len() as u64).sum();
                    words.push(WordReason {
                        generators: derivation.generators.len() as u64,
                        adjoined: derivation.is_spelled() && derivation.word_local().is_none(),
                        charts: acts.len() as u64,
                        frames,
                    });
                }
                Vertex::Demand { kind, measure, pieces, .. } => {
                    open.push(OpenReason { kind: kind.kind_byte(), measure: *measure, pieces: *pieces });
                }
                _ => {}
            }
        }
        let mut loops = Vec::new();
        for lp in &f.orbits {
            for (_, cv) in super::pivot::drive::class_vectors_on(&f.faces, &f.vertices, &f.axes, lp).scoped() {
                loops.push(LoopReason { curved: !cv.is_zero(), axes: cv.occupied() });
            }
        }
        Some(Reasons { words, loops, open, unsaid: e.receipts().remainder.len() as u64, measure: e.magnitude() })
    }

    /// THE REASONS AS A HUMAN READS THEM, one sentence per kind of reason.
    pub fn says(&self) -> Vec<String> {
        let mut out = Vec::new();
        let cited = self.words.iter().filter(|w| w.generators > 0).count();
        let letters = self.words.iter().filter(|w| w.adjoined).count();
        let spelled = self.words.len() - cited - letters;
        let generators: u64 = self.words.iter().map(|w| w.generators).sum();
        out.push(format!(
            "said with {} word(s): {cited} built on held language ({generators} generator(s) cited), {letters} new letter(s), {spelled} spelled from the alphabet",
            self.words.len()
        ));
        let charts: u64 = self.words.iter().map(|w| w.charts).sum();
        let frames: u64 = self.words.iter().map(|w| w.frames).sum();
        out.push(format!("acting on {charts} chart(s) across {frames} frame(s)"));
        let curved: Vec<&LoopReason> = self.loops.iter().filter(|l| l.curved).collect();
        match (self.loops.len(), curved.len()) {
            (0, _) => out.push("no loop closes: nothing here is meant beyond the motion".to_string()),
            (n, 0) => out.push(format!("{n} loop(s), every one returning flat: the motion is consistent, it carries no curvature")),
            (n, c) => {
                let mut axes: Vec<&'static str> = curved.iter().flat_map(|l| l.axes.iter().map(|a| a.label())).collect();
                axes.sort_unstable();
                axes.dedup();
                out.push(format!("{n} loop(s), {c} returning curved — the meaning, on {}", axes.join(", ")));
            }
        }
        match self.open.as_slice() {
            [] => out.push("nothing it saw is left unexplained".to_string()),
            open => {
                let mut by: Vec<String> = Vec::new();
                for (k, name) in KINDS.iter().enumerate() {
                    let of: Vec<&OpenReason> = open.iter().filter(|o| usize::from(o.kind) == k).collect();
                    if let [_, ..] = of.as_slice() {
                        by.push(format!("{} {name} ({} piece(s))", of.len(), of.iter().map(|o| o.pieces).sum::<u64>()));
                    }
                }
                out.push(format!("could not explain: {}", by.join(", ")));
            }
        }
        out.push(match self.unsaid {
            0 => "left nothing unsaid".to_string(),
            n => format!("left {n} cell(s) unsaid"),
        });
        out
    }

    /// The reasons as bytes, each list by its own count's coordinate.
    pub fn put(&self, out: &mut Vec<u8>) {
        use crate::transducer::address::put_key_count;
        put_key_count(out, self.words.len());
        for w in &self.words {
            for x in [w.generators, w.charts, w.frames] {
                out.extend_from_slice(&x.to_le_bytes());
            }
            out.push(u8::from(w.adjoined));
        }
        put_key_count(out, self.loops.len());
        for l in &self.loops {
            out.push(u8::from(l.curved));
            put_key_count(out, l.axes.len());
            out.extend(l.axes.iter().map(|a| a.byte()));
        }
        put_key_count(out, self.open.len());
        for o in &self.open {
            out.push(o.kind);
            for x in o.measure.iter().chain([o.pieces].iter()) {
                out.extend_from_slice(&x.to_le_bytes());
            }
        }
        for x in [self.unsaid].iter().chain(self.measure.iter()) {
            out.extend_from_slice(&x.to_le_bytes());
        }
    }

    /// Bytes that do not read whole are no reasons, never partial ones.
    pub fn take(b: &[u8], p: &mut usize) -> Option<Reasons> {
        use crate::transducer::address::take_key_count;
        let u = |p: &mut usize| -> Option<u64> {
            let x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(x)
        };
        let byte = |p: &mut usize| -> Option<u8> {
            let x = *b.get(*p)?;
            *p += 1;
            Some(x)
        };
        let flag = |p: &mut usize| -> Option<bool> {
            match byte(p)? {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        };
        let mut words = Vec::new();
        for _ in 0..take_key_count(b, p)? {
            let (generators, charts, frames) = (u(p)?, u(p)?, u(p)?);
            words.push(WordReason { generators, adjoined: flag(p)?, charts, frames });
        }
        let mut loops = Vec::new();
        for _ in 0..take_key_count(b, p)? {
            let curved = flag(p)?;
            let mut axes = Vec::new();
            for _ in 0..take_key_count(b, p)? {
                axes.push(super::Axis::of(byte(p)?)?);
            }
            loops.push(LoopReason { curved, axes });
        }
        let mut open = Vec::new();
        for _ in 0..take_key_count(b, p)? {
            let kind = byte(p)?;
            let measure = [u(p)?, u(p)?, u(p)?, u(p)?];
            open.push(OpenReason { kind, measure, pieces: u(p)? });
        }
        let unsaid = u(p)?;
        let measure = [u(p)?, u(p)?, u(p)?, u(p)?];
        Some(Reasons { words, loops, open, unsaid, measure })
    }
}

/// WHAT A MIND'S ADMITTED SENTENCES CARRIED AS REASONS, cumulative: the
/// sentences, the words, those built on held language and the generators
/// they cite, the new letters, the loops and the curved ones, the demands
/// left open, and the cells left unsaid.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReasonsTally {
    pub sentences: u64,
    pub words: u64,
    pub cited: u64,
    pub generators: u64,
    pub letters: u64,
    pub loops: u64,
    pub curved: u64,
    pub open: u64,
    pub unsaid: u64,
}

impl ReasonsTally {
    /// Fold one sentence's reasons in.
    pub fn note(&mut self, r: &Reasons) {
        self.sentences += 1;
        self.words += r.words.len() as u64;
        self.cited += r.words.iter().filter(|w| w.generators > 0).count() as u64;
        self.generators += r.words.iter().map(|w| w.generators).sum::<u64>();
        self.letters += r.words.iter().filter(|w| w.adjoined).count() as u64;
        self.loops += r.loops.len() as u64;
        self.curved += r.loops.iter().filter(|l| l.curved).count() as u64;
        self.open += r.open.len() as u64;
        self.unsaid += r.unsaid;
    }

    /// In the order of [`crate::observe::REASONS`].
    pub fn row(&self) -> [u64; 9] {
        [self.sentences, self.words, self.cited, self.generators, self.letters, self.loops, self.curved, self.open, self.unsaid]
    }
}
