// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 17.4, 3.3, 31 — THE CHART OF A FORM: its M4 coordinates, the glue of a
//! crystallized form into it, the coboundary gauge between two forms, the
//! A21 vertex key, and the `SentenceRule` a form is spoken as.

use super::form::*;
use super::transport::*;
use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::alphabet::Word;
use crate::transducer::element::{self, Element, Versor};
use crate::transducer::stalk;
use super::tape::from_element_form;

impl Form {
    pub fn glue(&mut self, s: &Form, s_addr: crate::transducer::address::Address, placement: Word, magnitude: [u64; 4]) -> Option<u16> {
        if !s.is_closed() {
            return None;
        }
        if !matches!(s.vertices.first(), Some(Vertex::Frame)) {
            return None;
        }
        if !s.vertices[0].word_in(s).is_some_and(|w| w.is_identity()) {
            return None;
        }
        let d = s.depth_of();
        let host = self.depth_of();
        if host > 0 && d >= host {
            return None;
        }
        let chart = chart_of_form(s)?;
        if chart.0.is_empty() {
            return None;
        }
        let idx = self.vertices.len() as u16;
        self.glued.insert(&s_addr, idx);
        self.vertices.push(Vertex::Glued {
            address: s_addr,
            chart,
            magnitude,
            depth: d,
            placement: placement.clone(),
        });
        self.faces.push(FormFace::new(0, idx, placement));
        self.loops = Self::loops_of(&self.faces, self.vertices.len());
        self.depth = self.depth_of();
        self.reindex();
        Some(idx)
    }

    /// Drop the named vertices (keyed by old index), drop faces that touched
    /// them, remap surviving faces through the old-to-new map, rebuild loops.
    pub fn retract_vertices(&mut self, drop: &Keyed<()>) {
        let n = self.vertices.len();
        if n == 0 {
            return;
        }
        let mut old_to_new: Vec<Option<u16>> = vec![None; n];
        let mut new_verts = Vec::new();
        for (i, v) in self.vertices.iter().enumerate() {
            if i != 0 && drop.contains(&vertex_ix_key(i as u16)) {
                continue;
            }
            old_to_new[i] = Some(new_verts.len() as u16);
            new_verts.push(v.clone());
        }
        let mut face_old_to_new: Vec<Option<u16>> = vec![None; self.faces.len()];
        let mut new_faces = Vec::new();
        for (fi, f) in self.faces.iter().enumerate() {
            let Some(&Some(i)) = old_to_new.get(f.i as usize) else {
                continue;
            };
            let Some(&Some(j)) = old_to_new.get(f.j as usize) else {
                continue;
            };
            face_old_to_new[fi] = Some(new_faces.len() as u16);
            new_faces.push(FormFace {
                i,
                j,
                transport: f.transport.clone(),
                edge: f.edge.clone(),
            });
        }
        let mut glued = Keyed::new();
        for (k, &old_i) in self.glued.fiber(ROOT) {
            if let Some(&Some(ni)) = old_to_new.get(old_i as usize) {
                glued.insert(&k, ni);
            }
        }
        let mut sections = Keyed::new();
        for (k, ix) in self.sections.fiber(ROOT) {
            let mapped: Vec<u16> = ix
                .iter()
                .filter_map(|&i| old_to_new.get(i as usize).copied().flatten())
                .collect();
            if !mapped.is_empty() {
                sections.insert(&k, mapped);
            }
        }
        let mut torsion = Keyed::new();
        for (k, FiledLoop { sigma, period: ell, chord }) in self.torsion.fiber(ROOT) {
            let Some(&Some(nc)) = face_old_to_new.get(*chord as usize) else {
                continue;
            };
            let Some(mapped) = sigma
                .iter()
                .map(|&(a, c)| {
                    let i = old_to_new.get(a as usize).copied().flatten()?;
                    let j = old_to_new.get(c as usize).copied().flatten()?;
                    Some((i as u8, j as u8))
                })
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            torsion.insert(&k, FiledLoop { sigma: mapped, period: *ell, chord: nc });
        }
        let mut axes = Keyed::new();
        for (k, chord) in self.axes.fiber(ROOT) {
            let Some(&Some(nc)) = face_old_to_new.get(*chord as usize) else {
                continue;
            };
            axes.insert(&k, nc);
        }
        self.vertices = new_verts;
        self.faces = new_faces;
        self.glued = glued;
        self.sections = sections;
        self.torsion = torsion;
        self.axes = axes;
        self.loops = Self::loops_of(&self.faces, self.vertices.len());
        self.depth = self.depth_of();
        self.reindex();
    }

    /// Replace every face `g_ij` by `γ_j · g_ij · γ_i⁻¹`. Holonomies of
    /// identity stay identity. The gauge's length is the vertex count.
    /// Every new transport is computed first; a form whose gauge cannot
    /// be applied is left unchanged.
    pub fn regauge(&mut self, gamma: &[Word]) -> Option<()> {
        if gamma.len() != self.vertices.len() {
            return None;
        }
        let mut next = Vec::with_capacity(self.faces.len());
        for f in &self.faces {
            let gi = gamma.get(f.i as usize)?;
            let gj = gamma.get(f.j as usize)?;
            let word = gj
                .compose(&f.transport.word.compose(&gi.inverse()))
                .normal_form();
            let gv = element::versor_of_word(gj)?;
            let inv = gv.versor_inverse()?;
            let carriage = gv.geo(&f.transport.carriage).geo(&inv);
            next.push(Transport {
                word,
                carriage,
                gauge: f.transport.gauge.clone(),
            });
        }
        for (f, t) in self.faces.iter_mut().zip(next) {
            f.transport = t;
        }
        self.loops = Self::loops_of(&self.faces, self.vertices.len());
        Some(())
    }

    /// Spanning forest (12.2): each component is rooted at its least vertex
    /// index with γ = 1; γ_j = γ_i · g_ij⁻¹ along tree edges. None only when
    /// a face names a vertex outside the form.
    pub fn solve_coboundary(&self) -> Option<Vec<Word>> {
        let n = self.vertices.len();
        if n == 0 {
            return None;
        }
        let mut adj: Vec<Vec<(usize, usize, bool)>> = vec![Vec::new(); n];
        for (fi, f) in self.faces.iter().enumerate() {
            let (i, j) = (f.i as usize, f.j as usize);
            if i >= n || j >= n {
                return None;
            }
            adj[i].push((j, fi, true));
            adj[j].push((i, fi, false));
        }
        let mut gamma: Vec<Option<Word>> = vec![None; n];
        for root in 0..n {
            if gamma[root].is_some() {
                continue;
            }
            gamma[root] = Some(Word::identity());
            let mut q = vec![root];
            let mut qi = 0usize;
            while qi < q.len() {
                let u = q[qi];
                qi += 1;
                let gu = gamma[u].as_ref()?.clone();
                for &(v, fi, u_is_i) in &adj[u] {
                    let f = &self.faces[fi];
                    let gv = if u_is_i {
                        gu.compose(&f.transport.word.inverse()).normal_form()
                    } else {
                        gu.compose(&f.transport.word).normal_form()
                    };
                    match &gamma[v] {
                        None => {
                            gamma[v] = Some(gv);
                            q.push(v);
                        }
                        Some(held) if held != &gv => {}
                        _ => {}
                    }
                }
            }
        }
        gamma.into_iter().collect()
    }

    /// Replace vertex `of` by `by` read in `of`'s frame (3.2). `of`'s own
    /// dilation, hue map and chart are kept on the Rebased vertex; only
    /// its word is now `by`'s. A vertex that is not a `Word` is left as it
    /// is, and `of` is returned either way.
    pub fn rebase(
        &mut self,
        by: u16,
        of: u16,
        carriage: Versor,
    ) -> Option<u16> {
        if (by as usize) >= self.vertices.len() {
            return None;
        }
        let Vertex::Word {
            gauge,
            class,
            ..
        } = self.vertices.get(of as usize).cloned()?
        else {
            return None;
        };
        self.vertices[of as usize] = Vertex::Rebased {
            by,
            of,
            carriage: Box::new(carriage),
            gauge,
            class,
        };
        Some(of)
    }

    pub fn unfold(&mut self, measure: Measure) -> u16 {
        let idx = self.vertices.len() as u16;
        self.vertices.push(Vertex::Unfolded { measure });
        idx
    }
}

#[derive(Debug, Clone)]
pub struct SentenceRule {
    pub form: Form,
    /// Classes present in some pairs only: class key → pair indices.
    pub sections: Keyed<Vec<u16>>,
    /// Family frame ratio ((h_out, h_in), (w_out, w_in)).
    pub ratio: ((usize, usize), (usize, usize)),
}

impl SentenceRule {
    pub fn section_count(&self) -> usize {
        self.sections
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, v)| v.len())
            .sum()
    }

    /// Hue map of a target chart. One Keyed from the form's Word vertices,
    /// then a read; never a scan.
    pub fn hue_for(&self, inv: &stalk::Invariant) -> Vec<(u8, u8)> {
        match self.form.hue_maps.get(&inv.key()) {
            Some(g) => g.clone(),
            None => Vec::new(),
        }
    }

    /// Carriage of a transport class, read from the form's carriages.
    pub fn carriage_of(&self, key: &[u8]) -> Option<Versor> {
        self.form.carriages.get(key).cloned()
    }
}

/// A form as a `SentenceRule` (chapter 31). Hue maps are filled from the
/// form's own Word vertices when the form does not already carry them.
pub fn rule_of_form(mut form: Form) -> SentenceRule {
    form.reindex();
    let sections = form.sections.clone();
    let ratio = form.ratio;
    SentenceRule {
        form,
        sections,
        ratio,
    }
}



/// Vertex key of a sentence form: Word vertices (M1, carriage, contact,
/// gauge) and face words. The body's absolute position is already quotiented
/// on the chain; the bundle's carriage is meaning.
pub fn vertex_address(e: &Element) -> Option<crate::transducer::address::Address> {
    from_element_form(e).map(|f| form_vertex_key(&f))
}
pub fn form_vertex_key(f: &Form) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_FORM];
    buf.extend_from_slice(&(f.vertices.len() as u32).to_le_bytes());
    for v in &f.vertices {
        match v {
            Vertex::Frame => buf.push(0),
            // 1.5: the key is the COMPOSED product's canonical rotor, so
            // two minds whose derivations differ reach one address. The
            // citations are content the court replays (50.2 step 2) and
            // are deliberately not keyed: keying them would fork one
            // vertex into one address per presentation.
            Vertex::Word {
                derivation,
                carriage,
                gauge,
                contact,
                ..
            } => {
                buf.push(1);
                crate::transducer::address::put_bytes(&mut buf, &derivation.word().key());
                carriage.to_bytes(&mut buf);
                match contact {
                    None => buf.push(0),
                    Some(c) => {
                        buf.push(1);
                        buf.extend_from_slice(&c.anchor.to_le_bytes());
                        buf.push(c.ring);
                    }
                }
                let mut g = gauge.clone();
                g.sort_unstable();
                buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
                for (a, b) in g {
                    buf.push(a);
                    buf.push(b);
                }
            }
            Vertex::Glued {
                address,
                placement,
                depth,
                ..
            } => {
                buf.push(2);
                crate::transducer::address::put_bytes(&mut buf, address);
                crate::transducer::address::put_bytes(&mut buf, &placement.key());
                buf.extend_from_slice(&depth.to_le_bytes());
            }
            Vertex::Rebased {
                by,
                of,
                carriage,
                gauge,
                ..
            } => {
                buf.push(3);
                buf.extend_from_slice(&by.to_le_bytes());
                buf.extend_from_slice(&of.to_le_bytes());
                carriage.to_bytes(&mut buf);
                let mut g = gauge.clone();
                g.sort_unstable();
                buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
                for (a, b) in g {
                    buf.push(a);
                    buf.push(b);
                }
            }
            Vertex::Unfolded { measure } => {
                buf.push(4);
                match measure {
                    Measure::Occupied => buf.push(0),
                    Measure::Remainder => buf.push(1),
                    Measure::Ring { anchor } => {
                        buf.push(2);
                        buf.extend_from_slice(&anchor.to_le_bytes());
                    }
                }
            }
            // 50.5: a demand is named by its kind and chart; the bodies it
            // stands for are the remainder, on the tape and not in the key.
            Vertex::Demand { kind, chart, .. } => {
                buf.push(5);
                crate::transducer::address::put_bytes(&mut buf, kind.key().as_bytes());
                crate::transducer::address::put_bytes(&mut buf, chart.key().as_bytes());
            }
        }
    }
    buf.extend_from_slice(&(f.faces.len() as u32).to_le_bytes());
    for face in &f.faces {
        buf.extend_from_slice(&face.i.to_le_bytes());
        buf.extend_from_slice(&face.j.to_le_bytes());
        crate::transducer::address::put_bytes(&mut buf, &face.transport.word.key());
        if let Some(k) = face.transport.carriage.key() {
            let _ = crate::transducer::address::put_addr(&mut buf, k);
        }
        let mut g = face.transport.gauge.clone();
        g.sort_unstable();
        buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
        for (a, b) in g {
            buf.push(a);
            buf.push(b);
        }
    }
    // The bundle's frame ratio and its sections are meaning (rule 7): two
    // forms that speak the same words onto differently scaled frames, or
    // that section different vertices, are two forms.
    let ((hn, hd), (wn, wd)) = f.ratio;
    for x in [hn, hd, wn, wd] {
        buf.extend_from_slice(&(x as u64).to_le_bytes());
    }
    for (class, members) in f.sections.fiber(crate::network::nerve::ROOT) {
        crate::transducer::address::put_bytes(&mut buf, &class);
        buf.extend_from_slice(&(members.len() as u16).to_le_bytes());
        for m in members {
            buf.extend_from_slice(&m.to_le_bytes());
        }
    }
    // An opened axis is a coordinate of the form (I6): the class it was
    // opened on, written out, key order.
    for (class, _) in f.axes.fiber(crate::network::nerve::ROOT) {
        crate::transducer::address::put_bytes(&mut buf, &class);
    }
    crate::transducer::address::Address::new(buf)
}
/// Chart of a crystallized form (M4 step 3): the form's cell counts,
/// then per vertex its A3-reduced plane form (a rebased vertex reads the
/// word it speaks; a glued vertex contributes the cited chart), sorted so
/// the order of vertices is not a coordinate. None when a word is off-plane.
pub(crate) fn chart_of_form(f: &Form) -> Option<crate::transducer::stalk::Invariant> {
    let mut axes = vec![
        2u64,
        f.vertices.len() as u64,
        f.faces.len() as u64,
        f.loops.len() as u64,
    ];
    let mut rows: Vec<Vec<u64>> = Vec::new();
    for v in &f.vertices {
        match v {
            Vertex::Word { .. } | Vertex::Rebased { .. } => {
                let pf = v.word_in(f)?.form()?;
                let mut row = vec![1u64, pf.orientation as u64];
                row.extend(pf.rot.iter().map(|&x| x as u64));
                rows.push(row);
            }
            Vertex::Glued { chart, .. } => {
                let mut row = vec![3u64];
                row.extend_from_slice(&chart.0);
                rows.push(row);
            }
            Vertex::Demand { kind, chart, .. } => {
                let mut row = vec![4u64, u64::from(kind.kind_byte())];
                row.extend_from_slice(&chart.0);
                rows.push(row);
            }
            Vertex::Frame | Vertex::Unfolded { .. } => {}
        }
    }
    rows.sort();
    for row in rows {
        axes.extend(row);
    }
    Some(crate::transducer::stalk::Invariant(axes))
}

/// Chart of a sentence element. None when the tape does not decode or a
/// vertex is off-plane. Never `[2,1,0,0]`.
pub fn chart_of(e: &Element) -> Option<crate::transducer::stalk::Invariant> {
    chart_of_form(&from_element_form(e)?)
}

pub(crate) fn vertex_match_key(form: &Form, i: u16) -> Option<Vec<u8>> {
    match form.vertices.get(i as usize)? {
        Vertex::Frame => Some(vertex_ix_key(0)),
        Vertex::Word { address, .. } | Vertex::Glued { address, .. } => Some(address.clone().into_vec()),
        Vertex::Rebased { of, .. } => match form.vertices.get(*of as usize) {
            Some(Vertex::Word { address, .. }) => Some(address.clone().into_vec()),
            _ => None,
        },
        Vertex::Unfolded { measure } => Some(unfolded_key(measure)),
        Vertex::Demand { kind, chart, .. } => Some(crate::transducer::sentence::demand_key(kind, chart)),
    }
}

pub(crate) fn ends_key(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::address::put_bytes(&mut buf, a);
    crate::transducer::address::put_bytes(&mut buf, b);
    buf
}

/// Two forms whose cochains differ by a coboundary δγ: the gauge that
/// takes `a` onto `b`. Word and Glued vertices match by address, not index.
pub fn coboundary_gauge(a: &Form, b: &Form) -> Option<Vec<Word>> {
    let mut gamma = Vec::with_capacity(a.vertices.len());
    for va in &a.vertices {
        let vb = match va {
            Vertex::Frame => b.vertices.first().filter(|v| matches!(v, Vertex::Frame))?,
            Vertex::Word { address, .. } => {
                let j = *b.by_address.get(address)?;
                b.vertices.get(j as usize)?
            }
            Vertex::Glued { address, .. } => {
                let j = *b.glued.get(address)?;
                b.vertices.get(j as usize)?
            }
            Vertex::Rebased { of, .. } => {
                let Vertex::Word { address, .. } = a.vertices.get(*of as usize)? else {
                    return None;
                };
                let j = *b.by_rebased.get(address)?;
                b.vertices.get(j as usize)?
            }
            Vertex::Unfolded { measure } => {
                let j = *b.by_unfolded.get(&unfolded_key(measure))?;
                b.vertices.get(j as usize)?
            }
            Vertex::Demand { kind, chart, .. } => {
                let j = *b.demands.get(&crate::transducer::sentence::demand_key(kind, chart))?;
                b.vertices.get(j as usize)?
            }
        };
        gamma.push(vb.word_in(b)?.compose(&va.word_in(a)?.inverse()));
    }
    let mut a_by_ends: Keyed<&FormFace> = Keyed::new();
    for fa in &a.faces {
        let i = vertex_match_key(a, fa.i)?;
        let j = vertex_match_key(a, fa.j)?;
        a_by_ends.insert(ends_key(&i, &j), fa);
    }
    let mut b_by_ends: Keyed<&FormFace> = Keyed::new();
    for fb in &b.faces {
        let i = vertex_match_key(b, fb.i)?;
        let j = vertex_match_key(b, fb.j)?;
        b_by_ends.insert(ends_key(&i, &j), fb);
    }
    for fa in &a.faces {
        let i = vertex_match_key(a, fa.i)?;
        let j = vertex_match_key(a, fa.j)?;
        let fb = b_by_ends.get(&ends_key(&i, &j))?;
        let expect = gamma[fa.j as usize]
            .compose(&fa.transport.word)
            .compose(&gamma[fa.i as usize].inverse());
        if !expect.same_form(&fb.transport.word) {
            return None;
        }
    }
    for fb in &b.faces {
        let i = vertex_match_key(b, fb.i)?;
        let j = vertex_match_key(b, fb.j)?;
        let _fa = a_by_ends.get(&ends_key(&i, &j))?;
    }
    Some(gamma)
}
