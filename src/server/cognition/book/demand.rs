// L8q, 50.5 — A PROBE THAT ANSWERS ONE DEMAND VERTEX of a listed
// sentence, settled on that vertex's own links.
//
//! The older probe deflated a listing: subtract the links the voice's
//! words already cover and record what is left. It cannot produce a held
//! generator, because nothing it does names a word — it names a smaller
//! chain. A sentence carries demand vertices (50.5), each a (kind, chart)
//! and the bodies it stands for; answering one is a claim about a FORM,
//! and the court settles it by speaking that form itself.

use crate::cognition::*;
use crate::transducer::address::Address;

impl Kernel {
    /// SETTLE A PROBE AGAINST ONE DEMAND VERTEX.
    ///
    /// `at` is the listing; `demand` is the key 50.5 files the vertex
    /// under. The court resolves the vertex on the listed sentence, frames
    /// the demand's own bodies as a scene (`scene_at_of_chain`, 50.6 seam
    /// 1 — both sides at one corner), and SPEAKS THE CITED FORM onto the
    /// before side with `apply_form`. What the form says is measured
    /// against the after side, link by link, in the demand's own
    /// coordinates.
    ///
    /// THE COURT REPLAYS; IT DOES NOT TAKE THE VOICE'S WORD. The cited
    /// address must be one this court holds, and the form spoken is the
    /// one the court holds under it, so a voice cannot answer a demand by
    /// asserting that it did.
    ///
    /// An empty residual is `Closed`: the demand is answered, the
    /// discoverer of the cited form is paid for the use, and WHAT answered
    /// is recorded in `answered` — the address and the transport. The
    /// listing stays on the book: a sentence has as many demands as it
    /// has, and one answered vertex is not a closed sentence. What retires
    /// it is the expansion (item 5).
    ///
    /// A non-empty residual is `Residual`: the demand's chart carries
    /// what the form did not reproduce, on the field, and NOTHING is
    /// written to any magnitude. A residual that writes a link count into a
    /// word's lanes makes the court slash its own listing on the next
    /// circuit.
    pub(crate) fn settle_demand(
        &mut self,
        voice: [u8; 32],
        at: &[u8],
        demand: &[u8],
        cites: &[crate::anastome::dialogue::CiteWire],
    ) -> crate::anastome::dialogue::ProbeReading {
        use crate::anastome::dialogue::{ProbeClosure, ProbeReading};
        use crate::transducer::family::{apply_form, scene_at_of_chain};
        use crate::transducer::stalk::Closure;
        let Some(listed) = self.pending.get(at) else {
            return ProbeReading::gone();
        };
        let held = crate::transducer::element::Held::of(listed.clone());
        let Some(form) = held.form() else {
            return ProbeReading::nothing();
        };
        // THE WIRE IS WHERE BYTES BECOME A COORDINATE. A probe names the
        // demand vertex it answers, and `demands` answers that system and no
        // other; the declaration is made here, at the boundary, rather than
        // inside the carrier where nothing could see it (54.7 R5).
        let Ok(demand) = <crate::network::nerve::coord::VertexAt
            as crate::network::nerve::Coordinate>::decode(demand)
        else {
            return ProbeReading::nothing();
        };
        let Some(crate::transducer::sentence::Vertex::Demand { chart, bodies, .. }) = form.vertices.at(&demand) else {
            return ProbeReading::nothing();
        };
        let (chart, bodies) = (chart.clone(), bodies.clone());
        let Some(((r0, c0), scene, image)) = scene_at_of_chain(&bodies) else {
            return ProbeReading::nothing();
        };
        let _ = &image;
        self.record_spoken(Address::from(at), voice);
        // The measure the settlement is read against is the DEMAND's, not
        // the listing's: this probe is about one vertex.
        let before = element::measure_lanes(&bodies);
        let placement = crate::transducer::alphabet::Word::identity();
        // WHAT THE DEMAND STILL ASKS FOR, over all the cites the court can
        // speak: a link is still demanded only where NOTHING said its
        // after colour. That is a fiber — the links no citation reached —
        // and never the shortest of several leftovers: choosing the cite
        // with the fewest links left would be a count deciding a branch
        // (rule 2), and it would throw away what the other cites did say.
        //
        // A cite naming an address this court does not hold is not a
        // refusal of the probe; it is a citation this court cannot replay,
        // and the probe stands on the ones it can.
        let mut open: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for &(r, c, _, _) in &bodies {
            open.insert(crate::transducer::family::site_key(r, c), ());
        }
        let mut spoke: Option<(Address, element::Versor)> = None;
        // ONE SPEND PER FORM THE COURT SPEAKS. `apply_form` perceives a
        // whole grid, and how many citations a probe carries is the
        // voice's to choose, not this court's — a settlement that speaks
        // every one of them has no boundary of its own (rule 10). The
        // citations beyond the stroke are not refused: what they would
        // have said stays demanded, which is what the field records.
        let mut spend = self.physics.stroke();
        for cite in cites {
            if !spend.spend() {
                break;
            }
            let Some(word) = self.basis.get(&cite.addr) else { continue };
            let Some(f) = word.form().cloned() else { continue };
            let Some(sp) = apply_form(&f, &scene, &placement, self) else { continue };
            let Some(spoken) = sp.grid() else { continue };
            // `scene_at_of_chain` gives the corner the framing subtracted,
            // so the spoken link goes back where the chain had it and
            // nothing re-derives that corner here (law 12).
            for &(r, c, _, a) in &bodies {
                // An offset below the corner is off the spoken grid, read
                // by `try_from` — never a wrap into a huge index that
                // happened to miss.
                let at = |x: i64| usize::try_from(x).ok();
                let said = at(r - r0)
                    .and_then(|dr| spoken.get(dr))
                    .and_then(|row| row.get(at(c - c0)?))
                    .copied();
                if said == Some(a) {
                    open.remove(&crate::transducer::family::site_key(r, c));
                }
            }
            if spoke.is_none() {
                spoke = Some((Address::from_bytes(&cite.addr), cite.transport.clone()));
            }
        }
        let Some((cited, transport)) = spoke else {
            return ProbeReading::nothing();
        };
        let residual: Vec<(i64, i64, u8, u8)> = bodies
            .iter()
            .copied()
            .filter(|&(r, c, _, _)| open.contains(&crate::transducer::family::site_key(r, c)))
            .collect();
        let after = element::measure_lanes(&residual);
        if residual.is_empty() {
            let used = cited.clone();
            self.answered.nested_insert(
                at,
                crate::network::nerve::Coordinate::encode(&demand).as_bytes(),
                (cited, transport),
            );
            self.write_field(at, chart, Closure::Flat, Vec::new(), &before, &[0; 4]);
            self.physics.relieve(&before);
            self.physics.syntropy(&before);
            self.append_step(crate::chronometry::ActKind::Settled, at, voice);
            // EC1: the use is paid AT the Settled act that carries it (POWC),
            // sealed first.
            let anchor = self.anchor();
            if let Some(word) = self.basis.get(used.as_bytes()) {
                let mag = word.magnitude();
                if let Some(disc) = self.discoverer_of(used.as_bytes()) {
                    self.treasury.settle_use(voice, disc, mag, anchor);
                }
            }
            return ProbeReading { settled: before, closure: ProbeClosure::Closed, remaining: [0; 4] };
        }
        // WHAT WAS RELEASED IS WHAT WAS SAID, measured on the links the
        // citations actually reached — not the difference of two measures.
        // Subtracting `after` from `before` assumes the residual's measure
        // is below the demand's on every lane, which is a claim about
        // `measure_lanes` and not something this settlement knows.
        let reached: Vec<(i64, i64, u8, u8)> = bodies
            .iter()
            .copied()
            .filter(|&(r, c, _, _)| !open.contains(&crate::transducer::family::site_key(r, c)))
            .collect();
        let v = super::speech::residual_versor(&bodies, &residual);
        self.write_field(at, chart, Closure::Cavity, residual, &before, &after);
        let settled = element::measure_lanes(&reached);
        self.physics.relieve(&settled);
        self.book_gen += 1;
        self.append_step(crate::chronometry::ActKind::Settled, at, voice);
        ProbeReading { settled, closure: ProbeClosure::Residual(v), remaining: after }
    }
}
