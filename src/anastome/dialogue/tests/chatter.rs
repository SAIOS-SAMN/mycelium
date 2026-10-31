// 78.161 — the chatter frame's wire: every sub-kind reads back whole once
// signed, a changed byte does not verify, and only the speaker's key signs.

use crate::anastome::dialogue::chatter::*;
use crate::chronometry::tuniversal::TPoint;
use crate::transducer::cga::Mv;

fn key(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

/// Every sub-kind reads back whole once signed, a forged frame does not
/// verify, and a key other than the speaker's signs nothing.
#[test]
fn verify_round_trips_and_refuses_forged_frame() {
    let k = key(7);
    let speaker = k.verifying_key().to_bytes();
    let point = TPoint { h: 9, lanes: [1, 2, 3, 4], k: 5, ..TPoint::default() };
    let one = Mv::scalar(num_rational::BigRational::from_integer(1.into()));
    for said in [
        Said::Say {
            said: vec![1, 2, 3],
            words: vec![Spoken {
                vertex: vec![4; 40],
                motion: one.clone(),
                gauge: vec![(1, 2), (2, 1)],
                terms: vec![(one.clone(), one.clone()), (one.clone(), one.clone())],
                vantage: vec![one.clone()],
            }],
            evidence: Some(Evidence { scene: vec![3; 32], word: vec![8; 50] }),
        },
        Said::Say { said: vec![1], words: vec![], evidence: None },
        Said::Heard {
            said: vec![6; 10],
            words: vec![
                HeardWord {
                    vertex: vec![4; 40],
                    derived: Some(one.clone()),
                    gauge: vec![(3, 4)],
                    cites: vec![(vec![7; 33], vec![8; 34], 1, -3)],
                    asks: None,
                    held: vec![one.clone()],
                    vantage: vec![one.clone(), one.clone()],
                },
                HeardWord { vertex: vec![5; 3], derived: None, gauge: vec![], cites: vec![], asks: Some(one.clone()), held: vec![], vantage: vec![] },
            ],
        },
        Said::Ack {
            said: vec![2; 7],
            words: vec![
                AckWord { vertex: vec![4; 40], motion: Some(one.clone()), hue: Some(vec![]) },
                AckWord { vertex: vec![5; 3], motion: None, hue: None },
            ],
        },
        Said::Hold {
            said: vec![2; 7],
            words: vec![AckWord { vertex: vec![4; 40], motion: Some(one.clone()), hue: None }],
            deviation: [0, 1, 0, 3].map(|n: i64| num_rational::BigRational::new(n.into(), 7.into())),
        },
    ] {
        let c = Chatter { speaker, point: Some(point), said };
        let f = c.sign(&k).expect("signed");
        assert_eq!(Chatter::verify(&f), Some(c.clone()));
        let mut forged = f.clone();
        let mid = forged.len() / 2;
        forged[mid] ^= 1;
        assert_eq!(Chatter::verify(&forged), None, "a changed byte does not verify");
    }
    // Signed by another key than the speaker it names: refused at signing.
    let c = Chatter { speaker, point: None, said: Said::Say { said: vec![], words: vec![], evidence: None } };
    assert_eq!(c.sign(&key(8)), None);
}
