// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Sentence form: vertices, faces, loops, and the tree gauge.

use super::chain::Cell;
use super::class::{Axis, Rank};
use super::transport::{
    apply_linear, relative_gauge, vertex_gauge, vertex_translation, Transport,
};
use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::alphabet::{self, Order, Rotor, Word};
use crate::transducer::element::{affine_of, Versor};
use crate::transducer::peel::ShellKind;
use crate::transducer::stalk;

/// How an unfolding stops. The step count is an instance coordinate, never
/// stored on the form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Measure {
    Occupied,
    Remainder,
    Ring { anchor: u16 },
}

/// A vertex of a sentence form (chapter 18).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vertex {
    Frame,
    Word {
        address: [u8; 32],
        word: Word,
        rho: num_rational::BigRational,
        placement: (num_rational::BigRational, num_rational::BigRational),
        gauge: Vec<(u8, u8)>,
        class: stalk::Invariant,
        /// Stop ring. A form coordinate with ρ, placement, gauge; not M1.
        contact: Option<alphabet::Contact>,
    },
    Glued {
        address: [u8; 32],
        chart: stalk::Invariant,
        magnitude: [u64; 4],
        depth: u16,
        placement: Word,
    },
    /// Vertex `of` read in its own frame as `by`'s word (3.2): the linear
    /// word is `by`'s, realised at `placement_by + shift`; the body it
    /// speaks to, its hue map and its dilation stay `of`'s own, so a
    /// rebased body keeps its shape, its colours and its scale.
    Rebased {
        by: u16,
        of: u16,
        /// Relative placement t = p_of − p_by.
        shift: (num_rational::BigRational, num_rational::BigRational),
        /// `of`'s dilation.
        rho: num_rational::BigRational,
        /// `of`'s hue map, spoken on `of`'s bodies.
        gauge: Vec<(u8, u8)>,
        /// `of`'s chart: the section coordinate naming the bodies it speaks to.
        class: stalk::Invariant,
    },
    Unfolded { measure: Measure },
}

/// Face of a `Form`: two vertices and the transport between them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormFace {
    pub i: u16,
    pub j: u16,
    pub transport: Transport,
}

impl FormFace {
    pub fn new(i: u16, j: u16, word: Word) -> FormFace {
        FormFace {
            i,
            j,
            transport: Transport::from_word(word),
        }
    }
}

/// Independent loop of a form: the closed vertex walk, the faces along it,
/// the chord (never a star face), and the oriented holonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loop {
    pub vertices: Vec<u16>,
    pub faces: Vec<u16>,
    pub chord: u16,
    pub holonomy: Word,
}

/// A sentence as vertices, faces, loops, remainder.
#[derive(Debug, Clone)]
pub struct Form {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<FormFace>,
    pub loops: Vec<Loop>,
    pub remainder: Vec<Cell>,
    pub depth: u16,
    pub hue_maps: Keyed<Vec<(u8, u8)>>,
    pub ratio: ((usize, usize), (usize, usize)),
    pub sections: Keyed<Vec<u16>>,
    /// The certified pivots this form's crystallization rests on (A20).
    /// Evidence, replayed by the court; never part of the address.
    pub certificates: Keyed<PivotCertificate>,
    /// Filed torsion cycles, keyed by class key: `(σ, ℓ, chord face)`.
    /// The form owns this carrier; the element reads it whole.
    pub torsion: Keyed<(Vec<(u8, u8)>, u32, u16)>,
    /// Cited form address → glued vertex index. Written by [`Form::glue`]
    /// and rebuilt by the decoder from Glued vertices.
    pub glued: Keyed<u16>,
    /// Incomparable discharges held as cells until a glue or ruling.
    /// Outer key is the after class; inner is the discharging rotor A1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// Word vertex address → index. The meet for A6 and Mayer–Vietoris.
    pub by_address: Keyed<u16>,
    /// Vertex index key → Frame face index. Star incidence, one face per vertex.
    pub star: Keyed<u16>,
    /// Word vertex address → placement, filled at construction.
    pub placements: Keyed<(num_rational::BigRational, num_rational::BigRational)>,
    /// Rebased vertex keyed by the `of` Word address.
    pub by_rebased: Keyed<u16>,
    /// Unfolded vertex keyed by its measure key.
    pub by_unfolded: Keyed<u16>,
}

impl Default for Form {
    fn default() -> Self {
        Form {
            vertices: vec![Vertex::Frame],
            faces: Vec::new(),
            loops: Vec::new(),
            remainder: Vec::new(),
            depth: 0,
            hue_maps: Keyed::new(),
            ratio: ((1, 1), (1, 1)),
            sections: Keyed::new(),
            certificates: Keyed::new(),
            torsion: Keyed::new(),
            glued: Keyed::new(),
            vortex: Keyed::new(),
            by_address: Keyed::new(),
            star: Keyed::new(),
            placements: Keyed::new(),
            by_rebased: Keyed::new(),
            by_unfolded: Keyed::new(),
        }
    }
}

impl Form {
    /// Fill by_address, star, placements (by transport-class key) and
    /// hue_maps from the vertices and faces. Two gauges or two placements
    /// on one class are two sections: that class is refused as one map.
    pub fn reindex(&mut self) {
        let mut by_address = Keyed::new();
        let mut by_rebased = Keyed::new();
        let mut by_unfolded = Keyed::new();
        let mut hues: Keyed<Vec<(u8, u8)>> = Keyed::new();
        let mut hue_conflict: Keyed<()> = Keyed::new();
        let mut places: Keyed<(
            num_rational::BigRational,
            num_rational::BigRational,
        )> = Keyed::new();
        let mut place_conflict: Keyed<()> = Keyed::new();
        let mut star = Keyed::new();
        for (i, v) in self.vertices.iter().enumerate() {
            match v {
                Vertex::Word {
                    address,
                    word,
                    placement,
                    gauge,
                    class,
                    ..
                } => {
                    by_address.insert(*address, i as u16);
                    let ck = class.key();
                    match hues.get(&ck) {
                        None => {
                            hues.insert(ck, gauge.clone());
                        }
                        Some(held) if held != gauge => {
                            hue_conflict.insert(ck, ());
                        }
                        _ => {}
                    }
                    let tk = vertex_transport_key(word, gauge);
                    match places.get(&tk) {
                        None => {
                            places.insert(tk, placement.clone());
                        }
                        Some(held) if held != placement => {
                            place_conflict.insert(tk, ());
                        }
                        _ => {}
                    }
                }
                Vertex::Rebased { of, .. } => {
                    if let Some(Vertex::Word { address, .. }) = self.vertices.get(*of as usize) {
                        by_rebased.insert(*address, i as u16);
                    }
                }
                Vertex::Unfolded { measure } => {
                    by_unfolded.insert(unfolded_key(measure), i as u16);
                }
                _ => {}
            }
        }
        let mut hue_maps = Keyed::new();
        for (k, g) in hues.fiber(&ROOT) {
            if !hue_conflict.contains(k) {
                hue_maps.insert(*k, g.clone());
            }
        }
        let mut placements = Keyed::new();
        for (k, p) in places.fiber(&ROOT) {
            if !place_conflict.contains(k) {
                placements.insert(*k, p.clone());
            }
        }
        for (fi, f) in self.faces.iter().enumerate() {
            if f.i == 0 {
                star.insert(vertex_ix_key(f.j), fi as u16);
            } else if f.j == 0 {
                star.insert(vertex_ix_key(f.i), fi as u16);
            }
        }
        self.by_address = by_address;
        self.by_rebased = by_rebased;
        self.by_unfolded = by_unfolded;
        self.star = star;
        self.placements = placements;
        self.hue_maps = hue_maps;
    }
}

impl Vertex {

    pub fn word_in(&self, form: &Form) -> Option<Word> {
        match self {
            Vertex::Frame | Vertex::Unfolded { .. } => Some(Word::identity()),
            Vertex::Word { word, .. } => Some(word.clone()),
            Vertex::Glued { placement, .. } => Some(placement.clone()),
            Vertex::Rebased { by, .. } => form.vertices.get(*by as usize)?.word_in(form),
        }
    }

    pub fn class(&self) -> stalk::Invariant {
        match self {
            Vertex::Word { class, .. } | Vertex::Rebased { class, .. } => class.clone(),
            Vertex::Glued { chart, .. } => chart.clone(),
            Vertex::Frame | Vertex::Unfolded { .. } => stalk::Invariant::default(),
        }
    }

    pub fn depth(&self) -> u16 {
        match self {
            Vertex::Glued { depth, .. } => *depth,
            _ => 0,
        }
    }
}

impl Form {

    pub fn face_of(&self, i: u16, j: u16) -> Option<Transport> {
        let va = self.vertices.get(i as usize)?;
        let vb = self.vertices.get(j as usize)?;
        let wa = va.word_in(self)?;
        let wb = vb.word_in(self)?;
        let word = wb.compose(&wa.inverse()).normal_form();
        let ta = vertex_translation(va, self)?;
        let tb = vertex_translation(vb, self)?;
        let mapped = apply_linear(&word, &ta)?;
        let shift = (tb.0 - mapped.0, tb.1 - mapped.1);
        let gauge = relative_gauge(&vertex_gauge(va), &vertex_gauge(vb));
        Some(Transport { word, shift, gauge })
    }

    pub fn delta1(&self) -> Vec<(u16, Word)> {
        self.loops
            .iter()
            .enumerate()
            .map(|(i, lp)| (i as u16, lp.holonomy.clone()))
            .collect()
    }

    pub fn coheres(&self) -> bool {
        self.delta1()
            .into_iter()
            .all(|(_, w)| w.rotor().projective_eq(&Rotor::identity()))
    }

    pub fn is_closed(&self) -> bool {
        self.coheres() && self.remainder.is_empty()
    }

    pub fn depth_of(&self) -> u16 {
        let mut d = 0u16;
        for v in &self.vertices {
            if let Vertex::Glued { depth, .. } = v {
                d = d.max(depth.saturating_add(1));
            }
        }
        d
    }

    pub fn is_cauterized(&self) -> bool {
        if !self.remainder.is_empty() {
            return false;
        }
        for lp in &self.loops {
            if lp.holonomy.is_identity() {
                continue;
            }
            match lp.holonomy.order() {
                Order::Infinite => return false,
                Order::Finite(ell) => {
                    let k = lp.holonomy.rotor().class_key();
                    match self.torsion.get(&k) {
                        Some((_, e, _)) if *e == ell => {}
                        _ => return false,
                    }
                }
            }
        }
        true
    }
}

pub(crate) fn form_face_step(faces: &[FormFace], fi: usize, from: usize, to: usize) -> Option<Word> {
    let f = faces.get(fi)?;
    if f.i as usize == from && f.j as usize == to {
        Some(f.transport.word.clone())
    } else {
        Some(f.transport.word.inverse())
    }
}

pub(crate) fn form_named_holonomy(faces: &[FormFace], path: &[usize], walked: &[usize]) -> Word {
    let mut w = Word::identity();
    for (win, &fi) in path.windows(2).zip(walked) {
        if let Some(s) = form_face_step(faces, fi, win[0], win[1]) {
            w = w.compose(&s);
        }
    }
    w.normal_form()
}

/// The tree path from `src` to `dst` as its vertices and the faces walked
/// between them, named in the same pass that finds them: each vertex is
/// reached through one face of its own adjacency, so no face is searched
/// for afterwards.
pub(crate) fn tree_path(adj: &[Vec<(usize, usize)>], src: usize, dst: usize) -> Option<(Vec<usize>, Vec<usize>)> {
    if src == dst {
        return Some((vec![src], Vec::new()));
    }
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; adj.len()];
    prev[src] = Some((src, 0));
    let mut q = vec![src];
    let mut qi = 0usize;
    while qi < q.len() {
        let u = q[qi];
        qi += 1;
        if u == dst {
            break;
        }
        for &(v, fi) in &adj[u] {
            if prev[v].is_some() {
                continue;
            }
            prev[v] = Some((u, fi));
            q.push(v);
        }
    }
    if prev[dst].is_none() {
        return None;
    }
    let mut path = vec![dst];
    let mut walked = Vec::new();
    let mut cur = dst;
    while cur != src {
        let (p, fi) = prev[cur]?;
        walked.push(fi);
        path.push(p);
        cur = p;
    }
    path.reverse();
    walked.reverse();
    Some((path, walked))
}

pub(crate) fn spell_versor(v: &Versor) -> Option<Word> {
    let a = affine_of(v)?;
    if a.len() < 2 || a[0].len() < 2 {
        return None;
    }
    let block = [[a[0][0].clone(), a[0][1].clone()], [a[1][0].clone(), a[1][1].clone()]];
    Some(alphabet::spell(&block)?.word)
}

/// Transport class of a conformal versor: the spelled word's class key.
pub fn class_key_of_versor(v: &Versor) -> Option<[u8; 32]> {
    Some(spell_versor(v)?.class_key())
}

pub fn vertex_ix_key(i: u16) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..2].copy_from_slice(&i.to_le_bytes());
    k
}

pub(crate) fn unfolded_key(measure: &Measure) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[0] = 4;
    match measure {
        Measure::Occupied => k[1] = 0,
        Measure::Remainder => k[1] = 1,
        Measure::Ring { anchor } => {
            k[1] = 2;
            k[2..4].copy_from_slice(&anchor.to_le_bytes());
        }
    }
    k
}

pub(crate) fn vertex_transport_key(word: &Word, gauge: &[(u8, u8)]) -> [u8; 32] {
    let mut buf = Vec::new();
    buf.push(ShellKind::Whole.as_u8());
    buf.extend_from_slice(&word.class_key());
    buf.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
    for &(b, a) in gauge {
        buf.push(b);
        buf.push(a);
    }
    *blake3::hash(&buf).as_bytes()
}

pub(crate) fn take_u16(b: &[u8], pos: &mut usize) -> Option<u16> {
    let v = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?);
    *pos += 2;
    Some(v)
}

pub(crate) fn take_count(b: &[u8], pos: usize, n: usize, item: usize) -> Option<usize> {
    if item == 0 {
        return Some(n);
    }
    let left = b.len().saturating_sub(pos);
    if item != 0 && n > left / item {
        return None;
    }
    Some(n)
}

/// Which side of the chord face a held word is written on: the face word
/// `g` becomes `g · w⁻¹` (Right) or `w · g` (Left). Both are the relation
/// "the loop's holonomy is this held word"; on a non-abelian chart they are
/// two vertices and two candidates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// The move a certificate records (A20 `by`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PivotBy {
    /// A held word written onto the chord face `face` as the relation the
    /// bundle was missing on that loop.
    Word { face: u16, word: Word, side: Side },
    /// The translation residue, subtracted from the chord shift (A4).
    Placement {
        face: u16,
        shift: (num_rational::BigRational, num_rational::BigRational),
    },
    /// The hue cycle type, composed as inverse onto the chord gauge (A4).
    Hue { face: u16, gauge: Vec<(u8, u8)> },
}

impl PivotBy {
    pub(crate) fn face(&self) -> u16 {
        match self {
            PivotBy::Word { face, .. }
            | PivotBy::Placement { face, .. }
            | PivotBy::Hue { face, .. } => *face,
        }
    }

    pub(crate) fn move_key(&self) -> [u8; 32] {
        match self {
            PivotBy::Word { word, .. } => word.rotor().key(),
            PivotBy::Placement { shift, .. } => {
                let mut buf = Vec::new();
                alphabet::put_q(&mut buf, &shift.0);
                alphabet::put_q(&mut buf, &shift.1);
                *blake3::hash(&buf).as_bytes()
            }
            PivotBy::Hue { gauge, .. } => {
                let mut buf = Vec::new();
                for &(a, b) in gauge {
                    buf.push(a);
                    buf.push(b);
                }
                *blake3::hash(&buf).as_bytes()
            }
        }
    }
}

/// μ⃗ on a certificate: measured over the bundle, or unmeasured.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mu {
    Measured { before: [u64; 4], after: [u64; 4] },
    Unmeasured,
}

/// A certified discharge of one occupied axis (A20). Content-addressed:
/// two minds that discharge one loop by one move produce one key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PivotCertificate {
    /// The loop's index in the walk before the move.
    pub chord: u16,
    pub axis: Axis,
    /// Class key of κ(ℓ) before, and after (`CLOSED_KEY` when the loop closed).
    pub before: [u8; 32],
    pub after: [u8; 32],
    pub rank_before: Rank,
    pub rank_after: Rank,
    /// μ⃗ of the form over the bundle before and after the move.
    pub mu: Mu,
    pub by: PivotBy,
}

impl PivotCertificate {
    pub fn key(&self) -> [u8; 32] {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.chord.to_le_bytes());
        buf.push(self.axis.to_byte());
        buf.extend_from_slice(&self.before);
        buf.extend_from_slice(&self.after);
        self.rank_before.to_bytes(&mut buf);
        self.rank_after.to_bytes(&mut buf);
        match &self.by {
            PivotBy::Word { face, word, side } => {
                buf.extend_from_slice(&face.to_le_bytes());
                buf.push(match side {
                    Side::Left => 0,
                    Side::Right => 1,
                });
                buf.extend_from_slice(&word.key());
            }
            PivotBy::Placement { face, shift } => {
                buf.push(1);
                buf.extend_from_slice(&face.to_le_bytes());
                alphabet::put_q(&mut buf, &shift.0);
                alphabet::put_q(&mut buf, &shift.1);
            }
            PivotBy::Hue { face, gauge } => {
                buf.push(2);
                buf.extend_from_slice(&face.to_le_bytes());
                buf.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
                for &(a, b) in gauge {
                    buf.push(a);
                    buf.push(b);
                }
            }
        }
        *blake3::hash(&buf).as_bytes()
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.chord.to_le_bytes());
        out.push(self.axis.to_byte());
        out.extend_from_slice(&self.before);
        out.extend_from_slice(&self.after);
        self.rank_before.to_bytes(out);
        self.rank_after.to_bytes(out);
        match &self.mu {
            Mu::Unmeasured => out.push(0),
            Mu::Measured { before, after } => {
                out.push(1);
                for m in before.iter().chain(after) {
                    out.extend_from_slice(&m.to_le_bytes());
                }
            }
        }
        match &self.by {
            PivotBy::Word { face, word, side } => {
                out.push(0);
                out.extend_from_slice(&face.to_le_bytes());
                out.push(match side {
                    Side::Left => 0,
                    Side::Right => 1,
                });
                word.to_bytes(out);
            }
            PivotBy::Placement { face, shift } => {
                out.push(1);
                out.extend_from_slice(&face.to_le_bytes());
                alphabet::put_q(out, &shift.0);
                alphabet::put_q(out, &shift.1);
            }
            PivotBy::Hue { face, gauge } => {
                out.push(2);
                out.extend_from_slice(&face.to_le_bytes());
                out.extend_from_slice(&(gauge.len() as u16).to_le_bytes());
                for &(a, b) in gauge {
                    out.push(a);
                    out.push(b);
                }
            }
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<PivotCertificate> {
        let chord = take_u16(b, pos)?;
        let axis = Axis::from_byte(*b.get(*pos)?)?;
        *pos += 1;
        let before: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let after: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
        *pos += 32;
        let rank_before = Rank::from_bytes(b, pos)?;
        let rank_after = Rank::from_bytes(b, pos)?;
        let mu_tag = *b.get(*pos)?;
        *pos += 1;
        let mu = match mu_tag {
            0 => Mu::Unmeasured,
            1 => {
                let mut before = [0u64; 4];
                let mut after = [0u64; 4];
                for m in before.iter_mut().chain(&mut after) {
                    *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                }
                Mu::Measured { before, after }
            }
            _ => return None,
        };
        let kind = *b.get(*pos)?;
        *pos += 1;
        let by = match kind {
            0 => {
                let face = take_u16(b, pos)?;
                let side = match *b.get(*pos)? {
                    0 => Side::Left,
                    1 => Side::Right,
                    _ => return None,
                };
                *pos += 1;
                let word = Word::from_bytes(b, pos)?;
                PivotBy::Word { face, word, side }
            }
            1 => {
                let face = take_u16(b, pos)?;
                let s0 = alphabet::take_q(b, pos)?;
                let s1 = alphabet::take_q(b, pos)?;
                PivotBy::Placement {
                    face,
                    shift: (s0, s1),
                }
            }
            2 => {
                let face = take_u16(b, pos)?;
                let ng = take_count(b, *pos, take_u16(b, pos)? as usize, 2)?;
                let mut gauge = Vec::with_capacity(ng);
                for _ in 0..ng {
                    gauge.push((*b.get(*pos)?, *b.get(*pos + 1)?));
                    *pos += 2;
                }
                PivotBy::Hue { face, gauge }
            }
            _ => return None,
        };
        Some(PivotCertificate {
            chord,
            axis,
            before,
            after,
            rank_before,
            rank_after,
            mu,
            by,
        })
    }
}
