// MU-CLIENT §3.1, §7.1 (#136, ruled 2026-11-06) — THE HYPHA'S ACTS REACH
// THE MIND AS FLUX ON ITS LANES: read off the hypha's line, chained to its
// signed head, each act's frame taken in on the hypha's well, the kind
// carrying nothing beyond its coordinate.

use super::signed_receipts::{digest, key, position};
use crate::anastome::dialogue;
use crate::chronometry::{ActKind, KStepReceipt, ReceiptChain};
use crate::client::sdk::hypha_acts::{chained, ActsRead, HyphaAct};
use crate::client::sdk::*;
use std::io::Write;
use std::sync::{Arc, Mutex};

/// A HYPHA THAT ANSWERS `ACT_RECEIPTS` with the page the test sets.
fn hypha_serving(page: Arc<Mutex<Vec<KStepReceipt>>>) -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("the hypha listens");
    let addr = listener.local_addr().expect("an address").to_string();
    std::thread::spawn(move || {
        for conn in listener.incoming() {
            let Ok(mut s) = conn else { continue };
            while let Some(body) = super::fixtures::read_act(&mut s) {
                let reply = match dialogue::decode_receipts_request(&body) {
                    Some(_) => dialogue::encode_receipts_reply(&page.lock().expect("the page").clone(), 1 << 20),
                    None => Vec::new(),
                };
                let _ = s.write_all(&super::fixtures::reply_bytes(&reply));
            }
        }
    });
    addr
}

/// A line of `acts`, each sealed with its frame, after an opening `List`.
fn line_of(signer: [u8; 32], acts: &[(ActKind, [u64; 4])]) -> Vec<KStepReceipt> {
    let mut c = ReceiptChain::with_signer(signer);
    let mut out = vec![c.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone()];
    for (kind, frame) in acts {
        out.push(c.append(*kind, *frame, Vec::new(), [0; 32], Vec::new()).expect("appends").clone());
    }
    out
}

/// MU-CLIENT §3.1 (#136) — A HYPHA ACT REACHES THE MIND AS FLUX ON ITS
/// LANES. Attached at the hypha's signed head, the mind reads the line up to
/// the next signed head: the hypha's own acts on it (a `Return` and a
/// `RankStep`, not the `List` between them) land on that hypha's well as
/// their frames, lane by lane, and stand as the court's deficit does, the
/// pressure the mind's beat is taken under. A page that does not end on the
/// signed head is refused whole, the read unmoved.
#[test]
fn hypha_act_reaches_the_mind_as_flux_on_its_lanes() {
    let k = key("acting hypha");
    let vk = k.verifying_bytes();
    let line = line_of(vk, &[(ActKind::Return, [1, 0, 0, 0]), (ActKind::List, [5, 5, 5, 5]), (ActKind::RankStep, [0, 2, 0, 3])]);
    let page = Arc::new(Mutex::new(Vec::new()));
    let hypha = hypha_serving(Arc::clone(&page));
    let mut mind = Mind::open(&hypha, Voice::named("attached"));
    let session = Session::open(&hypha, Voice::named("attached"));
    session.absorb_digest(&digest(&k, &hypha, &position(&k, &line[0], b"root")));
    assert_eq!(session.read_hypha_acts(), vec![(vk, ActsRead::Attached)], "attached at the signed head");
    session.absorb_digest(&digest(&k, &hypha, &position(&k, &line[3], b"root")));
    // A page off the line refuses: it does not chain from where the read stood.
    let mut fork = ReceiptChain::with_signer(vk);
    let _ = fork.append(ActKind::List, [8, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends");
    let off = fork.append(ActKind::Return, [9, 0, 0, 0], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    *page.lock().expect("the page") = vec![off];
    assert_eq!(session.read_hypha_acts(), vec![(vk, ActsRead::Refused)], "a page off the line is refused");
    // A page that chains but stops short of the signed head is held, and
    // nothing on it is perceived until the chain reaches a signed receipt.
    *page.lock().expect("the page") = line[1..3].to_vec();
    assert_eq!(session.read_hypha_acts(), vec![(vk, ActsRead::Read(Vec::new()))], "short of the head: held, not perceived");
    *page.lock().expect("the page") = line[3..].to_vec();
    let reads = session.read_hypha_acts();
    assert_eq!(
        reads,
        vec![(vk, ActsRead::Read(vec![HyphaAct { frame: [1, 0, 0, 0], height: line[1].stamp.h }, HyphaAct { frame: [0, 2, 0, 3], height: line[3].stamp.h }]))],
        "the hypha's acts, its List not among them"
    );
    let before = mind.pressing();
    mind.take_in_hypha_acts(&reads);
    let after = mind.pressing();
    let pressed: Vec<u64> = after.iter().zip(before).map(|(a, b)| a - b).collect();
    assert_eq!(pressed, vec![1, 2, 0, 3], "the frames press the mind's lanes, each on its own");
    assert_eq!(session.read_hypha_acts(), vec![(vk, ActsRead::Read(Vec::new()))], "read to the head, nothing more");
    mind.spend_pressing();
    assert_eq!(mind.pressing(), before, "a beat spends the hypha's flux");
}

/// THE ACT'S KIND CARRIES NO MEANING BEYOND ITS COORDINATE (the ruling):
/// a `Return`, a `RankStep` and a `Reconnect` sealed with one frame are one
/// perception, the same flux on the same lanes; the kind only says the
/// receipt is one of the hypha's own acts.
#[test]
fn act_kind_carries_no_meaning_beyond_its_coordinate() {
    let k = key("one frame three kinds");
    let vk = k.verifying_bytes();
    let frame = [3, 0, 1, 0];
    let read: Vec<Vec<[u64; 4]>> = [ActKind::Return, ActKind::RankStep, ActKind::Reconnect]
        .into_iter()
        .map(|kind| {
            let line = line_of(vk, &[(kind, frame)]);
            let (run, _) = chained(&line[1..], &crate::chronometry::Parent::of(&line[0]), &line[1]).expect("the page chains to the head");
            run.iter().filter(|r| crate::client::sdk::hypha_acts::is_hypha_act(r.kind)).map(|r| r.mu).collect()
        })
        .collect();
    assert!(read.iter().all(|r| r == &vec![frame]), "one frame, whatever the kind: {read:?}");
    let mut wells = [crate::client::sdk::hypha_acts::Attachment::default(); 3];
    for (w, r) in wells.iter_mut().zip(&read) {
        w.take_in(&r.iter().map(|f| HyphaAct { frame: *f, height: 1 }).collect::<Vec<_>>()).expect("fits");
    }
    assert!(wells.iter().all(|w| w.flux == frame), "and one flux on the well");
}
