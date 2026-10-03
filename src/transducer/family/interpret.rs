// 78.81 — THE REMAINDER INTERPRETED, ON THE VOICE: what a form leaves unsaid
// of its family is inverted and then read whole, in the act that spoke it —
// never in an instrument beside the mind.
//
//! ONE PASS, IN THE VOICE'S STROKE, over the remainders its speech measured:
//!
//! 1. INVERT (78.76): the remainder as a scene, its generators extracted and
//!    joined to the form, accepted only where speech leaves less unsaid on no
//!    pair more. An accepted form is the one the voice goes on to speak.
//! 2. On what is STILL unsaid, per pair —
//!    - LIGHT (78.77): the waveform rays out of the input's bodies and the
//!      enclosures that capture, by site: on a ray, captured, unlit;
//!    - CONFLUENCE (78.78): the unsaid output bodies as creations, single
//!      carriages and confluences, and the confluences their inverse
//!      generator reproduces;
//!    - CONSTRUCTORS (78.79): every single change by its constructors —
//!      kinematic, accreted on light, carved;
//!    - DECISION (78.80): the recolours in place judged under every key, the
//!      body's own moments and its rank in its pair, leave one out, by replay;
//!    - RECONSTRUCTION (78.82, HR4): the family's field carried by the axis
//!      whose loops close, each pair left out and judged by replay.
//!
//! Every reading is folded into this process's tally, read on the voice's
//! stroke line. What step 2 reads, the form cannot yet speak: a recolour that
//! a RELATIONAL key decides is a member keyed by rank, which no vertex holds
//! (`Acting` is keyed by chart and carriage) — that is a layout change, the
//! operator's to rule.

use super::incongruence::{decide, single_changes, Decision, KEYS};
use super::invert::{invert_measured, Inversion, Painted};
use super::*;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// WHAT ONE FAMILY'S REMAINDER READ.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Interpretation {
    pub inversion: Option<Inversion>,
    /// Unsaid sites after the inversion, and by the light.
    pub unsaid: u64,
    pub on_ray: u64,
    pub captured: u64,
    pub unlit: u64,
    /// Unsaid sites by the body they stand in, and the confluences the
    /// inverse generator reproduces.
    pub creation_sites: u64,
    pub single_sites: u64,
    pub confluence_sites: u64,
    pub confluences: u64,
    pub confluences_reproduced: u64,
    /// The single changes, the kinematic ones, constructors accreted on light
    /// and carved from the interior.
    pub changes: u64,
    pub kinematic: u64,
    pub on_light: u64,
    pub carved: u64,
    /// The recolours in place, those their own hue map replays (the ceiling
    /// of any key), and what each key decided of them.
    pub recolours: u64,
    pub replayable: u64,
    pub decided: [Decision; 8],
    /// HR4 (78.82): the family reconstructed by its holonomy, each pair left
    /// out in turn.
    pub reconstruction: super::reconstruct::Reconstruction,
    /// HR5 (78.83): the same, its loops judged in time.
    pub in_time: super::hyperfluid::InTime,
}

/// A form the voice speaks from here on, with the remainders it leaves.
pub type Spoken = (Form, Vec<Vec<(i64, i64, u8, u8)>>);

/// INTERPRET A SPOKEN FORM'S REMAINDER: the joined form with its remainders
/// where the inversion was accepted, and what was read. Nothing is read where
/// nothing is unsaid.
pub fn interpret(
    form: &Form,
    family: &[(Grid, Grid)],
    remainders: &[Vec<(i64, i64, u8, u8)>],
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> (Option<Spoken>, Interpretation) {
    let mut read = Interpretation::default();
    let Some((inversion, joined)) = invert_measured(form, family, remainders, crystal, budget, Painted::Bodies) else {
        return (None, read);
    };
    read.inversion = Some(inversion);
    let left: &[Vec<(i64, i64, u8, u8)>] = match &joined {
        Some((_, after)) => after,
        None => remainders,
    };
    for ((x, y), rem) in family.iter().zip(left) {
        if rem.is_empty() {
            continue;
        }
        let light = luminal::read_pair(x, y, rem, budget);
        read.unsaid += light.unsaid;
        read.on_ray += light.on_ray + light.both;
        read.captured += light.captured;
        read.unlit += light.neither;
        let gathered = confluence::read_pair(x, y, rem, budget);
        read.creation_sites += gathered.creation_sites;
        read.single_sites += gathered.single_sites;
        read.confluence_sites += gathered.confluence_sites;
        read.confluences += gathered.confluences.len() as u64;
        read.confluences_reproduced += gathered.confluences.iter().filter(|c| c.reproduces).count() as u64;
    }
    let changes = single_changes(family, left);
    read.changes = changes.len() as u64;
    for c in &changes {
        read.kinematic += u64::from(c.difference.kinematic());
        read.on_light += c.difference.on_light;
        read.carved += c.difference.carved;
    }
    read.decided = decide(&changes);
    read.recolours = read.decided[0].predicted + read.decided[0].refused + read.decided[0].alone;
    read.replayable = changes
        .iter()
        .filter(|c| c.difference.kinematic() && !c.difference.recoloured.is_empty() && super::incongruence::replayable(c))
        .count() as u64;
    read.reconstruction = super::reconstruct::reconstruct(family);
    read.in_time = super::hyperfluid::reconstruct_in_time(family);
    note_read(&read);
    (joined, read)
}

const KEYED: usize = 27;
const SLOTS: usize = KEYED + 2 * KEYS.len();
static TALLY: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];

fn note(slot: usize, by: u64) {
    if by > 0 && TALLY[slot].fetch_update(Relaxed, Relaxed, |held| held.checked_add(by)).is_err() {
        let mut past = u64::MAX;
        let _ = crate::physics::metabolism::step_or_refuse(&mut past, by);
    }
}

fn note_read(r: &Interpretation) {
    // Nothing unsaid is no inversion: read as such, never as a default one.
    let (accepted, closed, unextracted, before, after, joined) = match r.inversion {
        Some(i) => (u64::from(i.accepted), u64::from(i.closed), u64::from(i.unextracted), i.before, i.after, i.joined),
        None => (0, 0, 0, 0, 0, 0),
    };
    let flat = [
        1,
        accepted,
        closed,
        unextracted,
        before,
        after,
        joined,
        r.unsaid,
        r.on_ray,
        r.captured,
        r.unlit,
        r.creation_sites,
        r.single_sites,
        r.confluence_sites,
        r.confluences_reproduced,
        r.changes,
        r.kinematic,
        r.recolours,
        u64::from(r.recolours > 0),
        r.replayable,
        u64::from(r.reconstruction.whole),
        r.reconstruction.framed,
        r.reconstruction.reproduced,
        r.reconstruction.unclosed,
        r.reconstruction.unseen,
        u64::from(r.in_time.whole),
        r.in_time.reproduced,
    ];
    for (slot, by) in flat.into_iter().enumerate() {
        note(slot, by);
    }
    for (i, d) in r.decided.iter().enumerate() {
        note(KEYED + i, d.reproduced);
        note(KEYED + KEYS.len() + i, u64::from(d.whole));
    }
}

/// THE INTERPRETER'S TALLY since the process began: families read, the
/// inversion's acceptances and closures, links before and after, the light's
/// and the confluence's sites, the changes, and per key of [`KEYS`] the
/// recolours reproduced and the families every recolour of which it decided.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InterpretTally {
    pub families: u64,
    pub accepted: u64,
    pub closed: u64,
    pub unextracted: u64,
    pub before: u64,
    pub after: u64,
    pub joined: u64,
    pub unsaid: u64,
    pub on_ray: u64,
    pub captured: u64,
    pub unlit: u64,
    pub creation_sites: u64,
    pub single_sites: u64,
    pub confluence_sites: u64,
    pub confluences_reproduced: u64,
    pub changes: u64,
    pub kinematic: u64,
    pub recolours: u64,
    pub recoloured_families: u64,
    pub replayable: u64,
    /// HR4: families reconstructed whole, pairs framed and reproduced left
    /// out, and the left-out pairs no axis closed for or whose key was unseen.
    pub reconstructed: u64,
    pub framed: u64,
    pub rebuilt_pairs: u64,
    pub unclosed: u64,
    pub unseen: u64,
    /// HR5: families reconstructed whole in time, and pairs.
    pub in_time: u64,
    pub in_time_pairs: u64,
    pub reproduced: [u64; 8],
    pub whole: [u64; 8],
}

pub fn interpret_tally() -> InterpretTally {
    let r = |i: usize| TALLY[i].load(Relaxed);
    InterpretTally {
        families: r(0),
        accepted: r(1),
        closed: r(2),
        unextracted: r(3),
        before: r(4),
        after: r(5),
        joined: r(6),
        unsaid: r(7),
        on_ray: r(8),
        captured: r(9),
        unlit: r(10),
        creation_sites: r(11),
        single_sites: r(12),
        confluence_sites: r(13),
        confluences_reproduced: r(14),
        changes: r(15),
        kinematic: r(16),
        recolours: r(17),
        recoloured_families: r(18),
        replayable: r(19),
        reconstructed: r(20),
        framed: r(21),
        rebuilt_pairs: r(22),
        unclosed: r(23),
        unseen: r(24),
        in_time: r(25),
        in_time_pairs: r(26),
        reproduced: std::array::from_fn(|i| r(KEYED + i)),
        whole: std::array::from_fn(|i| r(KEYED + KEYS.len() + i)),
    }
}
