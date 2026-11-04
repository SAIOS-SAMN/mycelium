// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §3.3; ROADMAP track 26 S3 — THE SYMBOL SET DEDUCED. A family's
// symbol set Σ is its distinct symbols compared by equality, `|Σ|` read from
// the data; an index into Σ is the order of first appearance in canonical
// address order; e_∅ stands at `|Σ|`; a table over symbols is keyed by them;
// and a symbol's code is a label, so relabelling Σ changes no percept.

use crate::transducer::element::{fibre_on_symbols, lift_fibre, RESERVOIR};
use crate::transducer::family::transduce::Ground;
use crate::transducer::family::{fibre_relation, perceive, Family};

/// A family held whole: its pairs in the order the family gives them.
type Pairs = Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>;
use crate::transducer::raster::{ground_of, on_ground_of};
use crate::transducer::sample::{Place, Sample};
use crate::transducer::symbols::{SymbolRefusal, SymbolSet};

/// One row of symbols framed by the ground 0: a row of 0 above and below,
/// and a 0 at each end, so only 0 meets the seam.
fn framed(row: &[u8]) -> Vec<Vec<u8>> {
    let width = row.len() + 2;
    let mut middle = vec![0];
    middle.extend_from_slice(row);
    middle.push(0);
    vec![vec![0; width], middle, vec![0; width]]
}

/// A family on the ground 0: one body recoloured 1 → 3 in place on each
/// pair, a body of 2 standing still.
fn family() -> Pairs {
    vec![
        (framed(&[1, 1, 0, 2]), framed(&[3, 3, 0, 2])),
        (framed(&[2, 0, 1, 0, 1]), framed(&[2, 0, 3, 0, 3])),
    ]
}

/// A relabelling of the family's symbols: not monotone, and past ten. It
/// avoids the tape's record byte (`RESERVOIR`), which is no symbol until
/// track 26 S5 (MU-SENSING §8).
fn relabel(h: u8) -> u8 {
    match h {
        0 => 7,
        1 => 11,
        2 => 4,
        3 => 200,
        other => other,
    }
}

fn relabelled(family: &Family) -> Pairs {
    let raster = |g: &Vec<Vec<u8>>| g.iter().map(|row| row.iter().map(|&h| relabel(h)).collect()).collect();
    family.iter().map(|(x, y)| (raster(x), raster(y))).collect()
}

/// The symbols of a set in index order, written out.
fn listed(set: &SymbolSet) -> Vec<Vec<u8>> {
    set.symbols().map(<[u8]>::to_vec).collect()
}

/// A body read for comparison: its compartments, sorted, and its moduli.
type Read = (Vec<crate::transducer::blade::Compartment>, crate::transducer::stalk::Invariant);

/// The bodies of one raster under the family's own ground, with the
/// compartments' symbols relabelled by `by`.
fn bodies(family: &Family, raster: &Vec<Vec<u8>>, by: impl Fn(u8) -> u8) -> Vec<Read> {
    on_ground_of(family, || {
        perceive(raster)
            .into_iter()
            .map(|b| {
                let mut cs: Vec<_> = b
                    .compartments
                    .into_iter()
                    .map(|mut c| {
                        c.hue = by(c.hue);
                        c
                    })
                    .collect();
                cs.sort();
                (cs, b.moduli)
            })
            .collect()
    })
}

/// The fibre relation read on Σ's indices, rows in index order.
fn relation_on_symbols(family: &Family) -> Option<Vec<(usize, usize)>> {
    let sigma = fibre_relation(family)?;
    let mut on = fibre_on_symbols(&sigma, &SymbolSet::of_pairs(family))?;
    on.sort_unstable();
    Some(on)
}

/// MU-SENSING §3.3 — PERMUTING Σ'S LABELS IS A GAUGE TRANSFORMATION: the
/// ground, every body (its sites, its moduli, its compartments' symbols up
/// to the relabel), the fibre relation read on Σ's indices, the symbol set's
/// indices, the extraction's reading and the refusal of a family that is no
/// fibre relation all read the same, up to the relabel.
#[test]
fn percept_invariant_under_symbol_permutation() {
    let plain = family();
    let moved = relabelled(&plain);
    assert_eq!(ground_of(&plain), Ground::One(0), "the plain family stands on 0");
    assert_eq!(ground_of(&moved), Ground::One(relabel(0)), "the relabelled one on 0's image");
    let (sp, sm) = (SymbolSet::of_pairs(&plain), SymbolSet::of_pairs(&moved));
    assert_eq!(sp.len(), sm.len(), "|Σ| is read, and the relabel keeps it");
    for h in [0u8, 1, 2, 3] {
        assert_eq!(sp.index(&[h]), sm.index(&[relabel(h)]), "symbol {h} keeps its index");
    }
    for ((xp, yp), (xm, ym)) in plain.iter().zip(&moved) {
        assert_eq!(bodies(&plain, xp, relabel), bodies(&moved, xm, |h| h), "the input's bodies");
        assert_eq!(bodies(&plain, yp, relabel), bodies(&moved, ym, |h| h), "the output's bodies");
    }
    let on = relation_on_symbols(&plain).expect("the plain fibre moved");
    assert_eq!(Some(on), relation_on_symbols(&moved), "one relation on Σ's indices");
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let read = crate::transducer::family::extract_rule(&plain, &mut budget).map(|r| r.form.elements.len());
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let read_moved = crate::transducer::family::extract_rule(&moved, &mut budget).map(|r| r.form.elements.len());
    assert_eq!(read, read_moved, "the extraction reads alike");
    // A REFUSAL READS ALIKE: a symbol sent two ways is no fibre relation.
    let mut split = plain.clone();
    split.push((framed(&[1, 1]), framed(&[2, 2])));
    assert_eq!(fibre_relation(&split), None, "1 sent to 3 and to 2");
    assert_eq!(fibre_relation(&relabelled(&split)), None, "and its image sent two ways");
}

/// MU-SENSING §3.3 — AN INDEX INTO Σ IS FIRST APPEARANCE IN CANONICAL
/// ADDRESS ORDER: the samples in the order the family gives them, each
/// sample's sites with axis 0 slowest. Never a code's numeric order.
#[test]
fn symbol_index_is_first_appearance_in_address_order() {
    let place = |at: i64| Place::new(vec![at]);
    let first = Sample::new(vec![2, 2], 1, vec![9, 4, 4, 9], place(0)).expect("a sample");
    let second = Sample::new(vec![3], 1, vec![0, 4, 200], place(1)).expect("a sample");
    let set = SymbolSet::of_samples([&first, &second]).expect("one width");
    assert_eq!(set.len(), 4, "|Σ| is read: 9, 4, 0, 200");
    assert_eq!(listed(&set), vec![vec![9], vec![4], vec![0], vec![200]], "in first appearance");
    assert_eq!((set.index(&[9]), set.index(&[4]), set.index(&[0]), set.index(&[200])), (Some(0), Some(1), Some(2), Some(3)));
    assert_eq!(set.index(&[1]), None, "a symbol the family never showed has no index");
    assert_eq!(set.symbol(1), Some(&[4u8][..]), "the index reads back its symbol");
    // The order is the family's: walked the other way, the indices follow.
    let back = SymbolSet::of_samples([&second, &first]).expect("one width");
    assert_eq!(listed(&back), vec![vec![0], vec![4], vec![200], vec![9]]);
    // A raster's sites are read row by row, the same address order.
    let raster = SymbolSet::of_rasters([vec![vec![9u8, 4], vec![4, 9]].as_slice()]);
    assert_eq!(listed(&raster), vec![vec![9], vec![4]]);
    // Symbols at a wider width are compared whole; two widths are refused.
    let wide = Sample::new(vec![2], 2, vec![1, 0, 0, 1], place(2)).expect("a sample at width 2");
    let wide_set = SymbolSet::of_samples([&wide]).expect("one width");
    assert_eq!((wide_set.len(), wide_set.width()), (2, 2), "[1, 0] and [0, 1] are two symbols");
    assert_eq!(SymbolSet::of_samples([&first, &wide]), Err(SymbolRefusal::Width { sample: 1, width: 1, found: 2 }));
}

/// 78.58, MU-SENSING §3.3 — THE RESERVOIR FOLLOWS Σ: e_∅ stands at index
/// `|Σ|`, the place after the family's own set, read off each family and
/// never a fixed index. A merge's records read at that index on the family's
/// fibre, whatever the family's size.
#[test]
fn reservoir_follows_the_family_symbol_set() {
    // Three symbols: 2 merged into 1, which stands still.
    let small: Pairs = vec![(framed(&[1, 2]), framed(&[1, 1]))];
    let s = SymbolSet::of_pairs(&small);
    assert_eq!((s.len(), s.reservoir()), (3, 3), "e_∅ after 0, 1, 2");
    assert_eq!(s.symbol(s.reservoir()), None, "the reservoir is no symbol");
    let sigma = fibre_relation(&small).expect("a merge, recorded");
    assert_eq!(sigma, vec![(1, 1), (2, 1), (RESERVOIR, 2)]);
    assert_eq!(fibre_on_symbols(&sigma, &s), Some(vec![(1, 1), (2, 1), (3, 2)]), "the record at |Σ| = 3");
    // Five symbols, the same merge: the reservoir moves with Σ.
    let large: Pairs = vec![(framed(&[1, 2, 5, 6]), framed(&[1, 1, 5, 6]))];
    let l = SymbolSet::of_pairs(&large);
    assert_eq!((l.len(), l.reservoir()), (5, 5), "e_∅ after 0, 1, 2, 5, 6");
    let sigma = fibre_relation(&large).expect("the same merge");
    assert_eq!(fibre_on_symbols(&sigma, &l), Some(vec![(1, 1), (2, 1), (5, 2)]), "the record at |Σ| = 5");
    // A relation naming a symbol the family never showed is not read on it.
    assert_eq!(fibre_on_symbols(&[(1, 9)], &s), None);
}

/// MU-SENSING §3.3 — A FAMILY OF ELEVEN SYMBOLS IS READ WHOLE: Σ holds all
/// of them, every symbol a body, the ground deduced, the fibre relation read
/// over a symbol past ten, and every table and permutation over them reads
/// the symbol past ten as any other. Only the tape's record byte stays
/// outside the symbols' bytes until track 26 S5 (MU-SENSING §8).
#[test]
fn family_with_eleven_symbols_is_read_whole() {
    // Ground 0; bodies of 1..=9 and 11 standing still; 11 recoloured to 12.
    let row = |last: u8| framed(&[1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, last]);
    let family: Pairs = vec![(row(11), row(12)), (row(11), row(12))];
    let set = SymbolSet::of_pairs(&family);
    assert_eq!(set.len(), 12, "0..=9, 11 and 12, none truncated");
    assert_eq!(set.index(&[11]), Some(10), "11 at its first appearance");
    assert_eq!(set.reservoir(), 12);
    assert_eq!(ground_of(&family), Ground::One(0), "the ground deduced among twelve");
    let painted: Vec<u8> = on_ground_of(&family, || perceive(&family[0].0)).iter().flat_map(|b| b.compartments.iter().map(|c| c.hue)).collect();
    assert_eq!(painted.len(), 10, "ten bodies: {painted:?}");
    assert!(painted.contains(&11), "the eleventh symbol among them");
    let sigma = fibre_relation(&family).expect("11 → 12 moves a symbol past ten");
    assert_eq!(sigma, vec![(11, 12)]);
    let word = crate::transducer::element::colour_word(&sigma).expect("a colour word over symbols past ten");
    assert_eq!(word.letters().map(|l| l.sigma), Some(sigma), "the word states it");
    // Tables and permutations read the symbols past ten as any other.
    let table = crate::transducer::sentence::hue_table(&[(11, 12), (12, 11)]).expect("a map");
    assert_eq!(crate::transducer::sentence::hue_listing(&table), vec![(11, 12), (12, 11)]);
    assert_eq!(crate::transducer::element::orbit_period(&[(11, 12), (12, 11)], &[(11, 12)]), Ok(2));
    // The record byte is the one byte no tape row can carry as a symbol.
    assert_eq!(lift_fibre(&[(RESERVOIR, 1)]), None, "refused, never read as a record");
}

/// MU-SENSING §3.3 ("across all of its samples") — A SYMBOL SEEN ONLY IN A
/// TEST INPUT IS IN THE FAMILY'S SET: Σ walks the train pairs, then the test
/// inputs the task posts beside them, so the erase act and every Σ reader
/// map a symbol the demonstrations never show. Read with no test posted, the
/// family is its pairs; the scope closes as it returns.
#[test]
fn symbol_seen_only_in_a_test_input_is_in_the_family_set() {
    use crate::transducer::symbols::on_test_inputs;
    let train: Pairs = vec![(framed(&[1, 2]), framed(&[1, 0])), (framed(&[2, 1]), framed(&[0, 1]))];
    let tests = vec![framed(&[7, 2])];
    let pairs = SymbolSet::of_family(&train);
    assert_eq!(listed(&pairs), vec![vec![0], vec![1], vec![2]], "the pairs alone show 0, 1, 2");
    let whole = on_test_inputs(&tests, || SymbolSet::of_family(&train));
    assert_eq!(listed(&whole), vec![vec![0], vec![1], vec![2], vec![7]], "7, seen only in the test input, is in Σ");
    assert_eq!((whole.index(&[7]), whole.reservoir()), (Some(3), 4), "after the pairs' symbols; e_∅ at |Σ| = 4");
    assert_eq!(SymbolSet::of_family(&train), pairs, "the scope closed as it returned");
    // Nested, the inner posting stands, and the outer is restored after it.
    let inner = on_test_inputs(&tests, || {
        let none = on_test_inputs(&[], || SymbolSet::of_family(&train));
        (none, SymbolSet::of_family(&train))
    });
    assert_eq!(inner, (pairs, whole));
}
