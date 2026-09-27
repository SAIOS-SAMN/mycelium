// 25, 30 step 6 — THE HOLD: what taking a form in costs and what it writes.
//
//! Split from `contemplation.rs` at the §1.1 1200-line reading bound; the
//! item is moved, not rewritten (§1.6). One organ: a form arrives, the
//! bubble that will carry it is built, its escrow is drawn and its
//! incidence admitted, and the bytes the hold retains are read across it
//! (Z4a's `HELD_BYTES`).

use super::*;
use super::contemplation::{candidates_over, live_bytes, mark, HELD_BYTES};
use crate::network::nerve::Keyed;
use crate::transducer::element;
use crate::transducer::sentence::{Form, Vertex};
use crate::transducer::stalk;

impl Contemplation {
    /// Z4 item 1 — THE FORM IS MOVED IN, NOT COPIED. It took `&Form` and
    /// the bubble kept `f.clone()`: 12.8 MB on a 590-vertex rule, measured
    /// at e0f4e0d, paid once per distinct form the mind holds. The caller
    /// has an extracted form it does not need afterwards, so it hands it
    /// over; where a caller does need it after, it reads it back from the
    /// bubble by reference (`form_at`).
    pub fn hold_form(
        &mut self,
        f: Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<Link>],
        escrow: [u64; 4],
        budget: &mut crate::physics::Budget,
    ) {
        use crate::transducer::family::trace;
        if f.vertices.len() <= 1
            && f.faces.is_empty()
            && f.remainder.is_empty()
            && remainders.iter().all(|r| r.is_empty())
        {
            // A form with nothing on it is not held, and under rule 17 that
            // is a reader writing no coordinate: the line names the site.
            trace::trace("hold/empty", trace::Wrote::Nothing, None);
            return;
        }
        // Z4's instrument: the live bytes this hold adds. Read at entry and
        // again at exit, so what is counted is what the hold RETAINED.
        let before = live_bytes();
        // The idea key is derived once for the line and only when a line is
        // wanted; `hold_form_inner` derives its own from the form it owns,
        // and neither is stored beside the other (law 12).
        let key = trace::tracing().then(|| idea_key(&f));
        let _ = self.hold_form_inner(f, train, remainders, escrow, budget, super::resonance::Spores::default());
        let retained = match (before, live_bytes()) {
            (Some(a), Some(b)) => {
                HELD_BYTES.with(|c| c.set(c.get().saturating_add(b - a)));
                Some(b - a)
            }
            // The allocator was not readable at one of the two points, so
            // there is no difference to report. A hold is not claimed to
            // have retained zero because the instrument could not look.
            _ => None,
        };
        // THE HOLD'S COORDINATE IS THE IDEA KEY it filed the bubble under,
        // and the bytes are what the process kept because of it. This is
        // the one reader of the three that can answer both columns.
        if let Some(key) = &key {
            match retained {
                Some(n) => trace::trace_bytes("hold/form", trace::Wrote::Key(key.as_ref()), None, n),
                None => trace::trace("hold/form", trace::Wrote::Key(key.as_ref()), None),
            }
        }
    }

    /// N4 — A PERCEPTION HELD WITH ITS BODIES: the form, and the frames it was
    /// perceived from, by chart (`resonance::spores_of`) — and then the
    /// filaments its bodies grow with every held perception it meets
    /// (`link_perceived`), on the act's own stroke.
    pub fn hold_perceived(
        &mut self,
        f: Form,
        frames: &crate::transducer::family::Frames,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<Link>],
        escrow: [u64; 4],
        budget: &mut crate::physics::Budget,
    ) -> Option<crate::transducer::address::Address> {
        let key = self.hold_form_inner(f, train, remainders, escrow, budget, super::resonance::spores_of(frames))?;
        self.link_perceived(key.as_bytes(), budget);
        Some(key)
    }

    fn hold_form_inner(
        &mut self,
        f: Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<Link>],
        escrow: [u64; 4],
        budget: &mut crate::physics::Budget,
        spores: super::resonance::Spores,
    ) -> Option<crate::transducer::address::Address> {
        mark("idea_key");
        let form_key = idea_key(&f);
        // A PERCEPTION IS KEYED BY ITS FORM AND ITS BODIES (N4, A21): the
        // form's key, then the bodies' coordinates. A hold with no bodies of a
        // form this mind holds as perceptions is those perceptions renewed —
        // read as the stratum the form's key prefixes — never a second idea.
        let seen = spores.iter().any(|s| !s.is_empty());
        let key = match seen {
            true => super::resonance::perceived_key(form_key, &spores),
            false => {
                let perceptions: Vec<Vec<u8>> = match self.bubbles.fiber_at(form_key.as_bytes()) {
                    Some(fiber) => fiber.map(|(k, _)| k).filter(|k| k.len() > form_key.len()).collect(),
                    None => Vec::new(),
                };
                if !perceptions.is_empty() {
                    for k in perceptions {
                        self.dirty.insert(&k, ());
                        if let Some(b) = self.bubbles.get_mut(&k) {
                            b.dirty = true;
                        }
                    }
                    return None;
                }
                form_key
            }
        };
        let mut rem_keyed: Keyed<Vec<Link>> = Keyed::new();
        let mut volume = [0u64; 4];
        let mut cover = Keyed::new();
        for (i, r) in remainders.iter().enumerate() {
            #[expect(
                clippy::cast_possible_truncation,
                reason = "AT1 debt — a slot number of the form's Vec (a pair's remainder, one per pair); the atlas row replaces the slot by the chart's coordinates"
            )]
            rem_keyed.insert(crate::transducer::family::pair_idx_key(i as u16), r.clone());
            if r.is_empty() {
                continue;
            }
            let m = element::measure_lanes(r);
            for l in 0..4 {
                #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): a sum of chain measures over resident chains cannot pass u64")]
                let v = volume[l].saturating_add(m[l]);
                volume[l] = v;
            }
            for (ck, _) in cover_of(&stalk::invariant(r)).scoped() {
                cover.insert(&ck, ());
            }
        }
        if !f.remainder.is_empty() {
            for (ck, _) in cover_of(&stalk::invariant(&f.remainder)).scoped() {
                cover.insert(&ck, ());
            }
        }
        mark("remainders+cover");
        let mut glues = Keyed::new();
        for e in &f.cover {
            glues.insert(&e.target, ());
        }
        let mut candidates: Keyed<Keyed<Rotor>> = Keyed::new();
        for (_, v) in f.vertices.scoped() {
            match v {
                // EVERY CHART THE VERTEX ACTS ON is a chart the bubble's
                // cover holds: a vertex acting on several is on several.
                Vertex::Word { acts, .. } => {
                    // THE CHART, NOT THE MEMBER'S KEY (54.7 R1). `cover` is
                    // a set of CHART keys and this inserted whatever the
                    // fibre was keyed by — which WAS the chart until the
                    // fibre had to separate its sections. Keyed by
                    // `chart ‖ carriage`, the working atlas answered nothing
                    // to a reader asking for a chart, and `incident_chart`
                    // read every idea as standing on no chart at all.
                    for (_, a) in acts.scoped() {
                        cover.insert(a.chart.key(), ());
                    }
                }
                // THE CHART IS THE BOUNDARY'S, THE GLUE IS THE COVER'S
                // (54.7 B5a). A vertex says what rank of chart it mates
                // with; which form it mates with is on `Form::cover`, read
                // below, and one boundary may mate with several.
                Vertex::Glued { chart, .. } => {
                    cover.insert(chart.key(), ());
                }
                // P9 — AN OPEN DEMAND STANDS ON ITS OWN CHART. A demand
                // carries the chart its reading was typed at (50.5), and the
                // chart is the coordinate the contemplation files a bubble
                // under (`by_chart`). Dropped here, an idea whose form is a
                // demand was held on NO chart: `admit_incidence` wrote no
                // incidence, `link_new` found no neighbour, and nothing the
                // mind held could ever meet it. Measured before this on the
                // one-body deformation P8 leaves standing: one bubble, cover
                // 0, horizon 0 — held and unreachable.
                //
                // It carries no word, so it contributes no section and no
                // rotor; what it contributes is the coordinate, which is
                // what a meet is read at.
                //
                // P9 — AND THE TRANSPORT BETWEEN THE BODIES IT STANDS FOR,
                // as a candidate. The demand carries those bodies (50.5)
                // and the two sides of its transition are its links split
                // by which hue they occupy, so the transport is RE-DERIVED
                // here and never stored beside them (law 12, and the same
                // reading `ring_answer` makes in `family/memo.rs`).
                Vertex::Demand { chart, bodies, .. } => {
                    cover.insert(chart.key(), ());
                    for (rk, r) in candidates_over(bodies.as_slice(), budget).scoped() {
                        candidates.nested_insert(chart.key(), rk, r.clone());
                    }
                }
                _ => {}
            }
        }
        mark("vertex walk");
        let mut horizon = Keyed::new();
        self.paint_seeded(&mut horizon);
        // THE FORM'S OWN STANDING ORBITS ENTER THE HORIZON (L24). The pivot
        // reads the horizon it is given, so an orbit this form carries — a
        // cover's meet cycle whose edge holonomy stands — has to be on it,
        // or the class the perception just measured reaches `pivot_orbit`
        // as nothing at all.
        //
        // `Bubble::expand_with` does exactly this for a simplex it meets on
        // the atlas (`Fill::Class` → `horizon_put`). This is the same
        // admission for the classes the form ARRIVED with, so a perceived
        // obstruction and a contemplated one are one kind of object.
        //
        // `Standing::Held`, as the atlas arm writes: the class is met and
        // not ruled. An orbit whose class vector occupies nothing is not a
        // horizon member — there is nothing to discharge — and a rotor
        // with no order or no class key is refused by `horizon_put`.
        self.paint_standing(&f, &mut horizon);
        mark("paint");
        let bundle = if train.is_empty() {
            None
        } else {
            Some(train.to_vec())
        };
        mark("bundle");
        // THE FORM IS MOVED IN HERE (Z4 item 1). What follows reads the
        // four duplicated carriers off the MOVED form rather than off the
        // caller's, because there is no caller's any more: the same bytes,
        // one fewer copy of the whole.
        let mut held = f;
        held.reindex();
        mark("form move+reindex");
        let certs = held.certificates.clone();
        let torsion = held.torsion.clone();
        let vortex = held.vortex.clone();
        let bubble = Bubble {
            remainder: held.remainder.clone(),
            nerve: Complex::new(),
            cover,
            horizon,
            potential: volume,
            volume,
            dirty: true,
            // A perceived form is held at the perception layer (N4).
            percept: true,
            spores,
            moved_k: self.now_k,
            // Z4d: THE ESCROW IS WHAT THE CALLER DREW, not what the
            // thought measures. They were the same number written in two
            // places and they disagreed: a bubble built here without a
            // draw — `hold`, a merge, the tape — still returned `volume`
            // on its lapse, and the reserve INFLATED. Measured on the fleet
            // at 3245041: `reserve [2194, 203, 88, 3382]` against a
            // `reserve_max` of 88, with lane T at exactly 88 because
            // nothing draws or returns on it. One place draws, one place
            // records, the same number (law 12).
            escrow,
            until: self.aperture_tu().and_then(|span| self.now_k.checked_add(span)),
            paid_from: self.aperture_tu().map(|_| self.now_k),
            glues,
            glued_by: Keyed::new(),
            torsion,
            certificates: certs,
            vortex,
            candidates,
            // Derived once, here, where the form is taken (Z4 item 3).
            rotor: Bubble::rotor_of_form(&held),
            form: Some(held),
            bundle,
            remainders: rem_keyed,
            cells_read: None,
        };
        // RE-HOLDING AN UNMUTATED FORM IS NOT MOVEMENT (ruled 2026-09-14).
        //
        // `moved_k` is the PHASE CHANGE stamp — "TUniversal when this
        // bubble's charts last moved" — and D8 re-holds a withheld form on
        // every stroke, so a hold that rebuilt the bubble and stamped
        // `moved_k = now_k` each time would let no bubble ever age:
        // `held_since(now_k, moved_k)` zero by construction, `t_extent`
        // perpetually zero, lane 𝒯 unable to accumulate.
        //
        // A form retained across an evaluation stroke has moved ZERO in
        // configuration space. The key IS the topology and the carriages:
        // `form_vertex_key` writes every vertex, every member and every gauge
        // (law 2), so a bubble found under this key is that same form and
        // nothing about it has changed.
        //
        // What it has ACCUMULATED is kept with it — the complex it built, the
        // cover it expanded, the classes its horizon met, who glued it.
        // Rebuilding those would throw the expansion away every stroke,
        // which is what would keep re-entering the cover and re-stamping
        // the clock.
        //
        // `moved_k` still advances where a transformation actually mutates
        // the charts: `expand_with` stamps it when a chart enters the cover,
        // and a pivot rewrites the form, which changes the key and starts a
        // new bubble at its own phase change.
        let mut bubble = bubble;
        if let Some(prior) = self.bubbles.get(key.as_bytes()) {
            bubble.moved_k = prior.moved_k;
            bubble.nerve = prior.nerve.clone();
            bubble.cover = prior.cover.clone();
            bubble.glued_by = prior.glued_by.clone();
            // The bodies it was perceived as stand with it where this hold
            // carries none (a re-hold of the same form).
            if bubble.spores.iter().all(|s| s.is_empty()) {
                bubble.spores = prior.spores.clone();
            }
            // The horizon is a union: this stroke's painting of the form's
            // standing classes, over the classes expansion has already met.
            for (hk, hv) in prior.horizon.scoped() {
                if !bubble.horizon.contains(&hk) {
                    bubble.horizon.insert(&hk, hv.clone());
                }
            }
        }
        mark("prior merge");
        // A re-held perception keeps its filaments: its bodies are the same.
        let lifted = self.lift_hyphae(key.as_bytes());
        if let Some(old) = self.bubbles.get(key.as_bytes()).cloned() {
            self.retract_incidence(key.as_bytes(), &old);
        }
        mark("retract_incidence");
        self.admit_incidence(&key.shared(), &bubble);
        self.refile_hyphae(key.as_bytes(), lifted);
        mark("admit_incidence");
        self.bubbles.insert(&key, bubble);
        mark("bubbles.insert");
        self.dirty.insert(&key, ());
        // A held form is a new candidate (78.26(6)).
        self.link_new(&key, super::meet::Adjunct::Candidate);
        mark("link_new");
        Some(key)
    }
}
