// 78.80, 78.81 — THE REMAINDER INTERPRETED (`family/interpret.rs`) and the
// recolour decided RELATIONALLY (`incongruence::decide`): a body's rank by
// mass among the bodies of its pair decides where its own volume, density and
// direction cannot — judged by replay, leave one out.

use super::common::grid;
use crate::transducer::family::incongruence::{decide, single_changes};
use crate::transducer::family::interpret::interpret;

/// The larger body of each pair turns 2, the smaller 5. The masses differ
/// between the pairs (3 and 1, then 4 and 2), so no volume is seen twice and
/// the volume key has nothing to decide by; every bar lies along the rows, so
/// direction sends one key two ways and is refused. The rank in the pair —
/// none heavier, or none lighter — is shared across the pairs, predicts every
/// recolour from the other pair, and the replay reproduces all four.
#[test]
fn rank_in_the_pair_decides_where_the_body_alone_does_not() {
    let family = vec![
        (grid(&["11100", "00000", "00001"]), grid(&["22200", "00000", "00005"])),
        (grid(&["11110", "00000", "00011"]), grid(&["22220", "00000", "00055"])),
    ];
    // The form spoke every site its input held, in the input's hue: what
    // changed is unsaid.
    let remainders: Vec<Vec<(i64, i64, u8, u8)>> = family
        .iter()
        .map(|(x, y)| {
            let mut rem = Vec::new();
            for (r, row) in y.iter().enumerate() {
                for (c, &h) in row.iter().enumerate() {
                    if h != 0 {
                        rem.push((r as i64, c as i64, x[r][c], h));
                    }
                }
            }
            rem
        })
        .collect();
    let changes = single_changes(&family, &remainders);
    assert_eq!(changes.len(), 4, "four bodies, each standing on one input body");
    assert!(changes.iter().all(|c| c.difference.kinematic()), "recolours in place");
    let d = decide(&changes);
    assert_eq!((d[1].predicted, d[1].alone), (0, 4), "no volume seen twice: nothing to decide by");
    assert!(!d[5].whole, "one direction for both sizes: refused");
    assert_eq!((d[6].predicted, d[6].reproduced), (4, 4), "rank by mass, leave one out, reproduces every recolour");
    assert!(d[6].whole);
}

/// THE MIND'S INVERSION READS THE SPEECH'S OWN REMAINDER (78.76, 78.90). On
/// an extracted form's family: the inversion reads exactly the links the
/// speech left, and an accepted inversion hands back the joined form with
/// remainders summing to what it read after.
#[test]
fn the_interpreter_reads_the_remainder_the_speech_left() {
    let family = vec![
        (grid(&["11100", "00000", "00001"]), grid(&["22200", "00000", "00005"])),
        (grid(&["11110", "00000", "00011"]), grid(&["22220", "00000", "00055"])),
    ];
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let rule = crate::transducer::family::extract_rule(&family, &mut budget).expect("a rule");
    let empty = crate::client::sdk::Epigenome::new();
    let (_, rem) = crate::transducer::family::form_deflation(&rule.form, &family, &empty);
    let left: u64 = rem.iter().map(|r| r.len() as u64).sum();
    let (joined, read) = interpret(&rule.form, &family, &rem, &empty, &mut budget);
    match read.inversion {
        None => assert_eq!(left, 0, "nothing read only where nothing was left"),
        Some(inv) => {
            assert_eq!(inv.before, left, "the inversion reads the speech's own remainder");
            match &joined {
                Some((_, after)) => {
                    assert!(inv.accepted);
                    assert_eq!(after.iter().map(|r| r.len() as u64).sum::<u64>(), inv.after);
                }
                None => assert!(!inv.accepted, "a form is handed back only where the speech accepted it"),
            }
        }
    }
}
