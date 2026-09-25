// 18.2, 21 step 1, 24.1 — THE FORM'S INDEXES: every keyed read the form
// answers — by address, chart, transport class, site, demand and star face —
// rebuilt from its vertices and faces in one pass.

use super::*;

impl Form {
    /// Fill by_address, by_class, star, carriages (by transport-class key)
    /// and hue_maps from the vertices and faces, in one pass over the
    /// form's own vertices. Two gauges on one word class are two hue
    /// sections, each kept. Two carriages on one transport class stay two
    /// sections: that class is refused as one carriage. Two Word vertices
    /// on one chart are both on that chart's fiber (24.1).
    pub fn reindex(&mut self) {
        let mut by_address: Keyed<u16, coord::Member> = Keyed::new();
        let mut by_class: Keyed<Vec<u16>, coord::Chart> = Keyed::new();
        let mut by_transport: Keyed<Vec<u16>, coord::Transport> = Keyed::new();
        let mut by_rebased: Keyed<u16, coord::Member> = Keyed::new();
        let mut by_unfolded: Keyed<u16, coord::Unfolded> = Keyed::new();
        let mut demands: Keyed<u16, coord::DemandVertex> = Keyed::new();
        // `glued` IS BUILT HERE TOO. `Form::glue` wrote it as it appended each
        // vertex and the tape decoder rebuilt it in a pass of its own; now that
        // `depth_of` reads it, whoever builds it is the indexer and that is
        // this pass (law 12).
        let mut glued: Keyed<u16, coord::Site> = Keyed::new();
        let mut hue_maps = std::mem::take(&mut self.hue_maps);
        let mut places: Keyed<Versor> = Keyed::new();
        let mut star: Keyed<u16, coord::VertexIx> = Keyed::new();
        for (i, v) in self.vertices.iter().enumerate() {
            match v {
                Vertex::Word { address, derivation, acts, .. } => {
                    let rotor = derivation.rotor();
                    let Ok(member_key) = coord::Member::decode(address.as_bytes()) else {
                        continue;
                    };
                    by_address.put(&member_key, i as u16);
                    // THE TRANSPORT CLASS IS THE VERTEX'S, not a member's, so
                    // this files the vertex once however many charts it acts
                    // on. A rotor with no class has no transport class (15.3)
                    // and is filed under none: no key of its bytes stands in.
                    if let Some(tk) = crate::transducer::family::transport_class(v).map(|c| c.key())
                    {
                        // 21 step 1's own key, declared as the transport
                        // class it is. `TransportClass::key` is the one
                        // writer of these bytes, so this is the one place
                        // they become a coordinate of that system.
                        match by_transport.at_mut(&tk) {
                            Some(on) => on.push(i as u16),
                            None => {
                                by_transport.put(&tk, vec![i as u16]);
                            }
                        }
                    }
                    // EVERY CHART THIS VERTEX ACTS ON is a chart it stands
                    // at, so `by_class` files it under each of them. One
                    // member is the single-chart case and reads as before.
                    for (_, a) in acts.scoped() {
                        // KEYED BY THE CHART IT INDEXES (54.7 R1). It used
                        // the member's own key, which WAS the chart until
                        // the fibre had to separate its sections; now the
                        // key is `chart ‖ carriage` and a reader asking
                        // `by_class` for a chart found nothing. An index is
                        // keyed by the coordinate it answers, not by
                        // whatever the carrier it was built from used.
                        //
                        // 54.7 R5 — AND NOW THE COMPILER READS IT. `ck` is
                        // written as a chart coordinate here or this vertex
                        // stands on no chart at all; the member's key is a
                        // different type and cannot reach this carrier.
                        let chart_key = coord::Chart(a.chart.clone());
                        let ck = chart_key.encode().as_bytes().to_vec();
                        match by_class.at_mut(&chart_key) {
                            Some(on) => {
                                if !on.contains(&(i as u16)) {
                                    on.push(i as u16);
                                }
                            }
                            None => {
                                by_class.put(&chart_key, vec![i as u16]);
                            }
                        }
                        // `hue_maps` and `carriages` are DERIVATIONS OF THE
                        // FIBRE and are rebuilt from it here, or they are a
                        // second carrier for the fact the fibre now holds
                        // (law 12, D7) — the same fault as keeping the three
                        // fields beside it, one level up.
                        if let Some(hk) = hue_section_key_of(&rotor, &a.gauge) {
                            hue_maps.insert(hk, a.gauge.clone());
                        }
                        // 18.2 FINALLY TRUE OF ITS OWN TYPE. The key is the
                        // transport class AND the chart, so two carriages on
                        // one class are two entries — two sections — where
                        // before they were one entry refused by
                        // `place_conflict` and replaced by nothing. The
                        // comment here claimed the class "stays two
                        // sections" and no section was ever written; it is
                        // written now, and `place_conflict` is gone with the
                        // claim it could not keep.
                        if let Some(tk) = vertex_transport_key_of(&rotor, &a.gauge) {
                            let mut key = tk.into_vec();
                            crate::transducer::address::put_key_bytes(&mut key, &ck);
                            places.insert(key, (*a.carriage).clone());
                        }
                    }
                }
                Vertex::Rebased { of, .. } => {
                    if let Some(Vertex::Word { address, .. }) = self.vertices.get(*of as usize) {
                        if let Ok(k) = coord::Member::decode(address.as_bytes()) {
                            by_rebased.put(&k, i as u16);
                        }
                    }
                }
                Vertex::Unfolded { measure } => {
                    by_unfolded.put(&unfolded_key(measure), i as u16);
                }
                Vertex::Demand { kind, chart, .. } => {
                    demands.put(
                        &crate::transducer::sentence::demand_key(kind, chart),
                        i as u16,
                    );
                }
                // 54.7 B5a — KEYED BY THE SITE, which is what the vertex
                // now holds. It was keyed by the cited address, and that
                // address is the cover's fact on `Form::cover`.
                v @ Vertex::Glued { .. } => {
                    if let Ok(site) =
                        coord::Site::decode(&crate::transducer::sentence::vertex_bytes(v))
                    {
                        glued.put(&site, i as u16);
                    }
                }
                _ => {}
            }
        }
        // EVERY PLACE IS A SECTION AND NONE IS REFUSED. The key is the
        // transport class AND the chart, so two carriages on one class are
        // two entries rather than one entry conflicting with itself, and
        // 18.2's sentence is finally the type's. `place_conflict` marked
        // the conflict, dropped the class from `carriages` entirely, and
        // wrote no section anywhere — a local that died exactly as
        // `after_chain` does — so the one index keyed by transport class
        // went empty on precisely the classes the defect was about.
        let mut carriages = Keyed::new();
        for (k, p) in places.scoped() {
            carriages.insert(&k, p.clone());
        }
        // ONE STAR FACE PER VERTEX: the rule's face, the first between the
        // Frame and that vertex. A later face between the same two is a
        // section over some pairs (21 step 2, S12.2) and stays as the
        // bundle spoke it; the rebase pass re-derives star faces from their
        // vertices, and a section it re-derived was a section erased.
        for (fi, f) in self.faces.iter().enumerate() {
            let key = if f.i == 0 {
                vertex_ix_key(f.j)
            } else if f.j == 0 {
                vertex_ix_key(f.i)
            } else {
                continue;
            };
            if !star.holds(&key) {
                star.put(&key, fi as u16);
            }
        }
        self.by_address = by_address;
        self.by_class = by_class;
        self.by_transport = by_transport;
        self.by_rebased = by_rebased;
        self.by_unfolded = by_unfolded;
        self.demands = demands;
        self.glued = glued;
        self.star = star;
        self.carriages = carriages;
        self.hue_maps = hue_maps;
    }
}
