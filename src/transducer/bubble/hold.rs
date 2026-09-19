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
    ) {
        if f.vertices.len() <= 1
            && f.faces.is_empty()
            && f.remainder.is_empty()
            && remainders.iter().all(|r| r.is_empty())
        {
            return;
        }
        // Z4's instrument: the live bytes this hold adds. Read at entry and
        // again at exit, so what is counted is what the hold RETAINED.
        let before = live_bytes();
        self.hold_form_inner(f, train, remainders, escrow);
        if let (Some(a), Some(b)) = (before, live_bytes()) {
            HELD_BYTES.with(|c| c.set(c.get().saturating_add(b - a)));
        }
    }

    fn hold_form_inner(
        &mut self,
        f: Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<Link>],
        escrow: [u64; 4],
    ) {
        mark("idea_key");
        let key = idea_key(&f);
        let mut rem_keyed: Keyed<Vec<Link>> = Keyed::new();
        let mut volume = [0u64; 4];
        let mut cover = Keyed::new();
        for (i, r) in remainders.iter().enumerate() {
            rem_keyed.insert(crate::transducer::family::pair_idx_key(i as u16), r.clone());
            if r.is_empty() {
                continue;
            }
            let m = element::measure_lanes(r);
            for l in 0..4 {
                volume[l] = volume[l].saturating_add(m[l]);
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
        let mut candidates: Keyed<Keyed<Rotor>> = Keyed::new();
        for v in &f.vertices {
            match v {
                // EVERY CHART THE VERTEX ACTS ON is a chart the bubble's
                // cover holds: a vertex acting on several is on several.
                Vertex::Word { acts, .. } => {
                    for (ck, _) in acts.scoped() {
                        cover.insert(&ck, ());
                    }
                }
                Vertex::Glued { address, chart, .. } => {
                    cover.insert(chart.key(), ());
                    glues.insert(address, ());
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
                    for (rk, r) in candidates_over(bodies).scoped() {
                        candidates.nested_insert(chart.key(), rk, r.clone());
                    }
                }
                _ => {}
            }
        }
        mark("vertex walk");
        let mut horizon = Keyed::new();
        self.paint_seeded(&mut horizon);
        // THE FORM'S OWN STANDING LOOPS ENTER THE HORIZON (L24). The pivot
        // reads the horizon it is given, so a loop this form carries — a
        // cover's meet cycle whose edge holonomy stands — has to be on it,
        // or the class the perception just measured reaches `pivot_loop`
        // as nothing at all.
        //
        // `Bubble::expand_with` does exactly this for a simplex it meets on
        // the atlas (`Fill::Class` → `horizon_put`). This is the same
        // admission for the classes the form ARRIVED with, so a perceived
        // obstruction and a contemplated one are one kind of object.
        //
        // `Standing::Held`, as the atlas arm writes: the class is met and
        // not ruled. A loop whose class vector occupies nothing is not a
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
        self.admit_incidence(&key, &bubble);
        mark("admit_incidence");
        self.bubbles.insert(&key, bubble);
        mark("bubbles.insert");
        self.dirty.insert(&key, ());
        self.link_new(&key);
        mark("link_new");
        eprintln!(
            "  form held {:02x}{:02x}…",
            key[0], key[1]
        );
    }
}
