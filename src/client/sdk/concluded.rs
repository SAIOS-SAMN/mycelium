// 24.2 — what concluding a listing yields: the cited words and the residual.

/// WHAT THE MIND'S OWN REDUCTION DID, per lane: the residual measured
/// against the chain it came from. A reading — the caller decides what to
/// do with a residual that raised a lane or moved nothing, and 24.2 says
/// what that is: hold it as thought rather than speak it.
#[cfg(test)]
pub(crate) fn residual_move(
    residual: &[(i64, i64, u8, u8)],
    chain: &[(i64, i64, u8, u8)],
) -> [crate::economy::LaneMove; 4] {
    crate::economy::lane_moves(
        &crate::transducer::element::measure_lanes(residual),
        &crate::transducer::element::measure_lanes(chain),
    )
}

/// ONE CONTEMPLATED BUBBLE, as the quote reads it: the remainder it holds
/// and the volume that remainder stands at.
pub type Bubbled = (Vec<(i64, i64, u8, u8)>, [u64; 4]);

/// WHAT CITING A CHAIN YIELDS: the words it cited, and the part of the chain
/// no held word covers.
pub type Cited = (Vec<crate::anastome::dialogue::CiteWire>, Vec<(i64, i64, u8, u8)>);

/// WHAT CONCLUDING A LISTING RETURNS (24.2).
pub struct Concluded {
    /// The words this mind cited to speak the chain.
    pub cites: Vec<crate::anastome::dialogue::CiteWire>,
    /// The chain it could NOT reduce — its own claim, or the whole chain
    /// back when the residual raised a lane or moved nothing and is held
    /// as thought.
    pub residual: Vec<(i64, i64, u8, u8)>,
    /// Whether the genome already held that residual. A familiar residual
    /// is not spoken again.
    pub familiar: bool,
}
