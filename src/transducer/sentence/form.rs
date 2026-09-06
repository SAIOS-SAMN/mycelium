// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Sentence form: vertices, faces, loops, and the tree gauge.

use super::chain::Cell;
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

/// Face transport: the relative word, translation and hue map from vertex
/// `i` to vertex `j`. The same three coordinates `transport_class` keys on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transport {
    pub word: Word,
    pub shift: (num_rational::BigRational, num_rational::BigRational),
    pub gauge: Vec<(u8, u8)>,
}

impl Transport {
    pub fn from_word(word: Word) -> Transport {
        Transport {
            word,
            shift: q2_zero(),
            gauge: Vec::new(),
        }
    }

    pub fn inverse(&self) -> Option<Transport> {
        let word = self.word.inverse();
        let mapped = apply_linear(&word, &self.shift)?;
        Some(Transport {
            word,
            shift: (-mapped.0, -mapped.1),
            gauge: gauge_invert(&self.gauge),
        })
    }
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

pub(crate) fn q2_zero() -> (num_rational::BigRational, num_rational::BigRational) {
    let z = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    (z.clone(), z)
}

pub(crate) fn apply_linear(
    word: &Word,
    t: &(num_rational::BigRational, num_rational::BigRational),
) -> Option<(num_rational::BigRational, num_rational::BigRational)> {
    let b = word.plane_block()?;
    Some((
        &b[0][0] * &t.0 + &b[0][1] * &t.1,
        &b[1][0] * &t.0 + &b[1][1] * &t.1,
    ))
}

pub(crate) fn hue_key(h: u8) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[0] = h;
    k
}

pub(crate) fn gauge_invert(g: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let mut inv: Vec<(u8, u8)> = g.iter().map(|&(a, b)| (b, a)).collect();
    inv.sort_unstable();
    inv.dedup();
    inv
}

pub(crate) fn gauge_keyed(g: &[(u8, u8)]) -> Keyed<u8> {
    let mut m = Keyed::new();
    for &(a, b) in g {
        m.insert(hue_key(a), b);
    }
    m
}

pub(crate) fn gauge_image(g: &Keyed<u8>, h: u8) -> u8 {
    match g.get(&hue_key(h)) {
        Some(&x) => x,
        None => h,
    }
}

/// `second ∘ first` on the union of listed hues; a hue absent from a map
/// is fixed by that map. The listing holds non-fixed points only.
pub(crate) fn compose_gauge(second: &[(u8, u8)], first: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let a = gauge_keyed(first);
    let b = gauge_keyed(second);
    let mut hues: Keyed<()> = Keyed::new();
    for &(x, y) in first.iter().chain(second.iter()) {
        hues.insert(hue_key(x), ());
        hues.insert(hue_key(y), ());
    }
    let mut out = Vec::new();
    for (k, _) in hues.fiber(&ROOT) {
        let h = k[0];
        let image = gauge_image(&b, gauge_image(&a, h));
        if image != h {
            out.push((h, image));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

pub(crate) fn relative_gauge(from: &[(u8, u8)], to: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let fm = gauge_keyed(from);
    let tm = gauge_keyed(to);
    let mut hues: Keyed<()> = Keyed::new();
    for &(x, y) in from.iter().chain(to.iter()) {
        hues.insert(hue_key(x), ());
        hues.insert(hue_key(y), ());
    }
    let mut out = Vec::new();
    for (k, _) in hues.fiber(&ROOT) {
        let h = k[0];
        let b = gauge_image(&fm, h);
        let c = gauge_image(&tm, h);
        if b != c {
            out.push((b, c));
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn vertex_translation(v: &Vertex, form: &Form) -> (num_rational::BigRational, num_rational::BigRational) {
    match v {
        Vertex::Word { placement, .. } => placement.clone(),
        Vertex::Rebased { by, shift, .. } => match form.vertices.get(*by as usize) {
            Some(Vertex::Word { placement, .. }) => {
                (placement.0.clone() + &shift.0, placement.1.clone() + &shift.1)
            }
            _ => shift.clone(),
        },
        _ => q2_zero(),
    }
}

fn vertex_gauge(v: &Vertex) -> Vec<(u8, u8)> {
    match v {
        Vertex::Word { gauge, .. } | Vertex::Rebased { gauge, .. } => gauge.clone(),
        _ => Vec::new(),
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
        }
    }
}

impl Form {
    /// Fill by_address, star, placements (by transport-class key) and
    /// hue_maps from the vertices and faces. Two gauges or two placements
    /// on one class are two sections: that class is refused as one map.
    pub fn reindex(&mut self) {
        let mut by_address = Keyed::new();
        let mut hues: Keyed<Vec<(u8, u8)>> = Keyed::new();
        let mut hue_conflict: Keyed<()> = Keyed::new();
        let mut places: Keyed<(
            num_rational::BigRational,
            num_rational::BigRational,
        )> = Keyed::new();
        let mut place_conflict: Keyed<()> = Keyed::new();
        let mut star = Keyed::new();
        for (i, v) in self.vertices.iter().enumerate() {
            if let Vertex::Word {
                address,
                word,
                placement,
                gauge,
                class,
                ..
            } = v
            {
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
        let ta = vertex_translation(va, self);
        let tb = vertex_translation(vb, self);
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
        self.vertices
            .iter()
            .filter_map(|v| match v {
                Vertex::Glued { depth, .. } => Some(depth.saturating_add(1)),
                _ => None,
            })
            .max()
            .unwrap_or(0)
    }

    pub fn is_cauterized(&self) -> bool {
        if !self.remainder.is_empty() {
            return false;
        }
        for lp in &self.loops {
            if lp.holonomy.abelian_identity() {
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

/// declaration order is the walk order and is not a preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Sheet(u8),
    Unit(u8),
    Prime5(u8),
    Prime13(u8),
    Hue,
    Placement,
    OffPlane,
    Open,
}

impl Axis {
    /// One tape byte: the kind in the high bits, the slot plane in the low.
    pub(crate) fn to_byte(self) -> u8 {
        match self {
            Axis::Sheet(p) => p,
            Axis::Unit(p) => (1 << 3) | p,
            Axis::Prime5(p) => (2 << 3) | p,
            Axis::Prime13(p) => (3 << 3) | p,
            Axis::Hue => 4 << 3,
            Axis::Placement => 5 << 3,
            Axis::OffPlane => 6 << 3,
            Axis::Open => 7 << 3,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Axis::Sheet(_) => "sheet",
            Axis::Unit(_) => "unit",
            Axis::Prime5(_) => "prime5",
            Axis::Prime13(_) => "prime13",
            Axis::Hue => "hue",
            Axis::Placement => "placement",
            Axis::OffPlane => "offplane",
            Axis::Open => "open",
        }
    }

    pub(crate) fn from_byte(b: u8) -> Option<Axis> {
        let p = b & 7;
        match (b >> 3, p) {
            (0, p) if p < 6 => Some(Axis::Sheet(p)),
            (1, p) if p < 6 => Some(Axis::Unit(p)),
            (2, p) if p < 6 => Some(Axis::Prime5(p)),
            (3, p) if p < 6 => Some(Axis::Prime13(p)),
            (4, 0) => Some(Axis::Hue),
            (5, 0) => Some(Axis::Placement),
            (6, 0) => Some(Axis::OffPlane),
            (7, 0) => Some(Axis::Open),
            _ => None,
        }
    }
}

/// A18: the dimension audit of a nerve or form, a vector never summed.
/// `torsion` counts the open finite-order loops; `free` is the ℤ-rank of
/// the free parts of the open plane loops plus the open off-plane loops;
/// `axes` the chart axes opened by discharge (none on a nerve, whose atlas
/// is the contemplation's).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rank {
    pub torsion: u32,
    pub free: u32,
    pub axes: u32,
}

impl Rank {
    /// A pivot is admissible when it leaves rank equal or lowers `torsion`
    /// or `free` while `axes` does not fall (12.6, R2).
    pub fn admits(before: Rank, after: Rank) -> bool {
        after.torsion <= before.torsion && after.free <= before.free && after.axes >= before.axes
    }

    pub(crate) fn to_bytes(self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.torsion.to_le_bytes());
        out.extend_from_slice(&self.free.to_le_bytes());
        out.extend_from_slice(&self.axes.to_le_bytes());
    }

    pub(crate) fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Rank> {
        let mut f = [0u32; 3];
        for x in &mut f {
            *x = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
            *pos += 4;
        }
        Some(Rank {
            torsion: f[0],
            free: f[1],
            axes: f[2],
        })
    }
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
    /// μ⃗ of the form over the bundle before the move.
    pub mu_before: [u64; 4],
    /// μ⃗ after the trial move; `select_discharge` reads this.
    pub mu_after: [u64; 4],
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
        for m in self.mu_before.iter().chain(&self.mu_after) {
            out.extend_from_slice(&m.to_le_bytes());
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
        let mut mu_before = [0u64; 4];
        let mut mu_after = [0u64; 4];
        for m in mu_before.iter_mut().chain(&mut mu_after) {
            *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
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
            mu_before,
            mu_after,
            by,
        })
    }
}
