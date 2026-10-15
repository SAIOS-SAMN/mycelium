// 78.167 C — THE ACT READ OFF ITS EFFECT. A family whose vertices do not say
// it — a body erased, cells created, the frame resized — carries the act as
// clauses, read off the pairs and standing only where they re-derive every
// one; the claim is the perspective's own: a HELD-OUT pair the family never
// showed is re-derived exactly, every body spoken for.

use crate::arc::Grid;
use crate::transducer::family::{extract_rule, speak::speak_rule, NoCrystal};

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// The rule the family extracts, its clauses read, and its speech on the
/// held-out input: exactly the held-out output, nothing unreached.
fn knows(family: &[(&[&str], &[&str])], held: (&[&str], &[&str])) {
    let family: Vec<(Grid, Grid)> = family.iter().map(|(x, y)| (grid(x), grid(y))).collect();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let rule = extract_rule(&family, &mut budget).expect("the family has a rule");
    assert!(!rule.form.clauses.is_empty(), "the act is stated as clauses");
    let s = speak_rule(&rule, &grid(held.0), &NoCrystal).expect("it speaks the held-out input");
    assert!(s.unreached.is_empty(), "every body spoken for: {:?}", s.unreached);
    assert_eq!(s.grid(), Some(grid(held.1)), "the held-out pair re-derived; clauses {:?}", rule.form.clauses);
}

/// C1 — A VANISHED BODY: every body of one hue erased, the rest kept.
#[test]
fn a_family_erasing_one_hue_is_known() {
    knows(
        &[
            (&["10000", "00000", "00200", "00000", "00001"], &["10000", "00000", "00000", "00000", "00001"]),
            (&["00002", "00000", "00000", "01000", "00000"], &["00000", "00000", "00000", "01000", "00000"]),
            (&["00000", "02200", "00000", "00000", "10000"], &["00000", "00000", "00000", "00000", "10000"]),
        ],
        (&["00200", "00000", "10000", "00000", "00022"], &["00000", "00000", "10000", "00000", "00000"]),
    );
}

/// C3 — A LINE: from each cell of hue 3, a line of its hue to the border on
/// its right.
#[test]
fn a_family_drawing_lines_to_the_border_is_known() {
    knows(
        &[
            (&["0000", "3000", "0000", "0000"], &["0000", "3333", "0000", "0000"]),
            (&["0000", "0000", "0030", "0000"], &["0000", "0000", "0033", "0000"]),
            (&["0300", "0000", "0000", "3000"], &["0333", "0000", "0000", "3333"]),
        ],
        (&["0000", "0000", "0000", "0300"], &["0000", "0000", "0000", "0333"]),
    );
}

/// C4 — AN ENCLOSURE FILLED: what a ring's boundary encloses, painted 4.
#[test]
fn a_family_filling_enclosed_regions_is_known() {
    knows(
        &[
            (&["11100", "10100", "11100", "00000", "00000"], &["11100", "14100", "11100", "00000", "00000"]),
            (&["00000", "00000", "00111", "00101", "00111"], &["00000", "00000", "00111", "00141", "00111"]),
            (&["01111", "01001", "01001", "01111", "00000"], &["01111", "01441", "01441", "01111", "00000"]),
        ],
        (&["00000", "11110", "10010", "11110", "00000"], &["00000", "11110", "14410", "11110", "00000"]),
    );
}

/// C2 — A TEMPLATE STAMPED: the one plus of hue 5 copied, centre onto
/// centre, over each cell of hue 8, which it replaces.
#[test]
fn a_family_stamping_a_template_is_known() {
    knows(
        &[
            (
                &["0500000", "5550000", "0500000", "0000000", "0000800", "0000000", "0000000"],
                &["0500000", "5550000", "0500000", "0000500", "0005550", "0000500", "0000000"],
            ),
            (
                &["0000000", "0800000", "0000000", "0000000", "0000050", "0000555", "0000050"],
                &["0500000", "5550000", "0500000", "0000000", "0000050", "0000555", "0000050"],
            ),
            (
                &["0000000", "0000080", "0000000", "0050000", "0555000", "0050000", "0000000"],
                &["0000050", "0000555", "0000050", "0050000", "0555000", "0050000", "0000000"],
            ),
        ],
        (
            &["0000000", "0050000", "0555000", "0050000", "0000000", "0000080", "0000000"],
            &["0000000", "0050000", "0555000", "0050000", "0000050", "0000555", "0000050"],
        ),
    );
}

/// C5 — THE FRAME CROPPED: the output is the extent of the one body of
/// hue 2, whatever else the scene holds.
#[test]
fn a_family_cropping_to_a_body_is_known() {
    knows(
        &[
            (&["00000", "02200", "02200", "00003", "00000"], &["22", "22"]),
            (&["30000", "00000", "00222", "00222", "00000"], &["222", "222"]),
            (&["00000", "00000", "00000", "20003", "20000"], &["2", "2"]),
        ],
        (&["00030", "22200", "00000", "00000", "00000"], &["222"]),
    );
}
