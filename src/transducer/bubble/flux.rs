// 78.37(3), FX1 — THE FLUX, READ: per lane, over one orbit, through the
// filaments of the meet graph; and what the extent door lapsed against what
// flux would decide at the extent's end.
//
//! A FILAMENT is a meet-graph edge between two held thoughts. Work flows
//! through closed circuits only: a form SPOKEN out of the meet graph files
//! the edges its circuits traverse — an edge `(a, b)` is traversed where a
//! circuit of the form visits a chart both thoughts stand on, the chart the
//! edge was made from — and when that form's act is SEALED, the act's frame
//! `μ⃗` is credited to each filed edge ONCE, per lane: never multiplied by the
//! circuits through it, never divided among the edges, never summed across
//! lanes. `Φ⃗(e)` is the orbit's credit; a filament whose `Φ⃗` is zero on
//! every lane read no flux. A HELD ring credits nothing (the Orbit act's
//! frame is the whole orbit's, not a ring's) and its edges read `held`,
//! apart. An ISOLATED thought has no filament: its flux is undefined, never
//! zero, and it reads `unfilamented`.
//!
//! READING ONLY. The door — the extent as the window, flux deciding renewal
//! — lands after the recut's first reading; here the extent's lapses are
//! partitioned by what that door would do.

use super::*;

/// ONE WINDOW'S FILAMENTS: those a circuit traversed, those with no flux on
/// any lane, those on a held ring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FluxWindow {
    pub traversed: u64,
    pub zero: u64,
    pub held: u64,
}

/// What the extent lapsed would come to under flux: `ZeroFlux` (every edge
/// zero over the thought's paid extent — it lapses), flux on some lane (it
/// is renewed), unfilamented (no edge — the extent stays its door), held by
/// a ring (the door refuses while the ring stands), unread (the paid extent
/// did not read — the thought was never paid to a height).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderFlux {
    ZeroFlux,
    Renewed,
    Unfilamented,
    HeldByRing,
    Unread,
}

impl UnderFlux {
    pub const ALL: [UnderFlux; 5] =
        [UnderFlux::ZeroFlux, UnderFlux::Renewed, UnderFlux::Unfilamented, UnderFlux::HeldByRing, UnderFlux::Unread];

    pub fn index(self) -> usize {
        match self {
            UnderFlux::ZeroFlux => 0,
            UnderFlux::Renewed => 1,
            UnderFlux::Unfilamented => 2,
            UnderFlux::HeldByRing => 3,
            UnderFlux::Unread => 4,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            UnderFlux::ZeroFlux => "zero flux",
            UnderFlux::Renewed => "renewed",
            UnderFlux::Unfilamented => "unfilamented",
            UnderFlux::HeldByRing => "held by ring",
            UnderFlux::Unread => "unread",
        }
    }
}

/// FX1 — THE FLUX READING. The windows and the thoughts are the last closed
/// orbit's; the acts and the extent's lapses are cumulative at the producer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FluxTally {
    /// Orbits closed and read.
    pub closes: u64,
    /// The last orbit's filaments, and the same over the aperture (the
    /// room's window, `aperture_tu`); `None` where no aperture is measured.
    pub orbit: FluxWindow,
    pub aperture: Option<FluxWindow>,
    /// Edges credited in the last orbit that no longer stand at its close.
    pub gone: u64,
    /// Thoughts on no edge at the last close.
    pub unfilamented: u64,
    /// Acts credited; acts whose spoken form carries no circuit; acts whose
    /// word was spoken from no held thought; spoken forms no act sealed by
    /// the close; circuit nodes at no named chart.
    pub credited: u64,
    pub acyclic: u64,
    pub unfiled: u64,
    pub unsealed: u64,
    pub unnamed: u64,
    /// The escrow the extent lapsed, per lane, by what flux would do at the
    /// extent's end (`UnderFlux::index`), and the thoughts.
    pub extent: [[u64; 4]; 5],
    pub extent_n: [u64; 5],
    /// Alternative B at the last close: dialect words a circuit ran through
    /// this orbit, those over the idle-K door's aperture, and those idle
    /// over it; `None` where that aperture is not measured.
    pub dialect: (u64, Option<u64>, Option<u64>),
}

/// A FORM SPOKEN OUT OF THE MEET GRAPH, filed until its act is sealed: the
/// edges its circuits traversed, the word vertices they ran through, and
/// whether it carried any circuit at all.
#[derive(Clone, Debug, Default)]
pub struct Spoken {
    pub edges: Vec<Vec<u8>>,
    pub words: Vec<Vec<u8>>,
    pub cyclic: bool,
    /// An act has sealed it.
    pub sealed: bool,
}

/// A FORM'S CIRCUITS: the charts each visits, in circuit order; the word
/// vertices on them; the circuit nodes at no named chart.
#[derive(Clone, Debug, Default)]
pub struct Circuits {
    pub charts: Vec<Vec<Vec<u8>>>,
    pub words: Vec<Vec<u8>>,
    pub unnamed: u64,
}

/// A form's circuits, read off its orbits' members.
pub(crate) fn circuits_of(form: &crate::transducer::sentence::Form) -> Circuits {
    let mut circuits = Vec::new();
    let mut words: Keyed<()> = Keyed::new();
    let mut unnamed = 0u64;
    for o in &form.orbits {
        let mut charts = Vec::new();
        // Each chart the circuit leaves, by its Invariant's key; the Frame's
        // chart is its tag alone and names no chart (AT1).
        for c in o.charts() {
            match c.invariant() {
                Some(chart) => charts.push(chart.key().as_bytes().to_vec()),
                None => unnamed += 1,
            }
        }
        for v in o.vertices() {
            if let Some(crate::transducer::sentence::Vertex::Word { address, .. }) = form.vertices.at(&v) {
                words.insert(address.as_bytes(), ());
            }
        }
        circuits.push(charts);
    }
    Circuits { charts: circuits, words: words.scoped().map(|(k, _)| k).collect(), unnamed }
}

impl Contemplation {
    /// THE EDGES A CIRCUIT THROUGH `charts` TRAVERSES: every meet-graph edge
    /// between two thoughts standing on one of its charts.
    fn edges_on(&self, charts: &[Vec<u8>], out: &mut Keyed<()>) {
        for c in charts {
            let at: Vec<Vec<u8>> = self
                .by_chart
                .link(c)
                .filter_map(|(id, _)| Self::ordinal_of_key(&id).and_then(|o| self.address_at(o)).map(<[u8]>::to_vec))
                .collect();
            for (i, a) in at.iter().enumerate() {
                for b in &at[i + 1..] {
                    let k = crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts(vec![a.as_slice(), b.as_slice()]))
                        .as_bytes()
                        .to_vec();
                    if self.complex.degree_of(&k) == Some(1) {
                        out.insert(&k, ());
                    }
                }
            }
        }
    }

    /// A FORM SPEAKS OUT OF THE MEET GRAPH as the element at `addr`, its
    /// circuits read by `circuits_of`: the edges they traverse are filed,
    /// read before its thoughts leave.
    pub(crate) fn file_spoken(&mut self, addr: &[u8], Circuits { charts: circuits, words, unnamed }: Circuits) {
        self.flux_tally.unnamed += unnamed;
        let mut edges: Keyed<()> = Keyed::new();
        for c in &circuits {
            self.edges_on(c, &mut edges);
        }
        let spoken =
            Spoken { edges: edges.scoped().map(|(k, _)| k).collect(), words, cyclic: !circuits.is_empty(), sealed: false };
        self.spoken.insert(addr, spoken);
    }

    /// THE ACT SEALING THE ELEMENT AT `addr` measured `mu`: each edge its
    /// form's circuits traversed is credited `mu` once, per lane. Returns the
    /// word vertices the circuits ran through, for the dialect's reading.
    ///
    /// The entry stands until the orbit's close, so each act on the form —
    /// its standing and its utterance are two — credits its own frame once.
    pub fn credit_flux(&mut self, addr: &[u8], mu: [u64; 4]) -> Vec<Vec<u8>> {
        let Some(s) = self.spoken.get_mut(addr) else {
            self.flux_tally.unfiled += 1;
            return Vec::new();
        };
        s.sealed = true;
        if !s.cyclic {
            self.flux_tally.acyclic += 1;
            return Vec::new();
        }
        let s = s.clone();
        self.flux_tally.credited += 1;
        let flowed = mu.iter().any(|&m| m > 0);
        for e in &s.edges {
            match self.flux.get_mut(e) {
                Some(phi) => {
                    if !crate::core::lanes::lanes_step(phi, &mu) {
                        self.overflowed += 1;
                    }
                }
                None => {
                    self.flux.insert(e, mu);
                }
            }
            if flowed {
                self.flux_at.insert(e, self.now_k);
            }
        }
        s.words
    }

    /// The edges a held ring stands on: each handle the docket indexes it by
    /// — its closing edge, and the edges of its 2-simplices.
    pub(crate) fn held_edges(&self) -> Keyed<()> {
        let mut out: Keyed<()> = Keyed::new();
        for (h, _) in self.docket_rings.scoped() {
            match self.complex.degree_of(&h) {
                Some(1) => {
                    out.insert(&h, ());
                }
                Some(2) => {
                    if let Some(s) = self.complex.simplex(&h) {
                        for skip in 0..s.vertices.len() {
                            let face: Vec<&[u8]> =
                                s.vertices.iter().enumerate().filter(|(i, _)| *i != skip).map(|(_, v)| v.as_slice()).collect();
                            out.insert(crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts(face)).as_bytes(), ());
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }

    /// Did `edge` carry flux since `from` (on the contemplation's clock), or
    /// this orbit?
    pub(crate) fn flowed_since(&self, edge: &[u8], from: u64) -> bool {
        self.flux_at.get(edge).is_some_and(|&at| at >= from)
    }

    /// THE ORBIT CLOSED: its filaments read — traversed, zero on every lane,
    /// held — and the same over the aperture; the orbit's flux emptied, the
    /// spoken forms no act sealed counted, the stamps of edges no longer
    /// standing dropped.
    pub fn flux_close(&mut self) {
        let held = self.held_edges();
        let from = self.aperture_tu().and_then(|span| self.now_k.checked_sub(span));
        let mut orbit = FluxWindow::default();
        let mut aperture = FluxWindow::default();
        let mut standing: Keyed<()> = Keyed::new();
        if let Some(layer) = self.complex.degrees.get(1) {
            for (k, _) in layer.scoped() {
                standing.insert(&k, ());
                if held.contains(&k) {
                    orbit.held += 1;
                    aperture.held += 1;
                    continue;
                }
                match self.flux.get(&k).is_some_and(|phi| phi.iter().any(|&p| p > 0)) {
                    true => orbit.traversed += 1,
                    false => orbit.zero += 1,
                }
                match from.is_some_and(|f| self.flowed_since(&k, f)) {
                    true => aperture.traversed += 1,
                    false => aperture.zero += 1,
                }
            }
        }
        let t = &mut self.flux_tally;
        t.closes += 1;
        t.orbit = orbit;
        t.aperture = from.map(|_| aperture);
        t.gone = self.flux.scoped().filter(|(k, _)| !standing.contains(k)).count() as u64;
        t.unsealed += self.spoken.scoped().filter(|(_, s)| !s.sealed).count() as u64;
        // MU-CLIENT §4.5 — where the tension relaxed over the measured
        // aperture, the cells over it descend and its tethers drain; an
        // unmeasured aperture releases nothing.
        if let Some(f) = from {
            self.release_relaxed(f);
        }
        self.spoken = Keyed::new();
        self.flux = Keyed::new();
        let stale: Vec<Vec<u8>> = self.flux_at.scoped().filter(|(k, _)| !standing.contains(k)).map(|(k, _)| k).collect();
        for k in stale {
            self.flux_at.remove(&k);
        }
        let unstanding: Vec<Vec<u8>> = self.tethered_at.scoped().filter(|(k, _)| self.complex.degree_of(k) != Some(1)).map(|(k, _)| k).collect();
        for k in unstanding {
            self.tethered_at.remove(&k);
        }
        self.flux_tally.unfilamented =
            self.degrees_among_nerve().scoped().filter(|(_, &d)| d == 0).count() as u64;
    }

    /// WHAT FLUX WOULD DO TO THE THOUGHT AT `key`, whose paid extent ends now.
    pub(crate) fn under_flux(&self, key: &[u8], held: &Keyed<()>) -> UnderFlux {
        let edges: Vec<Vec<u8>> =
            self.complex.link(key, 1).into_iter().filter(|e| self.complex.degree_of(e) == Some(1)).collect();
        if edges.is_empty() {
            return UnderFlux::Unfilamented;
        }
        if edges.iter().any(|e| held.contains(e)) {
            return UnderFlux::HeldByRing;
        }
        // Its OWN paid extent, `[paid_from, until]` — the door's window.
        let Some(from) = self.bubbles.get(key).and_then(|b| b.paid_from) else {
            return UnderFlux::Unread;
        };
        let this_orbit = |e: &Vec<u8>| self.flux.get(e).is_some_and(|phi| phi.iter().any(|&p| p > 0));
        match edges.iter().any(|e| self.flowed_since(e, from) || this_orbit(e)) {
            true => UnderFlux::Renewed,
            false => UnderFlux::ZeroFlux,
        }
    }

    /// The extent is lapsing `doomed`: each read under flux, its escrow
    /// counted by what flux would do.
    pub(crate) fn note_extent(&mut self, doomed: &[Vec<u8>]) {
        if doomed.is_empty() {
            return;
        }
        let held = self.held_edges();
        for k in doomed {
            let Some(escrow) = self.bubbles.get(k).map(|b| b.escrow) else { continue };
            let i = self.under_flux(k, &held).index();
            self.flux_tally.extent_n[i] += 1;
            if !crate::core::lanes::lanes_step(&mut self.flux_tally.extent[i], &escrow) {
                self.overflowed += 1;
            }
        }
    }

    /// Alternative B's numbers at the close, from the epigenome.
    pub fn note_dialect_flux(&mut self, d: (u64, Option<u64>, Option<u64>)) {
        self.flux_tally.dialect = d;
    }
}
