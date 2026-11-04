// MU-SENSING §2, §6; ROADMAP track 26, S5 — samples of any rank on the
// wire: a task envelope, a posted task and a solve's outputs carry samples
// read back exactly; the limits are derived and refused by name.

use crate::anastome::dialogue::samples::{position, Member, Split};
use crate::anastome::dialogue::*;
use crate::transducer::sample::{Place, Sample, SampleRefusal};

/// A sample of `extents` at `width` bytes a symbol, its symbols counting up
/// through the bytes, placed at `at`.
fn sample(extents: &[usize], width: usize, at: Place) -> Sample {
    let sites: usize = extents.iter().product();
    let symbols: Vec<u8> = (0..sites * width).map(|i| u8::try_from(i % 251).expect("a byte")).collect();
    Sample::new(extents.to_vec(), width, symbols, at).expect("a sample")
}

fn at(split: Split, pair: usize, member: Member) -> Place {
    position(split, pair, member).expect("a position")
}

/// TRACK 26 S5 — A TASK ROUND-TRIPS SAMPLES OF RANK n: at ranks 1 to 4 and
/// widths 1, 2 and 4, the envelope, the posted task and a solve's outputs
/// read back the values written, exactly.
#[test]
fn task_round_trips_samples_of_rank_n() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[9; 32]);
    let periods = [2usize, 3, 1, 2];
    for rank in 1..=4 {
        for width in [1usize, 2, 4] {
            let extents = &periods[..rank];
            let train = vec![(
                sample(extents, width, at(Split::Train, 0, Member::Input)),
                sample(extents, width, at(Split::Train, 0, Member::Output)),
            )];
            let tests = vec![sample(extents, width, at(Split::Test, 0, Member::Input))];
            let body = encode_task_envelope(&[7; 32], &train, &tests).expect("the envelope writes");
            let env = decode_task_envelope(&body).expect("the envelope reads");
            assert_eq!(env, TaskEnvelope { id: [7; 32], train: train.clone(), tests: tests.clone() }, "rank {rank}, width {width}");
            assert_eq!(env.tests[0].rank(), rank);
            assert_eq!(env.tests[0].width(), width);

            let posted = PostedTask {
                train: train.clone(),
                test: vec![
                    (tests[0].clone(), Some(sample(extents, width, at(Split::Test, 0, Member::Output)))),
                    (sample(extents, width, at(Split::Test, 1, Member::Input)), None),
                ],
            };
            let file = encode_posted_samples(&posted).expect("the posted task writes");
            assert_eq!(decode_posted_samples(&file), Ok(posted), "the posted task, rank {rank}, width {width}");

            let outputs = vec![sample(extents, width, at(Split::Test, 0, Member::Output))];
            assert_eq!(decode_samples(&encode_samples(&outputs).expect("the outputs write")), Ok(outputs.clone()));
            let act = decode_solve(&encode_solve(&key, &[7; 32], &outputs).expect("the solve writes")).expect("the solve reads");
            assert_eq!(act.outputs, outputs, "the solve's outputs, rank {rank}, width {width}");
            assert!(act.signed);
        }
    }
    // Read whole or refused: a byte short, a byte over, a place off its
    // position.
    let train = vec![(sample(&[2], 1, at(Split::Train, 0, Member::Input)), sample(&[2], 1, at(Split::Train, 0, Member::Output)))];
    let body = encode_task_envelope(&[1; 32], &train, &[]).expect("writes");
    assert!(decode_task_envelope(&body[..body.len() - 1]).is_none(), "a cut envelope is none");
    let mut longer = body.clone();
    longer.push(0);
    assert!(decode_task_envelope(&longer).is_none(), "nothing after the last test input");
    let swapped = vec![(train[0].1.clone(), train[0].0.clone())];
    assert_eq!(encode_task_envelope(&[1; 32], &swapped, &[]), Err(SampleWire::PlaceOffPosition), "written only at its position");
}

/// MU-SENSING §6 — AN EXTENT PAST ITS FIELD IS REFUSED, NEVER WIDENED: an
/// axis of `u16::MAX + 1` sites is refused by name, and nothing is written;
/// an axis of `u16::MAX` sites stands in the field and reads back.
#[test]
fn extent_past_its_field_is_refused_never_widened() {
    let past = usize::from(u16::MAX) + 1;
    let wide = sample(&[past], 1, Place::new(Vec::new()));
    let mut out = vec![5u8];
    assert_eq!(push_sample(&mut out, &wide), Err(SampleWire::ExtentPastField { axis: 0, extent: past }));
    assert_eq!(out, vec![5u8], "nothing is written for a refused sample");
    let held = sample(&[1, usize::from(u16::MAX)], 1, Place::new(vec![3]));
    let mut tape = Vec::new();
    push_sample(&mut tape, &held).expect("the extent stands in its field");
    let mut p = 0;
    assert_eq!(read_sample(&tape, &mut p), Ok(held));
    assert_eq!(p, tape.len(), "read whole");
    // The envelope refuses it too, rather than send a smaller task.
    let train = vec![(sample(&[past], 1, at(Split::Train, 0, Member::Input)), sample(&[1], 1, at(Split::Train, 0, Member::Output)))];
    assert_eq!(encode_task_envelope(&[0; 32], &train, &[]), Err(SampleWire::ExtentPastField { axis: 0, extent: past }));
}

/// A header read off the wire: the rank, the extents, the width and an
/// empty place.
fn header(extents: &[u16], width: u16) -> Vec<u8> {
    let mut out = u16::try_from(extents.len()).expect("a rank").to_le_bytes().to_vec();
    for e in extents {
        out.extend_from_slice(&e.to_le_bytes());
    }
    out.extend_from_slice(&width.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

/// MU-SENSING §6 — THE SYMBOL BYTES' OVERFLOW IS REFUSED BY NAME: the
/// sites `∏ eᵢ` and the bytes `w · ∏ eᵢ` are counted with checked
/// arithmetic, and a count past the machine word is refused by the
/// sample's name before any byte is read or allocated; a count the tape
/// does not carry is short.
#[test]
fn symbol_bytes_overflow_is_refused_by_name() {
    let max = u16::MAX;
    let sites = |extents: &[u16]| extents.iter().try_fold(1usize, |n, &e| n.checked_mul(usize::from(e)));
    // The axis at which the sites pass the machine word.
    let mut extents = vec![max];
    while sites(&extents).is_some() {
        extents.push(max);
    }
    let axis = extents.len() - 1;
    assert_eq!(read_sample(&header(&extents, 1), &mut 0), Err(SampleWire::Sample(SampleRefusal::SitesOverflow { axis })));
    // The sites fit and the bytes do not.
    extents.pop();
    let fit = sites(&extents).expect("the sites fit");
    let width = (2..=max).find(|&w| fit.checked_mul(usize::from(w)).is_none()).expect("some width passes the word");
    assert_eq!(
        read_sample(&header(&extents, width), &mut 0),
        Err(SampleWire::Sample(SampleRefusal::BytesOverflow { sites: fit, width: usize::from(width) }))
    );
    // A count within the word that the tape does not carry.
    assert_eq!(read_sample(&header(&[2, 2], 1), &mut 0), Err(SampleWire::Short));
    // Rank and extents the bytes cannot hold are short before allocation.
    assert_eq!(read_sample(&max.to_le_bytes(), &mut 0), Err(SampleWire::Short));
}
