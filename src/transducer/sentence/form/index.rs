// 18.2, 21 step 1, 24.1 — THE FORM'S FIBRES: the vertices on each chart and
// of each transport class, and the hue and carriage sections, rebuilt from
// the vertices in one pass.
//
// AT1 — THE POSITION INDEXES ARE GONE. `by_address`, `by_rebased`,
// `by_unfolded`, `demands`, `glued` and `star` mapped a coordinate to a
// vertex's place in a list; the vertices are keyed by their coordinates now,
// so each of those is the carrier's own stratum and a keyed read of it. What
// stays is what the carrier's key does not order: a chart's vertices and a
// class's, each holding the vertices' coordinates.

use super::*;

impl Form {
    /// Fill `by_class`, `by_transport`, `carriages` and `hue_maps` from the
    /// vertices, in one pass. Two gauges on one word class are two hue
    /// sections, each kept. Two carriages on one transport class stay two
    /// sections. Two Word vertices on one chart are both on that chart's
    /// fiber (24.1).
    pub fn reindex(&mut self) {
        let mut by_class: Keyed<Vec<coord::VertexAt>, coord::Chart> = Keyed::new();
        let mut by_transport: Keyed<Vec<coord::VertexAt>, coord::Transport> = Keyed::new();
        let mut hue_maps = std::mem::take(&mut self.hue_maps);
        let mut places: Keyed<Versor> = Keyed::new();
        for (_, v) in self.vertices.scoped() {
            let (Some(at), Some(tk)) = (v.at(), v.class()) else { continue };
            // THE TRANSPORT CLASS IS THE VERTEX'S, not a member's, so this
            // files the vertex once however many charts it acts on.
            match by_transport.at_mut(&tk) {
                Some(on) => on.push(at.clone()),
                None => {
                    by_transport.put(&tk, vec![at.clone()]);
                }
            }
            let Vertex::Word { derivation, acts, .. } = v else { continue };
            let rotor = derivation.rotor();
            // EVERY CHART THIS VERTEX ACTS ON is a chart it stands at, so
            // `by_class` files it under each of them.
            for (_, a) in acts.scoped() {
                let chart_key = coord::Chart(a.chart.clone());
                let ck = chart_key.encode().as_bytes().to_vec();
                match by_class.at_mut(&chart_key) {
                    Some(on) => {
                        if !on.contains(&at) {
                            on.push(at.clone());
                        }
                    }
                    None => {
                        by_class.put(&chart_key, vec![at.clone()]);
                    }
                }
                // `hue_maps` and `carriages` are DERIVATIONS OF THE FIBRE and
                // are rebuilt from it here, or they are a second carrier for
                // the fact the fibre holds (law 12, D7).
                if let Some(hk) = hue_section_key_of(&rotor, &a.gauge) {
                    hue_maps.insert(hk, a.gauge.clone());
                }
                if let Some(tk) = vertex_transport_key_of(&rotor, &a.gauge) {
                    let mut key = tk.into_vec();
                    crate::transducer::address::put_key_bytes(&mut key, &ck);
                    places.insert(key, (*a.carriage).clone());
                }
            }
        }
        let mut carriages = Keyed::new();
        for (k, p) in places.scoped() {
            carriages.insert(&k, p.clone());
        }
        self.by_class = by_class;
        self.by_transport = by_transport;
        self.carriages = carriages;
        self.hue_maps = hue_maps;
    }

    /// THE SECTIONS ALONG `e₃` (51.8), READ OFF THE MEMBERS (AT1, ruled):
    /// each member's frames at the member's depth, and at a class's depth
    /// the union over the class's members. It was a carrier of its own
    /// holding what the members now say; it retired, and every reader asks
    /// here, so the frames have one carrier (law 12).
    pub fn sections(&self) -> Keyed<Keyed<()>, coord::SectionAt> {
        let mut out: Keyed<Keyed<()>, coord::SectionAt> = Keyed::new();
        for (_, v) in self.vertices.scoped() {
            let (Vertex::Word { address, acts, .. }, Some(class)) = (v, v.class()) else { continue };
            let mut union: Keyed<()> = Keyed::new();
            for (_, a) in acts.scoped() {
                let member = coord::SectionAt::Member(Box::new(coord::MemberSection {
                    class: class.clone(),
                    chart: a.chart.clone(),
                    address: address.clone(),
                    carriage: (*a.carriage).clone(),
                }));
                for (f, ()) in a.frames.scoped() {
                    union.insert(&f, ());
                }
                out.put(&member, a.frames.clone());
            }
            let at = coord::SectionAt::Class(class);
            match out.at_mut(&at) {
                Some(held) => {
                    for (f, ()) in union.scoped() {
                        held.insert(&f, ());
                    }
                }
                None => {
                    out.put(&at, union);
                }
            }
        }
        out
    }

    /// THE GLUED VERTICES, a fibre read at the Glued kind's own stratum, as
    /// [`Form::demands`] reads the demands.
    pub fn glued_vertices(&self) -> impl Iterator<Item = (coord::VertexAt, &Vertex)> {
        self.vertices
            .fiber_at(&[coord::VertexAt::GLUED])
            .into_iter()
            .flatten()
            .filter_map(|(k, v)| Some((coord::VertexAt::decode(&k).ok()?, v)))
    }

    /// THE DEMAND VERTICES, a fibre read at the Demand kind's own stratum:
    /// the carrier is keyed by the vertex's coordinate, which opens with its
    /// kind, so every demand stands under one prefix.
    pub fn demands(&self) -> impl Iterator<Item = (coord::VertexAt, &Vertex)> {
        self.vertices
            .fiber_at(&[coord::VertexAt::DEMAND])
            .into_iter()
            .flatten()
            .filter_map(|(k, v)| Some((coord::VertexAt::decode(&k).ok()?, v)))
    }
}
