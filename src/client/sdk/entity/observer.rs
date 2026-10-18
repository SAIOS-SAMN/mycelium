// MU-CLIENT §4.4, §4.12, §7.2 — THE MIND ANSWERS ITS OBSERVER (the operator
// 2026-10-15: "build the observer into the mind so we can have an API to read
// the live state and call functions : we can remove this later"): the calls
// `observe::live` leaves for the winding carrier, answered out of what the
// mind holds at the law boundary the carrier passes.
//
//! Every answer READS: the carrier's standing, the thoughts and their
//! winding (period ℓ, hours seen, winding m, returns, the next hour), the
//! field's orbits and tubes, what is held for the return, the scenes and
//! solves waiting, the questions open. None moves the mind, and nothing in
//! cognition reads an answer back.

use crate::client::sdk::*;
use crate::observe::live;

/// The calls the mind answers, as `calls` lists them.
const CALLS: [(&str, &str); 10] = [
    ("convergence", "scenes this mind spoke words on, heard words' evidence read (true, refuted, unread), and each convergence: another mind's word and this mind's, different vertices, one section"),
    ("perception", "what perception's cells did: circuits, cells, thoughts, hours, the consolidation gate, grounds filed and refused by name"),
    ("now", "where the winding carrier stands, the reserve, the court's pace, what the mind holds"),
    ("thoughts [from N]", "every held thought by ordinal: its scene, ℓ, hours, winding m, returns, next hour, twist, slips, vortex, escrow"),
    ("thought N", "one thought by ordinal, whole"),
    ("field", "the field's orbits and tubes: open and moved per lane, the reservoir, returns not yet taken"),
    ("held", "what waits for the mind's next return"),
    ("scenes", "scenes waiting on comprehension, unread solves, unheard words, owed forms"),
    ("questions", "the questions this mind has open"),
    ("calls", "this list"),
];

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// The last `n` bytes of a coordinate, as hex: where coordinates of one
/// stratum differ.
fn tail(b: &[u8], n: usize) -> String {
    match b.len().checked_sub(n) {
        Some(at) => hex(&b[at..]),
        None => hex(b),
    }
}

fn lanes(l: &[u64; 4]) -> String {
    format!("{}:{}:{}:{}", l[0], l[1], l[2], l[3])
}

impl Entity {
    /// THE WINDING CARRIER ENTERS `law`, and the calls waiting are answered
    /// at this boundary.
    pub(crate) fn at_law(&mut self, law: &'static str) {
        live::enter(law);
        for line in live::wanted() {
            let lines = self.answer_call(&line);
            live::answer(&line, lines);
        }
    }

    /// THE WINDING CARRIER BEGINS A STROKE.
    pub(crate) fn stroke_begins(&mut self) {
        live::stroke_began(self.own_k(), self.hypha.tuniversal().map(|p| p.h));
        self.at_law("stroke");
    }

    fn answer_call(&mut self, line: &str) -> Vec<String> {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words.as_slice() {
            [] | ["now"] => self.read_now(),
            ["thoughts"] => self.read_thoughts(0),
            ["thoughts", "from", n] => match n.parse::<usize>() {
                Ok(n) => self.read_thoughts(n),
                Err(e) => vec![format!("from {n}: {e}")],
            },
            ["thought", n] => match n.parse::<u16>() {
                Ok(n) => self.read_thought(n),
                Err(e) => vec![format!("thought {n}: {e}")],
            },
            ["field"] => self.read_field(),
            // What perception's cells did, every outcome counted where it
            // forms (`ResonanceTally`): where thoughts and grounds are made
            // or refused, by name.
            ["perception"] => self.contemplation.resonance.line().split(" | ").map(str::to_string).collect(),
            ["held"] => self.read_held(),
            ["scenes"] => self.read_scenes(),
            ["questions"] => self.read_questions(),
            ["convergence"] => self.read_convergence(),
            ["calls"] => CALLS.iter().map(|(c, what)| format!("{c:<18} {what}")).collect(),
            _ => {
                let mut out = vec![format!("no call `{line}`; the calls:")];
                out.extend(CALLS.iter().map(|(c, what)| format!("{c:<18} {what}")));
                out
            }
        }
    }

    fn read_now(&self) -> Vec<String> {
        let c = &self.contemplation;
        let grounded = c.bubbles.scoped().filter(|(_, b)| b.ground.is_some()).count();
        let percepts = c.bubbles.scoped().filter(|(_, b)| b.percept).count();
        let pace = &self.board.court;
        let short = self.physics().reserve_shortfall([true; 4]);
        vec![
            format!(
                "mind K {} · TUniversal {} · courts seen {} · answers landed and not taken {}",
                self.own_k(),
                match self.hypha.tuniversal() {
                    Some(p) => format!("h {}", p.h),
                    None => "no height carried".to_string(),
                },
                self.hypha.courts(),
                self.hypha.in_flight()
            ),
            format!(
                "reserve {} · short {} · court pressure {} · dwell left {}",
                lanes(&self.physics().reserve.lanes()),
                lanes(&short),
                lanes(&pace.pressure()),
                lanes(&pace.dwell_remaining)
            ),
            format!(
                "asks in flight: quote {} · task {} · sync {} · syllabus {} · solves {} · a quote landed {} · a task landed {}",
                pace.quoting,
                pace.pulling,
                pace.syncing,
                pace.schooling,
                pace.solving.len(),
                pace.quoted.is_some(),
                pace.task.is_some()
            ),
            format!(
                "thoughts held {} (grounded {grounded}, perceptions {percepts}) · pair grounds {} · came round, not yet transcribed {} · first closes {} · later returns {}",
                c.bubbles.len(),
                c.grounds.len(),
                c.closed.len(),
                c.closed_thoughts,
                c.thought_returns
            ),
            {
                let d = c.docket_reading();
                format!(
                    "docket: pending {} · open {} · answered {} · lapsed {} · demands taken {} · perception carrier {}",
                    d.pending,
                    d.open,
                    d.answered,
                    d.lapsed,
                    c.demands_taken.len(),
                    match self.board.perceiver.as_ref() {
                        Some(p) => format!("in flight {} (scenes and solves {})", p.in_flight(), self.board.scenes_perceiving),
                        None => "not opened".to_string(),
                    }
                )
            },
            // 24.2 — WHAT THE MIND HAS LEARNED: its genome and dialect, the
            // taught words it refused (they never enter, so a sync fetches
            // them again), the strata its last digest found discordant, the
            // strata glued, the obstructions kept.
            format!(
                "language: genome {} · dialect {} · taught words refused {} · discordant strata {} (measured {}) · glued {} · obstructions {} over {} strata",
                self.epigenome.genome_len(),
                self.epigenome.dialect_len(),
                self.epigenome.refused,
                self.discordance,
                self.discordance_measured,
                self.glued.len(),
                self.obstructions.scoped().map(|(_, on)| on.len()).sum::<usize>(),
                self.obstructions.len()
            ),
            format!(
                "held for returns {} over {} scenes · scenes waiting {} · speaking {} · unheard words {} · unread solves {} · owed {} · contracted {} · questions {}",
                self.held_for_returns(),
                self.at_return.len(),
                self.comprehending.len(),
                self.speaking.len(),
                self.unheard.len(),
                self.unread_solves.len(),
                self.owed.len(),
                self.contracted.len(),
                self.questions.len()
            ),
            // MU-CLIENT §7.1 — THE MIND'S ATTENTION: whether it holds tension
            // (and so asks), the offers standing, whether each couples, and
            // the offers it passed uncoupled since it began, by kind.
            {
                let standing = |o: Option<(&[u8], &super::offer::Offer)>| match o {
                    Some((id, o)) => format!("couples {} · crossed a return {}", self.couples(id, &o.charts), o.crossed),
                    None => "none".to_string(),
                };
                let passed: Vec<String> = super::offer::PASSED.iter().zip(&self.board.offers_passed).map(|(k, n)| format!("{k} {n}")).collect();
                let (items, coords) = self.board.corpus.size();
                let families = self.board.feed.as_ref().map_or(0, super::feed::Feed::held);
                format!(
                    "attention: tension {} · open questions {} · feed families {} · corpus {} items on {} coordinates · met item waits on the reserve: family {} question {} · court task {} · passed uncoupled: {}",
                    self.strained(),
                    self.board.open_residuals.len(),
                    families,
                    items,
                    coords,
                    self.board.feed_waits,
                    self.board.question_waits,
                    standing(pace.task.as_ref().map(|(e, o)| (&e.id[..], o))),
                    passed.join(" · ")
                )
            },
        ]
        .into_iter()
        .chain(self.seat_lines())
        .collect()
    }

    /// One thought as one line: its ordinal, scene, layer, and — where it
    /// holds a ground — its winding.
    fn thought_line(&self, key: &[u8], b: &crate::transducer::bubble::Bubble) -> String {
        let ordinal = match self.contemplation.ordinals.get(key) {
            Some(o) => format!("#{o}"),
            None => "#?".to_string(),
        };
        let scene = match b.family.is_empty() {
            true => "no scene".to_string(),
            false => format!("scene {}", tail(&b.family, 4)),
        };
        let layer = match b.percept {
            true => "perception",
            false => "thought",
        };
        let winding = match &b.ground {
            Some(g) => {
                let period = g.ground.period;
                match (g.hours.checked_rem(period), g.hours.checked_div(period)) {
                    (Some(m), Some(r)) => format!(
                        "class {} · ℓ {period} · hours {} · m {m} · returns {r} · next hour {} · {}",
                        tail(g.ground.class.as_bytes(), 4),
                        g.hours,
                        (m + 1) % period,
                        match &g.twist {
                            Some(_) => "twisted (precessing)",
                            None => "untwisted",
                        }
                    ),
                    _ => format!("class {} · ℓ 0: no winding reads", tail(g.ground.class.as_bytes(), 4)),
                }
            }
            None => "no ground: no winding".to_string(),
        };
        format!(
            "{ordinal} {layer} · {scene} · {winding} · slips {} · vortex {} · candidates {} · escrow {}",
            b.slips.len(),
            b.vortex.scoped().map(|(_, v)| v.len()).sum::<usize>(),
            b.candidates.scoped().map(|(_, v)| v.len()).sum::<usize>(),
            lanes(&b.escrow),
        )
    }

    fn read_thoughts(&self, from: usize) -> Vec<String> {
        let all: Vec<(Vec<u8>, &crate::transducer::bubble::Bubble)> = self.contemplation.bubbles.scoped().collect();
        let mut out = vec![format!("{} held; from {from}", all.len())];
        out.extend(all.iter().skip(from).map(|(k, b)| self.thought_line(k, b)));
        out
    }

    fn read_thought(&self, ordinal: u16) -> Vec<String> {
        let Some(Some(addr)) = self.contemplation.addresses.get(usize::from(ordinal)) else {
            return vec![format!("#{ordinal}: no thought stands at this ordinal")];
        };
        let Some(b) = self.contemplation.bubbles.get(addr) else {
            return vec![format!("#{ordinal}: its address stands in no bubble")];
        };
        let mut out = vec![self.thought_line(addr, b)];
        if let Some(g) = &b.ground {
            out.push(format!("  ground on chart {} · bodies {}", tail(&g.chart, 8), g.ground.bodies.len()));
        }
        if let Some(f) = &b.form {
            out.push(format!("  form: vertices {} · faces {} · orbits {} · closed {}", f.vertices.len(), f.faces.len(), f.orbits.len(), f.is_closed()));
        }
        for (chart, _) in &b.slips {
            out.push(format!("  slip on chart {}", tail(chart, 8)));
        }
        out.push(format!(
            "  glues {} · glued by {} · torsion filed {} · certificates {} · remainders {} · bundle pairs {}",
            b.glues.len(),
            b.glued_by.len(),
            b.torsion.len(),
            b.certificates.len(),
            b.remainders.len(),
            b.bundle.as_ref().map_or(0, Vec::len)
        ));
        out
    }

    fn read_field(&self) -> Vec<String> {
        let read = crate::transducer::mind::with_field(|f| {
            let (open, moved) = f.orbits.totals();
            let (holding, lines, open_tubes) = f.tubes.threaded();
            let reserved = match f.reserved() {
                Some(r) => lanes(&r),
                None => "past the carrier".to_string(),
            };
            let mut out = vec![
                format!("orbits standing {} · tubes {} · returns not yet taken {} · sealed {}", f.orbits.len(), f.tubes.len(), f.orbits.pending_returns(), f.orbits.sealed()),
                format!("open per lane {} · moved per lane {} · reservoir {} · condensed {}", lanes(&open), lanes(&moved), reserved, lanes(&f.condensed)),
                format!("tubes holding lines {holding} ({lines} lines) · tubes open {open_tubes}"),
            ];
            // MU-CLIENT §4.13 — each orbit standing, and the lines it holds.
            out.extend(f.orbits.standing().into_iter().map(|o| {
                let lane = match o.lane {
                    Some(l) => l.to_string(),
                    None => "?".to_string(),
                };
                let period = match o.period {
                    Some(p) => p.to_string(),
                    None => "none (a rosette)".to_string(),
                };
                format!(
                    "  orbit on lane {lane} · {} · carried {} of period {period} · {} · open {} · lines {}",
                    if o.rings { "rings" } else { "never returns" },
                    o.carried,
                    if o.in_motion { "carried since the last beat" } else { "at rest since the last beat" },
                    o.open,
                    o.lines
                )
            }));
            out
        });
        let c = &self.contemplation;
        let thoughts = format!(
            "thoughts: unthreaded {} · held by nothing, leaving at the next vent {} · hearings waiting {} (one on the carrier: {}) · lapsed unread across a return {}",
            c.unthreaded.len(),
            c.unheld.len(),
            self.hearings_waiting(),
            self.board.hearing,
            self.board.hearings_lapsed
        );
        let read = read.map(|mut lines| {
            lines.push(thoughts);
            lines
        });
        match read {
            Ok(lines) => lines,
            Err(_) => vec!["no field stands in the winding carrier's slots at this boundary".to_string()],
        }
    }

    fn read_held(&self) -> Vec<String> {
        let mut out = vec![format!(
            "{} sayings held over {} scenes, each leaving at the return of a thought of its scene (none: at the mind's next return)",
            self.held_for_returns(),
            self.at_return.len()
        )];
        for (scene, held) in self.at_return.scoped() {
            let name = match scene.is_empty() {
                true => "the mind's own".to_string(),
                false => format!("scene {}", tail(&scene, 4)),
            };
            out.push(format!("  {name}: {} held", held.len()));
            out.extend(held.iter().map(|(at, _)| {
                let kind = match at.as_slice() {
                    [0, ..] => "speech",
                    [1, ..] => "SAY to peers",
                    [2, ..] => "solve",
                    _ => "seat call",
                };
                format!("    {kind} · {}", tail(at, 8))
            }));
        }
        out
    }

    fn read_scenes(&self) -> Vec<String> {
        let mut out = vec![format!("scenes waiting on comprehension {}", self.comprehending.len())];
        out.extend(self.comprehending.scoped().map(|(id, (t, _))| {
            format!(
                "  scene {} · pairs {} · tests {} · thoughts on it {}",
                tail(&id, 4),
                t.train.len(),
                t.tests.len(),
                self.contemplation.holds_family(&id)
            )
        }));
        out.push(format!("unread solves {}", self.unread_solves.len()));
        out.extend(self.unread_solves.scoped().map(|(id, _)| format!("  solve on {}", tail(&id, 4))));
        out.push(format!("unheard words {}", self.unheard.len()));
        out.extend(self.unheard.scoped().map(|(a, fam)| format!("  word {} under scene {}", tail(&a, 4), tail(fam, 4))));
        out.push(format!("owed {} · contracted {}", self.owed.len(), self.contracted.len()));
        out
    }

    fn read_convergence(&self) -> Vec<String> {
        let s = &self.scenes;
        let (scenes, words) = s.held();
        let mut out = vec![
            format!("scenes spoken on {scenes} · words held there {words}"),
            format!("heard words' evidence: true {} · refuted {} · unread {}", s.read[0], s.read[1], s.read[2]),
            format!("convergences {}", s.converged.len()),
        ];
        out.extend(s.converged.scoped().map(|(k, by)| format!("  {} · heard from {}", tail(&k, 12), tail(by, 4))));
        out
    }

    fn read_questions(&self) -> Vec<String> {
        let mut out = vec![format!("questions open {}", self.questions.len())];
        out.extend(self.questions.scoped().map(|(k, q)| format!("  {} · {}", tail(&k, 6), q.observed())));
        out
    }
}
