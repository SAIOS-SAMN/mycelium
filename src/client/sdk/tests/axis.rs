// 24.3 — THE AXIS ON MULTIPLICITY: the glue between two bodies that
// deflated one listing is the versor between their shapes, minted as a
// connection with the axis opened on it; one body opens nothing, and two
// bodies no rational versor relates are refused by name.

use crate::client::sdk::*;
use crate::transducer::blade::versor_between;
use crate::transducer::element::{from_canonical, measure_lanes, same_versor, shape_of_chain};
use crate::transducer::germinate::germinate;

#[test]
fn two_bodies_deflating_one_listing_open_an_axis_on_the_versor_between_them() {
    // A row of two cells and a column of two: the same body turned a
    // quarter, two distinct words on the atlas.
    let row = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let column = from_canonical(&germinate(&[(0, 0, 1u8), (1, 0, 1)], &[(0, 0, 2u8), (1, 0, 2)]).unwrap());
    assert_ne!(row.address(), column.address(), "two words");
    let chain = vec![(0, 0, 1u8, 2u8), (0, 1, 1, 2), (0, 2, 1, 2)];
    let germ = Entity::axis_germ(&chain, &[row.sub().to_vec(), column.sub().to_vec()]).expect("two bodies glue");
    let v = versor_between(&shape_of_chain(row.sub()).unwrap(), &shape_of_chain(column.sub()).unwrap())
        .expect("a quarter turn carries the row onto the column");
    assert!(germ.is_connection(), "the axis stands on a connection");
    assert!(same_versor(&germ.versor().expect("a carriage"), &v), "the carriage is the versor between the bodies");
    assert_eq!(germ.opened_axes().len(), 1, "one axis opened on it");
    assert!(germ.sub().is_empty(), "a coordinate, not a remainder");
    assert_eq!(
        germ.magnitude()[3],
        measure_lanes(&chain)[3] + 2,
        "the holonomic lane counts the words that met"
    );
    assert_ne!(germ.address(), row.address());
    assert_ne!(germ.address(), column.address());
    assert_eq!(
        Entity::axis_germ(&chain, std::slice::from_ref(&row.sub().to_vec())).err(),
        Some(AXIS_ONE_BODY),
        "one body opens nothing"
    );
    let three = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1), (0, 2, 1)], &[(0, 0, 2u8), (0, 1, 2), (0, 2, 2)]).unwrap());
    assert_eq!(
        Entity::axis_germ(&chain, &[row.sub().to_vec(), three.sub().to_vec()]).err(),
        Some(AXIS_NO_VERSOR),
        "two cells and three: no rational versor carries one onto the other"
    );
}
