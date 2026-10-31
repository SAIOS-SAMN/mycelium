use crate::transducer::peel::*;

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

/// The peel closes or stands, and never forces a shell.
#[test]
fn peel_nest_closes_or_stands_never_forces() {
    let closed = peel_nest(vec![1, 2, 3], &TakeHead, &mut Shorter(None));
    assert_eq!(closed, PeelNest::Closed { cores: vec![1, 2, 3] });
    let empty = peel_nest(Vec::new(), &TakeHead, &mut Shorter(None));
    assert_eq!(empty, PeelNest::Closed { cores: vec![] });
    let standing = peel_nest(vec![1, 0xff, 3], &TakeUnlessPoison, &mut Shorter(None));
    assert_eq!(standing, PeelNest::Standing { cores: vec![1], arc: vec![0xff, 3] });
}

/// A body peels into its whole, its outline and its core.
#[test]
fn peel_body_yields_outline_and_core() {
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
fn peel_body_peels_each_section_in_own_frame() {
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
