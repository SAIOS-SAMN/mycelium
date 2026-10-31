// A9, MU-DERIVATIONS D3.3 (ruled 2026-10-31) — π* binds the WHOLE common set:
// every term common to the training evidence, held as a superposition and
// judged by the court's reconstruction over the withheld pairs.

use crate::court::verdict::derive_pi;
use crate::transducer::anchor::Selection;

/// A 4×4 scene of four 2×2 blocks, hues `a, b` on the top row of blocks and
/// `c, d` below.
fn quilt(a: u8, b: u8, c: u8, d: u8) -> Vec<(i64, i64, u8)> {
    (0..4)
        .flat_map(|r| {
            (0..4).map(move |col| {
                let hue = match (r / 2, col / 2) {
                    (0, 0) => a,
                    (0, _) => b,
                    (_, 0) => c,
                    _ => d,
                };
                (r, col, hue)
            })
        })
        .collect()
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 22)
}

/// THE SUPERPOSITION IS JUDGED BY RECONSTRUCTION ON THE WITHHELD PAIR: of the
/// terms common to the training pairs, those whose evaluation on the
/// withheld scene carries its footprint all stand — the whole support among
/// them — and a term that does not (the training corner's own hue, absent
/// from the withheld corner) is let go; no term is chosen by its key.
#[test]
fn derive_pi_keeps_every_term_reconstructing_withheld() {
    let fp = vec![(0, 0)];
    let train = vec![(fp.clone(), quilt(1, 2, 3, 4)), (fp.clone(), quilt(1, 5, 6, 7))];
    let common = derive_pi(&train, &[], &mut stroke()).expect("read");
    assert!(common.contains(&Selection::Hue(1)) && common.contains(&Selection::Hue(255)), "{common:?}");
    for (fp, scene) in &train {
        let set = crate::transducer::anchor::derive_carrying_all(fp, scene, &mut stroke()).expect("read");
        assert!(common.iter().all(|t| set.contains(t.key().as_bytes())), "common to every training set");
    }
    let withheld = vec![(fp.clone(), quilt(2, 1, 3, 4))];
    let judged = derive_pi(&train, &withheld, &mut stroke()).expect("read");
    assert!(judged.contains(&Selection::Hue(255)), "the whole support reconstructs the withheld corner");
    assert!(!judged.contains(&Selection::Hue(1)), "the training corner's hue does not");
    assert!(judged.len() > 1, "a superposition, every standing term kept: {judged:?}");
    for t in &judged {
        assert!(common.contains(t), "judged within the common set");
        assert!(t.eval(&withheld[0].1).contains(&(0, 0)), "{t:?} reconstructs the withheld footprint");
    }
}

/// ONE EVIDENCE OR NONE BINDS NOTHING BEYOND ITS OWN SET, AND A SPENT TOKEN
/// REFUSES THE WHOLE DERIVATION, never a part of it.
#[test]
fn derive_pi_refuses_spent_token_whole() {
    let fp = vec![(0, 0)];
    let train = vec![(fp.clone(), quilt(1, 2, 3, 4)), (fp, quilt(1, 3, 2, 4))];
    assert_eq!(derive_pi(&[], &[], &mut stroke()), Ok(Vec::new()));
    let reply = derive_pi(&train, &[], &mut crate::physics::Physics::default().stroke_n(5));
    assert!(reply.is_err(), "{reply:?}");
}
