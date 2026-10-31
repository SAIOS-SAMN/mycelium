// 25, 30 step 6 — THE HOLD: what taking a form in costs and what it writes.
//
//! Split from `contemplation.rs` at the §1.1 1200-line reading bound; the
//! item is moved, not rewritten (§1.6). One organ: a form arrives, the
//! bubble that will carry it is built, its escrow is drawn and its
//! incidence admitted, and the bytes the hold retains are read across it
//! (Z4a's `HELD_BYTES`).

use super::*;
use super::contemplation::{live_bytes, mark, rotor_of_transport, transport_over, HELD_BYTES};
use crate::network::nerve::Keyed;
use crate::transducer::element;
use crate::transducer::sentence::{Form, Vertex};
use crate::transducer::stalk;

/// MU-CLIENT §4.3 — THE BREATHS A FORM HOLDS, every plane, across its
/// motions, over every axis (ruled 2026-10-21: the split needs
/// wider machinery than one plane):
/// - WITHIN EACH MOTION: where it carries both a translation and a
///   transversion — what is carried forward meeting what is carried back
///   through the inversion — the breath of the two (`cga::breath`);
/// - ACROSS ITS MOTIONS: every pair of the motions' generators (the generators' exact
///   Cayley bivectors) breathed against each other (`cga::breathe`), each
///   pair once, in the form's own order.
///
/// Each breath is split into its radial axis and its commuting planes by
/// kind. A motion that does not factor, has no generator in the chart, or a
/// pair whose planes do not split over ℚ, breathes nothing here.
pub(crate) fn breaths_of(f: &Form, budget: &mut crate::physics::Budget) -> Vec<crate::transducer::cga::Breath> {
    let mut out = Vec::new();
    let mut generators: Vec<crate::transducer::cga::Mv> = Vec::new();
    for (_, v) in f.vertices.scoped() {
        let Vertex::Word { derivation, acts, .. } = v else { continue };
        for (_, a) in acts.scoped() {
            let Some(motion) = element::conformal_of_rotor(&derivation.rotor(), &a.carriage) else { continue };
            if let Some(g) = crate::transducer::cga::cayley_inverse(&motion) {
                generators.push(g);
            }
            let Ok(c) = element::conformal_factors(&motion) else { continue };
            if !c.transverses() || c.translation.iter().all(num_traits::Zero::is_zero) {
                continue;
            }
            if let Ok(b) = crate::transducer::cga::breath(&c.translation, &c.transversion, budget) {
                out.push(b);
            }
        }
    }
    for (i, x) in generators.iter().enumerate() {
        for y in generators.iter().skip(i + 1) {
            if !budget.spend() {
                return out;
            }
            if let Ok(b) = crate::transducer::cga::breathe(x, y, budget) {
                out.push(b);
            }
        }
    }
    out
}

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
        let _ = self.hold_form_inner(f, train, remainders, escrow, budget, super::resonance::Spores::default(), &[]);
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
    /// filaments its bodies grow with every held perception OF ITS FAMILY
    /// it meets (`perceive_cells`, 78.56 III), on the act's own stroke.
    /// `family` is the task's coordinate the perception was seen in (empty
    /// where none is named).
    #[expect(clippy::too_many_arguments, reason = "78.56 III — the family is the perception's own coordinate, passed where it is held, never carried as hidden context")]
    pub fn hold_perceived(
        &mut self,
        f: Form,
        frames: &crate::transducer::family::Frames,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<Link>],
        escrow: [u64; 4],
        family: &[u8],
        budget: &mut crate::physics::Budget,
    ) -> Option<crate::transducer::address::Address> {
        let heard = f.heard.clone();
        let breaths = breaths_of(&f, budget);
        // The docket as it stood before this perception: what the hold adds
        // to it stands in this perception's scene.
        let mut before: Keyed<()> = Keyed::new();
        for (k, _) in self.docket.scoped() {
            before.insert(&k, ());
        }
        let key = self.hold_form_inner(f, train, remainders, escrow, budget, super::resonance::spores_of(frames), family);
        if let Some(key) = &key {
            // 78.112 H7b — the torsion hearing its rings wound, on its vertex,
            // until the mind lets the perception go and it unwinds.
            if let Some(w) = &heard {
                self.complex.wind(key.as_bytes(), w);
            }
            // MU-CLIENT §4.3 — THE BREATH'S PINCH, wound on the thought's
            // vertex beside its hearing, and unwound with it when the mind
            // lets the thought go: where a motion it holds carries what it
            // takes forward (a translation) and what it carries back through
            // the inversion (a transversion), the two motions' commutator's orthogonal
            // part twists (`cga::breath`).
            for b in &breaths {
                let [pinched, null, boost] = b.by_kind();
                crate::observe::note_mind(|m| {
                    m.breath[0] += u64::from(!num_traits::Zero::is_zero(&b.radial));
                    m.breath[1] += pinched;
                    m.breath[2] += null;
                    m.breath[3] += boost;
                });
                for t in b.twists() {
                    self.complex.wind(key.as_bytes(), &t);
                }
            }
            self.perceive_cells(key.as_bytes(), budget);
        }
        self.demands_in_scene(&before, family);
        key
    }

    #[expect(clippy::too_many_arguments, reason = "78.56 III — the family rides with the spores it scopes")]
    fn hold_form_inner(
        &mut self,
        f: Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<Link>],
        escrow: [u64; 4],
        budget: &mut crate::physics::Budget,
        spores: super::resonance::Spores,
        family: &[u8],
    ) -> Option<crate::transducer::address::Address> {
        mark("idea_key");
        let form_key = idea_key(&f);
        // A PERCEPTION IS KEYED BY ITS FORM AND ITS BODIES (N4, A21): the
        // form's key, then the bodies' coordinates. A hold with no bodies of a
        // form this mind holds as perceptions is those perceptions renewed —
        // read as the stratum the form's key prefixes — never a second idea.
        let seen = !spores.is_empty();
        let key = match seen {
            true => super::resonance::perceived_key(form_key, &spores),
            false => {
                let perceptions: Vec<Vec<u8>> = match self.bubbles.fibre_at(form_key.as_bytes()) {
                    Some(fibre) => fibre.map(|(k, _)| k).filter(|k| k.len() > form_key.len()).collect(),
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
            rem_keyed.insert(crate::transducer::family::pair_index_key(i as u16), r.clone());
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
                    let transport = transport_over(bodies.as_slice(), budget);
                    let rotor = transport.as_ref().and_then(rotor_of_transport);
                    if let Some(r) = rotor.as_ref() {
                        candidates.nested_insert(chart.key(), r.key(), r.clone());
                    }
                    // 37.7 — THE DEMAND'S WHOLE TRANSPORT GOES ON THE DOCKET,
                    // its residue the remainder a held word would have to
                    // carry. Only its rotation reached the candidates, and
                    // nothing was docketed: a form that was all demands
                    // ("nothing in the form acts", 456 refusals observed on the
                    // development deployment, epoch 146) never reached the demand taker, so its
                    // remainder never became a letter. Its class is its
                    // rotation's — the identity's where the transport turns
                    // nothing (a slide, a dilation: the motion letter).
                    if let Some(v) = transport.as_ref() {
                        let word = match rotor {
                            Some(r) => crate::transducer::alphabet::Word::formed(r),
                            None => crate::transducer::alphabet::Word::identity(),
                        };
                        if let Some(class) = crate::transducer::sentence::ClassVector::of_word(&word) {
                            self.docket_demand(&class, std::slice::from_ref(chart), Some(v));
                        }
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
            family: family.to_vec(),
            moved_k: self.now_k,
            // Z4d: THE ESCROW IS WHAT THE CALLER DREW, not what the
            // thought measures. They were the same number written in two
            // places and they disagreed: a bubble built here without a
            // draw — `hold`, a merge, the tape — still returned `volume`
            // on its lapse, and the reserve INFLATED. Measured on the development
            // deployment at 3245041: `reserve [2194, 203, 88, 3382]` against a
            // `reserve_max` of 88, with lane T at exactly 88 because
            // nothing draws or returns on it. One place draws, one place
            // records, the same number (law 12).
            escrow,
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
            // A perception holds no ground: its hours begin when it relates.
            ground: None,
            slips: Vec::new(),
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
        let mut drawn_again: Option<[u64; 4]> = None;
        if let Some(prior) = self.bubbles.get(key.as_bytes()) {
            // ESCROW CONSERVED (MU-CLIENT §7.1 Phase 3): the thought standing
            // here holds what was drawn for it once; what this re-hold drew
            // is returned, never held twice.
            drawn_again = Some(bubble.escrow);
            bubble.escrow = prior.escrow;
            bubble.moved_k = prior.moved_k;
            bubble.nerve = prior.nerve.clone();
            bubble.cover = prior.cover.clone();
            bubble.glued_by = prior.glued_by.clone();
            // The bodies it was perceived as stand with it where this hold
            // carries none (a re-hold of the same form).
            if bubble.spores.is_empty() {
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
        if let Some(old) = self.bubbles.get(key.as_bytes()).cloned() {
            self.retract_incidence(key.as_bytes(), &old);
        }
        mark("retract_incidence");
        if let Some(again) = drawn_again {
            self.return_escrow(&again);
        }
        self.admit_incidence(&key.shared(), &bubble);
        mark("admit_incidence");
        self.bubbles.insert(&key, bubble);
        mark("bubbles.insert");
        self.dirty.insert(&key, ());
        // MU-CLIENT §4.13 — its line is threaded at the next vent.
        self.awaits_thread(key.as_bytes());
        // A held form is a new candidate (78.26(6)).
        self.link_new(&key, super::meet::Adjunct::Candidate);
        mark("link_new");
        Some(key)
    }
}
