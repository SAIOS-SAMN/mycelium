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

/// A named case: its family's pairs and the held-out pair, each grid as rows.
type Case<'a> = (&'a str, Vec<(&'a [&'a str], &'a [&'a str])>, (&'a [&'a str], &'a [&'a str]));

/// The rule the family extracts, its clauses read, and its speech on the
/// held-out input: exactly the held-out output, nothing unreached. The
/// clauses it was read as.
fn knows(family: &[(&[&str], &[&str])], held: (&[&str], &[&str])) -> Vec<crate::transducer::sentence::form::clause::Clause> {
    let family: Vec<(Grid, Grid)> = family.iter().map(|(x, y)| (grid(x), grid(y))).collect();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let rule = extract_rule(&family, &mut budget).expect("the family has a rule");
    assert!(!rule.form.elements.is_empty(), "the act is stated as clauses");
    let s = speak_rule(&rule, &grid(held.0), &NoCrystal).expect("it speaks the held-out input");
    assert!(s.unreached.is_empty(), "every body spoken for: {:?}", s.unreached);
    assert_eq!(s.grid(), Some(grid(held.1)), "the held-out pair re-derived; elements {:?}", rule.form.elements);
    glosses(&rule.form.elements)
}

/// The clauses naming a rule's elements: every element the act reader
/// reads is a clause it read, so each has its gloss.
fn glosses(elements: &[crate::transducer::sentence::form::rule_element::Lowered]) -> Vec<crate::transducer::sentence::form::clause::Clause> {
    elements.iter().map(|l| crate::transducer::sentence::form::rule_element::gloss(l).expect("a read clause's element is named")).collect()
}

/// C1 — A VANISHED BODY: every body of one hue erased, the rest kept.
#[test]
fn extract_rule_reads_erasing_one_hue() {
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
fn extract_rule_reads_lines_to_border() {
    knows(
        &[
            (&["0000", "3000", "0000", "0000"], &["0000", "3333", "0000", "0000"]),
            (&["0000", "0000", "0030", "0000"], &["0000", "0000", "0033", "0000"]),
            (&["0300", "0000", "0000", "3000"], &["0333", "0000", "0000", "3333"]),
        ],
        (&["0000", "0000", "0000", "0300"], &["0000", "0000", "0000", "0333"]),
    );
}

/// 78.167 Phase B — LINES ALONG TWO RAYS: from each cell of hue 3, a line of
/// its hue to the border rightward and another downward. Each ray is
/// measured off the effect — the displacement from the cell a run leaves —
/// so two rays are two clauses, never two of eight rays tried.
#[test]
fn extract_rule_reads_lines_along_two_rays() {
    knows(
        &[
            (&["0000", "0300", "0000", "0000"], &["0000", "0333", "0300", "0300"]),
            (&["3000", "0000", "0000", "0000"], &["3333", "3000", "3000", "3000"]),
            (&["0000", "0000", "0030", "0000"], &["0000", "0000", "0033", "0030"]),
        ],
        (&["0000", "0000", "0300", "0000"], &["0000", "0000", "0333", "0300"]),
    );
}

/// 78.167 Phase B, Step 1 — TWO ACTS IN ONE SCENE: a ring's interior filled 4
/// and, from each cell of hue 3, a line of its hue to the border on its right.
/// Read as one act the created cells are neither a fill nor a line; read
/// piece by piece, the fill's piece and the line's piece are each a clause of
/// its own, and the composed sentence is proved once.
#[test]
fn extract_rule_reads_fill_and_line_at_once() {
    knows(
        &[
            (&["11100", "10100", "11100", "00000", "30000"], &["11100", "14100", "11100", "00000", "33333"]),
            (&["00000", "00111", "30101", "00111", "00000"], &["00000", "00111", "33141", "00111", "00000"]),
            (&["01110", "01010", "01110", "00000", "00030"], &["01110", "01410", "01110", "00000", "00033"]),
        ],
        (&["30000", "00000", "11100", "10100", "11100"], &["33333", "00000", "11100", "14100", "11100"]),
    );
}

/// 78.167 Phase B, Step 2 — BODIES JOINED: every two cells of hue 1 facing
/// each other along a row or a column joined in hue 3; a 1 with no partner
/// kept. The join is read off the straight pieces whose ends abut two bodies;
/// no line says it (a line from the lone 1 would run to the border).
#[test]
fn extract_rule_reads_join_of_facing_bodies() {
    knows(
        &[
            (&["10010", "00000", "00100"], &["13310", "00000", "00100"]),
            (&["00000", "10001", "00000", "01000"], &["00000", "13331", "00000", "01000"]),
            (&["0100", "0000", "0000", "0100", "0001"], &["0100", "0300", "0300", "0100", "0001"]),
        ],
        (&["1000", "0000", "1001"], &["1000", "3000", "1331"]),
    );
}

/// 78.167 Phase B, Step 3 — A BODY ECHOED: every body of the scene repeated
/// two rows below itself, the source kept. The offset is the one every
/// echoed piece offers — read off them, no offset swept.
#[test]
fn extract_rule_reads_echo_of_bodies() {
    knows(
        &[
            (&["120", "000", "000", "000"], &["120", "000", "120", "000"]),
            (&["000", "033", "000", "000", "000"], &["000", "033", "000", "033", "000"]),
            (&["0400", "0400", "0000", "0000", "0000"], &["0400", "0400", "0400", "0400", "0000"]),
        ],
        (&["5000", "0000", "0000", "0000"], &["5000", "0000", "5000", "0000"]),
    );
}

/// AN ECHO MIRRORED (ruled 2026-10-20): every body
/// repeated three columns over, mirrored across its own columns, the source
/// kept. The piece is the body's shape under the square's symmetry the group
/// reaches first; no symmetry is tried against the output.
#[test]
fn extract_rule_reads_mirrored_echo() {
    use crate::transducer::sentence::form::clause::{About, Act};
    let clauses = knows(
        &[
            (&["1100000", "1000000", "0000000"], &["1101100", "1000100", "0000000"]),
            (&["0000000", "0022000", "0020000"], &["0000000", "0022022", "0020002"]),
            (&["000000", "110000", "100000", "000000"], &["000000", "110110", "100010", "000000"]),
        ],
        (&["330000", "300000", "000000"], &["330330", "300030", "000000"]),
    );
    assert!(clauses.iter().any(|c| matches!(c.act, Act::Echo { about: About::Body, .. })), "read as an echo about each body: {clauses:?}");
}

/// ECHOES AT TWO OFFSETS (ruled 2026-10-20: multi-offset
/// stamps): every body repeated two rows below and two columns over. No one
/// offset is offered by every echoed piece, so the one-offset reading leaves
/// them; each offset every pair shows is its own echo.
#[test]
fn extract_rule_reads_echoes_at_two_offsets() {
    use crate::transducer::sentence::form::clause::Act;
    let clauses = knows(
        &[
            (&["1000", "0000", "0000", "0000"], &["1010", "0000", "1000", "0000"]),
            (&["0000", "0200", "0000", "0000"], &["0000", "0202", "0000", "0200"]),
            (&["0300", "0000", "0000"], &["0303", "0000", "0300"]),
        ],
        (&["0000", "4000", "0000", "0000"], &["0000", "4040", "0000", "4000"]),
    );
    let echoes = clauses.iter().filter(|c| matches!(c.act, Act::Echo { .. })).count();
    assert_eq!(echoes, 2, "one echo for each offset: {clauses:?}");
}

/// BODIES JOINED ROUND A CORNER (ruled 2026-10-20:
/// Manhattan bent connectors): from the 2 along its row to the 3's column,
/// and along that column to the 3, in hue 8 — the corner an empty cell. The
/// bend is read off the piece: a run on a row and a run on a column meeting
/// at the corner, each abutting the body it leaves or reaches.
#[test]
fn extract_rule_reads_bend_round_corner() {
    use crate::transducer::sentence::form::clause::Act;
    let clauses = knows(
        &[
            (
                &["000030", "000000", "000000", "000000", "200000", "000000"],
                &["000030", "000080", "000080", "000080", "288880", "000000"],
            ),
            (&["020000", "000000", "000000", "000000", "000030"], &["028880", "000080", "000080", "000080", "000030"]),
            (
                &["000000", "000000", "000002", "000000", "000000", "030000"],
                &["000000", "000000", "088882", "080000", "080000", "030000"],
            ),
        ],
        (&["00030", "00000", "00000", "20000", "00000"], &["00030", "00080", "00080", "28880", "00000"]),
    );
    assert!(clauses.iter().any(|c| matches!(c.act, Act::Bend { .. })), "read as a bend: {clauses:?}");
}

/// THE CREATION ACTS CROSS THE WIRE: rules stating a mirrored echo, echoes
/// at two offsets and a bend, minted as the elements the court reads and
/// decoded back, re-mint at one address.
#[test]
fn from_element_form_reads_back_echoes_and_bends() {
    for family in [
        vec![(grid(&["1100000", "1000000", "0000000"]), grid(&["1101100", "1000100", "0000000"])), (grid(&["0000000", "0022000", "0020000"]), grid(&["0000000", "0022022", "0020002"]))],
        vec![(grid(&["1000", "0000", "0000", "0000"]), grid(&["1010", "0000", "1000", "0000"])), (grid(&["0000", "0200", "0000", "0000"]), grid(&["0000", "0202", "0000", "0200"]))],
        vec![
            (grid(&["000030", "000000", "000000", "000000", "200000", "000000"]), grid(&["000030", "000080", "000080", "000080", "288880", "000000"])),
            (grid(&["020000", "000000", "000000", "000000", "000030"]), grid(&["028880", "000080", "000080", "000080", "000030"])),
        ],
    ] {
        let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
        let rule = extract_rule(&family, &mut budget).expect("the family has a rule");
        assert!(!rule.form.elements.is_empty(), "stated as clauses");
        let element = crate::transducer::sentence::as_element_form(&rule.form, [1, 0, 0, 0]).expect("the rule mints");
        let back = crate::transducer::sentence::from_element_form(&element).expect("the court decodes it");
        assert_eq!(back.elements.len(), rule.form.elements.len(), "every clause read back");
        let again = crate::transducer::sentence::as_element_form(&back, [1, 0, 0, 0]).expect("the decoded rule mints");
        assert_eq!(again.address(), element.address(), "the clauses read back as they were read: one address");
    }
}

/// THE NEW ACTS CROSS THE WIRE: a rule stating a join and one stating an
/// echo, minted as the elements the court reads and decoded back, carry the
/// clauses they were read with — the court replays what the voice read.
#[test]
fn from_element_form_reads_back_join_and_echo() {
    for family in [
        vec![(grid(&["10010", "00000", "00100"]), grid(&["13310", "00000", "00100"])), (grid(&["00000", "10001", "00000", "01000"]), grid(&["00000", "13331", "00000", "01000"]))],
        vec![(grid(&["120", "000", "000", "000"]), grid(&["120", "000", "120", "000"])), (grid(&["000", "033", "000", "000", "000"]), grid(&["000", "033", "000", "033", "000"]))],
    ] {
        let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
        let rule = extract_rule(&family, &mut budget).expect("the family has a rule");
        assert!(!rule.form.elements.is_empty(), "stated as clauses");
        let element = crate::transducer::sentence::as_element_form(&rule.form, [1, 0, 0, 0]).expect("the rule mints");
        let back = crate::transducer::sentence::from_element_form(&element).expect("the court decodes it");
        assert_eq!(back.elements.len(), rule.form.elements.len(), "every clause read back");
        let again = crate::transducer::sentence::as_element_form(&back, [1, 0, 0, 0]).expect("the decoded rule mints");
        assert_eq!(again.address(), element.address(), "the clauses read back as they were read: one address");
    }
}

/// 78.167 Phase B, Step 4 — TWO PANES BY A TRUTH TABLE: the input two panes
/// across a separator column of 5, and the output painted 2 where exactly one
/// pane is painted (the table XOR names). The table is read cell by cell off
/// the pairs, every entry seen.
#[test]
fn extract_rule_reads_two_panes_by_truth_table() {
    knows(
        &[
            (&["10501", "01500"], &["22", "02"]),
            (&["11500", "00501"], &["22", "02"]),
            (&["00511", "10510"], &["22", "00"]),
        ],
        (&["01510", "10500"], &["22", "20"]),
    );
}

/// 78.167 Phase B, Step 4 — TILES BY THE SQUARE'S SYMMETRIES: the output two
/// by two blocks of the input — itself, mirrored across its columns, across
/// its rows, and turned 180°. Each block's symmetry is read by testing the
/// group's elements against it; no tiling is tried.
#[test]
fn extract_rule_reads_tiles_by_symmetries() {
    knows(
        &[
            (&["12", "30"], &["1221", "3003", "3003", "1221"]),
            (&["40", "05"], &["4004", "0550", "0550", "4004"]),
        ],
        (&["60", "70"], &["6006", "7007", "7007", "6006"]),
    );
}

/// THE FRAME ACTS CROSS THE WIRE: rules stating two panes and four tiles,
/// minted as the elements the court reads and decoded back, re-mint at one
/// address.
#[test]
fn from_element_form_reads_back_panes_and_tiles() {
    for family in [
        vec![(grid(&["10501", "01500"]), grid(&["22", "02"])), (grid(&["11500", "00501"]), grid(&["22", "02"])), (grid(&["00511", "10510"]), grid(&["22", "00"]))],
        vec![(grid(&["12", "30"]), grid(&["1221", "3003", "3003", "1221"])), (grid(&["40", "05"]), grid(&["4004", "0550", "0550", "4004"]))],
    ] {
        let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
        let rule = extract_rule(&family, &mut budget).expect("the family has a rule");
        assert!(!rule.form.elements.is_empty(), "stated as clauses");
        let element = crate::transducer::sentence::as_element_form(&rule.form, [1, 0, 0, 0]).expect("the rule mints");
        let back = crate::transducer::sentence::from_element_form(&element).expect("the court decodes it");
        let again = crate::transducer::sentence::as_element_form(&back, [1, 0, 0, 0]).expect("the decoded rule mints");
        assert_eq!(again.address(), element.address(), "the clauses read back as they were read: one address");
    }
}

/// C4 — AN ENCLOSURE FILLED: what a ring's boundary encloses, painted 4.
#[test]
fn extract_rule_reads_enclosure_fill() {
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
fn extract_rule_reads_template_stamp() {
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
fn extract_rule_reads_crop_to_body() {
    knows(
        &[
            (&["00000", "02200", "02200", "00003", "00000"], &["22", "22"]),
            (&["30000", "00000", "00222", "00222", "00000"], &["222", "222"]),
            (&["00000", "00000", "00000", "20003", "20000"], &["2", "2"]),
        ],
        (&["00030", "22200", "00000", "00000", "00000"], &["222"]),
    );
}

/// 78.167 Phase A — MAPS THAT DIFFER ACROSS BODIES: one hue, one shape, and
/// the topmost bodies recoloured 1 while the bottommost are recoloured 2.
/// Each map is read off the bodies that showed it, and each class is
/// selected by the coordinate that separates it — never one map forced over
/// all of them (refused as inseparable before).
#[test]
fn extract_rule_reads_recolour_by_position() {
    knows(
        &[
            (&["5050", "0000", "0500"], &["1010", "0000", "0200"]),
            (&["0005", "0500", "0000"], &["0001", "0200", "0000"]),
            (&["0000", "5000", "0050"], &["0000", "1000", "0020"]),
        ],
        (&["0500", "0005"], &["0100", "0002"]),
    );
}

/// SELECTION READ OFF THE TARGETS: the cells of hue 5 that touch a 2 are
/// recoloured 1 — a 5 touching nothing, and a 3 touching a 2, kept. Every
/// body is one cell, one chart, so no vertex's gauge can say it; and no one
/// coordinate of the list does (hue 5 holds a 5 that stays; touching a 2
/// holds the 3). The meet of what the targets share — hue 5, touches a 2 —
/// does, read off them.
#[test]
fn extract_rule_reads_selection_by_two_coordinates() {
    knows(
        &[
            (&["50000", "02003", "00020", "50000"], &["10000", "02003", "00020", "50000"]),
            (&["30000", "02000", "00500", "00020", "50000"], &["30000", "02000", "00100", "00020", "50000"]),
        ],
        (&["02000", "50005", "00000"], &["02000", "10005", "00000"]),
    );
}

/// 78.167 Phase A — A BODY RECOLOURED IN PART: one body of hues 1 and 5,
/// its least monochrome part recoloured 3 while a part of the same hue in
/// the same body is kept. Read on the scene's parts — each kept or
/// recoloured whole — the recolour is one clause on parts.
#[test]
fn extract_rule_reads_recolour_of_body_part() {
    knows(
        &[(&["15511", "00000"], &["35511", "00000"]), (&["11551", "00000"], &["11553", "00000"])],
        (&["155111", "000000"], &["355111", "000000"]),
    );
}

/// THE COURT SOLVES WHAT THE ACT READER READ: a rule stated as clauses —
/// tiles, panes, a fill and a line at once, recolours by where bodies stand,
/// an echo — mints over its family (the clauses its whole speech, no vertex
/// beside them claiming frames the effect refuted), and the court's own
/// judging solves it on a pair the family never showed. This is the path a
/// first closure takes.
#[test]
fn judge_solve_solves_act_reader_clauses() {
    let cases: Vec<Case> = vec![
        (
            "tiles",
            vec![(&["12", "30"], &["1221", "3003", "3003", "1221"]), (&["40", "05"], &["4004", "0550", "0550", "4004"])],
            (&["60", "70"], &["6006", "7007", "7007", "6006"]),
        ),
        (
            "panes",
            vec![(&["10501", "01500"], &["22", "02"]), (&["11500", "00501"], &["22", "02"]), (&["00511", "10510"], &["22", "00"])],
            (&["01510", "10500"], &["22", "20"]),
        ),
        (
            "a fill and a line",
            vec![
                (&["11100", "10100", "11100", "00000", "30000"], &["11100", "14100", "11100", "00000", "33333"]),
                (&["00000", "00111", "30101", "00111", "00000"], &["00000", "00111", "33141", "00111", "00000"]),
                (&["01110", "01010", "01110", "00000", "00030"], &["01110", "01410", "01110", "00000", "00033"]),
            ],
            (&["30000", "00000", "11100", "10100", "11100"], &["33333", "00000", "11100", "14100", "11100"]),
        ),
        (
            "recolours by where bodies stand",
            vec![
                (&["5050", "0000", "0500"], &["1010", "0000", "0200"]),
                (&["0005", "0500", "0000"], &["0001", "0200", "0000"]),
                (&["0000", "5000", "0050"], &["0000", "1000", "0020"]),
            ],
            (&["0500", "0005"], &["0100", "0002"]),
        ),
        (
            "an echo",
            vec![
                (&["120", "000", "000", "000"], &["120", "000", "120", "000"]),
                (&["000", "033", "000", "000", "000"], &["000", "033", "000", "033", "000"]),
                (&["0400", "0400", "0000", "0000", "0000"], &["0400", "0400", "0400", "0400", "0000"]),
            ],
            (&["5000", "0000", "0000", "0000"], &["5000", "0000", "5000", "0000"]),
        ),
        (
            "a mirrored echo",
            vec![
                (&["1100000", "1000000", "0000000"], &["1101100", "1000100", "0000000"]),
                (&["0000000", "0022000", "0020000"], &["0000000", "0022022", "0020002"]),
                (&["000000", "110000", "100000", "000000"], &["000000", "110110", "100010", "000000"]),
            ],
            (&["330000", "300000", "000000"], &["330330", "300030", "000000"]),
        ),
        (
            "echoes at two offsets",
            vec![
                (&["1000", "0000", "0000", "0000"], &["1010", "0000", "1000", "0000"]),
                (&["0000", "0200", "0000", "0000"], &["0000", "0202", "0000", "0200"]),
                (&["0300", "0000", "0000"], &["0303", "0000", "0300"]),
            ],
            (&["0000", "4000", "0000", "0000"], &["0000", "4040", "0000", "4000"]),
        ),
        (
            "a bend",
            vec![
                (
                    &["000030", "000000", "000000", "000000", "200000", "000000"],
                    &["000030", "000080", "000080", "000080", "288880", "000000"],
                ),
                (&["020000", "000000", "000000", "000000", "000030"], &["028880", "000080", "000080", "000080", "000030"]),
                (
                    &["000000", "000000", "000002", "000000", "000000", "030000"],
                    &["000000", "000000", "088882", "080000", "080000", "030000"],
                ),
            ],
            (&["00030", "00000", "00000", "20000", "00000"], &["00030", "00080", "00080", "28880", "00000"]),
        ),
    ];
    for (name, fam, held) in cases {
        let family: Vec<(Grid, Grid)> = fam.iter().map(|(x, y)| (grid(x), grid(y))).collect();
        let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
        let rule = extract_rule(&family, &mut budget).expect("the family has a rule");
        assert!(!rule.form.elements.is_empty(), "{name}: stated as clauses");
        let minted = match crate::transducer::family::mint_sentence_why(&rule, &family, &NoCrystal, 0) {
            Ok(word) => word,
            Err(why) => panic!("{name}: the rule mints over its family: {why:?}"),
        };
        let mut court = crate::cognition::Court::new();
        let verdict = court.judge_solve(&minted, &[(grid(held.0), grid(held.1))]);
        assert!(matches!(verdict, crate::cognition::SolveVerdict::Solved { .. }), "{name}: the court solves it on a pair it never saw: {verdict:?}");
    }
}

/// THE REACTIONS (ruled 2026-10-25): a body that falls until it meets the
/// floor, three rows in one pair, two in another, one in the third; and a
/// body that takes the hue of the body it touches, a different hue in each
/// pair. Each is one relation where a fixed carriage or a fixed map fits
/// each pair alone. The held-out pair — another distance, another hue — is
/// re-derived, and the court, decoding the clauses, solves it.
#[test]
fn judge_solve_solves_reaction_on_unseen_pair() {
    use crate::transducer::sentence::form::clause::{Act, Paint};
    let cases: Vec<Case> = vec![
        (
            "a slide to contact",
            vec![
                (&["0100", "0000", "0000", "0000", "2222"], &["0000", "0000", "0000", "0100", "2222"]),
                (&["0000", "0010", "0000", "0000", "2222"], &["0000", "0000", "0000", "0010", "2222"]),
                (&["0000", "0000", "1000", "0000", "2222"], &["0000", "0000", "0000", "1000", "2222"]),
            ],
            (&["0001", "0000", "0000", "0000", "2222"], &["0000", "0000", "0000", "0001", "2222"]),
        ),
        (
            "a partner's hue",
            vec![
                (&["300", "050", "000"], &["300", "030", "000"]),
                (&["000", "050", "004"], &["000", "040", "004"]),
                (&["006", "050", "000"], &["006", "060", "000"]),
            ],
            (&["000", "050", "700"], &["000", "070", "700"]),
        ),
    ];
    for (name, fam, held) in cases {
        let family: Vec<(Grid, Grid)> = fam.iter().map(|(x, y)| (grid(x), grid(y))).collect();
        let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
        let rule = extract_rule(&family, &mut budget).expect("the family has a rule");
        let reacted = glosses(&rule.form.elements).iter().any(|c| matches!(c.act, Act::Slide { .. } | Act::Repaint { paint: Paint::Of(_) }));
        assert!(reacted, "{name}: stated as a reaction: {:?}", rule.form.elements);
        let s = speak_rule(&rule, &grid(held.0), &NoCrystal).expect("it speaks the held-out input");
        assert_eq!(s.grid(), Some(grid(held.1)), "{name}: the held-out pair re-derived; elements {:?}", rule.form.elements);
        let minted = match crate::transducer::family::mint_sentence_why(&rule, &family, &NoCrystal, 0) {
            Ok(word) => word,
            Err(why) => panic!("{name}: the rule mints over its family: {why:?}"),
        };
        let mut court = crate::cognition::Court::new();
        let verdict = court.judge_solve(&minted, &[(grid(held.0), grid(held.1))]);
        assert!(matches!(verdict, crate::cognition::SolveVerdict::Solved { .. }), "{name}: the court solves it on a pair it never saw: {verdict:?}");
    }
}

/// AN INTERIOR SEVERAL BODIES WALL IN TOGETHER IS A FILL: four bars meeting
/// only at the bars' corners close a hole no one of them closes, whose extent
/// holds no ground, and whose halo reaches ground the truth leaves; it is
/// filled with 4 in three pairs of three sizes. The fill is read off the
/// bodies around it, together, as the bodies' joint interior; the held-out pair,
/// another size, is re-derived, and the court solves it.
#[test]
fn extract_rule_fills_hole_closed_jointly() {
    let family: Vec<(&[&str], &[&str])> = vec![
        (&["0110", "1001", "1001", "0110"], &["0110", "1441", "1441", "0110"]),
        (&["011110", "100001", "100001", "100001", "011110"], &["011110", "144441", "144441", "144441", "011110"]),
        (&["011110", "100001", "100001", "100001", "100001", "011110"], &["011110", "144441", "144441", "144441", "144441", "011110"]),
    ];
    let held: (&[&str], &[&str]) = (&["01110", "10001", "10001", "10001", "01110"], &["01110", "14441", "14441", "14441", "01110"]);
    let clauses = knows(&family, held);
    use crate::transducer::sentence::form::clause::{Act, Region};
    assert!(clauses.iter().any(|c| matches!(c.act, Act::Fill { region: Region::Interior, .. })), "read as the joint interior: {clauses:?}");
    let family: Vec<(Grid, Grid)> = family.iter().map(|(x, y)| (grid(x), grid(y))).collect();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let rule = extract_rule(&family, &mut budget).expect("the family has a rule");
    let minted = crate::transducer::family::mint_sentence_why(&rule, &family, &NoCrystal, 0).expect("the rule mints over its family");
    let mut court = crate::cognition::Court::new();
    let verdict = court.judge_solve(&minted, &[(grid(held.0), grid(held.1))]);
    assert!(matches!(verdict, crate::cognition::SolveVerdict::Solved { .. }), "the court solves it on a pair it never saw: {verdict:?}");
}

/// A STACK SETTLES AS IT LANDS: two bodies in one column slide down to
/// contact, the nearest the floor first, so the upper rests on the lower; a
/// slide with nothing to meet stops at the border; a ray of no step is
/// refused by name.
#[test]
fn speak_clauses_settles_stack_and_stops_at_border() {
    use crate::transducer::family::statement::{speak_clauses, Unstated};
    use crate::transducer::sentence::form::clause::{Act, Clause, Pred, Stop};
    let say = |clause: &Clause, x: &[&str]| -> Result<Grid, Unstated> {
        let x = grid(x);
        let spoken = speak_clauses(std::slice::from_ref(clause), &x, &crate::transducer::family::perceive(&x))?;
        let mut g: Grid = vec![vec![0; spoken.frame.1]; spoken.frame.0];
        for (r, c, h) in spoken.painted {
            g[usize::try_from(r).expect("on the grid")][usize::try_from(c).expect("on the grid")] = h;
        }
        Ok(g)
    };
    let fall = Clause { select: Pred::Hue(1), act: Act::Slide { ray: (1, 0), stop: Stop::Contact } };
    assert_eq!(say(&fall, &["0100", "0000", "0100", "0000", "2222"]), Ok(grid(&["0000", "0000", "0100", "0100", "2222"])), "the lower lands on the floor, the upper on it");
    assert_eq!(say(&fall, &["0100", "0000", "0000"]), Ok(grid(&["0000", "0000", "0100"])), "nothing met: the border");
    let still = Clause { select: Pred::Hue(1), act: Act::Slide { ray: (0, 0), stop: Stop::Contact } };
    assert_eq!(say(&still, &["0100"]), Err(Unstated::Still), "no step: refused by name");
}

/// A PARTNER OF TWO HUES IS NO ONE PARTNER: a body touching a body of 3 and
/// one of 4 takes neither, refused by name; one touching nothing likewise.
#[test]
fn speak_clauses_refuses_partner_hue_without_one_partner() {
    use crate::transducer::family::statement::{speak_clauses, Unstated};
    use crate::transducer::sentence::form::clause::{Act, Clause, Paint, Pred};
    let take = Clause { select: Pred::Hue(5), act: Act::Repaint { paint: Paint::Of(Pred::All) } };
    for x in [grid(&["300", "050", "004"]), grid(&["000", "050", "000"])] {
        let spoken = speak_clauses(std::slice::from_ref(&take), &x, &crate::transducer::family::perceive(&x));
        assert_eq!(spoken.err(), Some(Unstated::Unpartnered), "refused by name on {x:?}");
    }
}

/// THE REACTIONS CROSS THE WIRE, AND NO FORM ALREADY MINTED MOVES: a slide, a
/// repaint by a partner's hue and a line painted by one read back as written;
/// a line painted its own hue or a named one writes the bytes it always did.
#[test]
fn put_clauses_round_trips_reactions_keeps_old_bytes() {
    use crate::transducer::sentence::form::clause::{put_clauses, take_clauses, Act, Clause, Paint, Pred, Stop};
    let clauses = vec![
        Clause { select: Pred::Hue(1), act: Act::Slide { ray: (1, -1), stop: Stop::Contact } },
        Clause { select: Pred::Hue(5), act: Act::Repaint { paint: Paint::Of(Pred::Hue(3)) } },
        Clause { select: Pred::All, act: Act::Extend { ray: (0, 1), paint: Paint::Of(Pred::All), stop: Stop::Border } },
    ];
    let mut bytes = Vec::new();
    put_clauses(&mut bytes, &clauses);
    let mut p = 0;
    let back = take_clauses(&bytes, &mut p).expect("read back");
    assert_eq!(p, bytes.len(), "every byte read");
    let mut again = Vec::new();
    put_clauses(&mut again, &back);
    assert_eq!(again, bytes, "read back as written");
    for (paint, tail) in [(Paint::Own, vec![0u8]), (Paint::Hue(7), vec![1u8, 7])] {
        let mut b = Vec::new();
        put_clauses(&mut b, &[Clause { select: Pred::All, act: Act::Extend { ray: (0, 1), paint, stop: Stop::Contact } }]);
        let mut want = Vec::new();
        put_clauses(&mut want, &[]);
        let count = want.len();
        let mut expected = b[..count].to_vec();
        expected.push(0); // Pred::All
        expected.extend_from_slice(&[3, 0, 1]);
        expected.extend_from_slice(&tail);
        expected.push(1); // Stop::Contact
        assert_eq!(b[count..], expected[count..], "an extend writes the bytes it was first written with");
    }
}
