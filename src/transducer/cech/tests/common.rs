// Fixture constructors for the complex's tests (MU-STANDARDS 3.3): a motion,
// a vertex address, a volume, and one rotor of each order.

use crate::transducer::alphabet::Rotor;
use crate::transducer::element::Element;

pub fn motion(dc: i64) -> Element {
    crate::transducer::element::from_connection(
        &crate::transducer::alphabet::Word::identity(),
        &crate::transducer::element::translator_versor(0, dc),
        &[],
        &Vec::new(),
        [1, 0, 0, 0],
    )
    .expect("translator spells")
}

pub fn v(n: u8) -> crate::transducer::address::Address {
    crate::transducer::address::Address::from(vec![n])
}

pub fn vol() -> [u64; 4] {
    [1, 0, 0, 0]
}

pub fn turn_90() -> Rotor {
    let id = crate::transducer::alphabet::LetterId::turn_90((0, 1)).unwrap();
    crate::transducer::alphabet::Word::from_letters(&[(id, false)]).rotor()
}

pub fn sheet() -> Rotor {
    let id = crate::transducer::alphabet::LetterId::turn_90((1, 2)).unwrap();
    crate::transducer::alphabet::Word::from_letters(&[(id, false), (id, false)]).rotor()
}

/// A 90° turn on `e₀∧e₂`: a letter of the seventeen that takes `e₀`
/// OFF the sensor plane, so it has no sensor-plane block and no versor.
/// This is the transition `Cl(3,1)` cannot carry.
pub fn off_plane_turn_90() -> Rotor {
    let id = crate::transducer::alphabet::LetterId::turn_90((0, 2)).unwrap();
    crate::transducer::alphabet::Word::from_letters(&[(id, false)]).rotor()
}

pub fn pythag5() -> Rotor {
    let id = crate::transducer::alphabet::LetterId::pythagorean_5((0, 1)).unwrap();
    crate::transducer::alphabet::Word::from_letters(&[(id, false)]).rotor()
}

pub fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 20)
}
