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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

/// One shell of a body: its kind and its cells (row, col, hue).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shell {
    pub kind: ShellKind,
    pub cells: Vec<(i64, i64, u8)>,
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
    type Core = Shell;
    fn shell(&self, residual: &Vec<(i64, i64, u8)>) -> Option<(Shell, Vec<(i64, i64, u8)>)> {
        if residual.is_empty() {
            return None;
        }
        let outline = crate::transducer::blade::outline(residual);
        let core = crate::transducer::blade::core(residual);
        if outline.is_empty() {
            return None;
        }
        Some((Shell { kind: ShellKind::Outline, cells: outline }, core))
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
pub fn peel_body(cells: &[(i64, i64, u8)]) -> Vec<Shell> {
    peel_body_scaled(cells, None)
}

/// `scale` is ((h_out, h_in), (w_out, w_in)).
pub fn peel_body_scaled(
    cells: &[(i64, i64, u8)],
    scale: Option<((usize, usize), (usize, usize))>,
) -> Vec<Shell> {
    if cells.is_empty() {
        return Vec::new();
    }
    let mut out = vec![Shell { kind: ShellKind::Whole, cells: cells.to_vec() }];
    let mut residual = cells.to_vec();
    let mut gov = CountGov(Some(residual.len()));
    if let Some((outline, core)) = OutlineStep.shell(&residual) {
        match gov.read(&core) {
            ShellMotion::Closed => {
                if outline.cells != cells {
                    out.push(outline);
                }
                residual = core;
            }
            ShellMotion::Shrinking => {
                if outline.cells != cells {
                    out.push(outline);
                }
                residual = core;
                if !residual.is_empty() {
                    out.push(Shell { kind: ShellKind::Core, cells: residual.clone() });
                }
            }
            ShellMotion::Stalled => {}
        }
    }
    let interior = crate::transducer::blade::enclosed_interior(&residual);
    if !interior.is_empty() {
        match gov.read(&interior) {
            ShellMotion::Shrinking | ShellMotion::Closed => {
                out.push(Shell { kind: ShellKind::Interior, cells: interior });
            }
            ShellMotion::Stalled => {}
        }
    }
    let ring = crate::transducer::blade::adjacent_ring(&residual);
    if !ring.is_empty() {
        match gov.read(&ring) {
            ShellMotion::Shrinking | ShellMotion::Closed | ShellMotion::Stalled => {
                out.push(Shell { kind: ShellKind::Adjacent, cells: ring });
            }
        }
    }
    if let Some(((hn, hd), (wn, wd))) = scale {
        if let Some((kind, k)) = scale_factor(hn, hd).or_else(|| scale_factor(wn, wd)) {
            let scaled = match kind {
                ShellKind::Magnified => crate::transducer::blade::magnified(cells, k),
                ShellKind::Reduced => crate::transducer::blade::reduced(cells, k).unwrap_or_default(),
                _ => Vec::new(),
            };
            if !scaled.is_empty() {
                out.push(Shell { kind, cells: scaled });
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
            let head = *r.first()?;
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
            let head = *r.first()?;
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
        let mut cells = Vec::new();
        for r in 0..3 {
            for c in 0..3 {
                cells.push((r, c, 1u8));
            }
        }
        let shells = peel_body(&cells);
        assert!(shells.iter().any(|s| s.kind == ShellKind::Whole && s.cells.len() == 9));
        assert!(shells.iter().any(|s| s.kind == ShellKind::Outline && s.cells.len() == 8));
        assert!(shells.iter().any(|s| s.kind == ShellKind::Core && s.cells.len() == 1));
    }
}
