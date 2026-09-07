// Law 5, 12.6 — THE FOUR LANES: what a word measures, when one measurement
// descends against another, and what the surface prices it at.
//
//! Lanes are never summed and never ordered by a scalar. `descends` is the
//! product order, `dominant` is Dershowitz-Manna dominance and refuses when
//! no candidate dominates, and `price` is per lane.

use super::*;

/// A germ's polytonic magnitude — per-lane support size (‖Δ‖'s |supp| component),
/// never summed, computed EXACTLY from the germ's description. 𝒮 = motion support
/// (shift/rotation/scale), 𝒢 = relabeled-hue count, 𝒯 = temporal extent,
/// 𝒽 = holonomic volume. A settled demand's residual magnitude is the zero vector; a real
/// germ's is nonzero — hence the descent.
pub fn magnitude(g: &Element) -> [u64; 4] {
    g.magnitude()
}

/// The per-lane Dershowitz–Manna descent: `after ≺_DM before` iff after ≤ before
/// on every lane and strictly less on some lane. No lane is traded for another.
pub fn descends(after: &[u64; 4], before: &[u64; 4]) -> bool {
    after.iter().zip(before).all(|(a, b)| a <= b) && after.iter().zip(before).any(|(a, b)| a < b)
}

/// The unique index of the vector that equals the lane-wise minimum, if
/// there is exactly one. Several that match, or none (incomparable lanes),
/// are not dominant.
pub fn dominant(vals: &[[u64; 4]]) -> Option<usize> {
    let min: [u64; 4] = [
        vals.iter().map(|v| v[0]).min()?,
        vals.iter().map(|v| v[1]).min()?,
        vals.iter().map(|v| v[2]).min()?,
        vals.iter().map(|v| v[3]).min()?,
    ];
    let mut found = None;
    for (i, v) in vals.iter().enumerate() {
        if *v == min {
            if found.is_some() {
                return None;
            }
            found = Some(i);
        }
    }
    found
}

/// A refinement is measured as deflation on the split strata. No new price.
pub fn split_deflation(before: &[u64; 4], after: &[u64; 4]) -> Option<[u64; 4]> {
    if !descends(after, before) {
        return None;
    }
    Some(std::array::from_fn(|l| before[l] - after[l]))
}

/// THE YIELD SURFACE — the treasury's dynamic per-lane quote, read from GENUINE
/// metabolic tension: open pressure (unmet demand backlog) + reserve deficit
/// (the depletion premium) + entropic excess (torsion outrunning syntropy).
/// Strained lanes quote high; settled lanes quote low — never a static faucet,
/// and never a summed scalar (polytonic, #34).
pub fn yield_surface(p: &crate::physics::Physics) -> [u64; 4] {
    let flux = p.order_flux();
    let deficit = p.reserve_deficit();
    std::array::from_fn(|l| {
        let entropic = (-flux[l]).max(0) as u64;
        p.pressure[l] + deficit[l] + entropic
    })
}

/// PRICE IS THE MAGNITUDE ON THE LANE: a verified word is worth what it
/// deflated. The surface paces pressure, it does not scale the mint.
pub fn price(mag: &[u64; 4], _surface: &[u64; 4]) -> [u64; 4] {
    *mag
}

/// THE FLOW ORIENTATION of a ledger entry relative to its identity — a signed
/// coefficient in the flow algebra (the chain-complex orientation), never a
/// boolean gate: +1 credits the identity, −1 debits it, 0 orients nothing
/// (markers whose μ⃗ moved the stake bond, not a balance). An exhaustive total
/// function — an unknown kind orients nothing rather than silently crediting.
/// Transfer credits are the second of a paired pair (same μ⃗, K, receipt);
/// `recompute_balances` applies those, not this sign.
pub fn orientation(kind: &str) -> i8 {
    match kind {
        "mint" | "genesis" => 1,
        "transfer" | "slash" | "burn" => -1,
        _ => 0,
    }
}
