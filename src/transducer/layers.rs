// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 20 — THE PEEL: a nested manifold read outer layer to inner with the
//! residue riding inward, continuing while the governor reads `Shrinking`
//! and routing the residue out whole — the standing arc — when it does not.

/// What the governor reads off a residue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerMotion {
    /// Nothing remains: the nest closed.
    Closed,
    /// Strictly less remains than before this layer.
    Shrinking,
    /// The layer did not shrink the residue: the arc rides out.
    Stalled,
}

/// One layer of a nest: from a residue, the core it takes and what it
/// leaves. `None` when the layer cannot take anything whole.
pub trait LayerStep {
    type Residue;
    type Core;
    fn layer(&self, residue: &Self::Residue) -> Option<(Self::Core, Self::Residue)>;
}

/// The governor reads the residue between layers; it carries what it
/// needs to read strict shrinking.
pub trait ShrinkGovernor {
    type Residue;
    fn read(&mut self, residue: &Self::Residue) -> LayerMotion;
}

/// The verdict of a nested peel — the cores outer-to-inner and, when it
/// stalled, the arc that rides out whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeelNest<R, C> {
    Closed { cores: Vec<C> },
    Standing { cores: Vec<C>, arc: R },
}

impl<R, C> PeelNest<R, C> {
    pub fn cores(&self) -> &[C] {
        match self {
            PeelNest::Closed { cores } | PeelNest::Standing { cores, .. } => cores,
        }
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, PeelNest::Closed { .. })
    }
}

/// THE PEEL LAW: iterate the step with the residue riding inward, outer
/// constraining inner by construction. Bounded because a strictly
/// shrinking finite residue reaches empty or stalls.
pub fn peel_nest<S, G>(target: S::Residue, step: &S, governor: &mut G) -> PeelNest<S::Residue, S::Core>
where
    S: LayerStep,
    G: ShrinkGovernor<Residue = S::Residue>,
{
    let mut cores: Vec<S::Core> = Vec::new();
    if let LayerMotion::Closed = governor.read(&target) {
        return PeelNest::Closed { cores };
    }
    let mut arc = target;
    loop {
        let (core, residue) = match step.layer(&arc) {
            Some(layer) => layer,
            None => return PeelNest::Standing { cores, arc },
        };
        cores.push(core);
        match governor.read(&residue) {
            LayerMotion::Closed => return PeelNest::Closed { cores },
            LayerMotion::Shrinking => arc = residue,
            LayerMotion::Stalled => return PeelNest::Standing { cores, arc: residue },
        }
    }
}

/// Perception layers of one body. Not letters: the seventeen stay fixed.
///
/// ORDERED AND HASHABLE because it is a coordinate of [`coord::Piece`]
/// (§4.4 E8): a piece's address opens with its kind, and a seam's two ends
/// are stored in canonical order, so the kind has to compare. The order is
/// the declaration's own and is the same one `byte` writes.
///
/// [`coord::Piece`]: crate::network::nerve::coord::Piece
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum LayerKind {
    #[default]
    Whole,
    Outline,
    Core,
    Interior,
    Adjacent,
    Cut,
    Magnified,
    Reduced,
    Tiled,
    Bordered,
    /// The sample's own extent, not a body layer.
    Frame,
}

/// One layer of a body: its kind and its compartments on the three coordinates
/// of the base, `(row, col, frame, hue)` (51.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    pub kind: LayerKind,
    pub compartments: Vec<(i64, i64, i64, u8)>,
}

/// One layer of one SECTION: the planar reading a stack's layers are
/// assembled from (16.5), each section peeled in its own frame, as
/// painted sites.
#[cfg(test)]
struct SectionLayer {
    kind: LayerKind,
    painted: Vec<crate::transducer::spelled::PaintedSite>,
}

impl LayerKind {
    pub const FRAME: LayerKind = LayerKind::Frame;

    pub fn byte(self) -> u8 {
        match self {
            LayerKind::Whole => 0,
            LayerKind::Outline => 1,
            LayerKind::Core => 2,
            LayerKind::Interior => 3,
            LayerKind::Adjacent => 4,
            LayerKind::Cut => 5,
            LayerKind::Magnified => 6,
            LayerKind::Reduced => 7,
            LayerKind::Tiled => 8,
            LayerKind::Bordered => 9,
            LayerKind::Frame => 10,
        }
    }

    pub fn of(v: u8) -> Option<Self> {
        Some(match v {
            0 => LayerKind::Whole,
            1 => LayerKind::Outline,
            2 => LayerKind::Core,
            3 => LayerKind::Interior,
            4 => LayerKind::Adjacent,
            5 => LayerKind::Cut,
            6 => LayerKind::Magnified,
            7 => LayerKind::Reduced,
            8 => LayerKind::Tiled,
            9 => LayerKind::Bordered,
            10 => LayerKind::Frame,
            _ => return None,
        })
    }
}

/// Residue of the outline peel: the content still riding inward.
#[cfg(test)]
struct OutlineStep;
#[cfg(test)]
impl LayerStep for OutlineStep {
    type Residue = Vec<crate::transducer::spelled::PaintedSite>;
    type Core = SectionLayer;
    fn layer(&self, residue: &Vec<crate::transducer::spelled::PaintedSite>) -> Option<(SectionLayer, Vec<crate::transducer::spelled::PaintedSite>)> {
        if residue.is_empty() {
            return None;
        }
        let outline = crate::transducer::blade::outline(residue);
        let core = crate::transducer::blade::core(residue);
        if outline.is_empty() {
            return None;
        }
        Some((SectionLayer { kind: LayerKind::Outline, painted: outline }, core))
    }
}

#[cfg(test)]
struct CountGov(Option<usize>);
#[cfg(test)]
impl ShrinkGovernor for CountGov {
    type Residue = Vec<crate::transducer::spelled::PaintedSite>;
    fn read(&mut self, residue: &Vec<crate::transducer::spelled::PaintedSite>) -> LayerMotion {
        let n = residue.len();
        let motion = match (n == 0, self.0) {
            (true, _) => LayerMotion::Closed,
            (false, Some(before)) if n < before => LayerMotion::Shrinking,
            (false, None) => LayerMotion::Shrinking,
            _ => LayerMotion::Stalled,
        };
        self.0 = Some(n);
        motion
    }
}

#[cfg(test)]
fn scale_factor(num: usize, den: usize) -> Option<(LayerKind, i64)> {
    if den == 0 || num == den {
        return None;
    }
    if num > den && num % den == 0 {
        let k = (num / den) as i64;
        (k > 1).then_some((LayerKind::Magnified, k))
    } else if den > num && den % num == 0 {
        let k = (den / num) as i64;
        (k > 1).then_some((LayerKind::Reduced, k))
    } else {
        None
    }
}

/// Peel a body: whole, then outline with the core as residue, then the
/// interior the outline encloses, then the adjacent ring. Each layer is
/// taken from the residue the last layer left; the governor reads shrink.
/// Scale layers only when a frame ratio is supplied and is not one.
/// Cut is not produced here: it is taken only with sites a face names.
#[cfg(test)]
pub fn peel_body(compartments: &[(i64, i64, i64, u8)]) -> Vec<Layer> {
    peel_body_scaled(compartments, None)
}

/// THE LAYERS OF A STACK are the layers of its sections (16.5): each frame
/// of the body is peeled in its own plane, and the layers of one kind are
/// one layer of the body, lifted back to the layers' frames. `scale` is
/// ((h_out, h_in), (w_out, w_in)).
#[cfg(test)]
pub fn peel_body_scaled(
    compartments: &[(i64, i64, i64, u8)],
    scale: Option<((usize, usize), (usize, usize))>,
) -> Vec<Layer> {
    if compartments.is_empty() {
        return Vec::new();
    }
    let mut by_frame: Vec<(i64, i64, i64, u8)> = compartments.to_vec();
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: frame first, then the site's own row and column"
    )]
    by_frame.sort_unstable_by_key(|&(r, c, k, _)| (k, r, c));
    let mut out: Vec<Layer> = Vec::new();
    // The walk is the body's own compartments, one frame at a time.
    let mut at = 0usize;
    while at < by_frame.len() {
        let k = by_frame[at].2;
        let end = by_frame[at..]
            .iter()
            .position(|compartment| compartment.2 != k)
            .map_or(by_frame.len(), |n| at + n);
        let section: Vec<crate::transducer::spelled::PaintedSite> =
            by_frame[at..end].iter().map(|&(r, c, _, h)| crate::transducer::spelled::PaintedSite::on_plane([r, c], h)).collect();
        for layer in peel_section_scaled(&section, scale) {
            let lifted = layer.painted.iter().map(|p| {
                let [r, c] = p.site.coordinates();
                (r, c, k, p.symbol)
            });
            match out.iter_mut().find(|s| s.kind == layer.kind) {
                Some(s) => s.compartments.extend(lifted),
                None => out.push(Layer { kind: layer.kind, compartments: lifted.collect() }),
            }
        }
        at = end;
    }
    for s in &mut out {
        s.compartments.sort_unstable();
    }
    out
}

#[cfg(test)]
fn peel_section_scaled(
    painted: &[crate::transducer::spelled::PaintedSite],
    scale: Option<((usize, usize), (usize, usize))>,
) -> Vec<SectionLayer> {
    if painted.is_empty() {
        return Vec::new();
    }
    let mut out = vec![SectionLayer { kind: LayerKind::Whole, painted: painted.to_vec() }];
    let mut residue = painted.to_vec();
    let mut gov = CountGov(Some(residue.len()));
    if let Some((outline, core)) = OutlineStep.layer(&residue) {
        match gov.read(&core) {
            LayerMotion::Closed => {
                if outline.painted != painted {
                    out.push(outline);
                }
                residue = core;
            }
            LayerMotion::Shrinking => {
                if outline.painted != painted {
                    out.push(outline);
                }
                residue = core;
                if !residue.is_empty() {
                    out.push(SectionLayer { kind: LayerKind::Core, painted: residue.clone() });
                }
            }
            LayerMotion::Stalled => {}
        }
    }
    let interior = crate::transducer::blade::enclosed_interior(&residue);
    if !interior.is_empty() {
        match gov.read(&interior) {
            LayerMotion::Shrinking | LayerMotion::Closed => {
                out.push(SectionLayer { kind: LayerKind::Interior, painted: interior });
            }
            LayerMotion::Stalled => {}
        }
    }
    let ring = crate::transducer::blade::adjacent_ring(&residue);
    if !ring.is_empty() {
        match gov.read(&ring) {
            LayerMotion::Shrinking | LayerMotion::Closed | LayerMotion::Stalled => {
                out.push(SectionLayer { kind: LayerKind::Adjacent, painted: ring });
            }
        }
    }
    if let Some(((hn, hd), (wn, wd))) = scale {
        if let Some((kind, k)) = scale_factor(hn, hd).or_else(|| scale_factor(wn, wd)) {
            // A body whose blocks do not condense to one hue REDUCES TO
            // NOTHING, and a kind that is neither scaling scales nothing:
            // both are the absence of a scaled body, and no layer stands
            // for either.
            let scaled = match kind {
                LayerKind::Magnified => Some(crate::transducer::blade::magnified(painted, k)),
                LayerKind::Reduced => crate::transducer::blade::reduced(painted, k),
                _ => None,
            };
            if let Some(painted) = scaled.filter(|s| !s.is_empty()) {
                out.push(SectionLayer { kind, painted });
            }
        }
    }
    out
}
