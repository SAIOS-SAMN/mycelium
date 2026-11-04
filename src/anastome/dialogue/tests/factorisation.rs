// MU-SENSING §3.9, §6.2; ROADMAP track 26 S4c — A CLAIM OF FACTORISATION ON
// THE WIRE: the act and its verdict read back to the exact values written,
// at rank 3 and width 2, by decoded values, never bytes (Concordance 1.4).

use crate::anastome::dialogue::*;
use crate::transducer::base::Site;
use crate::transducer::family::transduce::{Enclosure, Motion, Sense, SignedPermutation};
use crate::transducer::sentence::Link;

fn key() -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[7u8; 32])
}

fn motion() -> Motion {
    let permutation = SignedPermutation::new(vec![(2, Sense::Kept), (0, Sense::Reversed), (1, Sense::Kept)]).expect("a signed permutation");
    Motion { permutation, onto: Enclosure::new(vec![3, 2, 2]).expect("an enclosure") }
}

fn residue() -> Vec<Link> {
    vec![Link::new(Site::of([0, 1, 2]), vec![0, 0], vec![1, 9]), Link::new(Site::of([2, 1, 0]), vec![4, 4], vec![0, 0])]
}

/// A CLAIM OF FACTORISATION ROUND-TRIPS: the voice, the task, the train
/// pair, the motion (each phase's source and sense, and the periods it is
/// taken onto), the residue's links at rank 3 and width 2, and the stated
/// ground read back exactly, signed by the voice; a claim stating no ground
/// reads none. A tampered byte breaks the signature, never the reading; a
/// byte past the signature, an empty stated ground, or a writing that is no
/// signed permutation reads as no claim. Each verdict reads back whole.
#[test]
fn factorisation_claim_round_trips() {
    let cite = [5u8; 32];
    let bytes = encode_factorisation(&key(), &cite, 3, &motion(), &residue(), Some(&[0, 0])).expect("the claim writes");
    let act = decode_factorisation(&bytes).expect("the claim reads");
    assert_eq!(
        act,
        FactorisationAct {
            voice: key().verifying_key().to_bytes(),
            task: cite,
            pair: 3,
            motion: motion(),
            residue: residue(),
            ground: Some(vec![0, 0]),
            signed: true
        }
    );
    // A claim kind under the solve (78.126): never read as a solve of
    // outputs, and a solve of outputs never read as a claim.
    assert_eq!((bytes[0], solve_kind(&bytes)), (ACT_SOLVE, Some(SOLVE_KIND_FACTORISATION)));
    assert!(decode_solve(&bytes).is_none(), "a claim of factorisation is no solve of outputs");
    let outputs = spoken_outputs(&[vec![vec![1]]]).expect("a sample");
    let solve = encode_solve(&key(), &cite, &outputs).expect("a solve writes");
    assert_eq!(solve_kind(&solve), Some(SOLVE_KIND_OUTPUTS));
    assert_eq!(decode_factorisation(&solve), None, "a solve of outputs is no claim of factorisation");
    let bare = decode_factorisation(&encode_factorisation(&key(), &cite, 0, &motion(), &[], None).expect("writes")).expect("reads");
    assert_eq!((bare.ground, bare.residue, bare.signed), (None, Vec::new(), true), "no ground stated reads none");

    let mut tampered = bytes.clone();
    tampered[40] ^= 1;
    assert_eq!(decode_factorisation(&tampered).map(|a| a.signed), Some(false), "a tampered claim reads unsigned");
    let mut longer = bytes.clone();
    longer.push(0);
    assert_eq!(decode_factorisation(&longer), None, "a byte past the signature is no claim");
    assert_eq!(decode_factorisation(&encode_factorisation(&key(), &cite, 0, &motion(), &[], Some(&[])).expect("writes")), None, "an empty ground is no symbol");
    // A permutation written with one source twice moves the seam set: no claim.
    let mut oblique = encode_factorisation(&key(), &cite, 0, &motion(), &[], None).expect("writes");
    let at = 1 + 1 + 32 + 32 + 1 + 2; // act, kind, voice, task, pair, rank: the first source
    oblique[at + 3] = oblique[at];
    assert_eq!(decode_factorisation(&oblique), None, "no signed permutation, no claim");

    for v in [
        FactorVerdict::Admitted,
        FactorVerdict::Residue { unclaimed: residue(), uncarried: vec![residue()[0].clone()] },
        FactorVerdict::Refused { reason: b"no such train pair".to_vec() },
    ] {
        assert_eq!(decode_factor_verdict(&encode_factor_verdict(&v)), Some(v));
    }
    assert_eq!(decode_factor_verdict(&[9]), None, "an unknown verdict reads as none");
    let mut admitted = encode_factor_verdict(&FactorVerdict::Admitted);
    admitted.push(0);
    assert_eq!(decode_factor_verdict(&admitted), None, "a verdict is read whole");
}
