// MU-SENSING §3.9 (ROADMAP track 26 S4b) — transduction as factorisation:
// every pair factors exactly as motion ∘ residue, several factorisations are
// held as a vortex, and a court derives the residue itself and admits on a
// match.

use super::{dilated, extents, sample_by, sample_n};
use crate::court::verdict::factorisation::{admit_factorisation, FactorClaim, FactorRefusal};
use crate::transducer::base::Site;
use crate::transducer::family::transduce::*;
use crate::transducer::sample::Sample;
use crate::transducer::sentence::Link;

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 12)
}

/// The frame with one site's symbol replaced.
fn changed_at(s: &Sample, site: &[i64], symbol: u8) -> Sample {
    sample_by(&extents(s), |t| if t == site { symbol } else { super::at(s, t) })
}

/// The motion of `g` onto `y`'s enclosure.
fn motion(g: SignedPermutation, y: &Sample) -> Motion {
    Motion { permutation: g, onto: Enclosure::of(y) }
}

/// DEFORMATION = MOTION ∘ RESIDUE, EXACTLY: at ranks 1, 2 and 3, every
/// factorisation a pair holds recomposes the output from the motion's
/// speech and its residue, and every residue link stands where the motion's
/// speech and the output differ, with the symbol each holds. The identity
/// on one enclosure leaves the coboundary; a dilation is carried whole by
/// its re-period with no residue.
#[test]
fn deformation_factors_as_motion_then_residue() {
    let frames = [sample_n(&[5], "01102"), sample_n(&[2, 3], "011002"), sample_n(&[2, 2, 3], "011000000002")];
    for x in &frames {
        let n = x.rank();
        let last: Vec<i64> = extents(x).iter().map(|&e| e as i64 - 1).collect();
        let y = changed_at(x, &last, 3);
        let factored = factorisations(x, &y, &mut budget()).expect("one rank");
        assert!(!factored.held().is_empty(), "rank {n}: the identity lands");
        for f in factored.held() {
            assert_eq!(f.recomposed(x).as_ref(), Ok(&y), "rank {n}: motion then residue is the output");
            let spoken = f.motion.spoken(x).expect("held, so it lands");
            for l in &f.residue {
                let here = spoken.symbol_at(&l.site).expect("on the speech");
                assert_eq!(here.bytes(), l.before.as_slice(), "the residue starts from the motion's speech");
                assert_eq!(y.symbol_at(&l.site).expect("on the output").bytes(), l.after.as_slice());
                assert_ne!(l.before, l.after, "a residue link is a change");
            }
        }
        let identity = Factorisation::of(x, &y, &motion(SignedPermutation::identity(n), &y), &mut budget()).expect("lands");
        assert_eq!(identity.residue, vec![Link::new(Site::new(last.clone()).expect("a site"), vec![2], vec![3])], "rank {n}");
        let doubled = dilated(x, &vec![2; n]);
        let carried = Factorisation::of(x, &doubled, &motion(SignedPermutation::identity(n), &doubled), &mut budget()).expect("lands");
        assert!(carried.residue.is_empty(), "rank {n}: the re-period carries the dilation whole");
        assert!(carried.carried.iter().all(Result::is_ok), "rank {n}: every body carried");
    }
}

/// SEVERAL FACTORISATIONS THAT HOLD ARE HELD AS A VORTEX: a square frame
/// with one body in a corner, against itself with the far corner painted,
/// holds a factorisation for each of the eight signed permutations of its
/// two phases, all of them, each recomposing the output, in the group's
/// generated order; they leave residues of different sizes and none is
/// dropped for another. No scalar picks one.
#[test]
fn several_factorisations_held_as_a_vortex() {
    let x = sample_n(&[3, 3], "100000000");
    let y = sample_n(&[3, 3], "100000002");
    let factored = factorisations(&x, &y, &mut budget()).expect("one rank");
    let Factored::Vortex(held) = &factored else { panic!("several hold: {factored:?}") };
    assert_eq!(held.len(), SignedPermutation::all(2).count(), "every motion lands on a square frame of one body");
    let order: Vec<SignedPermutation> = held.iter().map(|f| f.motion.permutation.clone()).collect();
    assert_eq!(order, SignedPermutation::all(2).collect::<Vec<_>>(), "held in the generated order, none picked");
    for f in held {
        assert_eq!(f.recomposed(&x).as_ref(), Ok(&y));
    }
    // The identity keeps the body where the output keeps it; the half
    // turn moves it onto the corner the output paints: residues of two
    // sizes, both held.
    let sizes: Vec<usize> = held.iter().map(|f| f.residue.len()).collect();
    let [identity, ..] = held.as_slice() else { panic!("the vortex holds the identity") };
    assert_eq!(identity.motion.permutation, SignedPermutation::identity(2));
    assert_eq!(identity.residue.len(), 1, "the identity leaves the painted corner alone: {sizes:?}");
    assert!(sizes.contains(&2), "a motion moving the body onto the painted corner leaves two: {sizes:?}");
    // At rank 3 a frame of unequal extents holds the motions that land and
    // names the rest, every one accounted for.
    let x3 = sample_n(&[1, 2, 3], "010010");
    let y3 = sample_n(&[1, 2, 3], "010210");
    let factored = factorisations(&x3, &y3, &mut budget()).expect("one rank");
    match &factored {
        Factored::Vortex(held) => assert!(held.len() > 1),
        other => panic!("several land at rank 3: {other:?}"),
    }
}

/// THE COURT DERIVES THE RESIDUE AND ADMITS ON A MATCH: a claim stating a
/// motion and the residue it leaves is admitted at rank 3, the court's
/// residue derived from the samples and the stated motion alone. A claim
/// that leaves a link out, or states one the motion does not leave, is
/// refused with the residue of the residues; a motion stated onto another
/// enclosure is refused by the transduction's own name.
#[test]
fn court_derives_the_residue_and_admits_on_match() {
    let x = sample_n(&[2, 2, 3], "011000000002");
    let y = sample_n(&[2, 2, 3], "011000000703");
    let stated = motion(SignedPermutation::identity(3), &y);
    let truth = Factorisation::of(&x, &y, &stated, &mut budget()).expect("lands").residue;
    assert_eq!(truth.len(), 2, "two sites differ");
    let claim = FactorClaim { motion: stated.clone(), residue: truth.clone() };
    let admitted = admit_factorisation(&x, &y, &claim).expect("the residue matches");
    assert_eq!(admitted.reconstructed.residue, truth);
    // One link left out: the court names it.
    let short = FactorClaim { motion: stated.clone(), residue: truth[..1].to_vec() };
    assert_eq!(
        admit_factorisation(&x, &y, &short),
        Err(FactorRefusal::Residue { unclaimed: truth[1..].to_vec(), uncarried: Vec::new() })
    );
    // A link the motion does not leave: the court names it.
    let stray = Link::new(Site::of([0, 0, 0]), vec![0], vec![5]);
    let mut long = truth.clone();
    long.push(stray.clone());
    assert_eq!(
        admit_factorisation(&x, &y, &FactorClaim { motion: stated.clone(), residue: long }),
        Err(FactorRefusal::Residue { unclaimed: Vec::new(), uncarried: vec![stray] })
    );
    // A motion onto an enclosure that is not the output's.
    let elsewhere = Motion { permutation: SignedPermutation::identity(3), onto: Enclosure::new(vec![2, 2, 4]).expect("periods") };
    assert_eq!(
        admit_factorisation(&x, &y, &FactorClaim { motion: elsewhere, residue: truth }),
        Err(FactorRefusal::Motion(TransductionRefusal::OntoAnother))
    );
}
