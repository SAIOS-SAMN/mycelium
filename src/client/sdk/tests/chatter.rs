use super::super::chatter::*;
use crate::transducer::cga::Mv;

fn key(b: u8) -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[b; 32])
}

/// 78.163 — THE LOOP said → heard, BY THE ALGEBRA: a hearer that echoed
/// the turn closes it still; one that said the turn twice leaves the said
/// turn itself as the holonomy — what its language did to the thought; a
/// swap heard as the swap closes the hue loop still, heard as nothing
/// leaves the swap; a merge has no inverse and the hue loop stays open; a
/// word the hearer could not read closes nothing; and both minds read one
/// loop, projectively.
#[test]
fn loop_of_carries_what_hearer_did_to_thought() {
    use crate::transducer::cga::{E1, E2};
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let turn = Mv::scalar(q(1)).add(&Mv::blade(E1 | E2, q(1))).widened(3);
    let swap = vec![(1, 2), (2, 1)];
    let said = Spoken { vertex: vec![1], motion: turn.clone(), gauge: swap.clone(), terms: vec![], vantage: vec![] };
    let heard = |derived: Option<Mv>, gauge: Vec<(u8, u8)>| HeardWord { vertex: vec![1], derived, gauge, cites: vec![], asks: None, held: vec![], vantage: vec![] };
    let exact = loop_of(&said, &heard(Some(turn.clone()), swap.clone()));
    assert!(still(&exact), "an echo carries nothing: {exact:?}");
    let twice = loop_of(&said, &heard(Some(turn.geo(&turn)), swap.clone()));
    assert!(!still(&twice));
    let h = twice.motion.clone().expect("a loop closes on a versor");
    assert!(h.projective_eq(&turn), "what did not cross is the said turn itself");
    let hueless = loop_of(&said, &heard(Some(turn.clone()), vec![]));
    assert!(!still(&hueless), "the swap heard as nothing leaves the swap");
    assert!(hueless.hue.as_ref().is_some_and(|m| crate::transducer::sentence::transport::gauge_eq(m, &swap)));
    let merge = Spoken { vertex: vec![1], motion: turn.clone(), gauge: vec![(3, 0)], terms: vec![], vantage: vec![] };
    assert_eq!(loop_of(&merge, &heard(Some(turn.clone()), vec![(3, 0)])).hue, None, "a merge has no inverse: G stays open");
    let unread = loop_of(&said, &heard(None, vec![]));
    assert_eq!((unread.motion, unread.hue), (None, None), "nothing derived closes nothing");
    assert!(same_loop(&exact, &loop_of(&said, &heard(Some(turn.scale(&q(3))), swap))), "one loop, read projectively by both minds");
}

/// A frame sent to a post is delivered to its carrier, and a question is
/// answered on its own connection.
#[test]
fn post_delivers_sent_frame_and_answers_question() {
    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let post = Post::listen(
        "127.0.0.1:0",
        move |f| {
            let _ = tx.send(f);
        },
        |q: &[u8]| (q == b"?").then(|| b"!".to_vec()),
    )
    .expect("listen");
    assert_eq!(ask(post.endpoint(), b"?").expect("answered"), b"!", "a question is answered on its own connection");
    let k = key(3);
    let c = Chatter { speaker: k.verifying_key().to_bytes(), point: None, said: Said::Say { said: vec![9; 100], words: vec![], evidence: None } };
    send(post.endpoint(), &c.sign(&k).expect("signed")).expect("sent");
    let heard = rx.recv().expect("delivered");
    assert_eq!(Chatter::verify(&heard), Some(c));
}

/// The rendezvous names every other mind's address and never the mind's
/// own.
#[test]
fn peers_names_every_other_mind() {
    let dir = std::env::temp_dir().join(format!("chatter-rv-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    publish(&dir, &[1; 32], "127.0.0.1:1").expect("published");
    publish(&dir, &[2; 32], "127.0.0.1:2").expect("published");
    assert_eq!(peers(&dir, &[1; 32]), vec!["127.0.0.1:2".to_string()]);
    let _ = std::fs::remove_dir_all(&dir);
}
