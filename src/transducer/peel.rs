// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 20 — THE PEEL: a nested manifold read outer shell to inner with the
//! residual riding inward, continuing while the governor reads `Shrinking`
//! and routing the residual out whole — the standing arc — when it does not.

/// What the governor reads off a residual.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellMotion {
    /// Nothing remains: the nest closed.
    Closed,
    /// Strictly less remains than before this shell.
    Shrinking,
    /// The shell did not shrink the residual: the arc rides out.
    Stalled,
}

/// One shell of a nest: from a residual, the core it takes and what it
/// leaves. `None` when the shell cannot take anything whole.
pub trait ShellStep {
    type Residual;
    type Core;
    fn shell(&self, residual: &Self::Residual) -> Option<(Self::Core, Self::Residual)>;
}

/// The governor reads the residual between shells; it carries what it
/// needs to read strict shrinking.
pub trait ShrinkGovernor {
    type Residual;
    fn read(&mut self, residual: &Self::Residual) -> ShellMotion;
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

/// THE PEEL LAW: iterate the step with the residual riding inward, outer
/// constraining inner by construction. Bounded because a strictly
/// shrinking finite residual reaches empty or stalls.
pub fn peel_nest<S, G>(target: S::Residual, step: &S, governor: &mut G) -> PeelNest<S::Residual, S::Core>
where
    S: ShellStep,
    G: ShrinkGovernor<Residual = S::Residual>,
{
    let mut cores: Vec<S::Core> = Vec::new();
    if let ShellMotion::Closed = governor.read(&target) {
        return PeelNest::Closed { cores };
    }
    let mut arc = target;
    loop {
        let (core, residual) = match step.shell(&arc) {
            Some(shell) => shell,
            None => return PeelNest::Standing { cores, arc },
        };
        cores.push(core);
        match governor.read(&residual) {
            ShellMotion::Closed => return PeelNest::Closed { cores },
            ShellMotion::Shrinking => arc = residual,
            ShellMotion::Stalled => return PeelNest::Standing { cores, arc: residual },
        }
    }
}

/// Perception shells of one body. Not letters: the seventeen stay fixed.
///
/// ORDERED AND HASHABLE because it is a coordinate of [`coord::Piece`]
/// (§4.4 E8): a piece's address opens with its kind, and a seam's two ends
/// are stored in canonical order, so the kind has to compare. The order is
/// the declaration's own and is the same one `as_u8` writes.
///
/// [`coord::Piece`]: crate::network::nerve::coord::Piece
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ShellKind {
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
    /// The grid's own extent, not a body shell.
    Frame,
}

/// One shell of a body: its kind and its compartments on the three coordinates
/// of the base, `(row, col, frame, hue)` (51.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shell {
    pub kind: ShellKind,
    pub compartments: Vec<(i64, i64, i64, u8)>,
}

/// One shell of one SECTION: the planar reading a stack's shells are
/// assembled from (16.5), each section peeled in its own frame, as
/// painted sites.
struct SectionShell {
    kind: ShellKind,
    painted: Vec<(i64, i64, u8)>,
}

impl ShellKind {
    pub const FRAME: ShellKind = ShellKind::Frame;

    pub fn as_u8(self) -> u8 {
        match self {
            ShellKind::Whole => 0,
            ShellKind::Outline => 1,
            ShellKind::Core => 2,
            ShellKind::Interior => 3,
            ShellKind::Adjacent => 4,
            ShellKind::Cut => 5,
            ShellKind::Magnified => 6,
            ShellKind::Reduced => 7,
            ShellKind::Tiled => 8,
            ShellKind::Bordered => 9,
            ShellKind::Frame => 10,
        }
    }

    pub fn from_u8(v: u8) -> Option<Self> {
        Some(match v {
            0 => ShellKind::Whole,
            1 => ShellKind::Outline,
            2 => ShellKind::Core,
            3 => ShellKind::Interior,
            4 => ShellKind::Adjacent,
            5 => ShellKind::Cut,
            6 => ShellKind::Magnified,
            7 => ShellKind::Reduced,
            8 => ShellKind::Tiled,
            9 => ShellKind::Bordered,
            10 => ShellKind::Frame,
            _ => return None,
        })
    }
}

/// Residual of the outline peel: the content still riding inward.
struct OutlineStep;
impl ShellStep for OutlineStep {
    type Residual = Vec<(i64, i64, u8)>;
    type Core = SectionShell;
    fn shell(&self, residual: &Vec<(i64, i64, u8)>) -> Option<(SectionShell, Vec<(i64, i64, u8)>)> {
        if residual.is_empty() {
            return None;
        }
        let outline = crate::transducer::blade::outline(residual);
        let core = crate::transducer::blade::core(residual);
        if outline.is_empty() {
            return None;
        }
        Some((SectionShell { kind: ShellKind::Outline, painted: outline }, core))
    }
}

struct CountGov(Option<usize>);
impl ShrinkGovernor for CountGov {
    type Residual = Vec<(i64, i64, u8)>;
    fn read(&mut self, residual: &Vec<(i64, i64, u8)>) -> ShellMotion {
        let n = residual.len();
        let motion = match (n == 0, self.0) {
            (true, _) => ShellMotion::Closed,
            (false, Some(before)) if n < before => ShellMotion::Shrinking,
            (false, None) => ShellMotion::Shrinking,
            _ => ShellMotion::Stalled,
        };
        self.0 = Some(n);
        motion
    }
}

fn scale_factor(num: usize, den: usize) -> Option<(ShellKind, i64)> {
    if den == 0 || num == den {
        return None;
    }
    if num > den && num % den == 0 {
        let k = (num / den) as i64;
        (k > 1).then_some((ShellKind::Magnified, k))
    } else if den > num && den % num == 0 {
        let k = (den / num) as i64;
        (k > 1).then_some((ShellKind::Reduced, k))
    } else {
        None
    }
}

/// Peel a body: whole, then outline with the core as residual, then the
/// interior the outline encloses, then the adjacent ring. Each shell is
/// taken from the residual the last shell left; the governor reads shrink.
/// Scale shells only when a frame ratio is supplied and is not one.
/// Cut is not produced here: it is taken only with sites a face names.
pub fn peel_body(compartments: &[(i64, i64, i64, u8)]) -> Vec<Shell> {
    peel_body_scaled(compartments, None)
}

/// THE SHELLS OF A STACK are the shells of its sections (16.5): each frame
/// of the body is peeled in its own plane, and the shells of one kind are
/// one shell of the body, lifted back to their frames. `scale` is
/// ((h_out, h_in), (w_out, w_in)).
pub fn peel_body_scaled(
    compartments: &[(i64, i64, i64, u8)],
    scale: Option<((usize, usize), (usize, usize))>,
) -> Vec<Shell> {
    if compartments.is_empty() {
        return Vec::new();
    }
    let mut by_frame: Vec<(i64, i64, i64, u8)> = compartments.to_vec();
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: frame first, then the site's own row and column"
    )]
    by_frame.sort_unstable_by_key(|&(r, c, k, _)| (k, r, c));
    let mut out: Vec<Shell> = Vec::new();
    // The walk is the body's own compartments, one frame at a time.
    let mut at = 0usize;
    while at < by_frame.len() {
        let k = by_frame[at].2;
        let end = by_frame[at..]
            .iter()
            .position(|compartment| compartment.2 != k)
            .map_or(by_frame.len(), |n| at + n);
        let section: Vec<(i64, i64, u8)> = by_frame[at..end].iter().map(|&(r, c, _, h)| (r, c, h)).collect();
        for shell in peel_section_scaled(&section, scale) {
            let lifted = shell.painted.iter().map(|&(r, c, h)| (r, c, k, h));
            match out.iter_mut().find(|s| s.kind == shell.kind) {
                Some(s) => s.compartments.extend(lifted),
                None => out.push(Shell { kind: shell.kind, compartments: lifted.collect() }),
            }
        }
        at = end;
    }
    for s in &mut out {
        s.compartments.sort_unstable();
    }
    out
}

fn peel_section_scaled(
    painted: &[(i64, i64, u8)],
    scale: Option<((usize, usize), (usize, usize))>,
) -> Vec<SectionShell> {
    if painted.is_empty() {
        return Vec::new();
    }
    let mut out = vec![SectionShell { kind: ShellKind::Whole, painted: painted.to_vec() }];
    let mut residual = painted.to_vec();
    let mut gov = CountGov(Some(residual.len()));
    if let Some((outline, core)) = OutlineStep.shell(&residual) {
        match gov.read(&core) {
            ShellMotion::Closed => {
                if outline.painted != painted {
                    out.push(outline);
                }
                residual = core;
            }
            ShellMotion::Shrinking => {
                if outline.painted != painted {
                    out.push(outline);
                }
                residual = core;
                if !residual.is_empty() {
                    out.push(SectionShell { kind: ShellKind::Core, painted: residual.clone() });
                }
            }
            ShellMotion::Stalled => {}
        }
    }
    let interior = crate::transducer::blade::enclosed_interior(&residual);
    if !interior.is_empty() {
        match gov.read(&interior) {
            ShellMotion::Shrinking | ShellMotion::Closed => {
                out.push(SectionShell { kind: ShellKind::Interior, painted: interior });
            }
            ShellMotion::Stalled => {}
        }
    }
    let ring = crate::transducer::blade::adjacent_ring(&residual);
    if !ring.is_empty() {
        match gov.read(&ring) {
            ShellMotion::Shrinking | ShellMotion::Closed | ShellMotion::Stalled => {
                out.push(SectionShell { kind: ShellKind::Adjacent, painted: ring });
            }
        }
    }
    if let Some(((hn, hd), (wn, wd))) = scale {
        if let Some((kind, k)) = scale_factor(hn, hd).or_else(|| scale_factor(wn, wd)) {
            // A body whose blocks do not condense to one hue REDUCES TO
            // NOTHING, and a kind that is neither scaling scales nothing:
            // both are the absence of a scaled body, and no shell stands
            // for either.
            let scaled = match kind {
                ShellKind::Magnified => Some(crate::transducer::blade::magnified(painted, k)),
                ShellKind::Reduced => crate::transducer::blade::reduced(painted, k),
                _ => None,
            };
            if let Some(painted) = scaled.filter(|s| !s.is_empty()) {
                out.push(SectionShell { kind, painted });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A residual of tokens; a shell takes the head. The governor reads
    /// emptiness and strict shortening by exact comparison.
    struct TakeHead;
    impl ShellStep for TakeHead {
        type Residual = Vec<u8>;
        type Core = u8;
        fn shell(&self, r: &Vec<u8>) -> Option<(u8, Vec<u8>)> {
            let [head, ..] = r.as_slice() else { return None };
            let head = *head;
            Some((head, r[1..].to_vec()))
        }
    }

    struct Shorter(Option<usize>);
    impl ShrinkGovernor for Shorter {
        type Residual = Vec<u8>;
        fn read(&mut self, r: &Vec<u8>) -> ShellMotion {
            let motion = match (r.is_empty(), self.0) {
                (true, _) => ShellMotion::Closed,
                (false, Some(before)) if r.len() < before => ShellMotion::Shrinking,
                (false, None) => ShellMotion::Shrinking,
                _ => ShellMotion::Stalled,
            };
            self.0 = Some(r.len());
            motion
        }
    }

    /// A shell that refuses a poisoned token: the arc rides out whole with
    /// the cores already taken.
    struct TakeUnlessPoison;
    impl ShellStep for TakeUnlessPoison {
        type Residual = Vec<u8>;
        type Core = u8;
        fn shell(&self, r: &Vec<u8>) -> Option<(u8, Vec<u8>)> {
            let [head, ..] = r.as_slice() else { return None };
            let head = *head;
            if head == 0xff {
                return None;
            }
            Some((head, r[1..].to_vec()))
        }
    }

    #[test]
    fn the_peel_closes_or_stands_never_forces() {
        let closed = peel_nest(vec![1, 2, 3], &TakeHead, &mut Shorter(None));
        assert_eq!(closed, PeelNest::Closed { cores: vec![1, 2, 3] });
        let empty = peel_nest(Vec::new(), &TakeHead, &mut Shorter(None));
        assert_eq!(empty, PeelNest::Closed { cores: vec![] });
        let standing = peel_nest(vec![1, 0xff, 3], &TakeUnlessPoison, &mut Shorter(None));
        assert_eq!(standing, PeelNest::Standing { cores: vec![1], arc: vec![0xff, 3] });
    }

    #[test]
    fn a_body_peels_into_outline_and_core() {
        let mut compartments = Vec::new();
        for r in 0..3 {
            for c in 0..3 {
                compartments.push((r, c, 0, 1u8));
            }
        }
        let shells = peel_body(&compartments);
        assert!(shells.iter().any(|s| s.kind == ShellKind::Whole && s.compartments.len() == 9));
        assert!(shells.iter().any(|s| s.kind == ShellKind::Outline && s.compartments.len() == 8));
        assert!(shells.iter().any(|s| s.kind == ShellKind::Core && s.compartments.len() == 1));
    }

    /// 16.5 — A STACK PEELS SECTION BY SECTION: two frames of one square
    /// are one outline shell of sixteen compartments and one core of two, each
    /// compartment in the frame it came from.
    #[test]
    fn a_stack_peels_each_section_in_its_own_frame() {
        let mut compartments = Vec::new();
        for k in 0..2 {
            for r in 0..3 {
                for c in 0..3 {
                    compartments.push((r, c, k, 1u8));
                }
            }
        }
        let shells = peel_body(&compartments);
        let outline = shells.iter().find(|s| s.kind == ShellKind::Outline).expect("an outline");
        assert_eq!(outline.compartments.len(), 16);
        assert!(outline.compartments.iter().any(|c| c.2 == 1), "the second frame's outline is in its frame");
        let core = shells.iter().find(|s| s.kind == ShellKind::Core).expect("a core");
        assert_eq!(core.compartments, vec![(1, 1, 0, 1), (1, 1, 1, 1)]);
    }
}
