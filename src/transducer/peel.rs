// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE PEEL — planes are peeled, never iterated. Re-expressed from the
//! ipsal-autic v0.4 peel law: a nested manifold is read outer shell to
//! inner shell with the residual RIDING INWARD, each shell's step receiving
//! only what the outer shell left, so the consumed core is structurally out
//! of reach (no self-resonance, by construction). The peel continues while
//! the governor reads `Shrinking`; the instant it reads anything else the
//! residual routes out whole — the standing arc — never discarded, never
//! forced through a shell that cannot take it.
//!
//! No magnitude decides anything here: the governor is a typed read
//! (`Closed`, `Shrinking`, `Stalled`) supplied by the manifold being
//! peeled. The alphabet's spelling, the frame of a connection and the
//! entity's axis opening are all nests of this one law.

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
}
