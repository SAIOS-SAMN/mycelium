// Fixture constructors for the verdict's tests (MU-STANDARDS 3.3): a clean
// word, the stalks it indexes into, and the resolver a replay reads through.

use crate::transducer::element::{from_residual, Element};
use crate::transducer::germinate::germinate;
use crate::network::nerve::Keyed;
use crate::transducer::stalk::Stalks;

pub fn clean_word() -> Element {
    from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap())
}

pub fn stalks_of(fed: &[(crate::transducer::a21::Address, Element)]) -> Stalks {
    Stalks::over_elements(fed.iter().map(|(_, e)| e))
}

pub fn index_of(fed: &[(crate::transducer::a21::Address, Element)]) -> Keyed<usize> {
    let mut k = Keyed::new();
    for (i, (a, _)) in fed.iter().enumerate() {
        k.insert(a, i);
    }
    k
}
