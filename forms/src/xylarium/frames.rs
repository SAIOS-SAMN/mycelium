// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! XYLARIUM'S NAMED FRAMES (CONCORDANCE A6): the exact maps a reading moves
//! between frames by, fixed before any joint read. The EXACT frame is the
//! form's own rational; the BRIDGE frame (base 2, depth 64) is the frame
//! landings are read in — an observer's reading, never a state.

use crate::Q;

/// THE BRIDGE FRAME: the reading floored on the grid of `2^-64`, once, on
/// the whole value — `⌊q · 2^64⌋ / 2^64`, exact.
pub fn bridge(q: &Q) -> Q {
    let whole = Q::from_integer(num_bigint::BigInt::from(1u8) << 64);
    (q * &whole).floor() / whole
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A grid point reads as itself; anything between two reads as the one
    /// below, never the nearer.
    #[test]
    fn the_bridge_floors_once() {
        let whole = Q::from_integer(num_bigint::BigInt::from(1u8) << 64);
        let step = Q::new(1.into(), 1.into()) / &whole;
        assert_eq!(bridge(&Q::new(1.into(), 2.into())), Q::new(1.into(), 2.into()));
        assert_eq!(bridge(&(&step * Q::new(3.into(), 2.into()))), step.clone());
        assert_eq!(bridge(&(&step * Q::new(1.into(), 2.into()))), Q::from_integer(0.into()));
    }
}
